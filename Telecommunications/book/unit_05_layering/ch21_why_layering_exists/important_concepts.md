# Chapter 21 — Important Concepts

**The combinatorial argument** *(§21.1)* — Connecting *m* applications to *n* media
directly costs *m*×*n* implementations; a common intermediary costs *m*+*n*. The
advantage grows with scale — 2.5× at 5×5, 25× at 50×50 — which is the property that
matters.

The marginal cost changes character *(§21.1)* — Adding a medium costs **one**
implementation regardless of how many applications exist, and vice versa. The point is
not fewer lines of code today; it is that the cost of change becomes independent of
the system's size.

What was actually bought *(§21.1)* — **Independent evolution** (Wi-Fi worked with
every existing application on day one, written by people who were dead or retired);
**independent expertise** (radio engineering and mail-server design need not meet);
**independent failure** (the basis of §22.4's diagnostic method); **substitutability**
(100 Mb/s to gigabit required no application change).

The middle is special *(§21.1)* — Many applications, many media, **one IP**. It
must be simple enough for anything, stable, and minimal — and changing it affects
everyone simultaneously and cannot be done incrementally. This is the hourglass, and
it is why IPv6 has taken thirty years.

Layers must earn their place *(§21.1)* — A layer is justified when it serves many
things above with one implementation below. A layer used by exactly one thing is not
a layer; it is a subroutine with delusions.

**Service vs protocol** *(§21.2)* — A **service** is what a layer offers upward,
defined by operations (**vertical**). A **protocol** is how peers implement it
(**horizontal**). The protocol can be replaced entirely without the layer above
noticing — TCP's congestion control was rewritten five times and no application
changed.

The four service primitives *(§21.2)* — Request, Indication, Response, Confirm.
A confirmed service uses all four; TCP's three-way handshake and `connect()`/`accept()`
are the same four with different names.

The socket interface won *(§21.2)* — Eight calls, forty years, every operating
system. It looks like file I/O, **hides the protocol** (one argument switches TCP to
UDP or IPv4 to IPv6), hides the medium completely, and is small. A
specified-but-unimplemented interface organises nothing — sockets beat OSI's more
complete service definitions by being available, adequate and free.

What an interface must not expose *(§21.2)* — The medium, unnecessary addressing
detail, or timing. The applications that broke on IPv6 assumed a 32-bit address;
the interface was general and the applications were not.

What it wrongly hides *(§21.2)* — Path MTU, connection quality, and which interface
was used. Where an abstraction hides something the user genuinely needs, it gets
bypassed rather than fixed.

Encapsulation treats payloads as opaque *(§21.2)* — IP does not parse the TCP
header; Ethernet reads only the EtherType. That discipline is what makes independent
evolution possible, and it is exactly what middleboxes violate.

**Header overhead** *(§21.3)* — 78 bytes minimum for TCP/IP/Ethernet. 5.1% for a
1460-byte payload; 1.3% efficiency for a single keystroke. G.729 VoIP carries 20
bytes of speech in a 98-byte frame — 20% efficiency, so an 8 kb/s codec consumes
39 kb/s. Capacity planning must use the on-the-wire rate.

Copying is the hidden cost *(§21.3)* — At 100 Gb/s memory bandwidth binds before
the network does. Every fix is a controlled layer violation: zero-copy,
scatter-gather, **TSO/LRO**, checksum offload, RDMA, DPDK/XDP.

**TSO** *(§21.3)* — The stack hands the NIC 64 KB and a template; the NIC produces
forty frames with correct sequence numbers. A link-layer device performing a
transport-layer function. Consequence: `tcpdump` on the host shows packets that
never existed on the wire.

**The information barrier** *(§21.3)* — The most consequential cost. A layer cannot
see what it is not told, and sometimes decides exactly wrong as a result.

**TCP over wireless** *(§21.3)* — TCP reads loss as congestion, which is right on wired
networks and wrong on radio, where loss is usually corruption. The link layer knows
and has no way to say so. Mitigations — 802.11 retransmission, ECN, BBR — are all
attempts to move information across a boundary drawn to keep it out.

What cannot be optimised across *(§21.3)* — IP cannot combine segments; Ethernet
cannot prioritise by application; TCP cannot know that a video frame has expired and
will retransmit it regardless. Real-time media uses UDP not because TCP is badly
built but because its service is the wrong service.

**Head-of-line blocking** *(§21.3)* — One lost packet stalls every HTTP/2 stream, because
TCP must deliver in order and cannot be told the streams are independent. QUIC's
central design decision is to put transport in user space so the multiplexing and
reliability layers can see each other.

**Latency** *(§21.3)* — 20–50 µs through a full kernel stack, 2–5 µs with DPDK, under
2 µs with RDMA. An order of magnitude, obtained by **removing layers**.

**NAT** *(§21.4)* — Rewrites Layer 3 addresses and Layer 4 ports, recomputes the
TCP checksum, and for protocols that embed addresses (FTP, SIP) must parse **Layer 7**.
The most consequential layer violation ever deployed, and it saved IPv4 — twenty-
five extra years at the cost of a permanently more complicated Internet.

**Application-layer gateways** *(§21.4)* — A Layer 3 device reading Layer 7 to rewrite
embedded addresses. **Encryption breaks them**, which is why STUN, TURN and ICE exist.

**L7 load balancers** *(§21.4)* — There is no end-to-end connection; there are two,
joined in the middle. `X-Forwarded-For` is an application header carrying network
information the layering destroyed — a layer violation whose remedy is another layer
violation.

**The TCP pseudo-header** *(§21.4)* — TCP's checksum covers the **IP addresses**, by
specification, since 1981. Sound reasoning (it catches misdelivery), and the cost is
that TCP and IP cannot be separated. It is the seam left when Cerf and Kahn's
single 1974 protocol was split in 1978.

Layer 2.5 means nothing *(§21.4)* — ARP and MPLS are both called it, for entirely
different reasons. The numbering is a description, not a constraint.

QUIC closes the loop *(§21.4)* — Middleboxes violated layering → the resulting
ossification made TCP unevolvable → the response was a transport in user space
that violates layering deliberately and encrypts its headers so the next generation
of middleboxes cannot ossify it too.

**The right position** *(§21.4)* — The model is a map: useful, simplified, accurate
for most of the territory, and wrong in specific known places you should learn.
Layering is a design discipline, not a law of physics.

Where the hard faults are *(§21.4)* — A fault that respects layering is usually
easy; §22.4's method finds it. A fault that crosses layers is hard, because the
symptom appears at a layer other than the cause — a NAT mangling a payload, a
firewall dropping fragments, an offload corrupting a checksum, a middlebox rejecting an
unfamiliar option.
