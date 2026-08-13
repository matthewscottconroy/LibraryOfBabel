# Chapter 6 — Exercises

## A. Recall

**A1.** Name the four things the world can do to a signal, and give the standard
term for each.

**A2.** For each impairment, state how it varies with: distance, frequency,
temperature, and offered traffic load. Present as a table.

**A3.** Why is a twisted pair twisted? Answer in terms of what the receiver does
with the two conductors.

**A4.** Distinguish NEXT from FEXT, and state which dominates on a short link and
why.

**A5.** What does an eye diagram's vertical opening measure? What does its
horizontal opening measure?

## B. Apply

**B1.** A fibre link: transmitter launches at −2 dBm, receiver sensitivity is
−24 dBm. The path is 55 km of fibre at 0.22 dB/km, with 8 splices at 0.08 dB and
4 connectors at 0.4 dB. Compute the total loss, the received power, and the
margin. Is the link acceptable? State your criterion.

**B2.** The same link is extended to 90 km with two additional splices. Recompute.
If the margin is inadequate, state three options and rank them by cost.

**B3.** Skin depth in copper is 66 µm at 1 MHz and scales as 1/√f. Compute it at
10 MHz, 100 MHz and 1 GHz. A conductor is 0.5 mm in diameter; at which of those
frequencies is the effective conducting cross-section less than half the physical
one?

**B4.** Cat5e loses about 22 dB per 100 m at 100 MHz. A run is 140 m. Estimate the
loss, assuming loss is proportional to length. The receiver needs a signal 18 dB
above the noise floor. Given a transmitter output of 0 dBm and a noise floor of
−40 dBm, does the link work at 100 MHz? Show the arithmetic.

**B5.** A multimode link at 10 Gb/s must span 250 m. Which OM grade is required?
If only OM2 is installed, state two options that do not involve replacing the
fibre, and the cost of each.

**B6.** Standard single-mode fibre has chromatic dispersion of 17 ps/(nm·km) at
1550 nm. A source has a 0.15 nm linewidth. Compute the pulse spread over 100 km.
At 10 Gb/s a bit period is 100 ps; at 100 Gb/s it is 10 ps. Comment on both.

**B7.** A receiver has a noise figure of 7 dB and a bandwidth of 40 MHz. Compute
its practical noise floor at 290 K. A signal arrives at −78 dBm. What is the SNR,
and what Shannon capacity does that imply?

**B8.** Use Friis's formula to compute the total noise figure of: (a) an LNA with
NF 1 dB and gain 20 dB, followed by a receiver with NF 8 dB; (b) the same two
components with 6 dB of cable loss placed *before* the LNA. State the penalty and
explain it in one sentence.

## C. Analyse

**C1.** Show that intersymbol interference and Nyquist's bandwidth limit are the
same constraint expressed in two domains. Your answer should explain why a channel
that attenuates high frequencies necessarily spreads pulses in time.

**C2.** Impulse noise is bursty. Consider two channels with the same average bit
error rate of 10⁻⁵: one with errors distributed uniformly, one delivering
100% errors for 10⁻⁵ of the time. For 1,500-byte frames, compute the frame error
rate in each case. Comment on which channel is harder to use and why the average
bit error rate is a misleading figure.

**C3.** A split-pair cable passes continuity and fails at 1 Gb/s. Explain the
mechanism from first principles, then explain why it works at 100 Mb/s. Then
design a test, using only a continuity tester and a laptop, that would distinguish
a split pair from a correctly wired cable — or argue rigorously that no such test
exists and state what instrument is required.

**C4.** Equalisation lets 802.3bz extract 2.5 Gb/s from Cat5e specified for
100 MHz. Explain what equalisation does and why it does not violate Shannon's
limit. Then state what property of the channel makes equalisation possible and
what would make it impossible.

**C5.** Argue for or against: "Shielded cable is always better than unshielded."
Your answer must address earthing, and must identify a realistic scenario in
which shielded cable performs worse.

## D. Design

**D1.** A manufacturing company is cabling a new plant. Three environments:

- **Offices** — 40 desks, standard commercial construction, 60 m maximum run.
- **Production floor** — 6 machine controllers, 3-phase drives and welding
  equipment throughout, runs up to 85 m, ambient temperature to 40 °C.
- **Yard** — a gatehouse and a weighbridge, 310 m and 180 m from the main comms
  room, outdoors, occasional lightning activity.

For each, specify the medium and justify it against **each of the four
impairments** by name. Where you specify shielded cable, state the earthing
requirement. Where you specify fibre, state the type and compute a loss budget.
State the assumption you are making about future data rates and what would change
your answer.

## E. Troubleshoot

**E1.** A 1 Gb/s copper link between two buildings has run for four years. Since
a new production line was commissioned six weeks ago, users report intermittent
slowness. The evidence:

- The link stays up; no flaps recorded.
- `iperf3` gives 940 Mb/s at 07:00 and 180 Mb/s at 14:00.
- Interface counters show CRC errors incrementing, at a rate that varies through
  the day.
- Plotting the error rate against time shows it rising sharply at 08:30, falling
  at 12:30, rising at 13:15, falling at 17:00.
- The cable was certified when installed and passed.
- Utilisation never exceeds 12%.

Identify the impairment, the mechanism, and the piece of evidence that identifies
it uniquely. State why utilisation is a red herring here, and give three remedies
ranked by cost and disruption. Then state what should have been in place that
would have identified this in the first week rather than the sixth.
