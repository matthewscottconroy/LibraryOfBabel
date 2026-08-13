# 16.2 Metcalfe's Memo and CSMA/CD

## The memo

On 22 May 1973, Robert Metcalfe circulated a memo at Xerox PARC describing a
solution to a problem PARC had.

PARC had built the **Alto** — arguably the first personal computer, with a bitmapped
display, a mouse and a graphical interface — and there were about a hundred of them.
It had also built a laser printer capable of a page per second. And there was no way
to connect the Altos to the printer at a rate that made the printer worth having.

Metcalfe had read Abramson's ALOHA paper as a graduate student at Harvard. His
doctoral thesis had been rejected as insufficiently theoretical; he rewrote it with
an improved analysis of ALOHA's performance and it passed. So when PARC needed a
network, he had both the problem and the relevant prior art.

He named it after the **luminiferous aether** — the hypothetical medium through
which nineteenth-century physicists believed light propagated, whose existence the
Michelson–Morley experiment disproved in 1887. The joke is deliberate: a passive
medium that carries waves and does not exist. It is the best-named technology in this
book.

## The two additions

ALOHA transmits blindly. Metcalfe's cable offered something a radio channel did not:
**every station can hear every other station.**

That makes two improvements possible, and together they take utilisation from 18% to
well over 90%.

### Carrier sense

**Listen before transmitting.** If the medium is busy, do not start.

The gain is immediate. ALOHA's vulnerable period is 2*T* — a transmission starting
any time within one packet time either side destroys yours. With carrier sense, a
station that has already begun is audible, so nobody starts on top of it.

The vulnerable period collapses to the **propagation delay**: the time for the first
bits of a transmission to reach the far end of the segment. A station at the far end
that begins transmitting during that window has not yet heard you, and will collide.
Everyone else defers.

On a 2,500 m segment, the one-way propagation is 12.5 µs against a packet time of
1.2 ms for a maximum frame at 10 Mb/s — a vulnerable period **1%** of a packet time
rather than **200%**. That ratio is why carrier sense is worth so much.

### Collision detection

**Listen while transmitting.** If what you hear on the wire is not what you sent,
somebody else is transmitting too.

ALOHA discovers a collision by the *absence of an acknowledgement*, which takes a
full timeout. CSMA/CD discovers it within the propagation delay and **aborts
immediately**, freeing the medium.

A collision therefore costs a fraction of a packet time rather than a whole one, and
the difference under load is large:

```bash
python3 tools/simnet.py csma --stations 20 --load 0.8
```

The station that detects a collision transmits a **32-bit jam signal** before
stopping, to ensure every other station also detects it rather than some seeing a
brief glitch.

## Where the 64-byte minimum comes from

Collision detection requires that a transmitting station **still be transmitting**
when a collision from the far end reaches it. Otherwise it finishes, assumes
success, and never retransmits — and the frame is silently lost with the sender
believing it delivered.

So the frame must occupy the wire for at least one **round-trip propagation time**.

The original specification: 2,500 m maximum with four repeaters, signal velocity
about 2 × 10⁸ m/s.

$$t_{\text{one way}} = \frac{2{,}500}{2 \times 10^8} = 12.5\ \mu\text{s}
\qquad t_{\text{round trip}} = 25\ \mu\text{s}$$

At 10 Mb/s, 25 µs is **250 bits**. The standard specified **512 bits = 64 bytes**,
roughly double, allowing for repeater latency and transceiver delay.

That 512-bit interval is the **slot time**, and it is the fundamental unit of
Ethernet's timing. It determines the minimum frame, it is the unit of backoff, and it
constrains the maximum segment length.

Verify it:

```bash
python3 tools/simnet.py minframe --length 2500 --rate 10M
```

**And the constraint scales badly.** At 100 Mb/s, a 512-bit slot time is 5.12 µs,
which permits a round trip of only 512 m of cable — which is why 100BASE-TX's
maximum collision domain is far smaller than 10BASE5's, and why Gigabit Ethernet had
to introduce **carrier extension** (padding short frames to 512 *bytes* on shared
media) to preserve a usable diameter at all.

That awkwardness is one reason the industry abandoned shared media entirely, which
§16.4 covers.

## Binary exponential backoff

After a collision, both stations must retry — and must not retry simultaneously.

The algorithm:

