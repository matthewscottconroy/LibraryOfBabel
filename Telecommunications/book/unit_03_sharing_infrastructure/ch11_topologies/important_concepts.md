# Chapter 11 — Important Concepts

The combinatorics of full mesh *(§11.1)* — *n*(*n*−1)/2 links and *n*−1
interfaces per device. Quadratic in links, linear in per-device cost, and absurd
beyond a few dozen nodes. For twenty billion devices it would need about 2 × 10²⁰
links.

Full mesh as the unaffordable ideal *(§11.1)* — It provides maximum resilience,
one-hop latency, no shared resource and no single point of failure. Every other
topology is a way of buying a fraction of those properties at a fraction of the
cost, which makes topologies points on a cost/resilience curve rather than a list
of shapes.

Single point of failure *(§11.1)* — A component whose loss disconnects something
previously connected. Counting them in a design is a five-minute exercise that finds
most resilience problems.

**The star's arithmetic** *(§11.1)* — *n* links for *n* devices, one interface each:
linear rather than quadratic. The saving is bought entirely by accepting one
single point of failure at the centre, which is why redundancy concentrates there.

**Partial mesh** *(§11.1)* — What real networks are. The design question is not "how
much redundancy" but "which failures must this survive?", which is a requirements
question with a cost-based answer.

Cost is not only links *(§11.1)* — Interfaces cost money; **paths** cost far
more than links (two fibres in one duct are barely dearer than one and share a
fate); and complexity costs operationally, because a topology nobody can reason
about fails badly at 3 a.m.

**Bus** *(§11.2)* — One shared medium, tapped, terminated at both ends. Minimal
cable; one collision domain; a break or a missing terminator takes down
everything; fault isolation requires walking the cable. Dead in the LAN, alive in
CAN bus, Modbus RTU, PROFIBUS and RS-485 — where the disadvantages are irrelevant
and wiring weight matters.

**Ring** *(§11.2)* — Each station connects to two others in a loop. Deterministic
access with a bounded worst-case delay, and graceful degradation under load,
both of which Ethernet cannot promise. A single ring breaks entirely on one failure.

**SONET's dual ring** *(§11.2)* — Two counter-rotating paths; on a cut the adjacent
nodes **wrap** traffic back the other way in **under 50 ms**. A design requirement
met in 1988, against spanning tree's original 30–50 seconds — three orders of
magnitude, sustained for two decades.

**Star** *(§11.2)* — Every station to a central device. **Fault isolation** is the
property that beat bus; changes are non-disruptive; the centre is a total single
point of failure and is therefore where all the redundancy goes.

**Tree** *(§11.2)* — Stars of stars. Scales by adding levels and permits
summarisation, at the cost that every internal node is a single point of failure
for everything beneath it. Remedied by dual-homing, which introduces the loops
Chapter 19 must manage.

**Physical topology** *(§11.3)* — Where the cables go. What a floor plan shows.

**Logical topology** *(§11.3)* — How the signal behaves: who hears whom, what
collides, what the broadcast domain is. May bear no resemblance to the cabling.

**The hub/switch example** *(§11.3)* — Twelve stations cabled to a central box is
physically a star either way. With a hub it is **logically a bus** — shared
bandwidth, collisions, CSMA/CD. With a switch it is **logically point-to-point
links** — per-port bandwidth, no collisions, full duplex. Nothing about the cabling
changed and everything about the behaviour did.

**Wi-Fi's two topologies** *(§11.3)* — Physically a star; logically a shared
half-duplex bus, which is why CSMA/CA is mandatory and why capacity does not scale
with client count the way switched Ethernet does.

**The widening gap** *(§11.3)* — Hub→switch, VLANs, MPLS, VXLAN, SDN and cloud VPCs
each pushed physical and logical topology further apart. The whole modern practice
of network virtualisation is the systematic exploitation of that gap, which is
what Unit XIV is about.

**Why three diagrams** *(§11.3)* — L1 physical, L2 logical, L3 routed genuinely
differ, so one diagram attempting all three is unreadable or silently wrong.
Chapter 53 §53.1's discipline follows directly.

**Hierarchy's three mechanisms** *(§11.4)* — **Aggregation** (many flows combine
upward, compounding statistical multiplexing gain), **summarisation** (detail hidden
across a boundary), and **containment** (blast radius bounded by structure). These
are why hierarchy is necessary rather than merely tidy.

**Access layer** *(§11.4)* — Ports, PoE, VLAN assignment, port-level security, and
the QoS trust boundary. The only layer touching untrusted devices, which is why the
hardening checklist lives here.

**Distribution layer** *(§11.4)* — Aggregation, inter-VLAN routing, **address
summarisation upward**, policy, and first-hop redundancy. Where the design
intelligence lives.

**Core layer** *(§11.4)* — Move packets between distribution blocks fast and do
nothing else. Anything the core does, it does to all traffic, so policy belongs
elsewhere.

**Collapsed core** *(§11.4)* — Distribution and core merged into two tiers.
Correct for a single building or campus up to a few thousand users, which describes
most enterprise networks. Recommending it is usually right rather than a compromise.

**North–south versus east–west** *(§11.4)* — Three-tier assumes clients reaching
servers and the Internet, so capacity concentrates toward the core and
oversubscription is deliberate. Distributed applications inverted this around 2010:
one user request triggers dozens of internal service calls, and server-to-server
traffic came to dominate by an order of magnitude.

Leaf-spine as a response *(§11.4)* — Every leaf to every spine, nothing leaf to
leaf, so all servers are two hops apart; routed with ECMP so no links are blocked.
Structurally a **complete bipartite graph** — a partial mesh, not a tree. A response
to a change in traffic direction, not a general improvement: three-tier remains
correct where traffic is genuinely north–south.

Topology follows traffic pattern *(§11.4)* — The chapter's design conclusion,
and a requirements question rather than a technology preference.
