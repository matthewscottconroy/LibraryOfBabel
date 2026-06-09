# Chapter 2 — Relations, Functions, and Cardinality

**Part I: Mathematical Foundations**
*Prerequisites: [Chapter 1 — Logic, Sets, and Proof](ch01-logic-sets-proof.md)*
*Next: [Chapter 3 — Fields and Vector Spaces](ch03-fields-and-vector-spaces.md)*

---

## Learning Objectives

- Define and manipulate relations; classify them by reflexivity, symmetry, transitivity
- Work with equivalence relations and quotient sets
- Understand functions as special relations; injections, surjections, bijections
- Compute with function composition and inverses
- Grasp Cantor's theory of cardinality: countable vs. uncountable sets
- Apply Zorn's Lemma and the Axiom of Choice in algebraic arguments

---

## 2.1 Relations

### 2.1.1 Definition

A **relation** $R$ from set $A$ to set $B$ is a subset $R \subseteq A \times B$. When $A = B$ we say $R$ is a relation **on** $A$. We write $aRb$ to mean $(a,b) \in R$.

### 2.1.2 Properties of Relations on a Set

Let $R$ be a relation on $A$:

| Property | Definition |
|----------|------------|
| **Reflexive** | $\forall a \in A,\, aRa$ |
| **Irreflexive** | $\forall a \in A,\, \neg(aRa)$ |
| **Symmetric** | $aRb \Rightarrow bRa$ |
| **Antisymmetric** | $aRb \land bRa \Rightarrow a = b$ |
| **Transitive** | $aRb \land bRc \Rightarrow aRc$ |

### 2.1.3 Equivalence Relations

$R$ is an **equivalence relation** if it is reflexive, symmetric, and transitive.

**Equivalence class** of $a$: $[a]_R = \{b \in A \mid aRb\}$.

**Theorem (Partition Theorem):** The equivalence classes of $R$ on $A$ form a **partition** of $A$ — a collection of pairwise disjoint, non-empty subsets whose union is $A$. Conversely, every partition defines an equivalence relation.

**Examples:**
- Congruence modulo $n$: $a \equiv b \pmod{n}$ iff $n \mid (a - b)$
- "Same cardinality" on sets
- "Isomorphic" on groups / rings / vector spaces

### 2.1.4 Quotient Sets

The **quotient set** $A/R$ is the set of all equivalence classes: $A/R = \{[a] \mid a \in A\}$.

This is a fundamental construction. It appears as quotient groups, quotient rings, quotient spaces throughout algebra. The map $\pi: A \to A/R$ given by $\pi(a) = [a]$ is the **canonical projection**.

### 2.1.5 Partial and Total Orders

$R$ is a **partial order** (poset) on $A$ if it is reflexive, antisymmetric, and transitive. Written $(A, \leq)$.

$R$ is a **total order** (linear order) if additionally every pair is comparable: $\forall a,b,\, aRb \lor bRa$.

**Examples:**
- $\leq$ on $\mathbb{R}$ (total order)
- $\subseteq$ on $\mathcal{P}(S)$ (partial order, not total in general)
- Divisibility $\mid$ on $\mathbb{N}$ (partial order)

---

## 2.2 Functions

### 2.2.1 Functions as Relations

A **function** $f: A \to B$ is a relation $f \subseteq A \times B$ such that:
$$\forall a \in A,\, \exists! b \in B,\, (a,b) \in f$$

We write $f(a) = b$ for the unique $b$ paired with $a$.

- $A$ = **domain**, $B$ = **codomain**
- $f(A) = \{f(a) \mid a \in A\}$ = **image** or **range**
- For $S \subseteq A$: **direct image** $f(S) = \{f(s) \mid s \in S\}$
- For $T \subseteq B$: **preimage** $f^{-1}(T) = \{a \in A \mid f(a) \in T\}$

### 2.2.2 Injections, Surjections, Bijections

