# Chapter 6 Overview: Set Theory

---

## Central Question

Can we provide a single, coherent foundation for all of mathematics — a few axioms from which every mathematical theorem can in principle be derived? And how do we do this without inadvertently making the system inconsistent?

Set theory, specifically Zermelo-Fraenkel set theory with the Axiom of Choice (ZFC), is the answer that dominated the twentieth century. Its history includes some of mathematics' most dramatic crises and its most brilliant solutions.

---

## Why This Chapter Matters

Every mathematical object — numbers, functions, relations, spaces — can be encoded as a set. The entire body of standard mathematics is, in principle, derivable from the ZFC axioms. Understanding ZFC provides: (a) the foundational context for formal proof; (b) a working knowledge of ordinals and cardinals essential for logic (well-foundedness in Chapter 7, model theory in Chapter 9); and (c) the conceptual tools to understand the limits of provability (Cohen's forcing, Chapter 10).

---

## Key Definitions

**Set.** In ZFC, there is only one kind of object: sets. Everything is a set, including numbers, ordered pairs, functions, and relations. This might seem limiting, but it provides a uniform foundation.

**Membership.** The only primitive relation is $\in$: "$x \in y$" means "x is an element of y."

**Extensionality.** Two sets are equal iff they have the same elements: $\forall x \forall y (\forall z(z \in x \leftrightarrow z \in y) \to x = y)$.

**Empty set.** The axiom of empty set asserts $\exists x \forall y (y \notin x)$. By extensionality, the empty set is unique; we write $\emptyset$.

**Subset.** $x \subseteq y$ iff $\forall z(z \in x \to z \in y)$.

**Power set.** The power set of $x$, written $\mathcal{P}(x)$, is the set of all subsets of $x$. The Axiom of Power Set asserts its existence.

**Ordinal.** A set $\alpha$ is an ordinal if it is transitive (every element of $\alpha$ is also a subset of $\alpha$) and well-ordered by $\in$. Ordinals represent "well-order types."

**Cardinal.** A cardinal is an ordinal that is not in bijection with any smaller ordinal. Cardinals represent "sizes" of sets.

**Well-ordering.** A set $X$ with a relation $<$ is well-ordered if every non-empty subset of $X$ has a least element.

---

## The ZFC Axioms

1. **Extensionality:** Equal sets have the same elements.
2. **Empty Set:** The empty set exists.
3. **Pairing:** For any $a, b$, the set $\{a, b\}$ exists.
4. **Union:** For any set $A$, the union $\bigcup A$ exists.
5. **Power Set:** For any set $A$, $\mathcal{P}(A)$ exists.
6. **Separation (Subset):** For any set $A$ and formula $\phi(x)$, the set $\{x \in A : \phi(x)\}$ exists.
7. **Replacement:** For any set $A$ and functional formula $\phi(x, y)$, the image $\{y : \exists x \in A, \phi(x, y)\}$ is a set.
8. **Infinity:** An infinite set exists (containing $\emptyset$ and closed under $x \mapsto x \cup \{x\}$).
9. **Foundation (Regularity):** Every non-empty set has an $\in$-minimal element. (Equivalently: no set is a member of itself.)
10. **Axiom of Choice (AC):** For every collection of non-empty sets, there is a choice function selecting one element from each.

These ten axioms (or their equivalents) suffice to derive all of standard mathematics.

---

## Main Theorems

### Cantor's Theorem

**Theorem (Cantor 1891).** For any set $A$, $|A| < |\mathcal{P}(A)|$. In particular, $\mathbb{N}$ and $\mathcal{P}(\mathbb{N})$ have different cardinalities.

**Proof.** We show there is an injection $A \hookrightarrow \mathcal{P}(A)$ (namely $x \mapsto \{x\}$) but no surjection. Suppose $f: A \to \mathcal{P}(A)$; define $D = \{x \in A : x \notin f(x)\}$. Then $D \in \mathcal{P}(A)$. For any $y \in A$: if $y \in D$ then $y \notin f(y)$; if $y \notin D$ then $y \in f(y)$. Either way, $f(y) \neq D$. So $f$ is not surjective. $\square$

**Corollary.** There is no largest cardinal: $\aleph_0 < \aleph_1 < \aleph_2 < \cdots$, and $|\mathbb{R}| = |\mathcal{P}(\mathbb{N})|$.

### Well-Ordering Theorem

**Theorem (equivalent to AC).** Every set can be well-ordered.

**Proof.** Using AC, construct a well-ordering by transfinite induction: at each stage, use AC to pick the next element. $\square$

The Well-Ordering Theorem, Zorn's Lemma, and the Axiom of Choice are all equivalent (over ZF).

### Ordinal Arithmetic

Ordinals support addition, multiplication, and exponentiation, but these operations are not commutative. For example: $1 + \omega = \omega$ but $\omega + 1 > \omega$ (where $\omega$ is the first infinite ordinal — the ordinal of the natural numbers).

**Transfinite recursion.** For any starting ordinal $\alpha_0$ and two functions $F$ (successor step) and $G$ (limit step), there is a unique function $f$ on ordinals such that:
- $f(0) = \alpha_0$
- $f(\alpha + 1) = F(f(\alpha))$
- $f(\lambda) = G(\{f(\beta) : \beta < \lambda\})$ for limit ordinals $\lambda$

This is the generalisation of recursion to all ordinals.

---

## Russell's Paradox and Why It Matters

The naive "set of all sets satisfying some property" axiom — called the comprehension axiom — leads directly to contradiction:

**Russell's Paradox (1901).** Let $R = \{x : x \notin x\}$ (the set of all sets not members of themselves). Then $R \in R \iff R \notin R$.

This destroyed Frege's *Grundgesetze* and forced a complete redesign of set theory foundations. The ZFC response: replace unrestricted comprehension with *restricted* separation ($\{x \in A : \phi(x)\}$, which requires an ambient set $A$).

---

## The Continuum Hypothesis

**CH:** $|\mathcal{P}(\mathbb{N})| = \aleph_1$ (the cardinality of the reals equals the first uncountable cardinal).

Cantor (1878) asked whether CH is true. **Gödel (1938)** proved that CH is consistent with ZFC (the constructible universe $L$ satisfies CH). **Cohen (1963)** proved that $\neg$CH is consistent with ZFC (using forcing). Together: CH is *independent* of ZFC — neither provable nor refutable.

This was the first example of a "natural" mathematical statement shown independent of ZFC.

---

## Historical Context

**Georg Cantor (1845–1918)** invented set theory in the 1870s–1890s while studying trigonometric series. He discovered the uncountability of the reals (1874), defined transfinite numbers, and established cardinal and ordinal arithmetic. His work was controversial (opposed by Kronecker: "God made the integers, all else is the work of man") but ultimately triumphed.

**Gottlob Frege** attempted to derive arithmetic from logic using unrestricted comprehension, which Russell's paradox destroyed in 1902 (Frege received Russell's letter the day his second volume went to press — one of intellectual history's great tragedies).

**Ernst Zermelo (1908)** proposed the first axiomatisation of set theory, motivated by making Cantor's work rigorous and avoiding paradoxes. His axioms were later extended by Fraenkel and Skolem to give ZFC.

**Paul Cohen (1934–2007)** invented the *forcing* technique in 1963 to prove the independence of CH. Forcing is now a central tool in set theory for constructing models with prescribed properties.

---

## Connections to Other Chapters

- **Chapter 7** uses well-founded induction (a key concept from ordinals) for general recursion theorems.
- **Chapter 9** (Model Theory) studies models of first-order theories — which are themselves sets (structures) in the set-theoretic sense.
- **Chapter 10** mentions that set theory is the context in which the incompleteness theorems are formulated and proved.
- **Chapter 11** (Type Theory) provides an alternative to set theory as a foundation that avoids Russell's paradox at the type level rather than the set level.
