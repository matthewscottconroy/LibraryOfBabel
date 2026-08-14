# Chapter 23 — Important Concepts

**Kahn's problem** *(§23.1)* — How do you connect networks designed without any
knowledge of one another, which will not change? The last clause is the whole
difficulty; any answer requiring the world to be different is not an answer.

Principle 1 — each network keeps its own internals *(§23.1)* — The internetwork
sits **on top**, using each network as an opaque delivery mechanism. This is why the
Internet absorbed Ethernet, Wi-Fi, LTE and 5G with no architectural change.

Principle 2 — stateless gateways *(§23.1)* — Gateways keep **no per-connection
state**, so a gateway can crash and be replaced and the connections survive. This
is **fate-sharing**: state lives with the entity whose failure it shares.

Principle 3 — best-effort *(§23.1)* — Packets may be lost, duplicated, reordered
or delayed. Radical at the time, and correct: a reliable network must implement
reliability in a way that suits every application, and no such way exists. Voice
wants timeliness, file transfer wants accuracy.

Principle 4 — reliability at the endpoints *(§23.1)* — The end-to-end argument in
embryo, seven years before it was formalised.

**The 1978 split** *(§23.1)* — The 1974 protocol was one thing. Splitting it into
**IP** (addressing and forwarding) and **TCP/UDP** (reliability, or not) is the
most important structural decision in the suite. Without it, real-time media
could not use IP and voice would have needed a separate network — which is exactly what
the telecoms industry expected.

The seam is still visible *(§23.1, §21.4)* — TCP's checksum covers a pseudo-header
of IP addresses, because in 1974 there was no boundary to respect.

**The flag day** *(§23.1)* — **1 January 1983**, ARPANET switched from NCP to TCP/IP,
~400 hosts, one authority, a hard deadline. There has never been another and never
will be. Every change since must be incrementally deployable — working when only
some have adopted it, and benefiting early adopters. That constraint explains IPv6,
DNSSEC, and BGP security.

**Why TCP/IP won** *(§23.1)* — **Free** (RFCs cost nothing); **in BSD** with a working
stack and sockets; **it worked**, on real traffic, with the bugs already found;
simple enough to implement (~130 pages); and **it arrived first**.

Rough consensus and running code *(§23.1)* — Not merely engineering culture but a
statement about what constitutes evidence: a working implementation is proof, a
ratified document is a claim.

The four-layer model was not designed *(§23.2)* — It is a description written after
the fact, in RFC 1122, whose purpose was to tell implementers what a host must do.

Collapse 1: OSI 5+6+7 → Application *(§23.2)* — There is **no separate
implementation** of session or presentation. No boundary, no header, no device. The
functions exist; the layers do not.

Collapse 2: OSI 1+2 → Link *(§23.2)* — Not that the distinction is unreal, but
that it is not IP's business. IP requires exactly one thing: *deliver this packet
to the next hop.* The refusal to specify is the design.

The specification's silence is its most valuable feature *(§23.2)* — Ethernet,
FDDI, Wi-Fi, DSL, LTE, 5G, 400G, LEO satellite: not one required a change to IP,
because IP never said what a link was. A specification constrains the future;
specifying less leaves more room.

The Internet layer is thin *(§23.2)* — IP, ICMP, IGMP, and nothing else. IP does
addressing, forwarding, fragmentation and a hop limit. Everything IP does not do was
a decision defended by the end-to-end argument.

**QUIC's position** *(§23.2)* — A transport inside a transport, because deploying a
new IP protocol number is impossible — middleboxes drop what they do not recognise.
Ossification forced a new transport to disguise itself as UDP.

Three models, no conflict *(§23.2)* — **OSI (7)** for vocabulary, certification and
the diagnostic method; **TCP/IP (4)** for describing what runs; **five-layer** for
teaching. Different decompositions for different purposes; being troubled by the
disagreement means taking them too literally.

**No security layer** *(§23.2)* — A deliberate omission that proved costly. Security
was retrofitted into **every protocol individually**, decades later: TLS, DNSSEC, RPKI,
IPsec, WPA. The architecture's largest mistake was assuming a trustworthy network.

**The PDU names** *(§23.3)* — data → segment/datagram → packet → frame → bits. Use
them precisely; saying "packet" for everything makes fault localisation harder.

Each layer treats its payload as opaque *(§23.3)* — Add a header, hand it down, and
the layer below neither knows nor cares what it contains.

**The trace's arithmetic** *(§23.3)* — 120 bytes of HTTP → 140-byte segment →
160-byte packet → 178-byte frame → 198 byte-times on the wire = 61% efficiency.

The frame is rebuilt at every hop; the packet is not *(§23.3)* — A router discards
the frame header entirely, reads the IP header, decrements TTL, recomputes the header
checksum, and builds a new frame. It never examines the TCP header.

**DNS happens first** *(§23.3)* — Step 0, invisible in most descriptions, and the
most common failure point in the whole sequence.

**The demultiplexing chain** *(§23.3)* — EtherType → Protocol → Port → process.
`0x0800` IPv4, `0x86DD` IPv6, `0x0806` ARP; protocol 6 TCP, 17 UDP, 1 ICMP. This chain
is exactly how Wireshark chooses a dissector and how you reason about what a device can
see.

A fault at any layer is a fault at every layer above it *(§23.3)* — §22.4's method
restated as a property of encapsulation rather than a heuristic.

**The end-to-end argument** *(§23.4)* — A function that must be correct end to end
cannot be made correct by the intermediate layers; a version provided there may be
useful as a performance enhancement. Two clauses, and people who quote only the first
get it wrong.

**The file-transfer example** *(§23.4)* — A reliable network catches wire and router
failures and cannot catch memory corruption, disk errors or application bugs. The
application must checksum anyway — so the network's guarantee is redundant for
correctness.

**The test** *(§23.4)* — Does this function eliminate the endpoint's obligation? If
not, it is an optimisation and must justify its cost. 802.11 link retransmission
passes this test; it does not remove TCP's need to retransmit, it just reduces it.

**The security exception** *(§23.4)* — The reasoning was impeccable and the result
was that neither the network nor the endpoints encrypted, for twenty-five years.
Telnet, FTP, HTTP, SMTP, SNMPv1, all plaintext. A principle that assigns
responsibility without assigning it to a specific party assigns it to nobody.

**The hourglass** *(§23.4)* — Many protocols above, many technologies below, one
protocol in the middle. Narrow because IP does the minimum.

What the waist costs *(§23.4)* — IP is nearly impossible to change (IPv6:
specified 1998, ~50% around 2024, still unfinished); anything IP omits must be done
everywhere else; and the waist can be pinched by middleboxes, whose ossification
is why QUIC hides in UDP. The greatest achievement and the permanent constraint,
both true.

Where the argument is weakest *(§23.4)* — **Trust** (endpoints ignoring congestion
control harm everyone, so routers police); **performance at scale** (CDNs are pure
middle-of-network optimisation and now carry most traffic); **constrained devices** (a
32 KB sensor cannot run a full stack).
