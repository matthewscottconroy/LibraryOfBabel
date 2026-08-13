# Chapter 8 — Exercises

## A. Recall

**A1.** Compute the wavelength and quarter-wave antenna length for: 100 kHz,
900 MHz, 2.4 GHz, 5 GHz, 28 GHz. Comment on which are practical.

**A2.** Name the three parameters of a sinusoidal carrier and the keying scheme
that varies each. Which is least robust, and why?

**A3.** How many bits per symbol does each carry: BPSK, QPSK, 16-QAM, 256-QAM,
4096-QAM?

**A4.** What does the *O* in OFDM stand for, and what property does it guarantee?

**A5.** What is a cyclic prefix and what does it discard?

## B. Apply

**B1.** A QPSK constellation has four points at radius *A*. Compute the minimum
distance between adjacent points. Repeat for 16-QAM with the same peak amplitude,
and state the ratio.

**B2.** Using the 3 dB per additional bit per symbol rule, compute the SNR required
for 1024-QAM given that QPSK requires 7 dB. Compare with §8.3's table and comment
on the discrepancy.

**B3.** A 20 MHz Wi-Fi channel has a noise floor of −95 dBm. Compute the received
signal strength required to support 256-QAM, and then 4096-QAM. Comment on what
those figures imply about physical proximity to the access point.

**B4.** 802.11a divides a 20 MHz channel into 64 subcarriers. Compute the
subcarrier spacing and confirm it equals the reciprocal of the 3.2 µs symbol
period. Explain why that relationship is required rather than convenient.

**B5.** With a 3.2 µs symbol and an 800 ns cyclic prefix, compute the overhead as
a percentage. Repeat for a 400 ns short guard interval and state the throughput
gain. Under what environmental condition would the short interval be a mistake?

**B6.** A delay spread of 250 ns is measured in a warehouse. Which guard interval
is safe? A vendor's default is short GI. State what you would change and what
symptom would have alerted you.

**B7.** A single-carrier system runs at 50 Msymbols/s. Compute the symbol period.
In an environment with 400 ns delay spread, how many subsequent symbols does each
reflection contaminate? Now compute the same for an OFDM system with 3.2 µs
symbols and comment.

## C. Analyse

**C1.** Derive the quadrature decomposition from the angle addition formula, and
show that *I* and *Q* are orthogonal by evaluating the integral of
cos(2π*ft*)·sin(2π*ft*) over one period. Explain what practical consequence
follows from the integral being zero.

**C2.** Gray coding assigns bit patterns so adjacent constellation points differ
in one bit. Construct a Gray-coded assignment for 16-QAM. Then compute the average
number of bit errors per symbol error for your assignment and for a naive binary
assignment, assuming errors go only to adjacent points.

**C3.** Explain why OFDM's peak-to-average power ratio is high, and why this
matters more for a battery-powered transmitter than for a base station. Then
explain why LTE uses SC-FDMA on the uplink and OFDMA on the downlink, and state
what SC-FDMA gives up.

**C4.** OFDM was patented in 1966 and became practical in the 1990s. Identify what
changed, name the specific algorithm involved, and explain why the technique is
computationally cheap despite generating hundreds of carriers.

**C5.** A constellation diagram shows points smeared into arcs around the origin,
with the radial spread unchanged. Diagnose it. Then describe what you would see
instead for: an over-driven amplifier, a frequency offset, and I/Q imbalance.
Explain why all four would look identical on a simple signal-strength measurement.

## D. Design

**D1.** You are specifying a wireless link for a fleet of automated guided
vehicles in a 12,000 m² distribution warehouse. Metal racking to 6 m, concrete
floor, vehicles moving at up to 3 m/s, and the control loop requires a round trip
under 20 ms with no more than 0.1% packet loss.

Address, with reasoning: the delay spread you would expect and how you would
measure it; the guard interval setting and why; whether you would permit the
highest QAM orders and what you would do about rate adaptation; whether OFDMA
helps here; and what modulation you would expect the links to actually settle at.
State one measurement you would take before committing to any of it.

## E. Troubleshoot

**E1.** A cable operator's technician is investigating complaints from one street.
The DOCSIS modems in twelve houses report:

- Downstream power levels within specification, all houses.
- Downstream SNR: 34–36 dB in ten houses, 27 dB in two.
- The two low-SNR houses have negotiated 256-QAM; the others 1024-QAM.
- Constellation display at the head end for the affected segment shows the outer
  points of the constellation pulled measurably inward, symmetrically, with the
  inner points unaffected.
- The problem appeared after an amplifier was replaced last month.

Identify the fault, explain the mechanism, and state why the two houses at the
lower SNR are affected more than the others. Then state what should be adjusted
and in which direction — noting that the intuitive direction is wrong.
