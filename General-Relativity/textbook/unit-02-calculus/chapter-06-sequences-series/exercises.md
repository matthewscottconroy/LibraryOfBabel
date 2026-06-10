# Chapter 6 Exercises: Sequences, Series, and Approximation

---

## Section 6.1: Sequences

**Exercise 6.1.1** *(ε-N proofs)*  
Prove from the definition that:
- (a) lim_{n→∞} n/(n+1) = 1
- (b) lim_{n→∞} (−1)^n/n = 0
- (c) lim_{n→∞} (2n² + 3n)/(5n² − 1) = 2/5

**Exercise 6.1.2** *(recursion and MCT)*  
Define a₁ = 2 and a_{n+1} = (aₙ + 6/aₙ)/2.
- (a) Show that if aₙ > √6, then a_{n+1} > √6 (the sequence stays above √6).
- (b) Show the sequence is decreasing: a_{n+1} < aₙ for aₙ > √6.
- (c) Conclude by MCT that the sequence converges, and find its limit.
- (d) Compare: the sequence is a variant of Newton's method for computing √6. In general, Newton's method for √c starts with a₁ = c and uses a_{n+1} = (aₙ + c/aₙ)/2. Write a brief proof that this converges to √c for any c > 0 and any a₁ > 0.

**Exercise 6.1.3** *(Cauchy and completeness)*  
Let aₙ = Σ_{k=1}^n (−1)^k/k (the partial sums of the alternating harmonic series).
- (a) Prove directly that (aₙ) is a Cauchy sequence. (For n > m: |aₙ − aₘ| = |Σ_{k=m+1}^n (−1)^k/k| ≤ 1/(m+1). This uses the alternating series error bound.)
- (b) By the Cauchy criterion, the sequence converges. Its limit is ln 2. Accept this and use it to evaluate: what does Σ_{n=1}^∞ (−1)^{n+1}/n equal?
- (c) Define bₙ = 1 + 1/2 + 1/3 + ⋯ + 1/n − ln n. Prove bₙ is decreasing and bounded below. Conclude bₙ → γ for some constant γ (the Euler-Mascheroni constant, γ ≈ 0.5772). (This requires: 1/n < ln(n/(n−1)) for the monotone part.)

**Exercise 6.1.4** *(limsup and liminf)*  
Let aₙ = sin(nπ/3).
- (a) List the first 12 terms and observe the periodicity.
- (b) Compute lim sup aₙ and lim inf aₙ.
- (c) Does the sequence converge? Does any subsequence converge to 1? To −1/2?
- (d) State the Bolzano-Weierstrass theorem and verify that it applies here.

---

## Section 6.2: Series

**Exercise 6.2.1** *(convergence tests — systematic)*  
Determine whether each series converges or diverges. State the test used and verify its conditions.
- (a) Σ n/2ⁿ
- (b) Σ n!/nⁿ
- (c) Σ 1/(n ln n)
- (d) Σ (−1)^n / √n
- (e) Σ sin(1/n²)
- (f) Σ n² e^{−n}

**Exercise 6.2.2** *(the harmonic series — multiple proofs)*  
Prove the harmonic series Σ 1/n diverges by three different methods:
- (a) Oresme's grouping argument.
- (b) The integral test: compare with ∫₁^∞ 1/x dx = ∞.
- (c) Assuming it converges to S: show S = 1 + 1/2 + 1/3 + ⋯ = (1 + 1/3 + 1/5 + ⋯) + (1/2 + 1/4 + ⋯) > (1/2 + 1/4 + ⋯) + (1/2 + 1/4 + ⋯) = S. Contradiction.

**Exercise 6.2.3** *(Riemann rearrangement)*  
The alternating harmonic series $\sum (-1)^{n+1}/n = \ln 2 ≈ 0.693$.

- (a) Rearrange the terms to obtain a partial sum exceeding 2 as follows: take positive terms until the partial sum exceeds 2, then take one negative term, then positive terms until the partial sum again exceeds 2, etc. Trace the first 20 or so terms of this rearrangement.
- (b) Prove rigorously that this rearrangement converges to 2. (The key: both the positive and negative subseries diverge, so you can always "top up" and "shave down" to any target.)
- (c) What does this say about rearranging the terms of an absolutely convergent series?

---

## Section 6.3: Power Series

