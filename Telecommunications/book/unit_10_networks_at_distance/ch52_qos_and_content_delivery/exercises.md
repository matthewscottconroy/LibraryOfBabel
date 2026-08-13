# Chapter 52 — Exercises

## A. Recall

**A1.** Explain why FIFO queueing gives a bulk transfer most of a congested link's capacity and
a voice call the worst service.

**A2.** State the sentence that governs this chapter, and explain what it rules out.

**A3.** Where in a network does QoS do something, and where does it do nothing? State the rule.

**A4.** Name the three steps of every QoS implementation.

**A5.** How many bits is DSCP, where does it live, and what was that byte previously called?

**A6.** Give the DSCP value and name for voice, and say what CS6 is reserved for and why.

**A7.** What is a trust boundary, and what happens if there is not one?

**A8.** State the difference between policing and shaping in one sentence each, and give the
rule of thumb for which to use in which direction.

**A9.** Why is outbound shaping to 95% of the carrier's rate standard practice?

**A10.** State the classical buffer sizing rule and the $\sqrt{n}$ refinement, and say what
both optimise for.

**A11.** What does CoDel measure, and how does that differ from what a tail-drop queue
measures?

**A12.** Explain anycast in two sentences, and name its classic deployment.

**A13.** Which component of delay does QoS act on, and which does a CDN act on?

## B. Apply

**B1.** A 10 Mb/s link carries a backup and a voice call.

(a) Compute the serialisation delay of one 1,500-byte packet.
(b) Compute the delay added to a voice packet arriving behind 20, 60 and 150 queued packets.
(c) The ITU one-way budget is 150 ms and 30 ms of it is already spent elsewhere. At what queue
depth does the call become unacceptable?
(d) Recompute (a) and (c) for a 100 Mb/s link and comment on why QoS matters less there.

**B2.** A G.711 call sends 160 bytes of payload every 20 ms.

(a) Compute the packets per second.
(b) Add 40 bytes of RTP/UDP/IP and 18 bytes of Ethernet framing. Compute the on-the-wire rate.
(c) How many concurrent calls fit in a 5 Mb/s priority queue?
(d) That queue sits on a 50 Mb/s circuit. What percentage is it, and why is the policer
essential?

**B3.** Convert between markings:

(a) EF, AF41, AF21, CS6 and CS1 to their decimal DSCP values
(b) EF's DSCP to the full ToS byte value with ECN bits zero
(c) Explain what AF11, AF12 and AF13 have in common and how they differ

**B4.** A shaper is configured with CIR 20 Mb/s.

(a) Compute Bc for Tc = 125 ms and for Tc = 10 ms.
(b) A voice packet arrives just after the bucket empties. What is the worst-case additional
delay under each setting?
(c) State which setting you would use on a circuit carrying voice, and why the other exists.

**B5.** A branch has a 100 Mb/s circuit which the carrier polices at exactly 100 Mb/s.

(a) Explain why a single TCP flow will measure well below 100 Mb/s.
(b) You configure an outbound shaper. At what rate, and why not 100?
(c) The platform cannot account for Layer 2 overhead. Recompute your answer.
(d) What symptom would tell you the shaper is set too high?

**B6.** Compute the drain time of the following buffers, and comment on each:

(a) 256 KB at 1 Mb/s
(b) 1 MB at 10 Mb/s
(c) 1 MB at 1 Gb/s
(d) 64 KB at 10 Mb/s

**B7.** Size a buffer for a 100 Mb/s link with a 60 ms RTT.

(a) Using the classical rule.
(b) Using the $\sqrt{n}$ rule with 400 concurrent flows.
(c) State why neither is the right answer for a branch edge link, and what is.

**B8.** A user in London fetches a resource from a server in Sydney over a 17,000 km cable
route.

(a) Compute the round-trip propagation delay.
(b) The protocol requires a TCP handshake, a TLS 1.3 handshake and one request/response.
Estimate the total time before the first byte arrives.
(c) A CDN edge in London serves the same content. Recompute.
(d) Express the improvement as a multiple, and compare it with the 20–50 ms a QoS policy might
save.

## C. Analyse

**C1.** §52.1 observes that bandwidth requirements and delay sensitivity are inversely related.
Explain why this makes prioritisation cheap, and state the condition under which QoS would stop
working.

**C2.** IntServ gives genuine guarantees and is effectively dead; DiffServ gives weaker
guarantees and is universal. Analyse the trade, and identify two other places in this book
where the Internet made the same choice.

**C3.** Deep packet inspection was the standard classification method in 2010 and is now
largely ineffective. Explain what changed, what classification methods replaced it, and what
this implies about the future of application-aware networking.

**C4.** Analyse the trust boundary as a security problem rather than a QoS one. What is the
threat model? Who is the adversary? Is the standard design (trust phones, distrust
workstations) actually sound?

