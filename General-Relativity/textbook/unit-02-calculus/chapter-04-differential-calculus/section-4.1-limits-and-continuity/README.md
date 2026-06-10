# Section 4.1: Limits and Continuity

---

## Section Introduction

The limit is the cornerstone of calculus. Every concept in analysis — the derivative, the integral, convergence of series, continuity — is defined in terms of limits. Getting limits right is not merely a matter of precision for its own sake; it is the key to understanding *why* calculus works and where it can fail.

The intuitive idea of a limit is this: we say f(x) approaches L as x approaches a if f(x) gets arbitrarily close to L whenever x is sufficiently close to (but not equal to) a. The ε-δ definition makes this precise: "arbitrarily close" means "within ε for any ε > 0," and "sufficiently close" means "within some δ > 0."

This may seem like a lot of machinery to formalize an intuition. But the formalism is what allows us to prove things — to establish that limits exist or do not, to compute them reliably, to build all of calculus on a secure foundation.

---

## 4.1.1 The Epsilon-Delta Definition of a Limit

**Definition** (Weierstrass, ca. 1861; Cauchy, 1821 in spirit): Let f be a function defined on an open interval containing a (but f need not be defined at a itself). We say the **limit of f(x) as x approaches a is L**, written:

$$\lim_{x \to a} f(x) = L$$

if:

$$\forall \varepsilon > 0 \; \exists \delta > 0 \; \forall x \in \mathbb{R}: 0 < |x - a| < \delta \implies |f(x) - L| < \varepsilon$$

Let us parse this definition carefully:
- **∀ε > 0**: for any required precision ε, no matter how small,
- **∃δ > 0**: we can find a window δ around a,
- **∀x: 0 < |x - a| < δ**: such that for all x within that window (and x ≠ a),
- **|f(x) - L| < ε**: the function value f(x) is within ε of L.

The condition 0 < |x - a| explicitly excludes x = a. This is important: the limit of f as x → a depends only on the behavior of f *near* a, not at a. The function need not even be defined at a.

**The adversarial game**: The ε-δ definition has a natural adversarial interpretation. Your opponent (the "ε-provider") gets to choose any positive ε. You (the "δ-responder") must respond with a δ that works. If you can always respond successfully — for every ε, no matter how small — then the limit exists and equals L.

---

## 4.1.2 Proving Limits from the Definition

**Example 1**: Prove that lim_{x→3} (2x + 1) = 7.

**Analysis** (scratch work): We need |f(x) - L| = |(2x + 1) - 7| = |2x - 6| = 2|x - 3| < ε. This holds when |x - 3| < ε/2. So we should take δ = ε/2.

**Proof**: Let ε > 0 be given. Set δ = ε/2. Suppose 0 < |x - 3| < δ. Then:

$$|(2x + 1) - 7| = |2x - 6| = 2|x - 3| < 2\delta = 2 \cdot \frac{\varepsilon}{2} = \varepsilon. \quad \square$$

**Example 2**: Prove that lim_{x→2} x² = 4.

**Analysis**: We need |x² - 4| = |x - 2||x + 2| < ε. The factor |x + 2| is problematic — it depends on x. But if we restrict to |x - 2| < 1 (i.e., 1 < x < 3), then |x + 2| < 5. So |x² - 4| < 5|x - 2| < ε when |x - 2| < ε/5. Take δ = min(1, ε/5).

**Proof**: Let ε > 0. Set δ = min(1, ε/5). Suppose 0 < |x - 2| < δ. Since δ ≤ 1, we have |x - 2| < 1, so 1 < x < 3 and |x + 2| < 5. Then:

$$|x^2 - 4| = |x - 2||x + 2| < \delta \cdot 5 \leq \frac{\varepsilon}{5} \cdot 5 = \varepsilon. \quad \square$$

The "min" trick is standard: we use one constraint (|x - 2| < 1) to bound |x + 2|, and another (|x - 2| < ε/5) to get the final estimate.

---

## 4.1.3 Limit Laws

Once limits are defined, we can prove general rules for computing them. These rules reduce limit computation to a mechanical process in most cases.

**Theorem** (Limit Laws): Suppose lim_{x→a} f(x) = L and lim_{x→a} g(x) = M. Then:

1. **Sum**: lim_{x→a} [f(x) + g(x)] = L + M
2. **Difference**: lim_{x→a} [f(x) - g(x)] = L - M
3. **Product**: lim_{x→a} [f(x)·g(x)] = L·M
4. **Quotient**: lim_{x→a} [f(x)/g(x)] = L/M, provided M ≠ 0
5. **Scalar**: lim_{x→a} [cf(x)] = cL for any constant c
6. **Composition**: lim_{x→a} f(g(x)) = f(M), provided f is continuous at M

