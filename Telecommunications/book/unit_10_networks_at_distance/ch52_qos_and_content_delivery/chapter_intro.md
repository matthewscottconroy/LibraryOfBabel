# Chapter 52 — QoS and Content Delivery

A link is congested. Three things want to use it: a voice call, a file backup, and
someone loading a web page.

Left alone, a router treats them identically — first in, first out — and each gets a
share proportional to how much it sends. The backup, which sends continuously and as
fast as it can, gets most of the capacity. The voice call, which sends a small packet
every twenty milliseconds, waits behind the backup's packets in the queue and arrives
late and jittery. The web page is somewhere in between.

**Fairness has produced a bad outcome**, and this is not a malfunction. It is what
happens when a mechanism with no knowledge of purpose allocates a resource among
purposes that differ enormously in their sensitivity to delay.

The backup does not care whether it finishes at 04:00 or 04:20. The voice call is
ruined by 100 ms of extra delay (Chapter 3 §3.3, and the ITU's 150 ms budget). Equal
treatment gives the indifferent application what the sensitive one desperately needed.

**Quality of service** is the machinery for treating traffic unequally on purpose,
and this chapter is about it — with an honest account of its limits, because QoS is
oversold more consistently than any other topic in this book.

## The sentence to hold onto

> **QoS does not create bandwidth. It decides who suffers.**

If a link is persistently oversubscribed, QoS lets you choose which traffic degrades.
That is genuinely valuable — it is much better that the backup slows than that every
telephone call becomes unusable — but it is a prioritisation tool, not a capacity
tool. An engineer who deploys QoS to fix a link that is simply too small has bought
themselves a more sophisticated description of the same problem.

The corollary, which §52.1 states plainly: **QoS matters where congestion happens.**
Inside a data centre with a non-blocking fabric, it does almost nothing. On a 20 Mb/s
branch circuit carrying voice, it is essential. Deploy it where the bottleneck is,
and do not deploy it elsewhere, where it adds configuration complexity for no benefit.

## The three-step model

§52.2 covers the mechanism, which is always the same three steps regardless of vendor.

**Classify.** Decide which class a packet belongs to. By port, by address, by
protocol, by deep inspection, or — best — by trusting a marking already applied.

**Mark.** Write the classification into the packet so that downstream devices do not
have to repeat the work. The field is **DSCP**, six bits in the IP header's
former Type of Service byte (Chapter 24 §24.2), giving 64 code points of which a
standard subset is used: EF (Expedited Forwarding, DSCP 46) for voice; AF classes for
various data; CS classes for control traffic; and default (0) for everything else.
At Layer 2 the equivalent is 802.1Q's three-bit PCP field (Chapter 20 §20.2).

**Queue and schedule.** Put packets into different queues and serve those queues by a
policy. A **priority queue** is served before all others — appropriate for voice, and
dangerous, because a priority queue with no policer can starve everything else, so it
is always rate-limited. **Weighted fair queueing** and its variants give each class a
guaranteed share while allowing unused capacity to be borrowed.

The critical operational point, and it is where most QoS deployments fail: **marking
must be trusted, and trust must have a boundary.** If any host can mark its own
traffic EF, every host eventually will, and the priority queue becomes the default
queue. The standard design marks at the access edge — trusting an IP phone's marking,
re-marking or clearing everything from a workstation — and trusts markings thereafter.
That boundary is a policy decision that must be made explicitly and is routinely
forgotten.

## Policing and shaping

Two mechanisms that sound similar and behave very differently, and confusing them
produces real outages.

**Policing** enforces a rate by **dropping** (or re-marking) traffic above it.
Immediate, memoryless, cheap. Its interaction with TCP is unkind: dropping packets
from a stream that was accelerating triggers the congestion response of Chapter 38 and
produces a sawtooth that averages well below the policed rate.

**Shaping** enforces a rate by **buffering** traffic above it and releasing it
smoothly. Kinder to TCP, and it costs memory and adds delay.

The rule of thumb: **shape traffic you send, police traffic you receive.** Shaping
outbound to slightly below the carrier's contracted rate is standard practice, because
it moves the queue from the carrier's device — where you have no visibility and no
control over the discipline — into your own, where your QoS policy can decide what
gets dropped. That single technique is one of the highest-value configurations in
branch networking, and it is why the shaper is usually set to about 95% of the
circuit rate.

§52.3 also covers buffer sizing, which connects directly to Chapter 66's bufferbloat:
**a bigger buffer is not a better buffer.** A large buffer converts loss into delay,
and for interactive traffic delay is the thing you were trying to avoid.

## The other answer

§52.4 makes the argument that has done more for perceived Internet performance than
every QoS mechanism ever deployed: **do not send the data across the distance at
all.**

**Caching** keeps a copy near the user. **Content delivery networks** industrialise
this — thousands of points of presence worldwide, each holding copies of popular
content, so a video streams from a server in the same city rather than another
continent. **Anycast** (Chapter 27 §27.3) makes the selection automatic: one address,
many locations, and BGP delivers the user to the nearest.

The effect is straightforwardly larger than any QoS policy. QoS can reduce queueing
delay by a few milliseconds. A CDN can reduce propagation delay by 150 milliseconds,
by moving the content 8,000 km closer, and propagation delay is the component
Chapter 3 §3.2 identified as otherwise irreducible.

There is a structural consequence worth noting, and §52.4 does not shy from it: the
CDN model has concentrated a very large share of the world's traffic in a small number
of organisations. That is excellent for performance and it is a genuine resilience and
governance question, as several large CDN outages have demonstrated by taking down
apparently unrelated services worldwide.

## By the end you will be able to

- Explain why FIFO fairness produces bad outcomes for mixed traffic.
- State what QoS can and cannot do, and identify where in a network it belongs.
- Classify, mark and queue traffic for a stated mix, choosing appropriate DSCP values.
- Explain the trust boundary and design one.
- Distinguish policing from shaping and choose correctly for a stated direction.
- Explain why shaping below the carrier's rate is standard practice.
- Explain how a CDN and anycast reduce latency, and compare the effect with QoS
  quantitatively.
