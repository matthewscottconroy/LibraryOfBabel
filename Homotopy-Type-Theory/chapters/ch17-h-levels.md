# Chapter 17: H-Levels and Truncations

## Introduction

Not all types have equally complex path structure. Some types have paths that are all trivial (contractible types, propositions). Some have paths but no non-trivial paths between paths (sets). And some have a rich, infinite tower of non-trivial higher paths (general types in HoTT).

The *h-level* (homotopy level) of a type measures how complex its path structure is. This hierarchy — contractible, mere propositions, sets, groupoids, 2-groupoids, ... — is one of the most important organizing principles in HoTT. It connects:
- Logic: propositions vs. data
- Mathematics: sets vs. higher structures
- Homotopy theory: contractible spaces vs. spaces with nontrivial homotopy groups

---

## 1. The Truncation Hierarchy

**Definition 17.1 (h-Level).** We define, by induction on $n : \mathbb{N}$, the predicate $\mathsf{is}\text{-}n\text{-}\mathsf{type} : \mathsf{Type} \to \mathsf{Prop}$:

- A type $A$ is *contractible* ($(-2)$-truncated, or *h-level $-2$*) if:
$$\mathsf{isContr}(A) :\equiv \Sigma_{c:A}\, \Pi_{x:A}\, (c = x)$$
(a center of contraction $c$ with a path from $c$ to every element)

- A type $A$ is a *mere proposition* ($(-1)$-truncated, or *h-level $-1$*) if:
$$\mathsf{isProp}(A) :\equiv \Pi_{x y : A}\, (x = y)$$
(any two elements are equal)

- A type $A$ is a *set* ($0$-truncated, or *h-level $0$*) if:
$$\mathsf{isSet}(A) :\equiv \Pi_{x y : A}\, \mathsf{isProp}(x = y)$$
(any two paths between the same endpoints are equal)

- A type $A$ is a *$1$-groupoid* ($1$-truncated) if:
$$\mathsf{is}\text{-}1\text{-}\mathsf{type}(A) :\equiv \Pi_{x y : A}\, \mathsf{isSet}(x = y)$$

- In general, $A$ is an *$n$-type* ($n$-truncated) if $\Pi_{x y : A}\, \mathsf{is}\text{-}(n-1)\text{-}\mathsf{type}(x = y)$.

The naming convention in the HoTT Book uses negative indices (contractible = $(-2)$-type, proposition = $(-1)$-type) to match the pattern where an $n$-type has trivial $\pi_k$ for $k > n + 2$, matching the homotopy-theoretic convention.

---

## 2. Contractible Types

### 2.1 Definition and Examples

**Definition 17.2.** A type $A$ is *contractible* if $\mathsf{isContr}(A)$: there exists $c : A$ such that every element of $A$ is equal to $c$.

The element $c$ is called the *center of contraction*, and the function $\Pi_{x:A} (c = x)$ is the *contracting homotopy*.

**Topological picture:** Contractible spaces are those homotopy equivalent to a point. $\mathbb{R}^n$, a disc, a cone, any convex subset of a normed space — all contractible.

**Examples:**
- $\mathbf{1}$ (the unit type) is contractible: center $\star$, contracting homotopy trivial.
- $\Sigma_{x:A} (a = x)$ (the total space of paths from $a$) is contractible, with center $(a, \mathsf{refl}_a)$.
- If $f : A \to B$ is an equivalence, then the fiber $\mathsf{fib}_f(b) = \Sigma_{x:A}(f(x) = b)$ is contractible for every $b : B$.

**Theorem 17.3.** $A$ is contractible iff $A \simeq \mathbf{1}$ (i.e., $A$ is equivalent to the unit type).

*Proof.* ($\Rightarrow$): If $(c, h)$ witnesses contractibility, define $f : A \to \mathbf{1}$ by $f(a) = \star$ and $g : \mathbf{1} \to A$ by $g(\star) = c$. Then $g \circ f = \lambda a, c$ is homotopic to $\mathsf{id}_A$ via $h$, and $f \circ g = \mathsf{id}_\mathbf{1}$.

