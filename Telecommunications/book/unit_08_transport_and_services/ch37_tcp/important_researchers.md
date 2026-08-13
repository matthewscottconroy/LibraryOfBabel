# Chapter 37 — The People

**Vint Cerf (b. 1943) and Bob Kahn (b. 1938).** The 1974 protocol, and the **1978 split**
that made TCP one option rather than the only one (Chapter 23 §23.1).

**What belongs in this chapter** is the sliding window and the cumulative acknowledgement —
mechanisms borrowed from earlier link-layer protocols and applied end to end across a
network that could reorder, duplicate and delay arbitrarily. **The generalisation was the
hard part**: a link-layer window operates over a wire with bounded delay, and TCP's operates
over a path whose delay is unknown and variable.

**Jon Postel (1943–1998).** **RFC 793**, September 1981, and the state diagram of §37.5.

**The specification stood for forty-one years.** RFC 9293 (2022) consolidated it with its
accumulated errata and extensions — and is recognisably the same protocol. **The state
machine in particular has not changed at all.**

**Van Jacobson (b. 1950).** His fourth appearance, and the most substantial.

**The RTT estimator of §37.3** is his, from the 1988 SIGCOMM paper that also gave the
congestion control of Chapter 38. **The insight is small and consequential: track the
variance, not merely the mean**, because a timeout on an erratic path must be generous in
proportion to how erratic it is.

**And the constants matter.** Choosing 1/8 and 1/4 made the whole calculation **shifts and
adds** — no multiplication, no floating point — which on a 1988 VAX was the difference
between affordable and not. **It still runs on every packet you send.**

**Phil Karn (b. 1956).** **Karn's algorithm**, 1987, with Craig Partridge.

**The problem is deceptively subtle:** an ACK arriving after a retransmission is ambiguous,
and *either* guess corrupts the estimator — one way makes the RTO too short and causes more
spurious retransmissions; the other makes it too long and cripples recovery.

**His answer is a refusal:** do not take an ambiguous sample at all. **Combined with
exponential backoff**, because refusing to measure means the estimator cannot adapt during
exactly the period when it must.

**The pairing is the elegant part** — a measurement rule that gives up, plus a
non-measurement rule that covers the gap it leaves.

Karn is also an amateur radio operator (KA9Q) whose TCP/IP implementation for packet radio
was, for years, the way many people first ran the protocol on a personal computer.

**Craig Partridge (b. 1961).** Karn's co-author, and a great deal of the transport work of
the 1980s and 90s — including RFC 1546 (anycast, Chapter 27) and the measurement studies
that established what the Internet's delay and loss actually looked like.

**Steve Bellovin (b. 1951).** *Security Problems in the TCP/IP Protocol Suite* (1989),
which described **sequence number prediction** — §37.1's attack — five years before Mitnick
used it.

**The paper is the founding document of network security**, and its position in this
chapter is a specific lesson: **the vulnerability was published, understood, and not fixed
for years**, because fixing it required changing every implementation and nobody had been
hurt yet.

**He later authored RFC 6528**, the defensive ISN generation, in 2012 — **twenty-three
years after describing the problem.**

**Tsutomu Shimomura and Kevin Mitnick.** The December 1994 attack. Shimomura's analysis of
his own compromise is the clearest account of a sequence-prediction attack in practice, and
the publicity did more to get ISN randomisation deployed than the preceding five years of
papers.

**A demonstrated attack moves an industry that a described one does not** — which recurs
throughout this book, in Chapter 18's ARP tooling and Chapter 32's BGP hijacks.

**John Nagle (b. 1949).** **RFC 896** (1984), and the algorithm that bears his name.

**Written to solve a specific problem at Ford Aerospace:** interactive traffic generating
41-byte packets to carry one byte, saturating a link with headers.

**And he is on record as being unimpressed by the interaction with delayed ACK** (§37.4),
which is not his mechanism and which combines with his to produce the 200 ms stall. **His
position is that the delayed-ACK timer is the mistake**, and there is a reasonable case for
it: Nagle's algorithm has a clear rationale and delayed ACK is a heuristic that guesses
wrong on request-response traffic.

**Nagle also wrote the 1985 RFC 970 on fair queueing**, which anticipated a great deal of
Chapter 52.

**David D. Clark (b. 1944).** **Silly window syndrome** and the receiver-side fix (§37.4),
plus RFC 813. His appearances throughout this book — the end-to-end argument, the design
philosophy paper, "rough consensus and running code" — make him the closest thing the
Internet has to a resident architect and critic.

**Matt Mathis, Jamshid Mahdavi, Sally Floyd and Allyn Romanow.** **SACK** (RFC 2018).

**The mechanism took eight years to become universal** after specification, and its absence
is still occasionally observable when a middlebox strips the option. **The gap between
"specified" and "deployed" is a recurring measurement in this book**, and SACK's is one of
the more successful.

**Sally Floyd (1950–2019)** deserves separate mention: SACK, ECN, RED, NewReno, and much of
the analytical foundation of congestion control (Chapter 38). **She is among the most
consequential researchers in the history of the Internet**, and her work is why the
transport layer behaves as well as it does under stress.

**Van Jacobson, Bob Braden and Dave Borman.** **RFC 1323**, now **RFC 7323** — window
scaling, timestamps and PAWS (§37.2, §37.4).

**Window scaling is the fix for a field that was sized in 1981** for links three orders of
magnitude slower. **The 64 KB wall is the clearest example in this book of a protocol field
outliving its assumptions**, and the fix — an option that multiplies rather than a wider
field — was chosen precisely because it could be deployed incrementally (Chapter 23 §23.1).

**Daniel J. Bernstein (b. 1971).** **SYN cookies**, 1996.

**The idea is genuinely beautiful:** rather than defending the state, **eliminate the
state** — encode it in a value the client must echo back, so the server allocates nothing
until the handshake is proven genuine.

**The general technique — put the state in the token and verify it cryptographically —
appears everywhere since**: stateless session cookies, JWTs, DTLS cookies, QUIC's retry
tokens. **Bernstein's is the earliest widely-deployed instance.**

**Yuchung Cheng, Neal Cardwell, Nandita Dukkipati and the Google TCP team.** **TLP** and
**RACK** (RFC 8985) — §37.3's modern loss detection.

**RACK is the more significant.** Replacing "three duplicate ACKs" with **time-based
reasoning about what should have arrived by now** handles reordering and tail loss in one
mechanism, and it is the first fundamental change to TCP's loss detection since fast
retransmit in 1990.

**Their motivation was measured, not theoretical:** at Google's scale, **tail loss was
demonstrably a major contributor to page-load latency**, and the existing mechanisms could
not address it. Chapter 38 §38.3's BBR comes from the same group and the same method —
**measure what actually limits performance, then design for that.**
