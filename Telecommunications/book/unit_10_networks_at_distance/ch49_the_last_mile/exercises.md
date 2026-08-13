# Chapter 49 — Exercises

## A. Recall

**A1.** State in one sentence why the last mile dominates access network cost.

**A2.** What limited a dial-up modem to 33.6 kb/s, and what limited V.90 to 56 kb/s? They are
different limits — say what each is.

**A3.** Explain what a DSL splitter does and why its cost mattered historically.

**A4.** State the relationship between DSL rate, frequency and loop length, and give the
approximate ADSL2+ rate at 1, 3 and 5 km.

**A5.** Give the architectural difference between DSL and DOCSIS in one sentence, and the
different user complaint each produces.

**A6.** What is the noise funnel, in which direction does it operate, and why has it no DSL
equivalent?

**A7.** State the DOCSIS upstream spectrum range in a legacy sub-split plant, and say why the
upstream is narrower than the downstream.

**A8.** What is a passive optical splitter, what does it require to operate, and what is its
loss for a 1:32 split?

**A9.** Why must PON downstream traffic be encrypted?

**A10.** What is ranging, and name three technologies in this book that require it.

**A11.** Give the altitude and round-trip propagation delay for GEO, MEO and LEO.

**A12.** What is a performance-enhancing proxy, what does it do, and what defeats it?

## B. Apply

**B1.** A voiceband channel has 3.4 kHz of bandwidth and a 30 dB signal-to-noise ratio.

(a) Compute the Shannon capacity.
(b) Compare with V.34's 33,600 b/s. Comment.
(c) What SNR would be required for 56 kb/s through the same analogue channel? Is it
achievable on a telephone line?

**B2.** Explain V.90's downstream mechanism as an arithmetic argument: state the PCM sample
rate, the bits per sample, how many are usable, and the resulting rate. Then state why the
upstream direction cannot use the same trick.

**B3.** A DSL line syncs at 6 Mb/s. The engineer measures 42 dB of attenuation.

(a) Estimate the loop length, stating your assumption about attenuation per kilometre.
(b) Is 6 Mb/s consistent with that length on ADSL2+?
(c) The customer is offered VDSL2. Predict the outcome and justify it.

**B4.** A 6 MHz cable channel carries QAM-256 at 5.36 Msym/s.

(a) Compute the raw bit rate.
(b) Compute the raw capacity of 32 bonded channels.
(c) A European plant uses 8 MHz channels at 6.95 Msym/s. Recompute (a).

**B5.** A DOCSIS node serves 320 homes, each sold a 200 Mb/s package, with 2 Gb/s of bonded
downstream capacity.

(a) What is the oversubscription ratio at the headline rate?
(b) At peak, 6% of homes transfer at an average of 30 Mb/s. Is the node saturated?
(c) At what concurrency does it saturate?
(d) The operator splits the node in two. Recompute (c).

**B6.** Compute the total optical loss for a GPON link with a 1:64 split, 15 km of fibre at
0.25 dB/km, and 2 dB of connector and splice loss. Assume 2 dB of splitter excess loss. Does
it fit within a 28 dB Class B+ budget? What about a 1:128 split?

**B7.** A GPON tree of 1:32 carries 2.488 Gb/s downstream.

(a) What does each subscriber get if all transmit continuously?
(b) The operator sells a 500 Mb/s package. What concurrency does that assume?
(c) Repeat for XGS-PON at the same split.

**B8.** Compute the one-way and four-hop round-trip propagation delay for satellites at 550 km,
1,200 km, 8,000 km and 35,786 km. Show the working for one of them.

**B9.** A GEO link has 100 Mb/s of capacity and a 477 ms round trip.

(a) What throughput does a single TCP stream with a 64 KB window achieve?
(b) What window is needed to fill the link?
(c) TCP slow start doubles each RTT from an initial window of 10 segments of 1,460 bytes. How
many round trips, and how many seconds, to reach the window in (b)?

**B10.** ONTs on a PON are between 300 m and 19 km from the OLT. Compute the spread in burst
arrival times at the OLT, taking light in fibre at 4.9 µs/km, and explain what ranging does
about it.

## C. Analyse

**C1.** §49.1 argues that V.90 "changed channels" rather than beating Shannon. Explain the
argument precisely, identify the two distinct channels involved, and say what general
engineering lesson it illustrates. Find one other example of the same move elsewhere in this
book.

**C2.** DSL, DOCSIS 3.1, Wi-Fi and LTE all divide their band into many narrow subcarriers and
load bits according to measured quality. Explain why this is the right answer, what property
of the channel makes it so, and why single-carrier modulation is not.

**C3.** Compare DSL and DOCSIS on: who your performance depends on, how a fault is isolated,
what the upgrade path costs, and what the failure modes are. Argue which architecture is
better and state clearly what "better" means in your argument.

