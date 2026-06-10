# 9.4.3 FEC in Photonic Computing Systems

## Overhead and Its System Implications

Every photonic computing system that interfaces with a digital network must handle FEC overhead. A 400G optical channel using 20% SD-FEC overhead transmits 400/0.8 = 500 Gbps gross, of which 100 Gbps is FEC redundancy. The modulator must support 500 Gbps (not 400 Gbps), and the DAC/ADC running the coherent modulation and detection runs at the corresponding symbol rate.

For a photonic matrix multiplier receiving inputs from a 400G WDM channel:
- Gross input rate: 500 Gbps
- Information rate: 400 Gbps
- FEC decoding required before data enters the photonic matrix
- FEC encoding required after the photonic matrix outputs

The latency of FEC decoding (typically 1–100 μs for block lengths of $10^4$–$10^5$ bits) dominates the latency of photonic matrix multiplication (picoseconds). For latency-sensitive applications, FEC adds orders of magnitude of delay.

## FEC Decoding as a Photonic Computing Application

Large-scale LDPC decoding is computationally intensive: a modern 400ZR coherent transceiver uses ~3 W just for the FEC decoder ASIC. The decoder processes $10^9$ messages per second in a belief propagation graph with $O(N^2)$ edge updates.

The matrix structure of belief propagation has been proposed as a potential application for photonic matrix-vector multiplication [1]. In this proposal, the messages passed in one iteration of BP are represented as a vector, and the update rule $\Delta \propto H \cdot \text{tanh}(L/2)$ (simplified) is implemented as a matrix-vector product in the optical domain.

The challenge: the tanh nonlinearity is required. For pure linear photonic processing, only the linear part of the update rule can be optically implemented; the nonlinearity must be implemented electronically, requiring D/A and A/D conversion at each step. At 50 iterations per codeword and 400 Gbps throughput, this is ~$2 \times 10^{13}$ floating-point-equivalent operations per second — requiring ADC sampling rates of terahertz, which is not feasible with current technology.

The honest assessment: photonic FEC decoding is not likely to be competitive with CMOS ASICs in the near term, given the nonlinearity requirement and the extreme maturity of CMOS LDPC decoder implementations (~3 W for 400 Gbps in 5 nm CMOS). Photonic systems are better suited to the *forward path* computation (matrix-vector multiply in the neural network inference case) rather than the error correction surrounding it.

## The Precision-FEC Tradeoff in Analog Computing

An important system-level consideration: photonic analog computing operates at limited precision (ENOB ~5–7 bits, as analyzed in Chapter 5). For applications where the computation must produce digital outputs — integer decisions, binary classifications — the precision limitation can be partially compensated by FEC at the output.

This is analogous to how soft-decision FEC exploits graded analog confidence information: if the photonic computer produces an analog output that is "almost right" rather than "completely right," a downstream error-correction step can recover the exact answer more often than a pure hard-decision threshold. The theory of "computational FEC" for analog computing systems is an active research area [2].

---

## References

[1] Argyris, A., Bueno, J., & Fischer, I. (2018). "Photonic machine learning implementation for signal recovery in optical communications." *Scientific Reports*, 8(1), 8487. [Photonic implementation of signal processing in optical communication systems.]

[2] Hamerly, R., Bandyopadhyay, S., Carolan, J., Englund, D., & Mabuchi, H. (2022). "Asymptotic advantages of loop-based boson sampling." *npj Quantum Information*, 8(1), 1–10. [Analysis of precision and error in analog optical computing systems.]
