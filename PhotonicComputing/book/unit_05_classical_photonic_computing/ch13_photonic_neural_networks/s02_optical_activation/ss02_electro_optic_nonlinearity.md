# Subsection 13.2.2: Electro-Optic Nonlinear Activation

## Orientation

If the nonlinearity will not come to the photons, bring the photons to the electrons. The electro-optic (O-E-O) activation detects the optical signal, applies the nonlinear function in the electrical domain — implicitly or explicitly — and re-modulates the result onto light for the next layer. Purists call this cheating; engineers call it the only approach that has produced complete, working, multi-layer photonic neural networks. This subsection develops the device physics, the clever variants that keep the electronics analog and local, and the energy-latency accounting that determines when O-E-O is acceptable.

---

## 13.2.2.1 The Basic Stage

The canonical O-E-O activation chain per neuron:

$$\text{PD} \;\rightarrow\; \text{TIA} \;\rightarrow\; \text{[nonlinear circuit / digital LUT]} \;\rightarrow\; \text{driver} \;\rightarrow\; \text{modulator}$$

A germanium photodetector converts the layer's optical output to photocurrent ($\mathcal{R} \approx 1$ A/W, bandwidth > 50 GHz); a transimpedance amplifier converts current to voltage; the voltage — optionally digitized, transformed, and regenerated — drives a modulator carrying the next layer's light. The stage inherently provides the three things all-optical devices struggle with: **gain** (electrical amplification is cheap), **fan-out** (a voltage drives any number of modulator gates), and **level restoration** (each layer restarts from a fresh laser carrier).

**The free sigmoid.** The nonlinear function need not be computed at all — it can be inherited from the modulator's transfer curve. A Mach-Zehnder modulator driven by voltage $v$ transmits

$$T(v) = \cos^2\!\left(\frac{\pi v}{2V_\pi} + \phi_{\text{bias}}\right)$$

Biased at quadrature, this is a smooth saturating sigmoid in the photocurrent; biased near a null, it approximates a rectifier. A microring modulator gives a sharper, Lorentzian-flanked response. Since the photocurrent is itself $\propto |E|^2$, the composite response of PD + MZM includes a quadratic front — the effective activation is $f(z) \propto \cos^2(\alpha |z|^2 + \phi_b)$-like, and its parameters ($\phi_b$, gain $\alpha$) are *electrically programmable*. Williamson et al. (2020) analyzed exactly this family for MZI-mesh networks, showing that tapping a small fraction of the optical signal, detecting it, and using the result to drive a phase shifter acting on the *remaining* light yields a reconfigurable ReLU-to-sigmoid family with the transfer function applied directly in the optical domain — an "electro-optic nonlinearity" in which the signal never fully leaves the light.

## 13.2.2.2 Energy and Latency Accounting

Per activation event at GHz rates, representative figures:

| Element | Energy | Latency |
|---|---|---|
| Photodetection (device) | — (converts signal) | < 10 ps |
| TIA | 0.1–1 pJ | 10–50 ps |
| Analog nonlinear shaping | 0.05–0.5 pJ | 10–50 ps |
| (Optional) ADC + LUT + DAC | 1–4 pJ | 100–500 ps |
| Modulator + driver | 0.1–1 pJ | 10–50 ps |
| **Analog O-E-O total** | **≈ 0.3–2 pJ** | **≈ 50–150 ps** |
| **Digitizing O-E-O total** | **≈ 2–6 pJ** | **≈ 0.2–1 ns** |

Set against the layer arithmetic: at $N = 100$, a layer performs $10^4$ MACs and $10^2$ activations, so a 1 pJ activation adds 0.01 pJ/MAC — subdominant to the laser and detection budget. At $N = 10$, the same activation adds 1 pJ/MAC and dominates everything. **O-E-O is amortizable exactly when the linear layers are large** — the same large-$N$ condition already required by the conversion-overhead argument of Section 12.3.3. Latency-wise, 100 ps per layer keeps a 10-layer network under 2 ns end-to-end, which remains spectacular by electronic standards.

The strategic choice is analog vs. digitizing O-E-O. Staying analog (no ADC/DAC in the loop) saves most of the energy and latency but accumulates analog noise layer over layer and fixes the function family in hardware. Digitizing restores the signal perfectly each layer — analog error stops compounding — at the price that the "photonic" network is now optically linear + electronically everything-else. Real systems (including the commercial architectures of Section 12.3.3) mostly digitize, precisely to contain error accumulation.

## 13.2.2.3 Demonstration: An End-to-End On-Chip Classifier

The cleanest published existence proof of the full O-E-O concept is the Penn silicon photonic classifier of Ashtiani, Geers, and Aflatouni (2022): a single chip integrating the photodetectors, nonlinear electro-optic neurons, and optical linear layers of a small feedforward network, classifying low-resolution images end-to-end with the image data entering optically and the class decision emerging in **under 570 ps** — a latency no electronic classifier of any architecture approaches. Accuracy on the two- and four-class letter-image tasks was ~90%+, modest in ML terms but beside the point: the demonstration closed the loop that Shen et al. 2017 left open (whose nonlinearity lived in a host computer), proving that multi-layer photonic inference with on-chip activations is an integration problem, not an open physics problem.

At Princeton, the modulator-neuron variant powers the broadcast-and-weight systems of Section 12.4.2 — Tait et al.'s silicon modulator neuron (2019) established the PD-drives-modulator cell as a cascadable analog neuron, and Huang et al.'s fiber-nonlinearity compensator (2021) ran it at communication rates on a real task. MIT's follow-on single-chip deep network (Bandyopadhyay et al., 2024) integrated coherent matrix units with electro-optic activations and in-situ-trainable weights on one silicon die, reporting ~92% accuracy on vowel classification fully on chip.

## 13.2.2.4 Assessment

The O-E-O activation resolves — bluntly but completely — every item on Subsection 13.2.1's requirements list: programmable shape, gain, fan-out, GHz speed, pJ energy, CMOS reproducibility, and an exactly known $f$ and $f'$ for training. Its costs are the pJ-scale energy floor per neuron, the latency of the electronic island, and a system now containing per-neuron analog electronics whose design effort rivals the photonics. It is the current answer, and — given the count asymmetry between MACs and activations — likely to remain the answer at large $N$ unless the all-optical devices of the next subsection improve by roughly two orders of magnitude in threshold power.

---

## References

[1] Williamson, I.A.D., Hughes, T.W., Minkov, M., Bartlett, B., Pai, S., & Fan, S. (2020). "Reprogrammable electro-optic nonlinear activation functions for optical neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7700412. [The programmable tap-detect-phase-shift activation family and its use in mesh networks.]

[2] Ashtiani, F., Geers, A.J., & Aflatouni, F. (2022). "An on-chip photonic deep neural network for image classification." *Nature*, 606, 501–506. [End-to-end sub-nanosecond on-chip inference with electro-optic neurons — the flagship O-E-O demonstration.]

[3] Tait, A.N., et al. (2019). "Silicon photonic modulator neuron." *Physical Review Applied*, 11, 064043. [The PD-drives-modulator cell as a cascadable analog neuron in broadcast-and-weight networks.]

[4] Bandyopadhyay, S., et al. (2024). "Single-chip photonic deep neural network with forward-only training." *Nature Photonics*, 18. [Fully integrated coherent network with on-chip activations and in-situ training; the state of the art in monolithic photonic deep learning.]

[5] Huang, C., et al. (2021). "A silicon photonic–electronic neural network for fibre nonlinearity compensation." *Nature Electronics*, 4, 837–844. [Modulator neurons at line rate on a real signal-processing task.]
