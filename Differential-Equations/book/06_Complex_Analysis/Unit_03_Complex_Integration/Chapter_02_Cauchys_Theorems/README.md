# Chapter 02: Cauchy's Theorems

Cauchy's theorems are the core of complex analysis. The central result — that the contour integral of an analytic function around a closed curve in a simply connected domain is zero — looks at first like a special property of a particular class of functions, but it turns out to imply a sweeping global theory with no real-variable counterpart. From Cauchy's theorem flow the integral formula for function values, the representation of all derivatives as contour integrals, and ultimately Liouville's theorem and the Fundamental Theorem of Algebra.

## The Cauchy-Goursat Theorem

The fundamental theorem is:

**Theorem (Cauchy-Goursat).** If $f$ is analytic on and inside a simple closed contour $C$, then $\oint_C f(z)\, dz = 0$.

Goursat's contribution was to prove this without assuming continuity of $f'$ — only differentiability is required. This technical strengthening matters: it allows the theorem to be applied in situations where one cannot a priori verify that $f'$ is continuous. The proof proceeds by a triangulation and subdivision argument, showing that the integral over any triangle tends to zero as the triangle shrinks.

## The Cauchy Integral Formula

**Theorem (Cauchy Integral Formula).** If $f$ is analytic on and inside $C$ and $z_0$ is any interior point:
$$f(z_0) = \frac{1}{2\pi i} \oint_C \frac{f(z)}{z - z_0}\, dz.$$

This astonishing formula says that the values of an analytic function inside a region are completely determined by its values on the boundary. It has no real-variable analogue: knowing a real differentiable function on the boundary of an interval tells you only its boundary values, not its interior ones.

## Derivatives via Cauchy

Differentiating the integral formula with respect to $z_0$:
$$f^{(n)}(z_0) = \frac{n!}{2\pi i} \oint_C \frac{f(z)}{(z - z_0)^{n+1}}\, dz.$$

This shows that every analytic function is infinitely differentiable — a property far stronger than real differentiability. In real analysis, there exist functions that are once differentiable but not twice; in complex analysis, once implies infinitely many times.

## Liouville's Theorem and the Fundamental Theorem of Algebra

**Liouville's theorem:** A bounded entire function is constant.

**Fundamental Theorem of Algebra:** Every nonconstant polynomial with complex coefficients has at least one root in $\mathbb{C}$.

Both are proved as corollaries of the integral formula. These theorems illustrate the global nature of analyticity: local information (boundedness on all of $\mathbb{C}$, or behavior of a polynomial for large $|z|$) forces global conclusions (constancy, or existence of a root).

## Learning Objectives

After this chapter, a student should be able to:

- State and apply the Cauchy-Goursat theorem, including the version for multiply connected domains.
- Apply the Cauchy integral formula to evaluate contour integrals.
- Use the differentiated form to compute $f^{(n)}(z_0)$ and to evaluate integrals.
- Prove Liouville's theorem from the integral formula for $f'$.
- Prove the Fundamental Theorem of Algebra from Liouville's theorem.
