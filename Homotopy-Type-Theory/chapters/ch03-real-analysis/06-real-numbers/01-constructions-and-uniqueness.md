# 6.1 The Real Numbers: Constructions and Uniqueness

## What Are the Real Numbers?

In calculus, the real numbers $\mathbb{R}$ are presented as an axiom system: there's a complete ordered field, and we call it $\mathbb{R}$. In a foundations course, we need to do more: we need to *construct* $\mathbb{R}$ from previously understood objects (like $\mathbb{Q}$), and prove it has the properties we want.

There are two main constructions:
1. **Dedekind cuts**: a real number is a "cut" that divides $\mathbb{Q}$ into two pieces.
2. **Cauchy sequences**: a real number is an equivalence class of Cauchy sequences of rationals.

Both constructions are valid. Both produce a complete ordered field. And in a precise sense, they produce *the same thing* — any two complete ordered fields are isomorphic. This uniqueness is one of the most important theorems in foundations.

## Construction 1: Dedekind Cuts

**Definition.** A *Dedekind cut* is a subset $\alpha \subseteq \mathbb{Q}$ satisfying:
1. $\alpha \neq \emptyset$ and $\alpha \neq \mathbb{Q}$.
2. If $p \in \alpha$ and $q < p$ (in $\mathbb{Q}$), then $q \in \alpha$.
3. $\alpha$ has no greatest element: for all $p \in \alpha$, there exists $q \in \alpha$ with $q > p$.

Informally: $\alpha$ is the set of all rationals "strictly to the left of" a real number $r$. The three conditions say: $\alpha$ is non-trivial, downward-closed, and open on the right.

**The real numbers.** Define $\mathbb{R}$ as the set of all Dedekind cuts.

**Examples:**
- The cut corresponding to $\sqrt{2}$: $\alpha_{\sqrt{2}} = \{q \in \mathbb{Q} \mid q < 0 \text{ or } q^2 < 2\}$.
- The cut corresponding to $0$: $\alpha_0 = \{q \in \mathbb{Q} \mid q < 0\}$.
- Each rational $r$ corresponds to the cut $\alpha_r = \{q \in \mathbb{Q} \mid q < r\}$.

**Ordering:** $\alpha \leq \beta$ iff $\alpha \subseteq \beta$ as sets. This gives a total order on Dedekind cuts.

**Arithmetic:** Addition of cuts is $\alpha + \beta = \{p + q \mid p \in \alpha, q \in \beta\}$. Multiplication is more complex (handling signs carefully).

**Completeness:** The Dedekind cut construction directly embodies completeness. If $S$ is a non-empty set of Dedekind cuts bounded above, then $\bigcup_{\alpha \in S} \alpha$ is a Dedekind cut, and it's the least upper bound of $S$. Completeness falls out immediately.

## Construction 2: Cauchy Sequences

**Definition.** Consider Cauchy sequences of rationals — sequences $(q_n)$ in $\mathbb{Q}$ where for all $\varepsilon > 0$, eventually $|q_m - q_n| < \varepsilon$.

Define an equivalence relation: $(q_n) \sim (r_n)$ iff $|q_n - r_n| \to 0$.

**The real numbers.** Define $\mathbb{R}$ as the set of equivalence classes $[(q_n)]$.

**Arithmetic.** Component-wise: $[(q_n)] + [(r_n)] = [(q_n + r_n)]$, $[(q_n)] \cdot [(r_n)] = [(q_n \cdot r_n)]$. One must check these are well-defined (independent of the representative) and that the results are Cauchy.

**Ordering.** $[(q_n)] < [(r_n)]$ iff there exists $\varepsilon > 0$ and $N$ with $r_n - q_n > \varepsilon$ for $n \geq N$.

**Completeness.** Every Cauchy sequence of equivalence classes converges. This requires a careful diagonal argument: given a Cauchy sequence of Cauchy sequences, extract a single Cauchy sequence whose equivalence class is the limit.

**Embedding $\mathbb{Q}$.** The rational $q$ embeds as the constant sequence $[(q, q, q, \ldots)]$.

## The Categorical Connection: Universal Property

Both constructions are valid, but which one is "the" real numbers? Neither — the real numbers are characterized by a *universal property*.

**Theorem.** $\mathbb{R}$ is (up to unique order-preserving field isomorphism) the unique complete ordered field.

*Proof sketch.* Suppose $F$ and $F'$ are both complete ordered fields. We need to construct an isomorphism $\phi : F \to F'$.

Both $F$ and $F'$ contain copies of $\mathbb{Q}$ (as the smallest subfield). The isomorphism must send the $\mathbb{Q}$ in $F$ to the $\mathbb{Q}$ in $F'$ (uniquely, since the identity is the only field automorphism of $\mathbb{Q}$).

For any $x \in F$, the set $\{q \in \mathbb{Q}_F \mid q < x\}$ (where $\mathbb{Q}_F$ is the copy of $\mathbb{Q}$ in $F$) is a Dedekind cut in $F$. The least upper bound in $F'$ of the corresponding set in $\mathbb{Q}_{F'}$ defines $\phi(x)$.

