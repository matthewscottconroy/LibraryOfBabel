# Unit 03: Complex Integration

Integration along curves in the complex plane is the technical engine that drives the deepest theorems in complex analysis. This unit develops contour integration from first principles, then establishes Cauchy's theorems and their extraordinary consequences, and finally introduces the series representations — Taylor and Laurent — that describe analytic functions locally. The culmination is a rigorous proof of the Fundamental Theorem of Algebra, obtained as a corollary of Liouville's theorem.

## Chapter 01: Contour Integrals

A contour integral $\int_C f(z)\, dz$ is defined by parametrizing the curve $C$ and reducing to a real-variable line integral. The definition is formally analogous to line integrals in multivariable calculus, but the complex structure gives it special properties: the modulus $|f(z)|$ can be estimated uniformly along $C$, leading to the estimation lemma (also called the ML inequality), which is the workhorse of all later estimates.

## Chapter 02: Cauchy's Theorems

**Cauchy's theorem** asserts that if $f$ is analytic on and inside a simple closed curve $C$, then $\int_C f(z)\, dz = 0$. The Goursat form of this theorem, which requires only differentiability (not the continuity of $f'$), is a remarkable technical achievement.

From Cauchy's theorem flow:
- The **Cauchy integral formula**: the value of an analytic function at a point $z_0$ is expressed as a contour integral over any surrounding curve.
- **Differentiation via Cauchy**: all higher derivatives $f^{(n)}(z_0)$ are given by contour integrals, proving that analytic functions are infinitely differentiable.
- **Liouville's theorem**: every bounded entire function is constant.
- The **Fundamental Theorem of Algebra**: every nonconstant polynomial has a root in $\mathbb{C}$.

These results establish complex analysis as qualitatively different from real analysis: a function differentiable once in the complex sense is automatically differentiable infinitely many times, representable by power series, and subject to global constraints.

## Chapter 03: Series Representations

Taylor's theorem for analytic functions asserts that every function analytic at $z_0$ equals its Taylor series in a neighborhood of $z_0$. Laurent's theorem extends this to annular domains around a singularity, where the series contains negative powers. The classification of isolated singularities — removable, poles, and essential — is determined by the Laurent series, as is the residue, which is the coefficient of $(z - z_0)^{-1}$.

## Learning Objectives

By the end of this unit, a student should be able to:

- Compute contour integrals by parametrization.
- Apply the estimation lemma to bound contour integrals.
- State and apply Cauchy's theorem and the Cauchy integral formula.
- Use Cauchy's formula to compute derivatives of analytic functions.
- State and prove Liouville's theorem and derive the Fundamental Theorem of Algebra.
- Compute Taylor and Laurent series for standard functions around a given point.
- Classify isolated singularities from the Laurent series.
- Identify and compute residues at poles of finite order.
