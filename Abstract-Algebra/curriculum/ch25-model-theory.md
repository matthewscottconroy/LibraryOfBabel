# Chapter 25 — Model Theory

**Part VI: Foundations of Mathematics**
*Prerequisites: [Chapter 24](ch24-set-theory-logic.md), [Chapter 11](ch11-ring-theory.md)*
*Next: [Chapter 26 — Category Theory as Foundation](ch26-category-theory-foundation.md)*

---

## Learning Objectives

- Define first-order logic, structures, and theories rigorously
- Prove the completeness and compactness theorems
- Understand elementary equivalence and the Löwenheim–Skolem theorems
- Apply model theory to algebra: Ax's theorem, Nullstellensatz, transfer principles
- Understand ultraproducts and nonstandard analysis
- See how model theory measures what can and cannot be expressed in first-order logic

---

## 25.1 Structures and Languages

### 25.1.1 First-Order Languages

A **first-order language** $\mathcal{L}$ consists of:
- **Constant symbols:** $c_1, c_2, \ldots$
- **Function symbols:** $f_1, f_2, \ldots$ with arities
- **Relation symbols:** $R_1, R_2, \ldots$ with arities

**Examples:**
| Theory | Language |
|--------|----------|
| Ordered fields | $\{0, 1, +, \cdot, <\}$ |
| Groups | $\{e, \cdot, {}^{-1}\}$ |
| Rings | $\{0, 1, +, -, \cdot\}$ |
| Set theory | $\{\in\}$ |
| Graphs | $\{E\}$ (binary relation) |

### 25.1.2 Structures

An **$\mathcal{L}$-structure** $\mathcal{M}$ consists of:
- A non-empty set $M$ (the **domain** or **universe**)
- An interpretation of each symbol: constants, functions, and relations on $M$

**Examples:** $(\mathbb{R}, 0, 1, +, \cdot, <)$, $(\mathbb{Z}, 0, 1, +, -, \cdot)$, any group $(G, e, \cdot, {}^{-1})$.

### 25.1.3 Formulas and Satisfaction

**Terms:** Built from constants, variables, and function symbols.

**Formulas:** Built from atomic formulas (equalities and relations), connectives ($\land, \lor, \neg, \Rightarrow$), and quantifiers ($\forall, \exists$).

**Satisfaction:** $\mathcal{M} \models \phi[a_1, \ldots, a_n]$ — "$\mathcal{M}$ satisfies $\phi$ with assignment $(a_1, \ldots, a_n)$."

A **sentence** is a formula with no free variables. $\mathcal{M} \models \sigma$ (unambiguously).

---

## 25.2 Theories and Models

### 25.2.1 Theories

A **theory** $T$ is a set of $\mathcal{L}$-sentences. A **model** of $T$ is a structure $\mathcal{M}$ with $\mathcal{M} \models \sigma$ for all $\sigma \in T$.

The **theory of a structure** $\mathrm{Th}(\mathcal{M}) = \{\sigma \mid \mathcal{M} \models \sigma\}$.

A theory is:
- **Consistent** if it has a model
- **Complete** if for every sentence $\sigma$, either $T \models \sigma$ or $T \models \neg\sigma$
- **Satisfiable** if consistent

### 25.2.2 Elementary Equivalence and Embeddings

$\mathcal{M} \equiv \mathcal{N}$ (**elementarily equivalent**) if $\mathrm{Th}(\mathcal{M}) = \mathrm{Th}(\mathcal{N})$ (same first-order theory).

$f: \mathcal{M} \to \mathcal{N}$ is an **elementary embedding** if $\mathcal{M} \models \phi[a_1, \ldots, a_n] \Leftrightarrow \mathcal{N} \models \phi[f(a_1), \ldots, f(a_n)]$ for all formulas $\phi$ and all $a_i \in M$.

---

## 25.3 The Compactness Theorem

### 25.3.1 Statement

**Theorem (Compactness):** A set $T$ of sentences has a model iff every finite subset of $T$ has a model.

**Proof:** Via Gödel's completeness theorem: $T$ has a model iff $T$ is consistent iff no finite subset is contradictory.

### 25.3.2 Applications

**Nonstandard arithmetic:** Add sentences $\{c > n \mid n \in \mathbb{N}\}$ to the theory of $\mathbb{N}$. Every finite subset has a model (take $c$ large enough); by compactness, there is a model of the whole theory. This model contains "infinite natural numbers" — nonstandard models.

**Nonstandard analysis:** Similarly, there exist ordered fields ${}^*\mathbb{R} \supset \mathbb{R}$ satisfying all first-order sentences of $\mathbb{R}$, but containing infinitesimals (nonzero elements smaller than $1/n$ for all $n$).

**Transfer principle:** Any first-order sentence true in $\mathbb{R}$ is true in ${}^*\mathbb{R}$ and vice versa. This justifies nonstandard analysis as a foundation for calculus.

---

## 25.4 Löwenheim–Skolem Theorems

### 25.4.1 Downward Löwenheim–Skolem

**Theorem:** If a countable theory $T$ has an infinite model, it has a countable model.

**Skolem's paradox:** ZFC is a countable theory (in a countable language), yet it proves the existence of uncountable sets. How can a countable model of ZFC contain an "uncountable" set? Resolution: "uncountable" is internally measured; the model's "uncountable set" is externally countable.

### 25.4.2 Upward Löwenheim–Skolem

