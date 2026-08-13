# Chapter 5 — Important Concepts

**Analog signal** *(§5.1)* — One in which the entire continuous range of values is
meaningful: the value *is* the information. The receiver measures.

**Digital signal** *(§5.1)* — One in which only a discrete set of values is
meaningful, everything between being an imperfect rendering of the nearest. The
receiver **decides**. The distinction is in the agreement, not in the waveform —
every signal on every wire is physically continuous.

**Regeneration** *(§5.1)* — Deciding which discrete value was sent and emitting a
fresh clean signal. The output is not a better copy but a *new* signal, so noise
does not accumulate across hops. The single largest advantage of digital
transmission and the reason Chapter 12 calls digitisation the telephone network's
biggest improvement.

**Amplification** *(§5.1)* — Multiplying the input. Signal and noise scale
together, so SNR is unchanged and each span's noise is permanent. Chained
amplifiers accumulate noise; chained regenerators do not.

**Threshold behaviour** *(§5.1)* — Digital transmission converts gradual
degradation into a cliff: perfect until the noise exceeds half the gap between
levels, then abruptly broken. This is why digital television is flawless or absent,
and why Unit XIII looks for error counters rather than quality measurements.

**Quantisation error** *(§5.1)* — The difference between a true analog value and
the nearest representable level. Noise we manufacture ourselves. Bounded — its
great virtue — and it does not grow with distance because the signal is digital
thereafter.

**Fourier decomposition** *(§5.2)* — Any reasonable periodic signal is a sum of
sinusoids at integer multiples of a fundamental. Claimed by Fourier in 1807,
rejected by Lagrange, published 1822.

**Time domain / frequency domain** *(§5.2)* — Two lossless descriptions of one
signal: what it does over time, and how much energy it holds at each frequency.
Identical information; different facts made obvious. Nearly every fact a network
engineer needs is obvious only in the second.

**Harmonics** *(§5.2)* — A square wave is its fundamental plus odd harmonics at
3×, 5×, 7× with amplitudes 1/3, 1/5, 1/7. **The sharp corners *are* the high
harmonics**, so a channel that attenuates high frequencies necessarily rounds the
corners — a subtraction rather than a mystery.

**Spectrum shapes worth recognising** *(§5.2)* — A sine is one line; a square wave
is a picket fence of odd harmonics; a short isolated pulse has a wide continuous
spectrum (hence fast edges radiate broadly); thermal noise is flat, which is why
noise power scales with bandwidth.

**Filter** *(§5.2)* — A device whose frequency response is deliberately not flat:
low-pass, high-pass, band-pass, band-stop. A cable is an unintentional low-pass
filter, and much of high-speed design consists of noticing that something you did
not think of as a filter is behaving as one.

**Fast Fourier Transform** *(§5.2)* — Cooley and Tukey, 1965. Reduces an *N*-point
transform from *N*² to *N* log *N* operations, making spectral analysis routine.
Present and running continuously in every modem, OFDM radio and DSL line card in
this book. Known to Gauss in 1805 and unpublished.

**Frequency response** *(§5.3)* — Output amplitude plotted against frequency for a
constant-amplitude input sweep. The complete characterisation of what a channel
does to a signal.

**3 dB bandwidth** *(§5.3)* — The width of the band over which output power stays
within a factor of two of its maximum. A convention, not a law; optical and
regulatory practice use others, so check which a datasheet means before comparing.

**Sources of a channel's bandwidth** *(§5.3)* — The medium's physics; the equipment
at the ends; regulation; and deliberate filtering. Identifying which one binds is a
diagnostic skill — DSL exists because the telephone's limit was in the equipment
rather than the copper.

**Bandwidth versus bit rate** *(§5.3)* — Different quantities in different units.
A given bandwidth supports a range of bit rates depending on bits per symbol: a
6 MHz television channel carries 19.4 Mb/s under ATSC and about 40 Mb/s under
DVB-C.

**Minimum bandwidth rule** *(§5.3)* — An alternating bit stream at *R* b/s is a
square wave at *R*/2 Hz, so *R*/2 Hz is the absolute minimum for binary
signalling — Nyquist read backwards. Practical systems want 0.5–1 Hz per b/s.

**Baseband** *(§5.4)* — Signalling that puts data directly on the medium,
occupying the band from DC upward. The `BASE` in `1000BASE-T`. Cannot be moved in
frequency, so sharing must be arranged in **time**.

**Broadband (engineering sense)** *(§5.4)* — Signalling modulated onto a carrier,
so its spectrum sits somewhere other than at DC. Several such signals coexist at
different frequencies, which is what makes frequency-division multiplexing
possible. Distinct from the colloquial sense of "fast Internet access", which is
now universal and unrelated.

**Why radio must be broadband** *(§5.4)* — A baseband signal's wavelength is
kilometres, so no practical antenna exists. Every radio system in this book
modulates; Chapter 8 §8.1 gives the arithmetic.
