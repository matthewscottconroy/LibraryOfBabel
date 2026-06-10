# Section 6.1: Sequences and Their Limits

---

## Section Introduction

A sequence in ℝ is a function from ℕ to ℝ: to each natural number n, we assign a real number aₙ. We write the sequence as (aₙ) or (a₁, a₂, a₃, ...). The central question: does aₙ approach a limit L as n → ∞?

---

## 6.1.1 Convergence of Sequences

**Definition**: A sequence (aₙ) **converges to L** if:

$$\forall \varepsilon > 0 \; \exists N \in \mathbb{N} \; \forall n \geq N: |a_n - L| < \varepsilon$$

We write aₙ → L or lim_{n→∞} aₙ = L. If no such L exists, the sequence **diverges**.

This is the same ε-δ formalism as for functions, with N playing the role of 1/δ: for any required precision ε, we can find a "tail" of the sequence (all terms beyond N) that lies within ε of L.

**Uniqueness of limits**: A sequence can have at most one limit. *Proof*: Suppose aₙ → L and aₙ → M. For any ε > 0, for large enough n: |aₙ − L| < ε/2 and |aₙ − M| < ε/2. Then |L − M| ≤ |L − aₙ| + |aₙ − M| < ε. Since ε is arbitrary, L = M. □

**Examples**:
- aₙ = 1/n → 0 (proof: given ε > 0, take N > 1/ε; then n ≥ N implies 1/n ≤ 1/N < ε).
- aₙ = (n+1)/n = 1 + 1/n → 1.
- aₙ = (−1)ⁿ diverges (oscillates between +1 and −1; no limit).
- aₙ = n → ∞ (diverges; not bounded).
- aₙ = (1 + 1/n)ⁿ → e. This is, by definition in one standard treatment, the definition of e.

**Algebra of limits**: If aₙ → L and bₙ → M, then:
- aₙ + bₙ → L + M
- aₙ · bₙ → L · M
- aₙ / bₙ → L/M (if M ≠ 0 and bₙ ≠ 0 for all n)

These follow from the same limit laws as for functions (Section 4.1.3), with the same proofs.

---

## 6.1.2 The Monotone Convergence Theorem

**Theorem** (MCT): Every bounded monotone sequence converges.

More precisely: if (aₙ) is non-decreasing (a₁ ≤ a₂ ≤ a₃ ≤ ...) and bounded above (aₙ ≤ M for all n), then (aₙ) converges to L = sup{aₙ : n ∈ ℕ}.

**Proof**: Let L = sup{aₙ}. This exists by completeness of ℝ. For any ε > 0, L − ε is not an upper bound, so ∃N with aₙ > L − ε. Since the sequence is non-decreasing, aₙ ≥ aₙ for all n ≥ N. Since L is the supremum, aₙ ≤ L. So |aₙ − L| < ε for all n ≥ N. □

**Why completeness matters**: This theorem fails over ℚ. The sequence 1, 1.4, 1.41, 1.414, ... is monotone and bounded in ℚ (by, say, 2), but does not converge in ℚ (its limit is √2 ∉ ℚ). The MCT is exactly where completeness of ℝ becomes essential for analysis.

**Application**: The MCT is often the easiest way to prove a sequence converges, when you can show it is monotone and bounded without knowing the limit. Once you know it converges, you find the limit by taking both sides of a recurrence to the limit.

**Example**: Define a₁ = 1 and a_{n+1} = √(2 + aₙ). Prove this converges and find the limit.

*Monotonicity*: a₁ = 1, a₂ = √3 > 1. Suppose aₙ < a_{n+1}. Then a_{n+1} = √(2+aₙ) < √(2+a_{n+1}) = a_{n+2}. By induction, (aₙ) is increasing.

*Boundedness*: a₁ < 2. Suppose aₙ < 2. Then a_{n+1} = √(2+aₙ) < √(2+2) = 2. By induction, aₙ < 2 for all n.

By MCT, L = lim aₙ exists. Taking the limit of both sides of a_{n+1} = √(2+aₙ): L = √(2+L), so L² = 2+L, giving L² − L − 2 = 0, (L−2)(L+1) = 0. Since L > 0: L = 2.

---

## 6.1.3 The Cauchy Criterion

