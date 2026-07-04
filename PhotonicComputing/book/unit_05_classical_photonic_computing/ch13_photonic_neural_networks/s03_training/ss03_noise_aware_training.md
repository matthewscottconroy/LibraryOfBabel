# Subsection 13.3.3: Noise-Aware Training

## Orientation

Noise-aware training keeps the offline digital-twin workflow of Subsection 13.3.1 but corrupts the forward pass on purpose: sample the hardware's stochastic imperfections — phase jitter, coupler-splitting scatter, detector shot noise, DAC quantization — and inject them into every training iteration. The optimizer, now forced to minimize loss *under* perturbation, is pushed off sharp minima toward weights sitting in a **flat basin** where small hardware deviations barely move the output. The method is cheap — it costs only extra sampling in software, no hardware time — and effective, but only against one of the two error classes it must confront.

---

## 13.3.3.1 Two Error Classes, and Which One This Fixes

The taxonomy of Subsection 13.3.1.2 collapses, for the purposes of robust training, into a single binary: is the error zero-mean across inferences, or is it a fixed bias?

| | Zero-mean stochastic | Systematic / static |
|---|---|---|
| Examples | phase noise $\sigma_{\text{phase}}$, shot noise, TIA/RIN noise, DAC dither | coupler-splitting offset, calibration bias, unmodeled crosstalk, fixed drift |
| Behavior over runs | averages out; re-sampled each inference | identical every inference |
| Noise-aware training | **helps** — trains for the *distribution* the hardware draws from | **does not help** — the wrong-but-fixed operator is simply learned around in software, then reappears on chip |

The distinction is the organizing idea of the subsection. Injecting a zero-mean random variable during training teaches the network to be indifferent to a class of perturbations it will genuinely re-draw at inference; the expected training loss is exactly the expected deployment loss, so the optimizer is solving the right problem. A *static* miscalibration, by contrast, is not a distribution — it is a single displaced operator. Adding zero-mean noise on top of it does nothing to the bias; the residual static error survives to hardware untouched. Systematic error is the province of calibration and error-corrected mapping (Subsection 13.3.1.3) or of putting the real hardware in the loop (Subsection 13.3.4). Noise-aware training is a complement to those methods, never a substitute.

## 13.3.3.2 Why Injecting Noise Regularizes Sensitivity

The mechanism is the classical result that training with noise is a form of regularization (Bishop; see also Goodfellow et al. 2016, §7.5). Let the deployed parameters be $\boldsymbol{\theta} + \boldsymbol{\epsilon}$ with $\boldsymbol{\epsilon}$ zero-mean and covariance $\sigma^2 \mathbf{I}$. Expanding the loss to second order and averaging over the noise,

$$\mathbb{E}_{\boldsymbol{\epsilon}}\!\left[\mathcal{L}(\boldsymbol{\theta} + \boldsymbol{\epsilon})\right] \;\approx\; \mathcal{L}(\boldsymbol{\theta}) \;+\; \tfrac{1}{2}\,\sigma^2\,\mathrm{Tr}\!\left(\mathbf{H}\right), \qquad \mathbf{H} = \nabla^2_{\boldsymbol{\theta}}\mathcal{L}.$$

The first-order term vanishes because $\boldsymbol{\epsilon}$ is zero-mean; what remains is a penalty proportional to the **trace of the loss Hessian** — the total curvature at the operating point. Minimizing the noisy objective therefore minimizes clean loss *and* curvature simultaneously, driving the optimizer toward wide, flat minima whose output is insensitive to parameter perturbation. This is the same "flat-minimum" preference that makes noise beneficial in digital deep learning, repurposed here for a concrete physical reason: in analog optics the perturbations are not a metaphor for generalization, they are literally the phase and amplitude errors the chip will exhibit.

## 13.3.3.3 Worked Example: Error Accumulation in a 64×64 Clements Mesh

