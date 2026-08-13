# 71.4 Deterministic Networking and TSN

**Everything else in this book has treated the network as best effort with statistical
guarantees** (Chapter 13, Chapter 52). **This section is about the requirement that is not
statistical**, and it is the frontier area with the most deployment and the least publicity.

## The requirement

> **Not "usually under 10 ms" but "never more than 250 microseconds, and if it is, the machine
> stops."**

**Which is a different kind of requirement**, and Chapter 52 §52.1's QoS does not supply it:

| | **QoS** | **Deterministic** |
|---|---|---|
| Provides | **priority under contention** | **a bounded worst case** |
| Guarantee | **statistical** | **absolute** |
| Failure | **occasional lateness** | **not permitted** |
| Achieved by | **scheduling policy** | **scheduling policy plus admission control plus time** |

**And the applications are the ones where lateness is a safety property:**

| | Requirement |
|---|---|
| **Industrial motion control** | **cycle times of 31.25 µs to 1 ms, jitter under 1 µs** |
| **Automotive** | **sensor to actuator, bounded, with functional safety requirements** |
| **Professional audio and video** | **sample-accurate synchronisation across devices** |
| **Power grid protection** | **IEC 61850, and a relay must trip within milliseconds** |
| **Avionics** | **AFDX, and it has been deterministic since the 1990s** |

> **The reason this matters now is convergence.** **These applications ran on dedicated
> fieldbuses — PROFIBUS, CAN, EtherCAT, ARINC 429 — and the pressure is to run them on Ethernet
> alongside everything else**, which requires Ethernet to provide something it was not designed
> to provide.

## What makes Ethernet non-deterministic

**Three sources, and TSN addresses each.**

**Queueing.** **A frame arriving at a busy egress port waits** (Chapter 52 §52.1), **and the wait
depends on what else is there.**

**Serialisation of a frame already in progress.** **A 1,500-byte frame at 1 Gb/s occupies the
wire for 12 µs**, and **a high-priority frame arriving one bit into it waits the full 12 µs** —
**which is larger than an entire motion control cycle.**

**And clock divergence.** **Two switches whose clocks differ cannot execute a coordinated
schedule.**

## The TSN toolkit

**IEEE 802.1 has produced a set of amendments, and they are complementary rather than
alternative.**

| Standard | Provides |
|---|---|
| **802.1AS** | **time synchronisation** — sub-microsecond, across the network |
| **802.1Qbv** | **time-aware shaping** — gates opened and closed on a schedule |
| **802.1Qbu / 802.3br** | **frame preemption** — interrupt a frame in flight |
| **802.1Qav** | **credit-based shaping** — for audio/video streams |
| **802.1CB** | **frame replication and elimination** — send twice, discard the duplicate |
| **802.1Qcc** | **configuration and stream reservation** |
| **802.1Qci** | **per-stream policing** — a misbehaving talker cannot disturb others |

### Time synchronisation is the foundation

**802.1AS is a profile of IEEE 1588 (PTP)**, and it achieves **sub-microsecond accuracy** —
**against NTP's milliseconds** (Chapter 41 §41.3).

> **The mechanism is hardware timestamping.** **The timestamp is applied by the MAC as the frame
> leaves the wire, not by software when it is queued** — **which removes the operating system's
> scheduling jitter from the measurement**, and it is the difference between milliseconds and
> nanoseconds.

**And everything else depends on it.** **A coordinated schedule across ten switches requires the
ten switches to agree what time it is**, to a fraction of the schedule's granularity.

### Time-aware shaping is the mechanism

**802.1Qbv: each egress queue has a gate, and the gates open and close on a repeating schedule.**

```
   Cycle: 1000 µs
   ┌────────────┬────────────┬───────────────────────────────┐
   │  0–125 µs  │ 125–250 µs │        250–1000 µs            │
   │ Q7 OPEN    │ Q6 OPEN    │  Q0–Q5 OPEN                   │
   │ others CLOSED           │  (best effort)                │
   └────────────┴────────────┴───────────────────────────────┘
        critical    control            everything else
```

> **During its window, the critical queue's traffic is the only thing that can be transmitted.**
> **There is nothing to queue behind**, so **the latency is the serialisation time and the
> propagation time, and both are computable.**

**Which is time-division multiplexing** (Chapter 9), **reintroduced into a packet network for
exactly the reason Chapter 13 said packet switching gave it up** — **and it is another instance
of the pattern Chapter 50 §50.4 identified with MPLS: an industry that abandoned circuits
discovering it wanted some of their properties.**

### Frame preemption solves the guard band problem

**The difficulty with a schedule: a frame already being transmitted when a gate should open.**

**Without preemption, the switch must not start a frame that would still be transmitting when
the gate opens** — **so it must leave a guard band the size of a maximum frame:**

