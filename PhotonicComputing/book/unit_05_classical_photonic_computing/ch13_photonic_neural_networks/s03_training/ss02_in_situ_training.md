# Subsection 13.3.2: In-Situ Training

## Orientation

In-situ training abandons the digital twin: the gradients themselves are obtained from the physical hardware. The network being optimized is then, by construction, the network being deployed — the sim-to-real gap vanishes identically. The cost is measurement: every gradient must be extracted from the chip through detectors, and the efficiency of that extraction separates the methods. This subsection covers the two families — zeroth-order (gradient-free) optimization, and the physically beautiful in-situ backpropagation of Hughes et al., in which the backward pass of Subsection 13.1.2 is executed by light propagating backward through the mesh.

---

## 13.3.2.1 Zeroth-Order Methods: Gradients from Forward Passes Alone

If the hardware is a black box $\mathcal{L}(\boldsymbol{\theta})$ that can only be *evaluated*, gradients must be estimated from function values.

**Finite differences** perturbs one parameter at a time: $\partial\mathcal{L}/\partial\theta_k \approx [\mathcal{L}(\boldsymbol{\theta} + \epsilon\mathbf{e}_k) - \mathcal{L}(\boldsymbol{\theta})]/\epsilon$. Cost: $P+1$ hardware evaluations per step for $P$ parameters — ruinous for $P \sim 10^3$–$10^6$, but usable for few-parameter demonstrations, and historically the first method applied to photonic circuits.

**Simultaneous perturbation (SPSA)** perturbs *all* parameters at once with a random sign vector $\boldsymbol{\Delta}$ ($\Delta_k = \pm1$):

$$\widehat{\frac{\partial\mathcal{L}}{\partial\theta_k}} = \frac{\mathcal{L}(\boldsymbol{\theta} + \epsilon\boldsymbol{\Delta}) - \mathcal{L}(\boldsymbol{\theta} - \epsilon\boldsymbol{\Delta})}{2\epsilon\,\Delta_k}$$

Two hardware evaluations yield an unbiased (though high-variance) estimate of the *entire* gradient; convergence theory guarantees descent on average. SPSA and its cousins (random-direction gradient estimates, evolutionary strategies like CMA-ES) are the workhorses of analog-hardware training generally — they require nothing but forward passes, tolerate noise (indeed average over it), and automatically optimize the true hardware including every unmodeled parasitic. Their weakness is sample efficiency: variance grows with parameter count, so training a $10^4$-phase mesh takes correspondingly many more steps than exact gradient descent. For photonic hardware whose forward pass runs at MHz–GHz rates, however, hardware evaluations are cheap and the economics are less damning than they look — the binding constraint is usually the μs-scale *weight update* between evaluations (Subsection 13.1.2), which zeroth-order methods invoke constantly.

## 13.3.2.2 In-Situ Backpropagation: The Adjoint Method in Light

Hughes, Minkov, Shi, and Fan (2018) proved a result that deserves to be called the field's most elegant: for a linear photonic circuit, **the gradient of the loss with respect to every internal phase shifter can be measured physically, using only forward and backward optical propagation plus local intensity measurements.**

The structure is the adjoint method of computational electromagnetics transplanted to network training. The gradient of the loss with respect to the permittivity perturbation of phase shifter $k$ takes the form of an interference term between two fields:

$$\frac{\partial \mathcal{L}}{\partial \theta_k} \;\propto\; \mathrm{Im}\!\left[\, e_{\text{fwd}}(\mathbf{r}_k)\; e_{\text{adj}}(\mathbf{r}_k) \,\right]$$

where $e_{\text{fwd}}$ is the ordinary forward field at the shifter and $e_{\text{adj}}$ is an *adjoint field* obtained by injecting the error vector $\boldsymbol{\delta}$ (computed at the output, per Subsection 13.1.2) **backward into the output ports**. Reciprocity guarantees the backward propagation implements the transposed linear operator — the exact $(W)^T\boldsymbol{\delta}$ the backward recursion demands. The product of the two fields at each shifter is obtained interferometrically: propagate forward and adjoint fields simultaneously and read the resulting intensity pattern at each phase shifter with a local monitor detector. Three optical passes (forward; adjoint; interference) and one detector per phase shifter deliver the complete gradient of the layer *in constant time, independent of parameter count* — the optical analog of backpropagation's celebrated efficiency, executed at light speed.

**Experimental realization.** Pai et al. (*Science*, 2023, Stanford) demonstrated the full protocol on silicon photonic meshes with integrated monitor detectors: forward inference, backward error injection, interferometric gradient readout, and gradient-descent training of the physical device on small classification tasks — backpropagation running *in the optics*, with measured gradients matching theory. The demonstration is small-scale, and practical deployment must add hardware (taps and detectors at every shifter, bidirectional couplers, coherent error-signal generation) — but it establishes that gradient-based training does not, in principle, require a digital model of the hardware at all.

**Scope and caveats.** The method natively differentiates the *linear* stages; nonlinear activations must either be handled electronically between optical layers (storing forward activations, applying $f'$ masks — straightforward in the O-E-O architectures of Section 13.2.2) or be of special forms admitting optical adjoints. Coherent backward injection requires phase-stable access to output ports, and monitor-detector calibration enters the gradient's accuracy. As with all in-situ schemes, the weight-update bandwidth of the phase shifters remains the wall-clock bottleneck.

## 13.3.2.3 Forward-Only Physical Gradients

A middle path has emerged: gradient estimates from *forward* passes structured more cleverly than SPSA. Bandyopadhyay et al.'s single-chip deep network (2024) trained with a forward-only scheme; related "physical local learning" proposals (forward-forward-style objectives, direct feedback alignment with random projections) replace the backward pass with additional forward measurements, trading gradient exactness for hardware simplicity. These methods matter because backward optical access is an intrusive hardware requirement; forward-only training needs nothing the inference system does not already have.

---

## References

[1] Hughes, T.W., Minkov, M., Shi, Y., & Fan, S. (2018). "Training of photonic neural networks through in situ backpropagation and gradient measurement." *Optica*, 5(7), 864–871. [The adjoint-method result: physical backward propagation computes the gradient; the central paper of this subsection.]

[2] Pai, S., et al. (2023). "Experimentally realized in situ backpropagation for deep learning in photonic neural networks." *Science*, 380(6643), 398–404. [The experimental demonstration on silicon meshes with integrated gradient readout.]

[3] Spall, J.C. (1992). "Multivariate stochastic approximation using a simultaneous perturbation gradient approximation." *IEEE Transactions on Automatic Control*, 37(3), 332–341. [SPSA: the two-measurement gradient estimator underlying most gradient-free analog training.]

[4] Bandyopadhyay, S., et al. (2024). "Single-chip photonic deep neural network with forward-only training." *Nature Photonics*, 18. [In-situ forward-only training of a fully integrated photonic network.]

[5] Spall, J., Guo, X., & Lvovsky, A.I. (2022). "Hybrid training of optical neural networks." *Optica*, 9(7), 803–811. [Optical forward pass combined with digital backward pass — the bridge between this subsection and hardware-in-the-loop training.]
