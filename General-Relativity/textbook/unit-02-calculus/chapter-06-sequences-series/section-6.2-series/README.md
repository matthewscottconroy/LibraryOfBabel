# Section 6.2: Series and Convergence Tests

---

## Section Introduction

An infinite series $\sum_{n=1}^\infty a_n$ is the attempt to add infinitely many terms. The sum is defined as the limit of partial sums: $S_N = \sum_{n=1}^N a_n$, and we say the series converges to S if Sₙ → S. Otherwise it diverges.

The difficulty: whether a series converges depends delicately on the terms aₙ. The harmonic series $\sum 1/n$ diverges despite its terms → 0. The series $\sum 1/n^2$ converges despite its terms being only slightly smaller. Developing reliable tests for convergence is the purpose of this section.

---

## 6.2.1 Basic Tests

**Necessary condition for convergence**: If $\sum aₙ$ converges, then aₙ → 0.

*Proof*: aₙ = Sₙ − S_{n-1} → S − S = 0. □

**The converse is false**: aₙ → 0 does not imply convergence. The harmonic series $\sum 1/n$ diverges (Oresme's argument, ca. 1350): group terms as 1 + 1/2 + (1/3 + 1/4) + (1/5 + ... + 1/8) + .... Each group sums to more than 1/2, so the partial sums grow without bound.

**Geometric series**: $\sum_{n=0}^\infty r^n = 1/(1-r)$ for |r| < 1. *Proof*: Sₙ = (1 − r^{n+1})/(1−r) → 1/(1−r) since |r| < 1 means r^{n+1} → 0. For |r| ≥ 1, the terms do not → 0, so the series diverges.

The geometric series is fundamental. The identity 1/(1−r) = $\sum r^n$ is used constantly in physics: perturbation theory, propagators in quantum field theory, and the Schwarzschild metric's expansion in GM/rc² are all geometric series in disguise.

---

## 6.2.2 The Comparison Tests

**Comparison Test**: If 0 ≤ aₙ ≤ bₙ for all n ≥ N, then:
- If $\sum bₙ$ converges, so does $\sum aₙ$.
- If $\sum aₙ$ diverges, so does $\sum bₙ$.

*Proof*: The partial sums of $\sum aₙ$ are bounded above by $\sum bₙ < ∞$, so they form a bounded monotone sequence; by MCT, they converge. □

**Limit Comparison Test**: If aₙ, bₙ > 0 and lim aₙ/bₙ = L ∈ (0, ∞), then $\sum aₙ$ and $\sum bₙ$ converge or diverge together.

**p-series**: $\sum_{n=1}^\infty 1/n^p$ converges iff p > 1. *Proof* (integral test, below). The boundary case p = 1 (harmonic series) diverges. The case p = 2 converges to π²/6 (Euler's Basel problem).

---

## 6.2.3 The Integral Test

**Theorem** (Integral Test): If f is positive, continuous, and decreasing on [1, ∞), and aₙ = f(n), then $\sum_{n=1}^\infty aₙ$ and $\int_1^\infty f(x) dx$ converge or diverge together.

*Proof sketch*: Since f is decreasing: f(n+1) ≤ ∫ₙ^{n+1} f(x) dx ≤ f(n). Summing: $\sum_{n=2}^N f(n) \leq \int_1^N f(x) dx \leq \sum_{n=1}^{N-1} f(n)$. Convergence of one implies boundedness of the partial sums of the other. □

**p-series**: ∫₁^∞ x^{-p} dx converges iff p > 1. By the integral test, $\sum 1/n^p$ converges iff p > 1.

---

## 6.2.4 The Ratio and Root Tests

**Ratio Test**: Let L = lim |a_{n+1}/aₙ|. If L < 1, $\sum aₙ$ converges absolutely. If L > 1, it diverges. If L = 1, the test is inconclusive.

*Proof* (L < 1 case): Choose r with L < r < 1. For large n, |a_{n+1}| < r|aₙ|. So |aₙ| < Cr^n for some constant C. Since $\sum Cr^n = C/(1-r) < ∞$, comparison gives absolute convergence. □

**Root Test**: Let L = lim sup ⁿ√|aₙ|. Same conclusions as ratio test. The root test is more powerful (lim sup vs. lim), but the ratio test is usually easier to apply.

**Examples**:
- $\sum n!/n^n$: ratio = (n+1)!/(n+1)^{n+1} · n^n/n! = n^n/(n+1)^n = (1 + 1/n)^{-n} → e^{-1} < 1. Converges.
- $\sum n^n/n!$: ratio = (n+1)^{n+1}/(n+1)! · n!/n^n = (1+1/n)^n → e > 1. Diverges.

---

## 6.2.5 Alternating Series

**Alternating Series Test** (Leibniz, 1682): If (bₙ) is positive, decreasing, and bₙ → 0, then the alternating series $\sum (-1)^{n+1} bₙ = b₁ − b₂ + b₃ − ⋯$ converges.

*Proof*: The even partial sums S₂ₙ form an increasing sequence; the odd partial sums S₂ₙ₊₁ form a decreasing sequence; they bracket each other and differ by b_{2n+1} → 0. Both subsequences converge to the same limit. □

**Error bound**: |S − Sₙ| ≤ b_{n+1}. The error after n terms is bounded by the absolute value of the first omitted term.

**Example**: $\sum (-1)^{n+1}/n = 1 − 1/2 + 1/3 − 1/4 + ⋯ = \ln 2$. This is the alternating harmonic series. It converges by the alternating series test.

---

## 6.2.6 Absolute vs. Conditional Convergence

**Definition**: $\sum aₙ$ converges **absolutely** if $\sum |aₙ|$ converges. It converges **conditionally** if $\sum aₙ$ converges but $\sum |aₙ|$ diverges.

**Theorem**: Absolute convergence implies convergence.

*Proof*: aₙ = aₙ⁺ − aₙ⁻ where aₙ⁺ = max(aₙ, 0) and aₙ⁻ = max(−aₙ, 0). Both 0 ≤ aₙ⁺ ≤ |aₙ| and 0 ≤ aₙ⁻ ≤ |aₙ|. If $\sum |aₙ|$ converges, both $\sum aₙ⁺$ and $\sum aₙ⁻$ converge, and $\sum aₙ = \sum aₙ⁺ − \sum aₙ⁻$. □

**Riemann Rearrangement Theorem** (1854): If $\sum aₙ$ converges conditionally but not absolutely, then for any real number L (or ±∞), the terms can be rearranged so the series converges to L.

This is extraordinary. A conditionally convergent series can be rearranged to converge to *any* value. This is a property of the "cancellation" between positive and negative terms: both $\sum aₙ⁺ = +∞$ and $\sum aₙ⁻ = +∞$ (otherwise the series would converge absolutely or diverge). You can add the right balance of positive and negative terms to reach any target.

**Physical implication**: When summing a series in physics (e.g., a perturbative expansion), conditional convergence means the result depends on the order of summation — a potentially disastrous situation. Absolutely convergent series are rearrangement-invariant. In quantum field theory, the formal power series expansions (perturbative expansions in the coupling constant) are often not absolutely convergent — indeed, they often diverge for any nonzero coupling. This is the problem of "asymptotic series" and requires techniques beyond the scope of this chapter (Borel summation, Padé approximants).

---

## References

- Euler, L. (1748). *Introductio in analysin infinitorum*. Lausanne: Marc-Michel Bousquet. [Euler's systematic treatment of series, including the result Σ 1/n² = π²/6 (the Basel problem, solved 1734).]
- Oresme, N. (ca. 1350). *Quaestiones super geometriam Euclidis*. [First proof that the harmonic series diverges, by the grouping argument.]
- Riemann, B. (1854/1868). "Über die Darstellbarkeit einer Function durch eine trigonometrische Reihe." [Contains the rearrangement theorem.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 3 on series convergence tests.]
