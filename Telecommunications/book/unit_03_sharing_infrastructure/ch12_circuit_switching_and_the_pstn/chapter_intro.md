# Chapter 12 — Circuit Switching and the PSTN

Almon Brown Strowger was an undertaker in Kansas City, and in 1888 he became
convinced — probably correctly — that the local telephone operator was diverting
calls intended for his business to a competitor, whose wife worked at the exchange.

Strowger's response was not to complain. It was to design, patent, and manufacture
an automatic telephone exchange that removed the operator from the process
entirely. The Strowger switch — a rotating, stepping electromechanical selector
driven directly by pulses from the caller's dial — went into service in La Porte,
Indiana, in 1892, and variants of it were still switching calls in some parts of
the world into the 1990s. A hundred years of infrastructure, from a grudge.

The story is worth telling for more than its comedy. It contains the two ideas
that define circuit switching. The first is that a **physical path** is
constructed, metal to metal, between two subscribers, and held for the duration of
the call. The second is that the *signalling* — the information about which path
to build — is distinct from the *conversation*, and the history of telephony is
substantially the history of separating those two things more and more cleanly,
ending in SS7 and, ultimately, in SIP.

## Why this chapter is not nostalgia

The public switched telephone network is a legitimate object of study for a
networking student in 2026, and not because the exam mentions it. Four reasons.

**It is the counterexample that defines packet switching.** You cannot understand
what the Internet gave up without understanding what the PSTN provided. Guaranteed
bandwidth. Constant delay. In-order delivery. Admission control — an honest
"no" when capacity was unavailable, delivered before you invested any effort.
Five-nines availability as a *design target that was routinely met*. Every one of
those is something the Internet does not do, and every one of them is something
some part of the modern industry is currently trying to reintroduce.

**Its numbers are still in use.** The 64 kb/s DS0 derived in Chapter 4 §4.2 is
still the reference against which every voice codec is measured. The T1 at
1.544 Mb/s and the E1 at 2.048 Mb/s still define circuit sizes in carrier
contracts. The µ-law and A-law companding curves are in every VoIP gateway.

**Its architecture was recreated.** SS7 separates a signalling network from a
media network, so that call setup travels a different path from the audio. SIP
(Chapter 41) does exactly this. RTP carries the media; SIP carries the signalling;
they take different paths and have different requirements. The engineers who
designed SIP knew precisely what they were reimplementing.

**Its economics explain the modern industry.** The regulatory structures, the
interconnect obligations, the universal service requirements, and the settlement
arrangements between carriers were all built for the PSTN, and the current shape
of the telecommunications industry — including who owns the fibre in your street —
is largely a consequence of them.

## The arc of the chapter

We follow the network's evolution as a sequence of problems and solutions, which
is also the sequence in which its ideas became reusable.

**Manual switching** solves the connection problem with a human being and a patch
cord. It does not scale, it is slow, and — as Strowger discovered — it is not
neutral.

**Electromechanical switching** automates the human away, and introduces the idea
of a *selector* driven by *signalling* from the subscriber. The dial pulse is the
first signalling protocol most people ever used.

**Analog transmission** limits the network's reach, because as Chapter 5 shows,
amplifying an analog signal amplifies its accumulated noise. A transcontinental
analog call in 1930 required careful engineering and sounded like it.

**Digitisation** solves that: sample at 8 kHz, quantise to 8 bits, and now the
signal can be *regenerated* rather than amplified, perfectly, indefinitely. This is
the largest improvement in the network's history and it produced the DS0.

**Time-division multiplexing** then packs 24 or 30 of those DS0s onto one physical
circuit, giving the T1 and E1, and the digital hierarchy grows upward from there
into Chapter 50's SONET.

**Common channel signalling** moves the call-setup information out of the voice
channel and onto a separate packet network — which is to say, the telephone
network's control plane became a packet network in 1975, twenty years before its
data plane did. SS7 is that network, and its security assumptions, made when only
a handful of trusted carriers existed, are the reason SS7 attacks against SMS
two-factor authentication remain viable today.

**Erlang's mathematics** underlies all of it: how many circuits does an exchange
need so that the probability of blocking is acceptably small? The answer is not
"one per subscriber," and the gap between that and the actual answer is the same
statistical multiplexing gain that Chapter 9 §9.3 computed and that Chapter 13
will claim as packet switching's founding argument.

## What this chapter does

§12.1 covers manual and electromechanical switching, the hierarchy of exchanges,
and the local loop.

§12.2 covers digitisation: sampling, quantisation, companding, the DS0, and the
T-carrier and E-carrier hierarchies.

§12.3 covers signalling: in-band versus out-of-band, the blue box and the security
lesson it taught, SS7's architecture, and the parallel with modern SIP.

§12.4 covers Erlang's traffic theory: the erlang as a unit, the Erlang B formula,
grade of service, and how to dimension a trunk group — plus its direct application
to modern capacity planning.

## By the end you will be able to

- Explain what a circuit is and enumerate exactly what a reserved circuit
  guarantees.
- Trace the digitisation of a voice signal and derive the 64 kb/s DS0 from the
  sampling theorem.
- State the composition of a T1 and an E1 and explain why they differ.
- Explain what out-of-band signalling is, why it was adopted, and what security
  assumption it embedded.
- Use the Erlang B formula to size a trunk group for a given call volume and
  blocking probability.
- Articulate, without caricature, what circuit switching does better than packet
  switching — which is the necessary preparation for Chapter 13.
