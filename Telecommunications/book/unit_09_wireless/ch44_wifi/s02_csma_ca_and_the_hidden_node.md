# 44.2 CSMA/CA and the Hidden Node

Chapter 16 built CSMA/CD for shared Ethernet: **listen, transmit, detect the collision,
back off.** Wireless cannot do that, and understanding *why* explains most of Wi-Fi's
behaviour.

## Why collision detection is impossible

**Three reasons, and each is sufficient alone.**

**1. A radio cannot listen while transmitting.** The transmitted signal at the antenna is
**perhaps a hundred million times stronger** than any received signal — a station transmitting
at 20 dBm and receiving at −70 dBm is a 90 dB difference. **The receiver is deafened by its
own transmitter**, so it cannot detect a collision as it occurs.

**2. A collision happens at the *receiver*, not at the transmitter.** In Ethernet, all
stations share one cable and see the same signals. **In radio, each station hears a different
subset of the world** — so a transmitter has no way to know whether its frame collided
somewhere it cannot hear.

**3. The hidden node problem** — below — makes it worse: two stations may be unable to hear
each other at all while both being heard by the receiver.

> **So Wi-Fi cannot detect collisions. It must avoid them**, which is what the CA stands for
> — and avoidance is necessarily more conservative and more expensive than detection.

## CSMA/CA

**The mechanism:**

```
   1. Listen. Is the medium busy?
        Physically (energy detected) OR virtually (NAV — see below)

   2. If busy, wait until it is free.

   3. Wait DIFS (a fixed interframe space).

   4. Choose a random backoff from the contention window, and count down
      — pausing the countdown whenever the medium becomes busy.

   5. At zero, transmit.

   6. Wait for an ACK.  NO ACK = assume collision.

   7. On failure: DOUBLE the contention window, and retry from 3.
```

**Two details carry most of the consequences.**

**Every unicast frame is acknowledged.** There is no other way to know it arrived — the
sender could not detect a collision. **So every data frame costs a data frame plus a SIFS
plus an ACK**, and at high data rates the fixed overheads dominate.

**The backoff counter pauses while the medium is busy.** So a station that has already
counted down some of its backoff resumes where it left off, **which gives it an advantage
over a station that starts fresh** — a deliberate fairness mechanism.

### The interframe spaces

**Priority is implemented as different waiting times**, and this is elegant:

| Space | Length (5 GHz) | Used for |
|---|---|---|
| **SIFS** | **16 µs** | **ACKs, CTS** — the shortest, so they always win |
| PIFS | 25 µs | point coordination |
| **DIFS** | **34 µs** | **ordinary data** |
| AIFS | varies | **QoS categories** — voice waits less than background |

> **A station wanting to send an ACK waits SIFS and a station wanting to send data waits
> DIFS**, so the ACK always goes first. **Priority by patience**, with no negotiation
> required.

**And 802.11e's QoS categories** — voice, video, best effort, background — **use different
AIFS values and contention window sizes**, so voice statistically wins contention against
bulk data. Chapter 52 covers what that is worth.

## The overhead

**Worth quantifying, because it explains why Wi-Fi's real throughput is roughly half its
nominal rate.**

**Sending one 1500-byte frame at a nominal 54 Mb/s:**

| Component | Time |
|---|---|
| DIFS | 34 µs |
| Average backoff | ~67 µs |
| Preamble and PLCP header | 20 µs |
| **Data (1500 bytes at 54 Mb/s)** | **222 µs** |
| SIFS | 16 µs |
| ACK | 24 µs |
| **Total** | **~383 µs** |

$$\text{efficiency} = \frac{222}{383} \approx \mathbf{58\%}$$

**And it gets worse at higher rates**, because the data time shrinks while the fixed
overheads do not:

| Nominal rate | Data time for 1500 B | Efficiency |
|---|---|---|
| 54 Mb/s | 222 µs | ~58% |
| **600 Mb/s** | **20 µs** | **~11%** |
| **1.3 Gb/s** | **9 µs** | **~5%** |

> **At gigabit rates, over 90% of the airtime is protocol overhead.** This is why **frame
> aggregation** (§44.1) was essential rather than merely nice — sending sixty-four frames as
> one amortises the fixed cost, and without it the higher rates would deliver almost nothing.

**This is the fundamental reason Wi-Fi throughput is about half the nominal rate**, and it is
worth being able to explain rather than merely assert.

## The hidden node problem

**The problem that has no clean solution.**

```
        A ●─────────── AP ───────────● C
          ╲           ▲             ╱
           ╲ can hear │ can hear   ╱
            ╲         │           ╱
             ╲── A and C CANNOT hear each other ──╱
```

**A and C are both in range of the access point and out of range of each other.**

**So when A transmits, C hears nothing** — carrier sense says the medium is free — **and C
transmits, and the two collide at the access point.**

**Neither knows.** Neither can hear the other, and neither can detect the collision. **They
learn only from the missing acknowledgement, and they will do it again.**

**This is not an edge case.** It arises whenever coverage is larger than the stations'
mutual range, which is normal — **the access point is usually higher, better positioned and
better equipped than the clients.**

**The symptoms:**

- **High retry rates with good signal on all clients**
- **Throughput far below expectation with no obvious cause**
- **Worse as client count rises**
- **Clients at opposite edges of a cell performing worst**

### RTS/CTS

**The mitigation**, and it is a trade rather than a fix.

