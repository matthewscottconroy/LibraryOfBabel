# Properties of Binary Relations

## Classifying How Relations Behave

A binary relation $R \subseteq A \times A$ (a relation on a set $A$) can have various structural properties. These properties are the foundation for understanding equivalence relations, partial orders, and other fundamental mathematical structures.

**Reflexive**: $\forall x \in A,\; xRx$ — every element is related to itself.

**Irreflexive**: $\forall x \in A,\; \neg(xRx)$ — no element is related to itself.

**Symmetric**: $\forall x, y \in A,\; xRy \to yRx$ — if $x$ relates to $y$, then $y$ relates to $x$.

**Antisymmetric**: $\forall x, y \in A,\; xRy \wedge yRx \to x = y$ — if $x$ and $y$ are mutually related, they are the same element.

**Asymmetric**: $\forall x, y \in A,\; xRy \to \neg(yRx)$ — if $x$ relates to $y$, then $y$ does not relate back. (Implies irreflexive.)

**Transitive**: $\forall x, y, z \in A,\; xRy \wedge yRz \to xRz$ — if $x$ relates to $y$ and $y$ to $z$, then $x$ relates to $z$.

**Euclidean**: $\forall x, y, z \in A,\; xRy \wedge xRz \to yRz$ — if two elements are both related to a common element, they are related to each other.

## Important Combinations

| Properties | Structure |
|-----------|-----------|
| Reflexive + Symmetric + Transitive | Equivalence relation |
| Reflexive + Antisymmetric + Transitive | Partial order |
| Reflexive + Antisymmetric + Transitive + Total | Total order (linear order) |
| Irreflexive + Asymmetric + Transitive | Strict partial order |

**Examples**:
- Equality ($=$): reflexive, symmetric, antisymmetric, transitive — an equivalence relation that is also an order
- $\leq$ on $\mathbb{Z}$: reflexive, antisymmetric, transitive, total — a total order
- $<$ on $\mathbb{Z}$: irreflexive, asymmetric, transitive — a strict total order
- Divisibility $|$ on $\mathbb{N}$: reflexive, antisymmetric, transitive — a partial order (but not total: 2 and 3 are incomparable)
- "Is a sibling of": symmetric but not reflexive (one is not one's own sibling), not transitive in general

## Closure Operations

Given a relation $R$, we can extend it to have desired properties:
- **Reflexive closure**: $R \cup \{(x,x) \mid x \in A\}$
- **Symmetric closure**: $R \cup R^{-1}$ (where $R^{-1} = \{(y,x) \mid xRy\}$)
- **Transitive closure**: $R^+ = \bigcup_{n \geq 1} R^n$ (where $R^n$ is $R$ composed $n$ times)
- **Reflexive-transitive closure**: $R^* = R^+ \cup \{(x,x) \mid x \in A\}$

Transitive closure appears everywhere in computer science: reachability in graphs, dependency analysis, type checking.

## In Lean 4

```lean
-- Reflexive, symmetric, transitive in Lean
example : Reflexive (· = · : ℕ → ℕ → Prop) := fun x => rfl
example : Symmetric (· = · : ℕ → ℕ → Prop) := fun _ _ h => h.symm
example : Transitive (· = · : ℕ → ℕ → Prop) := fun _ _ _ h1 h2 => h1.trans h2

-- Equivalence relation bundled:
#check Equivalence  -- structure with refl, symm, trans fields

-- Transitive closure in Lean (Relation.TransGen):
#check Relation.TransGen  -- the transitive closure of a relation
```

## Exercises
See [problems/ch06_set_theory/02_relations_exercises.md](../../../problems/ch06_set_theory/02_relations_exercises.md)
