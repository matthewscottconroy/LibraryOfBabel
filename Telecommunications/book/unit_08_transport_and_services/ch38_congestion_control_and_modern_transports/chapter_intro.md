# Chapter 38 — Congestion Control and Modern Transports

In October 1986, the link between Lawrence Berkeley Laboratory and the University of
California at Berkeley — a distance of about four hundred metres — dropped from
32,000 bits per second to **40 bits per second**.

A factor of eight hundred. Not from a hardware fault; the equipment was fine. The
network had entered **congestion collapse**, and Van Jacobson and Michael Karels,
who investigated it, found a failure mode that had been theoretically predicted by
John Nagle in RFC 896 two years earlier and comprehensively ignored.

The mechanism is a feedback loop that runs the wrong way. A router's queue fills and
it begins discarding packets. Senders, receiving no acknowledgement, retransmit. The
retransmissions arrive at the already-full queue and are also discarded, while
adding to the load. More senders time out, more retransmit, and the fraction of link
capacity carrying *useful* data — as opposed to copies of data already sent —
collapses toward zero. The network is fully utilised and delivering nothing.

The critical property: **it is stable.** It does not recover when load decreases,
because the retransmissions sustain it. Left alone, it persists.

The original TCP had no defence at all, and for a straightforward reason: it had
flow control, which prevents a sender from overwhelming a *receiver*, and its
designers had not distinguished that from overwhelming the *network*. The receiver's
buffer and the network's buffers are different resources with different owners, and
the protocol only knew about one of them.

## The fix, and why it was remarkable

Jacobson and Karels's response — presented at SIGCOMM in 1988 in a paper called
*Congestion Avoidance and Control* — added a second window. The sender now maintains
both the receiver's advertised window and its own **congestion window**, an estimate
of what the network can absorb, and sends no more than the smaller of the two.

Two questions follow: how do you estimate the network's capacity, and what do you do
when you are wrong?

The answers are **slow start** — begin small and double every round trip until
something breaks, which finds the capacity in logarithmic time — and **additive
increase, multiplicative decrease**: when a loss occurs, halve the window; when
things are going well, increase it by one segment per round trip.

AIMD produces the sawtooth pattern that anyone who has watched a TCP throughput
graph will recognise, and Chiu and Jain proved in 1989 that among the simple linear
control schemes, additive-increase/multiplicative-decrease is the one that converges
to a fair and efficient allocation. The choice was not arbitrary; it has a proof
behind it.

The deeper point, and the one worth carrying: **TCP treats packet loss as a signal
about network state.** The network never says "I am congested." It cannot; IP has no
such message and the end-to-end argument says it should not. So TCP *infers*
congestion from the only evidence available — a missing acknowledgement — and this
inference is the entire mechanism.

Which means the inference can be wrong. A packet lost to radio interference on a
Wi-Fi link is not congestion, and slowing down does not help; it is precisely the
layering cost Chapter 21 §21.3 predicted, where the layer that knows the answer has
no way to say it. Two decades of research went into this mismatch.

## The modern algorithms

§38.3 covers what actually runs today, because "TCP uses AIMD" has been an
approximation for twenty years.

**CUBIC**, the Linux default since 2006 and now the most widely deployed algorithm
in the world, replaces the linear increase with a cubic function of time since the
last loss. On high-bandwidth, high-latency paths — the long fat networks of
Chapter 3 §3.4 — linear increase takes minutes to recover a large window after a
single loss; CUBIC recovers in seconds. It is also *RTT-independent* by design, so
that a short connection does not systematically starve a long one.

**BBR**, from Google in 2016, abandons loss as the signal entirely. Instead it
measures the path's bottleneck bandwidth and its minimum round-trip time, and paces
sending to match. The argument is that loss-based control necessarily fills queues
before it detects anything — it must cause the problem to discover it — and
therefore guarantees bufferbloat (Chapter 66 §66.4). BBR aims to sit at the
bandwidth-delay product rather than above it, keeping queues nearly empty. It is
deployed at very large scale on Google and YouTube traffic. Its fairness when
competing with CUBIC has been the subject of vigorous and not entirely settled
argument, which §38.3 covers honestly.

**ECN** — Explicit Congestion Notification — is the mechanism that would let the
network simply say what is happening: a router marks a bit rather than dropping the
packet, and the receiver echoes the mark back. It has been standardised since 2001,
it works, and its deployment was obstructed for a decade by middleboxes that
discarded packets with the bits set. It is now broadly usable, and L4S is building
on it.

## The migration to userspace

§38.4 covers the structural change, which is arguably more significant than any
individual algorithm.

TCP lives in the kernel. Changing it means changing every operating system on the
Internet and waiting years for deployment — the window scaling of RFC 1323, needed
in 1992, was not universally on by default until well into the 2000s.

**QUIC** — standardised as RFC 9000 in 2021, after years of deployment by Google —
sidesteps this by running over UDP and implementing everything itself in userspace.
The consequences:

- **Deployable at software speed.** A browser update changes the transport.
- **No head-of-line blocking across streams.** HTTP/2 multiplexed many requests over
  one TCP connection, so one lost packet stalled *all* of them. QUIC's streams are
  independent.
- **Handshake and encryption combined.** TLS 1.3 is integrated; a new connection
  costs one round trip, and a resumed one can cost zero.
- **Connection survives address changes.** Connections are identified by a
  connection ID rather than the five-tuple, so a phone moving from Wi-Fi to cellular
  keeps its connections.
- **Almost everything is encrypted**, including most transport metadata. This was
  deliberate: it prevents middleboxes from ossifying the protocol the way they
  ossified TCP. It also blinds operators, and the argument about that tradeoff is
  live.

HTTP/3 is HTTP over QUIC, and a substantial fraction of web traffic now uses it.

## By the end you will be able to

- Explain congestion collapse mechanistically and say why it is self-sustaining.
- Distinguish flow control from congestion control, and name the window each uses.
- Trace slow start and congestion avoidance, computing window size per round trip.
- Explain why AIMD converges to fairness.
- Explain what BBR measures instead of loss, and why.
- State four concrete advantages of QUIC over TCP and one significant cost.
