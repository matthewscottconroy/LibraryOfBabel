# Section 9.7: Dual Spaces, Bilinear Forms, and Tensors

---

## Section Introduction

The **dual space** $V^*$ of a vector space $V$ consists of all linear maps (functionals) $\omega: V\to\mathbb{R}$. Elements of $V^*$ are called **covectors** or **one-forms**. If $V$ has a basis $\{e_1,\ldots,e_n\}$, then $V^*$ has a natural **dual basis** $\{e^1,\ldots,e^n\}$ defined by $e^i(e_j) = \delta^i_j$. The dual space is isomorphic to $V$ as a vector space, but not canonically — the isomorphism requires the choice of a basis, or equivalently, an inner product.

The distinction between vectors and covectors is one of the most important in physics, even if it is invisible in coordinate-based presentations. A displacement is a vector — it transforms contravariantly under coordinate changes. A gradient is a covector (one-form) — it transforms covariantly. The metric tensor provides the canonical isomorphism between vectors and covectors (index raising and lowering): $\omega_\mu = g_{\mu\nu}v^\nu$. Without a metric, vectors and covectors are genuinely different objects.

A **bilinear form** on $V$ is a bilinear map $B: V\times V\to\mathbb{R}$. The metric tensor $g$ in GR is a bilinear form on each tangent space. Bilinear forms can be symmetric ($B(\mathbf{u},\mathbf{v}) = B(\mathbf{v},\mathbf{u})$) or antisymmetric ($B(\mathbf{u},\mathbf{v}) = -B(\mathbf{v},\mathbf{u})$). Antisymmetric bilinear forms are **2-forms** (elements of $\Lambda^2 V^*$) — the simplest examples of differential forms.

**Tensors** are multilinear maps: a tensor of type $(p,q)$ is a multilinear map $T: V^*\times\cdots\times V^*\times V\times\cdots\times V\to\mathbb{R}$ (with $p$ covector arguments and $q$ vector arguments). The components $T^{\mu_1\cdots\mu_p}{}_{\nu_1\cdots\nu_q}$ in coordinates are the image of basis elements. Tensors are the central objects of GR: the metric $g_{\mu\nu}$ is a $(0,2)$ tensor, the Riemann tensor $R^\mu{}_{\nu\rho\sigma}$ is a $(1,3)$ tensor, and the Einstein equations $G_{\mu\nu} = 8\pi T_{\mu\nu}$ are an equation between $(0,2)$ tensors.

---

## Subsections

- [9.7.1: Dual Spaces and Covectors](9.7.1-dual-spaces.md)
- [9.7.2: Dual Bases and Coordinate Transformations](9.7.2-dual-bases.md)
- [9.7.3: Bilinear Forms and Quadratic Forms](9.7.3-bilinear.md)
- [9.7.4: Introduction to Tensors](9.7.4-tensors.md)
- [9.7.5: The Metric as a Bilinear Form and Index Manipulation](9.7.5-metric.md)
