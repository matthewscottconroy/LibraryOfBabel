# Chapter 21 — The People

**Edsger W. Dijkstra (1930–2002).** The layering idea does not originate in
networking. Dijkstra's 1968 paper *The Structure of the "THE" Multiprogramming System*
described an operating system built as strictly ordered levels, each using only the
level below, and argued that this was what made the system's correctness **provable
rather than merely plausible**.

His argument was about human limits, not machines: a system organised so that any
level can be understood using only the level beneath it is a system a person can
reason about. **Layering is a technique for managing human comprehension**, and every
benefit in §21.1 is downstream of that. Dijkstra is also responsible for the
shortest-path algorithm underlying OSPF (Chapter 31), which is an unusual range.

**Charles Bachman (1924–2017).** Proposed the seven-layer decomposition that became
OSI, drawing on his database work — where the separation of physical storage from
logical schema was already established practice. He won the Turing Award in 1973 for
the Integrated Data Store, and his contribution here is the *idea* that a
communications system could be decomposed the same way.

Chapter 22 covers the committee that followed.

**Hubert Zimmermann (1941–2012).** INRIA, and the author of the 1980 paper *OSI
Reference Model — The ISO Model of Architecture for Open Systems Interconnection*,
which is where the service/protocol distinction of §21.2 and the four service
primitives are laid out cleanly. Whatever became of OSI's protocols, **the vocabulary
in that paper is still how the field talks**, and the request/indication/response/
confirm framing remains the clearest way to compare protocols that share nothing else.

**Bill Joy (b. 1954) and the CSRG at Berkeley.** The socket interface, shipped in
4.2BSD in 1983 with DARPA funding. Its design brief was modest — make network I/O look
like file I/O so existing programmers could use it — and the consequences were enormous.

The reason it won is instructive and slightly deflating: **it was in the box.** BSD
came with sockets, BSD came with a working TCP/IP, and the source came with it. Every
Unix vendor copied the interface, Microsoft copied it as Winsock, and by 1995 there was
no alternative worth considering. OSI's service definitions were more complete, more
rigorous, and available as documents.

**Availability beats completeness.** Chapter 23 §23.2 tells the same story at the
protocol level.

**Jerome Saltzer (b. 1939), David Reed and David Clark (b. 1944).** *End-to-End
Arguments in System Design* (1984). The paper that gives a principled answer to the
question §21.1 leaves open — not *whether* to layer, but **where to put a function**.

Their argument: a function that must be correct end to end cannot be made correct by
the intermediate layers, so implementing it there is at best an optimisation and at
worst wasted work. It is the theoretical justification for the Internet's design and
the reason IP is as thin as it is. Chapter 23 §23.4 treats it properly; it belongs
here because it is the missing half of the layering argument.

**David Clark**, separately, is the source of much of the honest reflection on where
the architecture went wrong. His later papers on the tussle between stakeholders, and
on what the designers failed to anticipate, are unusually free of retrospective
tidying.

**Kevin Egevang and Paul Francis.** RFC 1631 (1994), *The IP Network Address
Translator*. The document is worth reading for its tone: it presents NAT explicitly as
a **short-term measure** pending IPv6, catalogues its architectural damage with
precision, and recommends it anyway.

Thirty years later NAT is universal and IPv6 is at roughly half of Internet traffic.
**The temporary measure outlived the permanent solution's deployment schedule**, which
is a pattern worth watching for in every transitional design.

**Jim Kurose and Keith Ross.** Their textbook popularised the **top-down**
teaching order — applications first, physical layer last — on the argument that
students should meet the layer they already use before the layers they do not.

This book goes bottom-up and dependency-ordered instead, for a specific reason: the
top-down order requires students to accept that lower layers work before they know how,
and the acceptance tends to become permanent. **Neither order is wrong.** Kurose and
Ross's book is excellent and worth reading alongside this one precisely because the
disagreement is visible.

**Van Jacobson (b. 1950).** His work appears throughout the transport chapters, and it
belongs here for **header compression** (RFC 1144), which reduced a 40-byte TCP/IP
header to 3–5 bytes for slow serial links by exploiting the fact that successive
headers in a flow differ in very few fields.

It is a textbook §21.3 case: the compressor must understand **both** the TCP and IP
headers simultaneously, which no layer is supposed to do. **The performance gain
required the violation**, the violation was contained to a single link, and the
technique is still used — as ROHC — on cellular links where every byte over the air is
expensive.

**Jim Gettys.** Bufferbloat again (Chapter 13). It appears in this chapter's
argument because it is a pure information-barrier failure: buffers were added at every
layer independently, each addition locally reasonable, and **no layer could see the
total**. A queue that a designer thought was 20 ms deep was in practice several
seconds. The fault was in nobody's component and in the composition of all of them,
which is exactly the failure mode layering makes possible.
