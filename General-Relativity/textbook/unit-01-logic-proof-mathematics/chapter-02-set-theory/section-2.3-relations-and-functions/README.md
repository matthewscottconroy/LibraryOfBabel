# Section 2.3: Relations and Functions

---

## Section Introduction

Sets are the raw material; relations and functions are the structures built from them. A **relation** is a set of ordered pairs — it specifies which elements of one set are "related to" which elements of another. A **function** is a special kind of relation, one where each input has exactly one output.

Functions are the central objects of analysis, and they appear throughout physics: the metric tensor is a function (or rather a tensor field), the Einstein field equations are equations between functions, the trajectory of a particle is a function of time. Understanding functions precisely — their domains, codomains, bijectivity, injectivity — is essential preparation for everything that follows.

---

## 2.3.1 Relations

A **binary relation** from a set A to a set B is a subset R ⊆ A × B. We write aRb (or (a, b) ∈ R) to mean "a is related to b by R."

**Examples:**
- The relation "≤" on ℝ: R = {(x, y) ∈ ℝ² : x ≤ y}.
- The relation "divides" on ℕ: R = {(m, n) ∈ ℕ² : m | n}.
- The relation "is orthogonal to" on a vector space V: R = {(u, v) ∈ V² : ⟨u, v⟩ = 0}.
- The relation "has the same mass" between fundamental particles.

**Properties of relations** on a set A (i.e., R ⊆ A × A):
- **Reflexive**: ∀a ∈ A, aRa.
- **Symmetric**: ∀a, b ∈ A, aRb → bRa.
- **Antisymmetric**: ∀a, b ∈ A, (aRb ∧ bRa) → a = b.
- **Transitive**: ∀a, b, c ∈ A, (aRb ∧ bRc) → aRc.

---

## 2.3.2 Equivalence Relations

An **equivalence relation** on A is a relation that is reflexive, symmetric, and transitive. The canonical example is equality.

More interesting examples:
- **Congruence modulo n**: a ≡ b (mod n) iff n | (a - b). This is an equivalence relation on ℤ.
- **Gauge equivalence**: in physics, two configurations related by a gauge transformation are physically equivalent. The quotient by gauge equivalence gives the physical state space.
- **Diffeomorphism equivalence**: two spacetime manifolds related by a diffeomorphism represent the same physical spacetime. This is the basis of the principle of general covariance.

**Equivalence classes**: If ~ is an equivalence relation on A, the **equivalence class** of a is:
$$[a] = \{b \in A : a \sim b\}$$

**Key theorem**: The equivalence classes of an equivalence relation partition the set A into disjoint, non-empty subsets that cover all of A. Conversely, any partition defines an equivalence relation. (Proof: straightforward from definitions; see exercises.)

**The quotient set** A/~ is the set of all equivalence classes: A/~ = {[a] : a ∈ A}.

This construction — taking a set and "gluing together" elements related by an equivalence — is pervasive in mathematics. The real numbers are constructed as equivalence classes of rational Cauchy sequences. Tangent vectors on a manifold are equivalence classes of curves. The physical state space of a gauge theory is a quotient of the space of field configurations.

---

## 2.3.3 Functions

A **function** f from A to B (written f: A → B) is a relation R ⊆ A × B such that for every a ∈ A, there exists a **unique** b ∈ B with (a, b) ∈ R. We write f(a) = b.

- **Domain**: dom(f) = A. Every element of A has exactly one image.
- **Codomain**: cod(f) = B. Not every element of B need be an image.
- **Image (range)**: im(f) = f(A) = {f(a) : a ∈ A} ⊆ B.

**Classification of functions:**
- **Injective** (one-to-one): f(a₁) = f(a₂) → a₁ = a₂. Different inputs give different outputs.
- **Surjective** (onto): ∀b ∈ B, ∃a ∈ A, f(a) = b. Every element of B is achieved.
- **Bijective**: both injective and surjective. A perfect pairing between A and B.

**Composition**: If f: A → B and g: B → C, then g ∘ f: A → C is defined by (g ∘ f)(a) = g(f(a)).

**Inverse function**: If f: A → B is bijective, the **inverse** f⁻¹: B → A is defined by f⁻¹(b) = a iff f(a) = b. It satisfies f⁻¹ ∘ f = id_A and f ∘ f⁻¹ = id_B, where id_A is the identity function on A.

**Physical significance**: Many physical quantities are functions — scalar fields (temperature, pressure), vector fields (velocity, electric field), tensor fields (metric tensor, curvature tensor). The smoothness conditions on these functions (continuity, differentiability, analyticity) are what make calculus applicable.

---

## 2.3.4 Partial Orders

A **partial order** on A is a relation that is reflexive, antisymmetric, and transitive. It expresses a notion of "≤" where not all pairs need be comparable.

A **total order** (or linear order) additionally requires that any two elements are comparable: ∀a, b: aRb or bRa.

**Examples:**
- ≤ on ℝ: total order.
- Divisibility on ℕ: partial order (4 and 6 are not comparable).
- Set inclusion ⊆ on 𝒫(A): partial order.
- Causal ordering in spacetime: p ≤ q iff q is in the causal future of p. This is a partial order (spacelike-separated events are not causally comparable). The causal structure of spacetime is fundamental to GR and to the formulation of the singularity theorems.

---

## References

- Halmos, P.R. (1960). *Naive Set Theory*. Springer. [Chapters 6–8 on relations and functions.]
- Suppes, P. (1960). *Axiomatic Set Theory*. Dover. [Chapter 3 on relations.]
- Wald, R.M. (1984). *General Relativity*. University of Chicago Press. [Appendix A develops the set-theoretic foundations needed for GR, including the partial order structure of spacetime.]
