# Chapter 24 — The People

**Jon Postel (1943–1998).** Author of **RFC 791**, the IPv4 specification, in September
1981. Forty-five pages, and the header in §24.2 is exactly as he wrote it. Not one field
has been added; two have been redefined.

The document's durability is worth pausing on. RFC 791 has never been revised — RFC 1812
clarified router behaviour and RFC 2474 redefined the TOS byte, but **the specification
itself stands unamended after forty-four years**, carrying a volume of traffic its author
could not have imagined. Very little engineering of any kind has that property.

Postel is also responsible for the **protocol number registry** that makes §24.2's
demultiplexing work, maintained personally for decades. Chapter 23's notes cover the
larger point about unglamorous work.

**Vint Cerf (b. 1943) and Bob Kahn (b. 1938).** Chapter 23's subjects, and the source of
§24.1's best-effort decision. The argument they had to win — that a network should
promise less — was contested for fifteen years by an industry whose entire experience
said otherwise.

**Louis Pouzin (b. 1931).** CYCLADES, and the datagram. The best-effort model is his
before it is anyone's.

**Danny Cohen (1937–2019).** The packet voice work that proved the point empirically.
Voice over the ARPANET in the mid-1970s demonstrated that **an application could want
the network to give up on a packet**, which is not an intuition anyone had before someone
tried it.

**Jeffrey Mogul and Steve Deering.** **RFC 1191** (1990), *Path MTU
Discovery* — §24.3's mechanism. The design is clean and the deployment history is a case
study in how a mechanism that depends on a third party's cooperation fails.

They anticipated the black hole. RFC 1191 explicitly discusses what happens when the
ICMP message does not return, and recommends a fallback. **The fallback was widely not
implemented**, and the result is a failure mode that has been diagnosed independently by
tens of thousands of engineers over thirty years.

**Mogul** also authored RFC 1122's fragmentation guidance and much of the early work on
what hosts should do when the network misbehaves — a persistent theme in his output.

**Steve Deering**, separately, is the principal architect of **IPv6**, and
several of §24.2's IPv4 criticisms are things he removed: the header checksum, router
fragmentation, and the options mechanism. Chapter 28 is largely his.

**Van Jacobson (b. 1950).** **`traceroute`, 1987.** Written, by his account, in an
evening, in response to a network problem he could not diagnose.

The design is the kind of thing that looks obvious afterwards: **deliberately cause a
failure at a controlled distance and read the error message.** Nobody had done it,
because TTL was understood as a safety mechanism rather than as a probe.

Jacobson's other contributions — TCP congestion control (Chapter 38), header compression
(Chapter 21), `tcpdump`, `pathchar`, and later CoDel and named data networking — make him
plausibly the most practically influential person in this book. **The recurring pattern
in his work is measurement:** he builds the instrument first, and the fix follows from
what it shows.

**Steve Bellovin.** AT&T Bell Labs, then Columbia. Author of **RFC 3514**, the
evil bit, published 1 April 2003 — and one of the most-cited jokes in the RFC series,
because the point it makes is one that security proposals keep needing to be told.

His serious work is substantial: the 1989 paper *Security Problems in the TCP/IP
Protocol Suite* identified sequence number prediction, ARP spoofing, routing attacks and
DNS attacks **years before any of them were exploited**, and is one of the foundational
documents of network security. Chapter 57's threat model is largely descended from it.

**Ron Bonica, Fred Baker, Geoff Huston, Joel Halpern, Matt Mathis and Warren Kumari.**
Authors of **RFC 8900** (2020), *IP Fragmentation Considered Fragile* — the formal
statement of §24.3's verdict, thirty-nine years after fragmentation was specified.

The document's value is that it is a **consensus statement**: fragmentation's problems
were folklore among operators for decades, and RFC 8900 collected the evidence and made
it citable. **Turning accumulated operational knowledge into a document you can point at
is a real contribution**, and an under-recognised category of work.

**Geoff Huston**, APNIC's chief scientist, deserves separate mention for
measuring things the community argues about — fragmentation loss rates, IPv6 deployment,
routing table growth, DNSSEC validation. His monthly analyses are the empirical
foundation under a great deal of this book's Unit VI and Unit X.

**Matt Mathis.** The Mathis equation (Chapter 38 §38.2) quantifies exactly how badly
loss hurts TCP, which is what makes §24.3's fragment-loss multiplication argument
concrete rather than rhetorical. He also co-authored **RFC 4821** (PLPMTUD), the
robust alternative to ICMP-dependent path MTU discovery — the correct fix for the black
hole, and still under-deployed twenty years on.

**Kathleen Nichols and K. K. Ramakrishnan.** DSCP (RFC 2474) and ECN (RFC 3168), the two
redefinitions of §24.2's most-recycled byte. **ECN is the more interesting story**: it
allows congestion to be signalled without loss, it was specified in 2001, and deployment
was blocked for over a decade because middleboxes cleared or mishandled the bits.

It is now widely enabled, and the delay is a precise measurement of what §21.4's
ossification costs. Nichols also co-created **CoDel** with Jacobson, the queue-management
algorithm that addresses bufferbloat.
