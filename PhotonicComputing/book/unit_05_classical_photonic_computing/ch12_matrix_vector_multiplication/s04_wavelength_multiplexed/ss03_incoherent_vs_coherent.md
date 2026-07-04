# Subsection 12.4.3: Incoherent vs. Coherent — A Structured Comparison

## Orientation

Chapter 12 has now developed two complete physical implementations of the same mathematical operation. The coherent MZI mesh multiplies complex field amplitudes and sums them by interference; the incoherent WDM weight bank multiplies optical powers and sums them by photodetection. Choosing between them — or combining them — is the first real architectural decision a photonic system designer faces. This subsection lays out the comparison systematically, then examines two landmark hybrid systems that used wavelength parallelism to push photonic computing past 10 TOPS: the phase-change-material tensor core of Feldmann et al. and the time-wavelength convolutional accelerator of Xu et al. (both *Nature*, 2021).

---

## 12.4.3.1 The Comparison

| Property | Coherent (MZI mesh) | Incoherent (ring weight bank) |
|---|---|---|
| Quantity encoding data | Complex field amplitude $A e^{i\phi}$ | Optical power $P \geq 0$ |
| Summation mechanism | Interference (field addition) | Photocurrent addition |
| Matrix class (native) | Unitary; arbitrary via SVD | Arbitrary real via differential pairs |
| Negative weights | Native (phase $\pi$) | Balanced-detector differential trick |
| Complex weights | Native | Not available |
| Phase stability required | Yes — small fraction of $\lambda$ across mesh | No |
| Laser requirement | Single coherent source (narrow linewidth) | $N$ wavelengths (DFB array or comb) |
| Footprint per weight | MZI: $\sim 10^4$ μm² | Ring: $\sim 10^2$ μm² |
| Demonstrated weight precision | $\sim$4–6 bits (7–8 with error correction) | $\sim$5 bits open-loop; $>$9 bits with feedback |
| Vector dimension $N$ per unit | 4–64 shown; loss/error grow with mesh depth | 4–32 per bus; bounded by FSR/crosstalk |
| Loss scaling | $\propto$ mesh depth ($N$ stages) | Splitter $1/N$ + fixed insertion loss |
| Natural strengths | Unitary transforms, quantum optics reuse, single laser | Robustness, recurrence, WDM ecosystem, precision |
| Natural weaknesses | Calibration burden, thermal crosstalk, depth loss | No phase/complex ops; channel count ceiling |

Two rows deserve emphasis.

**Precision.** It surprises many students that the "cruder" incoherent scheme holds the precision record ($>$9 bits, Zhang et al. 2022). The reason is structural: a ring weight is *one* device parameter monitored by *one* feedback loop, whereas an MZI-mesh matrix element is a global property of dozens of interfering paths — every phase shifter error in the mesh contributes to every matrix element. Coherence buys expressivity (complex, unitary matrices) at the price of error entanglement across the whole array.

**Loss scaling.** The mesh attenuates through $O(N)$ sequential stages ($\sim$0.2–0.5 dB each), so large coherent meshes pay $10$–$30$ dB; the broadcast bus pays a fundamental $1/N$ splitting share plus a *fixed* few-dB weight-bank loss. At large $N$ both lose the same $\sim\!10\log_{10}N$ dB asymptotically, but the mesh's loss is accompanied by coherent error accumulation while the bus's is a clean power division.

The deepest difference is what the photodetector means. In the coherent architecture, detection is the *readout* of a computation completed in the field. In the incoherent architecture, detection *is* the accumulation — which conveniently supplies a squaring nonlinearity ($P = |E|^2$) and unlimited fan-in, but forbids cascading linear operations optically: after one weighted sum, the signal is electronic. Multi-layer incoherent networks are therefore necessarily O/E/O per layer, while a coherent mesh can, in principle, cascade many linear stages in the optical domain before detecting once.

---

## 12.4.3.2 Case Study: The Photonic Tensor Core (Feldmann et al. 2021)

The Münster–Oxford–Exeter collaboration of Feldmann et al. combined three technologies into an incoherent matrix engine of striking elegance:

