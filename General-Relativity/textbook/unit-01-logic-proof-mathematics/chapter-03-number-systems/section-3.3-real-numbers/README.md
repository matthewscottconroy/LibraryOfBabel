# Section 3.3: Real Numbers

---

## Section Introduction

The rational numbers have a defect: they have gaps. The sequence 1, 1.4, 1.41, 1.414, 1.4142, ... gets closer and closer to √2, yet no rational number is its limit. This incompleteness of ℚ — the fact that some "obviously reasonable" limits of rational sequences do not exist in ℚ — is not merely an inconvenience. It makes calculus impossible over ℚ: the intermediate value theorem, the extreme value theorem, the existence of derivatives — all require the limit of a convergent sequence to actually exist.

The real numbers ℝ are constructed to fill these gaps. The key property of ℝ is **completeness**: every Cauchy sequence in ℝ converges to a limit in ℝ. Equivalently (and this is the way most analysts state it): every non-empty subset of ℝ that is bounded above has a **least upper bound** (supremum) in ℝ. These two formulations are equivalent, and together they characterize ℝ uniquely (up to isomorphism) among ordered fields.

---

## 3.3.1 Ordered Fields

Before constructing ℝ, we specify what we want it to be. An **ordered field** is a field F with a total order ≤ satisfying:
1. a ≤ b → a + c ≤ b + c (addition preserves order)
2. 0 ≤ a and 0 ≤ b → 0 ≤ ab (products of non-negatives are non-negative)

Both ℚ and ℝ are ordered fields. The difference is completeness.

**Completeness (Least Upper Bound Property)**: An ordered field F is **complete** if every non-empty subset S ⊆ F that is bounded above has a least upper bound (supremum) in F.

**Theorem** (uniqueness of the complete ordered field): Up to isomorphism, there is exactly one complete ordered field.

This is a deep result — it means ℝ is not merely *a* complete ordered field but *the* unique one. Any two constructions of the reals (Dedekind's, Cantor's, or any other) yield isomorphic structures.

---

## 3.3.2 Dedekind Cuts

Richard Dedekind (1872) constructed ℝ from ℚ using "cuts." A **Dedekind cut** is a pair (A, B) where A, B are non-empty subsets of ℚ with:
1. A ∪ B = ℚ (every rational is in A or B)
2. A ∩ B = ∅ (they are disjoint)
3. Every element of A is less than every element of B
4. A has no greatest element

Intuitively: A is the set of rationals "to the left" of some real number, and B is the set "to the right." The cut itself *is* the real number.

**Examples:**
- The cut for the rational number 1/2: A = {q ∈ ℚ : q < 1/2}, B = {q ∈ ℚ : q ≥ 1/2}.
- The cut for √2: A = {q ∈ ℚ : q < 0 or q² < 2}, B = {q ∈ ℚ : q ≥ 0 and q² ≥ 2}.

The second cut corresponds to an irrational number: no rational is greatest in A, and the "gap" between A and B in ℚ is filled by √2 ∈ ℝ \ ℚ.

We define ℝ as the set of all Dedekind cuts. Arithmetic operations and ordering are defined in terms of operations on cuts, and one verifies that the resulting structure is a complete ordered field.

---

## 3.3.3 Cauchy Sequences

Cantor's approach (1872) is different but equivalent. A **Cauchy sequence** of rationals is a sequence (qₙ) of rational numbers such that:

$$\forall \varepsilon > 0 \; \exists N \in \mathbb{N} \; \forall m, n \geq N: |q_m - q_n| < \varepsilon$$

Intuitively: the terms of the sequence become arbitrarily close to each other (though they need not converge in ℚ).

**Example**: The sequence of decimal approximations to √2: 1, 1.4, 1.41, 1.414, 1.4142, ... is a Cauchy sequence in ℚ that does not converge in ℚ (since √2 ∉ ℚ).

Two Cauchy sequences (qₙ) and (rₙ) are **equivalent** if lim_{n→∞} (qₙ - rₙ) = 0. This defines an equivalence relation on the set of Cauchy sequences. The **real numbers** are defined as the equivalence classes:

