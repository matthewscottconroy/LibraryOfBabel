# Chapter 11 — Further Reading

## Primary sources

**Baran, P. (1964). *On Distributed Communications*, RAND Memoranda RM-3420 to
RM-3428.**
Volume I, *Introduction to Distributed Communications Networks*, contains the
centralised/decentralised/distributed diagram and the survivability analysis behind
§11.1's resilience argument. Freely available from RAND. Short, readable, and the
plots showing survivability against node loss are worth seeing in the original.

**Clos, C. (1953). "A Study of Non-blocking Switching Networks." *Bell System
Technical Journal* 32(2): 406–424.**
The mathematics underlying leaf-spine, written seventy years ago for telephone
exchanges. The non-blocking condition — that the middle stage must have enough
elements — is what determines a modern fabric's spine count.

**Al-Fares, M., Loukissas, A. & Vahdat, A. (2008). "A Scalable, Commodity Data
Center Network Architecture." *ACM SIGCOMM Computer Communication Review* 38(4):
63–74.**
The fat-tree paper. Its argument — many cheap identical switches beat a few
expensive ones, given enough paths — reshaped data-centre design. Read §2 for the
statement of what was wrong with the hierarchical approach.

**Greenberg, A. et al. (2009). "VL2: A Scalable and Flexible Data Center Network."
*ACM SIGCOMM*.**
Microsoft's contemporaneous answer, with real measurements of data-centre traffic
patterns. §2's traffic characterisation is the empirical evidence for §11.4's
east–west claim.

**Telcordia GR-253-CORE, *SONET Transport Systems: Common Generic Criteria*.**
Where the 50 ms protection requirement is written down. Worth knowing it is a
specified requirement rather than an emergent property.

## Books

**Oppenheimer, P. (2010). *Top-Down Network Design*, 3rd ed. Cisco Press.**
The best book on the process §11.4 and Chapter 72 describe: requirements first,
then logical design, then physical. Chapters 5 and 6 cover topology selection
against requirements, which is exactly the framing this chapter argues for.

**Cisco Systems, *Campus LAN and Wireless LAN Design Guide* (current edition).**
Free. The three-tier and collapsed-core models in their canonical form, with the
oversubscription ratios and the reasoning. Vendor material, and honest about the
tradeoffs.

**Dutt, D. G. (2019). *Cloud Native Data Center Networking.* O'Reilly.**
Leaf-spine from first principles, including why the fabric is routed rather than
switched and what that eliminates. Chapters 2 and 3 are the clearest available
treatment of §11.4's east–west argument and its consequences.

**Tanenbaum, A. S., Feamster, N. & Wetherall, D. (2021). *Computer Networks*,
6th ed. Pearson.**
Chapter 1's topology taxonomy is the standard textbook treatment, and its coverage
of the classical shapes is more detailed than §11.2's.

**Perlman, R. (1999). *Interconnections: Bridges, Routers, Switches, and
Internetworking Protocols*, 2nd ed. Addison-Wesley.**
Opinionated, funny, and correct. Her arguments about when to bridge and when to
route are directly relevant to §11.4's Layer 2/Layer 3 boundary question, and she is
consistently sceptical of the large flat networks her own spanning tree enables.

## Historical

**Abbate, J. (1999). *Inventing the Internet.* MIT Press.**
Good on the ARPANET's topology decisions and on the relationship between Baran's
survivability argument and what was actually built — which is less direct than the
popular account suggests.

**Hafner, K. & Lyon, M. (1996). *Where Wizards Stay Up Late.* Simon & Schuster.**
The ARPANET's construction, including the practical topology choices made under
budget constraints rather than from theory.

## Applied

**Any carrier's metro Ethernet or SONET service description.**
Reading how a ring is sold — protected versus unprotected, and the price difference
— makes §11.2's resilience-per-link argument commercially concrete.

**RFC 7938, *Use of BGP for Routing in Large-Scale Data Centers*.**
How leaf-spine fabrics are actually run, using BGP as an interior protocol. Relevant
to Chapter 67 and a good example of a protocol being repurposed far from its
original domain.

## Tools

**Draw your own network's three diagrams.** Lab 11 assigns this against someone
else's network, which is harder and more instructive. The exercise of producing L1,
L2 and L3 separately is where §11.3's distinction stops being abstract.

**Containerlab or GNS3**, for building the topologies of Exercises 11.7–11.10 and
measuring the hop counts and failure behaviour directly rather than reasoning about
them.

## For the certification-minded

Objective 1.6 covers topology types, physical versus logical, and the three-tier and
collapsed-core models. Objective 3.5 covers data-centre architecture including
spine-and-leaf.

Three things worth over-learning:

1. **Physical versus logical**, tested with hub-versus-switch scenarios. Ask which
   question a topological fact answers before applying it.
2. **The three-tier layers and their roles**, particularly that the distribution
   layer is where routing and policy live and the core does neither.
3. **Star topology's dominance is a fault-isolation argument.** The exam asks which
   topology is most fault-tolerant and which is easiest to troubleshoot, and those
   are different questions with different answers.
