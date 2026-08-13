# Chapter 16 — Important Concepts

**ALOHAnet** *(§16.1)* — Operational June 1971, connecting Hawaiian islands by UHF
radio because leased lines were unaffordable. Answered a question nobody had needed
to: **when should a station transmit on a shared medium where nobody can hear anyone
else?**

**Pure ALOHA** *(§16.1)* — Transmit whenever you have something; if no
acknowledgement arrives, wait a **random** interval and retry. The randomness breaks
symmetry and is what makes it work at all.

**The 18.4% ceiling** *(§16.1)* — Vulnerable period is 2*T* (a transmission starting
up to one packet time either side destroys yours), giving *S* = *Ge*⁻²ᴳ, maximised at
*G* = 0.5 with *S* = **1/2e ≈ 0.184**.

**Slotted ALOHA** *(§16.1)* — Transmissions begin only at slot boundaries, halving
the vulnerable period to *T*: *S* = *Ge*⁻ᴳ, maximum **1/e ≈ 0.368**.

**Why build something 82% wasteful** *(§16.1)* — The comparison was against leased
telephone lines, which cost more per month than the radio system cost to build. **A
design is evaluated against available alternatives, not against an ideal.**

**ALOHA's instability** *(§16.1)* — Throughput *falls* past *G* = 0.5, so more load
produces more collisions produces more retransmissions. Congestion collapse in 1970,
in a single-hop radio network — the same phenomenon as Chapter 38 §38.1's 1986 NSFNET
collapse. **Binary exponential backoff is the damping.**

**The backoff shape** *(§16.1)* — **Detect failure, wait a random interval, retry,
lengthen the interval on repeated failure.** In CSMA/CD, CSMA/CA, TCP's
retransmission timers, and essentially every distributed system that retries.

**Where ALOHA still runs** *(§16.1)* — RFID anti-collision, LoRaWAN uplinks,
satellite access channels, and the cellular RACH. Wherever stations cannot sense each
other and traffic is sparse.

**Carrier sense** *(§16.2)* — Listen before transmitting. Collapses the vulnerable
period from 2*T* to the **propagation delay** — about 1% of a packet time on a
2,500 m segment, against 200% for ALOHA. This is where the utilisation gain comes
from.

**Collision detection** *(§16.2)* — Listen while transmitting; if what you hear is
not what you sent, abort immediately and send a 32-bit **jam signal**. A collision
costs a fraction of a frame rather than a whole one.

**The slot time** *(§16.2)* — 512 bit times, the fundamental unit of Ethernet
timing. Derived from the round trip on a 2,500 m segment: 25 µs at 2 × 10⁸ m/s is
250 bits, doubled for repeater margin. Determines the minimum frame, the backoff
unit, and the maximum collision domain — and **scales badly**, which is why gigabit
needed carrier extension and why shared media were abandoned.

**Binary exponential backoff** *(§16.2)* — After the *n*th collision, wait
*r* × 512 bit times with *r* uniform on {0 … 2ᵏ−1}, *k* = min(*n*, 10); give up after
16. The window **doubles** (damping), **caps at 1,024** slots, and is **unfair** —
the recently successful station has a small window and tends to win again, the
**capture effect**.

**CSMA/CD does not run on modern networks** *(§16.2, §16.4)* — Every switch port is
its own collision domain with one device, full duplex, nothing to collide with. The
mechanism was **removed** from the standard above 1 Gb/s in 802.3-2015. What survives
is the 64-byte minimum, and **a collision counter is now a fault indication**.

**Reading standard names** *(§16.3)* — rate, `BASE` for baseband, then medium and
reach: `T` twisted pair, `S` short-reach MMF, `L` long-reach SMF, `R` 64B/66B coding,
`X` 8B/10B, `4` four lanes. Older names used segment length in hundreds of metres
(`10BASE5` = 500 m).

