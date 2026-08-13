# Chapter 22 — The People

**Charles Bachman (1924–2017).** Proposed the seven-layer decomposition to ISO in
1977–78, drawing directly on his database work, where separating physical storage from
logical schema was already established practice. He had won the Turing Award in 1973
for the Integrated Data Store — the first database management system — and the
architectural instinct he brought was the same one: **define an interface, and the
things on either side of it can change independently.**

His model was not adopted unchanged. The committee negotiated, and layers 5 and 6 in
particular acquired the shape they have because participants had systems to
accommodate.

**Hubert Zimmermann (1941–2012).** INRIA, and the person who turned the model into
something teachable. His 1980 paper in *IEEE Transactions on Communications* is the
canonical statement, and it contains the material of §21.2 — the service/protocol
distinction, the four primitives — set out with a clarity the standards documents never
achieved.

He also worked on **CYCLADES**, the French research network of the early 1970s, and
this is the connection worth tracing. CYCLADES was the first network to make the
**hosts** responsible for reliability, leaving the network itself a simple datagram
service. Louis Pouzin's design directly influenced Cerf and Kahn (Chapter 23) and is
part of the ancestry of the end-to-end argument.

So one of OSI's principal architects had already helped build the idea that made
TCP/IP work. **The intellectual lineage crosses the political divide**, which the usual
OSI-versus-TCP/IP story tends to obscure.

**Louis Pouzin (b. 1931).** CYCLADES, 1972–1978. He introduced the **datagram** — a
self-contained packet requiring no prior connection state in the network — and argued
that the network should be unreliable and simple with the endpoints handling
correctness.

This was heresy. The telecoms establishment held that a network must be reliable, since
telephone networks were, and Pouzin's position was that this made the network complex,
slow to change, and unable to serve applications with different needs.

**He was right, and CYCLADES was cancelled in 1978** for reasons that were entirely
political — the French PTT preferred the connection-oriented X.25 it could control.
Pouzin's ideas won anyway, by way of the ARPANET, and he received the Queen Elizabeth
Prize for Engineering in 2013 alongside Cerf, Kahn, Berners-Lee and Andreessen.

**The idea outlived the funding**, which happens more often than the tidy histories
suggest.

**Radia Perlman (b. 1951).** Again. **IS-IS** is an OSI protocol — it was designed to
route CLNP, the connectionless network service that OSI adopted after argument — and it
is one of the two protocols running the world's service-provider backbones today
(Chapter 31).

It survived because it was designed to be **protocol-independent**: IS-IS carries
reachability information for whatever address family you configure, so when CLNP
disappeared and IPv4 was what mattered, IS-IS carried IPv4. When IPv6 arrived, IS-IS
carried IPv6 with no protocol change at all, while OSPF required an entirely new
version (OSPFv3).

**Generality that was designed in for one reason paid off for a completely different
one**, which is the best argument available for building things more generally than the
immediate requirement demands.

**Marshall Rose (b. 1961).** Implemented OSI protocols seriously — he wrote *The Open
Book* (1990) on OSI and ISODE, the ISO Development Environment — and then became one of
the most effective advocates for the IETF approach. His later work includes SNMP
(Chapter 53) and the POP3 mail protocol.

He is worth reading because he understood both sides properly and his criticism of OSI
is technical rather than tribal. His summary — that OSI produced excellent architecture
documents and unusable software — is the fairest short verdict available.

**Vint Cerf (b. 1943) and Bob Kahn (b. 1938).** The other side, and Chapter 23's
subject. Noted here for a detail that complicates the usual story: **they participated
in the OSI process**, and TCP/IP was for a period expected to be a transitional stack
pending OSI's arrival. The two efforts were not sealed off from each other, and several
people worked on both.

**Jon Postel (1943–1998).** Editor of the RFC series from 1969 until his death, and the
person most responsible for the culture that beat OSI. The RFC series was **free,
informal, numbered rather than versioned, and open to anyone** — the name *Request for
Comments* was chosen deliberately to signal that nothing was final.

His robustness principle — *be conservative in what you send, be liberal in what you
accept* — is the operating philosophy of a system that had to work before it was
finished. It has since been criticised, reasonably, for enabling the ambiguity that
middleboxes exploited and for making protocols harder to secure. **It was the right
principle for building an Internet and the wrong one for defending it**, which is a
distinction the 1970s had no reason to draw.

**David Clark (b. 1944).** MIT, chief protocol architect for the Internet 1981–1989,
and the author of the sentence that decided the argument, delivered at an IETF meeting
in 1992:

> **"We reject: kings, presidents and voting. We believe in: rough consensus and
> running code."**

It is a manifesto in fourteen words, and it names precisely the difference between the
two efforts: **running code as the standard of proof, rather than committee
ratification.** His 1988 paper on the DARPA Internet's design philosophy is the honest
account of what was being optimised for, and Chapter 23 §23.4 leans on it.
