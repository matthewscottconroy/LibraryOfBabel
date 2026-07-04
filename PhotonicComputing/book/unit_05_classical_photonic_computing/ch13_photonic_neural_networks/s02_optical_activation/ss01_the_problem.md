# Subsection 13.2.1: The Problem

## Orientation

Every photonic neural network paper contains, somewhere, a sentence acknowledging that the nonlinearity was done "electronically" or "in software." This subsection explains why that sentence keeps appearing. The difficulty is not engineering immaturity; it is a stack of physical requirements that pull against each other, and it deserves to be stated with the same care as the linear-algebra advantages of Chapter 12.

---

## 13.2.1.1 Linear Networks Collapse

Recall from Subsection 13.1.1: if the activation $f$ is linear, a depth-$L$ network collapses to a single effective matrix $W_{\text{eff}} = W^{(L)}\cdots W^{(1)}$. All the representational power of depth — the hierarchies of features that make deep learning work — comes from the interleaved nonlinearities. A photonic processor that only concatenates meshes has spent $L\times$ the hardware to implement one matrix it could have programmed directly.

The requirement is weaker than it first appears, though, and the weakening matters for architecture. The network needs *some* nonlinearity *somewhere*, but not necessarily an ideal ReLU at every hidden unit of every layer. Photodetection itself is quadratic in the field ($i \propto |E|^2$); a softmax readout is nonlinear; and entire model classes (kernel machines, extreme learning machines, reservoir computers — Section 13.4) need only one fixed nonlinear stage plus a trained linear map. Much of photonic architecture research is the art of positioning the few affordable nonlinearities where they buy the most expressivity.

## 13.2.1.2 The Requirements List

A device claiming to be an optical neuron's activation must simultaneously provide:

1. **A useful shape.** Saturating (sigmoid-like), rectifying (ReLU-like), or thresholding response with usable curvature at the operating power.
2. **Cascadability.** Output levels compatible with the next layer's input levels — which, after a lossy mesh ($-3$ to $-30$ dB, Section 12.2), means the activation stage generally must supply **gain**, or the signal decays geometrically with depth.
3. **Fan-out.** One neuron's output drives up to $N$ downstream inputs; the power budget multiplies accordingly.
4. **Speed.** Response and recovery faster than the symbol rate (GHz-class), else the neuron smears consecutive inputs together (which, uniquely, *reservoir* architectures turn into a feature — fading memory).
5. **Energy.** The point of photonics is fJ-scale operations; an activation costing 1 pJ–1 nJ per event at every one of $10^3$–$10^6$ neurons erases the linear-algebra savings.
6. **Reproducibility and stability.** Thousands of devices with matched transfer functions, stable against temperature and aging — an analog matching problem CMOS solved decades ago and photonics has not.
7. **Differentiability (for training).** A known, smooth, measurable $f$ whose derivative can be used in backpropagation (Subsection 13.1.2); bistable or hysteretic responses complicate gradient-based training profoundly.

## 13.2.1.3 Why Optics Fights Back

The physical obstruction is the weakness of optical nonlinearities at low power, quantified in Chapter 11 and worth restating in neural terms. A nonlinear phase shift via the Kerr effect accumulates as $\Delta\phi_{NL} = n_2 k_0 I L_{\text{eff}}$; for silicon-scale $n_2 \sim 5\times10^{-18}$ m²/W and mm-scale devices, order-unity nonlinearity requires intensities of GW/cm² — i.e., watts confined in a waveguide — while our computing signals carry microwatts. Resonators help by the finesse factor (circulating intensity $\sim \mathcal{F}/\pi \times$ input), photonic crystal cavities by $Q/V$, but demonstrated all-optical switching still sits at hundreds of femtojoules to picojoules *per event* with μs-scale thermal or carrier recovery tails in many platforms. Saturable absorbers respond at convenient powers only when made from strongly absorbing media, which then eat the very signal they process. There is no optical equivalent of the transistor's trick — a high-impedance control terminal that gates a large current with a few hundred electrons' worth of charge.

Two further problems are less advertised but equally structural. **Gain:** optics has amplifiers (SOAs, EDFAs, Chapter 4), but they add spontaneous-emission noise with noise figure $\geq 3$ dB, and cascading $L$ noisy nonlinear stages degrades SNR geometrically — the same accumulation that capped SOA logic chains in Chapter 11, now applied to analog values that are *more* sensitive than bits. **Uniformity:** a wafer of nominally identical microrings spreads its resonances over many linewidths; a wafer of nominally identical nonlinear activations spreads its thresholds correspondingly, and analog networks are far less forgiving of device-to-device spread than digital gates.

## 13.2.1.4 The Softer Verdict

Chapter 11's verdict on optical *logic* was terminal: CMOS wins by orders of magnitude and there is no path. The verdict on optical *activations* is softer, for three quantitative reasons:

1. **Count asymmetry.** A layer performs $N^2$ MACs but only $N$ activations. At $N = 10^3$, the activation may cost $1000\times$ more per operation than a MAC and still be a rounding error in the layer budget. This is the arithmetic that makes O-E-O conversion (next subsection) respectable rather than shameful.
2. **Analog tolerance.** An activation does not need the $10^{-15}$ error rates of logic; it needs a reproducible smooth curve. Devices far too noisy and slow-tailed for Boolean switching can be perfectly good sigmoids.
3. **Architectural escape routes.** Reservoir computing (Section 13.4) and single-nonlinear-layer models relocate the nonlinearity to where physics offers it cheaply — including into the detector itself.

The engineering question is therefore not "can optics make a nonlinearity?" (it can, expensively) but "on which side of the photodetector should the nonlinearity live?" The next two subsections examine the two answers.

---

## References

[1] Miller, D.A.B. (2010). "Are optical transistors the logical next step?" *Nature Photonics*, 4, 3–5. [The requirements-list methodology applied to optical switching; this subsection transposes it to analog activations.]

[2] Shastri, B.J., et al. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114. [Surveys the activation-function problem and the device candidates across the field.]

[3] Nozaki, K., et al. (2010). "Sub-femtojoule all-optical switching using a photonic-crystal nanocavity." *Nature Photonics*, 4, 477–483. [The record-scale demonstration defining what cavity enhancement can and cannot buy for low-power optical nonlinearity.]

[4] Zuo, Y., et al. (2019). "All-optical neural network with nonlinear activation functions." *Optica*, 6(9), 1132–1137. [An existence proof using electromagnetically induced transparency in cold atoms — scientifically fascinating, and a useful calibration of how far from chip-scale practicality true all-optical activations remain.]
