# Chapter 14: Exercises

## Mathematical Exercises

**14.1** (Rayleigh–Sommerfeld kernel and the far-field Fourier transform) The forward model of Section 14.1 propagates a field between layers with the operator $H_d$.

(a) Starting from the Huygens–Fresnel principle — each aperture point $(x',y')$ re-radiates a spherical wavelet $\propto e^{ikr}/r$ — write the superposition integral for the field $U(x,y)$ a distance $z$ downstream, and identify the obliquity factor $\cos\theta = z/r$ and the $1/(i\lambda)$ prefactor of the first Rayleigh–Sommerfeld solution.

(b) Apply the paraxial expansion $r \approx z + \frac{(x-x')^2 + (y-y')^2}{2z}$ to obtain the Fresnel diffraction integral, and show it is a convolution of $U(x',y')$ with a quadratic-phase kernel.

(c) In the far field (Fraunhofer regime), show that $U(x,y)$ reduces to the 2D Fourier transform of the aperture field evaluated at spatial frequencies $f_x = x/\lambda z$, $f_y = y/\lambda z$.

(d) Explain why this makes a single free-space propagation a *fixed* dense linear operator, so that a diffractive layer $H_d\,\mathrm{diag}(t^l)$ is a linear layer with fixed connectivity and only the diagonal $t^l = e^{i\phi^l}$ trained.

**14.2** (Fresnel number and the spacing for all-to-all connectivity) A layer has aperture side $A = 8$ cm and pixel pitch $\delta = 0.4$ mm at $\lambda = 0.75$ mm.

(a) Estimate the single-pixel diffraction half-angle $\theta \sim \lambda/\delta$, and find the minimum layer spacing $d$ at which one pixel's cone spans the full aperture; show $d \gtrsim A\delta/\lambda$ and evaluate it in centimeters.

(b) Compute the Fresnel number $N_F = a^2/(\lambda d)$ with $a = A/2$ at that spacing, and state whether the system is in the near-field, Fresnel, or Fraunhofer regime.

(c) If the layers are instead placed at half the distance from (a), describe qualitatively what happens to the connectivity: is the layer still fully connected, or does it become convolution-like?

(d) For a 5-layer stack at the spacing from (a), compute the total device length and comment on whether it is bench-scale.

**14.3** (Space-bandwidth product and neuron count) Use $\mathrm{SBP} \approx (2A\,\mathrm{NA}/\lambda)^2$.

(a) For $A = 5$ cm and $\mathrm{NA} = 0.5$ at $\lambda = 0.75$ mm, compute the one-dimensional neuron count and the SBP.

(b) Repeat at the telecom wavelength $\lambda = 1.55\ \mu$m and report the ratio of the two SBPs.

(c) What aperture would be required to hold $10^6$ neurons at $\lambda = 0.75$ mm with $\mathrm{NA} = 0.5$? Comment on practicality.

(d) Compare your part-(a) result to the $200\times200 = 4\times10^4$ neurons of Lin et al. (2018) and to the diffraction-limited maximum ($\mathrm{NA}\to 1$) for the same aperture.

**14.4** (Phase quantization) A phase mask is quantized to 8 bits over $[0, 2\pi)$.

(a) Compute the phase step $\Delta\phi = 2\pi/256$ in radians and the RMS quantization error $\Delta\phi/\sqrt{12}$.

(b) Using $\phi = \frac{2\pi}{\lambda}(n-1)h$, find the full-$2\pi$ height $h_{2\pi}$ and the 8-bit height step at $\lambda = 0.75$ mm with $n = 1.7$; repeat at $\lambda = 532$ nm with $n-1 = 0.5$. Contrast the fabrication demands.

(c) If the per-layer phase error compounds across the stack like the mesh error of Chapter 12, $\sim\!\sqrt{K}\,\sigma_\phi$, estimate the relative output error for a 5-layer, $200\times200$ network.

(d) How many quantization levels are needed to push the RMS phase error below $0.01$ rad?

**14.5** (Linear collapse and the role of $|\cdot|^2$) Consider $L$ phase-only diffractive layers with linear propagation between them.

(a) Show that the end-to-end field map is a single linear operator $M = H_d\,\mathrm{diag}(t^L)\cdots H_d\,\mathrm{diag}(t^1)$, and conclude that without an added nonlinearity the *field* stack has no more expressive power than one equivalent linear transform.

(b) Reconcile this with the empirical fact that a trained 5-layer network outperforms a trained single layer. (Hint: each layer is phase-only, a severe realizability constraint.)