$$\mathbb{R} = \{\text{Cauchy sequences in } \mathbb{Q}\} / \sim$$

Arithmetic is defined on equivalence classes in the obvious way: [(qₙ)] + [(rₙ)] = [(qₙ + rₙ)].

The rational number q is identified with the equivalence class of the constant sequence (q, q, q, ...).

---

## 3.3.4 The Completeness Theorem

**Theorem**: ℝ as constructed above (by either Dedekind cuts or Cauchy sequences) is a complete ordered field.

The proof that the least upper bound property holds is the key step. For Dedekind cuts: the supremum of a bounded set of cuts is the "union" cut — the cut whose A-part is the union of the A-parts of all cuts in the set. For Cauchy sequences: one must show that every Cauchy sequence of equivalence classes has a limit in ℝ (which requires a diagonal argument to extract a single Cauchy sequence of rationals from a Cauchy sequence of equivalence classes of Cauchy sequences).

---

## 3.3.5 Key Properties of ℝ

**(a) Archimedean Property**: ∀x ∈ ℝ, ∃n ∈ ℕ such that n > x. (There is no infinitely large real number.)

**(b) Density of ℚ in ℝ**: Between any two distinct real numbers, there is a rational. ∀a, b ∈ ℝ with a < b, ∃q ∈ ℚ with a < q < b.

**(c) Density of irrationals in ℝ**: Between any two distinct real numbers, there is an irrational.

**(d) ℝ is uncountable**: By Cantor's diagonal argument (Section 2.4).

**(e) ℝ has the cardinality of the continuum**: |ℝ| = 2^{ℵ₀} = c.

---

## 3.3.6 Why Completeness Is Necessary for Analysis

The following fundamental theorems of analysis require completeness:

- **Intermediate Value Theorem**: If f: [a, b] → ℝ is continuous and f(a) < 0 < f(b), then ∃c ∈ (a, b) with f(c) = 0. (Fails over ℚ: the polynomial x² - 2 changes sign between 1 and 2 on ℚ, but the "root" √2 ∉ ℚ.)
  
- **Extreme Value Theorem**: A continuous function on a closed interval achieves its maximum and minimum.

- **Mean Value Theorem**: If f is differentiable on (a, b) and continuous on [a, b], then ∃c ∈ (a, b) with f'(c) = (f(b) - f(a))/(b - a).

- **Convergence of Cauchy sequences**: Every Cauchy sequence in ℝ converges.

All of these fail without completeness. And without them, calculus — and hence differential geometry and GR — cannot be built.

---

## 3.3.7 Connection to Spacetime: The Real Line as Coordinate

In general relativity, each coordinate function on a spacetime manifold is a function to ℝ. The completeness of ℝ ensures that the limiting operations used in calculus — derivatives, integrals, solutions to differential equations — all behave as expected. Without completeness, the concept of a smooth curve would be pathological: limits that "should" exist would not.

More subtly: the Cauchy sequence construction of ℝ suggests a way to think about the approach to a singularity. A sequence of events on a timelike geodesic that is geodesically incomplete might behave like a Cauchy sequence that lacks a limit — "trying to converge" but finding no event to converge to because the manifold ends. Geodesic incompleteness (the mathematical definition of singularity in GR, following Penrose 1965) is precisely the absence of a "completion" for certain curves.

---

## References

- Cantor, G. (1872). "Über die Ausdehnung eines Satzes aus der Theorie der trigonometrischen Reihen." *Mathematische Annalen*, 5, 123–132. [Cauchy sequence construction of ℝ.]
- Dedekind, R. (1872). *Stetigkeit und irrationale Zahlen*. Braunschweig: Vieweg. English translation: *Continuity and Irrational Numbers*, in Dedekind (1901), *Essays on the Theory of Numbers*, Open Court. [The Dedekind cut construction.]
- Penrose, R. (1965). "Gravitational Collapse and Space-Time Singularities." *Physical Review Letters*, 14, 57–59. [Defines singularities as geodesic incompleteness — the GR analogue of sequence incompleteness.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 1 constructs ℝ from Dedekind cuts and establishes all properties listed here.]
