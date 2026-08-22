# Chapter 50 — Exercises

## A. Recall

**A1.** Derive the DS0 rate from the voice bandwidth, and state the two theorems involved.

**A2.** Give the T1 and E1 rates and show the arithmetic for each, including framing and
signalling.

**A3.** Why does a plesiochronous hierarchy require bit stuffing, and what does that cost when
you want one DS0 out of a DS3?

**A4.** State the STS-1 frame dimensions and derive 51.84 Mb/s from them.

**A5.** What was SONET's protection switching target, why that number, and how is it achieved?

**A6.** Name the three SONET overhead layers and say what equipment each spans.

**A7.** What does OTN add that SONET/SDH lacks, and why is it worth so much?

**A8.** State the C-band's wavelength range, and give the two independent reasons 1550 nm is
used.

**A9.** What does an EDFA do that a regenerator does not, and why does that matter for DWDM
specifically?

**A10.** Name the three MPLS label operations and where each occurs.

**A11.** State the original justification for MPLS and say why it stopped applying.

**A12.** In an MPLS L3VPN, what does each of RD, RT and VRF do?

**A13.** Roughly how many submarine cable systems are in service, and what share of
intercontinental traffic do they carry?

**A14.** What are the two leading causes of submarine cable faults, and what does that imply
about where faults occur?

## B. Apply

**B1.** Compute the following from first principles, showing the arithmetic:

(a) the T1 rate from 24 channels plus framing
(b) the E1 rate from 32 timeslots
(c) the DS3 rate from 28 DS1s (state why it exceeds 28 × 1.544)
(d) the STS-1 rate from the frame dimensions
(e) the STS-1 payload rate, given one column of path overhead

**B2.** A carrier must drop one DS0 at each of eight towns along a DS3 route.

(a) Under a plesiochronous hierarchy, how many complete demux/remux cycles are required?
(b) Under SONET, what equipment is required at each town?
(c) Explain in one sentence what property of SONET makes the difference.

**B3.** A SONET UPSR ring is built at OC-48.

(a) What is the ring's total capacity?
(b) How much traffic can it actually carry?
(c) A competitor offers an unprotected OC-48 for 60% of the price. Under what circumstances is
that the better purchase?

**B4.** A DWDM system uses the C-band from 1530 to 1565 nm.

(a) Convert both wavelengths to frequency and compute the band's width in THz.
(b) How many channels fit at 100 GHz spacing? At 50 GHz? At 12.5 GHz?
(c) At 400 Gb/s per channel, give the fibre pair's capacity for each spacing.

**B5.** A 6,600 km transatlantic cable has EDFAs every 80 km.

(a) How many amplifiers are in the system?
(b) At 0.2 dB/km, what is the loss per span, and what gain must each amplifier provide?
(c) Explain why the number of amplifiers, rather than their gain, sets the system's optical
SNR limit.

**B6.** A coherent transponder runs at 64 Gbaud with dual polarisation.

(a) Compute the line rate for QPSK, 16QAM and 64QAM.
(b) A 50 GHz channel has 17 dB of optical SNR. Compute the Shannon capacity.
(c) Which of the modulations in (a) is feasible, and what does that imply about the route
length?

**B7.** An MPLS L3VPN packet carries two labels over Ethernet.

(a) How many bytes of overhead does MPLS add?
(b) The customer sends 1,500-byte IP packets. What frame size must the carrier's core support?
(c) The core's MTU is 1,500. Describe the symptom the customer will report, and relate it to
Chapter 24 §24.3.

**B8.** Compute the one-way and round-trip propagation delay for:

(a) the 5,585 km London–New York great circle
(b) a 6,600 km cable route between the same cities
(c) the difference, and what it is worth to a trading firm at \$1M per millisecond per year

**B9.** A country is served by two submarine cables that both land within 30 km of each other
and both transit the same strait.

(a) State the availability assumption the operator is making.
(b) Give three failure scenarios that take out both.
(c) What would genuine diversity require?

## C. Analyse

**C1.** The T1/E1 divergence is described as a historical accident with permanent
consequences. Analyse what would have been required to avoid it, whether it was achievable in
1962, and what general lesson it offers about standardisation timing. Compare with Chapter 22's
OSI case.

**C2.** SONET reserves 50% of its capacity for protection. Packet networks refused that trade.
Analyse both positions: what each optimises for, what each gives up, and whether MPLS fast
reroute genuinely closes the gap or merely appears to.

**C3.** §50.3 claims the C-band exists because of a coincidence between silica's loss minimum
and erbium's gain band. Analyse what the optical industry would look like if these differed by
200 nm, and identify what would have had to be invented instead.

