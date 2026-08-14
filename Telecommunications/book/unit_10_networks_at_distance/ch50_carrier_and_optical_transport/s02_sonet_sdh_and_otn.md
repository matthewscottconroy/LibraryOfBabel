# 50.2 SONET/SDH and OTN

Two ideas make SONET what it is: one clock for the whole network, and a ring that heals in
under 50 milliseconds. Everything else follows.

## The word in the name

Plesiochronous systems (§50.1) let every multiplexer run its own clock, which forces bit
stuffing, which makes it impossible to find a low-rate channel inside a high-rate stream
without unwinding the whole hierarchy.

**SONET's answer: lock everything to one clock.**

```
   Primary Reference Clock (caesium / GPS-disciplined)
        │  accuracy 1 × 10^-11
        ▼
   Building Integrated Timing Supply ──▶ every network element in the building
        │
        ▼
   traceable, hierarchically distributed, network-wide
```

Every element derives its timing from a reference traceable to an atomic standard. With a
common clock, a byte's position in the frame is its identity — byte 47 of every frame is
the same channel, always, and you can extract it by reaching into the stream and taking
it.

> Synchronisation converts a multiplexing problem into an addressing problem, and that is
> the whole trick. **Add-drop multiplexing** — pulling one tributary out of a passing stream
> and inserting another, without touching the rest — is what makes a linear chain of cities
> economic to serve.

## The frame

The STS-1 frame is 9 rows of 90 bytes, transmitted 8,000 times per second:

$$9 \times 90 \times 8 \times 8{,}000 = 51.84 \text{ Mb/s}$$

```
   ◀──────────────── 90 columns ────────────────▶
   ┌───┬───┬───┬─────────────────────────────────┐ ▲
   │   │   │   │                                 │ │
   │ section &│      Synchronous Payload         │ │ 9
   │ line     │      Envelope (SPE)              │ │ rows
   │ overhead │      87 columns                  │ │
   │ 3 cols   │                                  │ │
   └───┴───┴───┴─────────────────────────────────┘ ▼
     ▲                    ▲
   3 columns of        one column is path overhead,
   overhead            86 carry payload = 49.536 Mb/s
```

The 8,000 frames per second is not arbitrary. It is the telephone sampling rate
(Chapter 12 §12.3), which means one byte per frame is exactly one DS0 — 8 bits × 8,000
per second = 64 kb/s. The entire structure is built so that a voice channel is one byte in a
fixed position.

**Higher rates are byte-interleaved multiples:**

| SONET | SDH | Rate |
|---|---|---|
| **OC-1 / STS-1** | — | **51.84 Mb/s** |
| **OC-3** | **STM-1** | **155.52 Mb/s** |
| OC-12 | STM-4 | 622.08 Mb/s |
| **OC-48** | **STM-16** | **2.488 Gb/s** |
| **OC-192** | **STM-64** | **9.953 Gb/s** |
| OC-768 | STM-256 | 39.813 Gb/s |

Every rate is exactly $n \times 51.84$ Mb/s, which is a small elegance the plesiochronous
hierarchy never had.

SDH starts at STM-1 (155.52 Mb/s = 3 × STS-1) because Europe's E1 hierarchy maps more
naturally into that size. The two are interoperable at STM-1/OC-3 and above, which is why
the divergence of §50.1 stops mattering here.

## The 50 millisecond ring

SONET's most distinctive property, and the number was a requirement rather than a result.

```
        ┌──────── working ring (clockwise) ────────┐
        │                                          │
      Node A ──── Node B ──── Node C ──── Node D ──┘
        │                                          │
        └─────── protect ring (anticlockwise) ─────┘

   Cut between B and C:
   traffic reverses direction at B and D, taking the long way round
   Detection + switching: under 50 ms
```

**Why 50 ms?** Because a telephone call must not drop. Telephone switches drop a call when
the circuit is lost for longer than roughly 60 ms, and voice-band data equipment of the era
lost synchronisation at about the same point. 50 ms was chosen with margin.

**How it is achieved:**

- The protection path is pre-provisioned — the capacity is already reserved and idle
- Failure detection is at the physical layer — loss of signal or loss of frame, in
  microseconds
- The switch is local — the two nodes adjacent to the break decide, with no protocol
  convergence, no election, and no distributed computation

> **Compare spanning tree** (Chapter 19 §19.2): 30–50 **seconds** for classic STP, a second or
> two for RSTP. SONET was three orders of magnitude faster in 1988, and it achieved that by
> reserving half its capacity for the purpose.

**Which is the honest trade.** A 1+1 protected SONET ring runs at 50% utilisation by
design. You buy an OC-48 and you can carry OC-48's worth of traffic, with an identical
amount of capacity sitting idle for the milliseconds in which you need it.

Packet networks refused that trade — statistical multiplexing exists precisely to avoid
reserving idle capacity — and spent twenty years reaching comparable protection times by
other means (MPLS fast reroute, §50.4).

## Ring architectures

| Type | Mechanism | Efficiency |
|---|---|---|
| **UPSR / SNCP** | **transmit both ways, receiver picks the better** | **50%; simple; ideal for hub-and-spoke** |
| **BLSR / MS-SPRing** | **protection capacity shared around the ring** | **better on meshed traffic; more complex** |
| **1+1 linear** | two paths, point to point | 50% |

