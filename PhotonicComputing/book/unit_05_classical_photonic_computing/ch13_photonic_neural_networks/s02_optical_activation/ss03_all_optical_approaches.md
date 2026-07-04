# Subsection 13.2.3: All-Optical Approaches

## Orientation

The all-optical activation — light nonlinearly transforming light, no electrons in the signal path — remains the aesthetic ideal of the field and a persistent research program. This subsection surveys the candidate mechanisms with their actual numbers, so that you can judge each against the requirements list of Subsection 13.2.1. The pattern to watch for: every mechanism offers a usable nonlinear *shape*; the failures are always in threshold power, recovery time, cascadability, or uniformity.

---

## 13.2.3.1 Saturable Absorption

A medium of absorbing two-level systems bleaches when the excitation rate outruns relaxation. The transmission of a thin saturable absorber follows

$$T(I) = \frac{T_{\text{lin}}}{1 + I/I_{\text{sat}}} \quad \text{(homogeneous line, thin sample)}$$

rising from $T_{\text{lin}}$ toward transparency as $I \gg I_{\text{sat}}$ — a smooth, monotonic, sigmoid-in-log-intensity curve that looks like a textbook activation. Implementations: semiconductor saturable absorber mirrors (SESAMs), quantum dots, carbon nanotubes, and graphene integrated on waveguides (graphene's $I_{\text{sat}}$ is high — MW/cm²-class — but its broadband response and easy integration keep it popular). This is exactly the nonlinearity Shen et al. 2017 *assumed* in simulation for their ONN.

The catch: an absorber attenuates by construction ($T \leq 1$ always), so cascaded layers still decay; $I_{\text{sat}}$ for fast (ps-recovery) absorbers corresponds to mW-class guided powers, $10^3\times$ above computing signal levels; and slow absorbers (μs recovery) confuse consecutive symbols. Saturable absorption is beloved in mode-locked lasers (Chapter 4), where one absorber serves a whole cavity; as a per-neuron device it has never been demonstrated at scale.

## 13.2.3.2 Cavity-Enhanced Kerr, Free-Carrier, and Thermal Nonlinearities

A resonator recirculates light, enhancing the internal intensity by $\sim \mathcal{F}/\pi$ and the effective nonlinearity accordingly. A ring or photonic-crystal cavity whose index shifts with stored energy exhibits a power-dependent detuning:

$$\delta\omega = -g\,|a|^2, \qquad \text{transmission } T(P_{\text{in}}) \text{ becomes S-shaped, then bistable for } P_{\text{in}} > P_{\text{th}}$$

Below the bistability threshold, the bent Lorentzian is a usable smooth activation; above it, the device is a latch (interesting for memory, hostile to gradient training). The record energy scale comes from photonic-crystal nanocavities: Nozaki et al. (2010) demonstrated all-optical switching in an InGaAsP nanocavity at $\approx$0.42 fJ internal energy with ps-scale response — proof that $Q/V$ engineering can reach computing-relevant energies *in a single device*. The unresolved issues are the trinity of Subsection 13.2.1: such cavities are attenuating (no gain), exquisitely sensitive to nanometer fabrication spread (uniform thresholds across thousands of neurons are unattained), and in silicon the fast Kerr response is contaminated by two-photon-generated free carriers and thermal shifts with ns–μs tails (the FOM problem of Chapter 11; silicon-specific).

## 13.2.3.3 Semiconductor Optical Amplifiers

The SOA is the one all-optical element that *supplies gain* while being nonlinear: its gain saturates with input power,

$$G(P_{\text{in}}) = \frac{G_0}{1 + P_{\text{out}}/P_{\text{sat}}}$$

giving a saturating activation with output powers self-limited near $P_{\text{sat}}$ (mW-class) and carrier lifetimes of 10–100 ps (multi-GHz capable). Cross-gain and cross-phase variants implement two-input nonlinear interactions. SOAs powered the optical logic era (Chapter 11) and the early photonic reservoir proposals, and they remain the default nonlinear node in fiber-based reservoir computers (Section 13.4). Their liabilities: electrical pump power of tens of mW *per device* continuously (a 1000-neuron layer burns tens of watts before computing anything), amplified spontaneous emission adding $\geq$3 dB noise figure per stage, and pattern-dependent distortion from finite carrier recovery.

## 13.2.3.4 Exotic and Emerging Media

- **Atomic vapors / EIT:** Zuo et al. (2019) built a two-layer all-optical network using electromagnetically induced transparency in laser-cooled Rb atoms as the activation — genuine photon-photon nonlinearity at ultralow light levels, in apparatus the size of an optical table. A superb existence proof; not a chip technology.
- **2D materials and heterostructures:** graphene/TMD saturable absorption and photorefractive-like effects integrated on Si/SiN waveguides; active research, mW thresholds typical.
- **Phase-change materials:** GST cells switched by the signal itself give thresholding with *nonvolatile memory* of past activity — used by Feldmann et al. (2019) to realize integrated all-optical spiking neurons whose PCM "membrane" both thresholds and stores. Energetics (pJ writes, μs–ms recrystallization) place this in the neuromorphic/plasticity domain of Unit VI rather than in GHz feedforward inference.
- **Quadratic ($\chi^{(2)}$) media:** thin-film lithium niobate offers parametric nonlinearities orders of magnitude stronger per photon than Kerr in silicon; pump-depleted second-harmonic stages have been proposed as activations, with thresholds still in the mW–W regime but falling as nanophotonic LN matures.

## 13.2.3.5 The Power-Budget Verdict, and the Architectural Dodge

Collect the thresholds: fast all-optical nonlinear devices operate at 0.1–10 mW signal powers (fJ–pJ per symbol at GHz rates *at best*, per neuron, plus pump power where gain is involved). A 1024-neuron layer of such devices consumes 0.1–10 W of *optical* power — before the lasers' wall-plug inefficiency (×10) and before cascading losses. Meanwhile the O-E-O stage of Subsection 13.2.2 delivers a programmable, uniform, gain-bearing activation for ~1 pJ from a mature technology. The all-optical activation is therefore not currently a competitive component; it is a research direction whose success condition is quantifiable: **fJ-class thresholds, ps recovery, dB-scale insertion loss, and thousandfold device uniformity, simultaneously.**

The field's most productive response has been architectural rather than device-level: place the network's nonlinearity where physics already provides it. The photodetector's $|E|^2$ is a free quadratic nonlinearity at readout (exploited by every incoherent architecture and by the diffractive networks of Chapter 14); the modulator's $\cos^2$ is free at input; and if the *internal* dynamics are kept linear but the readout is trained, one arrives at reservoir computing — the subject of the next section — where a fixed web of linear propagation plus a handful of cheap nonlinearities does the representational work that per-neuron activations do in a conventional deep network.

---

## References

[1] Nozaki, K., et al. (2010). "Sub-femtojoule all-optical switching using a photonic-crystal nanocavity." *Nature Photonics*, 4, 477–483. [The energy-scale benchmark for cavity-enhanced all-optical nonlinearity.]

[2] Zuo, Y., et al. (2019). "All-optical neural network with nonlinear activation functions." *Optica*, 6(9), 1132–1137. [EIT-based activations in cold atoms; the cleanest true all-optical multilayer demonstration.]

[3] Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569, 208–214. [PCM-based integrated all-optical neurons with thresholding and plasticity; the bridge to Unit VI.]

[4] Jha, A., Huang, C., & Prucnal, P.R. (2020). "Reconfigurable all-optical nonlinear activation functions for neuromorphic photonics." *Optics Letters*, 45(17), 4819–4822. [Cavity-loaded MZI activations demonstrating the shape-programmability achievable in the all-optical domain.]

[5] Miller, D.A.B. (2010). "Are optical transistors the logical next step?" *Nature Photonics*, 4, 3–5. [The requirements framework against which every device in this subsection should be scored.]
