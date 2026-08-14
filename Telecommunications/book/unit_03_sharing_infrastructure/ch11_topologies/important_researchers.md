# Chapter 11 — The People

**Paul Baran (1926–2011).** His 1960–64 RAND reports classified network topologies
into **centralised**, **decentralised** and **distributed**, and the diagram
distinguishing them is one of the most reproduced illustrations in networking. His
argument was that a centralised network is destroyed by removing its centre, a
decentralised one by removing a few nodes, and a distributed mesh survives the loss
of a substantial fraction — which was the point, given that his brief concerned
surviving a nuclear first strike. §11.1's cost/resilience framing is his argument
with the costs made explicit. Chapter 13 covers him properly.

**Leonard Kleinrock (b. 1934).** His 1962 MIT thesis and subsequent work supplied
the queueing analysis that lets you say what a given topology's delay will be under
a given load — turning topology from a qualitative comparison into a computation.
The ARPANET's first node went into his UCLA laboratory in 1969. See Chapters 3
and 13.

**Robert Metcalfe (b. 1946).** Ethernet began as a logical bus on a physical bus —
a shared coaxial cable with vampire taps. The subsequent move to a physical star
with 10BASE-T, while retaining the logical bus of the hub, and then to a logical
point-to-point arrangement with switching, is §11.3's canonical example and is
the clearest illustration in this book that physical and logical topology are
independent. Chapter 16 covers him.

**Olof Söderblom.** Swedish engineer who patented token ring in 1967 and
1969, several years before IBM's development of the technology. IBM licensed his
patents; so, after litigation, did a number of other manufacturers, and Söderblom
collected royalties on a technology he did not commercialise. Token Ring's
deterministic access and graceful degradation under load — §11.2's undersold
advantages — are properties of his mechanism.

**The Bellcore SONET team (1985–88).** SONET's 50 ms protection requirement did not
emerge from analysis of what was achievable; it came from a requirement that a
telephone call must not drop during a fibre cut, and the engineering was then made
to meet it. That ordering — requirement first, mechanism second — is what Chapter 72
§72.1 argues for, and it produced a capability that packet networks took two decades
to approach. Bellcore (Bell Communications Research) was the research arm the
divested Bell operating companies shared after the 1984 AT&T breakup.

**Radia Perlman (b. 1951).** Spanning tree, which is what makes redundant links
usable in a bridged network and therefore what makes the dual-homed tree of §11.4
possible at all. She has also been consistently critical of the Layer 2 designs her
own algorithm enables, arguing that routing the fabric — which is what leaf-spine
does — is the better answer, a position the industry took thirty years to adopt.
Chapter 19 covers her.

**Cisco's internetwork design authors (1990s).** The three-tier model was
formalised in Cisco's design guides rather than in a standard or a paper, and it is
worth noting that one of the most influential architectural patterns in enterprise
networking is a **vendor's design document**. That is not a criticism — the model is
sound and its layering of responsibilities is genuinely useful — but it explains why
it is described in terms of roles and best practice rather than derived from first
principles, and why "because Cisco says so" is an argument you will meet.

**Mohammad Al-Fares, Alexander Loukissas and Amin Vahdat.** Their 2008 SIGCOMM
paper *A Scalable, Commodity Data Center Network Architecture* proposed building
data-centre fabrics from large numbers of cheap, identical switches in a **fat-tree**
arrangement rather than from a few very large ones. The paper is the direct
intellectual ancestor of leaf-spine, and its argument is economic as much as
technical: commodity components in a regular topology beat specialised components in
a hierarchy, provided the topology gives you enough paths. Vahdat went on to lead
much of Google's network infrastructure work.

**Charles Clos.** French engineer at Bell Labs whose 1953 paper *A Study
of Non-blocking Switching Networks* described how to build a large switching fabric
from smaller crossbar elements arranged in stages, with a proof of the conditions
under which it is non-blocking. Written for telephone exchanges, and it is the
mathematical foundation of leaf-spine — modern data-centre fabrics are folded Clos
networks, and the term "Clos fabric" is used interchangeably with leaf-spine. Another
instance of the telephone network having solved the problem first.
