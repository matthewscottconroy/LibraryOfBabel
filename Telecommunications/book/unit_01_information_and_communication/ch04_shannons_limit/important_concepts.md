# Chapter 4 — Important Concepts

**Entropy** *(§4.1)* — The average information per symbol from a source:
*H* = −Σ *pᵢ* log₂ *pᵢ* bits/symbol. Maximised when all symbols are equally likely;
zero when the source is deterministic. Named on von Neumann's advice for its
formal parallel with thermodynamic entropy.

**Source coding theorem** *(§4.1)* — A source of entropy *H* can be losslessly
encoded in an average of *H* bits per symbol, and no fewer. The theorem that makes
"compresses any file by 50%" provably false.

**Conditional structure** *(§4.1)* — Real sources are not memoryless; context
reduces entropy dramatically. English is ~4.1 bits/char treated independently but
0.6–1.3 bits/char in reality. All practical compression exploits this.

**Incompressibility of encrypted and compressed data** *(§4.1)* — Ciphertext is
designed to be statistically indistinguishable from random, hence maximum entropy,
hence incompressible. Compression must precede encryption — with its own hazard
(CRIME/BREACH, Chapter 58).

**Symbol** *(§4.2)* — One signalling event: one discrete channel state held for one
signalling interval. Distinct from a bit.

**Baud / symbol rate** *(§4.2)* — Symbols per second, after Émile Baudot. Equal to
bits per second only when there are exactly two symbols. "Baud rate" is almost
always used incorrectly in vendor material.

**Bits per symbol** *(§4.2)* — log₂ *M* for an alphabet of *M* distinguishable
symbols. The design choice that separates baud from b/s:
data rate = symbol rate × log₂ *M*.

**Nyquist limit** *(§4.2)* — A channel of bandwidth *B* supports at most 2*B*
symbols per second without intersymbol interference. The bandwidth half of the
ceiling; says nothing about noise.

**Nyquist criterion / raised-cosine shaping** *(§4.2)* — Pulse shapes that are zero
at every other symbol's sampling instant, enabling zero-ISI signalling. The
roll-off factor (0.15–0.35) is the excess bandwidth that makes the filter
realisable.

**Nyquist sampling theorem** *(§4.2)* — Reconstructing a signal with components up
to *B* Hz requires ≥ 2*B* samples per second. Sampling too slowly causes
**aliasing**, which is irreversible. Source of the telephone network's 8 kHz
sampling rate and hence the 64 kb/s **DS0**.

**Decibel** *(§4.3)* — 10 log₁₀ of a power ratio (20 log₁₀ for amplitude ratios).
Turns multiplication into addition and compresses enormous ranges. +3 dB = double,
+10 dB = ten times.

**dBm** *(§4.3)* — Absolute power referenced to 1 mW. dBm + dB = dBm, which makes
a complete link budget one line of addition.

**Thermal (Johnson–Nyquist) noise** *(§4.3)* — *N* = *kTB*. Unavoidable noise from
thermal motion of charge carriers in any conductor above absolute zero. Measured by
Johnson, explained by Nyquist, 1926–27.

**−174 dBm/Hz** *(§4.3)* — The thermal noise floor at 290 K per hertz of
bandwidth. Add 10 log₁₀(*B*) for a given bandwidth. The number every receiver on
Earth works against; worth memorising.

**Noise figure** *(§4.3)* — The additional noise a receiver's own electronics
contribute above the thermal floor, typically 4–10 dB in consumer equipment.

**Signal-to-noise ratio (SNR)** *(§4.3)* — Signal power ÷ noise power. In dB, it is
simply received dBm minus noise-floor dBm. **SINR** adds interference to the
denominator and is the honest measure in any shared medium.

**Shannon–Hartley theorem** *(§4.4)* — *C* = *B* log₂(1 + SNR) bits per second.
Below *C*, arbitrarily reliable communication is achievable; above it, impossible.
The most important formula in telecommunications.

**Linear in bandwidth, logarithmic in SNR** *(§4.4)* — The formula's key structural
fact. Doubling bandwidth roughly doubles capacity; doubling power (+3 dB) buys one
extra bit per symbol per hertz. This is why the industry chases spectrum rather
than watts, and the quantitative form of Chapter 1's Whitehouse lesson.

**Low-SNR regime** *(§4.4)* — For SNR ≪ 1, *C* ≈ 1.44 *B* · SNR: capacity becomes
linear in SNR and bandwidth is nearly free. Governs deep-space links, spread
spectrum, and GPS.

**Shannon limit on E_b/N_0** *(§4.4)* — −1.59 dB. Below this energy per bit
relative to noise spectral density, no communication is possible at any rate with
any code. Modern LDPC and turbo codes operate within ~0.5 dB of it.

**The four-question test** *(§4.4)* — What bandwidth? What SNR? What does Shannon
permit? What rate is required? Four numbers and one logarithm decide whether any
proposed link is possible. The most durable result in Unit I.

**PHY rate versus throughput** *(§4.4)* — Advertised wireless rates are physical-
layer rates at ideal SNR before protocol overhead. Real throughput is typically
40–60% at good signal and much less at range.
