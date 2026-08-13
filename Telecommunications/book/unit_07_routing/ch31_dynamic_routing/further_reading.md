# Chapter 31 — Further Reading

## Primary sources

**Dijkstra, E. W. (1959). "A Note on Two Problems in Connexion with Graphs."
*Numerische Mathematik*, 1.**
**Two and a half pages.** Read it. The algorithm your routers run, as originally stated,
without a computer in sight.

**RFC 1058 — Hedrick, C. (1988). *Routing Information Protocol.***
Unusually honest: it documents existing practice, catalogues RIP's limitations frankly,
and recommends it only where it fits. Short, and a good model for how to specify something
you did not design.

**RFC 2453 — Malkin, G. (1998). *RIP Version 2.***
The version that carries the mask.

**RFC 2328 — Moy, J. (1998). *OSPF Version 2.***
**244 pages, and the definitive text.** Read §7 (neighbour and adjacency), §10 (the
neighbour state machine), §12 (LSAs) and §16 (the SPF calculation). The state machine in
§10 is what §31.3's diagnostic table summarises, and reading it once makes the stuck-state
diagnoses obvious rather than memorised.

**RFC 5340 — Coltun, R. et al. (2008). *OSPF for IPv6* (OSPFv3).**
What had to change, and why IS-IS did not need an equivalent.

**RFC 7868 — Savage, D. et al. (2016). *Cisco's EIGRP.***
DUAL and the feasibility condition, published twenty-two years after the protocol shipped.
§4.5 on the feasibility condition is the part worth reading.

**Garcia-Luna-Aceves, J. J. (1993). "Loop-Free Routing Using Diffusing Computations."
*IEEE/ACM Transactions on Networking*, 1(1).**
The original DUAL paper. The proof that the feasibility condition guarantees loop freedom
is worth working through — it is short and it is the most satisfying result in this
chapter.

**ISO/IEC 10589 — *Intermediate System to Intermediate System.***
IS-IS. Also RFC 1195 for the IP extensions and RFC 5308 for IPv6.

**RFC 5286 — Atlas, A. & Zinin, A. (2008). *Basic Specification for IP Fast Reroute:
Loop-Free Alternates.***
Precomputing the backup next hop.

## Books

**Moy, J. (1998). *OSPF: Anatomy of an Internet Routing Protocol.* Addison-Wesley.**
**By the protocol's author**, and unusually candid about what he would change. The best
explanation of *why* OSPF is shaped as it is.

**Doyle, J. & Carroll, J. (2005). *Routing TCP/IP, Volume 1*, 2nd ed. Cisco Press.**
**The reference for this unit.** Chapters 4–9 cover RIP, EIGRP and OSPF in depth with
worked convergence examples and the failure modes. Chapter 5's treatment of
count-to-infinity is better than this one.

**Perlman, R. (1999). *Interconnections*, 2nd ed.**
Chapters 12–14 on routing, from someone who designed one of the two link-state protocols.
Her comparison of IS-IS and OSPF is the fair one, written by the person with the most
reason to be unfair.

**White, R., Slice, D. & Retana, A. (2005). *Optimal Routing Design.* Cisco Press.**
The design half of §31.4 — areas, summarisation, convergence tuning — with the reasoning
rather than the rules.

**Medhi, D. & Ramasamy, K. (2017). *Network Routing*, 2nd ed.**
The formal treatment: convergence proofs, complexity, and the algorithms as algorithms.

## Applied

**Build the labs. This chapter is not learnable from reading.**

**FRRouting** (frrouting.org) or **BIRD**, in containers or VMs. Free, real
implementations of RIP, OSPF, IS-IS and BGP. **Containerlab** makes a ten-router topology
a single YAML file.

**Exercise F1 is the one to do first:** four routers running RIP, break a link, capture the
updates, and **watch the metric count to infinity.** Nothing else makes §31.2 real. It
takes about ten minutes to build and eight minutes to observe.

**Then F2:** the same topology with OSPF, and measure convergence. Then add BFD and
measure again. **The three numbers — minutes, 40 seconds, sub-second — are the whole
argument of this unit in one table you produced yourself.**

**`show ip ospf neighbor`** until the state names are automatic. Then deliberately break
an adjacency in each of the eight ways in §31.3's checklist and confirm the symptom each
time.

**Create an MTU mismatch on purpose** and watch the adjacency stick at ExStart. This is
the single most-diagnosed OSPF fault, and having seen it once you will recognise it
instantly.

**`show ip ospf database`** and read the LSAs. Identify a Type 1, a Type 2 from the DR, a
Type 3 crossing an area boundary, and a Type 5 external. The abstraction becomes concrete
immediately.

**Wireshark filters:** `ospf`, `ospf.hello`, `rip`, `eigrp`. OSPF hellos are readable and
show the adjacency requirements — area, timers, authentication — directly in the packet,
which is a fast way to find a mismatch.

**Lab 18** in this book's [labs/](../../../labs/) directory builds a two-area OSPF network
with summarisation, then flaps a link in the non-backbone area and demonstrates — with SPF
counters — that the other area does not recompute. **Lab 19** creates a redistribution
loop deliberately and fixes it with route tagging.

## For the certification-minded

Objective 2.2 expects routing protocols, their categories and their characteristics.

Eight things worth over-learning:

1. **Distance vector exchanges distances; link state floods topology.**
2. **RIP: hop count, max 15, 16 = unreachable, 30 s updates, AD 120.**
3. **OSPF: link state, cost from bandwidth, AD 110, `224.0.0.5`, hello 10 / dead 40.**
4. **EIGRP: advanced distance vector, AD 90 internal / 170 external, DUAL.**
5. **The five loop-prevention mechanisms**: split horizon, poison reverse, route
   poisoning, holddown, triggered updates.
6. **Area 0 is the backbone**; every area connects to it; inter-area traffic passes
   through it.
7. **ABR** joins areas; **ASBR** joins OSPF to another protocol.
8. **IGP versus EGP** — OSPF, EIGRP, IS-IS and RIP are interior; **BGP is the exterior
   protocol** (Chapter 32).

Expect a "which protocol is this?" question from a description, and a table-reading
question with administrative distances.

And the two things worth more than the objective:

**ExStart means MTU mismatch.** You will meet this in a job.

**Convergence time is dominated by failure detection, not by computation.** Every
convergence complaint in a real network resolves to this, and the fix is BFD rather than
tuning the protocol.
