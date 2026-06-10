# Section 12.2: Cauchy's Theorem and the Cauchy Integral Formula

---

## Section Introduction

Cauchy's theorem is the central result of complex analysis: the integral of a holomorphic function around a closed contour is zero. This theorem seems, at first, like a minor computational convenience. It is in fact one of the most powerful results in all of mathematics. From it flows the Cauchy integral formula (the value of f at any point is determined by its values on any surrounding contour), the fact that holomorphic functions are analytic (have convergent Taylor series), Liouville's theorem (bounded entire functions are constant), and the fundamental theorem of algebra.

In the language of differential forms, Cauchy's theorem is the statement that the 1-form f(z)dz is closed (its exterior derivative vanishes) if and only if f is holomorphic — and on a simply connected domain, closed forms are exact, so the integral over any closed contour is zero. This connects complex analysis directly to the de Rham cohomology and the topology of the complex plane minus its singularities.

---

## 12.2.1 Complex Line Integrals

**Definition**: For a smooth curve γ: [a, b] → ℂ and a continuous function f: ℂ → ℂ, the **contour integral** is:

$$\int_\gamma f(z) \, dz = \int_a^b f(\gamma(t)) \gamma'(t) \, dt$$

where γ'(t) is the complex derivative of γ (the velocity of the curve in ℂ).

**Properties**:
- Linearity: ∫_γ (αf + βg) dz = α∫f dz + β∫g dz.
- Orientation: reversing γ multiplies the integral by −1.
- The **ML estimate**: |∫_γ f dz| ≤ max|f| · L(γ), where L(γ) = ∫|γ'(t)|dt is the arc length of γ.

**Example**: Compute ∫_{|z|=r} z^n dz for integer n.

Parametrize: z = re^{iθ}, dz = ire^{iθ}dθ, θ ∈ [0, 2π].

$$\int_{|z|=r} z^n \, dz = \int_0^{2\pi} r^n e^{in\theta} \cdot ire^{i\theta} \, d\theta = ir^{n+1}\int_0^{2\pi} e^{i(n+1)\theta} \, d\theta$$

For n ≠ −1: the integral is $ir^{n+1} \cdot [e^{i(n+1)\theta}/(i(n+1))]_0^{2\pi} = 0$ (since e^{2πi(n+1)} = 1).

For n = −1: the integral is $i \int_0^{2\pi} d\theta = 2\pi i$.

**Key fact**: $\oint_{|z|=r} z^{-1} dz = 2\pi i$ — this is the fundamental integral of complex analysis. All residue calculations ultimately reduce to this.

---

## 12.2.2 Cauchy's Theorem

**Theorem** (Cauchy, 1825): If f is holomorphic on a simply connected domain U ⊂ ℂ, and γ is any closed curve in U, then:

$$\oint_\gamma f(z) \, dz = 0$$

*Proof*: Writing f(z)dz = f(x+iy)(dx+idy) = (u+iv)(dx+idy) = (u dx − v dy) + i(v dx + u dy), we get two real line integrals. Apply Green's theorem to each:

$$\oint (u\,dx - v\,dy) = \iint_D \left(-\frac{\partial v}{\partial x} - \frac{\partial u}{\partial y}\right) dA = 0$$

by the Cauchy-Riemann equations (∂u/∂y = −∂v/∂x). Similarly for the imaginary part. □

**The role of simple connectivity**: On ℂ \ {0}, the function f(z) = 1/z is holomorphic everywhere except the origin. The integral around a contour encircling the origin gives 2πi ≠ 0. Simple connectivity prevents contours from encircling "holes."

**In the language of forms**: f(z)dz is a complex 1-form on ℂ. Its exterior derivative is d(f\,dz) = (∂f/∂x̄) dz̄ ∧ dz where ∂/∂z̄ = (1/2)(∂/∂x + i∂/∂y) is the Cauchy-Riemann operator. Cauchy's theorem states: f is holomorphic (∂f/∂z̄ = 0) iff d(f\,dz) = 0 — the form is closed. On a simply connected domain, closed → exact → zero integral on all closed curves.

This is precisely the Poincaré lemma: H¹(ℂ) = 0 (the first de Rham cohomology of ℂ vanishes). On ℂ \ {0}, H¹(ℂ\{0}) ≅ ℝ (one generator: the class of dz/z), accounting for the non-zero integral around the origin.

---

## 12.2.3 The Cauchy Integral Formula

**Theorem** (Cauchy Integral Formula): If f is holomorphic on a domain containing a simple closed contour γ and its interior D, and z₀ is inside γ, then:

$$f(z_0) = \frac{1}{2\pi i} \oint_\gamma \frac{f(z)}{z - z_0} \, dz$$

*Proof*: Apply Cauchy's theorem to f(z)/(z − z₀) on D minus a small disk B_ε(z₀). Since f(z)/(z − z₀) is holomorphic on D \ {z₀}, Cauchy's theorem gives:

$$\oint_\gamma \frac{f(z)}{z-z_0} dz = \oint_{|z-z_0|=\varepsilon} \frac{f(z)}{z-z_0} dz$$

The right side equals $\oint_{|z-z_0|=\varepsilon} \frac{f(z₀)}{z-z₀} dz + \oint_{|z-z_0|=\varepsilon} \frac{f(z)-f(z_0)}{z-z_0} dz$. The first integral is $f(z₀) \cdot 2\pi i$ (from the example above). The second integral → 0 as ε → 0 (since |f(z)−f(z₀)|/|z−z₀| is bounded by the ML estimate). □