**C4.** Coherent detection moved dispersion compensation from the optical line into the
receiver's DSP. Analyse the economic consequence of this, and identify one other place in this
book where moving a function from hardware in the path to computation at the edge had a
comparable effect.

**C5.** MPLS's original performance justification disappeared within five years, yet deployment
accelerated. Analyse why, and derive a general observation about how technologies are actually
selected.

**C6.** Traffic engineering re-introduces circuits into a packet network. Argue whether this
vindicates or refutes the packet-switching argument of Chapter 13, being precise about which
claim you are assessing.

**C7.** Segment routing removes per-path state from the core by encoding paths in packets.
Analyse this against the end-to-end argument (Chapter 23 §23.4), and identify what it costs.

**C8.** "The Internet's routing is redundant; its geography is not." Explain this distinction
precisely, give two historical examples, and say what an organisation can actually do about it.

**C9.** Content providers now own the majority of new transoceanic capacity. Analyse the
consequences for: the carriers, competition, resilience, and regulation. State which concerns
you most and why.

## D. Design

**D1.** Design the transport for a regional carrier connecting eight cities in a ring, with
between 10 and 100 Gb/s of demand between each pair. Specify the optical layer, the protection
scheme, the electrical layer, and how capacity is added as demand grows. Justify each choice
against an alternative you rejected.

**D2.** An enterprise has 40 sites and currently buys MPLS L3VPN from one carrier. Design the
route target policy for: full mesh between all sites, a shared services VRF reachable from all
sites, and two sites that must not reach each other. Show the import/export configuration
logically.

**D3.** A carrier's core is congested on one path while a parallel path is idle. Design a
traffic engineering solution. Specify the LSPs, the constraints, the protection, and what you
would monitor. State what you would do differently with segment routing.

**D4.** Design the international connectivity for a Pacific island nation of 300,000 people
currently served by one cable. Budget is limited. Evaluate a second cable, LEO satellite, a
microwave link to a neighbouring island, and doing nothing. Recommend a plan with a stated
availability target.

**D5.** You are asked to verify that two "diverse" 100 Gb/s circuits between London and
Frankfurt are genuinely diverse. Write the verification procedure: what you would ask the
carriers for, what you would check, and what you would refuse to accept as evidence.

## E. Troubleshoot

**E1.** A T1 shows increasing errored seconds over a week with no outage. Give three causes and
the measurement that distinguishes them.

**E2.** A SONET circuit reports B1 errors at one repeater and clean B2 and B3. Localise the
fault and explain your reasoning.

**E3.** A protected SONET ring fails to restore after a fibre cut. State the most likely cause,
why it was not detected during commissioning, and how it should have been.

**E4.** An OTN circuit's pre-FEC error rate has risen steadily over six months; post-FEC errors
remain zero. Explain what is happening, whether it is urgent, and what you would do.

**E5.** After adding four new wavelengths to a DWDM system, three existing wavelengths begin
erroring. Give two possible mechanisms and the corrective action for each.

**E6.** A customer reports that small pings succeed across an MPLS VPN but file transfers hang.
Diagnose, and give the specific configuration to check.

**E7.** A newly provisioned L3VPN site can reach the hub but not the other spokes. State the
most likely cause and the command to confirm it.

**E8.** Traceroute across a carrier's network shows three hops where the customer knows there
are eleven. Explain, and say whether it is a fault.

**E9.** A regional office loses connectivity to Asia; European destinations are unaffected. Give
the first non-local thing you would check, and why it should come before local troubleshooting.

## F. Extend

**F1.** Examine the TeleGeography submarine cable map for your own region. Identify the landing
stations, the chokepoints, and any country served by a single system. Write a page assessing
the region's resilience.

**F2.** Find the ITU-T G.694.1 DWDM frequency grid. Compute the wavelength of channels 20, 40
and 60 at 50 GHz spacing, and verify against the published table.

**F3.** Read the ITU-T G.709 clauses on the OTN frame structure and FEC. Write a page explaining
what the FEC buys, with the coding gain figure and its consequence for span count.

**F4.** Configure an MPLS L3VPN in a lab or simulator (GNS3, containerlab, or equivalent) with
two customers using overlapping 10.0.0.0/8. Capture a packet in the core and identify both
labels. Document what the P router knows about each customer.

**F5.** Research one submarine cable outage in the last five years. Determine the cause, the
repair time, which countries were affected, and how traffic rerouted. Write an incident summary
of one page.

**F6.** Find a public looking glass on a carrier's network and compare the latency to a
destination against the great-circle distance. Compute the implied route length and explain the
difference.

**F7.** Investigate segment routing's adoption. Find two carriers that have publicly described
migrating from RSVP-TE, and summarise what they said motivated it and what it cost them.
