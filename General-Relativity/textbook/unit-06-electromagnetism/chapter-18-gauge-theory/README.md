# Chapter 18: Gauge Theory

---

## Chapter Introduction

The most important idea in modern physics is gauge invariance — the requirement that the laws of physics be invariant under local (position-dependent) transformations of a certain kind. In electromagnetism, the gauge group is U(1): the transformations are $A_\mu \to A_\mu + \partial_\mu\chi$ for any scalar function $\chi$. The physical fields $\mathbf{E}$ and $\mathbf{B}$ are invariant under this transformation; only the potential $A_\mu$ changes.

This chapter makes the gauge structure of electromagnetism explicit, then develops the full 4-potential formulation. Understanding this structure is indispensable for GR because:

1. GR is a gauge theory: diffeomorphism invariance (the freedom to change coordinates) is the gauge group of GR.

2. The gauge group of the Standard Model — U(1) × SU(2) × SU(3) — is a direct generalization of the U(1) of electromagnetism.

3. Kaluza-Klein theory shows that 5-dimensional GR contains 4-dimensional GR plus 4-dimensional Maxwell theory — electromagnetism literally emerges from gravity in one extra dimension.

The Aharonov-Bohm effect (1959) reveals that $A_\mu$ is more physical than the "mere gauge artifact" we might think. The vector potential affects quantum-mechanical phases even in regions where $\mathbf{E} = \mathbf{B} = 0$. This is the first hint that connections (in the sense of differential geometry) are the truly fundamental objects in field theory — not the field strengths.

---

## Chapter Contents

- **Section 18.1**: Gauge invariance; the physical and gauge degrees of freedom; the Aharonov-Bohm effect; gauge fixing; the scalar and vector potentials

- **Section 18.2**: The 4-potential $A^\mu = (\phi/c, \mathbf{A})$; the Faraday tensor $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$; Lorenz gauge; the covariant wave equation $\Box A^\mu = \mu_0 J^\mu$; the electromagnetic action; electromagnetism as U(1) gauge theory; the parallel transport interpretation

---

## From Potentials to Gauge Theory

In classical electromagnetism, one introduces potentials $\phi$ and $\mathbf{A}$ for computational convenience:

$$\mathbf{B} = \nabla\times\mathbf{A}, \qquad \mathbf{E} = -\nabla\phi - \partial_t\mathbf{A}$$

These automatically satisfy $\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{E} + \partial_t\mathbf{B} = 0$ (the first pair of Maxwell equations). But there is a freedom: for any function $\chi(\mathbf{r}, t)$:

$$\mathbf{A} \to \mathbf{A} + \nabla\chi, \qquad \phi \to \phi - \partial_t\chi$$

leaves $\mathbf{E}$ and $\mathbf{B}$ unchanged. This is **gauge invariance**.

The student's natural reaction: the potential is unphysical (only the fields matter), and gauge invariance is a redundancy to be eliminated by a gauge choice. This reaction is correct classically. But in quantum mechanics, Aharonov and Bohm showed in 1959 that the potential affects interference patterns even where the fields are zero — the potential is physical in a topological sense. The modern view: gauge fields are connections on fiber bundles, and $A_\mu$ is the local representative of this connection. The "redundancy" of gauge invariance is the fiber bundle's structure group acting.

This is exactly the geometric structure that appears in differential geometry, where the Christoffel symbols $\Gamma^\rho_{\mu\nu}$ are connections on the frame bundle, and the curvature (Riemann tensor) corresponds to the Faraday tensor.
