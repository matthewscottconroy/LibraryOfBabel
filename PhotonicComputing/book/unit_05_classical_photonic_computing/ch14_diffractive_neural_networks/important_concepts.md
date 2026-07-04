# Chapter 14: Important Concepts

## The Diffractive Neuron and Free-Space Connectivity

**A diffractive neuron is a single pixel on a phase plate, and its "synapses" are free-space diffraction.** Each pixel imposes a local phase (and possibly amplitude) on the field passing through it; diffraction then spreads that pixel's contribution across the entire next layer. One layer of $N\times N$ pixels therefore realizes an all-to-all connection between two planes — a fully connected layer built from a single passive slab, with no wires and no per-connection hardware. In the Lin et al. (2018) demonstration each layer held $200\times200 = 4\times10^4$ neurons.

**Rayleigh–Sommerfeld / angular-spectrum propagation is a dense linear layer with a fixed kernel.** The propagation operator $H_d$ between layers is exactly the scalar diffraction integral of Fourier optics (Chapter 11); in the far field it becomes a Fourier transform. A diffractive layer is thus $U^{l+1} = H_d\,\mathrm{diag}(t^l)\,U^l$: a fixed, dense, off-diagonal connectivity ($H_d$) times a trained diagonal transmission ($\mathrm{diag}(t^l)$). Only the diagonal is learned; the connectivity is supplied for free by physics.

---

## Training a Passive Optical Network

**Weights are phases: the transmission is phase-only, $t^l = e^{i\phi^l}$.** A passive layer cannot amplify, so its degrees of freedom are the phase delays $\phi^l$ set by material height, $\phi = \frac{2\pi}{\lambda}(n-1)h$. This is a severe constraint — a phase-only mask is far less flexible than an arbitrary complex weight matrix — and it is why depth (more phase layers) buys expressivity even though the field map stays linear.

**Differentiable-diffraction training is offline, and its backward pass is a physical adjoint.** The network is simulated as a differentiable stack, trained by gradient descent on a computer, and the converged phases are then frozen into hardware. Because free-space propagation is linear and reciprocal, the backpropagation step corresponds to light propagating *backward* through the same diffraction operator — the diffractive analogue of the adjoint method of Chapter 13. The whole scheme inherits the sim-to-real gap of any offline-trained analog system.

---

## The Passivity Bargain

**Depth without inter-layer nonlinearity is still linear — the $|\cdot|^2$ readout is the sole nonlinearity.** A stack of $L$ phase-only layers composes to one complex-linear operator $M = H_d\,\mathrm{diag}(t^L)\cdots\mathrm{diag}(t^1)$; no nonlinearity acts until the detector applies $|\cdot|^2$ at the output. This single fact explains both why a D2NN needs no optical nonlinearity to classify (the magnitude-squared supplies the decision surface) and why its capacity is bounded (the interior is linear). Depth helps by realizing better linear maps under the phase-only constraint, not by stacking activations.

**Non-negativity is the readout's price, and differential detection pays it.** A detector measures $|U|^2 \ge 0$, so outputs cannot be signed and any intermediate detection would destroy phase. Class-specific differential detection (Li et al. 2019) assigns each class a positive and a negative detector region and scores it as $s_c = I_c^+ - I_c^-$, restoring the sign axis at the cost of doubling detectors — the diffractive echo of Chapter 12's balanced detection.

---

## Physical Limits

**The space-bandwidth product caps neuron count at $\sim (2A\,\mathrm{NA}/\lambda)^2$.** Diffraction forbids pixels finer than $\sim\!\lambda/(2\,\mathrm{NA})$, so a fixed aperture holds a finite number of independent neurons (Kulce et al. 2021). At 0.4 THz a 6 cm aperture with $\mathrm{NA}=0.4$ holds only $\sim\!4\times10^3$ neurons; the same aperture at 500 nm holds $\sim\!9\times10^9$ — capacity scales as $\lambda^{-2}$.

**The sim-to-real gap, misalignment, and the terahertz-vs-visible fabrication trade all scale with wavelength.** Tolerances measured in fractions of $\lambda$ translate to $75\ \mu$m at 0.4 THz but only $53$ nm at 532 nm for the same $\lambda/10$ budget. This $\sim\!1400\times$ ratio is why terahertz networks with millimeter features tolerate ordinary mechanical alignment while visible and metasurface designs demand nanometer control — the wavelength that buys neurons also buys difficulty.

---

## Hardware and Energy

**Reconfigurable DPUs trade static perfection for programmability.** Fixed 3D-printed layers are cheap, low-loss, and immutable — one network per slab. Spatial light modulators and the reconfigurable diffractive processing unit (Zhou et al. 2021) make the phases electronically programmable, enabling retraining and multitasking at the cost of modulator loss, pixel count, and drive power.

**Passive inference is nearly free; the training and the periphery are not.** The diffractive slab itself dissipates essentially zero power and computes in the sub-nanosecond transit time of light across the stack ($L\times d/c$). But the illumination source, the input encoder (e.g., an SLM), and the output detectors all draw power, and every gradient is computed on a digital machine. The honest energy ledger separates a near-free optical forward pass from a decidedly non-free electronic training and I/O.