1. After the *n*th successive collision, choose *r* uniformly at random from
   {0, 1, …, 2ᵏ−1}, where *k* = min(*n*, 10).
2. Wait *r* × 512 bit times.
3. Retry.
4. After 16 successive collisions, give up and report failure to the layer above.

The properties are worth noting because each is deliberate:

**The window doubles**, so the more contention there is, the more spread out the
retries become. This is the damping that ALOHA's instability (§16.1) required, and it
is what keeps a loaded channel from collapsing.

**It caps at *k* = 10**, so the window stops growing at 1,024 slot times — about
52 ms at 10 Mb/s. Without the cap, a station could back off for absurd durations.

**It gives up after 16 attempts**, because a station that has collided sixteen times
is on a network with a serious problem, and continuing to retry makes it worse.

**And it is unfair.** A station that has just succeeded has *n* = 0 and a window of
1; a station that has collided several times has a large window. Under sustained
load, the recently successful station is more likely to win again — the **capture
effect**. It was a known and accepted flaw, and it is one more reason switching was
welcome.

## Worked example of a collision

Two stations, A and B, on a shared segment.

```
 t=0     A senses idle, begins transmitting
 t=4µs   B senses idle (A's signal has not yet arrived), begins transmitting
 t=8µs   A's signal reaches B. B detects collision, sends jam, aborts.
 t=12µs  B's signal reaches A. A detects collision, sends jam, aborts.
 t=13µs  Both stations idle. Both compute backoff:
           A: n=1, k=1, r ∈ {0,1} → chooses 0 → waits 0 slot times
           B: n=1, k=1, r ∈ {0,1} → chooses 1 → waits 51.2 µs
 t=14µs  A retransmits successfully.
 t=65µs  B senses idle, retransmits successfully.
```

Note that both wasted only the fraction of a frame they had sent before detecting —
which is exactly what collision detection buys over ALOHA, where both would have
transmitted complete frames before learning anything.

Note also the coin-flip: had both chosen the same *r*, they would collide again, and
the window would double to {0,1,2,3}.

## What CSMA/CD costs

Three properties, and all three eventually motivated its abandonment.

**Non-determinism.** There is no bound on how long a station may wait. The
probability of sixteen successive collisions is tiny and it is not zero, and there is
no guarantee of transmission within any interval. Token Ring could promise a bound;
Ethernet could not, and §16.3 covers why it won anyway.

**Degradation under load.** Utilisation is excellent at moderate load and falls as
contention rises, because collisions consume capacity. The curve is much better than
ALOHA's and it still bends the wrong way.

**Half duplex is mandatory.** A station must listen while transmitting to detect
collisions, so it cannot use the return path simultaneously. The medium's capacity is
shared between both directions.

## And then it went away

The important thing about CSMA/CD, from a 2026 perspective:

> **On a modern network, CSMA/CD does not run.**

Every port on a modern switch is a separate collision domain with exactly one device
on it (Chapter 17 §17.3), operating **full duplex** with separate paths for each
direction. There is nobody to collide with. Carrier sense is unnecessary; collision
detection is meaningless; backoff never happens.

The mechanism was formally **removed** from the standard in IEEE 802.3-2015 for
speeds above 1 Gb/s — it is not merely unused, it is no longer specified.

What survives is the fossil: the **64-byte minimum frame**, still enforced,
determined by the round trip on a coaxial cable that has not been manufactured in
decades.

And the diagnostic consequence: **a collision on a modern link means something is
wrong.** Late collisions in particular (Chapter 66 §66.2) indicate a duplex
mismatch or a segment exceeding the maximum length, and are never normal.

## What breaks here

**Collisions on a full-duplex link.** Should be zero. Any count indicates a duplex
mismatch.

**Late collisions** — detected after the first 64 bytes, so after the slot time.
Duplex mismatch, or a segment too long. Never normal.

**A half-duplex link under load.** Throughput collapses as contention rises, and it
gets *worse* as offered load increases — which is diagnostic, because most
performance problems degrade gracefully.

**The capture effect** on a legacy shared segment: one station dominating while
another starves.

> **Network+ note.** Objective 1.6 expects CSMA/CD and CSMA/CA and the distinction.
> Two things to carry: **the 64-byte minimum comes from the round trip on a 2,500 m
> segment**, which explains a number that otherwise looks arbitrary; and **CSMA/CD
> does not run on modern networks**, so a collision counter is a fault indication
> rather than normal operation.