**C5.** Explain precisely why a policer causes a TCP flow to average below its configured rate,
referring to Chapter 38's congestion control. Then explain why a shaper does not.

**C6.** FQ-CoDel and CAKE require no classification and frequently outperform hand-built DSCP
policies on edge links. Analyse why, and say what a class-based policy still offers that they
do not.

**C7.** The chapter argues that a CDN's effect is larger than any QoS policy's. Analyse the
claim: is it a fair comparison? Under what circumstances would QoS matter more?

**C8.** Assess the CDN concentration problem honestly. State the risk, state the counterargument,
and give your own position with reasons. What would you actually do as an architect?

**C9.** Anycast selects "approximately the nearest" instance. Analyse the sources of error, the
circumstances under which it selects badly, and what operators do about it.

## D. Design

**D1.** Design a complete QoS policy for a 50 Mb/s branch circuit serving 80 staff with IP
telephony, video conferencing, a business application on a known server, general web access
and overnight backups. Specify: classification method, markings, queue structure, guarantees,
the priority policer, the shaper rate and Tc, and the trust boundary. Justify every number.

**D2.** An organisation's QoS deployment has marked everything as high priority and achieves
nothing. Design the remediation: what you would measure first, what you would change, in what
order, and how you would prevent recurrence.

**D3.** Design the content delivery architecture for a video service with 2 million users across
Europe and South America. Address: CDN selection, whether to use multiple providers, cache
control policy, origin protection, and what happens during a CDN outage. State your
availability target and show that the design meets it.

**D4.** A home worker has a 500/40 Mb/s connection shared with a household, and video calls
degrade when anyone else uploads. Design the fix using only equipment they could reasonably own.
Explain what you are doing and why, in terms a non-engineer would accept.

**D5.** Design the QoS strategy for an organisation with 60 branches migrating to SD-WAN
(Chapter 51 §51.2). Address the relationship between SD-WAN path steering and DSCP-based
queueing: which does what, where each applies, and how they interact when both are configured.

## E. Troubleshoot

**E1.** Voice quality is poor. The WAN circuit's five-minute average utilisation is 55%. Explain
how both can be true and state what you would measure.

**E2.** A QoS policy is configured correctly and nothing improved. Give four causes in order of
likelihood and the check for each.

**E3.** A customer measures 7 Mb/s on a 10 Mb/s circuit. Single-stream and multi-stream tests
differ substantially. Diagnose.

**E4.** After a QoS change, BGP sessions across the WAN begin flapping under load. Explain and
give the fix.

**E5.** Latency to a nearby server is 4 ms when idle and 900 ms during an upload. Name the
condition, explain the mechanism, and give two fixes.

**E6.** A class-map shows zero packets matched after a week in production. State what this means
and three possible causes.

**E7.** Voice works on the wired network and is poor over Wi-Fi despite an identical DSCP
policy. Explain.

**E8.** A website's CDN hit rate is 12%. The content is almost entirely static images. Give the
three headers you would examine and what you would expect to find.

**E9.** Users in one country reach a CDN PoP on another continent. Explain the mechanism and
say what can and cannot be done about it.

**E10.** A CDN provider has an outage and a company's site is unreachable even though its origin
servers are healthy. Explain what should have been in place, and why most organisations
discover the gap during the incident.

## F. Extend

**F1.** Measure bufferbloat on your own connection. Run a latency test while saturating the
uplink (`ping` during a large upload, or a tool such as the Waveform bufferbloat test). Record
idle and loaded latency, then apply CAKE or FQ-CoDel on a Linux router or OpenWrt device and
repeat. Report the before-and-after figures.

**F2.** Capture traffic on a network with QoS configured and tabulate the actual DSCP
distribution (`tcpdump -v`, or Wireshark's IP DSCP field). Compare it with what the policy
intends. Report any traffic marking itself.

**F3.** Build a two-router lab with a rate-limited link. Configure a policer, measure single-flow
TCP throughput, then replace it with a shaper and measure again. Explain the difference with
reference to Chapter 38.

**F4.** Read RFC 2474 (DiffServ field), RFC 3246 (EF) and RFC 2597 (AF). Write a page on why the
AF drop-precedence structure exists and where you would use it.

**F5.** Read the Appenzeller, Keslassy and McKeown paper "Sizing Router Buffers" (2004).
Summarise the argument in one page, and explain why it changed core router design and why it
does not apply at an edge link.

**F6.** Use RIPE Atlas or a similar measurement platform to measure latency to a single anycast
address from ten locations on different continents. Identify which PoP each reached, and find at
least one case where the selection is not geographically nearest. Explain it.

**F7.** Investigate one large CDN outage in detail from the provider's own post-mortem.
Summarise the cause, the blast radius, the detection and recovery time, and what the provider
changed. Assess whether the change would prevent recurrence.
