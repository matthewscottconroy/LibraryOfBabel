# Chapter 26 — The People

**Jeffrey Mogul and Jon Postel (1943–1998).** RFC 950 (1985), and the subnet
mask. Chapter 25's notes cover the significance; the point that belongs here is that
**their document created the possibility of VLSM without enabling it.** The mask existed
locally, and the routing protocols of the day did not carry it, so every subnet of a
given network had to use the same mask for another five years.

**The gap between "the concept exists" and "the infrastructure supports it" is a
recurring pattern** in this book — it appears again with ECN (Chapter 24), with IPv6
(Chapter 28), and with DNSSEC (Chapter 39).

**Rob Braden and the RFC 1009 authors (1987).** *Requirements for Internet Gateways*,
which specified that routers must handle networks with **different masks** — the formal
enabling of VLSM. Six years before CIDR, and it took the routing protocols most of that
time to catch up.

**Vince Fuller, Tony Li, Jessica Yu, Kannan Varadhan and Yakov Rekhter.** CIDR, RFCs
1518 and 1519 (1993). Chapter 25's notes give the history. What belongs in this chapter
is the **aggregation** half of their contribution.

Their insight — that the routing table's problem was not its size but its **growth
rate**, and that the growth rate could be decoupled from the number of organisations by
allocating hierarchically — is the reason §26.3's arithmetic works. **Provider-based
allocation means the table grows with the number of providers.**

The cost, which Rekhter stated plainly at the time, is that **changing provider means
renumbering**. This remains unsolved in IPv4 and in IPv6, and it is why organisations
that can obtain provider-independent space do so, and why those that cannot rely on NAT
(Chapter 33) to insulate themselves.

**Yakov Rekhter (1953–2023)**, separately, is one of the most consequential people in
this book — BGP, MPLS, VPN architectures, and the address allocation architecture. He
worked at IBM, Cisco and Juniper across four decades, and the routing half of the modern
Internet is substantially his.

**Tony Li.** Cisco, Juniper, and much of the router architecture work that made
longest-prefix match at line rate practical. §26.3's claim that aggregation is what makes
a global routing table *possible* rests on the physical limits of TCAM, and Li is one of
the people who established what those limits are and how to work within them.

His later writing on routing scalability — including the argument that the current
architecture has a finite remaining lifespan — is worth reading against Chapter 32's more
optimistic account.

**Radia Perlman (b. 1951).** Again, for the observation that **hierarchy is the only
known way to make a large network's routing tractable**, argued across her work on IS-IS
and in *Interconnections*. The hierarchical plans of §26.4 are an application of a
principle she states more generally: **a flat namespace of size *n* requires *n* state
everywhere; a hierarchical one requires log *n*.**

Chapter 18 §18.1's argument about MAC versus IP addressing is the same principle at a
different scale, and Chapter 31's areas in OSPF are the same principle again.

**Geoff Huston.** APNIC, and the person who has actually measured the routing
table for thirty years. The figure in §26.3 — roughly 950,000 prefixes — comes from his
continuously-updated dataset at bgp.potaroo.net, and the projections that made CIDR
urgent in 1992 were validated against measurements of the same kind.

**His most useful contribution to a student is the habit**: when a claim about the
Internet's scale is made, it is usually measurable, and somebody is measuring it.

**The unnamed instructors.** The magic-number method of §26.2 has no author. It does not
appear in any RFC, no paper introduced it, and it is universally known by everyone who
teaches or does this work.

It emerged from instructors solving the same pedagogical problem repeatedly through the
1990s — how to make a student fast at binary arithmetic under exam conditions — and
propagated by being taught. **A genuine piece of craft knowledge**, refined by use and
transmitted orally, in a field that usually documents everything.

It is worth noticing that the best-known technique in the most-examined topic in
networking is the one thing in this chapter with no citation.

**Todd Lammle, Wendell Odom, Rick Graziani and the certification authors.** The
pedagogical literature on subnetting is unusually developed, because unusually many
people have had to learn it under time pressure. Whatever one thinks of certification as
an institution, **the teaching technique it produced is genuinely good**, and the
drilling approach of §26.2 is borrowed from it without embarrassment.
