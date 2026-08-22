# Chapter 24 — Axiomatic Set Theory

**Part VI: Foundations of Mathematics**
*Prerequisites: [Chapter 2](ch02-relations-functions-cardinality.md)*
*Next: [Chapter 25 — Model Theory](ch25-model-theory.md)*

---

## Learning Objectives

- Understand why naive set theory is insufficient; motivate ZFC
- Know and understand each ZFC axiom
- Work with ordinals and cardinals rigorously
- Understand the Axiom of Choice and its independence from ZF
- Get an introduction to forcing and independence proofs
- See how the foundations of mathematics are themselves a mathematical subject

---

## 24.1 The Need for Axiomatic Set Theory

### 24.1.1 Russell's Paradox Revisited

Naive set theory allows $R = \{x \mid x \notin x\}$, which gives $R \in R \Leftrightarrow R \notin R$.

**The resolution:** Restrict which collections are "sets." Proper classes (like "all sets") are not sets.

**ZFC** (Zermelo–Fraenkel with Choice) is the standard axiomatic foundation.

### 24.1.2 The Language

First-order logic with equality and one binary predicate: $\in$ (membership).

All mathematical objects are sets. The only primitive notion: "$x \in y$" — "$x$ is an element of $y$."

---

## 24.2 The ZFC Axioms

### ZF1. Extensionality
$$\forall x\, \forall y\, [(\forall z,\, z \in x \Leftrightarrow z \in y) \Rightarrow x = y]$$
Sets are determined by their elements.

### ZF2. Empty Set
$$\exists x\, \forall y\, y \notin x$$
The empty set $\emptyset$ exists.

### ZF3. Pairing
$$\forall x\, \forall y\, \exists z\, \forall w\, [w \in z \Leftrightarrow w = x \lor w = y]$$
For any $x, y$, the set $\{x, y\}$ exists.

### ZF4. Union
$$\forall \mathcal{F}\, \exists A\, \forall y\, [y \in A \Leftrightarrow \exists B \in \mathcal{F},\, y \in B]$$
The union of a family of sets exists: $\bigcup \mathcal{F}$.

### ZF5. Power Set
$$\forall x\, \exists y\, \forall z\, [z \in y \Leftrightarrow z \subseteq x]$$
$\mathcal{P}(x) = \{z \mid z \subseteq x\}$ exists.

### ZF6. Separation (Schema)
$$\forall x\, \exists y\, \forall z\, [z \in y \Leftrightarrow z \in x \land \phi(z)]$$
For any formula $\phi$ and set $x$: $\{z \in x \mid \phi(z)\}$ exists. (One axiom per formula $\phi$.)

### ZF7. Replacement (Schema)
If $\phi(x, y)$ is functional (each $x$ determines a unique $y$), then the image of any set under $\phi$ is a set.

### ZF8. Infinity
$$\exists x\, [\emptyset \in x \land \forall y \in x,\, y \cup \{y\} \in x]$$
An infinite set exists (the natural numbers are constructed from this).

### ZF9. Foundation (Regularity)
$$\forall x\, [x \neq \emptyset \Rightarrow \exists y \in x,\, y \cap x = \emptyset]$$
Every non-empty set has an $\in$-minimal element. Eliminates self-membership ($x \in x$).

