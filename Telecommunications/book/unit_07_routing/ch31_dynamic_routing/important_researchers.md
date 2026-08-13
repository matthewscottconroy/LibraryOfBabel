# Chapter 31 — The People

**Richard Bellman (1920–1984) and Lester Ford Jr. (1927–2017).** The Bellman–Ford
algorithm, from operations research in the 1950s, is what §31.1's distance-vector protocol
is — running distributed, asynchronously, with no coordinator, on hardware neither of them
imagined.

**Bellman** also gave us dynamic programming, and the name is a small joke worth knowing:
he chose "dynamic programming" partly because it was impossible for his RAND Corporation
funders to object to. Research-funding politics has always shaped what things are called.

**The algorithm's property that matters here** is that it converges without any node
having global knowledge — which is exactly why it was reachable for a 1969 network of
machines with kilobytes of memory, and exactly why it has the defect of §31.2.

**Edsger W. Dijkstra (1930–2002).** The shortest-path algorithm, devised in about twenty
minutes in 1956 at a café terrace in Amsterdam while shopping with his fiancée, to
demonstrate the ARMAC computer. He published it in 1959 in a **two-and-a-half page**
paper.

His own account is worth quoting for its ordinariness: *"One of the reasons that it is so
nice was that I designed it without pencil and paper... Eventually, that algorithm became,
to my great amazement, one of the cornerstones of my fame."*

**It runs on every OSPF and IS-IS router on Earth**, several times a second.

Dijkstra appears twice in this book — here and in Chapter 21, for the layering argument of
the "THE" multiprogramming system — which is an unusual range, and both contributions are
about the same thing: **structuring a problem so a person can reason about it.**

**Charles Hedrick.** Rutgers, and the author of **RFC 1058** (1988) — the RIP
specification, written six years after RIP was already in universal use through BSD's
`routed`.

**The document's honesty is unusual and worth reading.** Hedrick states plainly that he is
describing existing practice rather than proposing a design, catalogues RIP's limitations
— the 15-hop limit, the hop-count metric, the slow convergence — and recommends it anyway
for the networks where it fits. **Specifying what already exists, accurately, including
its faults, is a genuine service** and it is rarer than it should be.

**John McQuillan, Ira Richer and Eric Rosen.** BBN, and the **ARPANET's 1979 transition
from distance vector to link state** — the first deployment of the idea.

The context matters: the ARPANET had run a distance-vector protocol for a decade and had
suffered from exactly §31.2's problems, at a scale where they were operationally painful
rather than theoretical. **Link state was invented because distance vector had visibly
failed in production**, not because someone preferred it aesthetically.

**Eric Rosen** went on to MPLS (Chapter 51) and to a great deal of the routing
architecture work at Cisco.

**Radia Perlman (b. 1951).** **IS-IS**, and — with the ARPANET work above — the
formalisation of link-state routing.

IS-IS is an OSI protocol (Chapter 22) that outlived OSI, and it survives for a reason
worth extracting: **it was designed to be protocol-independent.** IS-IS carries
reachability for whatever address family you configure, so when CLNP disappeared it
carried IPv4, and when IPv6 arrived it carried IPv6 **with no protocol change at all** —
while OSPF required an entirely new version, OSPFv3.

**Generality designed in for one reason paid off for a completely different one.** It is
the best argument in this book for building something more general than the immediate
requirement demands.

A large share of the world's service-provider backbones run IS-IS rather than OSPF, and
the reason is mostly this: it scales further and it took IPv6 in its stride.

**John Moy (b. 1955).** The author of **OSPF** — RFC 1131 (1989) through RFC 2328 (1998) —
and its chief advocate through a decade of standardisation.

The "Open" in the name is a deliberate political statement. Cisco's **IGRP** was
proprietary, and a customer running it could not use another vendor's routers. OSPF was
built to be an open alternative, and **it succeeded — it is the most widely deployed
interior gateway protocol in enterprise networks.**

His book *OSPF: Anatomy of an Internet Routing Protocol* is the reference, and it is
unusually candid about which decisions he would make differently.

**J. J. Garcia-Luna-Aceves (b. 1955).** **DUAL** — the Diffusing Update Algorithm — and
the **feasibility condition** of §31.2.

His result is the elegant one in this chapter. Where RIP needs split horizon, poison
reverse, holddown and triggered updates — four heuristics, none complete — DUAL uses a
single arithmetic test that **guarantees loop freedom at every instant**, not merely after
convergence, and needs no timers at all.

**EIGRP is Cisco's implementation of it**, and EIGRP's sub-second convergence is a direct
consequence. Cisco published EIGRP as an informational RFC in 2016 (RFC 7868) after
twenty-two years of it being proprietary — by which time the industry had standardised on
OSPF and IS-IS, which is a case study in how proprietary advantages expire.

**Dave Katz and Dave Ward.** **BFD**, again — Chapter 30's notes give the argument.
§31.4's convergence table is the case for it: the 40-second dead timer is not a property of
link-state routing, it is an artefact of OSPF's hello design, and factoring failure
detection out of the routing protocol fixes it for every protocol at once.

**Alia Atlas, Alex Zinin and the IP Fast Reroute working group.** **Loop-free alternates**
(RFC 5286) and the fast-reroute work of §31.4 — precomputing a backup next hop so failover
is a table flip rather than a recomputation.

**The same idea as RSTP's alternate port** (Chapter 19 §19.3), arrived at independently at
a different layer, and the convergence: **when the answer must be produced faster than it
can be computed, compute it in advance.**