**Theorem:** If $T$ has an infinite model of cardinality $\kappa$, then for all $\lambda \geq \kappa$, $T$ has a model of cardinality $\lambda$.

**Consequence:** No first-order theory can characterize a structure up to isomorphism if that structure is infinite. $\mathbb{R}$ cannot be pinned down by first-order sentences — only by the second-order axiom of completeness.

---

## 25.5 Ultraproducts and Ultrapowers

### 25.5.1 Ultrafilters

A **filter** on $I$ is a family $\mathcal{F} \subseteq \mathcal{P}(I)$ closed under supersets and finite intersections. An **ultrafilter** is a maximal filter.

**Key property:** For an ultrafilter $\mathcal{U}$ on $I$: for every $A \subseteq I$, either $A \in \mathcal{U}$ or $I \setminus A \in \mathcal{U}$.

### 25.5.2 The Ultraproduct Construction

Given a family of structures $\{\mathcal{M}_i\}_{i \in I}$ and an ultrafilter $\mathcal{U}$ on $I$, the **ultraproduct**:
$$\prod_{i \in I} \mathcal{M}_i / \mathcal{U}$$

has domain $\prod_{i \in I} M_i / \sim$ where $(a_i) \sim (b_i)$ iff $\{i \mid a_i = b_i\} \in \mathcal{U}$.

**Łoś's theorem:** $\prod \mathcal{M}_i / \mathcal{U} \models \phi([a_i]/\mathcal{U})$ iff $\{i \mid \mathcal{M}_i \models \phi(a_i)\} \in \mathcal{U}$.

The ultraproduct satisfies exactly the sentences that "almost all" $\mathcal{M}_i$ satisfy.

An **ultrapower** is an ultraproduct where all $\mathcal{M}_i = \mathcal{M}$: $\mathcal{M}^I / \mathcal{U}$.

---

## 25.6 Model Theory and Algebra

### 25.6.1 Complete Theories in Algebra

**Theorem (Tarski):** The complete ordered field $\mathbb{R}$ has a complete and decidable first-order theory (**real closed fields** — the theory of real closed fields is decidable).

**Algebraically closed fields:** $\mathrm{ACF}_p$ (algebraically closed fields of characteristic $p$) is complete and decidable.

### 25.6.2 Ax's Theorem

**Theorem (Ax, 1968):** If $f: \mathbb{C}^n \to \mathbb{C}^n$ is an injective polynomial map, then it is surjective.

**Model-theoretic proof:** 
1. Injectivity implies surjectivity for polynomial maps over finite fields $\mathbb{F}_q$ (easy: finite cardinality argument)
2. Pass to ultraproduct of $\mathbb{F}_q$ for varying $q$ to get a model of $\mathrm{ACF}_0$
3. By completeness of $\mathrm{ACF}_0$, the result holds in $\mathbb{C}$

This proof is much shorter than purely algebraic proofs.

### 25.6.3 The Model Theory of Groups and Rings

- **Tarski's problem:** Is the first-order theory of free groups decidable? (Proved by Kharlampovich–Myasnikov and Sela, independently, 2006.)
- **Model theory of $\mathbb{Z}$:** Undecidable (Gödel). Hilbert's 10th problem: no algorithm to determine solvability of Diophantine equations (Matiyasevich, 1970).
- **O-minimality:** Real closed fields are o-minimal — a geometric tameness condition with applications to Hodge theory and number theory.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Compactness | $T$ has model $\Leftrightarrow$ every finite $T_0 \subseteq T$ has model |
| Completeness (Gödel) | $T \models \sigma$ iff $T \vdash \sigma$ (syntax = semantics) |
| Downward Löwenheim–Skolem | Infinite countable theory $\Rightarrow$ countable model |
| Upward Löwenheim–Skolem | Model of size $\kappa \Rightarrow$ model of all sizes $\geq \kappa$ |
| Łoś's theorem | Ultraproducts satisfy $\phi$ iff "almost all" factors do |
| Ax's theorem | Injective polynomial $\mathbb{C}^n \to \mathbb{C}^n$ is surjective |

---

## Milestone Exercises

1. Write the axioms of groups in the language $\{e, \cdot, {}^{-1}\}$. Write the statement "every element has order dividing $n$."

2. Use compactness to show: if a sentence $\sigma$ is true in every field of characteristic 0, then there exists $N$ such that $\sigma$ is true in all fields of characteristic $p > N$.

3. Prove Łoś's theorem for atomic formulas.

4. Show that no first-order theory can have exactly two infinite models up to isomorphism.

5. Prove: $\mathbb{Q} \equiv \mathbb{R}$ is false — find a sentence true in $\mathbb{R}$ but not $\mathbb{Q}$.

6. Formulate (but not necessarily prove) why Hilbert's 10th problem cannot be decided: sketch the connection to the undecidability of provability in Peano Arithmetic.

7. Use Łoś and the Ax theorem argument to prove: if a polynomial map $f: \mathbb{C}^n \to \mathbb{C}^n$ is injective, it is surjective.

---

## Connections Forward

- **Chapter 26:** Categorical logic formalizes the semantic/syntactic relationship differently, using categories as "generalized universes."
- **Chapter 27:** Type theory provides a constructive alternative to classical model theory, with Curry-Howard correspondence.

---

*Next: [Chapter 26 — Category Theory as Foundation](ch26-category-theory-foundation.md)*