1. **Weights in phase-change material.** Cells of Ge$_2$Sb$_2$Te$_5$ (GST) deposited on waveguide crossings of an $N \times M$ crossbar array set the optical transmission of each crossing. GST is *non-volatile* (Chapter 11, Section 11.3.3): once written by optical pulses into a partially crystallized state, a weight holds indefinitely with **zero static power** — eliminating the heater-power tax that burdens both MZI meshes and ring banks. Multi-level programming gives $\sim$5 bits per cell.
2. **Inputs on a frequency comb.** A Kerr soliton microcomb (Chapter 8; a single Si$_3$N$_4$ microresonator emitting dozens of phase-locked carriers on a fixed grid) supplied the WDM channels, replacing racks of DFB lasers — the enabling economy for wavelength parallelism at scale.
3. **Parallel convolution by WDM.** Different wavelengths propagate through the *same* crossbar simultaneously and are demultiplexed onto separate detectors: one physical matrix multiplies many input vectors at once. The team ran convolutional kernels over images in this fully parallel mode, demonstrating aggregate processing at tera-MAC-per-second scale and performing on-hardware inference (digit recognition) with a convolutional front end.

The architecture's significance is the *combination*: in-memory computing (weights stored where they are used, no von Neumann traffic), zero static weight power, and wavelength-parallel throughput. Its constraints are equally instructive: GST write endurance and speed (10$^3$–10$^4$ cycles demonstrated in such devices, μs–ms programming) confine it to inference with rarely updated weights, and GST absorption costs several dB of optical budget.

## 12.4.3.3 Case Study: Time-Wavelength Interleaving (Xu et al. 2021)

Xu et al. (Swinburne/Monash/RMIT) reached $\approx$11 TOPS with almost no integrated matrix hardware at all. Their convolutional accelerator encodes the input vector as a serial *time* sequence on an electro-optic modulator, copies it across the ~90 lines of a soliton crystal microcomb, imposes the kernel weights spectrally (a programmable spectral shaper attenuating each comb line), and then uses **dispersive fiber delay** to shift each wavelength's copy in time by one symbol relative to the next. A single fast photodetector summing all wavelengths then outputs, at each instant, a complete dot product — the convolution emerges sample by sample from the wavelength-delay-sum structure. Handwritten-digit classification at $\approx$88% accuracy was demonstrated with the matrix operation running at tens of gigabaud.

The lesson: *parallelism can live in any physical dimension* — space (mesh, crossbar), wavelength (WDM), or time (delay interleaving) — and the highest demonstrated throughputs to date have come from spending the wavelength and time dimensions, which leverage the telecom industry's fastest components, rather than the space dimension, which requires large custom photonic circuits.

---

## 12.4.3.4 Design Guidance

A practical decision procedure for the working architect:

- Need complex-valued or unitary linear algebra (quantum photonics, coherent signal processing)? → **MZI mesh**, no substitute.
- Need maximum weight precision, recurrence, or processing of already-optical wideband signals? → **broadcast-and-weight**.
- Need inference-only throughput with weights fixed for millions of operations? → **PCM crossbar + comb**.
- Need raw TOPS on serial data with minimal photonic integration? → **time-wavelength interleaving**.
- Need training-speed weight updates? → none of the above yet; see Chapter 13, Section 13.3, for how the field copes.

---

## References

[1] Feldmann, J., Youngblood, N., Karpov, M., Gehring, H., Li, X., Stappers, M., Le Gallo, M., Fu, X., Lukashchuk, A., Raja, A.S., Liu, J., Wright, C.D., Sebastian, A., Kippenberg, T.J., Pernice, W.H.P., & Bhaskaran, H. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58. [PCM crossbar + soliton microcomb; the in-memory photonic computing landmark.]

[2] Xu, X., Tan, M., Corcoran, B., Wu, J., Boes, A., Nguyen, T.G., Chu, S.T., Little, B.E., Hicks, D.G., Morandotti, R., Mitchell, A., & Moss, D.J. (2021). "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51. [Time-wavelength interleaved convolution; the companion landmark published in the same issue.]

[3] Zhang, W., et al. (2022). "Silicon microring synapses enable photonic deep learning beyond 9-bit precision." *Optica*, 9(5), 579–584. [The precision comparison point cited in the table.]

[4] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Careful energy accounting for both coherent and incoherent MAC implementations; the quantitative backbone of this comparison.]
