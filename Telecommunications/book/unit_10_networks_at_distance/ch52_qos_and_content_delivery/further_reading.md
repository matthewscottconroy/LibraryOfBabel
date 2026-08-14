# Chapter 52 — Further Reading

## Specifications

RFC 2474 — "Definition of the Differentiated Services Field."
The DSCP definition, and short. Read it with the code point table in §52.2 beside you.

RFC 2475 — "An Architecture for Differentiated Services."
**The model**: per-hop behaviours, boundary nodes, trust. The trust boundary of §52.2 is
specified here and is worth reading in the authors' own framing.

RFC 3246 — Expedited Forwarding PHB, and RFC 2597 — Assured Forwarding PHB Group.
Two short documents that define what EF and AF actually promise, which is less than most
people assume. **F4 uses both.**

RFC 4594 — "Configuration Guidelines for DiffServ Service Classes."
The most practically useful of the set. A worked, opinionated mapping of application types
to code points, with reasoning. If you are building a policy, start here rather than with a
vendor's template.

RFC 8290 — "The FlowQueue-CoDel Packet Scheduler and Active Queue Management Algorithm."
**FQ-CoDel, specified.** Readable, and the design rationale sections explain why it needs no
configuration.

RFC 8033 (PIE), RFC 8289 (CoDel), and the CAKE paper (Høiland-Jørgensen et al., 2018).
The modern AQM family. CAKE's paper is the one to read if you run an edge link, because
it is explicit about the shaping and overhead-accounting problems of §52.3.

RFC 7871 — "Client Subnet in DNS Queries."
**EDNS Client Subnet**, the partial fix for GeoDNS. Read the privacy considerations
section; it is unusually candid.

## Papers

Floyd, S. & Jacobson, V. (1993). "Random Early Detection Gateways for Congestion Avoidance."
*IEEE/ACM Transactions on Networking*.
**CoDel's ancestor**, and a case study in a correct idea defeated by configuration burden.

Nichols, K. & Jacobson, V. (2012). "Controlling Queue Delay." *ACM Queue*.
The CoDel paper, and it is genuinely well written. Read it for the argument that queue
length is the wrong thing to measure, which is the chapter's most transferable idea.

Gettys, J. & Nichols, K. (2011). "Bufferbloat: Dark Buffers in the Internet." *ACM Queue*.
**The naming paper.** Gettys's account of discovering it on his own home connection is the
best introduction to the phenomenon there is.

Appenzeller, G., Keslassy, I. & McKeown, N. (2004). "Sizing Router Buffers." SIGCOMM.
**F5 uses it.** The $\sqrt{n}$ result, and a good example of a statistical argument overturning
an accepted engineering rule.

Karger, D., Lehman, E., Leighton, T., Levine, M., Lewin, D. & Panigrahy, R. (1997).
"Consistent Hashing and Random Trees." STOC.
The algorithm behind Akamai, and behind most distributed caches since. Its influence far
exceeds content delivery, and Chapter 69's systems are full of it.

Calder, M. et al. (2013). "Mapping the Expansion of Google's Serving Infrastructure." IMC.
Measurement of how a large content network actually places its serving locations, and a
good companion to §48.1's flattening argument.

## Books

Szigeti, T., Hattingh, C., Barton, R. & Briley, K. — *End-to-End QoS Network Design*.
The standard practitioner's reference, and unusually thorough on the trust boundary and on
the mapping between DSCP, CoS and wireless WMM. Vendor-flavoured, and the design reasoning
transfers.

Ferguson, P. & Huston, G. — *Quality of Service*.
Older, and the clearest statement anywhere of what QoS can and cannot do. Huston's
scepticism is the correct posture for this topic.

**Grigorik, I. — *High Performance Browser Networking*.**
**Free online.** The chapters on latency, TCP and TLS handshakes quantify §52.4's argument
about round trips better than anything else, from the application's side.

## Tools and practical work

**`tc` on Linux** — `tc qdisc add dev eth0 root cake bandwidth 47500kbit`. One line, and F1's
whole experiment. Learn `tc qdisc`, `tc class` and `tc filter`; they are the reference
implementation of everything in §52.2 and §52.3.

**OpenWrt** — CAKE and FQ-CoDel are built in, with a configuration page. The single
easiest way to demonstrate the bufferbloat fix on real hardware.

The Waveform bufferbloat test, `flent`, and `netperf-wrapper` — measure latency under
load, which is the only measurement that reveals bufferbloat. An idle ping proves nothing,
and F1 depends on this.

`tcpdump -v` and Wireshark's `ip.dsfield.dscp` — look at what is actually marked on the
wire. F2 is this, and it reliably surprises people.

**`tc netem`** — introduce delay, loss and jitter deliberately. Essential for testing any QoS
or steering policy without waiting for a real problem.

**RIPE Atlas** (Chapter 48's further reading) — F6 measures anycast selection from ten
continents with it, free, in an afternoon.

**A CDN's free tier.** Cloudflare, Fastly and others offer them. Put a static site behind one
and measure the difference from three continents; the numbers make §52.4's argument better
than the table does.

## Post-mortems worth reading

**Cloudflare's public post-mortems** (blog.cloudflare.com) — among the best incident write-ups
published by anyone. The July 2019 regular-expression outage and the 2020 backbone incident
are the two to start with, and F7 uses one.

Fastly's June 2021 summary and Akamai's July 2021 statement.
Shorter and less forthcoming than Cloudflare's, which is itself informative about the range
of disclosure practice.

AWS, Google and Azure service health post-incident reports.
Read one that affected something you use. The value is in seeing how a change with a
bounded intent produced an unbounded effect.

## Where to look next

**Chapter 54** covers measuring the things this chapter tunes; **Chapter 66** treats bufferbloat
and queueing as a diagnostic problem rather than a design one, and is the direct continuation of
§52.3; and **Chapter 69** returns to anycast and load balancing as cloud architecture.