**Proof of the Sum Law** (all others are similar):

Let ε > 0. Since lim f(x) = L, ∃δ₁ > 0 such that 0 < |x-a| < δ₁ ⟹ |f(x) - L| < ε/2.
Since lim g(x) = M, ∃δ₂ > 0 such that 0 < |x-a| < δ₂ ⟹ |g(x) - M| < ε/2.

Set δ = min(δ₁, δ₂). If 0 < |x-a| < δ, then:

$$|[f(x)+g(x)] - [L+M]| = |[f(x)-L] + [g(x)-M]| \leq |f(x)-L| + |g(x)-M| < \frac{\varepsilon}{2} + \frac{\varepsilon}{2} = \varepsilon. \quad \square$$

The triangle inequality did the essential work.

---

## 4.1.4 One-Sided Limits and Limits at Infinity

**One-sided limits**: 
- lim_{x→a⁺} f(x) = L: limit from the right — only x > a are considered.
- lim_{x→a⁻} f(x) = L: limit from the left — only x < a are considered.

lim_{x→a} f(x) exists iff both one-sided limits exist and are equal.

**Limits at infinity**:
- lim_{x→∞} f(x) = L: for every ε > 0, ∃M > 0 such that x > M ⟹ |f(x) - L| < ε.

These extend the ε-δ definition in natural ways.

---

## 4.1.5 Continuity

**Definition**: A function f is **continuous at a** if:
1. f is defined at a
2. lim_{x→a} f(x) exists
3. lim_{x→a} f(x) = f(a)

Equivalently (in ε-δ language): ∀ε > 0 ∃δ > 0 ∀x: |x - a| < δ ⟹ |f(x) - f(a)| < ε.

(Note: continuity allows x = a, so the condition |x - a| < δ does not exclude x = a, unlike the limit definition which uses 0 < |x - a|.)

A function is **continuous on an interval** if it is continuous at every point of the interval.

**Examples**:
- All polynomials are continuous everywhere.
- Rational functions p(x)/q(x) are continuous wherever q(x) ≠ 0.
- sin, cos, eˣ, ln x are continuous on their domains.
- |x| is continuous everywhere.
- The function f(x) = sin(1/x) has no limit as x → 0 (it oscillates infinitely fast).
- The function f(x) = x sin(1/x) is continuous if we define f(0) = 0 (this is verified by the squeeze theorem).

---

## 4.1.6 The Intermediate Value Theorem

**Theorem** (IVT): Let f be continuous on the closed interval [a, b]. If f(a) < c < f(b) (or f(b) < c < f(a)), then there exists some x₀ ∈ (a, b) such that f(x₀) = c.

*Proof requires completeness of ℝ — it fails over ℚ.* The standard proof uses the least upper bound property: let S = {x ∈ [a, b] : f(x) < c} and show that x₀ = sup(S) satisfies f(x₀) = c.

**Physical application**: The IVT guarantees existence of roots of equations. In GR, such existence arguments appear in the proof that the geodesic equation has solutions (via the Picard-Lindelöf theorem), in the existence of trapped surfaces (Penrose singularity theorem), and in many other contexts. The underlying logic is always the same: a continuous function that takes values on both sides of a threshold must cross it.

---

## 4.1.7 The Extreme Value Theorem

**Theorem** (EVT): Let f be continuous on the closed interval [a, b]. Then f achieves its maximum and minimum on [a, b]: there exist x_max, x_min ∈ [a, b] with:

$$f(x_{\min}) \leq f(x) \leq f(x_{\max}) \quad \forall x \in [a, b]$$

*Also requires completeness.* The proof uses compactness of [a, b] (as a closed, bounded subset of ℝ), a concept we generalize in Chapter 13.

---

## 4.1.8 The Squeeze Theorem

**Theorem** (Squeeze / Sandwich): If g(x) ≤ f(x) ≤ h(x) near a, and lim_{x→a} g(x) = lim_{x→a} h(x) = L, then lim_{x→a} f(x) = L.

**Example**: lim_{x→0} x² sin(1/x) = 0. Proof: |x² sin(1/x)| ≤ x², and lim_{x→0} x² = 0. By the squeeze theorem, the limit is 0.

---

## References

- Cauchy, A.L. (1821). *Cours d'analyse de l'École Royale Polytechnique*. Paris: de Bure. [First systematic treatment of limits, though without the full ε-δ apparatus.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 4 on limits and continuity; the gold standard for rigor.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapter 5 on limits; beautifully written and fully rigorous.]
- Apostol, T.M. (1967). *Calculus*, Vol. 1, 2nd ed. Wiley. [Chapter 3.]
