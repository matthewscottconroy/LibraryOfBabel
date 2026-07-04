# Chapter 13: Photonic Neural Networks — Architecture and Training

## From Matrix Engine to Learning Machine

Chapter 12 built the engine: photonic hardware that multiplies a vector by a matrix in a single optical transit. But a matrix multiplier is not a neural network, any more than a multiplier circuit is a computer. A neural network is matrices *plus* nonlinear activation functions, *plus* a training algorithm that finds the weights, *plus* an architecture that arranges these pieces for a task. Each of these additional ingredients is problematic for photonics, and each problem is interesting enough to have generated its own research literature.

Consider what a "photonic neural network" must actually accomplish. Between every pair of linear layers, something must compute a nonlinear function — but photons famously do not interact with each other, and coaxing a nonlinear response out of optics at microwatt power levels runs against the grain of the physics (this is the same wall that doomed optical logic in Chapter 11, now met in analog form). The weights must come from somewhere — but backpropagation assumes exact knowledge of the forward computation, and an analog chip with fabrication errors, thermal drift, and shot noise computes a function that is never exactly the one in the training script. And the resulting system must beat a GPU at *something* — which requires understanding precisely where the electronic hardware bottleneck for AI actually lies, rather than where a casual reading of TOPS specifications suggests it lies.

This chapter works through all four ingredients. It is the most machine-learning-flavored chapter of the book, and deliberately so: the physics of Units I–III constrains everything here, but the questions are posed in the language of deep learning, and a photonics student who cannot follow that language cannot evaluate the field's claims.

---

## What This Chapter Establishes

**Deep learning is matrix multiplication punctuated by cheap nonlinearities** (Section 13.1). A feedforward layer is $\mathbf{a}^{(l)} = f(W^{(l)}\mathbf{a}^{(l-1)} + \mathbf{b}^{(l)})$; more than 99% of the arithmetic lives in $W\mathbf{a}$, which is exactly the operation Chapter 12 accelerates. Backpropagation — the algorithm that computes gradients — is itself built from matrix multiplications by the *transposed* weight matrices, a fact with a beautiful photonic echo: a reciprocal optical network run backward implements the transpose automatically. The section closes with an unsentimental accounting of the AI hardware bottleneck: where the energy actually goes in a GPU, and which parts of that budget photonics can and cannot attack.

**The activation function is the hard part** (Section 13.2). We survey the two roads: opto-electronic activation (detect, transform electronically, re-modulate — practical, cascadable, and costing picojoules and ~100 ps per neuron) and all-optical activation (saturable absorption, resonator bistability, cavity-enhanced Kerr — elegant, but with threshold powers that typically exceed the entire computing power budget). The honest current answer is opto-electronic, and the section quantifies why that answer is less damaging than it first appears: activations are $O(N)$ while the optics does the $O(N^2)$ work.

**Training is where analog hardware meets its epistemological problem** (Section 13.3). Offline training assumes a model of the hardware; the hardware disagrees; accuracy falls. We examine the escalating responses: noise-aware training (make the model robust to the disagreement), in-situ gradient measurement (make the hardware compute its own gradients, including the in-situ backpropagation of Hughes et al. and its experimental realization by Pai et al.), and hardware-in-the-loop training (let the physical forward pass replace the model entirely, as in physics-aware training).

**Some architectures dodge the problems instead of solving them** (Sections 13.4–13.5). Reservoir computing fixes the recurrent network at random and trains only a linear readout — turning fabrication variation from a bug into the network itself; photonic reservoirs based on time-delayed feedback have delivered some of the field's most convincing benchmark results. At the opposite end of fashion, we analyze the transformer's attention mechanism as a matrix-multiplication workload and ask what a photonic attention accelerator would require — a question sharpened by the fact that attention multiplies *activations by activations*, not activations by static weights, undermining the weight-stationary assumption on which most photonic accelerators rest.

---

## A Reading Frame

Two recurring tensions organize everything in this chapter. First, **expressivity versus physicality**: the operations deep learning wants (exact nonlinearities, transposes, gradients, softmax) versus the operations photonic hardware gives cheaply (linear maps, intensity detection, noise). Second, **generality versus advantage**: the more faithfully a photonic system replicates a GPU's abstractions, the more of its physical advantage it spends on the replication; the architectures that win benchmarks (reservoirs, in-flight signal processors) are the ones that let the physics be itself. Keep both tensions in view and the design choices of every paper in this literature become legible.

---

## References

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The chapter's point of departure; analyzed in detail in Section 12.3.2.]

[2] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114. [The standard survey of the architectures treated in this chapter.]

[3] Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47. [A cross-community manifesto connecting integrated, free-space, and computational-imaging approaches to optical AI.]

[4] LeCun, Y., Bengio, Y., & Hinton, G. (2015). "Deep learning." *Nature*, 521, 436–444. [The canonical short introduction to the algorithms this chapter maps onto hardware.]
