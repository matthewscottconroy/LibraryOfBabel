# Chapter 32 — The People

**Yakov Rekhter (1953–2023), Kirk Lougheed and Susan Hares.** BGP, and the famous origin
story: **Rekhter and Lougheed sketched the protocol on three paper napkins** at an IETF
meeting in 1989. It was published as RFC 1105 and became known as the **"three napkin
protocol"**.

The story is told as a charming anecdote and it is better understood as a warning.
**A protocol designed on napkins in 1989 now carries every packet that crosses an
organisational boundary on Earth**, and its security properties — §32.4's absence of any
authorisation — are the properties of a napkin sketch among colleagues who trusted each
other.

**The design's durability is genuinely remarkable.** BGP-4 (RFC 4271) is recognisably the
same protocol, scaled from a few hundred prefixes to nearly a million and from dozens of
ASes to 75,000, with extensions rather than replacement. **Nothing else in this book has
scaled by four orders of magnitude without being redesigned.**

**Rekhter** is the most consequential person in the routing half of this book — BGP, MPLS,
VPN architectures, and CIDR's address allocation architecture (Chapter 26). He worked at
IBM, Cisco and Juniper across four decades.

**Susan Hares** co-authored the BGP-4 specification and chaired the IDR working group for
years — the unglamorous, essential work of keeping a protocol coherent while everyone
extends it.

**Kirk Lougheed** was at Cisco, and co-founded it in a sense that matters here: the
implementation and the specification advanced together, which is why BGP worked before it
was finished. Chapter 23's rough-consensus-and-running-code, again.

**Jon Postel and the EGP authors.** BGP replaced **EGP** (RFC 904), which assumed a
**tree-structured** Internet with a single backbone at the root. That assumption held
while NSFNET was the backbone and collapsed the moment commercial networks began peering
with each other.

**EGP's failure is instructive:** it was not badly built, it was built for a topology that
stopped existing. **BGP's path vector makes no assumption about the shape of the graph**,
which is why it survived the flattening of §32.3 that would have destroyed any protocol
assuming a hierarchy.

**Geoff Huston (b. 1954).** APNIC, and the person who has measured the global routing
table continuously since the 1990s. Every figure in this chapter — 75,000 ASes, ~950,000
prefixes, RPKI coverage — comes from his datasets at bgp.potaroo.net.

**His long-running argument** is that routing table growth is a collective-action problem:
each network's decision to announce more specifics is individually rational and
collectively expensive, and nothing prices the externality. Worth reading against the
comfortable account.

**Sean Doran, Curtis Villamizar and the Route Flap Damping authors.** RFC 2439 — the
mechanism for suppressing a prefix that flaps repeatedly, so that instability at one edge
does not consume the whole Internet's CPU.

**And then it was largely turned off.** Research in the 2000s showed that damping's
default parameters suppressed *stable* prefixes for far too long after transient events —
the cure was worse than the disease. RIPE published revised recommendations, and the
episode is a good example of a mechanism that was correct in intent, wrong in
calibration, and dangerous because everyone deployed the defaults.

**The AS 7007 operator, April 1997.** Unnamed, and the incident bears the AS number rather
than the person's, which is merciful and correct — **the failure was the industry's, not
an individual's.** A single router misconfiguration should not have been able to disrupt
the global Internet, and the fact that it could was a property of universal
under-filtering rather than of one engineer's mistake.

**The incident is why prefix filtering became standard practice**, and §32.4's list shows
how incompletely.

**Randy Bush (b. 1949) and Rob Austein.** **RPKI** — RFC 6480 and the surrounding
architecture. Bush has been among the most persistent advocates for routing security, over
two decades, largely against indifference.

**His public argument is the honest one:** the technical problem was solved long ago and
the obstacle is entirely economic and organisational. A network that validates gains
nothing until others sign. **He continued anyway**, and RPKI's crossing of the fifty
per cent threshold around 2023 is substantially the result of that persistence plus the
commercial shock of the 2018 attacks.

Bush is also worth reading on **why operators resist security mechanisms**, which is less
about laziness than about the real risk that a validation failure takes your own network
off the air.

**Andrei Robachevsky and the MANRS initiative.** Mutually Agreed Norms for Routing
Security, from the Internet Society — a public commitment to filtering, anti-spoofing,
coordination, and validation.

**The insight is social rather than technical.** MANRS adds no mechanism. It creates a
**public list of networks that have committed**, which turns "we filter properly" from an
unverifiable claim into a reputational asset — and gives a network's customers something to
ask about.

**Making good behaviour visible is sometimes more effective than making bad behaviour
impossible**, and MANRS is the clearest example in this book.

**The Cloudflare, RIPE NCC and NLnet Labs engineers** who built and gave away the RPKI
validators, the monitoring tools, and the public post-mortems.

**The post-mortems deserve specific credit.** Cloudflare's account of the June 2019 leak
names its own losses, explains the mechanism precisely, and identifies what each party
should have done. **Publishing that is a genuine contribution to the field**, and it is
why §32.4 can be specific rather than vague.
