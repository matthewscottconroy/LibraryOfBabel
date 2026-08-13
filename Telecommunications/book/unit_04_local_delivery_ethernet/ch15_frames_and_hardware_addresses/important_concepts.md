# Chapter 15 — Important Concepts

**Why frames exist** *(§15.1)* — Three independent reasons: **fairness** on a shared
medium (a maximum size stops one station monopolising it), **bounding error cost**
(a lost 1,500-byte frame is far cheaper than a lost 40 kB transmission), and
**delimiting** (a continuous voltage stream has no punctuation).

**Frame delimiting techniques** *(§15.1)* — By length field (fails badly on
corruption), by delimiter pattern (needs bit or byte stuffing), by **code violation**
(cleanest — a physical symbol that cannot occur in data), or by silence. Ethernet
uses a **preamble and SFD** plus a mandatory **interframe gap** of 96 bit times
(12 bytes).

**The 64-byte minimum** *(§15.1)* — Derived from CSMA/CD: a station must still be
transmitting when a collision from the far end returns. Round trip on a 2,500 m
coaxial segment at 2 × 10⁸ m/s is 25 µs, which at 10 Mb/s is 250 bits; 512 bits was
specified for margin. **Still enforced on every modern switch**, entirely vestigial,
and the reason **runts** are always abnormal.

**The 1,500-byte maximum** *(§15.1)* — Chosen in 1980 balancing efficiency against
medium monopolisation and 1980 buffer costs. **Jumbo frames** (~9,000 bytes) raise
efficiency from 94.9% to 99.1% and require **every device on the path to agree** —
one dissenter produces the black-hole failure of Chapter 66 §66.3.

**MAC address structure** *(§15.2)* — 48 bits: 24-bit **OUI** from the IEEE registry
plus 24 manufacturer-assigned. 2⁴⁸ ≈ 281 trillion addresses. The OUI is publicly
searchable and identifies the vendor.

**The I/G bit** *(§15.2)* — Bit 0 of the first byte: 1 means multicast. Practically:
**an odd first byte means the address is not unicast**. `01:00:5e` is IPv4 multicast,
`01:80:c2:00:00:00` is the STP BPDU address, `33:33` is IPv6 multicast,
`ff:ff:ff:ff:ff:ff` is broadcast.

**The U/L bit** *(§15.2)* — Bit 1: 1 means **locally administered**, so uniqueness is
the administrator's responsibility. Appears in virtual machines, FHRP virtual
routers, and privacy randomisation.

**Flat addressing** *(§15.2)* — A MAC address is **a name, not a coordinate**. No
rule summarises a set of them, so any device needing to know where an address lives
must learn it **individually, by observation, in a finite table**. Everything
follows: switch learning, finite CAM tables, MAC flooding attacks, bounded broadcast
domains, VLANs, and the necessity of IP.

**Why flat addressing anyway** *(§15.2)* — It requires **no administration**. Plug in
a device and it works, uniquely, with nobody assigning anything. The general
principle: **flat buys zero-configuration and costs scalability; hierarchical buys
scalability and costs administration.** Large systems use both, at different layers.

**MAC randomisation** *(§15.2)* — Since ~2014 for probe requests and iOS 14 /
Android 10 for association, devices use rotating locally administered addresses to
defeat retail and public-space tracking. It broke MAC filtering, DHCP reservations,
captive portal persistence and Wi-Fi analytics. **The correct response is to stop
using the MAC address as an identity** and use 802.1X, which is unaffected.

**Preamble and SFD** *(§15.3)* — Seven bytes of `10101010` for clock recovery, then
`10101011` whose final consecutive ones mark the start of frame. **Stripped by the
interface and absent from captures**, which surprises people checking byte offsets.

**Destination address first** *(§15.3)* — Deliberate: a receiver (or a cut-through
switch) can decide after six bytes rather than after the whole frame.

**EtherType versus length** *(§15.3)* — Ethernet II uses the field as a **type**;
IEEE 802.3 used it as a **length**. The rule: **≤ 1500 is a length, ≥ 1536 is an
EtherType**, with the gap reserved to keep them unambiguous. Ethernet II won
essentially completely. The EtherType is Chapter 2 §2.4's self-describing framing
and Chapter 23's layering made physical.

**Padding** *(§15.3)* — Payloads under 46 bytes are padded to reach the 64-byte
minimum. **Ethernet II carries no length field**, so the upper-layer protocol must
supply one — which is why IP has Total Length, and why a protocol without one cannot
safely ride in a padded frame.

**Total on the wire** *(§15.3)* — 12 (gap) + 8 (preamble/SFD) + 1,518 (frame) =
**1,538 bytes** for a maximum-size frame, which is Chapter 3 §3.1's efficiency
denominator.

**The error-detection ladder** *(§15.4)* — **Parity** (all single-bit errors, no
double); **checksum** (more, and insensitive to reordering because addition
commutes); **CRC** (far more, cheap in hardware).

**CRC mechanism** *(§15.4)* — Treat the message as a polynomial over GF(2), divide by
a generator, transmit the remainder. The receiver divides the whole frame; **a zero
remainder means no error detected**. Implemented as a shift register and XOR gates,
which is why it runs at line rate at 400 Gb/s.

**What CRC-32 detects** *(§15.4)* — All single-bit errors, all double-bit errors,
all odd numbers of errors, **all burst errors up to 32 bits**, and 99.99999998% of
longer bursts. The burst guarantee matters because real errors are bursty
(Chapter 6 §6.2).

**Detect versus correct** *(§15.4)* — Correction needs roughly twice the redundancy
of detection, and **the choice depends on retransmission cost**: microseconds on a
LAN (detect), forty minutes to deep space (correct). 400GBASE-R added forward error
correction because the tradeoff moved when the channel did.

**Silent discard** *(§15.4)* — A failed FCS means the frame is dropped with no
notification and no link-layer retransmission. Consequently **a low corruption rate
shows no symptom except a counter and degraded throughput** — link up, full speed,
dashboards green, `ping` clean, and TCP capped by the Mathis relation.

**The counters** *(§15.4)* — CRC/FCS errors (physical), alignment errors (usually
duplex mismatch), runts (collision or duplex mismatch), giants (unexpected 802.1Q tag
or jumbo mismatch), **output drops (congestion, not corruption)**. The last
distinction most often separates a correct diagnosis from a wasted cable
replacement.

**A CRC protects against accident, not intent** *(§15.4)* — An attacker who modifies
the payload recomputes it. Cryptographic MACs exist for exactly this gap
(Chapter 57 §57.2).

**A passing FCS is not end-to-end integrity** *(§15.4)* — A store-and-forward switch
verifies and then **recomputes** the FCS, so corruption inside the switch is covered
by a freshly valid one. Stone and Partridge's measurements found end-to-end checksum
failures far above link error rates, largely from middleboxes and host software.
This is why TCP's checksum exists despite Ethernet's stronger CRC — the end-to-end
argument applied to integrity.
