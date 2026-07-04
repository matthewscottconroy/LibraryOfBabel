# Section 13.1: Deep Learning Foundations (Photonics-Oriented)

## What This Section Is About

This section is a compressed course in deep learning, written for a reader who knows Maxwell's equations better than PyTorch. It is not a substitute for a machine learning text; it is a translation layer. Every concept is introduced with one question in mind: *which parts of this computation belong on a photonic chip, and which parts must remain electronic?*

The answer has a consistent shape. The bulk arithmetic — the $O(N^2)$ multiply-accumulates of each layer — is linear algebra, photonics' native operation (Chapter 12). The connective tissue — nonlinearities, biases, normalization, gradient bookkeeping — is cheap per-element work, $O(N)$, that electronics does well. The engineering question is never "can optics do deep learning?" but "does the boundary between the optical and electronic domains fall in a place that saves energy and time?"

**13.1.1: The Feedforward Network as Matrix Operations** — Layers as affine maps plus pointwise nonlinearities; parameter and operation counting for real networks from MNIST-scale MLPs to GPT-class transformers; convolution as matrix multiplication; why >99% of inference arithmetic is MAC operations; the precision requirements of inference versus training.

**13.1.2: Backpropagation** — The chain rule organized as a backward pass; the recursion $\boldsymbol{\delta}^{(l)} = (W^{(l+1)})^T \boldsymbol{\delta}^{(l+1)} \odot f'(\mathbf{z}^{(l)})$; why gradients cost roughly three forward passes; the transpose-matrix requirement and its elegant photonic realization via backward propagation through a reciprocal mesh.

**13.1.3: The Hardware Bottleneck for AI** — Where the joules actually go: arithmetic versus data movement; the memory wall; training versus inference economics; a quantitative statement of the target photonics must beat, and of the regimes (batch-1 latency, weight-stationary inference, in-flight analog signals) where the target is softest.
