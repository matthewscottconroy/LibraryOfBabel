# Chapter 1 — Logic, Sets, and Proof

**Part I: Mathematical Foundations**
*Prerequisites: None. This is the ground floor.*
*Next: [Chapter 2 — Relations, Functions, and Cardinality](ch02-relations-functions-cardinality.md)*

---

## Learning Objectives

By the end of this chapter you should be able to:
- Read and write formal mathematical statements using logical connectives and quantifiers
- Construct rigorous proofs by direct argument, contradiction, contrapositive, and induction
- Work fluently with sets, operations on sets, and the naive set-theoretic universe
- Identify and avoid common logical errors
- Understand what an axiomatic system is and why it matters

---

## 1.1 Propositional Logic

### 1.1.1 Propositions and Connectives

A **proposition** is a statement that is either true or false. The basic connectives:

| Symbol | Name | Read as |
|--------|------|---------|
| $\neg P$ | Negation | "not $P$" |
| $P \land Q$ | Conjunction | "$P$ and $Q$" |
| $P \lor Q$ | Disjunction | "$P$ or $Q$" (inclusive) |
| $P \Rightarrow Q$ | Implication | "if $P$ then $Q$" |
| $P \Leftrightarrow Q$ | Biconditional | "$P$ if and only if $Q$" |

**Truth tables** define the semantics of each connective. Key identity: $P \Rightarrow Q \equiv \neg P \lor Q$.

### 1.1.2 Tautologies and Logical Equivalence

A **tautology** is a proposition true under every assignment of truth values. Two propositions are **logically equivalent** ($P \equiv Q$) if $P \Leftrightarrow Q$ is a tautology.

Essential tautologies:
- *Modus ponens:* $[(P \Rightarrow Q) \land P] \Rightarrow Q$
- *Modus tollens:* $[(P \Rightarrow Q) \land \neg Q] \Rightarrow \neg P$
- *Hypothetical syllogism:* $[(P \Rightarrow Q) \land (Q \Rightarrow R)] \Rightarrow (P \Rightarrow R)$
- *De Morgan's laws:* $\neg(P \land Q) \equiv \neg P \lor \neg Q$ and $\neg(P \lor Q) \equiv \neg P \land \neg Q$

### 1.1.3 Proof Strategies for Implications

Given $P \Rightarrow Q$:
- **Direct proof:** Assume $P$; derive $Q$
- **Contrapositive:** Prove $\neg Q \Rightarrow \neg P$ (logically equivalent)
- **Contradiction:** Assume $P \land \neg Q$; derive $\bot$ (a contradiction)

---

## 1.2 Predicate Logic and Quantifiers

### 1.2.1 Predicates

A **predicate** $P(x)$ is a proposition-valued function. The **universal quantifier** $\forall$ and **existential quantifier** $\exists$:

$$\forall x \in S,\, P(x) \qquad \exists x \in S,\, P(x)$$

### 1.2.2 Negating Quantified Statements

$$\neg(\forall x,\, P(x)) \equiv \exists x,\, \neg P(x)$$
$$\neg(\exists x,\, P(x)) \equiv \forall x,\, \neg P(x)$$

This is the engine behind most counterexample arguments. To disprove $\forall x, P(x)$, find a single $x$ where $P(x)$ fails.

### 1.2.3 Nested Quantifiers

Order matters: $\forall x\, \exists y,\, R(x,y)$ and $\exists y\, \forall x,\, R(x,y)$ are different. The second is strictly stronger.

**Key example:** $\forall \varepsilon > 0\, \exists \delta > 0$ (the analytic definition of continuity) vs. $\exists \delta > 0\, \forall \varepsilon > 0$ (uniform continuity with swapped roles — understand why these differ).

---

## 1.3 Naive Set Theory

### 1.3.1 Sets and Membership

A **set** is an unordered collection of distinct objects called **elements** or **members**. Notation: $x \in S$ (element of), $x \notin S$ (not element of).

Set-builder notation: $S = \{x \mid P(x)\}$ — the set of all $x$ satisfying predicate $P$.

Standard sets:
- $\emptyset = \{\}$ — the empty set
- $\mathbb{N} = \{0, 1, 2, \ldots\}$ — natural numbers (note: some authors exclude 0)
- $\mathbb{Z} = \{\ldots, -2, -1, 0, 1, 2, \ldots\}$
- $\mathbb{Q}, \mathbb{R}, \mathbb{C}$

### 1.3.2 Set Operations

Let $A, B \subseteq U$ (a universal set).

| Operation | Notation | Definition |
|-----------|----------|------------|
| Union | $A \cup B$ | $\{x \mid x \in A \text{ or } x \in B\}$ |
| Intersection | $A \cap B$ | $\{x \mid x \in A \text{ and } x \in B\}$ |
| Difference | $A \setminus B$ | $\{x \mid x \in A \text{ and } x \notin B\}$ |
| Complement | $A^c$ | $U \setminus A$ |
| Power set | $\mathcal{P}(A)$ | $\{S \mid S \subseteq A\}$ |
| Cartesian product | $A \times B$ | $\{(a,b) \mid a \in A, b \in B\}$ |

**Laws:** Commutativity, associativity, distributivity, De Morgan's laws for sets (mirror the logical laws).

### 1.3.3 Indexed Families

For a family of sets indexed by $I$:

$$\bigcup_{i \in I} A_i = \{x \mid \exists i \in I,\, x \in A_i\}$$
$$\bigcap_{i \in I} A_i = \{x \mid \forall i \in I,\, x \in A_i\}$$

