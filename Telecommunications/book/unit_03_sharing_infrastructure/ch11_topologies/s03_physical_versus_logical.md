# 11.3 Physical versus Logical Topology

The distinction that resolves more confusion than any other in this chapter, and
the one that makes the whole of Unit XIV comprehensible.

## The two questions

**Physical topology** answers: *where do the cables go?* It is what you see in the
cable tray, what a floor plan shows, what an electrician installs. It is about
copper, glass and rack units.

**Logical topology** answers: *how does the signal behave?* Who hears whom. Whether
transmissions collide. What the broadcast domain is. It is about protocol
behaviour, and it may bear no resemblance whatever to the cabling.

Both descriptions are correct simultaneously. Nearly every confusing statement about
topology comes from someone applying a fact about one to the other.

## The canonical example

Twelve computers, each with a cable to a central box. What is the topology?

**Physically it is a star.** Unambiguously — look at the cable tray.

**Logically it depends entirely on what the box is.**

**If the box is a hub:** the hub is an electrical repeater that copies every
incoming signal to every other port. Every station hears every transmission.
Simultaneous transmissions collide. CSMA/CD arbitration is required. The bandwidth
is shared among all twelve.

That is a **bus**, exactly — logically identical to twelve stations tapped onto one
coaxial cable. The physical star is a wiring convenience with no logical
consequence.

**If the box is a switch:** the switch reads the destination address and forwards
the frame only to the port where that address lives (Chapter 17). Stations do not
hear each other's traffic. Simultaneous transmissions between different port pairs
proceed in parallel. Each port has its own collision domain, and full duplex becomes
possible because there is nobody to collide with.

That is a set of **point-to-point links**, logically. Not a bus at all.

**Nothing about the cabling changed and everything about the network's behaviour
did.** This is the single most important observation in the chapter, and it is why
replacing a hub with a switch produces an order-of-magnitude improvement that no
cable change could deliver.

## More cases worth working

**Wi-Fi.** Physically a star: every client associates with the access point and
there is no client-to-client cabling. Logically a **shared bus**: all clients share
one half-duplex radio channel, all can hear each other's transmissions (usually),
and a medium access protocol is mandatory. Chapter 44's CSMA/CA exists precisely
because the logical topology is a bus regardless of the physical star.

This is why Wi-Fi capacity does not scale with client count the way switched
Ethernet does, and it surprises people who reason from the physical picture.

**Token Ring, as actually deployed.** Physically a star — every station cabled to a
central Multistation Access Unit. Logically a **ring**, because the MAU internally
connected the ports in a loop so the token circulated through each station in turn.
The physical star was adopted for exactly the fault-isolation reasons §11.2 gives,
and the logical ring was preserved for its deterministic access.

Two topologies, one installation, and describing it as either alone is incomplete.

**PON.** Physically a **tree** — one fibre from the head end to a passive splitter,
then one fibre to each home. Logically a **bus downstream** (the head end broadcasts
and every terminal receives everything, which is why downstream is encrypted —
Chapter 49 §49.3) and a **TDM star upstream** (each terminal transmits in an assigned
slot). Three descriptions, all correct, each answering a different question.

**A VLAN.** Physically whatever the cabling is. Logically, a set of ports that
constitute one broadcast domain — potentially spanning several switches in different
rooms, and potentially excluding the port next to it on the same switch (Chapter 20).
The logical topology is defined entirely by configuration and can be changed without
touching a cable.

**A VXLAN overlay.** Physically a leaf-spine fabric of routed links. Logically a
flat Ethernet segment spanning three data centres in different cities, which share
no cables at all (Chapter 67 §67.2). The gap between the two descriptions is at its
widest here, and exploiting that gap is the entire point of the technology.

## Why the gap keeps widening

Notice the trajectory. In 1985 physical and logical topology were nearly the same
thing — a coaxial bus was a bus in both senses. Each subsequent technology has
pushed them further apart:

| Technology | What it decoupled |
|---|---|
| Hub → switch | Logical bus → logical point-to-point, same cabling |
| VLAN | Broadcast domain → independent of physical port location |
| MPLS | A circuit → over a packet-switched physical network |
| VXLAN / GENEVE | An Ethernet segment → over a routed fabric, across sites |
| SDN | The control plane → off the devices entirely |
| Cloud VPC | The whole network → into a configuration document |

**The whole modern practice of network virtualisation is the systematic exploitation
of the gap between physical and logical topology.** Unit XIV is that sentence,
expanded, and recognising it here makes those chapters feel like a continuation
rather than a new subject.

## The operational consequence

This is why Chapter 53 §53.1 insists on **three diagrams rather than one**.

- The **L1 physical** diagram answers "what is plugged into what" and is what you
  want at 03:14 with a torch.
- The **L2 logical** diagram answers "what is in which broadcast domain" and may
  look nothing like L1.
- The **L3 routed** diagram answers "how does traffic get between segments".

A single diagram attempting all three is unreadable and wrong, because the three
topologies genuinely differ. Producing one diagram and calling it "the network
diagram" is the most common documentation failure, and it costs time in exactly the
situations documentation exists for.

## Diagnostic value

The distinction is a troubleshooting tool, not merely a vocabulary point.

When a fault is reported, ask **which topology the symptom belongs to**:

| Symptom | Topology |
|---|---|
| No link light | Physical |
| Link up, no traffic | Logical (wrong VLAN, STP blocking, 802.1X) |
| Two devices on adjacent ports cannot communicate | Logical — same physical switch, different VLAN |
| Two devices in different buildings communicate fine | Logical — same VLAN over a trunk |
| Traffic taking an unexpected path | Logical — STP root placement, or routing |

The second and third rows are where the distinction earns its keep. "They are on
the same switch, so they must be able to talk to each other" is a physical-topology
inference applied to a logical-topology question, and it is wrong (Lab 05 §Part 1
demonstrates it deliberately).

## What breaks here

**Reasoning about collisions from the physical star.** A hub-based star has one
collision domain; a switched star has one per port. The cabling does not tell you
which.

**Assuming same switch means same network.** VLANs make this false, and it is the
single most common surprise for people meeting VLANs for the first time.

**Producing one diagram.** It will be either unreadably dense or silently wrong
about one of the three layers.

**Wireless capacity planned from a physical star.** The logical topology is a shared
half-duplex bus, and capacity planning from the physical picture over-estimates
badly. Chapter 45 §45.3.

> **Network+ note.** Objective 1.6 explicitly distinguishes physical from logical
> topology, and the exam tests it with scenarios of exactly the hub-versus-switch
> form. The transferable version: **ask which question a topological fact answers —
> where the cables go, or how the signal behaves — before applying it.**
