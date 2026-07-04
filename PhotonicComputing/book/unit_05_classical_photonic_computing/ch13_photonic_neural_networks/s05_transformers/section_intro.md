# Section 13.5: Optical Transformers and Attention Mechanisms

## What This Section Is About

The transformer is the architecture behind modern large language models, and its central operation — self-attention — is, arithmetically, a sequence of matrix multiplications. That should make it the ideal customer for the photonic matrix engines of Chapter 12. But attention carries a twist that stresses those engines in a way ordinary feedforward layers do not, and the point of this section is to make that twist precise and quantify what a photonic attention accelerator would actually require.

**13.5.1: Self-Attention as Matrix Operations** — We formalize scaled dot-product attention (Vaswani et al., 2017) as query, key, and value projections followed by two products, $QK^{\top}$ and $AV$, with a softmax between them. The decisive observation is a split: the projection weights are *static* and weight-stationary — perfect for meshes and weight banks — but $QK^{\top}$ and $AV$ multiply *activations by activations*. Both operands are computed at runtime, so the weight-stationary assumption on which most photonic processors rest no longer holds for the very operation that defines the architecture. We count MACs to show that this dynamic part is a large, sequence-length-quadratic fraction of the work, and that softmax remains an electronic nonlinearity (Section 13.2).

**13.5.2: Photonic Dot-Product Accelerator for Attention** — We ask what hardware could accelerate the dynamic products, contrasting reconfigured-weight schemes (blocked by the microsecond weight-update wall) with coherent photoelectric multipliers (Hamerly et al., 2019) that stream both operands and hold no static weights. We then work through the McMahon-group "Optical transformers" energy argument (Anderson et al., 2024): the optical matrix-multiply advantage *grows* with model dimension and can dominate total inference energy at large scale.

The organizing tension: photonic advantage grows with model size, but dynamic-operand attention spends part of that advantage re-loading matrices.
