# Section 2.4: Cardinality and Infinity

---

## Section Introduction

The theory of cardinality is Cantor's great gift to mathematics: a precise, rigorous way to compare the sizes of sets, including infinite ones. The key insight is deceptively simple. We say two sets have the same "size" (the same **cardinality**) if there is a bijection between them — a perfect one-to-one correspondence. For finite sets, this agrees with counting. For infinite sets, it reveals a hierarchy of infinities more complex than anyone had imagined.

---

## 2.4.1 Cardinality and Bijection

**Definition**: Two sets A and B have the same cardinality (|A| = |B|) if there exists a bijection f: A → B.

**Definition**: |A| ≤ |B| if there is an injection f: A → B.

**The Cantor-Schroeder-Bernstein Theorem** (Cantor, 1887; Schröder, 1898; Bernstein, 1898): If |A| ≤ |B| and |B| ≤ |A|, then |A| = |B|. Equivalently, if there are injections in both directions, there is a bijection.

This theorem is non-trivial to prove (it requires a careful construction) but is indispensable for establishing cardinality equalities without constructing an explicit bijection.

---

## 2.4.2 Countable and Uncountable Sets

A set A is:
- **Finite** if |A| = n for some n ∈ ℕ (i.e., A can be put in bijection with {0, 1, ..., n-1}).
- **Countably infinite** (or **denumerable**) if |A| = |ℕ| — there is a bijection between A and ℕ.
- **Countable** if A is finite or countably infinite.
- **Uncountable** if A is infinite but not countably infinite.

The cardinality of ℕ is denoted ℵ₀ (aleph-nought).

**Examples of countably infinite sets:**
- ℕ (by definition)
- ℤ (bijection: 0↦0, 1↦1, -1↦2, 2↦3, -2↦4, ...)
- ℚ (Cantor's diagonal enumeration of fractions: list fractions in a grid and traverse diagonally)
- The set of all finite strings over a finite alphabet (e.g., all English sentences — this includes all mathematical propositions ever written or to be written)

**The rational numbers are countable**: This is the one result about infinity that surprises beginners. There are "as many" rational numbers as there are natural numbers, even though the rationals are dense in the reals (between any two rationals there is another) while the naturals are not.

**Proof sketch**: Arrange the positive rationals in an infinite grid: row m contains m/1, m/2, m/3, ..., and column n contains 1/n, 2/n, 3/n, .... Traverse this grid by Cantor's diagonal method (move diagonally through the grid, skipping duplicates). This gives an enumeration of all positive rationals. Adding negative rationals and zero as before gives all of ℚ. ∎

---

## 2.4.3 Cantor's Diagonal Argument

**Theorem** (Cantor, 1891): The set ℝ (and in particular, the interval [0, 1]) is uncountable.

**Proof**: Suppose, for contradiction, that [0, 1] is countable — i.e., we can list all elements as x₁, x₂, x₃, .... Write each xₙ in decimal form:

$$x_n = 0.d_{n1}d_{n2}d_{n3}d_{n4}\cdots$$

where dₙₖ ∈ {0, 1, ..., 9} is the k-th decimal digit of xₙ.

**Construct** a new number y = 0.e₁e₂e₃... where:

$$e_k = \begin{cases} 5 & \text{if } d_{kk} \neq 5 \\ 6 & \text{if } d_{kk} = 5 \end{cases}$$

(The choice of 5 and 6 avoids issues with infinite trailing 9s, which would give a second decimal representation.)

Then y ∈ [0, 1] (since 0 < y < 1), but y differs from every xₙ in the list: y differs from x₁ in the 1st decimal place, from x₂ in the 2nd decimal place, and in general from xₙ in the n-th decimal place (by construction). So y is not in our list.

But we assumed the list contains all elements of [0, 1]. Contradiction. Therefore [0, 1] is uncountable. ∎

The cardinality of ℝ is denoted c = 2^{ℵ₀} (the continuum). It can be shown that |ℝ| = |𝒫(ℕ)| = 2^{ℵ₀}.

---

## 2.4.4 Cantor's Theorem and the Hierarchy of Infinities

**Cantor's Theorem**: For any set A (including infinite sets), |A| < |𝒫(A)|.

That is, the power set of A is strictly larger than A itself. This holds even for infinite sets.

**Proof**: First, |A| ≤ |𝒫(A)|, since the map a ↦ {a} is an injection from A into 𝒫(A). 

Second, there is no surjection from A onto 𝒫(A). Suppose f: A → 𝒫(A) is any function. Define D = {a ∈ A : a ∉ f(a)}. Then D ⊆ A, so D ∈ 𝒫(A). We claim D ∉ im(f). If D = f(b) for some b ∈ A, then:
- If b ∈ D, then b ∉ f(b) = D — contradiction.
- If b ∉ D, then b ∈ f(b) = D — contradiction.

So D is not in the image of f, and f is not surjective. ∎

This proof is a generalization of Cantor's diagonal argument and of Russell's paradox (taking A = U = "the set of all sets" leads to the paradox, showing why no universal set can exist in a consistent theory).

**The infinite hierarchy**: By iterating the power set construction:
$$\aleph_0 = |\mathbb{N}| < 2^{\aleph_0} = |\mathbb{R}| < 2^{|\mathbb{R}|} < 2^{2^{|\mathbb{R}|}} < \cdots$$

There are infinitely many distinct infinite cardinalities — a hierarchy of infinities extending without bound.

**The Continuum Hypothesis** (Cantor, 1878): There is no set whose cardinality is strictly between ℵ₀ and 2^{ℵ₀}. This was the first of Hilbert's 23 famous problems (1900). Gödel (1940) showed it is consistent with ZFC, and Cohen (1963, Fields Medal) showed its negation is also consistent. The Continuum Hypothesis is therefore **independent of ZFC** — it can neither be proved nor disproved from the standard axioms of set theory.

---

## Physical Significance

The fact that ℝ is uncountable — that it contains vastly more points than ℚ — has direct physical relevance. The configuration space of a classical field (e.g., the electromagnetic field or the spacetime metric) is a function space: an infinite-dimensional space of functions from ℝ⁴ (spacetime) to some target space. Such spaces are uncountably infinite-dimensional. The mathematics of such spaces — functional analysis, distribution theory, quantum field theory — requires the full machinery of Lebesgue measure theory and Hilbert spaces, which we begin in Chapter 13.

The Bekenstein-Hawking entropy formula S = A/(4ℓ_P²) can be interpreted as saying that the number of *distinguishable states* of a black hole with horizon area A is e^{A/(4ℓ_P²)} — a finite number for finite A. This finiteness is deeply at odds with the infinite-dimensional field theory description of spacetime. The tension between these two views is one of the central puzzles of quantum gravity.

---

## References

- Cantor, G. (1878). "Ein Beitrag zur Mannigfaltigkeitslehre." *Journal für die reine und angewandte Mathematik*, 84, 242–258. [Cantor proposes the Continuum Hypothesis.]
- Cantor, G. (1891). "Über eine elementare Frage der Mannigfaltigkeitslehre." *Jahresbericht der DMV*, 1, 75–78. [The diagonal argument.]
- Cohen, P.J. (1963). "The Independence of the Continuum Hypothesis." *Proceedings of the National Academy of Sciences*, 50, 1143–1148. [Proves CH is independent of ZFC; Fields Medal work.]
- Gödel, K. (1940). *The Consistency of the Axiom of Choice and the Generalized Continuum Hypothesis with the Axioms of Set Theory*. Princeton University Press.
- Halmos, P.R. (1960). *Naive Set Theory*. Springer. [Chapter 13 on cardinal numbers.]
