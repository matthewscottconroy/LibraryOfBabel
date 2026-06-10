# Section 9.2: Modulation Formats

The Shannon-Hartley theorem tells us the capacity of a channel; modulation formats determine how closely we approach that capacity in practice. A modulation format is a mapping from information bits to physical waveform parameters — amplitude, phase, frequency, polarization — and back. The choice of format determines the SNR required to achieve a given bit rate, the spectral efficiency, and the complexity of the transmitter and receiver.

The history of optical modulation formats is a history of increasing sophistication, driven by the demand for higher capacity without increasing the number of fibers or amplifiers:

1. **OOK (on-off keying)**: Bit 1 = light on, bit 0 = light off. Simple, direct detection, 1 bit per symbol.
2. **PAM4 (4-level pulse amplitude modulation)**: Four amplitude levels, direct detection, 2 bits per symbol. Current standard for 400G short-reach.
3. **BPSK/QPSK (binary/quadrature phase-shift keying)**: Phase encodes bits, coherent detection required, 1–2 bits per symbol. First generation coherent.
4. **DP-16QAM (dual-polarization 16-QAM)**: 16 amplitude-phase states × 2 polarizations = 8 bits per symbol. Current standard for 400G+ long-haul.

This section develops each format's noise tolerance, spectral efficiency, and SNR requirement, building toward the comparison table in Section 9.2.3 that shows where each format operates relative to the Shannon limit.
