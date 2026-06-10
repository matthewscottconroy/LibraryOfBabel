# Section 10.1: First-Order ODEs and the Picard-Lindelöf Theorem

---

## Section Introduction

The simplest ODE is the first-order equation y' = f(x, y). Given an initial condition y(x₀) = y₀, we ask: does a solution exist, and is it unique? The **Picard-Lindelöf theorem** answers both questions — affirmatively, under a Lipschitz continuity condition on f.

The theorem's proof uses the **Banach fixed-point theorem**: the method of successive approximations (Picard iterations) converges to the unique solution, provided f is Lipschitz in y. The Banach fixed-point theorem is one of the most useful results in all of analysis, appearing in: the proof of the Implicit Function Theorem (Section 7.3), the construction of solutions to PDEs (Chapter 11), and existence proofs throughout differential geometry.

---

## 10.1.1 The Initial Value Problem

**Definition**: An **initial value problem (IVP)** is:

$$y'(x) = f(x, y(x)), \quad y(x_0) = y_0$$

A **solution** is a differentiable function y: (α, β) → ℝ (containing x₀) satisfying both the ODE and the initial condition.

**Equivalent integral formulation**: y is a solution iff:

$$y(x) = y_0 + \int_{x_0}^x f(t, y(t)) \, dt$$

This is the key. The ODE is equivalent to a fixed-point equation: y = T(y) where T(φ)(x) = y₀ + ∫_{x₀}^x f(t, φ(t)) dt.

---

## 10.1.2 The Banach Fixed-Point Theorem

**Definition**: A metric space (X, d) is **complete** if every Cauchy sequence converges. A map T: X → X is a **contraction** if ∃L < 1 such that d(T(x), T(y)) ≤ L d(x,y) for all x, y.

**Theorem** (Banach Fixed-Point Theorem / Contraction Mapping Theorem): If T: X → X is a contraction on a complete metric space, then T has a **unique** fixed point x* = T(x*). Moreover, the iteration xₙ₊₁ = T(xₙ) converges to x* from any starting point x₀.

*Proof*: The sequence (xₙ) is Cauchy: d(xₙ₊₁, xₙ) ≤ Lⁿ d(x₁, x₀). Since X is complete, xₙ → x*. Taking the limit of xₙ₊₁ = T(xₙ) and using continuity of T gives T(x*) = x*. Uniqueness: if T(x*) = x* and T(y*) = y*, then d(x*, y*) = d(T(x*), T(y*)) ≤ L d(x*, y*), so (1−L)d(x*, y*) ≤ 0, forcing d(x*, y*) = 0. □

---

## 10.1.3 The Picard-Lindelöf Theorem

**Definition**: f is **Lipschitz in y** (with constant K) on a rectangle R = {|x − x₀| ≤ a, |y − y₀| ≤ b} if:

$$|f(x, y_1) - f(x, y_2)| \leq K|y_1 - y_2| \quad \text{for all } (x, y_1), (x, y_2) \in R$$

A continuously differentiable f with |∂f/∂y| ≤ K on R is Lipschitz (with constant K).

**Theorem** (Picard-Lindelöf, 1890): If f: R → ℝ is continuous and Lipschitz in y on R, then the IVP y' = f(x, y), y(x₀) = y₀ has a **unique** solution on |x − x₀| ≤ h, where h = min(a, b/M, 1/K) and M = sup_R |f|.

*Proof*: On X = C([x₀−h, x₀+h]) (complete with sup norm), define T(φ)(x) = y₀ + ∫_{x₀}^x f(t, φ(t)) dt. Show T maps a ball B = {φ: ||φ − y₀|| ≤ b} to itself (using |f| ≤ M and h ≤ b/M) and is a contraction (||T(φ₁) − T(φ₂)|| ≤ Kh||φ₁ − φ₂|| ≤ ||φ₁ − φ₂|| since Kh ≤ 1). By Banach's theorem, T has a unique fixed point, which is the unique solution. □

**The Picard iteration**: Start with φ₀(x) = y₀ (constant) and iterate φₙ₊₁ = T(φₙ). This is the **method of successive approximations**: each iterate is an explicit integral of the previous. The sequence converges to the solution.

---

## 10.1.4 Limitations and Blow-Up

The Picard-Lindelöf theorem guarantees only **local** existence. Solutions can cease to exist in finite time.

**Example**: y' = y², y(0) = 1. Solution: y(x) = 1/(1−x). This blows up at x = 1. The solution exists only on (−∞, 1).

**Extension to maximal interval**: Every solution can be extended to a maximal interval of existence. If the maximal interval is bounded, then |y(x)| → ∞ as x approaches the endpoint (blow-up).

**Physical significance**: Blow-up in the solution of an ODE corresponds to a physical singularity. In GR, the breakdown of solutions to the geodesic equation — incomplete geodesics — is the definition of a spacetime singularity (Penrose-Hawking singularity theorems, Chapter 53). The parallel between ODE blow-up and spacetime singularities is not merely metaphorical.

---

## 10.1.5 First-Order Linear ODEs

The first-order linear ODE y' + P(x)y = Q(x) is always solvable explicitly.

**Integrating factor**: Multiply through by μ(x) = e^{∫P dx}:

$$\frac{d}{dx}[\mu(x) y] = \mu(x) Q(x)$$

Integrate both sides: $y = \frac{1}{\mu(x)}\int \mu(x) Q(x) \, dx + \frac{C}{\mu(x)}$.

**Example**: y' + (2/x)y = x². Integrating factor: μ = x². Then (x²y)' = x⁴, so x²y = x⁵/5 + C, giving y = x³/5 + C/x².

**Connection to GR**: The ODE for energy of a particle in Schwarzschild geometry, E = (1−rₛ/r) dt/dτ = const, is a first integral of the geodesic equation. The first-order ODE for dr/dτ (from energy conservation) gives the effective potential for radial motion. The structure of black hole interiors and the behavior of infalling observers are determined by these first-order ODEs.

---

## References

- Arnold, V.I. (1992). *Ordinary Differential Equations*, 3rd ed. Springer. [The most insightful treatment; connects ODEs to geometry from the start.]
- Picard, É. (1890). "Mémoire sur la théorie des équations aux dérivées partielles et la méthode des approximations successives." *Journal de Mathématiques Pures et Appliquées*, 6, 145–210. [The original paper on successive approximations.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [The contraction mapping theorem and its application to ODEs; Chapter 9.]
- Lindelöf, E. (1894). "Sur l'application de la méthode des approximations successives aux équations différentielles ordinaires du premier ordre." *Comptes rendus de l'Académie des sciences*, 118, 454–457. [The Lipschitz condition for uniqueness.]
