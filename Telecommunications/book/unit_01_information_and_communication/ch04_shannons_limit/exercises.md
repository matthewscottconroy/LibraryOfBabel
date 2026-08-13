# Chapter 4 — Exercises

## A. Recall

**A1.** Compute the entropy of: (a) a fair six-sided die; (b) a coin biased 75/25;
(c) a source emitting one of eight symbols with equal probability; (d) a source
that always emits the same symbol.

**A2.** Convert to decibels: ratios of 2, 10, 50, 1,000, 0.5, 0.001.
Convert to linear ratios: 0 dB, +6 dB, +13 dB, −20 dB, −33 dB.

**A3.** Convert to dBm: 1 W, 250 mW, 40 mW, 1 µW, 5 pW.
Convert to watts: +23 dBm, 0 dBm, −30 dBm, −85 dBm.

**A4.** A transmitter emits +23 dBm. The path loses 78 dB. The receive antenna
provides +5 dBi of gain and the cable to it loses 2 dB. What is the received power
in dBm? If the noise floor is −95 dBm, what is the SNR in dB?

**A5.** State the difference between a symbol and a bit, and between baud and bits
per second. Under what condition are the last two numerically equal?

## B. Apply

**B1.** A source emits four symbols with probabilities 0.5, 0.25, 0.125, 0.125.
(a) Compute the entropy. (b) Design a prefix-free code achieving exactly that
average length. (c) Compare with a fixed 2-bit code and state the saving.

**B2.** A channel has 6 MHz of bandwidth (one North American TV channel).
(a) What is the Nyquist maximum symbol rate? (b) At 64-QAM, what data rate does
that permit? (c) ATSC 8-VSB digital television achieves about 19.4 Mb/s in this
channel. Explain the gap.

**B3.** Compute the thermal noise floor in dBm for bandwidths of 1 kHz, 200 kHz
(a GSM channel), 20 MHz, and 100 MHz, at 290 K. Then recompute the 20 MHz figure
for equipment running at 55 °C, and state the degradation in dB.

**B4.** A Wi-Fi client measures −72 dBm signal and −96 dBm noise on a 40 MHz
channel. (a) Compute SNR. (b) Compute the Shannon capacity. (c) The client
negotiates a 200 Mb/s PHY rate. Is that plausible? (d) The user walks 15 m
further and signal drops to −86 dBm. Recompute capacity and comment.

**B5.** Compute Shannon capacity for a telephone channel of 3,100 Hz at SNR of
20 dB, 30 dB, and 40 dB. Plot or tabulate. How much does each additional 10 dB
buy, and what does that tell you about the value of improving line quality?

**B6.** A satellite link has 36 MHz of transponder bandwidth and operates at 12 dB
SNR. (a) Compute capacity. (b) The operator proposes doubling transmit power.
Compute the new capacity, assuming noise is unchanged, and express the improvement
as a percentage. (c) Instead they propose leasing a second 36 MHz transponder.
Compute that improvement. Which is the better investment and why?

**B7.** Show that Nyquist's *C* = 2*B* log₂ *M* and Shannon's *C* = *B* log₂(1+SNR)
agree when *M* = √(1 + SNR). Interpret this: what does it say about how many
signal levels a given SNR can support?

## C. Analyse

**C1.** A V.90 modem achieves 56 kb/s downstream but only 33.6 kb/s upstream.
Explain both figures using the material of §4.2 and §4.4. Your answer must
identify why the downstream path is not subject to the same Shannon bound as the
upstream path, and must account for the gap between the theoretical 64 kb/s and
the achieved 56 kb/s.

**C2.** Derive the low-SNR approximation *C* ≈ 1.44 *B* · SNR from the exact
formula, using log₂(1+x) ≈ x/ln 2 for small *x*. Then use it to explain how a GPS
receiver decodes a signal arriving 20 dB below the thermal noise floor. What is
being traded for what?

**C3.** Shannon's coding theorem promises arbitrarily low error rates below
capacity, but its proof uses randomly chosen codes over arbitrarily long blocks.
Explain concretely why long blocks are necessary, and what practical cost they
impose. Then explain why 5G defines a separate ultra-reliable low-latency
communication mode rather than simply using its best codes everywhere.

**C4.** An administrator has 30 access points in an office and, to improve
coverage, raises every AP's transmit power from 11 dBm to 20 dBm. Argue
quantitatively that this will *reduce* aggregate network capacity. Your answer
should address SNR at the client, the noise-plus-interference floor at every other
AP, and the effect on the CSMA/CA deferral behaviour you will meet in Chapter 44.

**C5.** English text has a per-symbol entropy of about 4.1 bits treating
characters independently, but Shannon's 1951 experiments estimated the true value
at 0.6–1.3 bits/character. (a) Explain the gap. (b) Compute the maximum
theoretical compression ratio implied. (c) Explain why gzip achieves roughly 3:1
on English text rather than the 4:1 or better this suggests.

## D. Design

**D1.** You must provide a data link to a mountaintop weather station 34 km from
the nearest connected building, with clear line of sight. The station generates
2 Mb/s of sensor and camera data, needs interactive SSH access, and has 200 W of
solar power available.

Options: (a) unlicensed 5 GHz point-to-point radio, 20 MHz channel, +23 dBm
maximum EIRP under local rules, 27 dBi dish antennas at both ends; (b) licensed
microwave at 18 GHz, 28 MHz channel; (c) trenched fibre at €40/metre.

For option (a), compute the free-space path loss (Chapter 42 gives the formula;
for now use FSPL(dB) = 32.45 + 20 log₁₀ f(MHz) + 20 log₁₀ d(km)), the received
signal level, the noise floor, the SNR, and the Shannon capacity. State whether
the requirement is met and with what margin. Then justify a recommendation across
all three options, addressing cost, capacity, availability in rain, and
regulatory risk.

## E. Troubleshoot

**E1.** A wireless vendor's datasheet advertises "up to 1.2 Gb/s" for a 2×2
802.11ax access point. A customer deploys it, measures 340 Mb/s with `iperf3` from
a laptop 4 m away with a reported RSSI of −41 dBm, and opens a support case
alleging the product is defective.

Using the material of Chapters 3 and 4, write the technical response. Account for
every factor separating 1.2 Gb/s from 340 Mb/s, quantifying where you can:
the PHY-rate-versus-throughput gap, the number of spatial streams the laptop
supports, channel width, guard interval, MAC overhead, half-duplex operation, and
anything else you can identify. State what measurement would distinguish a genuine
fault from expected behaviour.
