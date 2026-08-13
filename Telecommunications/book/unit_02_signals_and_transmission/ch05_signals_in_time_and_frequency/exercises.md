# Chapter 5 — Exercises

## A. Recall

**A1.** State what distinguishes an analog signal from a digital one, without
using the words "smooth" or "square".

**A2.** A receiver measures 4.3 V on a link whose agreed levels are 0 V and 5 V.
What does it output, and what happened to the 0.7 V of discrepancy?

**A3.** Sketch the frequency spectrum of: (a) a pure 1 kHz sine wave; (b) a 1 kHz
square wave; (c) thermal noise; (d) a single very short pulse.

**A4.** Define the 3 dB bandwidth. What ratio of output power does 3 dB
correspond to, and why is the convention 3 rather than 6 or 10?

**A5.** Classify as baseband or broadband, in the engineering sense: 1000BASE-T;
ADSL; Wi-Fi; DOCSIS; a fibre link running 10GBASE-LR.

## B. Apply

**B1.** A square wave at 2 MHz is sent down a cable whose 3 dB bandwidth is
7 MHz. Which harmonics survive substantially? Describe the received waveform, and
state whether a receiver deciding at the midpoint would still recover the data
correctly.

**B2.** Explain, in terms of harmonics, why a cable that carries 10 Mb/s
comfortably may fail at 100 Mb/s even though its physical length is unchanged.

**B3.** Ten amplifiers in series, each adding 0.05 V RMS of noise to a 1 V signal
and each with a gain of exactly 1. Compute the final signal-to-noise ratio in dB,
assuming noise powers add. Repeat for ten regenerators, assuming each makes its
decision correctly.

**B4.** Derive the minimum bandwidth needed to carry 100 Mb/s using binary
signalling, from the alternating-bit argument in §5.3. Then explain why
100BASE-TX is specified for Cat5 cable rated to 100 MHz rather than to the
minimum you computed.

**B5.** 1000BASE-T carries 1 Gb/s over four pairs of Cat5e rated to 100 MHz.
Work the arithmetic: bits per pair, symbols per second given PAM-5, and the
bandwidth that implies. Show that it fits, and state the margin.

**B6.** A DSL line uses subcarriers spaced 4.3125 kHz apart from 138 kHz to
1.1 MHz. How many downstream subcarriers are there? If each can carry between 0
and 15 bits per symbol at 4,000 symbols per second, what is the theoretical
maximum downstream rate, and what would reduce it in practice?

**B7.** Quantisation error is bounded but non-zero. For an 8-bit quantiser
covering a 2 V range, compute the maximum error. Then explain why this error does
not accumulate over a 5,000 km digital link, and why the equivalent analog error
would.

## C. Analyse

**C1.** Show that a channel which passes only the fundamental of an alternating
bit stream still permits correct decisions, and identify the property of the
received sine wave that makes this possible. Then construct a bit sequence for
which fundamental-only reception fails, and explain what a line code (Chapter 7)
would do about it.

**C2.** The Fourier transform of a very short pulse is very wide. Use this to
explain (a) why fast digital edges cause electromagnetic interference across a
broad band, (b) why deliberately slowing edges is a standard mitigation, and
(c) what that mitigation costs.

**C3.** Argue for or against the following claim: "Digital transmission is
superior to analog transmission." Your answer must identify at least one
circumstance in which analog is preferable and quantify the cost digital pays.

**C4.** A receiver's channel filter is narrowed from 20 MHz to 10 MHz. State the
effect on (a) the noise power admitted, in dB, (b) the maximum symbol rate, and
(c) the Shannon capacity, assuming signal power is unchanged. Is the change
beneficial? Under what circumstance would it be?

## D. Design

**D1.** You are specifying the cabling for a new building. The client wants
"future-proof" and cannot say what rate will be needed in ten years.

Using only the material of Chapters 4 and 5, construct the argument for what
property of the cable you should actually specify — and explain why specifying a
data rate is the wrong approach. Support it with the 2.5GBASE-T example: a
standard published in 2016 extracting 2.5 Gb/s from cable specified in 1999 for
100 Mb/s. What does that history imply for your recommendation?

## E. Troubleshoot

**E1.** A 300 m fibre run between two buildings has worked at 1 Gb/s for six
years. The link is upgraded to 10 Gb/s transceivers at both ends. The link comes
up, and then shows a steady rate of CRC errors and unusable throughput. Reverting
to the 1 Gb/s transceivers restores normal operation.

The fibre has not been touched. Using the material of this chapter and
Chapter 6's preview, give three candidate explanations, rank them, and state the
single measurement that would distinguish them. Then explain the general principle
about why a medium that is adequate at one rate can be inadequate at a higher one
without anything having changed.
