# Chapter 30 — The People

This chapter has fewer named figures than most, and the reason is itself informative:
**static routing was never invented.** It is what you do when there is no protocol, and
every early network did it because nothing else existed. The people below are those who
had to decide what to do when it stopped working.

**The ARPANET operators, 1969–1975.** The first routing tables were maintained by hand,
by people at BBN, for a network of a few dozen nodes. **They abandoned it almost
immediately** — the ARPANET had a distance-vector protocol by 1969 — and the reason was
precisely §30.4's: the network had redundant paths, links failed regularly, and nobody
could keep up.

**A network with redundant paths outgrows manual routing before it outgrows anything
else**, and it was demonstrated within months of the Internet's ancestor existing.

**The Cisco engineers who chose the administrative distance values.** The table in §30.2
is not from any standard. It is a set of Cisco defaults from the late 1980s that the
industry adopted because Cisco's equipment was ubiquitous, and it has become the de-facto
convention that other vendors match or approximate.

**Standardisation by market share rather than by committee** is a recurring theme worth
noticing, and it is the same mechanism by which `traceroute`'s behaviour, the `/24`
filtering convention (Chapter 29 §29.3), and much of Ethernet's operational practice
became universal without anyone specifying them.

The values are defensible, and the important thing about them is not that they are optimal
but that they are **shared**. A network where every vendor chose differently would be
unworkable.

**Bob Hinden and the early gateway implementers.** RFC 1812's forwarding requirements
(Chapter 29) formalised what routers already did, including the requirement that a route
with an unreachable next hop must not be used. §30.1's platform variation exists because
that requirement was stated normatively and implemented differently.

**Dave Katz and Dave Ward.** **BFD** — RFC 5880 — the answer to §30.3's central problem.
Their observation was that **failure detection had been tied to whatever protocol happened
to be running**, so its speed was an accident of that protocol's timer design: OSPF
detected failure in 40 seconds because its hello timer was 10 seconds, not because 40
seconds was the right answer.

**BFD separates failure detection from the protocol that consumes it.** A single
lightweight mechanism runs between two devices at whatever rate they negotiate — often
sub-second — and every protocol, and static routes, can subscribe to its verdict.

**The general principle is worth extracting:** *when several mechanisms each need the same
piece of information and each derives it independently, factor it out.* The same reasoning
produced Chapter 19's link aggregation control protocol, and it is the same instinct as
Chapter 21's layering argument applied within the control plane.

**The IP SLA and object tracking designers.** A less elegant solution to the same problem,
and the one that is actually deployed on branch routers everywhere, because it requires
nothing of the far end.

**BFD needs both ends to cooperate; IP SLA needs only your own router to care.** That
asymmetry is why the worse mechanism is the common one — Chapter 27 §27.3's lesson that
**a solution requiring only your own network beats one requiring everyone's**, appearing
again at a much smaller scale.

**The unnamed engineer who left a static route in place.** Deserves inclusion, because
§30.2's most practically important fact — that administrative distance 1 lets a forgotten
static route silently override a correctly-functioning routing protocol — is a lesson
almost every network engineer learns by finding one.

The route is usually years old, was added during an incident, solved the problem, and was
never removed. **It works until the topology changes underneath it**, at which point it
becomes a fault that looks like a protocol failure and is not.

**Radia Perlman (b. 1951).** Her framing of the general question — *what must a node know,
and how does it come to know it?* — is what separates this chapter from the next. Static
routing answers "it is told"; Chapter 31 answers "it works it out". **Her insistence that
the second requires the network to be able to detect its own failures** is §30.4's
argument, made properly.

**Jeff Doyle.** Not a researcher, and worth naming: *Routing TCP/IP* is the book that
taught a generation of engineers the material in this unit, and its treatment of
administrative distance and route selection is more careful than any vendor documentation.

**The pedagogical contribution matters.** A great deal of networking knowledge exists only
as folklore or as vendor configuration guides, and the small number of people who wrote it
down properly — Doyle, Perlman, Stevens, Seifert — are the reason it can be learned at all
rather than only absorbed.
