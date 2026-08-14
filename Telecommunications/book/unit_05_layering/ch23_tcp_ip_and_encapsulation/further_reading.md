# Chapter 23 — Further Reading

## Primary sources

**Cerf, V. & Kahn, R. (1974). "A Protocol for Packet Network Intercommunication."
*IEEE Transactions on Communications*, 22(5).**
**The paper.** Thirteen pages, and everything is there: gateways, best-effort delivery,
endpoint reliability, and a TCP that had not yet been split. Read it and note what they
got wrong as well as right — the address format in particular.

**Saltzer, J., Reed, D. & Clark, D. (1984). "End-to-End Arguments in System Design."
*ACM TOCS*, 2(4).**
**Read this one properly.** Fourteen pages, freely available, and the most important
paper in network architecture. The file-transfer example is deliberately not about
networks, which is what makes the argument general.

**Clark, D. (1988). "The Design Philosophy of the DARPA Internet Protocols."
*ACM SIGCOMM*.**
The seven design goals in priority order, by one of the architects. Note what is absent
from the list.

**RFC 791 (IP), RFC 793 (TCP), RFC 768 (UDP).**
Postel, 1980–81. About 130 pages between them, and a determined person can implement
from them. **RFC 768 is three pages** — read it in one sitting as a demonstration of
how little a transport protocol needs to be.

**RFC 1122 & RFC 1123 — Braden, R. (1989). *Requirements for Internet Hosts.***
Where the four-layer model is stated. More usefully, the MUST/SHOULD/MAY discipline that
tells implementers which parts they may get wrong.

**RFC 2119 — Bradner, S. (1997). *Key words for use in RFCs.***
Three pages defining MUST, SHOULD and MAY. Cited by nearly every specification since,
and the reason independent implementations interoperate.

**RFC 1958 — Carpenter, B. (1996). *Architectural Principles of the Internet.***
The architecture stated as principles rather than protocols. Includes the observation
that **the end-to-end argument is the most important principle** and that
constant change is the only constant.

**Clark, D. et al. (2002). "Tussle in Cyberspace: Defining Tomorrow's Internet."
*ACM SIGCOMM*.**
Clark's reframing: the network as a place where parties with conflicting interests
negotiate. Read it against the 1984 paper and notice what eighteen years changed.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1: The Protocols.***
**The reference for encapsulation.** Chapters 1–3 cover exactly §23.3's material with
real captures. The method — state the protocol, then show it happening — is what this
chapter imitates.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed.**
The revision, covering IPv6 and modern behaviour throughout. The better single
reference now.

**Abbate, J. (1999). *Inventing the Internet.* MIT Press.**
The historical account, well researched and honest about the contingency. Good on
CYCLADES and on the ARPANET-to-Internet transition.

**Hafner, K. & Lyon, M. (1996). *Where Wizards Stay Up Late.***
The narrative history of the ARPANET. Less rigorous than Abbate, considerably more
enjoyable, and the people come across as people.

**Russell, A. L. (2014). *Open Standards and the Digital Age.***
Chapter 22's recommendation, and equally relevant here for the account of why free and
working beat specified and complete.

## Applied

**Wireshark, on a single HTTP request.** Expand every layer and account for every byte
against §23.3's arithmetic. This is exercise F3 and it is the most valuable
practical exercise in this unit — **encapsulation stops being a diagram the moment you
watch it happen.**

**`tcpdump -X`**, to see the raw bytes and find the headers by hand. Slower than
Wireshark and much better for learning, because you have to know where the boundaries
are rather than being shown.

**`dig +trace example.com`**, to watch step 0 of the trace happen in full.

**`traceroute` and `mtr`**, and notice that no MAC addresses appear anywhere in the
output. Exercise E3 asks why; the answer is the whole of §23.3.

**Capture the same transfer at two points** — the client and a mirror port at the far
side of a router — and compare. The IP header will match except for TTL and checksum;
the Ethernet header will be entirely different. **This is the most convincing single
demonstration in the chapter.**

**Lab 11** in this book's [labs/](../../../labs/) directory does exactly that
two-point capture, with a worksheet for recording every field at every hop.

## For the certification-minded

Objective 1.1 expects the TCP/IP model, the OSI mapping, **encapsulation and the PDU
names**. All are examined.

Six things worth over-learning:

1. **The four layers**: Application, Transport, Internet, Link.
2. **The mapping**: Application = OSI 5+6+7; Link = OSI 1+2; Transport and Internet
   map one-to-one.
3. **The PDU names**: bit, frame, packet, segment (TCP) / datagram (UDP).
4. **EtherType values**: `0x0800` IPv4, `0x86DD` IPv6, `0x0806` ARP.
5. **IP protocol numbers**: 1 ICMP, 6 TCP, 17 UDP.
6. **IP is connectionless and best-effort; TCP adds reliability; UDP does not.**

Items 4 and 5 appear as direct recall questions. Item 6 appears in a dozen different
disguises.

And the framing that makes all of it stick: **every layer has a demultiplexing key, and
they chain.** EtherType tells the link layer who gets the payload, the protocol number
tells IP, the port tells the transport, and the process is the end of the chain. If you
know that, you can reconstruct most of the specifics.