One verifies $\phi$ is a field isomorphism. Uniqueness follows because the ordering forces $\phi(x)$ to be determined by the Dedekind cut of $x$. $\square$

This theorem has a crucial implication: *the real numbers are unique up to isomorphism*. There is, essentially, one complete ordered field.

## Why This Matters Philosophically

The uniqueness theorem says: it doesn't matter which construction you use. Dedekind cuts and Cauchy sequences give isomorphic fields, and there's a unique isomorphism between them.

But in ZFC, they are different *sets*. The set of Dedekind cuts is a set of sets of rationals. The set of equivalence classes of Cauchy sequences is a set of sets of sequences of rationals. These are genuinely different sets in ZFC — they have different elements.

This is the identity problem from Chapter 1 in a concrete form. Mathematicians say "the real numbers" as if there's one $\mathbb{R}$. ZFC gives two (or more), related by isomorphism. We work around this by "choosing a construction" or by "working up to isomorphism" — but neither resolution is built into the logic.

**The type-theoretic resolution.** In Homotopy Type Theory, the real numbers can be defined by their universal property: $\mathbb{R}$ is defined as *the* complete ordered field, which exists and is unique (by the uniqueness theorem). Two different constructions give *equal* types (by Univalence), not just isomorphic ones.

This is what Univalence buys us: the mathematician's informal practice of "treating isomorphic things as the same" becomes literally true.

## The Archimedean Property

**Definition.** An ordered field $F$ is *Archimedean* if for every $x \in F$, there exists $n \in \mathbb{N}$ with $n > x$ (where $n$ is the $n$-fold sum $1 + 1 + \cdots + 1$).

In other words: the natural numbers are unbounded in $F$.

**Theorem.** Every complete ordered field is Archimedean.

*Proof.* Suppose $x > n$ for all $n \in \mathbb{N}$. Then $\mathbb{N}$ is bounded above by $x$. By completeness, $s = \sup \mathbb{N}$ exists. But $s - 1 < s$, so $s - 1$ is not an upper bound, hence $n > s - 1$ for some $n \in \mathbb{N}$. Then $n + 1 \in \mathbb{N}$ and $n + 1 > s$ — contradicting that $s$ is an upper bound. $\square$

The Archimedean property is what makes calculus work: it ensures $1/n \to 0$, that $\varepsilon$-$\delta$ proofs can always find an appropriate $n$, and that real numbers can be approximated by rationals.

## Density of $\mathbb{Q}$ in $\mathbb{R}$

**Theorem.** Between any two real numbers, there is a rational.

*Proof.* If $a < b$ are real, then $b - a > 0$. By Archimedean property, $n > 1/(b-a)$, so $n(b-a) > 1$. Among $\lfloor na \rfloor, \lfloor na \rfloor + 1$, one lies between $na$ and $nb$, giving a rational $m/n \in (a, b)$. $\square$

**Corollary.** $\mathbb{Q}$ is dense in $\mathbb{R}$: every non-empty open interval $(a, b)$ contains a rational.

This means $\mathbb{R}$ is "well approximated" by $\mathbb{Q}$. Every real number is a limit of rationals. In the Cauchy sequence construction, this is by design; in the Dedekind cut construction, it requires the Archimedean property.

## The Continuum

The real numbers form the *continuum* — an uncountable set that is connected, complete, and Archimedean. The intuition "there are no gaps" is made precise by completeness.

**Cantor's theorem (revisited).** $|\mathbb{R}| > |\mathbb{N}|$: the reals are uncountable.

*Proof.* By Cantor's diagonal argument: any listing $r_1, r_2, r_3, \ldots$ of real numbers in $[0, 1]$ misses some number. Construct $r$ whose $n$-th decimal digit differs from the $n$-th digit of $r_n$. Then $r \neq r_n$ for all $n$. $\square$

The *Continuum Hypothesis (CH)* asks: is $|\mathbb{R}| = \aleph_1$? This is independent of ZFC (Gödel + Cohen). In ZFC, we know $|\mathbb{R}| = 2^{\aleph_0}$ (the cardinality of the power set of $\mathbb{N}$), but whether $2^{\aleph_0} = \aleph_1$ is undecidable.

In HoTT, the Continuum Hypothesis takes a different form — it's not about cardinalities of sets but about the homotopy type of $\mathbb{R}$, and its status is an active area of research.

## Real Analysis as Preparation for Topology

The real numbers are the canonical example of a complete metric space, a complete ordered field, and a connected, locally compact, separable topological space. Every definition in this chapter was motivated by properties of $\mathbb{R}$.

The natural next step is *topology*: the study of what happens when you keep the open sets but throw away the metric. In a topological space, you have a notion of open set and continuity but no distance function. Compactness, connectedness, and path-connectedness generalize cleanly. Completeness does not (it's a metric notion, not a topological one).

The transition from metric spaces to topological spaces, and from there to the abstract notion of homotopy, is the subject of the following chapters. The real line $[0, 1]$ remains at the center of the theory — it's the interval of parameters for paths, homotopies, and higher homotopies throughout homotopy theory and HoTT.