| | |
|---|---|
| **Guard band without preemption** | **12 µs at 1 Gb/s** (a 1,500-byte frame) |
| **Eight queues, eight guard bands** | **96 µs of a 1,000 µs cycle — 9.6% wasted** |
| **Guard band with preemption** | **~0.5 µs** (a 64-byte fragment) |

**802.3br splits a frame in flight**, transmits the urgent one, and **resumes the interrupted
frame afterwards** — **which reduces the guard band by a factor of twenty and recovers the
capacity.**

### Replication for reliability

**802.1CB sends every critical frame twice, by disjoint paths, and the receiver discards the
duplicate.**

> **Which is Chapter 51 §51.2's packet duplication argument** — **redundancy is cheap when the
> payload is small and the consequence of loss is large** (Chapter 5) — **and here the
> consequence of loss is a machine stopping.**

**And it removes reconvergence from the failure path entirely.** **A link failure does not
require the network to detect and reroute** (Chapter 31 §31.4); **the second copy was already in
flight.**

## What it costs

**Four things, and they are why TSN is deployed in specific places rather than generally.**

**Configuration complexity.** **A schedule must be computed for every switch in the path of every
critical stream**, and **the computation is a constraint-satisfaction problem** — **which is why
802.1Qcc specifies a centralised configuration entity** (Chapter 68 §68.1's controller, in a
domain where the argument is unambiguous).

**Admission control.** **A guarantee requires that the traffic offered does not exceed what was
planned**, so **streams must be declared and reserved before they transmit** — **which is
IntServ's model** (Chapter 52 §52.1), **and it works here because the number of streams is small
and known.**

**Capacity.** **Time reserved for a critical stream is time unavailable to everything else**,
whether the stream uses it or not — **which is Chapter 56 §56.2's protection trade and
Chapter 13 §13.1's circuit argument.**

**And hardware.** **Every switch in the path must support the relevant amendments**, and
**hardware timestamping is a silicon feature.** **A TSN domain is as deterministic as its least
capable switch.**

## Where it is actually deployed

| Domain | Status |
|---|---|
| **Industrial automation** | **real, growing** — PROFINET, EtherCAT and OPC UA over TSN |
| **Automotive in-vehicle networks** | **shipping** — and it is the largest volume |
| **Professional audio/video** | **AVB, TSN's predecessor, is mature** |
| **Power utilities** | **IEC 61850 process bus** |
| **Aerospace** | **AFDX predates TSN and solves the same problem differently** |
| **Enterprise and campus** | **essentially none, and correctly so** |

> **The last row is the honest one.** **A campus network has no deterministic requirement**, and
> deploying TSN there would add configuration complexity and admission control for no benefit.

## DetNet

**The IETF's Layer 3 counterpart**, and it is less mature.

**TSN is a Layer 2 mechanism within a bridged domain.** **DetNet extends the requirement across
routed networks** using **explicit paths (segment routing, Chapter 50 §50.4), resource
reservation and packet replication** — **the same toolkit, at Layer 3.**

**Its difficulty is that the routed network's variability is larger and its administrative scope
is wider**, and **deployment is largely limited to single-operator networks with a specific
requirement.**

## What this section illustrates

**Beyond the technology, an argument the book has made repeatedly.**

> **Packet switching gave up guarantees to gain statistical efficiency** (Chapter 13 §13.1).
> **Every subsequent mechanism that provides a guarantee — ATM, MPLS traffic engineering, IntServ,
> TSN, DetNet — reintroduces reservation, and pays the same price: capacity held for traffic that
> may not arrive.**

**And the pattern of adoption is consistent:** **the guarantee is bought where the consequence of
lateness justifies the cost, and best effort is used everywhere else** — **which is the correct
outcome and is not a failure of either approach.**

## What breaks here

**A TSN schedule that works on the bench and fails in the plant.** **Clock synchronisation**,
or a switch in the path that does not support the amendments.

**A guarantee that fails when an undeclared stream appears.** **No admission control**, and the
guarantee was conditional on the traffic being what was planned.

**9.6% of the cycle lost to guard bands.** **No frame preemption.** 802.3br.

**Best-effort traffic starved during a critical window.** **Working as designed**, and the
capacity was reserved.

**A TSN deployment in a campus network.** **No deterministic requirement**, and it is
configuration complexity for nothing.

**One switch in the path without hardware timestamping.** **The domain is as deterministic as its
least capable element.**

**Reconvergence causing a missed deadline.** **802.1CB removes reconvergence from the failure
path** — the second copy was already sent.

> **Network+ note.** TSN is beyond the syllabus. The transferable content is Chapter 52's:
> **QoS provides priority and not a guarantee**, and **a guarantee requires reservation, which
> requires capacity to be held idle.** **And Chapter 41 §41.3's: time synchronisation is a
> networking problem**, which TSN demonstrates at a thousand times the precision NTP provides.
