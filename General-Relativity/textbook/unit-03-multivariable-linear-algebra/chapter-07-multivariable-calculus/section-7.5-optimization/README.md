# Section 7.5: Optimization in Several Variables

---

## Section Introduction

Finding maxima and minima is among the oldest problems in mathematics. In one variable, a necessary condition for an interior extremum is that $f'(a) = 0$ (a **critical point**), and the second derivative test distinguishes maxima from minima from inflection points. In several variables, the necessary condition generalizes cleanly: $\nabla f(\mathbf{a}) = \mathbf{0}$. But the classification of critical points becomes richer and more interesting.

At a critical point of $f: \mathbb{R}^n\to\mathbb{R}$, the behavior is determined by the **Hessian matrix** $H_{ij} = \partial^2 f/\partial x^i\partial x^j$. If $H$ is positive definite (all eigenvalues positive), the critical point is a local minimum; negative definite, a local maximum; indefinite (mixed positive and negative eigenvalues), a **saddle point**. The saddle point — a maximum in one direction and a minimum in another — is the characteristic feature of higher-dimensional optimization and has no analogue in one dimension.

**Constrained optimization** — finding extrema of $f$ subject to the constraint $g(\mathbf{x}) = 0$ — is handled by **Lagrange multipliers**: at a constrained extremum, $\nabla f = \lambda\nabla g$ for some scalar $\lambda$. The Lagrange multiplier $\lambda$ measures how much the optimum value of $f$ would change if the constraint were relaxed. This connects to physics: Hamilton's principle of stationary action is a problem in the calculus of variations (infinite-dimensional optimization), and Lagrange multipliers appear in the constrained equilibrium problems of thermodynamics and field theory.

The **inverse function theorem** and **implicit function theorem** — covered in the companion section on the implicit function theorem — solve the problem: when can we solve $\mathbf{F}(\mathbf{x}) = \mathbf{0}$ locally for some variables in terms of others? The answer is: when the Jacobian is invertible. These theorems are the workhorses of differential geometry, guaranteeing the existence of local coordinates on manifolds.

---

## Subsections

- [7.5.1: Critical Points and the Hessian](7.5.1-critical-points.md)
- [7.5.2: Classification of Critical Points](7.5.2-classification.md)
- [7.5.3: Lagrange Multipliers](7.5.3-lagrange.md)
- [7.5.4: Global Optimization and Compact Domains](7.5.4-global.md)
- [7.5.5: Applications in Physics and Mechanics](7.5.5-applications.md)