**Definition**: A sequence (aₙ) is a **Cauchy sequence** if:

$$\forall \varepsilon > 0 \; \exists N \in \mathbb{N} \; \forall m, n \geq N: |a_m - a_n| < \varepsilon$$

**Theorem** (Cauchy Criterion): In ℝ, a sequence converges iff it is Cauchy.

**Proof**: 
- (→) If aₙ → L, then for large enough m, n: |aₘ − aₙ| ≤ |aₘ − L| + |L − aₙ| < ε/2 + ε/2 = ε.
- (←) If (aₙ) is Cauchy: first, it is bounded (all terms beyond some N are within 1 of aₙ, plus finitely many earlier terms — this gives a bound). A bounded sequence in ℝ has a convergent subsequence (Bolzano-Weierstrass, see below). Let aₙₖ → L be a convergent subsequence. Then for large n and large k: |aₙ − L| ≤ |aₙ − aₙₖ| + |aₙₖ − L| < ε. □

The Cauchy criterion is powerful because it tests convergence without knowing the limit. It is the definition of convergence in general metric spaces (where "Cauchy sequence converges" = "the space is complete"). A complete metric space is one where all Cauchy sequences converge.

---

## 6.1.4 Subsequences and the Bolzano-Weierstrass Theorem

**Definition**: A **subsequence** of (aₙ) is a sequence (a_{nₖ}) where n₁ < n₂ < n₃ < ⋯ is a strictly increasing sequence of natural numbers.

**Theorem** (Bolzano-Weierstrass): Every bounded sequence in ℝ has a convergent subsequence.

**Proof**: Let (aₙ) be bounded: aₙ ∈ [p, q] for all n. Bisect [p, q] into two halves. At least one half contains infinitely many terms; call it [p₁, q₁]. Bisect again; find a half [p₂, q₂] ⊂ [p₁, q₁] with infinitely many terms. Continue. Choose one term aₙₖ from each [pₖ, qₖ] with nₖ strictly increasing. The intervals' lengths → 0, and their left endpoints form a Cauchy sequence (since all chosen terms lie in [pₖ, qₖ] of length (q-p)/2ᵏ). So (aₙₖ) is Cauchy, hence convergent. □

**Connection to compactness**: The Bolzano-Weierstrass theorem is equivalent to the compactness of closed bounded subsets of ℝ (Heine-Borel theorem). In topology, a space is compact if every sequence has a convergent subsequence (sequential compactness, for metric spaces). Compact spaces are "finite-like" in the topological sense. In GR, compactness of Cauchy surfaces (spacelike slices of spacetime) is a strong physical assumption related to the topology of space.

---

## 6.1.5 Limsup and Liminf

For sequences that may not converge, the **limit superior** and **limit inferior** capture the long-run behavior.

**Definition**:
$$\limsup_{n \to \infty} a_n = \lim_{n \to \infty} \sup_{k \geq n} a_k \quad \text{(decreasing limit of suprema of tails)}$$
$$\liminf_{n \to \infty} a_n = \lim_{n \to \infty} \inf_{k \geq n} a_k \quad \text{(increasing limit of infima of tails)}$$

Always: lim inf aₙ ≤ lim sup aₙ.

**The sequence (aₙ) converges iff lim inf aₙ = lim sup aₙ**, and the common value is the limit.

**Example**: aₙ = (−1)ⁿ. lim sup = 1, lim inf = −1. The sequence diverges, but its oscillation is bounded — the lim sup and lim inf capture the two "accumulation points."

The ratio lim sup |aₙ₊₁/aₙ| determines convergence of series (Section 6.2) and the lim sup √(|aₙ|)... determines the radius of convergence of power series (Section 6.3).

---

## References

- Bolzano, B. (1817). *Rein analytischer Beweis des Lehrsatzes, dass zwischen je zwey Werthen, die ein entgegengesetztes Resultat gewähren, wenigstens eine reelle Wurzel der Gleichung liege*. Prague. [Contains the Bolzano-Weierstrass theorem in embryonic form.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 3 on sequences; the canonical treatment.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapters 22–23; excellent motivation and careful treatment of Cauchy sequences.]
- Weierstrass, K. (1874). Lectures on function theory (published in collected works). [The Bolzano-Weierstrass theorem in its modern form is due to Weierstrass.]