### 1.3.4 Russell's Paradox

The naive comprehension axiom ("$\{x \mid P(x)\}$ is always a set") leads to contradiction. Let $R = \{x \mid x \notin x\}$. Then $R \in R \Leftrightarrow R \notin R$. This motivates axiomatic set theory (see Chapter 24).

---

## 1.4 Methods of Proof

### 1.4.1 Direct Proof

**Structure:** Assume hypotheses; chain implications; arrive at conclusion.

**Example:** Prove: if $n$ is odd, then $n^2$ is odd.
*Proof.* Suppose $n$ is odd. Then $n = 2k+1$ for some $k \in \mathbb{Z}$. Then $n^2 = (2k+1)^2 = 4k^2 + 4k + 1 = 2(2k^2+2k)+1$, which is odd. $\square$

### 1.4.2 Proof by Contradiction

**Structure:** Assume $\neg Q$; derive contradiction $\bot$; conclude $Q$.

**Example:** Prove $\sqrt{2}$ is irrational.
*Proof.* Suppose $\sqrt{2} = p/q$ in lowest terms. Then $2q^2 = p^2$, so $p^2$ is even, so $p$ is even. Write $p = 2m$. Then $2q^2 = 4m^2$, so $q^2 = 2m^2$, so $q$ is even. But then $p/q$ is not in lowest terms — contradiction. $\square$

### 1.4.3 Mathematical Induction

**Weak induction:** To prove $\forall n \geq n_0, P(n)$:
1. **Base case:** Verify $P(n_0)$
2. **Inductive step:** Assuming $P(k)$ (inductive hypothesis), prove $P(k+1)$

**Strong induction:** In the inductive step, assume $P(j)$ for all $n_0 \leq j \leq k$, then prove $P(k+1)$.

**Well-ordering principle:** Every non-empty subset of $\mathbb{N}$ has a least element. Equivalent to induction over $\mathbb{N}$.

**Structural induction:** Induction on recursively-defined structures (trees, expressions, derivations). The same pattern applied to a well-founded ordering.

### 1.4.4 Existence and Uniqueness

- **Existence:** Construct an object (constructive) or argue non-existence leads to contradiction (non-constructive / existence by contradiction)
- **Uniqueness:** Assume two objects $x, y$ satisfy the property; show $x = y$

The pattern "$\exists !$" (there exists a unique) combines both.

---

## 1.5 The Axiomatic Method

### 1.5.1 What Is an Axiomatic System?

An axiomatic system consists of:
- **Primitive terms** — undefined concepts taken as given
- **Axioms** — statements accepted without proof
- **Definitions** — names for derived concepts
- **Theorems** — statements proved from axioms and prior theorems

**Ideal properties:** Consistency (no contradiction derivable), completeness (every true statement provable), independence (no axiom provable from others).

**Gödel's incompleteness theorems** (preview): Any consistent axiomatic system strong enough to express arithmetic is either incomplete or inconsistent. This is the shadow the cathedral casts — even foundations have limits.

### 1.5.2 Why Rigor?

Intuition fails at scale. The history of mathematics is full of "obvious" statements that are false and "impossible" statements that are true. Rigor is not pedantry — it is the only reliable mechanism for building structures that hold under their own weight.

---

## 1.6 Key Theorems and Results

| Result | Statement |
|--------|-----------|
| De Morgan's Laws | $\neg(P \land Q) \equiv \neg P \lor \neg Q$ |
| Double Negation | $\neg\neg P \equiv P$ |
| Contrapositive | $P \Rightarrow Q \equiv \neg Q \Rightarrow \neg P$ |
| Well-Ordering of $\mathbb{N}$ | Every non-empty $S \subseteq \mathbb{N}$ has a minimum |
| Induction $\equiv$ Well-Ordering | The two principles are equivalent over $\mathbb{N}$ |

---

## Milestone Exercises

1. Prove or disprove: For all integers $n$, if $n^2$ is divisible by 4, then $n$ is divisible by 4.

2. Prove by induction: $\displaystyle\sum_{k=1}^{n} k = \frac{n(n+1)}{2}$ for all $n \geq 1$.

3. Prove by strong induction: Every integer $n \geq 2$ has a prime factorization.

4. Use De Morgan's laws to negate: $\forall \varepsilon > 0\, \exists N \in \mathbb{N},\, \forall n \geq N,\, |a_n - L| < \varepsilon$.

5. Show that $\{x \in \mathbb{R} \mid x^2 = -1\} = \emptyset$. What is $\mathcal{P}(\emptyset)$?

6. Prove the distributive law for sets: $A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$.

7. Where does the following "proof" go wrong? "Claim: $1 = 2$. Proof: Let $a = b$. Then $a^2 = ab$, so $a^2 - b^2 = ab - b^2$, so $(a-b)(a+b) = b(a-b)$, so $a+b = b$, so $2b = b$, so $2 = 1$."

---

## Connections Forward

- **Chapter 2:** Relations and functions are defined purely in terms of sets and logic.
- **Chapter 9:** Group axioms are propositional statements; proving group properties uses exactly the proof techniques here.
- **Chapter 13:** Galois theory lives inside predicate logic — field extensions satisfy precisely stated axioms.
- **Chapter 24:** Axiomatic set theory (ZFC) revisits the foundations here with full rigor.

---

*Next: [Chapter 2 — Relations, Functions, and Cardinality](ch02-relations-functions-cardinality.md)*
