# Section 7.4: The Total Derivative and the Jacobian

---

## Section Introduction

The single-variable derivative $f'(a)$ has two equivalent interpretations: it is the slope of the tangent line, and it is the best linear approximation to $f$ near $a$ (in the sense that $f(a+h) = f(a) + f'(a)h + o(h)$). The multivariable generalization of differentiation must generalize the *second* interpretation — the first (slope) makes no sense in higher dimensions.

A function $f: \mathbb{R}^n\to\mathbb{R}^m$ is **differentiable** at $\mathbf{a}$ if there exists a linear map $L: \mathbb{R}^n\to\mathbb{R}^m$ such that
$$\lim_{\mathbf{h}\to\mathbf{0}}\frac{\|f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) - L(\mathbf{h})\|}{\|\mathbf{h}\|} = 0$$
The map $L$ is the **total derivative** (or **Fréchet derivative**) of $f$ at $\mathbf{a}$, denoted $Df(\mathbf{a})$. If $f$ is differentiable, then $L$ is represented by the **Jacobian matrix** $J_{ij} = \partial f^i/\partial x^j$.

The total derivative is more demanding than the existence of partial derivatives: it requires a good linear approximation in *all* directions simultaneously, not just the coordinate ones. But it is the right notion of derivative for the purposes of analysis, because it makes the **chain rule** work: if $g: \mathbb{R}^m\to\mathbb{R}^k$ is differentiable at $f(\mathbf{a})$, then $g\circ f$ is differentiable at $\mathbf{a}$ and $D(g\circ f)(\mathbf{a}) = Dg(f(\mathbf{a}))\circ Df(\mathbf{a})$ — the composition of linear maps. This is the multivariable chain rule.

For GR, the Jacobian matrix is the coordinate change matrix. If $x^\mu$ and $\tilde{x}^\mu$ are two coordinate systems on a manifold, the transition functions $\tilde{x}^\mu = \tilde{x}^\mu(x^\nu)$ have Jacobians $\partial\tilde{x}^\mu/\partial x^\nu$ that define how tensors transform under coordinate changes. The abstract definition of a tensor is precisely a multilinear object that transforms in a specific way under these Jacobians. The total derivative is the seed of differential geometry.

---

## Subsections

- [7.4.1: The Fréchet Derivative](7.4.1-frechet.md)
- [7.4.2: The Jacobian Matrix](7.4.2-jacobian.md)
- [7.4.3: The Multivariable Chain Rule](7.4.3-chain-rule.md)
- [7.4.4: Coordinate Changes and Jacobians](7.4.4-coordinates.md)
- [7.4.5: The Mean Value Theorem for Vector Functions](7.4.5-mean-value.md)