UPSR is the "send it both ways and let the receiver choose" design — no signalling at all
on failure, because the receiver simply notices one path has gone quiet and uses the other.
It is beautifully simple and it wastes exactly half.

BLSR shares the protection bandwidth around the ring, so traffic between adjacent nodes
does not consume protection capacity all the way round. More efficient, and it requires
signalling between nodes to coordinate the switch.

## Overhead, and what it bought

SONET devotes about 4% of its bandwidth to overhead, organised in three layers that
correspond exactly to three kinds of equipment:

| Layer | Between | Carries |
|---|---|---|
| **Section** | **adjacent repeaters** | framing, error monitoring (B1), the section trace |
| **Line** | **adjacent multiplexers** | protection switching (K1/K2), error monitoring (B2) |
| **Path** | **end to end** | payload type, end-to-end error monitoring (B3), path trace |

And this is the property that made carriers love SONET: each layer is monitored
independently, so a fault localises itself.

> A B2 error count rising at one multiplexer and not at its neighbour identifies the failing
> span without anyone leaving the office. Chapter 65's layer-by-layer diagnosis is SONET's
> overhead structure, generalised — and SONET did it in hardware, continuously, on every
> frame.

**The path trace** — a 16-byte string carried end to end — is a small thing worth admiring.
Each end writes its own identifier; if what you receive is not what you expect, the circuit
is patched wrongly, and you find out immediately rather than during an outage.

## OTN — the modern wrapper

**ITU-T G.709**, and it replaced SONET/SDH for new build.

**The problem with SONET at 2000s rates:** it was designed to carry voice channels, and what
carriers actually carry is 10 Gb/s and 100 Gb/s Ethernet, plus SONET, plus storage protocols,
plus whatever comes next. Mapping a 10 GbE signal into SONET is awkward.

OTN's structure is a wrapper rather than a hierarchy:

```
   ┌─────────────────────────────────────────────────┐
   │ OTU overhead │      client signal      │  FEC   │
   │              │  (10GbE, 100GbE, SDH,   │        │
   │              │   anything)             │  ~7%   │
   └─────────────────────────────────────────────────┘
        digital wrapper — "put anything in, get it out unchanged"
```

| Level | Rate | Typically carries |
|---|---|---|
| **OTU1** | 2.666 Gb/s | STM-16 / OC-48 |
| **OTU2** | **10.709 Gb/s** | **10 GbE, OC-192** |
| OTU3 | 43.018 Gb/s | 40 GbE |
| **OTU4** | **111.81 Gb/s** | **100 GbE** |

Note the rates are above the client rate — 10.709 for a 9.953 client — and the difference
is the overhead plus the FEC.

### The FEC is the point

OTN's forward error correction is what it is actually for.

Standard OTN uses Reed–Solomon RS(255,239) — about **7% overhead** — giving roughly
6 dB of coding gain. Enhanced FEC schemes give 8–11 dB.

And 6 dB of coding gain buys optical reach, which is worth more than almost anything else
at these distances:

> 6 dB is a factor of four in required optical power, or — held the other way — it lets
> the signal traverse substantially more fibre and more amplifiers before regeneration.
> Each regeneration site avoided is a building, a power feed, and a maintenance contract for
> twenty-five years.

**§50.3 makes the argument concretely.** For now: FEC is the reason a modern coherent system
crosses an ocean without electrical regeneration, and it is the largest reason OTN
displaced SONET.

**OTN also does what SONET did well:** six levels of **tandem connection monitoring**, so a
circuit crossing four carriers can be monitored **per carrier**, and each can be held to its
own SLA. That is a commercial feature implemented in framing overhead, and it matters more
than it sounds.

## What breaks here

**B1 errors rising and B2 clean.** **A section-layer problem** — the span between two
repeaters. The layer that errors identifies the segment.

**Path alarms with all line layers clean.** The problem is beyond this network — the
end-to-end circuit is impaired somewhere you do not control.

**A path trace mismatch.** The circuit is patched wrongly. Extremely common during
provisioning, and the trace catches it before service does.

**Protection switching that does not restore.** Both paths are on the same physical
route — a "diverse" pair sharing one duct or one bridge crossing. This is the classic and
expensive discovery, and it is made during the outage.

**Repeated protection switches with no hard failure.** **A marginal span** flapping around the
threshold. Worse than a clean failure, because the traffic is disturbed each time.

An OTN circuit with rising pre-FEC errors and no post-FEC errors. The FEC is doing its
job and the margin is being consumed. This is a warning, not an alarm, and it is the most
useful early-failure indicator in optical networking — act on it before the errors become
uncorrectable.

> **Network+ note.** Objective 1.2 and 2.1. Over-learn: **SONET and SDH are synchronous
> optical transport**; OC-3 is 155 Mb/s, OC-12 is 622, OC-48 is 2.5 Gb/s, OC-192 is 10;
> **SONET rings provide sub-50 ms protection**; and **SDH is the international equivalent of
> SONET.** The rate ladder is examinable; the 50 ms figure is the one worth understanding
> rather than memorising.
