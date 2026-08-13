# 17.4 Forwarding Modes and Buffers

How quickly a switch commits to forwarding, and what happens to frames that cannot
leave immediately. The second half is where most real switch problems live.

## The three forwarding modes

**Store-and-forward.** Receive the entire frame, verify the FCS (Chapter 15 §15.4),
then forward.

- **Latency:** one full frame time, plus processing. At 1 Gb/s, 12 µs for a
  maximum-size frame; at 10 Gb/s, 1.2 µs.
- **Never forwards a corrupted frame.** The CRC covers the whole frame, so it cannot
  be checked until the whole frame has arrived.
- **Permits rate adaptation.** A frame arriving at 10 Gb/s and leaving at 1 Gb/s must
  be buffered; there is no alternative.
- **The default on essentially every modern switch.**

**Cut-through.** Read the destination address — the first 6 bytes after the preamble
— and begin forwarding immediately.

- **Latency:** roughly 500 ns, independent of frame size.
- **Forwards corrupted frames**, because the FCS has not been seen yet. The
  corruption is detected by the *receiver*, one hop later, having consumed capacity
  along the way.
- **Cannot adapt rates**, since ingress and egress must run at the same speed.
- Used where latency dominates: high-frequency trading, some HPC fabrics.

**Fragment-free.** Read the first 64 bytes, then forward.

- **Latency:** about 5 µs at 1 Gb/s — between the two.
- Catches most corruption, on the reasoning that collisions on a legacy shared
  segment occur within the slot time (Chapter 16 §16.2), so damage concentrates in
  the first 64 bytes.
- A compromise designed for a hazard that full-duplex switching eliminated. Largely
  historical.

## Choosing between them

| | Store-and-forward | Cut-through |
|---|---|---|
| Latency at 1 Gb/s | ~12 µs | ~0.5 µs |
| Latency at 10 Gb/s | ~1.2 µs | ~0.5 µs |
| Forwards bad frames | **no** | yes |
| Rate adaptation | **yes** | no |
| Modern default | **yes** | specialist |

Note the third row's effect at high speed: **as rates rise, store-and-forward's
latency penalty shrinks** — 12 µs at 1 Gb/s but 1.2 µs at 10 Gb/s — while
cut-through's advantage is fixed. Combined with the requirement for rate adaptation
in almost every real topology, this is why store-and-forward is now essentially
universal outside specialist fabrics.

Many switches offer **adaptive cut-through**: run cut-through until the error rate
on a port exceeds a threshold, then fall back to store-and-forward for that port. It
gets the latency when the link is clean and stops propagating corruption when it is
not.

## Buffers, and where frames actually wait

More important in practice than the forwarding mode, and less discussed.

A frame must be buffered whenever it cannot leave immediately:

- **The egress port is busy** transmitting something else.
- **Rate mismatch** — arriving at 10 Gb/s, leaving at 1 Gb/s.
- **Many-to-one** — several ports sending to one destination simultaneously.

The third case is the one that causes trouble, and it is called **incast**.

### Buffer architectures

**Shared memory.** All ports draw from one pool. Flexible — a single congested port
can use a large share — and the flexibility is also the hazard, since one port can
starve the others. Most access switches.

**Dedicated per-port.** Each port has its own fixed allocation. Predictable and
wasteful, since an idle port's buffer helps nobody.

**Hybrid**, with a guaranteed minimum per port plus a shared pool. What most modern
switches actually do.

The important observation: **switch buffers are small.** A typical access switch has
a few megabytes total across all ports — not per port. A 48-port switch with 4 MB of
shared buffer has, on average, 85 KB per port, which at 1 Gb/s is **680
microseconds** of transmission.

That is not much. Which is deliberate, for the reason §17.4's bufferbloat discussion
gives.

## Microbursts

The phenomenon that explains most "the switch is dropping packets but utilisation is
low" reports.

