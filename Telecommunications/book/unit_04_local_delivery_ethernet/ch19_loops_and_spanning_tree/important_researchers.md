# Chapter 19 — The People

**Radia Perlman (b. 1951).** The dominant figure of this chapter and of Chapter 17.

The circumstances are worth stating plainly because they are so often mythologised.
She was at Digital Equipment Corporation in 1984. DEC was building bridges, bridges
looped, and her manager gave her roughly a week to solve it. She had the algorithm in
a day. The remaining time went on the specification — and, because the specification
needed an abstract and she found abstracts tiresome, on *Algorhyme*.

The algorithm's quality is not that it finds a spanning tree; that is a standard graph
problem with well-known solutions. It is that it finds one **in a distributed system
where no node can see the graph**, using only messages to immediate neighbours,
converging from any initial state, requiring no configuration, and terminating
deterministically because MAC addresses are unique. Every one of those constraints was
imposed by the deployment reality, and the algorithm satisfies all of them in about a
page.

She has been consistently, publicly ambivalent about it. Her position — argued for
three decades — is that **spanning tree made bridged networks work well enough that
people built them far larger than they should have**, and that the right answer was
always to route. She proposed **TRILL** (RFC 6325) to do exactly that: put a routing
protocol inside the Layer 2 domain, with a hop count, so that all paths could be used
and loops would be survivable rather than fatal. TRILL was standardised, deployed
modestly, and was largely overtaken by VXLAN.

The industry did eventually adopt her recommendation, in the form of routed leaf-spine
data-centre fabrics (Chapter 67). It took thirty years.

She dislikes "Mother of the Internet" — reasonably, since the Internet has many
parents and her contributions are specific and identifiable without the epithet. Her
other major work includes **IS-IS** (Chapter 31), which is the routing protocol
underneath a large fraction of the world's service-provider networks, and substantial
work on network security. *Interconnections* remains the best book on when to bridge
and when to route.

**Mick Seaman.** Chaired IEEE 802.1 for many years and drove **RSTP (802.1w)**. The
proposal/agreement handshake of §19.3 is the key contribution: replacing a timer that
*waits until everyone has surely stopped believing the old topology* with an explicit
exchange that *establishes the same fact directly*.

The general lesson is worth extracting. Classic STP's 30 seconds were not the time
anything took; they were a bound on uncertainty. When a protocol's delay is a proxy
for a condition, replacing the proxy with a direct check of the condition is almost
always available and almost always faster. The same reasoning drives TCP fast open,
QUIC's 0-RTT handshake (Chapter 38), and BFD's replacement of routing-protocol hold
timers (Chapter 31 §31.4).

**The IEEE 802.1s working group.** MSTP, standardising what Cisco's PVST+ had
demonstrated the need for while avoiding its scaling failure. The insight — that the
useful unit is not "one tree per VLAN" but "one tree per *traffic pattern*", of which
there are two or three — is an example of finding the right granularity, which is more
often the hard part of a design than the mechanism.

**The IEEE 802.3ad working group (2000).** Link aggregation and LACP. The decision
worth noting is the **hash-based distribution** of §19.4: the committee accepted a
significant functional limitation (no single flow exceeds one member's rate) in
exchange for preserving frame ordering, because reordering would have broken TCP for
every user of the technology. **Preserving an invariant that everything above you
depends on is worth more than the headline feature**, and the committee understood
that clearly.

Aggregation moved to **802.1AX** in 2008, on the reasoning that it is a bridging
matter rather than a media-access matter.

**Joyce Kilmer (1886–1918).** *Trees*, published in 1913, is the poem *Algorhyme*
parodies. Kilmer was killed at the Second Battle of the Marne. His poem was for
decades among the most widely memorised in American schools, which is why a 1984
technical audience recognised the parody instantly — and why the joke needs explaining
now.

**Dave Katz and Dave Ward.** Authors of **BFD** (Bidirectional Forwarding Detection,
RFC 5880), which detects link failure in milliseconds and is the general solution to
the problem Loop Guard and UDLD address specifically. It appears again in Chapter 31.
Their design generalises the §19.3 observation: **a protocol's failure detection should
not be tied to its own message rate**, because the two have entirely different
requirements.