($\Leftarrow$): If $A \simeq \mathbf{1}$, then $A$ has at most one element (the preimage of $\star$) and the equivalence gives a path from any element to any other (by transport). $\square$

**Lemma 17.4.** If $A$ is contractible and $B$ is any type, then $A \times B \simeq B$.

**Lemma 17.5.** If $A$ is contractible and $P : A \to \mathsf{Type}$ is any family, then $\Sigma_{x:A} P(x) \simeq P(c)$ (where $c$ is the center of contraction).

---

## 3. Mere Propositions (h-Props)

### 3.1 Definition and Examples

**Definition 17.6.** A type $A$ is a *mere proposition* (or *h-prop* or *subsingleton*) if any two elements are equal: $\mathsf{isProp}(A) :\equiv \Pi_{x y : A} (x = y)$.

**Why "mere"?** A proposition in mathematics is either true or false — it has no interesting internal structure. A *mere* proposition in HoTT is a type where the proof doesn't matter: any two proofs are equal.

**Examples:**
- $\mathbf{0}$ (the empty type) is a proposition (vacuously: there are no elements to compare)
- $\mathbf{1}$ (unit) is a proposition: $\Pi_{x y : \mathbf{1}} (x = y)$ because the only element is $\star$
- For any $A$: $\mathsf{isProp}(A)$, $\mathsf{isContr}(A)$, $\mathsf{isSet}(A)$ are all propositions
- $a =_A b$ is not always a proposition (that's exactly UIP, which fails in general HoTT)

**Theorem 17.7.** $\mathbf{0}$ and $\mathbf{1}$ are propositions. Contractible types are propositions. If $A$ is a proposition, then $A$ is contractible iff $A$ is inhabited (has an element).

**Theorem 17.8 (Propositions are Subtypes).** $\mathsf{isProp}(A)$ iff for any $x, y : A$, the type $x = y$ is contractible.

*Proof.* If $A$ is a proposition, then any $x, y : A$ gives a path $p : x = y$ (by the proposition assumption). The contractibility of $x = y$ follows from the groupoid laws (any other path is equal to $p$, which requires UIP for the identity types... so actually this is slightly more subtle and requires a calculation). $\square$

### 3.2 Propositions as Logic

In classical set-theoretic mathematics, a proposition is just a truth value (true or false). In HoTT, mere propositions play the role of truth values in the constructive logic embedded in the type theory.

**Theorem 17.9 (Propositions form a Boolean algebra if LEM holds).** If we assume the law of excluded middle ($\Pi_{A : \mathsf{Type}} (\mathsf{isProp}(A) \to (A \vee \neg A))$), then the type of propositions $\mathsf{hProp} = \Sigma_{A:\mathsf{Type}} \mathsf{isProp}(A)$ has the structure of a Boolean algebra.

Without LEM, $\mathsf{hProp}$ is still a Heyting algebra (the lattice of propositions with intuitionistic logic).

### 3.3 Closure Properties

**Theorem 17.10.** If $A$ and $B$ are propositions, so are:
- $A \times B$ (product of propositions is a proposition)
- $A \to B$ (function type between propositions is a proposition)
- $\Pi_{x:A} B(x)$ for a proposition-valued family $B : A \to \mathsf{Prop}$
- $a =_A b$ for any $a, b : A$ where $A$ is a set

---

## 4. Sets (h-Sets)

### 4.1 Definition

**Definition 17.11.** A type $A$ is an *h-set* (or just *set*) if its identity types are all mere propositions:
$$\mathsf{isSet}(A) :\equiv \Pi_{x y : A}\, \mathsf{isProp}(x = y)$$

**Why this is the right definition of "set":** In classical mathematics, two proofs of $a = b$ are always "the same" — equality is just a truth value. H-sets are exactly the types where this classical behavior holds: there is at most one proof that any two elements are equal.

**Topological picture:** H-sets correspond to *discrete topological spaces* (spaces where all homotopy groups above $\pi_0$ vanish) — up to homotopy equivalence, a disjoint union of contractible spaces.

**Examples:**
- $\mathbb{N}$: any two natural numbers are either equal (with a unique proof) or not equal. $\mathbb{N}$ is a set.
- $\mathbb{Z}$, $\mathbb{Q}$, $\mathbb{R}$ (as classically defined): all sets.
- $\mathsf{Bool} = \mathbf{1} + \mathbf{1}$: a set (a two-element discrete space).
- $\mathbf{0}$ and $\mathbf{1}$: sets (trivially).
- $S^1$ (as a type/HIT in HoTT): *not* a set. Its identity type at the basepoint is $\mathbb{Z}$ — the loop space.

### 4.2 Hedberg's Theorem

**Theorem 17.12 (Hedberg's Theorem).** If $A$ has decidable equality (i.e., $\Pi_{x y : A} (x = y) + (x \neq y)$), then $A$ is a set.

*Proof sketch.* Given decidable equality, we can define a "canonical" path between any two equal elements and show all other paths are propositionally equal to it, via a path-flattening argument. $\square$

**Corollary 17.13.** $\mathbb{N}$ is a set (since equality of natural numbers is decidable by computation).

**Important:** Hedberg's theorem requires the `--without-K` flag in Agda to be non-trivial. In Agda with `K` (Uniqueness of Identity Proofs), *every* type is a set. HoTT works without K.

### 4.3 The Category of Sets in HoTT

The type $\mathsf{hSet} = \Sigma_{A:\mathsf{Type}} \mathsf{isSet}(A)$ of h-sets forms a category:
- Objects: h-sets $(A, \mathsf{isSet}_A)$
- Morphisms: functions $A \to B$
- Composition: function composition

**Theorem 17.14.** The category of h-sets in HoTT is equivalent to an elementary topos satisfying classical axioms — provided we assume LEM and AC (axiom of choice, which is provable in HoTT in a specific form).

This shows that classical set-based mathematics embeds faithfully into HoTT, at the level of h-sets.

---

## 5. General n-Types

**Definition 17.15.** A type $A$ is an *$n$-type* (for $n \geq -2$) if all its $(n+1)$-st identity types are contractible, equivalently:
$$\mathsf{is}\text{-}n\text{-}\mathsf{type}(A) :\equiv \Pi_{x y : A}\, \mathsf{is}\text{-}(n-1)\text{-}\mathsf{type}(x = y)$$

with the base case $\mathsf{is}\text{-}(-2)\text{-}\mathsf{type}(A) :\equiv \mathsf{isContr}(A)$.

The *h-level* of a type is the smallest $n$ such that $A$ is an $n$-type.

**Topological correspondence:** An $n$-type in HoTT corresponds to an $(n+2)$-truncated space in classical homotopy theory (a space with $\pi_k = 0$ for $k > n + 2$):
- $(-2)$-type: contractible space ($\pi_k = 0$ for all $k$)
- $(-1)$-type: space with $\pi_k = 0$ for all $k \geq 1$ (homotopy equivalent to $\emptyset$ or $*$)
- $0$-type: space with $\pi_k = 0$ for $k \geq 1$ (discrete space, disjoint union of contractibles)
- $1$-type: space with $\pi_k = 0$ for $k \geq 2$ (a $K(G,1)$ = Eilenberg-MacLane space)
- $2$-type: space with $\pi_k = 0$ for $k \geq 3$

---

## 6. Truncations

Given any type $A$, we can "force" it to be an $n$-type by taking its *$n$-truncation* $\|A\|_n$. This is a HIT (Higher Inductive Type) that adds paths until all higher structure above level $n$ becomes contractible.

### 6.1 Propositional Truncation $\|A\|$

The *propositional truncation* $\|A\| = \|A\|_{-1}$ is the mere proposition that "$A$ is inhabited":
- Constructor: $| - | : A \to \|A\|$
- Path constructor: $\mathsf{squash} : \Pi_{x y : \|A\|}\, x = y$

**The universal property:** For any mere proposition $P$: $(A \to P) \to (\|A\| \to P)$. So $\|A\|$ is the "best approximation" of $A$ by a proposition.

**Why this matters:** The propositional truncation $\|A\|$ captures "there merely exists" — the constructive version of classical existence. 
- $\Sigma_{x:A} P(x)$: "there exists $x$ with $P(x)$, and we have a witness"
- $\|\Sigma_{x:A} P(x)\|$: "there merely exists $x$ with $P(x)$" (no computational witness)

This distinction is invisible in classical logic (where both reduce to the same truth value) but crucial in constructive mathematics.

**Example 17.16.** "The intermediate value theorem" classically says: if $f : [0,1] \to \mathbb{R}$ is continuous and $f(0) < 0 < f(1)$, then $\exists x, f(x) = 0$. Constructively:
- Weakly: $\|\Sigma_{x:[0,1]} f(x) = 0\|$ (there merely exists a root)
- Strongly: $\Sigma_{x:[0,1]} f(x) = 0$ (there exists a computable root)

The classical proof gives the weak version; a constructive proof (e.g., bisection algorithm) gives the strong version.

### 6.2 Set Truncation $\|A\|_0$

The *set truncation* $\|A\|_0$ is the "set of connected components" of $A$:
- Constructor: $| - |_0 : A \to \|A\|_0$
- Truncation: $\Pi_{x y : \|A\|_0}\, \mathsf{isProp}(x = y)$

**Example 17.17.** $\|S^1\|_0 = \mathbf{1}$ (the circle has one path-component).

**Example 17.18.** $\|\Sigma_{n:\mathbb{N}} P(n)\|_0$ is the set of natural numbers satisfying $P$, forgetting the proof. Wait — no: $\|\Sigma_{n:\mathbb{N}} P(n)\|_0$ identifies any two witnesses for the same $n$, but does *not* identify different $n$'s with $P(n)$. It's more like the set $\{n : \mathbb{N} \mid P(n)\}$.

### 6.3 Higher Truncations

The $n$-truncation $\|A\|_n$ cuts off the homotopy groups of $A$ above level $n$:
$$\pi_k(\|A\|_n) = \begin{cases} \pi_k(A) & k \leq n \\ 0 & k > n \end{cases}$$

**The universal property:** $\|A\|_n$ is the initial $n$-type equipped with a map from $A$. For any $n$-type $B$ and any function $f : A \to B$, there is a unique function $\|A\|_n \to B$ making the appropriate triangle commute.

---

## 7. The Hierarchy at a Glance

$$\text{Contractible} \subset \text{hProp} \subset \text{hSet} \subset 1\text{-Type} \subset 2\text{-Type} \subset \cdots$$

| h-Level | Name | Identity Types | Topology | Logic |
|---|---|---|---|---|
| $-2$ | Contractible | Contractible | Point | True |
| $-1$ | hProp | Contractible | $\emptyset$ or Point | Proposition |
| $0$ | hSet | hProp | Discrete | Set |
| $1$ | 1-Groupoid | hSet | $K(G,1)$ space | Groupoid |
| $2$ | 2-Groupoid | 1-Groupoid | 2-type | 2-Groupoid |
| $\infty$ | ∞-Groupoid | ∞-Groupoid | Any space | ∞-Groupoid |

---

## Exercises

**17.1.** Prove that $\mathbf{0}$ is a proposition (no elements, so the condition holds vacuously). Prove that $\mathbf{1}$ is contractible.

**17.2.** Prove that if $A$ is a proposition and $A$ is inhabited (has an element), then $A$ is contractible.

**17.3.** Show that $\mathsf{isProp}(A)$ is itself a proposition: $\mathsf{isProp}(\mathsf{isProp}(A))$.

**17.4.** Prove Theorem 17.10: if $A$ and $B$ are propositions, then $A \times B$ is a proposition.

**17.5.** Prove Hedberg's theorem (Theorem 17.12) in full. The key step is: given decidable equality, define a "constant" endofunction on $x = y$ (one that maps all paths to the same path) and use it to flatten the identity type.

**17.6.** Show that if $A$ is a set, then $\mathsf{isSet}(A)$ is a proposition.

**17.7.** In Lean 4 or Agda, formalize the definition of `isProp`, `isSet`, and prove:
  - `isProp Bool`
  - `isSet Nat`
  - `isProp (isProp A)` for any `A`

**17.8.** The *propositional truncation* $\|A\|$ satisfies the universal property: for any proposition $P$, $(A \to P) \simeq (\|A\| \to P)$. Prove this using the definitions.

**17.9.** Give an example of a type in HoTT that is not a set. (*Hint:* The circle $S^1$ has $\pi_1(S^1) = \mathbb{Z}$, so there are non-trivial loops at the basepoint.)

**17.10 (Challenge).** Define the *$n$-truncation* $\|A\|_n$ as a higher inductive type. State its universal property formally. Show that $\|A\|_{-2} \simeq \mathbf{1}$ iff $A$ is nonempty and $\|-\|_{-2}$ of any type is contractible (since there is only one contractible type up to equivalence).

---

## See Also

**In chapters/:**
- `ch16-identity-types` — Prerequisite. H-levels are defined in terms of path types: a type A is at h-level n iff its path types are at h-level n−1. The hierarchy bottoms out at contractible types (h-level −2): `isContr A := Σ(a : A), Π(b : A), a = b`.
- `ch18-univalence` — The universe `Type` is not a set: its path type `A =_{Type} B` is equivalent to `A ≃ B` by univalence, and equivalences do not always form a mere proposition. The universe is at h-level 1 (a groupoid) for propositions, but has no finite h-level overall — it is a genuine ∞-type.
- `ch19-higher-inductive-types` — Propositional truncation `‖A‖` (h-level −1 truncation) and general n-truncations are defined as HITs. The truncation modalities `‖–‖ₙ` are the HoTT analogs of Postnikov sections in classical homotopy theory.
- `ch20-synthetic-homotopy` — Connectedness (`n-connected maps`) and n-truncation interact in the Blakers-Massey theorem and the Freudenthal suspension theorem, both proved in ch20. The h-level hierarchy from ch17 is the primary organizational tool for those proofs.

**In book/:**
- `book/unit-06-core-hott/ch17-h-levels/` — Extended narrative explaining the h-level hierarchy as the HoTT refinement of the classical notion of "discrete" vs. "continuous" space. Emphasizes the philosophical significance of the distinction between `A` (a type with witnesses) and `‖A‖` (merely having a witness).

**In demos/:**
- `demos/demo_hlevels.py` — Computes h-levels for basic types (ℕ, Bool, S¹, Type), illustrating the hierarchy and the contrast between discrete and continuous types.
- `demos/demo_truncations.py` — Propositional truncation `‖A‖` and n-truncations as HITs. Demonstrates the universal property: maps from `‖A‖` into a proposition factor uniquely through `A`.

**The h-level hierarchy:**
| h-level | Name | Condition | Examples |
|---|---|---|---|
| −2 | Contractible | `Σ(a:A), Π(b:A), a=b` | `𝟙`, `Σ(b:A), a=b` |
| −1 | Mere proposition | `Π(a b:A), a=b` | `𝟘`, `𝟙`, `isProp A` |
| 0 | Set | `Π(a b:A), isProp(a=b)` | `ℕ`, `ℤ`, `Bool` |
| 1 | Groupoid | path types are sets | `S¹` (π₁=ℤ, but π₂=0) |
| n | n-groupoid | iterated paths reach sets | `Sⁿ` |
| ∞ | (no truncation) | | `Type` itself |

**Critical distinction — `Σ` vs. `‖Σ‖`:**
- `Σ(n : ℕ), P n` = "there exists n : ℕ with P n, and I have a witness"
- `‖Σ(n : ℕ), P n‖` = "there merely exists n : ℕ with P n (witness not retained)"
This distinction is invisible in classical set theory but is mathematically and computationally essential in HoTT.