```
   A → AP:   RTS (Request to Send)      "may I transmit for N microseconds?"
   AP → all: CTS (Clear to Send)        "A may transmit for N microseconds"
                                         ↑
                     C HEARS THIS, even though it cannot hear A,
                     and sets its NAV — it will not transmit for N µs
   A → AP:   DATA
   AP → A:   ACK
```

**The access point's CTS is heard by everyone in its cell**, including the stations that
cannot hear each other. **So the hidden node learns to defer from the access point rather
than from the transmitter.**

**The NAV — Network Allocation Vector — is the mechanism:** every frame carries a duration
field, and every station that hears it **sets a timer and treats the medium as busy until it
expires**, whether or not it can hear the actual transmission.

> **This is "virtual carrier sense": a station defers because it was told to, not because it
> heard anything.**

**The cost:** RTS and CTS are two extra frames plus two SIFS, **on every transmission.**

| Frame size | RTS/CTS overhead |
|---|---|
| **Small (64 B)** | **enormous** — more overhead than payload |
| 1500 B | ~10–15% |
| **Aggregated (64 KB)** | **negligible** |

**Which is why it is controlled by a threshold**: use RTS/CTS only for frames above a certain
size, where the overhead is proportionally small.

```
   RTS threshold 2347 (default) = effectively disabled
   RTS threshold 500            = used for most data frames
```

**The default is off**, and turning it on is a specific remedy for a diagnosed hidden-node
problem — **not a general improvement.** Enabling it universally costs throughput everywhere
to fix a problem in one place.

## The exposed node problem

**The mirror image, and it is less discussed and equally real.**

```
        AP1 ●──────● B      C ●──────● AP2
                    ╲      ╱
                     ╲    ╱
                  B and C CAN hear each other
```

**B is transmitting to AP1. C wants to transmit to AP2.**

**C hears B and defers** — but **C's transmission would not have interfered**, because it is
directed at a different receiver in a different direction.

> **Carrier sense is about the transmitter's neighbourhood; interference is about the
> receiver's.** The exposed node defers unnecessarily, and **capacity is lost.**

**802.11ax's BSS colouring** (§44.1) addresses part of this: **a station can identify whether
an overheard transmission belongs to its own network**, and apply a more permissive threshold
to other networks' traffic — **so it defers less to transmissions that were never going to
affect it.**

## Protection mechanisms

**The cost of backward compatibility**, and it is substantial.

**An 802.11b device cannot decode an OFDM transmission** — it hears energy and does not know
how long it will last, so it cannot set its NAV.

**So when a b device is associated, the access point enables protection:** every OFDM
transmission is preceded by **a CTS-to-self or an RTS/CTS exchange sent at a legacy rate that
b devices can decode.**

**The cost is severe:**

| Situation | Throughput |
|---|---|
| Pure 802.11g | 100% |
| **g with protection enabled** | **~50–60%** |
| Pure 802.11n | 100% |
| n with legacy protection | ~60–70% |

> **One 802.11b device associated to an access point can halve the throughput for every
> other client on that radio.**

**The remedy is to disable the low data rates**, so that legacy devices cannot associate at
all:

```
   Disable 1, 2, 5.5 and 11 Mb/s  →  802.11b devices cannot join
   Set the minimum basic rate to 12 or 24 Mb/s
```

**Which also has a second benefit**: it shrinks the effective cell size, because a client
must have enough SNR for the minimum rate. **Distant clients that would have connected at
1 Mb/s and consumed disproportionate airtime are excluded**, and §44.2's next point explains
why that matters.

## Airtime fairness

**The consequence of rate adaptation that surprises people.**

**A slow client occupies the medium for longer to send the same data.**

```
   Fast client, 400 Mb/s:  1 MB takes  20 ms
   Slow client,   6 Mb/s:  1 MB takes 1333 ms   ← 67× the airtime
```

**And CSMA/CA gives each station an equal chance to transmit, not an equal share of time.**

> **So one distant client at 6 Mb/s can consume more airtime than twenty nearby clients at
> 400 Mb/s**, and everyone's throughput collapses to accommodate it.

**This is the "one slow client ruins it for everyone" effect**, and it is real, quantifiable
and frequently the explanation for a cell that performs badly with no obvious fault.

**The remedies:**

**Airtime fairness scheduling** — the access point allocates *time* rather than
*opportunities*, so a slow client gets its share of milliseconds rather than its share of
transmissions. **Standard on enterprise equipment and worth verifying is enabled.**

**Disable low rates** (above), so very slow clients cannot associate.

**Better coverage**, so clients are not slow in the first place — which is Chapter 45's
subject.

## What breaks here

**High retries with good signal on all clients.** Hidden nodes. Consider RTS/CTS with a
threshold.

**Throughput about half the nominal rate.** Normal. That is the protocol overhead.

**Throughput far below half.** Contention, interference, protection mechanisms, or a slow
client consuming airtime.

**One old device degrading a whole cell.** Protection mechanisms. Disable low rates.

**A cell that degrades whenever one particular user is present.** Airtime fairness. Check it
is enabled and check that user's rate.

**RTS/CTS enabled everywhere and throughput lower.** It is a targeted remedy, not a general
improvement.

> **Network+ note.** Objective 2.4 expects CSMA/CA. Over-learn: **wireless cannot detect
> collisions because a radio cannot listen while transmitting, so it avoids them**; **every
> unicast frame is acknowledged**; **the hidden node problem is two stations that cannot hear
> each other but share a receiver**; and **RTS/CTS mitigates it at the cost of overhead.**
> The CSMA/CD-versus-CSMA/CA distinction is examined directly.
