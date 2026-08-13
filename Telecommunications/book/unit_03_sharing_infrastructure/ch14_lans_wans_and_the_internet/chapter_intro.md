# Chapter 14 — LANs, WANs, and the Internet

Networking has a habit of turning engineering constraints into vocabulary, and
then forgetting that the vocabulary was ever a constraint. "LAN" and "WAN" are the
clearest example. They sound like categories of network. They are, in origin,
categories of *problem*, and the problems have shifted enough that the words now
mislead as often as they help.

## What the distinction originally was

In 1980, connecting two machines in the same building and connecting two machines
in different cities were not variations on one task. They were different
industries.

Inside a building you owned the cable. You could run whatever you liked over it —
a shared coaxial bus at 10 Mb/s, which was faster than any computer of the era
could fill. Latency was microseconds. Errors were rare enough to ignore.
Nobody billed you per byte. If it broke you walked to it with a screwdriver.

Between cities you owned nothing. You leased a circuit from a telephone company,
at 9,600 bits per second, for a monthly fee that would buy a decent car over the
life of the contract. Latency was tens of milliseconds and error rates were high
enough that link-layer error recovery was mandatory. If it broke you telephoned
someone and waited.

Those two environments differed by three orders of magnitude in speed, three in
cost per bit, and three in latency. Protocols designed for one were useless on the
other, and the LAN/WAN distinction encoded that reality honestly.

## What changed

Essentially all of it, and the erosion is worth tracking because it explains a
great deal of modern design.

**Speed converged.** A 100 Gb/s wide-area link is entirely ordinary in 2026, and
faster than the internal fabric of most enterprise buildings. The rate gap has
inverted in places: many organisations have a faster path to their cloud provider
than between two floors of their own office.

**Cost per bit collapsed.** Wide-area bandwidth is no longer metered in a way that
shapes protocol design for most organisations. The engineering effort that once
went into minimising bytes on a WAN link now goes elsewhere.

**Latency did not change at all.** This is the crucial asymmetry. Propagation delay
is fixed by the speed of light (Chapter 1 §1.1), and it is the *only* one of the
original distinctions that is as true today as in 1980. Chicago to Frankfurt was
about 35 ms one way in 1980 and it is about 35 ms one way now, and it will be
about 35 ms one way in 2075.

So the modern form of the LAN/WAN distinction is not about ownership or speed or
cost. **It is about latency, and therefore about round trips**, which is exactly
the lesson of Chapter 3 §3.4. An application that performs twenty round trips per
operation is fine on a LAN and unusable on a WAN, and no amount of bandwidth
changes that. Every other difference has eroded; this one cannot.

## Scope as an engineering variable

The chapter treats the scope categories — PAN, LAN, CAN, MAN, WAN — as a spectrum
rather than a taxonomy, because what actually varies along it is a set of
continuous parameters:

| Scope | Typical span | What dominates the design |
|---|---|---|
| PAN | metres | Power budget; radio coexistence |
| LAN | building | Switching, broadcast domain size, VLANs |
| CAN | campus | Fibre plant, distribution layer, routing |
| MAN | city | Provider relationships, redundant paths |
| WAN | continental+ | Latency, cost, availability, encryption |

The useful skill is not classifying a network into a box but reading off which
parameters bind. A warehouse WLAN and a metropolitan fibre ring are both
"networks"; what makes them different engineering problems is that one is
constrained by RF contention and the other by fibre availability and provider
diversity.

## The network of networks

The second half of the chapter is about the construction that gives the Internet
its name. An **internetwork** is what you get when you connect networks — not
hosts — and treat each constituent network as a black box that delivers packets
internally by means you do not need to know.

This is a genuinely different idea from "a big network," and it is what Cerf and
Kahn contributed in 1974 (Chapter 23). It requires:

- A **universal address space** meaningful across every constituent network,
  because a MAC address is not (Chapter 24).
- A **device that joins networks** and makes forwarding decisions between them —
  the router (Chapter 29).
- A **lowest-common-denominator service**, since the only guarantees available
  across all constituent networks are the ones the weakest of them can provide.
  This is why IP is best-effort: not because best-effort is desirable, but because
  it is the most that can be promised universally.
- A **way for independently administered networks to exchange reachability
  information** without any central authority — which is BGP (Chapter 32), and
  which is why the Internet has no off switch and no owner.

The chapter closes with **convergence**: the fact that voice, video, television,
industrial control and data now all run over the same packet infrastructure. This
was neither obvious nor welcome when it was proposed. Telephone engineers had
excellent reasons to believe that voice needed a purpose-built network, and they
were right about the requirements and wrong about the trajectory. The pattern —
a general-purpose substrate absorbing a specialised one because the economics of
the substrate improve faster — recurs often enough to be worth naming, and it is
what Chapters 67 through 70 describe happening again to networking hardware itself.

## What this chapter does

§14.1 covers the scope categories and what actually varies across them.

§14.2 develops the internetworking construction and its four requirements.

§14.3 covers architectural patterns: client–server, peer-to-peer, three-tier
applications, and the cloud's re-centralisation — with an honest look at how much
of the "decentralised" Internet now depends on a handful of providers.

§14.4 covers convergence, VoIP's displacement of the PSTN, and what happens when a
network carries traffic with incompatible requirements.

## By the end you will be able to

- State which of the classical LAN/WAN differences still hold and which have
  eroded, and explain why latency is the durable one.
- Explain what an internetwork is and why it requires a universal address space, a
  router, and a best-effort service model.
- Identify the architectural pattern of a described application and its
  consequences for network design.
- Explain why convergence happened and what it cost.