**Remarkable consequences**:
1. **f is determined by its boundary values**: The value of f at any interior point is a weighted average of its boundary values. A holomorphic function on a disk is completely determined by its values on the bounding circle.

2. **f is infinitely differentiable**: Differentiating the integral formula:

$$f^{(n)}(z_0) = \frac{n!}{2\pi i} \oint_\gamma \frac{f(z)}{(z - z_0)^{n+1}} \, dz$$

So every holomorphic function has derivatives of all orders — a property that fails dramatically for real-differentiable functions (e.g., f(x) = |x|² is differentiable once but not twice in a neighborhood of 0 as a real function of two real variables).

3. **f is analytic**: Since the Taylor coefficients can be bounded using the above, the Taylor series of f converges in any disk free of singularities. In complex analysis, differentiable = analytic — these two concepts, distinct for real functions, coincide.

---

## 12.2.4 Liouville's Theorem and the Fundamental Theorem of Algebra

**Liouville's Theorem** (1844): Every bounded entire function (holomorphic on all of ℂ) is constant.

*Proof*: For a bounded entire f (|f(z)| ≤ M for all z), the Cauchy integral formula for f'(z₀) with a contour of radius R gives:

$$|f'(z_0)| = \left|\frac{1}{2\pi i} \oint_{|z-z_0|=R} \frac{f(z)}{(z-z_0)^2} dz\right| \leq \frac{M}{R}$$

As R → ∞, |f'(z₀)| → 0, so f' = 0 everywhere, hence f is constant. □

**Fundamental Theorem of Algebra**: Every non-constant polynomial p(z) with complex coefficients has at least one complex root.

*Proof* (via Liouville): If p has no roots, then 1/p(z) is entire. For large |z|, |p(z)| → ∞, so 1/p is bounded. By Liouville's theorem, 1/p is constant, contradicting that p is non-constant. □

This proof — using complex analysis to establish a result about polynomials — is characteristic of the power of holomorphicity. The polynomial is real or complex, the result is algebraic, and the proof is analytic. This interplay between algebra, analysis, and topology is a hallmark of complex analysis.

---

## 12.2.5 Morera's Theorem and Holomorphic Extensions

**Morera's Theorem** (converse to Cauchy): If f is continuous on a domain U and $\oint_\gamma f\,dz = 0$ for every closed triangle in U, then f is holomorphic.

*Proof*: Define F(z) = ∫_{z₀}^z f(w)dw (line integral, well-defined because it's path-independent by the hypothesis). Then F'(z) = f(z) (by the fundamental theorem of calculus for complex integrals), so F is holomorphic, hence has all derivatives, and in particular F' = f is holomorphic. □

Morera's theorem is useful for proving holomorphicity of functions defined by integrals — as in the definition of the Gamma function, the Riemann zeta function, and various special functions.

**Analytic continuation** (elaborated from Section 12.3): By the **identity theorem**, if f and g are holomorphic on a connected domain U and agree on a set with a limit point in U, then f = g on all of U. This means holomorphic functions cannot be "locally modified" — a holomorphic function on a disk is determined by its values on any convergent sequence inside.

The **monodromy theorem**: If f can be analytically continued along every path in a simply connected domain U, then all such continuations agree, giving a single well-defined holomorphic function on U. On non-simply connected domains, analytic continuation along different paths may give different values — producing a **multi-valued function** (like √z or log z).

**GR application**: The analytic continuation of the Schwarzschild metric through the coordinate singularity at r = r_s (the horizon) — performed by Kruskal (1960) using the coordinate transformation u = (r/r_s − 1)^{1/2} e^{r/(2r_s)} cosh(t/(2r_s)), v = (r/r_s − 1)^{1/2} e^{r/(2r_s)} sinh(t/(2r_s)) — is an analytic extension in the sense of complex analysis. The maximal analytic extension of the Schwarzschild metric is the Kruskal-Szekeres spacetime. [Kruskal, M.D. (1960). "Maximal extension of Schwarzschild metric." *Physical Review*, 119, 1743–1745.]

---

## References

- Cauchy, A.L. (1825). "Mémoire sur les intégrales définies, prises entre des limites imaginaires." Reprinted in *Bulletin des sciences mathématiques*, 7 (1874), 265–304. [The foundational paper for complex integration and Cauchy's theorem.]
- Liouville, J. (1844). "Leçons sur les fonctions doublement périodiques." *Journal de Mathématiques Pures et Appliquées*, 88. [Contains Liouville's theorem, in the context of elliptic functions.]
- Ahlfors, L.V. (1979). *Complex Analysis*, 3rd ed. McGraw-Hill. [The classic graduate text; Chapters 4–5 on integration and Cauchy's theorem; rigorous and elegant.]
- Kruskal, M.D. (1960). "Maximal extension of Schwarzschild metric." *Physical Review*, 119, 1743–1745. [The analytic extension of Schwarzschild through the horizon — a direct application of analytic continuation.]
- Conway, J.B. (1978). *Functions of One Complex Variable*, 2nd ed. Springer. [Chapters 4–5: Cauchy's theorem, integral formula, and their consequences; rigorous with good examples.]