Traffic is bursty at every timescale (Chapter 13 §13.3's self-similarity). A link
averaging 30% over five minutes can be at **100% for tens of milliseconds** at a
time — and during those bursts, arriving frames queue, and if the burst outlasts the
buffer, they are discarded.

The numbers:

```
   Five-minute average utilisation:     30%   ← what the graph shows
   One-second peak:                     71%
   One-millisecond peak:               100%   ← where the drops happen
```

**Standard monitoring cannot see this.** SNMP polling at five-minute intervals
(Chapter 54 §54.1) averages the burst into invisibility. The only evidence is the
**output drop counter**, which is why Chapter 15 §15.4 insisted that output drops are
a distinct signal from CRC errors — and why an interface showing drops on a
lightly-utilised link is reporting microbursts rather than a fault.

**The remedies**, in order of preference: more buffer (helps, with the caveat below);
QoS so that the traffic you care about is not what gets dropped (Chapter 52); traffic
shaping at the source; and — often best — accepting it, since TCP handles occasional
loss and the alternative may be worse.

## Head-of-line blocking

A structural problem worth understanding because it explains a class of surprising
behaviour.

If a switch buffers frames in a single FIFO queue per *ingress* port, then a frame at
the head of the queue destined for a congested egress port **blocks every frame
behind it** — including frames destined for completely idle ports.

```
   ingress queue:  [→ port 5 (busy)] [→ port 2 (idle)] [→ port 7 (idle)]
                          ↑
                   blocks these two
```

The throughput ceiling for a switch with pure input queueing and random traffic is
about **58%**, which is a substantial loss.

**Virtual output queueing** is the fix: maintain a separate queue per egress port at
each ingress port, so a frame for a congested port never blocks one for an idle port.
Every serious switch does this, and it is one of the things that distinguishes a
well-engineered switch from a cheap one.

## Why more buffer is not better

The intuition is that a full buffer means loss, so more buffer means less loss and
better performance.

It is wrong, and Chapter 13 §13.3 and Chapter 66 §66.4 develop why. The short form:

**TCP uses loss as its congestion signal.** A large buffer does not prevent
congestion; it **hides** it. Frames that would have been dropped are queued instead —
for milliseconds, sometimes hundreds of milliseconds — and TCP, receiving no loss
signal, keeps increasing its window. The queue grows until it finally overflows, and
by then **every frame crossing that port is delayed by the full queue depth**,
including latency-sensitive traffic that had nothing to do with filling it.

**A buffer's job is to absorb a burst, not to store a backlog.** The right size is
enough to ride out a microburst and no more, and the modern answer for anything that
carries interactive traffic is **active queue management** — CoDel, FQ-CoDel, PIE,
CAKE — which drops based on how long a frame has waited rather than on how full the
buffer is.

## Switching capacity, and what the datasheet means

Two numbers appear on every switch datasheet and both are frequently misread.

**Switching capacity**, in Gb/s. A 48 × 1 Gb/s switch with four 10 Gb/s uplinks
quotes 176 Gb/s: (48 × 1 + 4 × 10) × 2, counting both directions. **Non-blocking**
means the fabric can carry every port at line rate simultaneously.

**Forwarding rate**, in packets per second. This is the harder number, because a
switch is limited by *lookups* rather than by bits. At minimum frame size —
64 bytes plus 20 bytes of gap and preamble, so 84 bytes = 672 bits — a 1 Gb/s port
can deliver

$$\frac{10^9}{672} \approx 1.488 \ \text{million packets per second}$$

For 48 such ports plus four 10 Gb/s uplinks, line rate is about 130 Mpps. A switch
quoting a lower forwarding rate **cannot** handle all ports at line rate with small
frames, however impressive its Gb/s figure.

This matters because small-frame traffic is exactly what a denial-of-service attack
generates, and a switch chosen on its Gb/s figure may fall over under a packet-rate
attack that is nowhere near its bandwidth capacity.

## What breaks here

**Output drops on a lightly-utilised interface.** Microbursts. The five-minute
average is hiding them; the drop counter is the only evidence.

**Cut-through propagating corruption.** A bad cable on one port produces corrupted
frames forwarded across the fabric, consuming capacity and appearing as errors on
distant ports. Adaptive cut-through, or store-and-forward, prevents it.

**Head-of-line blocking on a cheap switch** without virtual output queueing. Traffic
to an idle port stalls behind traffic to a busy one, and the symptom is unexplained
latency that does not correlate with the affected port's utilisation.

**Bufferbloat on a large-buffered switch.** A bulk transfer destroys interactive
latency on the same port.

**A switch chosen on Gb/s that cannot sustain its packet rate.** Falls over under
small-frame load.

**Incast in a data centre** — many servers replying to one request simultaneously,
overwhelming the egress buffer. A well-known and genuinely difficult problem, and one
of the drivers for lossless data-centre Ethernet (Chapter 71 §71.5).

> **Network+ note.** Objective 1.2 expects switch operation. The transferable
> content here is diagnostic: **output drops mean congestion or microbursts, not a
> physical fault**, and **utilisation graphs cannot show microbursts** — which
> between them account for a large share of "the switch is dropping packets and I
> cannot see why".
