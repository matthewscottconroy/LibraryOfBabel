# Section 6.4: Uniform Convergence

---

## Section Introduction

We have been assuming that limits of sequences of functions "work the same way" as limits of sequences of numbers — that we can differentiate or integrate a limit by differentiating or integrating each function in the sequence. This assumption can fail. The distinction between *pointwise* and *uniform* convergence is where it either holds or fails, and understanding this distinction is essential for analysis.

---

## 6.4.1 Pointwise vs. Uniform Convergence

Let (fₙ) be a sequence of functions defined on a set E.

**Pointwise convergence**: (fₙ) converges **pointwise** to f on E if, for each x ∈ E, fₙ(x) → f(x). In ε-N language:

$$\forall x \in E \; \forall \varepsilon > 0 \; \exists N(x, \varepsilon) \in \mathbb{N} \; \forall n \geq N: |f_n(x) - f(x)| < \varepsilon$$

The critical feature: N depends on x. For different x, we may need different N.

**Uniform convergence**: (fₙ) converges **uniformly** to f on E if:

$$\forall \varepsilon > 0 \; \exists N(\varepsilon) \in \mathbb{N} \; \forall x \in E \; \forall n \geq N: |f_n(x) - f(x)| < \varepsilon$$

A single N works for all x simultaneously. The graph of fₙ stays uniformly within ε of the graph of f, for all n ≥ N.

Uniform convergence implies pointwise convergence, but not vice versa.

**Geometric picture**: Pointwise convergence means the graph of fₙ eventually passes through any ε-tube around f(x), but different points may require different n. Uniform convergence means the entire graph of fₙ eventually lies inside the ε-band around the graph of f.

---

## 6.4.2 A Counterexample to Naive Limit Interchange

**Example**: Let fₙ(x) = xⁿ on [0, 1]. Then:
- For x ∈ [0, 1): xⁿ → 0 (since |x| < 1).
- For x = 1: fₙ(1) = 1 for all n, so fₙ(1) → 1.

Pointwise limit: f(x) = 0 for x ∈ [0, 1) and f(1) = 1. This is discontinuous at x = 1, even though each fₙ is continuous. Continuity is not preserved under pointwise limits.

Is the convergence uniform? The sup on [0, 1] of |fₙ(x) − f(x)| = xⁿ (for x ∈ [0, 1)) → sup_{x ∈ [0,1)} xⁿ. For any n, sup_{x < 1} xⁿ = 1 (take x arbitrarily close to 1). So the convergence is not uniform.

The moral: pointwise convergence of continuous functions may produce a discontinuous limit.

---

## 6.4.3 Uniform Convergence Preserves Continuity

**Theorem**: If fₙ → f uniformly on E and each fₙ is continuous at x₀ ∈ E, then f is continuous at x₀.

**Proof**: Write |f(x) − f(x₀)| ≤ |f(x) − fₙ(x)| + |fₙ(x) − fₙ(x₀)| + |fₙ(x₀) − f(x₀)|. For any ε > 0, choose n large enough so that the first and third terms are each < ε/3 (using uniform convergence). Then choose δ so that |x − x₀| < δ implies the middle term < ε/3 (using continuity of fₙ). □

**Corollary**: The uniform limit of a sequence of continuous functions is continuous.

---

## 6.4.4 Interchange of Limit and Integral

**Theorem**: If fₙ → f uniformly on [a, b] and each fₙ is integrable, then:

$$\int_a^b f(x) \, dx = \lim_{n \to \infty} \int_a^b f_n(x) \, dx$$

**Proof**: |∫f dx − ∫fₙ dx| ≤ ∫|f − fₙ| dx ≤ (b−a) · sup|f − fₙ| → 0. □

**Corollary** (Term-by-term integration of power series): If $\sum fₙ$ converges uniformly, the integral of the sum equals the sum of the integrals.

---

## 6.4.5 Interchange of Limit and Derivative

**Theorem**: If (fₙ) converges pointwise to f, and the derivatives f'ₙ converge uniformly to some function g, then f is differentiable and f' = g.

The conclusion is subtle: uniform convergence of the *derivatives* (not of the functions themselves) is what licenses term-by-term differentiation.

**Power series differentiation** (revisited): For a power series with radius R > 0, the differentiated series $\sum n cₙ (x-a)^{n-1}$ converges uniformly on any closed disk |x−a| ≤ r < R. Therefore, within the radius of convergence, differentiation term-by-term is valid.

---

## 6.4.6 The Weierstrass M-Test

**Theorem** (Weierstrass M-test): Let $\sum fₙ$ be a series of functions on E. If there exist constants Mₙ ≥ 0 such that |fₙ(x)| ≤ Mₙ for all x ∈ E and $\sum Mₙ < ∞$, then $\sum fₙ$ converges absolutely and uniformly on E.

*Proof*: For any ε > 0, choose N so that $\sum_{n>N} Mₙ < ε$. Then for all x and n > N: the tail sum $|\sum_{k>n} fₖ(x)| \leq \sum_{k>n} Mₖ \leq \sum_{k>N} Mₙ < ε$. So the partial sums form a Cauchy sequence uniformly in x. □

**Application**: For power series, Mₙ = |cₙ| rⁿ on the disk |x−a| ≤ r < R. Since $\sum |cₙ| rⁿ$ converges (by the definition of R), the power series converges uniformly on |x−a| ≤ r. This is the foundation for the term-by-term differentiation and integration results.

---

## 6.4.7 Connection to Physics

Uniform convergence is the precise condition that justifies interchanging limits with derivatives, integrals, and sums. In physics:

- **Perturbation theory**: physical quantities are expanded in power series in a small parameter (coupling constant, curvature, v/c). Term-by-term differentiation of these series requires uniform convergence. When the series only converges asymptotically (not absolutely), greater care is needed.

- **Fourier series**: a function f can be represented as $f(x) = \sum_{n} (a_n \cos nx + b_n \sin nx)$. The series converges uniformly if f is sufficiently smooth (e.g., continuously differentiable). Term-by-term differentiation of a Fourier series requires the series of derivatives to converge uniformly — a stronger condition on f.

- **Green's functions in GR**: the propagation of perturbations on a curved background is expressed as a series of modes. The convergence of this series, and the validity of term-by-term integration, requires exactly the uniform convergence conditions developed here.

---

## References

- Dirichlet, P.G.L. (1829). "Sur la convergence des séries trigonométriques qui servent à représenter une fonction arbitraire entre des limites données." *Journal für die reine und angewandte Mathematik*, 4, 157–169. [First rigorous result on Fourier series convergence, motivating the need for uniform convergence.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 7: Sequences and Series of Functions — the canonical treatment of uniform convergence.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapter 24 on uniform convergence; Spivak's counterexamples and geometric intuition are excellent.]
- Weierstrass, K. (1885). "Über die analytische Darstellbarkeit sogenannter willkürlicher Functionen einer reellen Veränderlichen." *Sitzungsberichte der Königlich Preußischen Akademie der Wissenschaften zu Berlin*. [Contains the Weierstrass M-test and the Weierstrass approximation theorem: every continuous function on [a,b] is uniformly approximable by polynomials.]
