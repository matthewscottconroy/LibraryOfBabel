# Chapter 23 — The People

**Bob Kahn (b. 1938).** Posed the problem. At DARPA in 1972 he was running the packet
radio and satellite programmes alongside the ARPANET, and he was the person who had to
live with the fact that they could not talk to each other.

**Stating the problem correctly was the contribution.** Not "how do we make all
networks the same" — which was the obvious framing and is unachievable — but "how do we
connect networks that will not change". Everything in the design follows from taking
that constraint seriously.

He also ran the office that funded the work, which is a form of contribution the
histories tend to undervalue.

**Vint Cerf (b. 1943).** Stanford, then DARPA. Wrote the protocol with Kahn, and then
spent decades doing the far less glamorous work of making it happen: chairing the IETF's
predecessor bodies, running the transition, founding the Internet Society, and — still —
travelling continuously to explain the Internet to people who need to understand it.

The **1978 split** of TCP into TCP and IP is largely his call, taken after Danny Cohen
and others argued that packet voice could not use a protocol that insisted on
retransmission. It is the decision that made the suite general, and it came from
listening to an application whose needs contradicted the design.

Cerf is also, quietly, the reason the Internet has been so consistently explained
honestly. His public account of the design has never claimed more foresight than the
designers had.

**Louis Pouzin (b. 1931).** CYCLADES, and the datagram. Chapter 22 tells the story; the
point that belongs here is that **Cerf and Kahn credit him directly.** The idea that
the network should be unreliable and simple, with the endpoints handling correctness,
came from CYCLADES, and Cerf has said so consistently.

Pouzin's network was cancelled for political reasons in 1978. **The idea escaped
anyway**, which is the most hopeful thing in this chapter.

**Danny Cohen (1937–2019).** Ran the packet voice experiments on the ARPANET in the
mid-1970s and made the argument that split TCP: **a retransmitted voice sample is worse
than a lost one**, so an application must be able to decline reliability.

He is also the author of *On Holy Wars and a Plea for Peace* (1980), the paper that
introduced **big-endian** and **little-endian** to computing, borrowing the terms from
*Gulliver's Travels* to satirise a dispute he considered unresolvable and not worth the
energy. Network byte order is big-endian because somebody had to choose.

**Jon Postel (1943–1998).** RFC editor, IANA, and the person who held the number
assignments — protocol numbers, port numbers, address blocks — in his head and then in
a series of files, for twenty-nine years. The demultiplexing chain of §23.3 works
because somebody maintained the registries that make `0x0800` mean IPv4 and `6` mean
TCP, consistently, worldwide, for decades.

**The unglamorous work is load-bearing.** When Postel died in 1998 the transition of
IANA's functions to an institution took years, because one person had been doing
something that turned out to require an organisation.

**Jerome Saltzer (b. 1939), David Reed and David Clark (b. 1944).** *End-to-End
Arguments in System Design* (1984). Fourteen pages, and the most cited paper in network
architecture.

What makes it valuable is that it is an **argument about where functions belong**,
stated generally enough to apply outside networking — Saltzer and Reed came to it from
operating systems and distributed systems, and the file-transfer example is
deliberately not about networks at all.

**David Reed** is also the author of UDP (RFC 768), which is three pages long and is
the end-to-end argument expressed as a protocol: ports, a length, a checksum, and
nothing else.

**David Clark (b. 1944).** MIT, chief protocol architect 1981–1989, and the source of
most of the honest retrospection. His 1988 paper on the DARPA Internet's design
philosophy lists what the designers were optimising for, in priority order:

1. Continue despite loss of networks or gateways
2. Support multiple types of service
3. Accommodate a variety of networks
4. Permit distributed management
5. Be cost-effective
6. Permit host attachment with low effort
7. Account for resources

**Security is not on the list.** Neither, in any serious way, is accountability. Clark
has been explicit that this reflected the context — a network of mutually-trusting
research institutions — and that the omission is the largest architectural debt the
Internet carries.

His later work on "tussle in cyberspace" reframes the network as a place where parties
with conflicting interests negotiate, and argues that architecture should make the
tussle visible and manageable rather than pretend it does not exist. It is a very
different intellectual posture from 1984's and worth reading against it.

**Bob Braden (1934–2018).** Edited **RFC 1122** and **RFC 1123**, the host requirements
documents that turned a pile of specifications into something implementable. The
four-layer model of §23.2 is stated there, and the documents' real contribution is the
MUST/SHOULD/MAY discipline — telling implementers not merely what the protocols are but
**which parts they are permitted to get wrong.**

He also ran the End-to-End Research Group for many years, which is where much of the
transport work of Chapter 36 was argued out.

**Scott Bradner (b. 1946).** Author of **RFC 2119**, which defines MUST, SHOULD, MAY,
SHOULD NOT and MUST NOT — three pages that make every other RFC readable without
ambiguity. It is cited by essentially every specification written since 1997.

**A vocabulary for degrees of obligation**, which sounds like pedantry and is the
reason independent implementations interoperate.
