# Chapter 20 — VLANs

A company occupies three floors. On each floor there are staff workstations, a
handful of IP telephones, two security cameras, a printer, and — in a cupboard on
the second floor — the payroll server.

Every one of those devices is plugged into the same switched network, which by
Chapter 17 §17.3 means every one of them is in the same broadcast domain. Which
means every device can reach every other device at Layer 2, which means the
security camera in reception can send frames to the payroll server, and a
broadcast storm on the guest Wi-Fi takes down the telephones, and an ARP spoof
(Chapter 18) from any port can intercept traffic from any other.

The obvious remedy is to build separate physical networks: separate switches,
separate cabling, separate everything. This works, and it is what was done before
1996, and it is ruinous. Four physical networks per floor means four times the
switches, four cable runs to every location, and a decision at cabling time about
what each outlet will be used for that must then hold for a decade.

**Virtual LANs** deliver the separation without the duplication. One physical
switch is partitioned into several independent logical switches. Ports assigned to
VLAN 10 constitute one broadcast domain; ports in VLAN 20 constitute another; and
frames cannot pass between them inside the switch at all. As far as a host can
tell, it is plugged into a switch that has only the other VLAN 10 ports on it.

The consequence to hold onto:

> **A VLAN is a broadcast domain. Creating a VLAN creates a broadcast domain.
> Traffic between VLANs must be routed, exactly as if they were physically
> separate networks — because logically, they are.**

That last clause is the one students most often fail to internalise, and it is the
source of the most common VLAN misconfiguration: creating the VLANs,
assigning the ports, and then being surprised that nothing can reach anything,
because no routing was configured. §20.4 is about the several ways to supply it.

## The tag

If one cable between two switches must carry traffic for eight VLANs, the frames
must somehow be distinguishable. **IEEE 802.1Q**, standardised in 1998, inserts
four bytes into the Ethernet frame between the source address and the EtherType:

```
  ┌──────────┬──────────┬─────────────────────────┬───────────┬─────────┬─────┐
  │ Dest MAC │ Src MAC  │  802.1Q tag (4 bytes)   │ EtherType │ Payload │ FCS │
  └──────────┴──────────┴─────────────────────────┴───────────┴─────────┴─────┘
                         TPID 0x8100 │ PCP │ DEI │ VLAN ID (12 bits)
```

Twelve bits of VLAN ID gives 4,096 values, of which 0 and 4095 are reserved,
leaving 4,094 usable — a number that felt generous in 1998 and became a genuine
constraint for cloud providers two decades later, which is precisely why VXLAN
(Chapter 67) exists with its 24-bit identifier and 16 million segments.

The three-bit **PCP** field is the Class of Service marking used for Layer 2
quality of service, and it is how a switch knows to prioritise voice frames over
file transfers before any IP header is examined. Chapter 52 uses it.

Note the consequence of adding four bytes: a tagged frame can be 1,522 bytes rather
than 1,518. Equipment that does not expect this reports the extra frames as
**giants** or **baby giants** and may discard them, which is a classic and
maddening failure at the boundary between two administrative domains.

## The native VLAN, and why it is a security problem

A trunk port carries tagged frames for many VLANs — except, by default, for one.
The **native VLAN**'s frames traverse the trunk *untagged*, a provision included in
1998 for compatibility with devices that did not understand tagging.

It is the source of two distinct problems that appear on every certification exam
and in real outages.

**Mismatch.** If one end of a trunk has native VLAN 1 and the other has native
VLAN 99, untagged frames arriving from VLAN 1 are placed into VLAN 99. Two
broadcast domains are silently merged. There is no error; there is simply
connectivity that should not exist, which is worse than an outage because nobody
notices.

**VLAN hopping.** An attacker on an access port in the native VLAN can craft a
frame with *two* 802.1Q tags. The first switch strips the outer tag (it matches the
native VLAN, so it is removed) and forwards the frame with the inner tag intact —
which the next switch honours, delivering the frame into a VLAN the attacker was
never permitted to reach. The double-tagging attack is thirty years old and still
works against default configurations. The mitigations — set the native VLAN to an
unused ID, never use VLAN 1 for anything, and tag the native VLAN explicitly — are
three commands, and Chapter 62 returns to them.

## What this chapter does

§20.1 develops the case for logical segmentation, working the three-floor example
through in cost and security terms, and covers the standard VLAN design patterns
(by function, by department, by security zone) with their tradeoffs.

§20.2 covers 802.1Q tagging bit by bit, the frame size consequence, QinQ, and how a
switch decides whether to tag.

§20.3 covers access ports, trunk ports, the native VLAN, VLAN pruning and allowed
lists, dynamic trunking protocols and why they should be disabled, and the
configuration of each on real equipment.

§20.4 covers inter-VLAN routing: router-on-a-stick with subinterfaces, switched
virtual interfaces on a Layer 3 switch, and routed ports — with the performance
and design arguments for each — plus voice VLANs and how an IP phone with a
built-in switch fits into all of this.

## By the end you will be able to

- Explain why a VLAN is a broadcast domain and what follows from that.
- Decode an 802.1Q tag in a capture and state which VLAN a frame belongs to.
- Distinguish access and trunk ports and configure both.
- Explain the native VLAN, predict the effect of a mismatch, and describe the
  double-tagging attack and its mitigations.
- Choose an inter-VLAN routing method for a stated scenario and justify it.
- Diagnose the four classic VLAN faults: wrong access VLAN, native mismatch,
  missing VLAN on a trunk's allowed list, and missing routing.
