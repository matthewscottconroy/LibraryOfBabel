# Chapter 52 — Important Concepts

A router with no policy still has a policy *(§52.1)* — FIFO under congestion gives each flow
a share proportional to how much it offers. The backup sends continuously and takes most of
the link; the voice call sends 50 small packets a second and waits behind the backup's
1,500-byte packets. At 10 Mb/s each queued packet is 1.2 ms, so 60 of them is 72 ms
added to a 150 ms budget.

Bandwidth need and delay sensitivity are inversely related *(§52.1)* — The applications
that need the least bandwidth need the most careful treatment. Giving voice absolute priority
costs the backup under 1% of a 10 Mb/s link. QoS works because the traffic that needs
protection is small — if real-time traffic were 80% of the load, no scheduler would help.

QoS does not create bandwidth. It decides who suffers *(§52.1)* — Genuinely valuable: far
better that the backup slows than that every call becomes unusable. Not a capacity tool.
An engineer who deploys QoS on a link that is simply too small has bought a more sophisticated
description of the same problem.

QoS matters where congestion happens, and nowhere else *(§52.1)* — Congestion forms where
the rate steps down. The WAN edge, an oversubscribed uplink, wireless. On a 10 Gb/s link
carrying 1 Gb/s it does exactly nothing, because a queue that never holds more than one packet
cannot be scheduled. The commonest deployment mistake is configuring it everywhere.

You cannot control inbound congestion from the receiving end *(§52.1)* — The bottleneck for
traffic coming to you is in the carrier's equipment, where your policy does not run.
Shaping (§52.3) is a partial answer; the full answer is that only senders can slow down.

IntServ reserves per flow and does not scale; DiffServ marks per class and does *(§52.1)* —
A core router carrying a million flows cannot hold a million reservations. Chapter 23
§23.4's argument arriving as an engineering failure. The same trade as segment routing versus
RSVP-TE, and as stateless versus stateful firewalls — the Internet has chosen aggregate
treatment every time.

Classify, mark, queue — every vendor, every time *(§52.2)* — Only the syntax changes.

Trusting an existing marking is the right classification method *(§52.2)* — Classify once
at the edge; every subsequent device reads the answer. Deep packet inspection has largely
stopped working — TLS 1.3, encrypted SNI and QUIC's hidden transport header (Chapter 38
§38.4) — so classification is moving back to endpoint marking, which is where it started.

DSCP is six bits of the former ToS byte *(§52.2)* — EF = 46 for voice; AF41 = 34 for
interactive video; CS6 = 48 for routing protocols; CS1 = 8 for scavenger; 0 for everything
else. AFxy: x is the class, y is the drop precedence — same class, increasing willingness
to be dropped. At Layer 2 the equivalent is 802.1Q's 3-bit PCP, which an untagged frame
cannot carry.

Scavenger is the least-known and most useful marking *(§52.2)* — CS1 should be dropped
before best effort. Backups and software distribution should use only what nobody else wants.

If any host can mark EF, every host eventually will *(§52.2)* — And the priority queue
becomes the default queue. The trust boundary is a policy decision that must be made
explicitly and is routinely forgotten: trust the phone, distrust the workstation, re-mark at
the access edge, trust everything inside.

Your markings do not survive into someone else's network *(§52.2)* — Carriers re-map to a
small number of classes at ingress, contractually, in a service description nobody reads.
And the Internet does not honour DSCP at all. QoS is an intra-domain mechanism.

A priority queue without a policer starves everything else *(§52.2)* — **Always rate-limit
it.** This is not optional, and omitting it is the classic QoS misconfiguration.

"Guarantee" in CBWFQ means a minimum, not a maximum *(§52.2)* — A class not using its
share does not waste it. Statistical multiplexing with a floor under each tenant.

FQ-CoDel and CAKE target delay and need no classification *(§52.2, §52.3)* — On a small
edge link they frequently outperform a carefully built DSCP policy, and they cannot be
defeated by a host marking its own traffic — which sidesteps the trust-boundary problem
entirely. Under-taught, and the most effective configuration change available to most
people.

