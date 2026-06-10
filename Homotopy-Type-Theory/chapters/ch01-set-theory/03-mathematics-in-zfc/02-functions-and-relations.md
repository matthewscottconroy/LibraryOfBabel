# 3.2 Functions, Relations, and Ordered Pairs

## The Problem of Ordered Pairs

In mathematics, an ordered pair $(a, b)$ should satisfy: $(a, b) = (c, d) \iff a = c \wedge b = d$. Order matters; $(1, 2) \neq (2, 1)$.

But in set theory, sets have no order: $\{a, b\} = \{b, a\}$. How do we encode ordered pairs as sets?

The standard solution is the **Kuratowski encoding**:
$$\langle a, b \rangle := \{\{a\}, \{a, b\}\}$$

**Theorem (Kuratowski).** $\langle a, b \rangle = \langle c, d \rangle \iff a = c \wedge b = d$.

*Proof.* ($\Leftarrow$) Obvious substitution.

($\Rightarrow$) Suppose $\{\{a\}, \{a, b\}\} = \{\{c\}, \{c, d\}\}$.

Case 1: $a = b$. Then $\{\{a\}, \{a, a\}\} = \{\{a\}\}$ (a singleton). By the equality, $\{\{c\}\} = \{\{c, d\}\}$, so $\{c\} = \{c, d\}$, meaning $d = c$. And $\{a\} = \{c\}$, so $a = c$. Thus $a = b = c = d$.

Case 2: $a \neq b$. The set $\{\{a\}, \{a,b\}\}$ has two elements ($\{a\}$ and $\{a,b\}$ are distinct since $a \neq b$). By the equality, $\{\{c\}, \{c,d\}\}$ also has two elements, so $c \neq d$. The singleton $\{a\}$ must equal either $\{c\}$ or $\{c,d\}$. If $\{a\} = \{c,d\}$, then $a = c = d$, contradicting $c \neq d$. So $\{a\} = \{c\}$, giving $a = c$. Then $\{a,b\} = \{c,d\} = \{a,d\}$, so $b = d$. $\square$

The Kuratowski encoding is somewhat arbitrary — there are other encodings that work. The point is not what the encoding is but that it *works*: it satisfies the desired property.

## Cartesian Products

**Definition.** The *Cartesian product* $A \times B$ is:
$$A \times B = \{\langle a, b \rangle \mid a \in A \wedge b \in B\}$$

This exists by a combination of Power Set, Separation, and the Kuratowski encoding (the elements $\langle a, b \rangle = \{\{a\}, \{a,b\}\}$ are all elements of $\mathcal{P}(\mathcal{P}(A \cup B))$, which exists by Union and Power Set).

## Relations

**Definition.** A *binary relation* between sets $A$ and $B$ is any subset $R \subseteq A \times B$. We write $aRb$ or $R(a, b)$ to mean $\langle a, b \rangle \in R$.

**Special relations on a set $A$:**
- *Reflexive:* $\forall a, aRa$
- *Symmetric:* $aRb \to bRa$
- *Transitive:* $aRb \wedge bRc \to aRc$
- *Antisymmetric:* $aRb \wedge bRa \to a = b$

**Equivalence relation:** reflexive, symmetric, and transitive. Examples: $=$ (equality), congruence mod $n$, homotopy of paths.

**Partial order:** reflexive, antisymmetric, transitive. Examples: $\leq$ on $\mathbb{N}$, $\subseteq$ on sets.

**Total order (linear order):** a partial order where additionally $\forall a, b: aRb \vee bRa$. Examples: $\leq$ on $\mathbb{Z}$.

**Well-order:** a total order where every non-empty subset has a least element. The natural numbers with $\leq$ are well-ordered.

## Functions as Sets

**Definition.** A *function* $f : A \to B$ is a relation $f \subseteq A \times B$ satisfying:
- *Total:* $\forall a \in A, \exists b \in B, \langle a, b \rangle \in f$
- *Functional (single-valued):* $\langle a, b \rangle \in f \wedge \langle a, b' \rangle \in f \to b = b'$

In other words: $f$ is a set of ordered pairs where every element of $A$ appears as a first component exactly once.

The *value* $f(a)$ is the unique $b$ with $\langle a, b \rangle \in f$.

**The function set $B^A$:** All functions $A \to B$ form a set $B^A = \{f \in \mathcal{P}(A \times B) \mid f \text{ is a function}\}$.

**Composition:** If $f : A \to B$ and $g : B \to C$, then $g \circ f : A \to C$ is defined by $(g \circ f)(a) = g(f(a))$. As a set of pairs: $\{\langle a, c \rangle \mid \exists b, \langle a, b \rangle \in f \wedge \langle b, c \rangle \in g\}$.

**Properties of functions:**
- *Injective:* $f(a) = f(a') \to a = a'$
- *Surjective:* $\forall b \in B, \exists a \in A, f(a) = b$
- *Bijective:* both injective and surjective

## Equivalence Classes and Quotients

Given an equivalence relation $\sim$ on $A$, the *equivalence class* of $a$ is:
$$[a] = \{b \in A \mid a \sim b\}$$

The *quotient* $A/{\sim}$ is the set of equivalence classes:
$$A/{\sim} = \{[a] \mid a \in A\} \subseteq \mathcal{P}(A)$$

This exists by the Axiom of Separation (applied to the power set of $A$).

**The quotient is a partition:** The equivalence classes are pairwise disjoint and their union is $A$.

**Examples:**
- $\mathbb{Z} = (\mathbb{N} \times \mathbb{N})/{\sim}$ where $(m, n) \sim (m', n')$ iff $m + n' = m' + n$ (encoding $m - n$)
- $\mathbb{Q} = (\mathbb{Z} \times (\mathbb{Z} \setminus \{0\}))/{\sim}$ where $(p, q) \sim (p', q')$ iff $pq' = p'q$ (encoding $p/q$)
- $\mathbb{Z}/n\mathbb{Z} = \mathbb{Z}/{\sim}$ where $a \sim b$ iff $n \mid (a - b)$
- Fundamental group: homotopy classes of loops

## Set Theory vs. Type Theory: The Identity Issue

Here's the critical issue with the set-theoretic approach to identity.

Consider two constructions of the rational number $1/2$:
1. As a Dedekind cut: $\{q \in \mathbb{Q} \mid q < 1/2\}$ — wait, this is circular. Let's be more careful: $\{q \in \mathbb{Q}_{\text{arith}} \mid 2q < 1\}$ where $\mathbb{Q}_{\text{arith}}$ is the set constructed arithmetically.
2. As an equivalence class of pairs: $[(1, 2)] = \{(p, q) \in \mathbb{Z} \times (\mathbb{Z} \setminus \{0\}) \mid 1q = 2p\} = \{(1,2), (2,4), (-1,-2), \ldots\}$

In set theory: these are different sets (literally different collections of elements). Yet we treat them as "the same rational number." We achieve this by working within whichever construction we're using and never comparing across constructions.

This is epistemically unsatisfying. Mathematicians routinely say "the rational number $1/2$" as if there is one canonical object, but in ZFC, the answer depends on which construction you chose.

**Type theory's answer:** In type theory, the rational numbers $\mathbb{Q}$ are defined by their *universal property*: a type with operations satisfying certain axioms, unique up to (unique) isomorphism. The "elements of $\mathbb{Q}$" don't need to be sets — they are terms of the type. Two different constructions give equivalent types, and the Univalence Axiom says equivalent types are equal.

This resolves the identity issue by design: there is one $\mathbb{Q}$ (up to equality), not "the rational numbers as Dedekind cuts" vs. "the rational numbers as equivalence classes of pairs."
