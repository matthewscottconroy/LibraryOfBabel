# Chapter 14 — Exercises

## A. Recall

**A1.** List the five scope categories with their typical spans.

**A2.** Which of the classical LAN/WAN distinctions has not eroded, and why can it
not?

**A3.** Define *internetwork* and state how it differs from "a big network".

**A4.** Name the four requirements an internetwork imposes, and the chapter of this
book that addresses each.

**A5.** Give three network consequences of the client–server asymmetry.

## B. Apply

**B1.** An application performs 18 round trips per screen load. Compute the
latency-only screen-load time on: a LAN at 0.3 ms RTT; a national WAN at 22 ms; an
intercontinental WAN at 280 ms. State how much bandwidth would be required to
improve the third figure.

**B2.** For each of the following, state which parameter binds and what the design
question therefore is: (a) a warehouse WLAN with 40 handheld scanners; (b) a 400 km
metro fibre ring; (c) a battery sensor reporting once a minute for ten years; (d) a
branch office whose ERP screens are slow; (e) a data centre fabric carrying
microservice traffic.

**B3.** Explain why IP promises nothing, in terms of the lowest-common-denominator
argument. Then identify three constituent network technologies whose properties would
have to be excluded if IP guaranteed delivery.

**B4.** A branch office has a 200 Mb/s circuit to headquarters and a 100 Mb/s
Internet circuit. Measurement shows 80% of the branch's traffic is destined for
cloud services. Compute the fraction of the headquarters circuit that is carrying
traffic which does not need to go there, under a full-backhaul design. State the
remedy.

**B5.** In a converged network, list every service that becomes unavailable when a
single access switch loses power, assuming PoE telephones, cameras and access
points. Then state the mitigation for each and its cost.

**B6.** A microservices application generates 34 internal calls per external
request. External requests arrive at 500/second. Compute the internal call rate and
the ratio of east–west to north–south traffic, assuming similar message sizes.
Comment on the implications for a three-tier topology with 20:1 access-to-
distribution oversubscription.

## C. Analyse

**C1.** Argue that the LAN/WAN distinction should be taught as a set of
constraints rather than a taxonomy. Your answer should identify which rows of §14.1's
parameter table have converged, which has not, and what a student who learns the
taxonomy without the constraints will get wrong.

**C2.** The Internet's network layer is peer-to-peer and its application layer has
re-centralised. Trace the three stages of that re-centralisation, and for each state
what caused it and what network-design consequence followed. Then argue whether the
trend is likely to reverse, and what would have to change.

**C3.** Explain why the "waist" of the hourglass is the hardest part of the
architecture to change, and use that to account for IPv6's thirty-year transition.
Then identify what a *successful* waist replacement would require, and whether any
mechanism has ever achieved one.

**C4.** The telephone engineers who objected to carrying voice over packet
networks were correct about the requirements and wrong about the outcome. Identify
precisely what they were right about, what changed, and state the general pattern.
Then find one current technology dispute with the same structure and predict the
outcome using the pattern.

**C5.** Convergence exchanged physical separation for logical separation.
Enumerate what was gained and what was lost, and argue for or against the claim that
the industry has spent two decades recreating in software what it removed in
hardware. Reference at least four mechanisms from later chapters.

## D. Design

**D1.** A regional charity is consolidating from three offices into one new
building. Currently each office has:

- A TDM PBX with analog handsets
- A separate Ethernet data network
- Analog CCTV on coax to a recorder
- A proprietary building-management bus for heating and access control

The new building will be cabled from scratch. The trustees have been told that
"convergence saves money" and have asked you to confirm it.

Produce the analysis. Address: what a single cabling plant saves, quantified as
best you can; what new failure modes convergence introduces and what each mitigation
costs; what logical separation the design must specify and why; what the
organisation's two-person IT team will now be responsible for that they were not
before; and what regulatory or safety obligations transfer. Conclude with a
recommendation and the condition under which it would change.

## E. Troubleshoot

**E1.** An organisation converged voice onto its data network eighteen months ago.
It has worked well. Over the past three weeks, users at one site report that calls
"break up" for a few seconds, several times a day, with no pattern anyone can
identify.

Evidence:

- The site's WAN circuit peaks at 62% on five-minute averages.
- Voice is marked EF and has a priority queue configured on the WAN edge.
- The priority queue is not policed.
- `mtr` to the SIP provider shows 0.3% loss and 40 ms jitter during a reported
  incident, and clean results otherwise.
- The site recently deployed a video-conferencing system, which marks its traffic
  EF at the endpoint.
- The access switches trust DSCP markings from all ports.

Diagnose it. Identify the specific configuration decision that made this possible,
explain the mechanism by which the video system's traffic degrades the voice
traffic, and explain why the WAN utilisation figure conceals the problem. State the
three configuration changes required and which of them addresses the cause rather
than the symptom.
