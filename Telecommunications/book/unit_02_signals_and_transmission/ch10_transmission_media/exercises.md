# Chapter 10 — Exercises

## A. Recall

**A1.** State the maximum distance for twisted-pair Ethernet and explain what
determines it. Why is the figure the same for 10BASE-T and 10GBASE-T?

**A2.** Why do 50 Ω and 75 Ω coaxial cable both exist? Give the engineering
optimum each approximates.

**A3.** What eliminates modal dispersion, and what is the core diameter that
achieves it?

**A4.** State the PoE power available at the device for each of the four
standards.

**A5.** Give the six questions of §10.5's media decision procedure, in order.

## B. Apply

**B1.** Compute the reflection coefficient and the percentage of power reflected
when a 50 Ω source drives a 75 Ω cable. Repeat for 75 Ω into 50 Ω and comment on
the symmetry.

**B2.** A fibre link: launch −4 dBm, receiver sensitivity −22 dBm, 31 km of
single-mode at 0.22 dB/km, 6 fusion splices at 0.08 dB, 4 connector pairs at
0.45 dB, and a 2.5 dB repair allowance. Compute the budget, the total loss and the
margin. Is it acceptable?

**B3.** The same route is later extended by 12 km with three more splices.
Recompute. If the margin is now inadequate, give three options and rank them by
cost and disruption.

**B4.** A switch has a 370 W PoE budget across 24 ports. The deployment needs
14 access points at 25.5 W, 6 cameras at 12.9 W and 4 telephones at 6.5 W. Does it
fit? Show the arithmetic. If it does not, give two solutions.

**B5.** Compute the critical angle for a fibre with *n*₁ = 1.4682 and
*n*₂ = 1.4629, and the numerical aperture. What half-angle acceptance cone does that
NA correspond to?

**B6.** A 15 m antenna feeder at 5 GHz uses RG-58, which loses about 1 dB/m at
that frequency. Compute the loss. The antenna has 12 dBi of gain. Is the
installation a net gain or a net loss relative to a stub antenna at the radio?
State what you would do instead and reference the relevant principle from
Chapter 6.

**B7.** Using §10.1's cost model with your own local figures, compute the
percentage cost difference between Cat5e and Cat6a for a 150-outlet installation.
Then compute the cost of re-cabling the same building in eight years, including a
figure for disruption. Present the comparison as you would to a finance director.

## C. Analyse

**C1.** Argue rigorously for fibre between two buildings, using the earthing and
lightning argument rather than the bandwidth one. Your answer should explain what
physically happens during a nearby strike, why copper conducts it and fibre does
not, and what a copper installation would require to be defensible.

**C2.** Single-mode fibre often costs less than multimode, yet multimode
dominates in data centres. Explain the apparent contradiction, identify where the
cost actually sits, and state the link count above which the economics reverse.

**C3.** Coax lost the LAN despite having better electrical properties than
twisted pair. Construct the argument for why, identifying the operational rather
than electrical factors that decided it. Then identify one other technology in this
book that won or lost on the same kind of grounds.

**C4.** A vendor offers a "Cat7 shielded" installation at a 40% premium over
Cat6a, claiming future-proofing to 40 Gb/s. Evaluate the claim. Address: whether
Cat7 is a TIA category, what connector it specifies, what 40GBASE-T actually
requires, and what you would recommend instead.

**C5.** §10.4 states that free space is chosen when cable is impossible,
prohibitive or too slow, or the endpoint moves. Construct a case where all four
conditions are absent and wireless is still the right choice, and defend it.

## D. Design

**D1.** A distribution company occupies a site with:

- **Main building**, three floors, 180 desks, 26 IP telephones per floor, 14 access
  points, 31 cameras. Longest horizontal run 82 m.
- **Warehouse**, 60 m from the main building across a service yard used by
  forklifts and articulated lorries. Contains 9 access points, 12 cameras, and 4
  fixed terminals. Steel-framed with metal racking.
- **Gatehouse**, 310 m from the main building, across the yard and a public
  footpath over which the company has no rights.
- **Weighbridge**, 140 m from the main building, in an open area, requiring one
  camera and one terminal.

Specify the medium for every link. For each, state the decision, the questions from
§10.5 that determined it, the alternative rejected and why, and the condition under
which the answer would change. Compute a loss budget for any fibre link and a PoE
budget for the main building. Identify explicitly which links require local power
provision because fibre cannot supply it.

## E. Troubleshoot

**E1.** A 74 m Cat6 link between a distribution switch and an access switch has
carried 1 Gb/s cleanly for five years. Both switches are replaced with models
supporting 10GBASE-T, and the link negotiates 10 Gb/s.

Observations over the following week:

- The link stays up, no flaps.
- CRC errors increment steadily, worse during business hours.
- Throughput at 10 Gb/s is measurably lower than the old link achieved at 1 Gb/s.
- The cable was certified to Cat6 at installation and passed.
- The run shares a 100 mm containment with eleven other identical links, all newly
  upgraded, all showing the same symptom.
- Forcing any single link to 1 Gb/s makes that link clean; the others still show
  errors.
- Removing four cables from the containment and re-routing them measurably improves
  the remaining eight.

Identify the impairment precisely, explain why the certification result does not
contradict the diagnosis, and explain why the fault appeared only after the upgrade
and only when many links were upgraded together. State two remedies, one immediate
and one correct, and say what should have been specified at installation.
