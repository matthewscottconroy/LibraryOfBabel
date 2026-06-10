# Subsets and Set Equality

## Subset: The Containment Relation

If every element of $A$ is also an element of $B$, we say $A$ is a **subset** of $B$, written $A \subseteq B$:

$$A \subseteq B \iff \forall x\, (x \in A \to x \in B)$$

Note the asymmetry: $A \subseteq B$ does not require $B \subseteq A$. If $A \subseteq B$ but $B \not\subseteq A$, we say $A$ is a **proper subset** of $B$, written $A \subsetneq B$.

**Examples:**
- $\{1, 3\} \subseteq \{1, 2, 3, 4\}$
- $\mathbb{N} \subseteq \mathbb{Z} \subseteq \mathbb{Q} \subseteq \mathbb{R}$
- $\emptyset \subseteq A$ for every set $A$ (vacuously true — every element of $\emptyset$ satisfies any condition)
- $A \subseteq A$ for every $A$ (reflexivity)

## Set Equality via Double Inclusion

By the **Extensionality Axiom**, two sets are equal iff they have the same elements. Equivalently:

$$A = B \iff A \subseteq B \text{ and } B \subseteq A$$

This is the standard proof strategy for set equality: prove $A \subseteq B$ (every element of $A$ is in $B$) and $B \subseteq A$ (every element of $B$ is in $A$). This double inclusion argument is fundamental to set theory and is used constantly.

**Example**: Prove $A \cap (A \cup B) = A$.

$(\subseteq)$: Let $x \in A \cap (A \cup B)$. Then $x \in A$ and $x \in A \cup B$. Since $x \in A$, done.

$(\supseteq)$: Let $x \in A$. Then $x \in A$ (tautologically) and $x \in A \cup B$ (since $x \in A$). So $x \in A \cap (A \cup B)$.

## The Subset Relation as a Partial Order

On the power set $\mathcal{P}(A)$ of any set $A$, the subset relation $\subseteq$ is a **partial order**:

- **Reflexive**: $A \subseteq A$ ✓
- **Antisymmetric**: $A \subseteq B \wedge B \subseteq A \Rightarrow A = B$ ✓ (by Extensionality)
- **Transitive**: $A \subseteq B \wedge B \subseteq C \Rightarrow A \subseteq C$ ✓

In fact, $(\mathcal{P}(X), \subseteq)$ is a **complete lattice**: every collection of subsets of $X$ has both a greatest lower bound (intersection) and least upper bound (union).

## In Lean 4

```lean
-- Subset in Lean (Set α):
-- A ⊆ B is defined as ∀ x, x ∈ A → x ∈ B

example (A B : Set ℕ) (h : A ⊆ B) (x : ℕ) (hx : x ∈ A) : x ∈ B := h hx

-- Set equality via double inclusion
theorem set_eq_iff (A B : Set α) : A = B ↔ A ⊆ B ∧ B ⊆ A :=
  ⟨fun h => h ▸ ⟨fun x hx => hx, fun x hx => hx⟩,
   fun ⟨h1, h2⟩ => Set.Subset.antisymm h1 h2⟩

-- The absorption law: A ∩ (A ∪ B) = A
example (A B : Set α) : A ∩ (A ∪ B) = A :=
  Set.inter_union_self A B
```

## Exercises
See [problems/ch06_set_theory/01_set_operations_exercises.md](../../../problems/ch06_set_theory/01_set_operations_exercises.md)