| Type | Property | Alternative |
|------|----------|-------------|
| **Injective** (one-to-one) | $f(a) = f(b) \Rightarrow a = b$ | $a \neq b \Rightarrow f(a) \neq f(b)$ |
| **Surjective** (onto) | $\forall b \in B,\, \exists a \in A,\, f(a) = b$ | $f(A) = B$ |
| **Bijective** | Injective and surjective | Has a two-sided inverse |

A bijection $f: A \to B$ witnesses that $A$ and $B$ are "the same size."

### 2.2.3 Composition and Inverses

**Composition:** $(g \circ f)(a) = g(f(a))$ for $f: A \to B$, $g: B \to C$.

Properties:
- Composition is associative: $(h \circ g) \circ f = h \circ (g \circ f)$
- $f$ injective and $g$ injective $\Rightarrow$ $g \circ f$ injective
- $f$ surjective and $g$ surjective $\Rightarrow$ $g \circ f$ surjective

**Inverse:** If $f: A \to B$ is bijective, its **inverse** $f^{-1}: B \to A$ satisfies $f^{-1} \circ f = \mathrm{id}_A$ and $f \circ f^{-1} = \mathrm{id}_B$.

**Left/right inverses:** $f$ has a left inverse iff $f$ is injective; right inverse iff $f$ is surjective (requires choice for the latter).

### 2.2.4 The Identity and Constant Functions

- $\mathrm{id}_A: A \to A$ with $\mathrm{id}_A(a) = a$
- Constant function $c_b: A \to B$ with $c_b(a) = b$ for all $a$

---

## 2.3 Cardinality

### 2.3.1 Finite Sets and Counting

Two sets $A$, $B$ have the **same cardinality** (written $|A| = |B|$ or $A \sim B$) if there exists a bijection $f: A \to B$.

For finite sets, this recovers ordinary counting: $|A| = n$ iff there is a bijection $A \to \{1, 2, \ldots, n\}$.

### 2.3.2 Countable and Uncountable Sets

A set $A$ is:
- **Countably infinite** if $A \sim \mathbb{N}$ (there exists a bijection $\mathbb{N} \to A$)
- **Countable** if finite or countably infinite
- **Uncountable** if not countable

**Theorem (Cantor):** $\mathbb{Z}$ and $\mathbb{Q}$ are countably infinite.

*Sketch for $\mathbb{Z}$:* Map $0, 1, -1, 2, -2, \ldots$

*Sketch for $\mathbb{Q}$:* Arrange fractions in a grid; follow a diagonal path, skipping repeats.

