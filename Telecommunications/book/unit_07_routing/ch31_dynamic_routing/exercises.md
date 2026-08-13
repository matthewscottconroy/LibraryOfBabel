# Chapter 31 — Exercises

## A. Recall

**A1.** State the distance-vector algorithm in four steps.

**A2.** State the link-state algorithm in three steps.

**A3.** Give RIP's metric, maximum, update interval, transport, multicast address and
administrative distance.

**A4.** Name the five loop-prevention mechanisms in distance-vector protocols and say what
each does.

**A5.** List the OSPF neighbour states in order, and give the fault each of Init, ExStart
and Loading indicates.

**A6.** Give OSPF's administrative distance, protocol number, multicast addresses, hello
interval and dead interval.

**A7.** State the three OSPF area rules.

**A8.** What do LSA types 1, 2, 3 and 5 carry, and what is each one's scope?

## B. Apply

**B1.** Four routers in a line, A–B–C–D, each with one attached network, RIP with 30 s
updates. Draw the distance tables after each round until convergence, and state the
elapsed time.

**B2.** Work the count-to-infinity for the three-router case of §31.2 through eight
rounds, giving each router's metric at each step. State the total elapsed time to reach
16 with RIP's timers.

**B3.** For this topology, run Dijkstra from **A** and give the cost, path and next hop to
every other node. Show each step of the algorithm.

```
        A ──1── B ──4── C
        │       │       │
        3       2       1
        │       │       │
        D ──5── E ──2── F
```

**B4.** Repeat B3 from **F**.

**B5.** Compute the number of adjacencies on a broadcast segment with 3, 6, 12 and 20
routers, with and without a DR.

**B6.** For each OSPF symptom, give the most likely cause:

(a) stuck in ExStart  (b) stuck in Init  (c) stuck in Two-Way between two routers on a
LAN  (d) no neighbour at all  (e) adjacency flaps every few minutes  (f) all paths appear
to have equal cost

**B7.** A network has 200 routers. Compare, for a link failure in the far corner: the
number of routers that recompute with one area versus with five areas and summarisation.

## C. Analyse

**C1.** Explain precisely why a distance-vector router cannot detect that a route loops
back through itself, and name the three different fixes the successor protocols use.

**C2.** Work through why split horizon prevents a two-router loop and fails on a
three-router ring. Draw both.

**C3.** Explain EIGRP's feasibility condition, prove informally why it guarantees loop
freedom, and explain why it makes split horizon, poison reverse and holddown unnecessary.

**C4.** Holddown is described as the crudest of the four fixes. Explain what it costs and
compare it with classic spanning tree's timers (Chapter 19 §19.2).

**C5.** "Consensus by identical computation." Explain how link state achieves agreement
between routers without any agreement protocol, and identify the same technique elsewhere
in this book.

**C6.** OSPF is described as "link state within an area and distance vector between them".
Justify this, and explain how the area rules compensate for the resulting loop risk.

**C7.** Explain why the real motivation for areas is churn rather than CPU or memory.

**C8.** "A summary does not change when a component of it changes." Explain why this is
the strongest argument for summarisation, and connect it to Chapter 26 §26.4's addressing
discipline.

**C9.** Explain how mutual redistribution between two protocols creates a loop, and give
the three mechanisms that prevent it.

**C10.** Compare the convergence of RIP, OSPF with default timers, OSPF with BFD, and
EIGRP after a link failure. Identify the dominant term in each.

## D. Design

**D1.** Design the OSPF area structure for a company with a head office, two regional
hubs and eighteen branches. Specify areas, types, where summarisation happens, and the
address plan that makes summarisation possible.

**D2.** For the semester project's network, choose a routing protocol and justify it
against the tests of Chapter 30 §30.4. Then write the complete configuration.

**D3.** A network runs EIGRP at head office and OSPF at a recently-acquired site. Design
the interconnection: where redistribution happens, what is filtered, how loops are
prevented, and what you would monitor.

**D4.** Design the failure detection for a network where a 40-second outage is
unacceptable and sub-second hellos have caused adjacency flaps. Justify your choice.

**D5.** An OSPF network has one area and 180 routers, and users report brief outages
several times a day with no obvious cause. Diagnose the likely mechanism and write the
remediation plan.

## E. Troubleshoot

**E1.** Two directly-connected routers will not form an adjacency. Give the eight things
to check, in order.

**E2.** An adjacency reaches ExStart and stops. One command identifies it — which, and
what are you looking for?

**E3.** After adding a router to a LAN, the DR did not change even though the new router
has a higher priority. Explain.

**E4.** A branch router's routing table has 780,000 entries and its CPU is pinned.
Diagnose and give the one-line fix.

**E5.** A link in Area 3 flaps every few minutes. Routers in Area 1 show constant SPF
runs. What is missing?

**E6.** After configuring `area 2 range`, nothing changed. Give two causes.

**E7.** Traffic to a summarised range is black-holed when one component subnet is down.
Explain and give the fix.

**E8.** After connecting OSPF and EIGRP at two points, both protocols' tables begin
oscillating. Explain the mechanism precisely.

**E9.** An OSPF adjacency formed with a device nobody recognises. What was not
configured, and what are the two lines that prevent it?

**E10.** `debug ip rip` shows a metric climbing 2, 4, 6, 8. Name the phenomenon and state
what will stop it and when.

## F. Extend

**F1.** Build a four-router lab with RIP. Break a link and capture the updates. Watch the
count-to-infinity happen and time it. Then enable split horizon and repeat.

**F2.** Build the same topology with OSPF, break the same link, and measure convergence.
Then enable BFD and measure again. Tabulate all three.

**F3.** Implement Dijkstra's algorithm in a language of your choice, feed it the topology
from B3, and verify your hand computation.

**F4.** Configure a two-area OSPF network, then flap a link in the non-backbone area and
confirm — with `show ip ospf statistics` or equivalent — that the other area does not run
SPF.

**F5.** Read RFC 2328's §16 (the SPF calculation) and compare it with the textbook
Dijkstra. Identify what OSPF adds and why.

**F6.** Deliberately create a redistribution loop in a lab, observe the symptoms, then fix
it with route tagging. Document both states.
