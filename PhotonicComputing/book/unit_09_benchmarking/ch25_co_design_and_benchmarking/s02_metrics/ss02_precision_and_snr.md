# 25.2.2 Precision and SNR: The Effective Bits of Analog Optics

## Analog Computers Do Not Have a Bit Width

A digital multiplier is exact by construction: INT8 in, INT8×INT8 products out, every time. An analog optical multiplier has no such property. It has a signal, a noise floor, and a distortion budget, and its "precision" is a derived quantity: the **effective number of bits**,

$$\text{ENOB} = \frac{\text{SNDR}_{\text{dB}} - 1.76}{6.02}$$

inherited from converter metrology [1]: an ideal $b$-bit quantizer has SNR $= 6.02\,b + 1.76$ dB, so any measured signal-to-noise-and-distortion ratio maps to the bit width of the ideal quantizer that would match it. When a photonic accelerator is said to "compute at 6 bits," the checkable claim is that the analog chain — DAC, modulator, mesh, detector, TIA, ADC — delivers SNDR ≈ 38 dB at its digitized output. Every noise and error source in the chain spends from this budget.

## The Noise Inventory

| Source | Scaling | Typical ceiling it imposes |
|--------|---------|---------------------------|
| Shot noise | $\sigma_n = \sqrt{n}$ photons | sets photon budget $n \gtrsim 2^{2b}$ |
| Laser RIN | $\text{SNR} = 1/(\text{RIN} \cdot B)$ | 6–8 ENOB at 10 GHz for −140 to −150 dB/Hz |
| TIA thermal noise | $\sigma_I = i_n\sqrt{B}$ | sets minimum optical power (often > shot limit) |
| DAC quantization | by design | input precision floor |
| ADC quantization + jitter | 25.1.1 | ~7 ENOB at 10 GHz signals |
| Inter-channel crosstalk | −20 to −40 dB typical | 3–6 ENOB if uncorrected |
| Phase error & drift | accumulates with mesh depth | slow error floor; needs calibration |

The system ENOB is set by the root-sum-square of all of these; in practice, one or two dominate, and finding *which* is the first diagnostic task on any analog accelerator.

### Shot Noise and the Photon Budget

Photodetection is Poissonian: detecting $n$ photons per symbol window carries variance $n$. To resolve $2^b$ distinguishable output levels across a full scale of $n_{\max}$ photons, the level spacing $n_{\max}/2^b$ must exceed the noise $\sqrt{n_{\max}}$:

$$\frac{n_{\max}}{2^b} \gtrsim \sqrt{n_{\max}} \quad\Longrightarrow\quad \boxed{n_{\max} \gtrsim 2^{2b}\ \text{photons per output symbol}}$$

Each extra bit costs **4×** the light. At $\lambda = 1550$ nm ($E_{\text{ph}} = 0.128$ aJ):

| Target bits $b$ | Photons | Detected energy per output symbol |
|-----------------|---------|-----------------------------------|
| 4 | 256 | 33 aJ |
| 6 | 4,096 | 0.52 fJ |
| 8 | 65,536 | 8.4 fJ |
| 10 | 1.05 × 10⁶ | 134 fJ |

This is the fundamental energy-precision exchange rate of optical analog computing, and it explains the field's concentration at 4–8 bits: at 10+ bits the *optical* energy alone approaches digital MAC energies, before loss (×10 for 10 dB) and laser wall-plug efficiency (×10 for 10%) multiply it at the wall. It also underlies the deepest scaling argument in the field [2]: since one detected output symbol absorbs the light of $N$ MACs, the shot-limited optical energy *per MAC* is $2^{2b} E_{\text{ph}}/N$ — attojoules for large $N$ — which is why photonic energy claims improve with array size while digital energies do not.

### RIN and Thermal Noise: The Practical Floors

Laser relative intensity noise adds a signal-proportional term: integrating RIN over receiver bandwidth $B$ caps the SNR at $1/(\text{RIN} \cdot B)$. A respectable DFB with RIN = −150 dB/Hz read at $B = 10$ GHz gives SNR ≤ 10⁵ (50 dB) — an 8-ENOB ceiling; a noisier or comb-line source at −140 dB/Hz caps near 6.4 bits. Meanwhile the TIA's input-referred noise (0.2–2 μA RMS over 10 GHz, Section 25.1.1) usually demands tens of microwatts of signal power — comfortably *above* the shot-limited budget in the table. Most direct-detection accelerators are therefore thermal/RIN-limited, not shot-limited; coherent (homodyne) readout, which amplifies the signal against a strong local oscillator, is the standard route back toward the shot limit [2].

