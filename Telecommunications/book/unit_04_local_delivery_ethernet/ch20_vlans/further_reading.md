# Chapter 20 — Further Reading

## Primary sources

**IEEE 802.1Q, *Bridges and Bridged Networks*.**
The standard. Now the consolidated bridging specification, having absorbed 802.1D,
802.1s and 802.1ad. Clause 9 defines the tag format of §20.2 — worth reading for the
precision, and short.

**IEEE 802.1ad, *Provider Bridges* (QinQ).**
Stacked VLANs and the service/customer tag distinction.

**IEEE 802.3ac (1998).**
The two-page amendment that extended the maximum frame size to 1522 bytes to
accommodate the tag. A good example of how much coordination a four-byte change
requires.

**TIA-1057, *LLDP-MED*.**
How an IP telephone learns its voice VLAN, power requirement and QoS policy without
being configured. Unglamorous and responsible for a great deal.

**RFC 5517 — Hamilton, D. et al. (2010). *Cisco Systems' Private VLANs.***
Private VLANs: isolation *within* a VLAN, for hosting environments and guest networks
where members should reach the gateway but not each other. The natural next step after
this chapter.

## Books

**Convery, S. (2004). *Network Security Architectures.* Cisco Press.**
**The reference for §20.3.** The systematic treatment of Layer 2 attacks — VLAN
hopping, DTP abuse, MAC flooding, ARP spoofing — at a time when security attention was
elsewhere. Dated in its product specifics and entirely current in its framing: the
access layer is a security layer, and a wall socket is an attack surface.

**Seifert, R. & Edwards, J. (2008). *The All-New Switch Book*, 2nd ed. Wiley.**
Chapters 11–12 on VLANs, with the standards-level detail and the implementation
variations. Good on why the tag is where it is.

**Vachon, B. & Graziani, R. *Accessing the WAN* / current CCNA materials.**
The configuration in depth, with the error cases. Whichever edition you find; VLAN
configuration has been stable for twenty years.

**Hucaby, D. *CCNP Enterprise Advanced Routing / Switching* volumes.**
For SVIs, first-hop redundancy, and the interaction between VLANs, spanning tree and
routing at campus scale.

## Applied

**`show vlan brief`, `show interfaces trunk`, `show interfaces Gi0/x switchport`.**
The three commands. **`show interfaces trunk` is the incident command** — it shows
allowed, active and forwarding VLANs in three columns, and most VLAN faults are a
disagreement between those columns.

**`show vtp status`.**
Check the revision number on any switch before you connect it to anything. This takes
five seconds and has saved entire campuses.

**Linux VLAN interfaces:** `ip link add link eth0 name eth0.10 type vlan id 10`.
A free way to build tagged interfaces and watch the frames. Combine with a bridge and
two namespaces for a complete VLAN lab on one machine.

**Open vSwitch**, for VLAN, trunk and tunnel behaviour that is fully inspectable.
`ovs-vsctl` and `ovs-ofctl dump-flows` show the forwarding decisions directly, which
makes the abstract rule of §20.1 concrete.

**Wireshark filters:** `vlan`, `vlan.id == 20`, `vlan.priority == 5`.
Capture on a trunk and on an access port in the same VLAN, and compare. Every
difference should be accountable — this is exercise F1 and it is the fastest way to
make tagging real.

**Lab 07** in this book's [labs/](../../../labs/) directory builds a two-switch VLAN
topology with a trunk, verifies isolation, then adds inter-VLAN routing with SVIs and
demonstrates the `ip routing` failure deliberately. **Lab 08** performs the double-
tagging attack on an isolated segment and then defeats it.

## On the security

**Yersinia** and the Layer 2 attack tooling.
Implements DTP abuse, VLAN hopping, STP root takeover and more. Use only on equipment
you own or are authorised to test. The point of running it is to see how little effort
each attack requires, which is the argument for the hardening lines in §20.3.

**Cisco, "VLAN Security White Paper"** and the equivalent from other vendors.
The vendor treatment of §20.3's attacks and mitigations, with configuration.

**Rouiller, S. (2003). "Virtual LAN Security: Weaknesses and Countermeasures."** SANS.
A careful early analysis of what VLANs do and do not provide as a security boundary.
Its central claim — that VLANs are a segmentation tool and not a security control —
is §20.1's position and remains contested by people selling things.

## For the certification-minded

Objective 2.3 expects VLANs, 802.1Q, trunking, inter-VLAN routing and voice VLANs.
Objective 1.6 expects VLANs as broadcast-domain boundaries. Objective 4.2 expects VLAN
hopping as an attack.

Seven things worth over-learning:

1. **Each VLAN is a separate broadcast domain and normally a separate IP subnet.**
2. **Access port = one VLAN, untagged. Trunk = many VLANs, tagged.**
3. **The 802.1Q tag is 4 bytes; TPID `0x8100`; 12-bit VID gives 4,094 usable VLANs.**
4. **The native VLAN is untagged on a trunk; the default is VLAN 1.**
5. **VLAN hopping has two forms** — switch spoofing via DTP, and double tagging via
   the native VLAN — with different mitigations.
6. **Inter-VLAN routing requires a router or a Layer 3 switch.** Router-on-a-stick uses
   subinterfaces with `encapsulation dot1Q`; an SVI is `interface VlanN`.
7. **A voice VLAN carries voice tagged and data untagged on a single access port.**

And two that are worth more than several examined items:

**`ip routing` is missing.** When VLANs work and inter-VLAN routing does not on a
Layer 3 switch whose configuration looks perfect, this is the answer far more often
than anything else.

**A VLAN is not a security control.** It is what makes a security control placeable.
The ACL or firewall at the routing point is the control, and a design that stops at
"we put them in separate VLANs" has not yet done the security work.