**Theorem (Cantor's Diagonal Argument):** $\mathbb{R}$ is uncountable.

*Proof.* Suppose $\mathbb{R}$ (or $(0,1)$) is countable: list all elements as $r_1, r_2, r_3, \ldots$ with decimal expansions. Construct $d$ whose $n$-th digit differs from the $n$-th digit of $r_n$. Then $d \notin \{r_1, r_2, \ldots\}$ — contradiction. $\square$

**Corollary:** $|\mathbb{R}| > |\mathbb{N}|$. In fact, $|\mathbb{R}| = |\mathcal{P}(\mathbb{N})|$.

### 2.3.3 Cantor's Theorem

**Theorem:** For any set $A$, $|A| < |\mathcal{P}(A)|$.

The diagonal argument generalizes: the map $A \to \mathcal{P}(A)$ cannot be surjective. This gives an infinite hierarchy:

$$|\mathbb{N}| < |\mathcal{P}(\mathbb{N})| < |\mathcal{P}(\mathcal{P}(\mathbb{N}))| < \cdots$$

### 2.3.4 The Schröder–Bernstein Theorem

**Theorem:** If there exist injections $f: A \hookrightarrow B$ and $g: B \hookrightarrow A$, then $|A| = |B|$.

This is non-obvious: two injections in opposite directions imply a bijection. The proof constructs the bijection explicitly.

**Significance:** Allows comparison of cardinalities without explicit bijection construction.

---

## 2.4 The Axiom of Choice and Its Equivalents

### 2.4.1 The Axiom of Choice (AC)

**Axiom of Choice:** For any collection $\{A_i\}_{i \in I}$ of non-empty sets, there exists a function $f: I \to \bigcup_i A_i$ with $f(i) \in A_i$ for all $i$.

Such $f$ is called a **choice function**. This is non-trivial when $I$ is infinite.

### 2.4.2 Zorn's Lemma

**Zorn's Lemma:** If $(P, \leq)$ is a non-empty poset in which every chain (totally ordered subset) has an upper bound in $P$, then $P$ has at least one maximal element.

**Equivalent to AC.** This is the most useful form for algebra.

**Applications:**
- Every vector space has a basis (Hamel basis theorem)
- Every ring has a maximal ideal
- Every field has an algebraic closure
- Every group has a maximal proper subgroup (not always, but in important cases)

### 2.4.3 Well-Ordering Theorem

**Well-Ordering Theorem:** Every set can be well-ordered (i.e., totally ordered so every non-empty subset has a least element).

Also equivalent to AC. The well-ordering of $\mathbb{R}$ cannot be exhibited explicitly.

### 2.4.4 Using Zorn's Lemma

**Template for Zorn arguments:**
1. Define a poset $P$ of "partial solutions" with an order by extension
2. Show every chain has an upper bound (take the union)
3. Conclude a maximal element exists
4. Show maximality implies the element is a "complete solution"

**Example:** Every vector space has a basis.
*Proof.* Let $P = \{S \subseteq V \mid S \text{ is linearly independent}\}$, ordered by inclusion. Every chain $S_1 \subseteq S_2 \subseteq \cdots$ has upper bound $\bigcup_i S_i$. By Zorn, a maximal element $B$ exists. Claim: $B$ spans $V$ (if not, add any vector not in $\mathrm{span}(B)$ to get a strictly larger independent set, contradicting maximality). So $B$ is a basis. $\square$

---

## 2.5 Key Theorems Summary

| Theorem | Statement |
|---------|-----------|
| Partition Theorem | Equivalence relations $\leftrightarrow$ partitions (bijection) |
| Cantor's Diagonal | $\mathbb{R}$ is uncountable |
| Cantor's Theorem | $\|A\| < \|\mathcal{P}(A)\|$ for any set $A$ |
| Schröder–Bernstein | $f: A \hookrightarrow B$ and $g: B \hookrightarrow A$ imply $\|A\| = \|B\|$ |
| Zorn's Lemma | Chains have upper bounds $\Rightarrow$ maximal elements exist |

---

## Milestone Exercises

1. Let $A = \{1,2,3,4,5,6\}$ with $a \sim b$ iff $3 \mid (a-b)$. List all equivalence classes. What is $A/{\sim}$?

2. Prove: if $f: A \to B$ is injective and $g: B \to C$ is injective, then $g \circ f$ is injective.

3. Prove that $\mathbb{Z} \times \mathbb{Z}$ is countable.

4. Prove Cantor's theorem: $|A| < |\mathcal{P}(A)|$.

5. Prove the Schröder–Bernstein theorem (this is a substantial exercise — look up the proof and understand every step).

6. Using Zorn's Lemma, prove every commutative ring with identity has a maximal ideal (you will need ring theory vocabulary — return to this after Chapter 11).

7. Show that the following are equivalent (modulo standard set theory): (a) Axiom of Choice, (b) Zorn's Lemma, (c) Well-Ordering Theorem.

---

## Connections Forward

- **Chapter 3:** Vector spaces are defined over fields; bases exist by the Zorn's Lemma argument above.
- **Chapter 9:** Cosets of a subgroup $H \leq G$ form a partition of $G$ — exactly the partition theorem applied.
- **Chapter 11:** Maximal ideals exist by Zorn; quotient rings are quotient sets with extra structure.
- **Chapter 13:** The algebraic closure of a field is constructed using Zorn's Lemma.
- **Chapter 14:** Functions are morphisms in the category **Set**; bijections are isomorphisms.
- **Chapter 24:** The full story of AC, large cardinals, and independence is in axiomatic set theory.

---

*Next: [Chapter 3 — Fields and Vector Spaces](ch03-fields-and-vector-spaces.md)*
