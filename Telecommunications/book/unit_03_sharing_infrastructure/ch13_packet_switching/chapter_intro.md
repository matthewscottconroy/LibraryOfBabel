# Chapter 13 — Packet Switching

Between 1960 and 1965, two men who had never met, working on opposite sides of the
Atlantic, for organisations with entirely different purposes, invented the same
thing.

**Paul Baran** was at the RAND Corporation in Santa Monica, and his problem was
nuclear war. The American command-and-control network of the late 1950s was
centralised and would not survive a first strike; Baran's assignment was to design
a communications system that would. Between 1960 and 1964 he produced a series of
reports, culminating in the eleven-volume *On Distributed Communications*, arguing
for a **distributed** network — a mesh with high redundancy — in which messages
were broken into standard-sized blocks that he called *message blocks*, each
routed independently by nodes performing what he termed "hot-potato routing":
pass it on immediately, in whatever direction is currently best.

AT&T, to whom the design was shown, explained that it would not work. Baran later
recalled being told, more or less, that the people proposing it did not understand
how telephony worked. He was not entirely wrong about their objection: it *would*
not have worked with the technology of 1964, and building it would have required
AT&T to admit that the network it had spent eighty years perfecting was the wrong
shape.

**Donald Davies** was at the National Physical Laboratory in Teddington, and his
problem was time-sharing. Computers were expensive and interactive terminal
sessions were bursty — a user types for a moment, then thinks for thirty seconds —
so dedicating a circuit to a terminal wasted almost all of it. In 1965 Davies
independently arrived at the same architecture, and gave it the word we use. He
chose *packet* deliberately: he wanted a short, common English word that would
translate cleanly into other languages, and he consulted a linguist about it.

When Davies presented his ideas in 1966 he learned of Baran's work, and the two
designs were found to be substantially identical. Larry Roberts, designing the
ARPANET, took ideas from both, plus Leonard Kleinrock's queueing analysis, and the
first ARPANET node was installed at UCLA in September 1969.

## Two problems, one answer

The convergence is worth dwelling on, because it tells you something about the
strength of the idea.

Baran wanted **survivability**. If any single node can be destroyed, no node may be
essential; therefore no path may be pre-established; therefore each unit of data
must carry its own destination and be routed independently at every hop.

Davies wanted **efficiency**. If traffic is bursty, reserving capacity for the
peaks wastes it during the troughs; therefore capacity must not be reserved;
therefore each unit of data must contend for the link when it has something to
send, and must carry its own destination because there is no reservation to
identify it.

**Different requirement, identical mechanism.** Packet switching is the answer to
both "how do we survive losing pieces of the network" and "how do we avoid paying
for idle capacity," and the fact that one design satisfies two unrelated
constraints is a large part of why it displaced everything else.

## The efficiency argument, since it is the one that won

Baran's survivability case is the more famous story, but it was Davies's
efficiency case that actually determined the outcome, and it deserves the numbers.

Consider 100 users, each needing 1 Mb/s while active, each active 5% of the time.

**Circuit switching** must reserve 1 Mb/s per user for the duration of each
session. To serve all 100 simultaneously requires 100 Mb/s, and that capacity sits
idle 95% of the time. Alternatively, provision fewer circuits and block users when
they are all busy — Erlang's calculation from Chapter 12.

**Packet switching** provisions for the *aggregate*. The expected load is
100 × 0.05 × 1 Mb/s = 5 Mb/s. Provision 20 Mb/s and ask: what is the probability
that more than 20 users are simultaneously active? By the binomial distribution
with *n* = 100, *p* = 0.05, that probability is under 10⁻⁶.

**One fifth of the capacity, essentially the same service.** That is the
statistical multiplexing gain, and it grows with the number of users and with the
burstiness of the traffic. Computer traffic is extraordinarily bursty, so the gain
is large, and it compounds at every level of aggregation in the network.

§13.4 works this properly and also states the price honestly, because there is one:
that "essentially the same service" hides an occasional queue, and an occasional
discard, and no way to promise otherwise in advance.

## What packet switching gave up

This chapter does not present packet switching as an unalloyed triumph, because it
was not, and pretending otherwise leaves you unable to explain why so much modern
engineering effort goes into partially undoing it.

Gone: guaranteed bandwidth, constant delay, in-order delivery, admission control,
and any ability to say in advance what service you will receive. In exchange:
efficiency, survivability, and the ability to add a new application without asking
anyone's permission — which turned out to matter more than any of the guarantees.

The subsequent fifty years have been spent selectively buying guarantees back.
Virtual circuits (X.25, Frame Relay, ATM). Integrated and differentiated services.
MPLS traffic engineering. Data centre lossless Ethernet. Time-sensitive networking.
5G network slicing. Every one of these is an attempt to reintroduce, for some
traffic, a property that circuit switching provided for all traffic — and each is
harder than it looks, for reasons this chapter establishes.

## What this chapter does

§13.1 covers Baran and Davies, the distributed network argument, and the
survivability case.

§13.2 distinguishes **datagram** from **virtual circuit** packet switching — the
connectionless and connection-oriented variants — with X.25, Frame Relay and ATM as
the virtual-circuit examples and IP as the datagram one, and explains why the
datagram model won the Internet and the virtual-circuit model won inside carriers.

§13.3 covers store-and-forward operation, the queue, and where the delay of
Chapter 3 §3.2 actually accumulates.

§13.4 works the efficiency argument with numbers and states the costs.

## By the end you will be able to

- Explain the two independent motivations for packet switching and why one
  mechanism serves both.
- Compute statistical multiplexing gain for a stated user population and traffic
  profile.
- Distinguish datagram from virtual-circuit switching and identify which model a
  given technology uses.
- Explain store-and-forward and compute its contribution to end-to-end delay.
- State precisely what guarantees packet switching abandons, and name the modern
  mechanisms that attempt to restore each.
