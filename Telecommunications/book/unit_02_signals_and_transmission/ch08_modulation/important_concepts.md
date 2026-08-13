# Chapter 8 — Important Concepts

**Why a carrier is necessary** *(§8.1)* — Two independent reasons. **Antenna
size**: an efficient antenna is a substantial fraction of a wavelength, and a
3 kHz baseband signal has a 100 km wavelength, so a quarter-wave antenna would be
25 km tall. **Sharing**: a baseband signal cannot be moved in frequency, so
multiple baseband transmitters on one medium necessarily collide.

**The three carrier parameters** *(§8.1)* — Amplitude, frequency, phase. There is
no fourth, so there are exactly three primitive keying schemes and every modulation
scheme in existence is a combination of them plus the use of multiple carriers.

**ASK** *(§8.1)* — Vary amplitude. Simplest to build; least robust, because
amplitude is exactly what fading and attenuation attack. Survives in optical
on-off keying and as a component of QAM.

**FSK** *(§8.1)* — Vary frequency. Robust to amplitude variation and tolerant of
non-coherent detection; spectrally inefficient. Used where robustness and low cost
matter more than efficiency — Bluetooth basic rate, LoRa, low-rate IoT radios.

**PSK** *(§8.1)* — Vary phase. Most power-efficient for a given error rate;
requires a phase reference, supplied by coherent detection or by differential
encoding. BPSK and QPSK are the robust fallback modes of every modern standard.

**Quadrature decomposition** *(§8.2)* — *A*cos(2π*ft* + φ) = *I*cos(2π*ft*) +
*Q*sin(2π*ft*), where *I* = *A*cos φ and *Q* = −*A*sin φ. Converts an
amplitude-and-phase problem into two independent amplitude problems.

**Why quadrature matters** *(§8.2)* — A transmitter becomes two multipliers and an
adder; a receiver becomes two multipliers and two filters; and because sine and
cosine at one frequency are **orthogonal**, *I* and *Q* carry independent data in
the same bandwidth. QPSK therefore carries two bits per symbol at the same power
and bandwidth as BPSK carries one.

**The I/Q plane** *(§8.2)* — Distance from origin is amplitude, angle is phase.
A modulation scheme is a chosen set of points; that set is a **constellation**.

**Constellation diagnostics** *(§8.2)* — Radial smearing indicates amplitude noise;
**tangential** smearing indicates **phase noise**; slow rotation indicates a
frequency offset; outer points pulled inward indicate **amplifier compression**;
elongation along one axis indicates I/Q imbalance. All look identical on a simple
signal-strength measurement.

**Error vector magnitude** *(§8.2)* — RMS distance between where symbols landed and
where they should have, as a percentage of the constellation scale. The single
summary number on transmitter datasheets.

**Software-defined radio** *(§8.2)* — Because modern radios generate *I* and *Q*
digitally and feed a scheme-agnostic quadrature modulator, changing modulation is a
software change. This is what makes millisecond-timescale rate adaptation possible
and what makes SDR possible at all.

**QAM** *(§8.3)* — Vary amplitude and phase together; points on a grid. 4-QAM
(= QPSK) through 4096-QAM, carrying 2 to 12 bits per symbol.

**The QAM SNR ladder** *(§8.3)* — Roughly **6 dB per doubling of constellation
size, 3 dB per additional bit per symbol**. Approximate requirements: BPSK 4 dB,
QPSK 7, 16-QAM 15, 64-QAM 21, 256-QAM 27, 1024-QAM 33, **4096-QAM 39 dB**. The
same arithmetic as Chapter 7 §7.4's PAM penalty, in two dimensions instead of one.

**Rate adaptation** *(§8.3)* — A radio measures its channel and selects the highest
modulation whose SNR requirement it currently meets, adjusting on a
millisecond timescale. The ladder spans a **12× range** in rate, which is why
signal strength alone predicts throughput poorly and SNR predicts it well.

**Gray coding** *(§8.3)* — Assigning bit patterns so adjacent constellation points
differ in exactly one bit, so a symbol error between neighbours produces one bit
error rather than several. Patented by Frank Gray in 1953 for mechanical shaft
encoders; used in every QAM constellation since.

**APSK and non-uniform constellations** *(§8.3)* — Points on concentric rings
rather than a grid, for lower peak-to-average ratio (DVB-S2 satellite, where the
amplifier's physics rather than the channel's shapes the choice); and deliberately
uneven spacing to approach the Shannon bound more closely (DVB-T2, ATSC 3.0).

**Peak-to-average power ratio** *(§8.3, §8.4)* — Constant-envelope schemes let an
amplifier run near saturation efficiently; varying-envelope schemes require
**backoff**, wasting power and generating heat. A serious constraint for
battery-powered transmitters.

**Delay spread** *(§8.4)* — The time between the earliest and latest significant
multipath copy. 50 ns in a small office, 100–300 ns in a large one, up to 20 µs in
hilly terrain. Compared against the symbol period, it determines how much
intersymbol interference multipath produces.

**OFDM** *(§8.4)* — Split the channel into many narrow subcarriers each running
slowly, so the symbol period is long compared with the delay spread. 802.11a uses
64 subcarriers at 312.5 kHz spacing with a 3.2 µs symbol, reducing a 100 ns delay
spread from 1,000% of a symbol period to 3%.

**Orthogonality in OFDM** *(§8.4)* — Subcarriers spaced at exactly the reciprocal
of the symbol period integrate to zero against one another, so their spectra
overlap while their information does not. Far more spectrally efficient than
guard-banded FDM.

**Cyclic prefix** *(§8.4)* — A copy of each symbol's tail prepended to its front and
discarded at the receiver, so that reflections land within it and orthogonality is
preserved. 800 ns for a 3.2 µs symbol in 802.11a — 20% overhead — with a **short
guard interval** of 400 ns trading multipath tolerance for about 11% more rate.

**OFDM as an inverse FFT** *(§8.4)* — An OFDM symbol is the inverse Fourier
transform of the subcarriers' constellation points, and the receiver recovers them
all with one forward FFT. This is why OFDM, patented in 1966, became practical only
when Cooley–Tukey-based DSP became cheap.

**OFDM's costs** *(§8.4)* — High peak-to-average ratio (10–13 dB), requiring
amplifier backoff — which is why LTE uses SC-FDMA on the uplink; and sensitivity to
frequency offset, which destroys orthogonality and produces inter-carrier
interference.

**Discrete multitone (DMT)** *(§8.4)* — OFDM in DSL. VDSL2 measures SNR on each of
up to 4,096 subcarriers individually and loads bits accordingly — Shannon's formula
applied hundreds of times per line, and the reason DSL rate falls smoothly with
loop length rather than in steps.

**OFDMA** *(§8.4)* — Allocating groups of subcarriers (**resource units**) to
different users simultaneously. Its gain is not raw throughput but efficiency with
small frames, since per-transmission overhead is paid once for many clients. Hence
Wi-Fi 6's advantage is largest in dense multi-client environments and negligible in
a single-client benchmark.
