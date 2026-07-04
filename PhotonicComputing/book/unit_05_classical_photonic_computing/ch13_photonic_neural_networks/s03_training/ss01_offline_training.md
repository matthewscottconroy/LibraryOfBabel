# Subsection 13.3.1: Offline Training with Hardware Simulation

## Orientation

The default way to train a photonic neural network is to not involve the photonics at all: build a differentiable software model of the hardware, train it with standard tools (PyTorch, JAX, Adam), and transfer the resulting parameters to the chip. Every landmark demonstration in this unit — Shen 2017, Lin 2018, Feldmann 2021 — was trained this way. This subsection examines the method's mechanics, its characteristic failure (the sim-to-real gap), and the modeling and calibration discipline that narrows the gap.

---

## 13.3.1.1 The Digital Twin

The training model should mirror the physics, layer by layer:

- **Linear stages** as products of MZI transfer matrices with the actual mesh topology (not as free matrices $W$ — training the phases $\{\theta_k, \phi_k\}$ directly guarantees hardware realizability, per Subsection 12.3.1). Software frameworks built for exactly this purpose — `neurophox` and `neuroptica` for mesh networks — provide mesh layers whose parameters are the physical phases, with autograd support.
- **Nonlinear stages** as the measured device transfer function (the $\cos^2$ modulator sigmoid, a fitted saturable-absorption curve), not an idealized ReLU.
- **Readout** as $|E|^2$ detection with the correct signed-weight or differential arrangement.
- **Constraints and quantization**: phases wrapped to $[0, 2\pi)$, weights clipped to realizable ranges, DAC resolution imposed via quantization-aware training (straight-through gradient estimators).

Training then proceeds exactly as in Subsection 13.1.2, with the chain rule running through the physical parameterization. The gradient with respect to a phase, $\partial\mathcal{L}/\partial\theta_k$, is automatic: each MZI matrix is an analytic function of its phases.

## 13.3.1.2 Transfer and the Sim-to-Real Gap

Deployment maps trained parameters to hardware settings through each device's calibration curve (heater power ↔ phase, Section 12.2.4). The deployed network then computes not $f_{\text{model}}(\mathbf{x}; \boldsymbol{\theta})$ but $f_{\text{hw}}(\mathbf{x}; \boldsymbol{\theta} + \boldsymbol{\epsilon}_{\text{static}}, t)$, where the discrepancy has a structure worth taxonomizing:

| Error class | Examples | Statistical character |
|---|---|---|
| Static, per-device | coupler splitting error, calibration offset, unmodeled crosstalk | fixed bias, device-correlated |
| Slow drift | ambient temperature, laser wavelength, aging | correlated over minutes–days |
| Fast stochastic | shot noise, TIA noise, laser RIN | zero-mean, per-inference |
| Quantization | DAC/ADC resolution | deterministic, signed |

The observed symptom is uniform across the literature: simulation accuracy is high, hardware accuracy is meaningfully lower — Shen et al.'s 91.7% → 76.7% is the canonical instance. Robotics named this the **sim-to-real gap**, and the photonic community has converged on the same three-part response: better twins (model the errors), better hardware mapping (calibrate the errors away), and robust training (make the network indifferent to the errors — Subsection 13.3.3).

## 13.3.1.3 Closing the Gap from the Software Side

**Characterize, then model.** The gap shrinks roughly in proportion to how much of the static error is moved from "unknown" to "modeled." Measured coupler ratios and measured activation curves can be inserted per device into the twin; retraining with the *measured* model then customizes the weights to the individual chip. This chip-specific retraining is effective but breaks the one-model-many-chips manufacturing economics — an underappreciated tension between analog computing and mass production.

**Map with error correction.** Rather than retraining, keep the ideal weights and improve the mapping: the hardware error-correction algorithms of Bandyopadhyay et al. (2021) adjust mesh phases to compensate measured coupler errors (recovering, in their demonstrations, roughly a 10× reduction of matrix error without touching the training), and Miller-style self-configuration (Section 12.2.4) sidesteps explicit models entirely for the matrix stage.

**Impose hardware limits during training.** Quantization-aware training at the DAC's bit depth, activation clipping at the modulator's range, and loss regularizers penalizing extreme singular values (which, per Subsection 12.3.1, cost optical dynamic range) all produce networks that arrive on chip already living within its means.

## 13.3.1.4 What Offline Training Cannot Fix

Two limits are structural. First, *unmodeled* static error — anything the characterization missed — lands directly on accuracy, and characterizing every parasitic coupling in a 10⁴-element mesh is its own measurement campaign. Second, drift: a twin frozen at calibration time decays in fidelity as the chip's environment evolves, demanding either periodic recalibration (downtime) or active stabilization (power). These limits motivate the rest of Section 13.3: methods where the physical hardware, with all its unmodeled truth, sits inside the optimization loop.

Offline training nevertheless remains the right default. It uses mature tools at GPU speed, requires zero hardware time per training step, and — combined with error-corrected mapping and noise-aware training — has delivered hardware accuracies within 1–3 points of simulation on MNIST-class tasks. The methods of the following subsections should be understood as increasingly expensive insurance against its residual gap.

---

## References

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The archetypal offline-trained demonstration and the canonical sim-to-real gap datapoint.]

[2] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [Correcting the mapping instead of retraining the model; the quantitative backbone of Subsection 13.3.1.3.]

[3] Pai, S., Bartlett, B., Solgaard, O., & Miller, D.A.B. (2019). "Matrix optimization on universal unitary photonic devices." *Physical Review Applied*, 11, 064044. [Gradient-based optimization directly in the mesh-phase parameterization; the theoretical basis for training the physical parameters rather than abstract weights.]

[4] Jacob, B., et al. (2018). "Quantization and training of neural networks for efficient integer-arithmetic-only inference." *Proceedings of CVPR*. [Quantization-aware training — developed for digital INT8, adopted wholesale for analog photonic precision limits.]
