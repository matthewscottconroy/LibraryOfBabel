# Unit III — Many Machines, One Infrastructure

Everything so far has been about *one* link: one sender, one receiver, one channel
with a capacity. Unit II ended with a catalogue of media and a procedure for
choosing between them, and if the world contained exactly two computers we would
now be finished.

The world contains rather more, and this unit is about the consequence.

## The question

Suppose you have *n* devices and each may need to communicate with any other. The
obvious construction — a dedicated link between every pair — requires

$$\frac{n(n-1)}{2}$$

links. For 5 devices, 10 links. For 100 devices, 4,950. For the roughly 20 billion
connected devices now in existence, about 2 × 10²⁰ links, which exceeds the number
of grains of sand on Earth by several orders of magnitude and would require every
device to have 20 billion network ports.

So full mesh is out, immediately and permanently, and the entire architecture of
every network ever built follows from that arithmetic. **Devices must share
infrastructure**, and sharing requires a mechanism for deciding who gets what,
when.

There have been exactly two great answers to that question, and they represent
genuinely different philosophies about what a network is for. This unit is the
argument between them.

## The first answer: reserve a path

**Circuit switching** sets up a dedicated end-to-end path for the duration of a
conversation, and tears it down afterwards. Between the setup and the teardown,
the path is yours: the bandwidth is guaranteed, the delay is constant, the
sequence is preserved, and nothing anyone else does can affect you.

This is how the telephone network worked for a hundred years, and it worked
extraordinarily well. A telephone call in 1985 had better and more predictable
audio characteristics than most video calls do today. The network could tell you,
before you started, whether it could serve you — that is what a busy signal *is*,
and it is an honest and useful thing for a network to say.

Chapter 12 treats this properly, and treats it with respect rather than as a
quaint precursor. Understanding what circuit switching bought is necessary to
understand what packet switching gave up, and a great deal of modern networking —
MPLS, QoS, network slicing in 5G, deterministic Ethernet — is an attempt to buy
some of it back.

## The second answer: label the data and let it find its way

**Packet switching** abandons the reserved path entirely. Data is chopped into
labelled chunks; each chunk carries the destination's address; each intermediate
node looks at the address and forwards the chunk onward. No setup, no reservation,
no guarantee. When the network is busy, your chunks wait, or are discarded.

By the standards of a telephone engineer in 1965, this was an obviously
irresponsible design. It offers no assurance of anything. It wastes capacity on
address headers attached to every single chunk. It permits congestion collapse.
It cannot tell you in advance whether it can serve you, and it cannot promise that
the service it gives you now will be the service it gives you in ten seconds.

It also, per the arithmetic in Chapter 9 §9.3, uses the underlying capacity
between five and a hundred times more efficiently for bursty traffic — and
computer traffic is extraordinarily bursty. Chapter 13 works through why that
efficiency argument won, and why it kept winning until the telephone network
itself was eventually rebuilt on top of packet infrastructure, which is a reversal
worth pausing over.

## What the unit contains

**Chapter 11 — Topologies.** The combinatorial argument above, then the shapes
networks actually take: bus, ring, star, mesh, hybrid, and the crucial distinction
between *physical* topology (where the cables go) and *logical* topology (how the
signal behaves). A modern Ethernet is physically a star and was logically a bus,
and knowing which one a fact applies to resolves a great deal of confusion. The
chapter ends with hierarchical design — access, distribution, core — which is the
structure most enterprise networks still use and the reason they scale.

**Chapter 12 — Circuit Switching and the PSTN.** The operator, the crossbar, the
electronic exchange; the digitisation of voice and the birth of the DS0;
out-of-band signalling and SS7; and Erlang's blocking formula, which is how you
decide how many circuits an exchange needs and which is still used to staff call
centres.

**Chapter 13 — Packet Switching.** Paul Baran at RAND and Donald Davies at NPL,
arriving independently at the same idea for entirely different reasons; datagrams
versus virtual circuits; store-and-forward and the queue; and the efficiency
argument, worked with numbers.

**Chapter 14 — LANs, WANs, and the Internet.** Scope as an engineering variable;
the network of networks and what "internetwork" actually means; the architectural
patterns (client–server, peer-to-peer, and the cloud's quiet re-centralisation);
and convergence — the fact that voice, video, television and data now all run over
the same infrastructure, which was not obvious and was resisted.

## Reading this unit

There is a temptation to treat Chapter 12 as history to be skimmed. Resist it for
two reasons.

First, the PSTN's design decisions are still load-bearing. The 64 kb/s DS0 shapes
every VoIP codec discussion. SS7's separation of signalling from media is exactly
the architecture SIP uses. The telephone network's willingness to say "no" —
the busy signal — is what admission control means, and it is what your video
conferencing system does not do and should.

Second, the packet-versus-circuit argument is not settled history. It is being
re-fought right now, in 5G network slicing, in time-sensitive networking for
industrial control, and in every data centre that implements lossless Ethernet for
storage traffic. The people re-fighting it are rediscovering, sometimes painfully,
what the telephone engineers knew.