**Exercise 6.3.1** *(radii of convergence)*  
Find the radius and interval of convergence (including endpoint behavior) of each power series:
- (a) Σ xⁿ/n!
- (b) Σ n xⁿ
- (c) Σ (x−2)ⁿ / √n
- (d) Σ (2n)! xⁿ / (n!)²

**Exercise 6.3.2** *(deriving series from known ones)*  
Using known Taylor series and operations (differentiation, integration, substitution), derive the Taylor series for:
- (a) 1/(1−x)² (differentiate 1/(1−x) = Σ xⁿ term by term)
- (b) arctan x (integrate 1/(1+x²) = Σ (−x²)ⁿ term by term)
- (c) The series for arctan x evaluated at x = 1 gives the **Leibniz formula**: π/4 = 1 − 1/3 + 1/5 − 1/7 + ⋯. Prove that the Taylor series for arctan x is valid at x = 1 (Abel's theorem says it suffices that the series converges at x = 1 — check by alternating series test).
- (d) Derive the first four terms of the series for sec x by computing d/dx tan x = sec²x and integrating appropriately.

**Exercise 6.3.3** *(the binomial series in GR)*  
The binomial series $(1+x)^\alpha = \sum_{n=0}^\infty \binom{\alpha}{n} x^n$ converges for |x| < 1.

- (a) Compute the first four terms for α = 1/2 (giving the series for √(1+x)).
- (b) Compute the first four terms for α = −1/2.
- (c) In the Schwarzschild metric, time dilation gives dt_proper/dt_coord = √(1 − 2GM/rc²) for a stationary observer. For weak fields (ε = 2GM/rc² << 1), expand to second order in ε. What is the leading-order time dilation, and what is the correction?
- (d) In special relativity, the Lorentz factor γ = (1 − v²/c²)^{-1/2}. Expand to second order in v/c. The leading correction to kinetic energy is mv²/2 (Newtonian) — what is the next term?

---

## Section 6.4: Uniform Convergence

**Exercise 6.4.1** *(testing for uniform convergence)*  
Determine whether each sequence converges uniformly on the given domain. Prove your answers.
- (a) fₙ(x) = x/n on ℝ
- (b) fₙ(x) = xⁿ on [0, 1)
- (c) fₙ(x) = xⁿ on [0, r] for any fixed r < 1
- (d) fₙ(x) = nx/(1+n²x²) on ℝ

**Exercise 6.4.2** *(failure of interchange)*  
Define fₙ(x) = n²x e^{-nx} on [0, 1].
- (a) Show fₙ → 0 pointwise on [0, 1].
- (b) Compute ∫₀¹ fₙ(x) dx (use integration by parts or the substitution u = nx).
- (c) Show that lim_{n→∞} ∫₀¹ fₙ dx ≠ ∫₀¹ lim fₙ dx. Explain why this does not contradict the theorem on uniform convergence and integrals.
- (d) Is the convergence uniform? (Check whether sup_{x ∈ [0,1]} |fₙ(x)| → 0.)

**Exercise 6.4.3** *(the Weierstrass approximation theorem)*  
The Weierstrass approximation theorem states: for any continuous f: [a,b] → ℝ and any ε > 0, there exists a polynomial p such that |f(x) − p(x)| < ε for all x ∈ [a,b]. In other words, continuous functions on closed intervals are uniformly approximable by polynomials.

- (a) Use this to show that if ∫ₐᵇ f(x) xⁿ dx = 0 for all n ≥ 0 and f is continuous, then f = 0. (Approximate f by polynomials and show ∫ f² = 0.)
- (b) This exercise has immediate application: in GR, if a symmetric tensor field has zero contraction with all polynomial test functions, it must be the zero tensor. Explain why this follows from the same logic.

---

## Thought Experiments

**Thought Experiment 6.1** *(does order of summation matter?)*  
Consider the double sum $\sum_{m=1}^\infty \sum_{n=1}^\infty a_{mn}$. In general, the sum along rows may differ from the sum along columns, and both may differ from summing along diagonals. When are these all equal?

The answer (Fubini's theorem for series): if $\sum_{m,n} |a_{mn}| < \infty$, then all orderings of summation agree. This is the series version of Fubini's theorem for integrals (Chapter 7). In quantum field theory, the interchange of multiple integrals (over loop momenta) requires exactly this condition — and when it fails, renormalization is needed.

**Thought Experiment 6.2** *(asymptotic series)*  
Not all useful series are convergent. An **asymptotic series** is a formal power series $\sum aₙ xⁿ$ (in 1/x as x → ∞, say) that *diverges* for every nonzero x, yet truncated to N terms gives an approximation with error O(x^{-N-1}). That is: the partial sum is a better and better approximation as we take more terms, up to a point — then the error grows again.

The perturbative expansion in quantum electrodynamics (QED) is believed to be asymptotic but not convergent. The expansion in the fine structure constant α ≈ 1/137 gives extraordinarily precise predictions (e.g., the electron magnetic moment agreed with experiment to 12 decimal places) but the series itself diverges for every nonzero α.

What does this mean philosophically? Can a divergent series be physically meaningful? What criteria — mathematical or physical — should we demand of a theoretical prediction?

**Thought Experiment 6.3** *(Fourier series and spacetime)*  
A function on a circle can be decomposed into Fourier modes: f(θ) = Σ cₙ e^{inθ}. Each mode is a wave with wavelength 2π/n. The coefficients cₙ determine the "frequency content" of f.

In GR, perturbations of a black hole can be decomposed into quasi-normal modes — the black hole's "ringing" frequencies after it absorbs a perturbation. These are complex frequencies: the real part is the oscillation frequency, the imaginary part is the decay rate. Just as a Fourier series decomposes a function into simple oscillations, the quasi-normal mode expansion decomposes a perturbation into simple decaying modes.

But the quasi-normal modes do not form a complete basis — they are not enough to reconstruct arbitrary initial data. This is one way black holes are fundamentally different from bounded Euclidean regions. Explain in your own words what the mathematical content of this statement is, using the concepts from this chapter.

---

## Laboratory Projects

**Lab 6.1** *(convergence in practice)*  
For the series Σ 1/n^p on [1, ∞), the partial sum Sₙ = Σ_{k=1}^n 1/k^p converges for p > 1.

- For p = 2: compute Sₙ for n = 10, 100, 1000, 10000. The exact limit is π²/6 ≈ 1.6449.
- How fast does Sₙ → π²/6? What is the error |π²/6 − Sₙ|? Compare with the integral estimate: ∫_n^∞ x^{-2} dx = 1/n.
- For p = 1.1 (barely convergent): compute Sₙ for n = 10^6. How close are you to the limit (approximately 10.6)? How many terms do you need to achieve 3 decimal places of accuracy?
- Conclude: "convergent" does not mean "practically computable in finite time." Convergence rate matters.

**Lab 6.2** *(Euler's formula and waves)*  
Implement the complex exponential e^{iθ} numerically as $\sum_{n=0}^N (i\theta)^n/n!$ for N = 1, 2, 5, 10, 20.

- Plot the partial sum in the complex plane as a curve (x = Re[S_N(θ)], y = Im[S_N(θ)] for θ ∈ [0, 2π]).
- As N increases, the curve approaches the unit circle.
- Demonstrate that |e^{iθ}| = 1 for all θ by computing |S_N(θ)| numerically for large N.
- Use Euler's formula to verify De Moivre's theorem: (e^{iθ})^n = e^{inθ} = cos(nθ) + i sin(nθ). Verify numerically for n = 3 and θ = π/6.

**Lab 6.3** *(the Weierstrass function and fractals)*  
Define W_N(x) = Σ_{n=0}^N (1/2)^n cos(3^n π x).

- Plot W_N on [0, 2] for N = 1, 2, 5, 10, 15.
- Estimate the "box-counting dimension" of the graph of W_15 by covering it with boxes of side ε and counting how N(ε) scales with ε. For the Weierstrass function, the fractal dimension is theoretically between 1 and 2.
- The function is continuous everywhere (by Weierstrass M-test: |W_N(x)| ≤ Σ (1/2)^n = 2) but not differentiable anywhere. Estimate the "modulus of continuity" — how does |W(x+h) − W(x)| scale with h? — and verify it is continuous but not Lipschitz.
- This connects uniform convergence (which proves continuity) to the non-existence of derivatives (which requires non-uniform estimates). The Weierstrass function is the prototype for Brownian motion paths in physics.
