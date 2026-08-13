# Chapter 20 — Important Concepts

**The problem VLANs solve** *(§20.1)* — A switched network of any size is **one
broadcast domain**. Three consequences, of very different weight: broadcast volume
(real, least important), **failure blast radius**, and **security** — anything on a
broadcast domain can ARP-spoof, scan, and attack anything else on it. **A lobby laptop
shares a trust boundary with the finance server.**

**Why physical segmentation failed** *(§20.1)* — People are not sorted by department,
people move (every reorganisation becomes a cabling project), port utilisation is
terrible, and exceptions are impossible. Correct in principle, unaffordable in
practice.

**The insight** *(§20.1)* — Segmentation is a property of the **forwarding decision**,
not the wiring. One added rule: **a frame may only be forwarded to ports in the same
VLAN as the port it arrived on.**

**What follows per VLAN** *(§20.1)* — Its own broadcast domain, its own MAC table
entries, its own spanning tree (or MSTP instance), and its own IP subnet. And
membership is **configuration, not cabling**.

**What VLANs are not** *(§20.1)* — Not encryption (traffic within a VLAN is as visible
as ever). **Not a firewall** — once inter-VLAN routing exists, VLANs talk freely unless
policy is applied. Not unlimited: 4,094.

**The precondition, not the control** *(§20.1, §20.4)* — **Segmentation creates
enforcement points; it does not enforce anything.** The same relationship holds for
firewall zones, cloud security groups and microsegmentation.

**Assignment methods** *(§20.1)* — **Static/port-based** overwhelmingly; **802.1X
dynamic** growing fast and completing the original goal, because **the VLAN follows the
user rather than the cable**; MAC-based and protocol-based essentially extinct.

**The trunk problem** *(§20.2)* — A frame crossing between switches carries no VLAN
information, because VLAN membership is switch configuration and configuration does not
travel with frames. Pre-1998 answer: one cable per VLAN.

**The 802.1Q tag** *(§20.2)* — **4 bytes, inserted after the source MAC**:
**TPID `0x8100`** (a reserved EtherType) plus a 2-byte TCI containing **3-bit PCP**,
**1-bit DEI** (formerly CFI), and **12-bit VID**.

**A non-802.1Q device discards tagged frames** *(§20.2)* — It reads `0x8100` as an
unknown EtherType. This is what the native VLAN exists to accommodate, and therefore
the root of §20.3's security problem.

**4,094 usable VLANs** *(§20.2)* — 2¹² = 4096; **VID 0 means priority-only, no VLAN**;
**4095 reserved**. The limit is why VXLAN's identifier is **24 bits (16.7 million)**.

**PCP / 802.1p** *(§20.2)* — Eight classes; 5 is voice, 0 is best effort, and **1 is
lower than 0**. **Layer 2 only — stripped at every routed hop.** DSCP is the Layer 3
equivalent and the two must be explicitly mapped. **A marking is a request, not a
guarantee.**

**Tagged frames are 1522 bytes** *(§20.2)* — 802.3ac formalised the extension from
1518. **Every tag and tunnel costs MTU** (QinQ 1526, VXLAN 1568), which is why data
centres run jumbo frames.

**QinQ / 802.1ad** *(§20.2)* — An outer **service tag** (TPID `0x88a8`) identifying the
customer, wrapping the customer's own inner tag. **The same trick as NAT**: when an
identifier space is exhausted, add hierarchy and reuse it per scope.

**Access port** *(§20.3)* — One VLAN, **untagged**. The tag is added internally on
ingress and stripped on egress. **The attached device knows nothing about VLANs**, and
that invisibility is why VLANs deployed so easily.

**Trunk port** *(§20.3)* — Many VLANs, **tagged**. `switchport trunk allowed vlan` is
the most important and most neglected line — a trunk carries **all** VLANs by default.
Beware: without `add`, the command **replaces** the list.

**DTP and switch spoofing** *(§20.3)* — A port at `dynamic auto` **becomes a trunk if
asked**. An attacker sends a DTP frame from a wall socket and receives every VLAN.
**Configure trunks and access ports explicitly; `switchport nonegotiate` everywhere.**

**The native VLAN** *(§20.3)* — The one VLAN sent **untagged** on a trunk, for
compatibility with devices that do not understand 802.1Q. **Default 1 — which is also
the default access VLAN of every unconfigured port.**

**Double-tagging VLAN hopping** *(§20.3)* — Attacker in the native VLAN sends a frame
with two tags. The first switch **strips the outer (native) tag** as specified; the
second switch reads the inner tag and delivers into the target VLAN. **Unidirectional**
— no replies — and **every switch behaved exactly as specified**, which is what makes
it instructive.

**The three mitigations** *(§20.3)* — (1) **native VLAN to an unused ID** (999);
(2) **do not use VLAN 1 for anything**, and put unused ports in a dead VLAN **and**
shut them down; (3) **`vlan dot1q tag native`**, which removes the untagged case
entirely.

**Native VLAN mismatch** *(§20.3)* — Traffic from one VLAN silently arrives in another,
with no error. CDP/LLDP detect and log it — one of the better arguments for leaving
them on internally.

**VTP's failure mode** *(§20.3)* — Updates carry a **revision number** and switches
accept any higher one. **A lab switch with a high revision and an empty VLAN database,
plugged into a production trunk, deletes every VLAN in the domain.** Use transparent
mode, VTP version 3, or nothing.

**Why inter-VLAN routing needs a router** *(§20.4)* — Definitional: a switch forwards
only within a VLAN. And hosts send off-subnet traffic to their gateway anyway
(§18.1), so the traffic arrives at a router because the host addressed it there.

**One VLAN = one broadcast domain = one IP subnet** *(§20.4)* — Not convention.
**Two subnets in one VLAN** makes hosts route to a gateway that sends traffic straight
back. **One subnet across two VLANs** makes hosts ARP for each other and receive no
reply.

**Router-on-a-stick** *(§20.4)* — One trunk to a router with a **subinterface per
VLAN** (`encapsulation dot1Q N`). **Every inter-VLAN packet crosses the trunk twice**,
so a 1 Gb/s link supports 500 Mb/s of inter-VLAN traffic, in software.

**SVIs** *(§20.4)* — `interface VlanN` on a Layer 3 switch: hardware forwarding, line
rate, no extra device. **What is actually used.** `ip routing` is required and is **the
most commonly forgotten line in the chapter** — its omission gives working VLANs, no
routing between them, and a configuration that looks correct.

**First-hop redundancy** *(§20.4)* — An SVI's failure removes off-VLAN connectivity for
its whole VLAN. VRRP/HSRP/GLBP share a **virtual IP and MAC**, and failover works by
**gratuitous ARP** (§18.3).

**Voice VLAN** *(§20.4)* — A deliberate special case: one access port carrying **data
untagged and voice tagged**. The telephone learns its voice VLAN by **CDP or
LLDP-MED**, tags its own traffic, and passes the workstation's through untagged. Not a
trunk — the narrow scope keeps PortFast/BPDU Guard applicable and avoids DTP and VLAN
hopping exposure.

**Trust markings only at a boundary you control** *(§20.4)* — `mls qos trust device
cisco-phone` accepts the telephone's markings and **rewrites everything else to zero**,
so a workstation cannot mark itself as voice. A general QoS principle.

**PoE budget is a design constraint** *(§20.4)* — 48 ports × 15.4 W exceeds many
switches' supply, and the failure mode is telephones failing to boot in an order nobody
predicted.