A policer and a shaper differ only in what happens to a non-conforming packet *(§52.3)* —
Same token bucket, same CIR, same Bc. Policer drops or re-marks; shaper queues.

Shape what you send; police what you receive *(§52.3)* — A policer drops from a stream
precisely when it is accelerating, in bursts, producing timeouts rather than fast recoveries;
a 10 Mb/s policer typically yields 60–80% of 10 Mb/s to a single TCP flow. Hence the
perennial "we bought 10 and get 7", where the circuit is fine and the policer is working
exactly as configured.

Shaping below the carrier's rate moves the queue into your own device *(§52.3)* — The
single highest-value configuration in branch networking. Without it your QoS policy sits on a
device that never queues anything, and the real queue is in the carrier's equipment where you
have no visibility and no control over what it drops. 95% covers the Layer 2 overhead your
shaper is not counting; with proper overhead accounting, 98%.

Tc is as important as CIR when voice is present *(§52.3)* — A 125 ms default interval
releases a burst and then goes silent, adding up to 125 ms of jitter. Set Tc to 10 ms on any
circuit carrying real-time traffic.

A bigger buffer is not a better buffer *(§52.3)* — It converts loss into delay, and delay
is what you were avoiding. 256 KB on a 1 Mb/s uplink is 2,048 ms; 1 MB at 10 Mb/s is 800 ms.
And TCP will fill it, because filling the bottleneck buffer is what loss-based congestion
control does.

Both classical buffer rules optimise for throughput, not delay *(§52.3)* — $B = \mathrm{RTT}
\times C$, refined by Appenzeller, Keslassy and McKeown to $\mathrm{RTT} \times C / \sqrt{n}$ —
a tenfold reduction at 100 flows, and a genuinely surprising result that changed core router
design. Neither is what you want at an edge link, where the modern answer is not to pick a
size at all.

CoDel measures time in queue, not bytes in queue *(§52.3)* — Above 5 ms of minimum
queueing delay for longer than 100 ms, it begins dropping. FQ-CoDel adds per-flow fairness,
so a bulk transfer cannot delay a voice packet regardless of markings.

QoS acts on queueing delay; a CDN acts on propagation delay *(§52.4)* — A perfect QoS
policy saves 20–50 ms. Moving content 8,000 km closer saves 78 ms of round trip, and
London–Sydney's 166 ms becomes under 1 ms. And the saving multiplies by every round trip
the protocol needs — a TCP plus TLS handshake is three or four before any data flows.

A CDN is not only about caching *(§52.4)* — Terminating TLS and the transport 5 ms away
instead of 80 benefits completely uncacheable content, and the CDN's own backbone frequently
beats the public Internet to origin, so even a cache miss is faster. The shield tier exists
so a thousand edge misses do not become a thousand origin requests.

`Vary` is where caching breaks *(§52.4)* — `Vary: User-Agent` on a resource that does not
vary by user agent fragments the cache into hundreds of copies, and a stray `Set-Cookie` on
a static asset makes it uncacheable. Both are usually accidental.

Forward proxies died because of HTTPS *(§52.4)* — A proxy cannot cache what it cannot see.
The caching relocated from the organisation to the CDN, which is a small part of the
concentration story.

Anycast selects by routing, not by geography *(§52.4)* — One address announced from many
locations; BGP prefers by AS-path and local policy, so a user in Lisbon may be sent to London
rather than Madrid. Very good, not optimal. Failover is a BGP withdrawal — seconds — and a
DDoS is spread across every announcing location. 13 root server addresses, over 1,900
physical instances.

Anycast suits short, stateless exchanges *(§52.4)* — A long TCP connection can break if
routing shifts mid-session. Rare enough in practice that anycast TCP works at scale, and it
is why DNS over UDP was its first natural application.

The routing layer's redundancy is intact; the experience layer's is not *(§52.4)* —
Fastly June 2021, Akamai July 2021, Cloudflare repeatedly — none an attack, each a
configuration change, each taking out thousands of unrelated sites in under a minute.
And the alternative is worse: a world without CDNs has worse performance, availability and
DDoS resilience. Multi-CDN is the only real answer, and it is not free.