Take a single unitary layer implemented as a $64 \times 64$ Clements mesh (Subsection 12.3.1), which uses $K = N(N-1)/2 = 64\cdot 63/2 = 2016$ programmable phase shifters. Suppose each phase carries an independent zero-mean Gaussian jitter $\sigma_{\text{phase}} = 0.02$ rad ($\approx 1.1^\circ$), a realistic figure for combined thermal drift and DAC quantization on a thermo-optic mesh.

Using the mesh error-propagation scaling for the aggregate perturbation of the realized matrix,

$$\text{relative output error} \;\approx\; \sqrt{K}\,\sigma_{\text{phase}} \;=\; \sqrt{2016}\times 0.02 \;\approx\; 44.9 \times 0.02 \;\approx\; 0.90 \text{ rad-equivalent.}$$

The per-phase error is a modest 2%, yet accumulated (in the random-walk sense, variance adding as $K\sigma^2$) across two thousand degrees of freedom it reaches order unity — the matrix as a whole is substantially derandomized. Classification is more forgiving than that raw figure suggests, because decisions are taken by $\arg\max$ over a trained, redundant readout rather than by exact vector values; empirically, perturbations of this scale cost roughly **2–3 percentage points** of MNIST accuracy for a network trained *without* noise. A network trained *with* the same $\sigma_{\text{phase}}$ injected each iteration recovers most of that gap, because its weights already occupy the flat basin the perturbation cannot escape.

The same discipline hardens the network against readout noise. For shot-noise-limited detection the signal-to-noise ratio is set by the mean detected photon count $\bar n$, with $\mathrm{SNR}_{\text{dB}} = 10\log_{10}\bar n$ and effective resolution $\mathrm{ENOB} = (\mathrm{SNR}_{\text{dB}} - 1.76)/6.02$. Targeting a robust 6–8 ENOB requires only $\bar n \approx 6\times 10^{3}$ to $10^{5}$ detected photons per symbol — a few fJ to $\sim$10 fJ at $h\nu(1550\,\text{nm}) = 1.28\times 10^{-19}$ J. Training with a matched Poisson/Gaussian readout-noise model at these photon budgets lets the network tolerate the low-light, low-energy operating point that makes photonic MACs attractive in the first place, rather than demanding the mW-scale power a noise-naive network would need to reach the same accuracy.

## 13.3.3.4 The Robustness–Accuracy Trade-off

Noise-aware training is not free of cost in accuracy. Flattening the loss landscape sacrifices the sharp minima that would yield the highest score on *clean, perfectly calibrated* hardware; a network trained at injected $\sigma$ underperforms a noise-naive network when evaluated on a (hypothetical) noiseless chip, and overperforms it as soon as real noise appears. The correct injection level is therefore a hyperparameter matched to the deployment hardware: too little and the flat basin is not found, too much and the network is regularized into mediocrity for perturbations it will never see. In practice one sweeps $\sigma$ around the measured hardware value and selects the setting that maximizes *deployed* accuracy — the only figure of merit that matters. Combined with the error-corrected mapping of Subsection 13.3.1.3 for the static component, noise-aware training is what closes the last one to two points of the sim-to-real gap on MNIST-class benchmarks without ever powering on the chip during training.

---

## References

[1] Bandyopadhyay, S., et al. (2024). "Single-chip photonic deep neural network with forward-only training." *Nature Photonics*, 18. [Reports accuracy under realistic on-chip noise and the robustness gains of training that accounts for it; the modern datapoint for this subsection.]

[2] Wright, L.G., Onodera, T., Stein, M.M., Wang, T., Schachter, D.T., Hu, Z., & McMahon, P.L. (2022). "Deep physical neural networks trained with backpropagation." *Nature*, 601, 549–555. [Establishes that modeling the physical system's stochasticity during training is what makes transfer to hardware succeed; the conceptual bridge to Subsection 13.3.4.]

[3] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The empirical observation that accumulated mesh phase error, not the algorithm, sets deployed accuracy — the noise-tolerance problem this subsection addresses.]

[4] Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. [§7.5: adding noise to weights is equivalent to a curvature penalty favoring flat minima — the regularization mechanism of Subsection 13.3.3.2.]