**10BASE-T (1990) as the decisive step** *(§16.3)* — Same rate, and it moved from a
shared coaxial bus to a **physical star over twisted pair** — logically still a bus,
because a hub repeats everything. What changed was **operational**: fault isolation,
non-disruptive changes, and reuse of the Cat3 already installed for telephones.
Chapter 10 §10.2's argument that coax lost on operational grounds, decided here.

**802.3bz (2016)** *(§16.3)* — 2.5GBASE-T and 5GBASE-T on **unchanged Cat5e and
Cat6**, by encoding harder. An economic motivation — the installed base is enormous —
producing a technical result, and Chapter 10's claim that a medium's properties
follow the manufacturing art.

**Why Ethernet won** *(§16.3)* — **Cheaper at every point** (Token Ring's protocol
complexity is silicon, and the gap never closed); **good enough, and then the
objection evaporated** (switching removed contention, so Ethernet became
deterministic by construction and Token Ring's central advantage lost its subject);
and **the interface stayed stable while the implementation was replaced entirely**.

**Standardise the interface, not the mechanism** *(§16.3)* — Frame format and
addressing unchanged since 1983; medium, topology, coding, arbitration, duplex and
rate all replaced, four times over, across a factor of 80,000 in speed. The most
complete demonstration in this book of the principle Chapter 21 argues abstractly.

**Full duplex** *(§16.4)* — Two devices, separate paths per direction, nothing to
collide with. Carrier sense unnecessary, collision detection meaningless, capacity
doubled, and Token Ring's argument dissolved. Standardised in 802.3x (1997).

**PAUSE frames** *(§16.4)* — 802.3x flow control, rarely enabled because it pauses
**everything** including uncongested traffic and propagates congestion backward.
**Priority Flow Control** (802.1Qbb) refines it per-priority and is essential in
lossless data-centre fabrics.

**Autonegotiation** *(§16.4)* — Fast link pulse bursts encoding a 16-bit capability
word, designed so a non-participating 10BASE-T device sees ordinary link pulses —
**backwards compatible by construction**. Each end selects the highest common
capability, with **full duplex always outranking half at the same speed**. Mandatory
for 1000BASE-T and above.

**The duplex mismatch mechanism** *(§16.4)* — One end hard-coded, the other
autonegotiating. The autonegotiating end receives no advertisement, falls back to
**parallel detection**, can determine speed from the signalling but **cannot
determine duplex**, and the standard requires it to assume **half**. The link comes
up and both administrators believe they configured correctly.

**Duplex mismatch symptoms** *(§16.4)* — Throughput collapses and **gets worse as
load rises**, which is diagnostic because most performance faults degrade gracefully.
Counter signature: **late collisions** on the half-duplex end, plus alignment and CRC
errors. A late collision is impossible on a correct link and means duplex mismatch or
an over-long segment.

**Let both ends autonegotiate** *(§16.4)* — The folklore that hard-coding is safer
was defensible in 1997 and has been wrong for twenty-five years. **Hard-coding one
end is the primary cause of the fault it is meant to prevent.** If a link must be
hard-coded, hard-code both ends identically and document it.

**PoE operation** *(§16.4)* — **Detection** (a 25 kΩ signature, so power is never
applied to a device not expecting it), **classification** (by current signature or
LLDP, so only what is needed is allocated), then 44–57 V DC. **Mode A** rides on the
data pairs as a **common-mode** voltage, which works precisely because the data is
differential and Chapter 6 §6.4's subtraction ignores it.

**PoE budgets are per switch** *(§16.4)* — A 48-port switch with 802.3at on every
port and a 740 W supply serves 24 ports at 30 W, not 48. Exceeding it powers devices
in port order and leaves the rest dark. The arithmetic must be done before ordering.

**PoE's second-order effects** *(§16.4)* — Dissipated power warms the bundle, raising
attenuation and reducing effective length; and **the switch's power becomes the
telephone's power**, so the access switch's UPS is part of the voice system's
availability design — routinely forgotten until the first mains failure.