### Error Accumulation in Large Meshes

Static component errors — splitter imbalance, phase-setting error $\sigma_\phi$, thermal crosstalk between heaters — act as a frozen noise on the *matrix itself*. In a Clements-style mesh, light traverses $O(N)$ interferometer stages, and uncorrected phase errors compound with depth: matrix fidelity degrades systematically as meshes grow, which is the precision analog of the loss-scaling problem. Self-configuration and hardware-aware error correction can recalibrate much of this away [4, 5], at the cost of monitor photodiodes, calibration time, and control complexity that belong in the system budget (Section 25.1.2). Drift then makes calibration a *recurring* cost: silicon's thermo-optic sensitivity moves phase by parts in 10³ per millikelvin-scale gradients, so a mesh trimmed this morning is a slightly different matrix by afternoon.

## From ENOB to Task Accuracy

Hardware bits matter only through their effect on task metrics. The transfer is nontrivial in both directions:

- **Neural networks are forgiving — to a point.** A decade of quantization practice shows most CNN/transformer inference survives 8-bit weights and activations essentially unharmed, and 4-bit with quantization-aware training; below that, accuracy falls off a cliff that is model- and task-dependent. Analog noise behaves differently from deterministic rounding: zero-mean per-pass noise partially averages out (and can even regularize), while *correlated, drifting* error — the signature failure mode of analog photonics — does not.
- **Noise-aware training buys back several bits' worth of accuracy.** Injecting the measured hardware noise model into training, or fine-tuning with the hardware in the loop, routinely recovers a large fraction of the digital baseline; it is now standard methodology rather than an optional trick [3, 6].
- **The canonical cautionary datum** remains the first coherent-nanophotonic deep-learning demonstration [3]: a vowel-classification task at 76.7% correct on the photonic hardware versus 91.7% for the same network in floating point — a gap attributed to phase-encoding error and photodetection noise. The experiment was a landmark *because* it reported both numbers; much later work reports only one.

Hence the reporting rule this chapter will enforce in 25.2.3: **an analog accelerator result is incomplete unless it states task accuracy on hardware alongside the digital baseline of the identical model** — the iso-accuracy discipline that MLPerf institutionalizes for electronic systems.

## Precision Co-Design Rules

Three rules close the loop with Section 25.1:

1. **Match the chain.** ADC bits beyond the analog ENOB waste exponential energy; DAC bits beyond it waste linearly. Measure the optical ENOB first, then buy converters to match — not the reverse.
2. **Accumulate digitally.** Summing $K$ analog partial results grows noise as $\sqrt{K}$; digital accumulation is exact. Tile large matrices so that each analog pass stays within budget and the reduction happens in electronics (the 25.1.2 partition).
3. **Spend photons where bits are needed.** Layers differ in sensitivity; mixed-precision mapping — more optical power or repeated passes for sensitive layers, fewer photons elsewhere — converts the $2^{2b}$ law from a tax into a knob.

---

## References

[1] Walden, R.H. (1999). "Analog-to-digital converter survey and analysis." *IEEE Journal on Selected Areas in Communications*, 17(4), 539–550. [Source of the ENOB/SNR formalism applied here to full analog chains.]

[2] Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9(2), 021032. [Standard-quantum-limit analysis of optical neural networks; the photon-budget-per-MAC scaling and the case for coherent readout.]

[3] Shen, Y., Harris, N.C., Skirlo, S., Prabhu, M., Baehr-Jones, T., Hochberg, M., Sun, X., Zhao, S., Larochelle, H., Englund, D., & Soljačić, M. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11(7), 441–446. [The 56-MZI programmable mesh demonstration, including the honest hardware-versus-simulation accuracy comparison quoted here.]

[4] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [Quantifies component-error accumulation in MZI meshes and demonstrates correction strategies.]

[5] Bogaerts, W., Pérez, D., Capmany, J., Miller, D.A.B., Poon, J., Englund, D., Morichetti, F., & Melloni, A. (2020). "Programmable photonic circuits." *Nature*, 586, 207–216. [Calibration, monitoring, and self-configuration of large programmable meshes.]

[6] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114. [Review of noise, precision, and training strategies for analog photonic AI hardware.]