**C4.** Analyse fibre-to-the-cabinet. It is frequently dismissed as a delaying tactic. Argue
the case for it on engineering and economic grounds, then state honestly what its limits are
and when it stops being the right answer.

**C5.** PON, DOCSIS and Wi-Fi all broadcast downstream and rely on encryption rather than
physics for privacy. Assess the security consequence of this in each case, and say which
worries you most and why.

**C6.** Performance-enhancing proxies violate the end-to-end principle deliberately and are
defeated by QUIC and VPNs. Analyse this as a conflict between two correct design principles.
Which should win, and does your answer change for a satellite link versus a terrestrial one?

**C7.** LEO constellations perform best where users are sparse. Explain the capacity argument
behind this, and analyse what it implies about claims that satellite will replace terrestrial
broadband.

**C8.** Fixed wireless costs scale per tower and per subscriber; wired access costs scale per
metre of route. Derive the crossover condition, and identify what other factors would move it
in practice.

## D. Design

**D1.** A rural district of 4,000 premises spread over 300 km² has no broadband beyond 2 Mb/s
ADSL. There is a fibre point of presence at the district centre and a hill with power and
sightlines to about 60% of premises. Design the access network. Specify the technology for
each group of premises, justify each choice, and state what you would build first and why.

**D2.** A cable operator's node serving 400 homes is congested every evening. Compare four
options — node split, DOCSIS 3.1 upgrade, high-split for upstream, and overbuild with PON — on
cost, disruption, timescale and how long each buys. Recommend a sequence.

**D3.** Design the PON architecture for a new 6,000-home development: split ratio, splitter
placement (centralised or distributed), reach, standard, and the headroom you leave for
upgrade. Justify the split ratio with an optical budget calculation.

**D4.** An oil platform 200 km offshore needs connectivity for 80 staff, including video calls
and a control system with a 100 ms latency requirement. Evaluate GEO, LEO, MEO and microwave
relay. Recommend a primary and a backup, and state what the control system's requirement rules
out.

**D5.** A national regulator asks you to advise on subsidising rural broadband. Write a
one-page technical recommendation covering which technologies should be eligible, what
performance floor should be required, and how to avoid subsidising infrastructure that will be
obsolete in ten years. State the trade-offs explicitly.

## E. Troubleshoot

**E1.** A DSL line resyncs several times a day, always in the evening, and syncs at a good rate
in between. Give the three most likely causes and how to distinguish them.

**E2.** Ten homes on one street report intermittent cable dropouts; the rest of the node is
fine. Give the likely cause, explain the mechanism, and describe how it is found.

**E3.** A cable modem shows an upstream transmit power of 55 dBmV and frequent T3 timeouts.
Interpret both symptoms and state what they indicate about the plant.

**E4.** A customer's ONT reports −29 dBm received optical power and drops out during hot
weather. Give the likely cause and the diagnostic sequence.

**E5.** A whole PON tree goes dark simultaneously. What is the most likely cause, what is the
least likely, and what would you check first?

**E6.** A satellite customer reports that web browsing is acceptable but the corporate VPN is
unusable. Explain what is happening and what can and cannot be done about it.

**E7.** A fixed wireless link commissioned in winter at 40 dB of margin fails intermittently
in June. Give the cause and the two possible remedies.

**E8.** A DSL customer's sync rate is 18 Mb/s and their measured throughput is 6 Mb/s.
Everything on the line looks healthy. Where do you look, and what have you already ruled out?

## F. Extend

**F1.** Find your own access line's statistics — DSL sync rate, attenuation and SNR margin;
cable downstream/upstream power and SNR; or ONT optical power. Record them over a week and
plot anything that varies. Explain what you observe.

**F2.** Estimate your distance from your exchange or cabinet using the attenuation figure and
the relationship in §49.1, then check it against a map. Explain any discrepancy.

**F3.** Use a public latency measurement service or `ping` to compare round-trip times to a
server via your fixed connection, via a mobile connection, and — if you can borrow access —
via a satellite connection. Decompose each figure into propagation, access and queueing as far
as you can.

**F4.** Research the current status of DOCSIS 4.0 deployment in your country. Explain what the
plant changes require, why the timescale is long, and what the operators say about it.

**F5.** Read the ITU G.984 (GPON) or G.9807 (XGS-PON) specification's clauses on ranging and
dynamic bandwidth allocation. Write a page explaining the mechanism in your own words, with a
timing diagram.

**F6.** Track a LEO constellation with a satellite-tracking application or a public API for one
hour. Record how many satellites pass above 40° elevation from your location, and use the
result to estimate the constellation size required for continuous coverage. Compare with the
actual figure and explain the difference.
