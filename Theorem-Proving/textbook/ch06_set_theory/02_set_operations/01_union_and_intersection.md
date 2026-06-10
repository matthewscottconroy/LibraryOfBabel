# Union and Intersection

## The Two Fundamental Binary Operations

**Union** ($\cup$) and **intersection** ($\cap$) are the two fundamental ways to combine sets.

$$A \cup B = \{x \mid x \in A \vee x \in B\}$$
$$A \cap B = \{x \mid x \in A \wedge x \in B\}$$

Union collects everything in either set; intersection collects only what is in both.

**Visual intuition**: If $A$ and $B$ are overlapping regions, $A \cup B$ is the total shaded region and $A \cap B$ is the overlap region (Venn diagram).

## Algebraic Laws

Union and intersection satisfy a rich set of algebraic identities:

**Commutativity**: $A \cup B = B \cup A$ and $A \cap B = B \cap A$

**Associativity**: $(A \cup B) \cup C = A \cup (B \cup C)$ and $(A \cap B) \cap C = A \cap (B \cap C)$

**Identity elements**: $A \cup \emptyset = A$ and $A \cap U = A$ (where $U$ is the universal set)

**Annihilation**: $A \cup U = U$ and $A \cap \emptyset = \emptyset$

**Idempotence**: $A \cup A = A$ and $A \cap A = A$

**Absorption**: $A \cup (A \cap B) = A$ and $A \cap (A \cup B) = A$

**De Morgan's Laws**: $\overline{A \cup B} = \bar{A} \cap \bar{B}$ and $\overline{A \cap B} = \bar{A} \cup \bar{B}$

**Distributivity**: $A \cup (B \cap C) = (A \cup B) \cap (A \cup C)$

This algebra — a **Boolean algebra** — is isomorphic to propositional logic: $\cup$ corresponds to $\vee$, $\cap$ to $\wedge$, complement to $\neg$, $\emptyset$ to $\bot$, $U$ to $\top$.

## Generalized Union and Intersection

For a family of sets $\mathcal{F} = \{A_i\}_{i \in I}$:

$$\bigcup_{i \in I} A_i = \{x \mid \exists i \in I,\; x \in A_i\}$$
$$\bigcap_{i \in I} A_i = \{x \mid \forall i \in I,\; x \in A_i\}$$

These are guaranteed to exist by the Union axiom (for unions) and Separation (for intersections, starting from any $A_j$).

**Note**: $\bigcap \emptyset$ (intersection over an empty family) is problematic — it would be the set of all sets, which does not exist in ZF. Convention: only take intersections of non-empty families.

## In Lean 4

```lean
-- Union and intersection of sets
example (A B : Set ℕ) : 3 ∈ A ∪ B ↔ 3 ∈ A ∨ 3 ∈ B := Set.mem_union 3 A B

-- De Morgan for sets
example (A B : Set α) : (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ := Set.compl_union A B

-- Distributivity
example (A B C : Set α) : A ∪ (B ∩ C) = (A ∪ B) ∩ (A ∪ C) :=
  Set.union_inter_distrib_left A B C
```

## Exercises
See [problems/ch06_set_theory/01_set_operations_exercises.md](../../../problems/ch06_set_theory/01_set_operations_exercises.md)
