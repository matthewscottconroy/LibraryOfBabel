# Section 9.5: Inner Product Spaces

---

## Section Introduction

A vector space is an abstract algebraic structure: you can add vectors and scale them, but there is no notion of length or angle. An **inner product** (generalized dot product) adds this metric structure. A real inner product $\langle\cdot,\cdot\rangle: V\times V\to\mathbb{R}$ must be bilinear, symmetric, and **positive definite**: $\langle\mathbf{v},\mathbf{v}\rangle > 0$ for all $\mathbf{v}\neq\mathbf{0}$. The inner product defines the **norm** $\|\mathbf{v}\| = \sqrt{\langle\mathbf{v},\mathbf{v}\rangle}$ and the **angle** between vectors.

In finite dimensions, every inner product space over $\mathbb{R}$ is isomorphic to $\mathbb{R}^n$ with the standard dot product. But in infinite dimensions (function spaces), inner product spaces — called **Hilbert spaces** when complete — have a rich structure and are the arena for quantum mechanics. The wave function $\psi(x)$ lives in $L^2(\mathbb{R})$, the Hilbert space of square-integrable functions with inner product $\langle\psi,\phi\rangle = \int\bar{\psi}(x)\phi(x)\,dx$.

The **Gram-Schmidt process** orthogonalizes any basis of an inner product space into an orthonormal basis — one where $\langle\mathbf{e}_i,\mathbf{e}_j\rangle = \delta_{ij}$. Orthonormal bases are computationally convenient (coordinates are computed by inner products: $v^i = \langle\mathbf{e}_i,\mathbf{v}\rangle$) and theoretically important (every Hilbert space with a countable orthonormal basis is isomorphic to $\ell^2$).

For GR, the inner product concept must be generalized: the metric tensor $g_{\mu\nu}$ defines a bilinear form on each tangent space, but it is **not** positive definite — it has signature $(-,+,+,+)$ for Lorentzian spacetimes. This pseudo-inner product (indefinite metric) allows vectors to have negative, zero, or positive "length squared." The distinction between timelike, spacelike, and null vectors, which is at the heart of GR's causal structure, is exactly the signature information of the pseudo-inner product.

---

## Subsections

- [9.5.1: Inner Products and Norms](9.5.1-definition.md)
- [9.5.2: The Cauchy-Schwarz Inequality](9.5.2-cauchy-schwarz.md)
- [9.5.3: Orthogonality and Orthonormal Bases](9.5.3-orthogonality.md)
- [9.5.4: Gram-Schmidt Orthogonalization](9.5.4-gram-schmidt.md)
- [9.5.5: Indefinite Inner Products and Pseudo-Riemannian Metrics](9.5.5-indefinite.md)
