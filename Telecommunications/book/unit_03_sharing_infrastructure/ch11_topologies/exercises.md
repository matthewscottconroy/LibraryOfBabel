# Chapter 11 — Exercises

## A. Recall

**A1.** Compute the link count and per-node interface count for a full mesh of 8,
25 and 200 nodes.

**A2.** For each topology — bus, ring, star, tree — state the number of links for
*n* nodes and identify the single points of failure.

**A3.** Twelve computers cabled to a central box. State the physical topology and
both possible logical topologies, and what determines which.

**A4.** Name the three layers of the three-tier model and state each layer's
principal job in one clause.

**A5.** What traffic pattern does the three-tier model assume, and what pattern
broke it?

## B. Apply

**B1.** Four sites must interconnect. Tabulate: star from one site, ring, ring plus
one diagonal, and full mesh — giving link count, single points of failure, and the
failures each survives. Recommend one for a company where a site outage costs
£4,000 per hour, and justify against that figure.

**B2.** A campus has 6 buildings, each with 4 access switches. Design a three-tier
topology with every access switch dual-homed and a redundant core. Count: total
devices, total inter-switch links, and total switch ports consumed by
infrastructure links.

**B3.** Repeat 11.7 as a collapsed core. State the saving in devices, links and
ports, and state what capability was given up.

**B4.** A leaf-spine fabric has 16 leaf switches and 4 spine switches. Compute the
number of fabric links. Each leaf has 48 × 25 Gb/s server ports and 4 × 100 Gb/s
uplinks. Compute the oversubscription ratio and state whether it is appropriate for
an east–west-dominated workload.

**B5.** In the fabric of 11.9, how many hops separate two servers on different
leaves? On the same leaf? Compare with a three-tier design in which the two racks
attach to different distribution blocks.

**B6.** Two switches are connected by two fibres running in the same duct.
Compute the single-point-of-failure count as a naive link count suggests, then
recount using shared-fate reasoning. What would make the redundancy genuine, and
roughly what would it cost relative to the original?

**B7.** A hub-based star of 16 stations shares 100 Mb/s. Compute the average
bandwidth per station under full load. Replace the hub with a switch and recompute.
State which topology changed and which did not.

## C. Analyse

**C1.** Full mesh is described as unaffordable rather than bad. Defend that
framing: list the properties full mesh provides, and for each of star, ring and tree
state which of those properties it retains and which it sacrifices. Then state
which topology gives the best resilience per link added, with the arithmetic.

**C2.** SONET's dual ring achieved 50 ms protection switching in 1988; spanning
tree took 30–50 seconds. Explain mechanistically why the ring can be so much faster,
identifying what information each system must gather before it can act and how far
that information must travel.

**C3.** §11.3 argues that the gap between physical and logical topology has
widened steadily. Trace that widening through six technologies, and for each state
what was decoupled from what. Then predict what the next decoupling might be and
what would make it valuable.

**C4.** A colleague proposes leaf-spine for a four-floor office building with 600
users, arguing that it is the modern design. Construct the counter-argument.
Address the traffic pattern, the cost, and the operability constraint, and state the
circumstance under which they would be right.

**C5.** Show that hierarchy's three mechanisms — aggregation, summarisation,
containment — appear in each of: IP subnetting, OSPF areas, DNS, and the three-tier
LAN. For each, identify what is aggregated, what is summarised across the boundary,
and what is contained.

## D. Design

**D1.** A manufacturer occupies four buildings on one site:

- **Building A** — head office, 3 floors, 240 users, 2 comms rooms per floor.
- **Building B** — production, 1 floor, 60 users, 40 machine controllers requiring
  bounded worst-case latency under 10 ms.
- **Building C** — warehouse, 20 users, 30 wireless devices.
- **Building D** — gatehouse, 4 users.

Ducts exist between A–B and A–C. There is no duct to D. The company's DR
requirement is that a single fibre cut must not isolate any building.

Design the topology. State: the topology between buildings and why; the topology
within each building; where the Layer 2/Layer 3 boundary sits and why; whether you
propose three-tier or collapsed core and against what criterion; and how you satisfy
the DR requirement given the duct constraints. Identify every remaining single point
of failure and state whether you are accepting it, with reasons.

Address Building B's latency requirement explicitly, referring to §11.2's
determinism discussion.

## E. Troubleshoot

**E1.** An organisation has a three-tier network. Users report that file transfers
between two servers — in racks 3 and 4 of the same room, on the same access
switch — are slower than transfers between those servers and a client on another
floor.

Investigation shows:

- Both servers are on the same access switch, ports 12 and 14.
- They are in different VLANs.
- The Layer 3 boundary is at the distribution layer.
- Access-to-distribution uplinks are 2 × 10 Gb/s; server ports are 25 Gb/s.
- The distribution switches are also handling inter-VLAN routing for the whole
  floor.
- Utilisation on the access uplinks peaks at 78% during the transfers.

Explain the path the traffic actually takes and why it is longer than the physical
adjacency suggests. Identify which assumption of the three-tier model this workload
violates. Then give three remedies — one configuration change, one design change,
one architectural change — and state the cost and the appropriateness of each.
