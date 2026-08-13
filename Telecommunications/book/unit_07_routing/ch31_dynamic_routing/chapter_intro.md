# Chapter 31 — Dynamic Routing

There are two ways for routers to work out where things are, they were invented
about a decade apart, and the difference between them is one of the cleaner
illustrations in computer science of how the choice of algorithm determines the
shape of everything built on it.

## Distance vector: tell your neighbours what you know

Each router maintains a table of destinations and distances. Periodically, it tells
its directly connected neighbours the whole table: *"I can reach 10.1.0.0/16 at a
cost of 3."* A neighbour receiving this adds the cost of the link and considers
whether the result is better than what it has.

This is the **Bellman–Ford** algorithm, distributed. Its virtues are that it is
trivial to implement, requires almost no memory, and requires no router to
understand the topology at all — a router knows *distances* and *directions* and
nothing else, like someone navigating by signposts.

Its vice is the same fact. A router that knows only what its neighbours claim has no
way to verify a claim, and no way to detect that a neighbour's information came
originally from the router itself. This produces the **count-to-infinity problem**,
which §31.2 works through in detail with a worked example, and which is worth
studying carefully because it is the canonical instance of a whole family of
distributed-systems failures.

The short version: when a network fails, a router may learn a route to it *back from
a neighbour who learned it from them*, believe it, and advertise it onward with an
increased cost. The cost climbs by one on each exchange, slowly, while traffic loops
between the two routers. It converges eventually only because RIP defines 16 hops as
infinity and gives up.

The mitigations — split horizon, poison reverse, holddown timers, triggered updates
— each address part of the problem and none solves it completely. §31.2 explains why
a complete solution is not available within the distance-vector framework: **the
information needed to detect the loop is exactly the information the protocol throws
away.**

RIP is the classic instance and is essentially obsolete. It appears here because the
failure mode teaches something durable, and because EIGRP — which is not obsolete —
is a distance-vector protocol with a genuinely clever solution (the Diffusing Update
Algorithm) to precisely this problem.

## Link state: tell everyone what you see

The other approach inverts the flow of information.

Each router determines the state of its own directly attached links — which
neighbours it can see, and at what cost — and **floods that small fact to every
router in the area**. Not its table; its immediate observations. Every router
therefore accumulates the same set of observations, and from them builds an
identical map of the entire area.

Having a map, each router then computes the shortest path to everywhere using
**Dijkstra's algorithm**, published by Edsger Dijkstra in 1959 and reportedly worked
out in about twenty minutes in a café in Amsterdam while he and his fiancée were
having coffee.

The consequences are almost all improvements:

- **No count-to-infinity.** A router with a complete map can see that a path loops.
- **Fast convergence.** A change is flooded immediately; every router recomputes
  independently and in parallel. Seconds rather than minutes.
- **Richer metrics.** Cost can reflect bandwidth rather than hop count, so a
  four-hop gigabit path correctly beats a two-hop 10 Mb/s path — which RIP cannot
  express at all.
- **Loop-free by construction** within an area, since all routers compute from the
  same database.

The costs are real too: more memory (every router holds the full area database),
more CPU (Dijkstra runs on every change), and considerably more protocol complexity.
OSPF is a substantially harder protocol to implement and to debug than RIP.

§31.3 has you run Dijkstra by hand on a small topology. Do it. It takes fifteen
minutes and it converts OSPF from a set of configuration commands into a mechanism
you can reason about.

## Areas: hierarchy, again

The link-state cost that bites is the database. Every router in an area holds every
link-state advertisement, and runs Dijkstra over all of them. At a few hundred
routers this becomes uncomfortable.

OSPF's answer is **areas** — and it is the same answer as subnetting, and DNS
delegation, and the three-tier design: divide the problem and summarise across the
boundary. Routers within an area hold full detail of that area. Between areas, only
summaries cross, through a backbone area (area 0) to which all other areas must
connect.

This is Chapter 26's aggregation argument applied to topology rather than to
addresses, and it is why §31.4 insists that **address planning and area design must
be done together**. An area whose subnets cannot be summarised into one or two
prefixes leaks its full detail across the boundary and defeats the entire purpose.
This is the practical link between Chapter 27's address plan and this chapter's
routing design, and it is the thing most often got wrong in real networks — usually
because the address plan was written years earlier by someone who was not thinking
about OSPF.

## What this chapter does

§31.1 covers distance vector and RIP: the algorithm, the update mechanism, the
metric, and honest coverage of where it survives.

§31.2 works count-to-infinity through with a concrete four-router example, then
covers each mitigation and what it does and does not fix.

§31.3 covers link state: the LSA flooding process, the database, Dijkstra worked by
hand, and OSPF's adjacency formation including the DR/BDR election on multi-access
segments.

§31.4 covers convergence, areas, LSA types, summarisation, and how routing design
and addressing design constrain each other. It also covers IS-IS briefly, since it
is what most large service providers actually run, and EIGRP's DUAL.

## By the end you will be able to

- Explain the essential difference between distance vector and link state in terms
  of what information each router holds.
- Trace count-to-infinity through a small topology and explain why split horizon is
  insufficient.
- Execute Dijkstra's algorithm by hand and produce a shortest-path tree.
- Explain why OSPF converges faster than RIP, mechanistically.
- Explain what areas are for and design an area boundary that summarises.
- State the criteria for choosing between static, OSPF, IS-IS and EIGRP for a given
  network.