### AC. Axiom of Choice
$$\forall \mathcal{F}\, [\emptyset \notin \mathcal{F} \Rightarrow \exists f: \mathcal{F} \to \bigcup \mathcal{F},\, \forall A \in \mathcal{F},\, f(A) \in A]$$
Every family of non-empty sets has a choice function. (Equivalent to Zorn's Lemma, Well-Ordering Theorem.)

---

## 24.3 Ordinals and Cardinals

### 24.3.1 Ordinals

A set $\alpha$ is an **ordinal** if:
- It is **transitive:** $x \in \alpha \Rightarrow x \subseteq \alpha$
- It is **well-ordered** by $\in$

The ordinals are: $0 = \emptyset$, $1 = \{\emptyset\}$, $2 = \{0, 1\}$, ..., $\omega = \{0, 1, 2, \ldots\}$, $\omega + 1 = \omega \cup \{\omega\}$, ...

**Successor:** $\alpha + 1 = \alpha \cup \{\alpha\}$. **Limit ordinal:** no immediate predecessor (e.g., $\omega, \omega \cdot 2, \omega^2, \ldots$).

**Transfinite induction:** Prove $\phi(\alpha)$ for all ordinals $\alpha$ by:
- $\phi(0)$
- $\phi(\alpha) \Rightarrow \phi(\alpha+1)$
- For limit $\lambda$: $(\forall \beta < \lambda, \phi(\beta)) \Rightarrow \phi(\lambda)$

### 24.3.2 Cardinals

A **cardinal** $\kappa$ is an ordinal that is not bijective with any smaller ordinal.

Cardinals: $0, 1, 2, \ldots, \aleph_0, \aleph_1, \aleph_2, \ldots, \aleph_\omega, \ldots$

$\aleph_0 = |\mathbb{N}|$, $\aleph_1 =$ smallest uncountable cardinal, etc.

**Cardinal arithmetic:**
- $\kappa + \lambda = |\kappa \sqcup \lambda|$
- $\kappa \cdot \lambda = |\kappa \times \lambda|$
- $\kappa^\lambda = |{}^\lambda \kappa|$ (set of functions $\lambda \to \kappa$)

For infinite cardinals: $\kappa + \kappa = \kappa$, $\kappa \cdot \kappa = \kappa$ (under AC).

### 24.3.3 The Continuum Hypothesis

$|\mathbb{R}| = 2^{\aleph_0}$. What is this?

**Continuum Hypothesis (CH):** $2^{\aleph_0} = \aleph_1$ (there is no set of cardinality strictly between $|\mathbb{N}|$ and $|\mathbb{R}|$).

**Generalized CH (GCH):** $2^{\aleph_\alpha} = \aleph_{\alpha+1}$ for all $\alpha$.

---

## 24.4 Independence and Forcing

### 24.4.1 Gödel's Completeness and Incompleteness Theorems

**Completeness (1930):** A sentence is provable from a set of axioms iff it is true in every model.

**First Incompleteness Theorem (1931):** Any consistent recursive axiom system strong enough to express Peano Arithmetic is **incomplete** — there exist sentences neither provable nor disprovable.

**Second Incompleteness Theorem:** Such a system cannot prove its own consistency.

**Application to ZFC:** ZFC (if consistent) cannot prove its own consistency. The independence results below rely on this.

### 24.4.2 Gödel's Constructible Universe $L$

**Theorem (Gödel, 1938):** $\mathrm{Con}(\mathrm{ZF}) \Rightarrow \mathrm{Con}(\mathrm{ZF} + \mathrm{AC} + \mathrm{GCH})$.

Gödel defined the **constructible universe** $L = \bigcup_\alpha L_\alpha$ (sets "constructible in stages"), and showed $L$ is a model of ZF + AC + GCH.

**Consequence:** CH cannot be disproved from ZF alone.

### 24.4.3 Cohen's Forcing

**Theorem (Cohen, 1963):** $\mathrm{Con}(\mathrm{ZF}) \Rightarrow \mathrm{Con}(\mathrm{ZF} + \mathrm{AC} + \neg\mathrm{CH})$.

Cohen introduced **forcing**: a technique to extend a model of ZF by adding "generic" objects, creating a new model satisfying desired properties.

**Consequence:** CH cannot be proved from ZFC either. CH is **independent** of ZFC.

This was the first independence result obtained by forcing — it revolutionized set theory.

### 24.4.4 Large Cardinals

Axioms asserting the existence of cardinals with special properties (inaccessibles, measurables, Woodin cardinals, etc.) form a **large cardinal hierarchy**. These are not provable from ZFC but are not known to be contradictory.

Large cardinal axioms imply consistency of many weaker theories. Their study is the frontier of set-theoretic foundations.

---

## 24.5 Ordinal Arithmetic and Transfinite Induction

### 24.5.1 Transfinite Recursion

Define functions on ordinals by:
- $f(0) =$ initial value
- $f(\alpha+1) = g(f(\alpha))$ (successor step)
- $f(\lambda) = h(\{f(\beta) \mid \beta < \lambda\})$ (limit step)

This generalizes ordinary recursion. Used to define $\aleph_\alpha$, the Von Neumann universe $V = \bigcup_\alpha V_\alpha$, and the constructible universe $L$.

### 24.5.2 The Von Neumann Universe

$$V_0 = \emptyset, \quad V_{\alpha+1} = \mathcal{P}(V_\alpha), \quad V_\lambda = \bigcup_{\alpha < \lambda} V_\alpha$$

$$V = \bigcup_\alpha V_\alpha$$

The Foundation Axiom ensures every set is in some $V_\alpha$.

The **rank** of a set $x$ is the least $\alpha$ with $x \in V_{\alpha+1}$.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Gödel's incompleteness | ZFC (if consistent) is incomplete |
| Gödel $L$ | $\mathrm{Con}(\mathrm{ZF}) \Rightarrow \mathrm{Con}(\mathrm{ZFC} + \mathrm{GCH})$ |
| Cohen forcing | $\mathrm{Con}(\mathrm{ZF}) \Rightarrow \mathrm{Con}(\mathrm{ZFC} + \neg\mathrm{CH})$ |
| Independence of AC | AC is independent of ZF |

---

## Milestone Exercises

1. From the ZFC axioms, prove that the intersection $A \cap B = \{x \in A \mid x \in B\}$ exists (use Separation).

2. Construct the natural numbers in ZF: define $0 = \emptyset$, $1 = \{0\}$, $2 = \{0,1\}$, etc. Prove the Infinity Axiom gives a set containing all of them.

3. Show that ordinal addition is not commutative: $1 + \omega \neq \omega + 1$.

4. Prove: the ordinals are well-ordered by $\in$ (i.e., every non-empty set of ordinals has a least element).

5. Show $|\mathbb{R}| = 2^{\aleph_0}$ using Schröder-Bernstein.

6. Verify that $V = \bigcup_\alpha V_\alpha$ satisfies the ZFC axioms (a sketch — this is a substantial exercise).

7. Research Gödel's incompleteness theorem: what is the Gödel sentence $G$ that is true but unprovable? Write a 1-page explanation.

---

## Connections Forward

- **Chapter 25:** Model theory studies the relationship between formal theories and their models; independence results are fundamentally model-theoretic.
- **Chapter 26:** Category-theoretic foundations (toposes) provide an alternative to set-theoretic foundations that avoids some of ZFC's pathologies.
- **Chapter 27:** Homotopy type theory is a new foundations based on type theory and homotopy, where the Univalence Axiom plays the role of an independence result.

---

*Next: [Chapter 25 — Model Theory](ch25-model-theory.md)*
