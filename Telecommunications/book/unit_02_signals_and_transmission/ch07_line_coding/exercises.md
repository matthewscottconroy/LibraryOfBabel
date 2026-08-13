# Chapter 7 — Exercises

## A. Recall

**A1.** State the two problems raw NRZ signalling fails to solve, and say which
physical component in an Ethernet interface causes the second.

**A2.** Encode `1 0 0 1 1 0` in Manchester, drawing the waveform. Then state the
signalling rate needed to carry it at 100 Mb/s.

**A3.** Compute the overhead and efficiency of: Manchester, 4B/5B, 8B/10B,
64B/66B, 256B/257B.

**A4.** What is running disparity, and what property does it guarantee?

**A5.** How many bits per symbol does PAM-8 carry, and what SNR penalty does it
pay relative to NRZ?

## B. Apply

**B1.** Two crystals differ by 80 ppm. How many bit periods elapse before the
accumulated drift reaches half a bit? At 2.5 Gb/s, how long is that in
microseconds, and how many maximum-size Ethernet frames does it correspond to?

**B2.** A 4B/5B code guarantees no more than three consecutive zeros. Verify this
by taking the code words for `0000` and `0001` from §7.3's table, concatenating
them in both orders, and counting the longest zero run in each.

**B3.** 1000BASE-X carries 1 Gb/s using 8B/10B. What is the signalling rate on the
wire? Repeat for 10GBASE-R with 64B/66B carrying 10 Gb/s.

**B4.** Work the full 1000BASE-T arithmetic from §7.4: data rate, pairs, bits per
symbol, symbols per second per pair, fundamental frequency. Show that it fits
within Cat5e's 100 MHz and state the margin.

**B5.** Repeat for 10GBASE-T: 10 Gb/s over four pairs using PAM-16 at 800 Mbaud.
Compute bits per symbol, and confirm that the arithmetic is consistent. What
bandwidth does 800 Mbaud imply, and which cable category is required?

**B6.** A channel has 12 dB more SNR available than PAM-2 requires. Which is the
highest PAM order it can support at the same error rate? Show your working.

**B7.** A designer must double a link's data rate on an existing cable. Option A:
double the symbol rate, costing 7 dB of additional attenuation at the higher
frequencies. Option B: move from PAM-2 to PAM-4. Which requires less additional
SNR? State the assumption that decides it and what measurement would settle it.

## C. Analyse

**C1.** 8B/10B provides error detection from two independent mechanisms. Name both
and explain how each detects an error. Then construct a single-bit error that one
mechanism catches and the other does not, and state whether a single-bit error can
ever escape both.

**C2.** The comma character `K28.5` contains a bit sequence that cannot appear
anywhere in any concatenation of valid data code words, including across code word
boundaries. Explain why this property is necessary for symbol alignment, and
explain what a receiver must do on power-up before it can decode anything.

**C3.** 64B/66B provides its guarantee statistically rather than by construction.
Estimate the probability of a 66-bit run of identical bits from a scrambled stream,
treating the scrambler output as uniformly random. Then explain why the standards
nonetheless specify adversarial test patterns, and what an attacker with control
of payload contents could in principle attempt.

**C4.** Trellis-coded modulation recovers 3–6 dB of the multilevel penalty without
increasing transmit power. Explain, qualitatively, where the gain comes from —
your answer must address why constraining which *sequences* are legal increases
the effective distance between signals. Then explain why this was surprising in
1982.

**C5.** Trace the industry's movement down the efficiency ladder from Manchester
to 256B/257B, and identify for each step (a) what constraint had changed, and
(b) what was given up. Then predict what the next step would require and what it
would give up.

## D. Design

**D1.** You are designing a link for a sensor network in a steel mill. The
constraints:

- 400 m runs, in conduit alongside high-current three-phase cable.
- 10 Mb/s of data is ample and will not grow.
- Cost per node must be minimal; the nodes are battery-powered where possible.
- Field wiring will be done by electricians, not network technicians, and
  conductor reversal is likely.
- Ambient electrical noise is severe and impulsive.

Choose a line code and a signalling scheme, and justify every choice against the
constraints. Address specifically: why you would or would not use a self-clocking
code here despite its 50% efficiency cost; whether you would use multilevel
signalling; and what you would do about the reversal risk. State what you would
measure before committing.

## E. Troubleshoot

**E1.** A 1000BASE-T link between two switches has run cleanly for three years.
The switches are replaced with models supporting 2.5GBASE-T, and both ends
negotiate 2.5 Gb/s. The link comes up.

Within a day, monitoring shows:

- The link stays up; no flaps.
- CRC errors incrementing steadily, at a rate that rises with traffic.
- Throughput at 2.5 Gb/s negotiated is *lower* than the old link achieved at
  1 Gb/s.
- Forcing both ends to 1 Gb/s restores clean operation and full throughput.
- The cable is Cat5e, 78 m, certified when installed and never touched.

Explain the mechanism using §7.4's material. State why the link negotiated
successfully despite being unable to sustain the rate, why throughput fell *below*
the old figure rather than merely failing to improve, and what single measurement
would confirm your diagnosis. Then state two remedies and the condition under which
each is appropriate.
