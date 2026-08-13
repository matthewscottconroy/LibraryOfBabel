# 13.4 Why Packets Won

The argument, with the numbers, and an honest account of what it cost.

## The efficiency case, computed

Chapter 9 §9.3 did this arithmetic for multiplexing in general. Here it is applied
to the specific historical question, because this is the calculation that decided
the outcome.

**100 users, 1 Mb/s each while active, 5% duty cycle.**

**Circuit switching** reserves capacity per session. To serve all 100 simultaneously
requires **100 Mb/s**, utilised at 5%.

**Packet switching** provisions for the aggregate. Expected load is 5 Mb/s;
provision **20 Mb/s** and the binomial tail gives *P*(more than 20 simultaneously
active) ≈ 2 × 10⁻⁸.

**One fifth of the capacity. Essentially the same service.**

And the gain grows with scale, because the mean grows as *n* while the standard
deviation grows as √*n*:

| Users | Circuit capacity | Packet capacity (6σ) | Gain |
|---|---|---|---|
| 10 | 10 Mb/s | ~5 Mb/s | 2× |
| 100 | 100 Mb/s | ~18 Mb/s | 5.5× |
| 1,000 | 1,000 Mb/s | ~91 Mb/s | 11× |
| 10,000 | 10 Gb/s | ~631 Mb/s | 16× |

At the scale of a national network, the factor is well over ten. Applied to
infrastructure costing billions, that is not an optimisation; it is a different
business.

## The other three arguments

Efficiency was decisive and it was not alone.

**Setup cost is intolerable for short transactions.** A circuit requires setup and
teardown. For a two-hour telephone call, a few hundred milliseconds of setup is
irrelevant. For a DNS query — one small packet out, one small reply back — the setup
costs more than the transaction, by orders of magnitude. Computer traffic is full
of short transactions, and the circuit model has no answer to them.

**Heterogeneity.** A circuit requires every switch along the path to participate in
setup and hold state, which requires every network the path crosses to implement the
same signalling. A datagram requires only that each network can move it from one
attached device to another — Chapter 14 §14.2's lowest-common-denominator argument —
which is what let IP run over Ethernet, radio, satellite, DSL, LTE, and everything
since.

**Innovation without permission.** This is the one that mattered most in the long
run and was least visible at the time. In a circuit network with intelligence in the
core, a new service requires the network operator to implement it. In a datagram
network with intelligence at the edges, a new application requires nobody's
agreement — you write it and deploy it. The web, email, video streaming and every
application in Chapter 41 were built by people who did not ask a carrier's
permission, and could not have been built otherwise.

Chapter 23 §23.4's end-to-end argument is the principle; this is its consequence.

## What was given up

This is the honest half, and it is the half that Unit XIV keeps returning to.

**Guaranteed bandwidth.** A circuit's capacity is yours. A packet network's is
shared, and what you receive depends on what everyone else is doing.

**Constant delay.** A TDM slot arrives every 125 µs, always. A packet arrives when
the queue lets it, and Chapter 3 §3.3's jitter is the consequence — which real-time
applications must then absorb into a buffer, converting jitter back into latency.

**In-order delivery.** Datagrams may take different paths and arrive reordered. TCP
fixes this at the endpoints, at the cost of buffering and head-of-line blocking.

**Admission control.** This is the loss the industry most regrets, and it is worth
dwelling on.

A telephone exchange with full trunks returns a **busy signal**. That is an honest
refusal, delivered *before* the caller invests any effort, and — crucially — the
calls already in progress are unaffected. The system degrades by **refusing new
work**, not by degrading existing work.

A packet network accepts everything. When capacity is exceeded, **everyone**
degrades — including the flows that were already running acceptably. There is no
mechanism to say "we are full, come back later", because there is no notion of
admitting a flow in the first place.

Consider a video conference that becomes unusable when four more people join a
saturated link. The circuit-switched equivalent would have refused the fourth
connection and left the first three perfect. Which behaviour is preferable is a real
question with a real answer that depends on the application, and the packet network
does not offer the choice.

**Immunity to congestion collapse.** A circuit network cannot collapse under load;
it blocks. A packet network can, and did, in October 1986 (Chapter 38 §38.1).
Preventing it required inventing congestion control, which took two more years and
is still an active research area forty years later.

**Per-unit overhead.** A TDM slot needs no identifier — slot *i* is conversation *i*
by construction. A packet must carry a header saying where it is going, and
Chapter 3 §3.1 computed the cost: 5% for a large frame, 33% for a small voice packet.

## Buying it back

The subsequent fifty years have been spent selectively reintroducing, for some
traffic, properties that circuit switching provided for all traffic. The list is
long and worth assembling in one place:

| Mechanism | What it restores | Chapter |
|---|---|---|
| X.25, Frame Relay, ATM | Virtual circuits, ordering | 13 §13.2 |
| IntServ / RSVP | Per-flow reservation and admission control | 52 |
| DiffServ | Class-based prioritisation | 52 §52.2 |
| MPLS traffic engineering | Placed paths with reserved bandwidth | 50 §50.4 |
| Data centre lossless Ethernet | No congestion loss | 71 §71.5 |
| Time-Sensitive Networking | Bounded latency, scheduled traffic | 71 §71.4 |
| 5G network slicing | Isolated virtual networks with guarantees | 46 §46.4 |

Every one is an attempt to recover something that was deliberately given up in 1964,
and every one is harder than it looks — because the architecture was designed
around not having it.

The pattern is worth naming: **a general-purpose substrate absorbs a specialised
one because its economics improve faster, and then spends decades reimplementing the
specialist's guarantees imperfectly.** Chapter 14 §14.4's convergence and
Chapter 67's virtualisation are the same story at different layers.

## The honest verdict

Packet switching won because computer traffic is bursty and the efficiency gain was
between five and twenty times, on infrastructure costing enormous sums.

It won *despite* offering no guarantees, and the guarantees it abandoned were
genuinely valuable — which is why the industry has spent five decades and a great
deal of engineering effort partially recovering them.

The telephone engineers who resisted it were not stupid and were not merely
defending an incumbency. They were right that voice needed properties a best-effort
network did not provide, and wrong about the trajectory: the general substrate
improved faster than the specialised one, until it was good enough that the
guarantees stopped mattering for most traffic.

That is the shape of the argument, and recognising it is worth more than the
history — because you will meet it again, in this book and in your career, whenever
a general-purpose thing is displacing a specialised one and the specialists are
listing what will be lost. They will be right about the list and probably wrong
about the outcome.

## What breaks here

**Expecting circuit-switched behaviour from a packet network.** "Why did my call
degrade when someone else started a backup?" has an answer, and the answer is that
you are on a shared network that admitted both.

**Deploying QoS to fix an undersized link.** QoS decides who suffers; it does not
create capacity (Chapter 52 §52.1). If the link is persistently oversubscribed, you
have bought a more sophisticated description of the same problem.

**Assuming the guarantees are recoverable cheaply.** Every mechanism in the table
above requires state, configuration and operational discipline that the base
architecture does not need. MPLS traffic engineering works and it is not free.

**Forgetting the price was paid deliberately.** An organisation that saved 80% on
capacity and then objects to jitter has been paid for the tradeoff and is now
objecting to the other half of it.

> **Network+ note.** Objective 1.6 expects the circuit/packet distinction.
> The exam will not ask what was given up; every performance and QoS question in
> objectives 2.1, 5.4 and 5.5 is downstream of it, and knowing that packet networks
> *chose* to abandon admission control makes the QoS material follow rather than
> needing to be memorised.
