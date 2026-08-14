# Chapter 14 — The People

**Vinton Cerf (b. 1943) and Robert Kahn (b. 1938).** The internetworking
construction of §14.2 is theirs. Faced in 1973 with three incompatible ARPA
networks — the ARPANET, a packet radio network and a satellite network — whose
operators had no intention of converging, they proposed a gateway and a common
packet format rather than a common network.

The decision to assume **almost nothing** about the constituent networks is the one
that mattered. Cerf has described the design constraint as needing to work over
networks whose properties they could not know and could not change, and the
resulting minimalism is why IP later ran over technologies neither of them
anticipated. Both received the Turing Award in 2004. Chapter 23 covers the paper.

**Jon Postel (1943–1998).** Edited RFC 791 and RFC 793, ran the numbers registry
that became IANA, and for two decades was the person who decided what a protocol
number meant. His **robustness principle** — *be conservative in what you send, be
liberal in what you accept* — appears in RFC 761 and is one of the most quoted and
most argued-about design maxims in the field.

The argument is worth knowing: liberal acceptance made early interoperation possible
and, over decades, allowed sloppy implementations to proliferate and become the de
facto specification, since anything that worked in practice stayed. RFC 9413 (2023)
formally revisits it. Postel's death in 1998 prompted a genuine institutional crisis
about how much of the Internet's coordination had rested on one person's judgement.

**Barry Leiner, David Clark, and the authors of "A Brief History of the Internet"
(1997).** Their account is the standard reference for how the architecture developed
and is unusually explicit about **what was not the motivation** — including the
nuclear-war story that Chapter 13's notes correct.

**David Clark (b. 1944).** MIT, and the Internet Architecture Board's chief protocol
architect through the 1980s. Co-author of the end-to-end paper (Chapter 23), author
of the "rough consensus and running code" formulation (Chapter 48 §48.4), and — more
relevant here — of *The Design Philosophy of the DARPA Internet Protocols* (1988),
which lists the architecture's goals **in priority order** and explains what was
traded against what. It is the most useful document for understanding why the
Internet is the way it is, and its ranking of goals (survivability high,
accountability last) explains a great deal about what the architecture is bad at.

**Tim Berners-Lee (b. 1955).** Included in §14.3's argument rather than for the web
itself. He invented and deployed HTTP and HTML at CERN **without asking any network
operator's permission**, which is the property Chapter 13 §13.4 identified as
packet switching's most consequential and which §14.2's hourglass makes possible.
The counterfactual is worth holding: in a network with intelligence in the core, the
web would have required a carrier to implement it.

**Robert Kahn, again**, for the *open architecture networking* principle: that each
constituent network should be able to keep its own internal design, and that the
internetwork should require no changes to it. This is §14.2's black-box framing, and
it is the reason the construction is recursive.

**Van Jacobson (b. 1950).** His later work on Content-Centric and Named Data
Networking is an argument that §14.3's re-centralisation reveals a mismatch: the
architecture addresses **hosts**, while what users and applications actually want is
**content**, and CDNs are an elaborate workaround for that mismatch. Whether the
argument is right is genuinely open; it is the most serious attempt to reconsider the
waist of the hourglass, and it is worth knowing that a serious attempt exists.

**Marc Andreessen (b. 1971) and the Mosaic team**, and — differently — **the
Napster, Gnutella and BitTorrent authors**. Between them they produced §14.3's
oscillation: the web made the Internet overwhelmingly client–server, peer-to-peer
file sharing pushed it back, and NAT plus the cloud pushed it forward again. Bram
Cohen's BitTorrent (2001) is the technically interesting one, since its design
assumes participants who cannot receive inbound connections and works anyway.

**The ITU-T Study Group 12 authors of Recommendation G.114.** The 150 ms one-way
delay budget for interactive speech, which is the number every converged-network
design in §14.4 is measured against. It is a psychoacoustic result rather than an
engineering one — derived from what conversation participants notice — and it is
one of the few hard human-factors constraints in this book.
