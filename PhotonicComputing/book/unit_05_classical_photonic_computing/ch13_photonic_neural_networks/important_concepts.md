# Chapter 13: Important Concepts

## The Deep-Learning Substrate

**A layer is an affine map followed by a pointwise nonlinearity, and almost all the arithmetic is in the map.** The recursion $\mathbf{a}^{(l)} = f(W^{(l)}\mathbf{a}^{(l-1)} + \mathbf{b}^{(l)})$ costs $N_l N_{l-1}$ MACs for the matrix but only $N_l$ activations, a ratio of $N_{l-1}$ — hundreds to tens of thousands. In an MNIST MLP ($784\to256\to128\to10$), 99.83% of the operations are MACs; a GPT-3-class layer performs $\sim 12d^2 \approx 1.8\times10^9$ MACs against $\sim 10^5$ pointwise operations per token. An accelerator that speeds up only the linear algebra therefore addresses essentially all of the arithmetic.

**Without nonlinearities, depth collapses.** If $f$ were the identity, $W^{(L)}\cdots W^{(1)} = W_{\text{eff}}$, a single matrix, and the network would have the expressivity of one layer. The nonlinearity is the entire source of a deep network's power — which is exactly why the activation function, cheap in electronics, becomes the crux of a photonic implementation.

**Inference tolerates low precision; training does not.** Post-training quantization to INT8 typically costs under 1% accuracy, and 6–8 effective bits suffice for classification because decisions have margins and networks can be trained into robustness. This matches photonic hardware's 4–9 ENOB closely enough to be workable. Training, needing accumulation of millions of small gradient updates, demands higher precision and enormous weight-update bandwidth — the quantitative root of the field's focus on inference.

---

## The Activation Problem

**The activation is the hard part because photons do not interact.** Material nonlinearities exist (Kerr, saturable absorption, free-carrier, gain saturation) but demand optical powers orders of magnitude above the femtojoule budgets that make photonic computing attractive. The requirements list is demanding: a usable activation needs nonlinear shape, cascadability, gain, fan-out, speed, low energy, reproducibility, and differentiability — the same list that doomed optical logic in Chapter 11, now met in analog form.

**The pragmatic answer is opto-electronic (O-E-O), and it is affordable because activations are $O(N)$.** Detect, transform in the electrical domain, re-modulate: a modulator transfer function supplies a free sigmoid, at roughly 0.5–5 pJ and $\sim$100 ps per neuron. Since the optics does the $O(N^2)$ matrix work and the O-E-O crossings scale only as $O(N)$, the conversion overhead is asymptotically negligible at large $N$ — and O-E-O restores signal levels and provides the gain that all-optical schemes lack. All-optical activations (sub-fJ cavity switching, Nozaki et al. 2010, $\sim$0.42 fJ) are elegant but held back by threshold power and cascadability, not switching energy.

---

## Training Analog Hardware

**There is an epistemology ladder from trusting a model to trusting the hardware.** Offline training assumes a digital twin; noise-aware training makes the model robust to the twin's inaccuracy; in-situ methods extract gradients from the chip itself; hardware-in-the-loop training lets the physical forward pass replace the model. Each rung buys accuracy with hardware complexity or measurement cost.

**The sim-to-real gap is the central problem, and noise-aware training flattens it.** An analog chip with fabrication error, thermal drift, and shot noise computes a function that is never exactly the one in the training script; Shen et al. (2017) lost 15 points of accuracy to it. Injecting the perturbation statistics during training minimizes $\mathbb{E}[\mathcal{L}] \approx \mathcal{L} + \tfrac12\sigma^2\,\mathrm{tr}(H)$, selecting flat minima whose decisions are insensitive to deployment noise.

**In-situ backpropagation is the adjoint method in light.** Hughes et al. (2018) showed the gradient with respect to every phase shifter is $\propto \mathrm{Im}[e_{\text{fwd}}\,e_{\text{adj}}]$, where the adjoint field is the error vector injected *backward* into the output ports; reciprocity makes the backward pass implement the transpose $W^{\top}\boldsymbol\delta$ automatically. Three optical passes and one detector per shifter yield the whole gradient in constant time — backpropagation at light speed, demonstrated experimentally by Pai et al. (2023). The wall-clock bottleneck remains the microsecond phase-shifter update, not the arithmetic.

---

## Architectures That Embrace the Physics

**Reservoir computing fixes the network at random and trains only a linear readout.** A high-dimensional nonlinear dynamical system (the reservoir) projects inputs into a rich state space; a ridge-regression readout $W_{\text{out}} = YX^{\top}(XX^{\top}+\lambda I)^{-1}$ is the only trained part. Fabrication variation becomes the network instead of a defect. The echo-state property — fading memory, spectral radius $\rho < 1$ — guarantees the state depends on recent inputs with exponentially decaying weight.

**A single nonlinear node with delayed feedback emulates a whole network.** Appeltant et al. (2011): time-multiplex $N$ *virtual nodes* into a delay loop of length $\tau$ at spacing $\theta = \tau/N$, choosing $\theta \approx 0.2\times$ the node response time so neighbors couple. Photonic delay-line reservoirs reach Gb/s rates (Brunner et al. 2013) and $\sim$1 million words/s classification (Larger et al. 2017); integrated silicon reservoirs (Vandoorne et al. 2014) trade the delay line for a passive waveguide network.

**Inference degrades gracefully under analog noise.** Classification needs the right class to win the argmax, not numerical exactness, so a few percent of matrix error costs points rather than collapse — and noise-aware training recovers most of them. This graceful degradation, together with the 6–8 ENOB sufficiency of inference, is what makes analog photonic AI plausible at all.

---

## Attention and Transformers

**Attention is matrix multiplication with a dynamic-operand twist.** Scaled dot-product attention (Vaswani et al. 2017) is projections $Q=XW_Q$, $K=XW_K$, $V=XW_V$; scores $S=QK^{\top}/\sqrt{d_k}$; weights $A=\mathrm{softmax}(S)$; output $Y=AV$. The projections are weight-stationary and map onto Chapter 12 meshes directly, but $QK^{\top}$ and $AV$ multiply *activations by activations* — both operands are computed at runtime, so a weight-stationary core would have to reload matrices at the token rate, hitting the microsecond weight-update wall. Softmax stays electronic.

**The attention bottleneck is quadratic in sequence length.** Projections cost $O(Ld^2)$, the attention products $O(L^2d)$; at $L=1024$, $d=768$ the dynamic products are $\sim 40\%$ of the block and overtake the projections once $L > 2d$. This is the workload photonics must address to accelerate transformers, not merely the projections.

**The optical advantage grows with model size.** Because optical energy per MAC is $\bar{n}\,h\nu/N$ — shot-noise photons amortized over $N$ products — it falls as matrices grow, while digital per-MAC energy is roughly fixed. Anderson et al. (2024) model transformer inference under this law and find the optical MVM energy can dominate total inference energy at large scale, given a sufficiently low optical energy per MAC (approaching femtojoules). Coherent photoelectric multiplication (Hamerly et al. 2019), holding no static weights, is the architecture that fits attention's dynamic products.
