# Chapter 29 — The People

**Vint Cerf (b. 1943) and Bob Kahn (b. 1938).** The hop-by-hop, destination-based,
stateless forwarding model of §29.1 is theirs, and Chapter 23 covers the reasoning. The
decision that belongs here is the negative one: **gateways keep no per-connection state**,
which is what allows a router to be replaced mid-conversation and what makes forwarding a
table lookup rather than a session operation.

**Louis Pouzin (b. 1931).** The datagram. Every property in §29.1 — no path knowledge, no
state, best-effort, independent per-packet decisions — is his model before it is anyone
else's.

**Donald Davies (1924–2000) and Paul Baran (1926–2011).** Chapter 13's subjects, and the
originators of the argument that a network of independently-forwarding nodes survives
damage that a circuit-switched network does not. §29.1's "the path emerges" is their
insight expressed as a mechanism.

**The CIDR authors — Fuller, Li, Yu, Varadhan, Rekhter.** Longest-prefix match existed
before CIDR in a limited form; **CIDR made it the fundamental operation.** Under classful
addressing the mask was implied by the address, so a lookup was closer to an exact match
on a network number. Once prefixes could be any length and could nest, the router's
question changed from *"which network is this?"* to *"which of my many overlapping rules
is most specific?"*

**That change is what made §29.3 the core algorithm of the Internet**, and it also made it
computationally much harder — which motivated everything below.

**Tony Li.** Cisco, Juniper, and much of the work on making longest-prefix match feasible
at line rate. §29.1's claim that aggregation is a physical rather than aesthetic necessity
rests on the properties of the hardware, and Li is among the people who established what
those properties are.

His long-running argument — that the current routing architecture has a finite remaining
lifespan because table growth outpaces affordable TCAM — is worth reading against
Chapter 32's more comfortable account.

**Stefan Nilsson and Gunnar Karlsson.** The **LC-trie** (1999), a level-compressed trie
for fast longest-prefix lookup in software. **The Linux kernel's IPv4 route lookup uses an
LC-trie**, so this algorithm runs on essentially every Linux machine on Earth, which is a
reasonable claim to influence.

**Mikael Degermark, Andrej Brodnik, Svante Carlsson and Stephen Pink.** *Small Forwarding
Tables for Fast Routing Lookups* (SIGCOMM 1997) — compressing a full Internet routing
table small enough to fit in a processor cache, so that a software lookup could keep pace
with hardware.

Their paper is a good example of a recurring result in systems: **the win came from
changing the data structure so that the working set fit in a faster memory**, not from a
cleverer algorithm.

**Pankaj Gupta and Nick McKeown.** Stanford, and much of the analysis of packet
classification and lookup algorithms — including the systematic treatment of what is
achievable at what cost in hardware. McKeown appears in Chapter 17 for virtual output
queueing and in Chapter 68 for OpenFlow, and the thread through all three is the same:
**work out precisely what the hardware can do, then design the protocol to fit.**

**Van Jacobson (b. 1950).** `traceroute` (Chapter 24), and the reason §29.1's "nobody
knows the path" is an observable fact rather than an assertion — his tool is what let
people see the emergent path for the first time.

**Radia Perlman (b. 1951).** Her general statement of why hierarchy is unavoidable — *a
flat namespace of size n requires n state everywhere; a hierarchical one requires log n* —
is what §29.3's aggregation-with-exceptions structure implements. Chapter 26's notes
develop it.

**The operators of the default-free zone.** §29.4's default-free routers are run by a few
thousand people worldwide who collectively hold the entire routing table in their
networks and, largely, in their heads. There is no organisation, no authority and no
contract binding them — the arrangement is Chapter 32's subject.

The relevant point for this chapter: **the /24 filtering convention that limits
more-specific hijacking has no protocol basis whatever.** It is a norm, enforced by each
operator filtering their neighbours, and it is one of several places where the Internet's
stability rests on shared practice rather than on any mechanism. Chapter 32 §32.4 examines
what happens when the norm fails.