(c) Identify the single genuine nonlinearity in a passive D2NN and where it acts.

(d) Contrast with a digital MLP, in which removing the activation functions collapses depth to a single linear layer.

## Design Exercises

**14.6** (Design a terahertz D2NN classifier) Design a 5-layer diffractive network to classify MNIST digits at 0.4 THz.

(a) Fix $\lambda = 0.75$ mm and choose an aperture $A$, pixel pitch $\delta$ (verify $\delta \ge \lambda/2$), and per-layer neuron count.

(b) Choose the layer spacing $d$ so that connectivity is all-to-all (use $d \gtrsim A\delta/\lambda$), and report the total device length.

(c) Lay out ten detector regions in the output plane; justify their size and spacing against detector crosstalk.

(d) Estimate the inference latency ($L\times d / c$) and confirm it is sub-nanosecond.

**14.7** (Differential-detection output for ten classes) Redesign the output plane of 14.6 to use class-specific differential detection.

(a) How many detector regions and photodiodes are now required?

(b) Propose a spatial layout of the positive/negative pairs that minimizes cross-class leakage.

(c) Write the decision rule in terms of $s_c = I_c^+ - I_c^-$ and explain what signed evidence buys over a single-ended argmax.

(d) Estimate the additional optical-power and detector overhead, and name the mechanism by which accuracy is expected to improve.

**14.8** (Platform selection) For each specification, choose among a 3D-printed fixed stack, an SLM-based reconfigurable system, and a metasurface / on-chip diffractive processor, and justify in 3–5 sentences using tolerance, reconfigurability, footprint, and wavelength.

(a) A low-cost, fixed terahertz classifier for a factory inspection line.

(b) A reconfigurable visible-wavelength research testbed that must be retrained weekly.

(c) An ultracompact CMOS-integrated diffractive front-end for on-chip inference.

## Conceptual Exercises

**14.9** (Connectivity: diffractive vs. convolutional) 

(a) Contrast a diffractive neuron's receptive field with a CNN kernel's: is D2NN connectivity fully connected, convolutional, or tunable between the two, and what physical parameter selects the regime?

(b) A digital fully-connected layer and a single free-space propagation are both dense linear maps. What distinguishes the diffractive one? (Consider that its off-diagonal structure is a fixed Fresnel/Fourier kernel and only the diagonal transmission is trained.)

(c) Under what layer spacing does a diffractive layer best approximate a convolution rather than a fully connected layer?

**14.10** (Why it works, and why it is hard) 

(a) Using the $\lambda/10$ tolerance argument of Subsection 14.4.2, explain why sim-to-real transfer is far harder at 532 nm than at 0.4 THz, quantifying the ratio.

(b) Explain why a passive, phase-only diffractive network — which is complex-linear on the inside — can nonetheless perform nontrivial classification.

(c) Explain why adding layers improves accuracy even though the field interior remains linear, and why the improvement diminishes with depth.

## Lab / Computational Exercises

**14.11** (Simulate and train a diffractive network) 

(a) Implement angular-spectrum (Rayleigh–Sommerfeld) propagation via the FFT in NumPy: multiply the field spectrum by the transfer function $H(f_x,f_y) = \exp\!\big(i k z\sqrt{1 - (\lambda f_x)^2 - (\lambda f_y)^2}\big)$. Validate against the analytic single-slit or Gaussian-beam result.

(b) In PyTorch, build a 5-layer phase-only D2NN — each layer a trainable array $e^{i\phi}$, propagation by your operator from (a) — read out ten detector regions, apply a softmax, and train on MNIST. Report test accuracy and compare to the $\sim 91.75\%$ reported by Lin et al. (2018).

(c) Visualize the learned phase masks as grayscale images and comment on any structure.

(d) Inject modeled lateral misalignment (Gaussian, $\sigma = \lambda/10$ and $\lambda/4$) at test time and measure the accuracy drop with and without misalignment-aware training, reproducing the "vaccination" effect of Mengu et al. (2020).

**14.12** (Gerchberg–Saxton phase design) 

(a) Implement the Gerchberg–Saxton iteration to design a single phase mask that maps a plane-wave input to a target intensity at a fixed distance.

(b) Design a mask that produces a $3\times3$ array of focal spots; report the diffraction efficiency and spot-to-spot uniformity.

(c) Study how efficiency and uniformity change with the number of iterations and with mask resolution.

(d) Discuss how Gerchberg–Saxton relates to single-layer D2NN training, and why gradient-based training of a multi-layer stack is strictly more expressive.
