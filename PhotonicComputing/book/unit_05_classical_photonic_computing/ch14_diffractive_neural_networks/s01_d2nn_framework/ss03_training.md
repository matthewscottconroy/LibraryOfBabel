# Subsection 14.1.3: Training with Backpropagation

## Orientation

A D2NN is trained not on the bench but in a computer: the diffraction model of Subsection 14.1.2 is differentiable, so the whole forward pass — modulations and propagations alike — sits inside an automatic-differentiation graph, and the per-pixel phases are optimized by gradient descent exactly as the weights of any deep network. This subsection sets out the parameters, the loss, and the structure of the backward pass, and connects the simulated gradient to the physical in-situ backpropagation of Subsection 13.3.2.

---

## 14.1.3.1 Parameters and Forward Pass

The trainable parameters are the per-pixel phases $\{\phi^l(x,y)\}$ — one real number per neuron. Phase-only training enforces $|t^l| = 1$, so $t^l = e^{i\phi^l}$ and the plate is loss-free by construction. A five-layer, $200\times200$ network has

$$5 \times 200 \times 200 = 200{,}000$$

trainable phases — modest by deep-learning standards (a small convolutional net has more). The forward pass alternates $\mathrm{diag}(e^{i\phi^l})$ modulations with angular-spectrum propagations (Subsection 14.1.2.3), producing the complex output field $U^{\text{out}}$; the detector then records intensities $|U^{\text{out}}|^2$.

---

## 14.1.3.2 Loss: Detector Regions, Softmax, Cross-Entropy

For a $K$-class classifier, partition the output plane into $K$ non-overlapping detector regions $R_1,\dots,R_K$ and integrate the intensity in each:

$$I_k = \sum_{(x,y)\in R_k} |U^{\text{out}}(x,y)|^2, \qquad k = 1,\dots,K.$$

The predicted class is the brightest region, $\hat y = \arg\max_k I_k$. For training, the region intensities are passed through a softmax, $p_k = e^{I_k}/\sum_j e^{I_j}$, and scored by cross-entropy against the one-hot label; alternatively a mean-squared-error loss drives $|U^{\text{out}}|^2$ toward a target intensity map (used for the imaging D2NNs of Section 14.3). The intensity operation $|\cdot|^2$ is the network's *sole* nonlinearity — the single element that keeps the deep linear stack (Subsection 14.1.2.2) from collapsing to one equivalent plate.

---

## 14.1.3.3 The Backward Pass: Adjoint of the Propagator

Because every stage is linear and the readout is $|\cdot|^2$, the gradient follows from the chain rule with no special machinery. Each factor contributes its adjoint:

- the modulation $\mathrm{diag}(e^{i\phi^l})$ has a diagonal adjoint, and $\partial t^l/\partial\phi^l = i\,e^{i\phi^l}$ threads the phase parameter into the gradient;
- the propagation $H_d$ has adjoint $H_d^\dagger$ — and because Rayleigh–Sommerfeld propagation is *reciprocal*, $H_d^\dagger$ is simply propagation of the error field *backward* through the same gap (in the angular-spectrum form, multiplication by $\tilde H_d^{*}$).

The gradient with respect to a layer's phases is thus an interference term between the forward field at that layer and a *backward-propagating error field* injected from the output:

$$\frac{\partial \mathcal{L}}{\partial \phi^l(x,y)} \;\propto\; \mathrm{Im}\!\left[\,U^l_{\text{fwd}}(x,y)\;\overline{U^l_{\text{adj}}(x,y)}\,\right].$$

This is the same forward-times-adjoint structure as the physical in-situ backpropagation of Hughes et al. (Subsection 13.3.2), whose gradient reads $\propto \mathrm{Im}[\,e_{\text{fwd}}\,e_{\text{adj}}\,]$. Here it is executed in simulation by autograd; there it is executed by light propagating backward through a reciprocal mesh. The D2NN is the setting where the two views coincide most cleanly, because the forward model simply *is* Maxwell propagation — the digital twin and the hardware compute the identical operator.

---

## 14.1.3.4 Practicalities: Frameworks, Quantization, Offline Training

D2NN training is implemented in standard deep-learning frameworks (PyTorch, TensorFlow) with complex-valued tensors and FFT-based propagation layers; the $\sim2\times10^5$ parameters and small fields make it light. Two wrinkles distinguish it from ordinary training. First, the phases are periodic and, at deployment, *quantized* to the fabrication resolution (a finite number of printable height levels); this is handled by post-training quantization or, better, quantization-aware training that folds the discretization into the forward pass. Second — the decisive practical point — training is *pure simulation*: no chip, no laser, no detector is in the loop. The sim-to-real gap is deferred entirely to fabrication (Section 14.4), so training itself is fast, offline, and as reproducible as any software experiment.

---

## Worked Example: Training Budget for a 5-Layer Network

Take the five-layer, $200\times200$ network.

**Parameters.** $2\times10^5$ real phases, $8\times10^5$ bytes at fp32 — under a megabyte of weights.

**Compute per sample.** Five propagations, each two 2D FFTs: $\sim 6\times10^6$ complex operations for the forward pass (Subsection 14.1.2), with the backward pass a small constant multiple more.

**Memory.** Backpropagation must cache the complex field before each layer: $5\times(200\times200)$ complex numbers $\approx 1.6$ MB per sample at complex64. A minibatch of 64 needs $\sim100$ MB of activations — trivial for a modern GPU.

The entire 60,000-image MNIST training set therefore passes through the model in seconds per epoch. The scarce resource in D2NN research is not training compute; it is the fabrication and alignment of the physical stack into which the trained phases are eventually poured.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [Trains the diffractive stack by error-backpropagation through the differentiable RS model with detector-region losses — the recipe of this subsection.]

[2] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Analyzes loss design, detector-region layout, and training choices, including hybrid diffractive–electronic backpropagation.]

[3] Goodman, J.W. (2017). *Introduction to Fourier Optics* (4th ed.). W.H. Freeman. [The reciprocity of Rayleigh–Sommerfeld propagation that makes the backward pass a physical back-propagation of the error field.]
