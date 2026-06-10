# Chapter 6 Important Concepts

---

**Sequence** — A function from ℕ to ℝ: an ordered list a₁, a₂, a₃, .... Convergence of a sequence is the primary notion; all other limit concepts in analysis reduce to sequence limits.

**Convergence of a Sequence** — aₙ → L if ∀ε > 0 ∃N ∀n≥N: |aₙ − L| < ε. The limit is unique when it exists. Divergence means no such L exists.

**Cauchy Sequence** — A sequence where the terms become arbitrarily close to each other: ∀ε > 0 ∃N ∀m,n≥N: |aₘ − aₙ| < ε. In ℝ, Cauchy sequences are exactly the convergent sequences (Cauchy Criterion). This is a restatement of completeness of ℝ.

**Monotone Convergence Theorem (MCT)** — Every bounded monotone sequence converges. Requires completeness of ℝ. Fails over ℚ.

**Bolzano-Weierstrass Theorem** — Every bounded sequence in ℝ has a convergent subsequence. Equivalent to compactness of closed bounded intervals.

**Subsequence** — A sequence (a_{nₖ}) formed by choosing indices n₁ < n₂ < n₃ < ⋯. If aₙ → L, then every subsequence also → L. Conversely, if two subsequences converge to different limits, the sequence diverges.

**Lim Sup / Lim Inf** — The limit superior lim sup aₙ is the largest accumulation point of the sequence; lim inf is the smallest. A sequence converges iff lim sup = lim inf.

**Series** — The formal sum Σ aₙ; formally, the sequence of partial sums Sₙ = Σ_{k=1}^n aₖ. The series converges iff (Sₙ) converges.

**Necessary Condition for Convergence** — If Σ aₙ converges, then aₙ → 0. The converse fails (harmonic series).

**Geometric Series** — Σ rⁿ = 1/(1−r) for |r| < 1; diverges for |r| ≥ 1. Prototype of a convergent series.

**Harmonic Series** — Σ 1/n diverges. First proved by Nicole d'Oresme (ca. 1350) by grouping terms. The divergence is slow (Sₙ ≈ ln n + γ).

**p-Series** — Σ 1/n^p converges iff p > 1. Proved via the integral test. Special case p = 2 gives Σ 1/n² = π²/6 (Euler).

**Comparison Test** — If 0 ≤ aₙ ≤ bₙ and Σ bₙ converges, then Σ aₙ converges.

**Ratio Test** — lim |a_{n+1}/aₙ| < 1 ⟹ convergence; > 1 ⟹ divergence. Inconclusive at L = 1.

**Root Test** — lim sup |aₙ|^{1/n} < 1 ⟹ convergence; > 1 ⟹ divergence. More powerful than ratio test.

**Alternating Series Test** — Σ (−1)^n bₙ with bₙ decreasing to 0 converges. Error after N terms is bounded by b_{N+1}.

**Absolute Convergence** — Σ aₙ is absolutely convergent if Σ |aₙ| converges. Absolute convergence implies convergence; the converse fails.

**Conditional Convergence** — Converges but not absolutely. Example: alternating harmonic series. Conditionally convergent series can be rearranged to converge to any value (Riemann rearrangement theorem).

**Power Series** — A series of the form Σ cₙ(x−a)ⁿ. Converges absolutely on |x−a| < R (the radius of convergence) and diverges for |x−a| > R.

**Radius of Convergence** — R = 1/lim sup |cₙ|^{1/n} (Cauchy-Hadamard formula). If lim |c_{n+1}/cₙ| = L, then R = 1/L.

**Taylor Series** — The power series Σ f⁽ⁿ⁾(a)/n! · (x−a)ⁿ. A power series that converges to f is the Taylor series of f. Not every smooth function equals its Taylor series (e.g., e^{-1/x²}).

**Analytic Function** — A function equal to its Taylor series in a neighborhood of every point. All elementary functions are analytic on their domains.

**Euler's Formula** — e^{iθ} = cos θ + i sin θ. Connects exponential and trigonometric functions via the complex numbers. The most important formula in applied mathematics.

**Euler's Identity** — e^{iπ} + 1 = 0. Special case θ = π.

**Pointwise Convergence** — fₙ → f pointwise if for each x, fₙ(x) → f(x). The threshold N may depend on x.

**Uniform Convergence** — fₙ → f uniformly if a single N works for all x: ∀ε ∃N ∀x ∀n≥N: |fₙ(x) − f(x)| < ε. Uniform convergence preserves continuity, commutes with integration, and (when applied to derivatives) commutes with differentiation.

**Weierstrass M-Test** — If |fₙ(x)| ≤ Mₙ and Σ Mₙ < ∞, then Σ fₙ converges absolutely and uniformly. The standard tool for proving uniform convergence of power series.

**Asymptotic Series** — A formal power series that diverges but whose partial sums provide increasingly accurate approximations up to a point. Common in perturbative physics (QED, post-Newtonian GR). The series is useful even without convergence.

**Euler-Mascheroni Constant** — γ = lim_{n→∞} (Σ_{k=1}^n 1/k − ln n) ≈ 0.5772. Characterizes the growth of partial sums of the harmonic series. Appears in the Gamma function: Γ'(1) = −γ.
