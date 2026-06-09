# Chapter 2 — Exercises

## Important Figures

- **Georg Cantor (1845–1918)** — created set theory; defined sets, cardinality, and the bijection criterion for equal size; first to study transfinite sets rigorously
- **Richard Dedekind (1831–1916)** — independently developed set-theoretic foundations; characterized infinite sets by the existence of a bijection with a proper subset; defined real numbers via cuts
- **Ernst Zermelo (1871–1953)** — proposed the first axiomatic system for set theory (1908), eliminating the paradoxes
- **Abraham Fraenkel (1891–1965)** — completed and strengthened Zermelo's axioms into the ZF system; added the Axiom of Replacement
- **Felix Hausdorff (1868–1942)** — extended set theory; *Grundzüge der Mengenlehre* (1914): foundational treatment of ordered sets and cardinality

## References and Primary Sources

- **G. Cantor, *Beiträge zur Begründung der transfiniten Mengenlehre* (1895–1897)** — Cantor's definitive exposition
- **E. Zermelo, "Untersuchungen über die Grundlagen der Mengenlehre I" (1908)** — original ZF axioms
- **P. Halmos, *Naive Set Theory* (Van Nostrand, 1960)** — elegant, concise introduction; readable in a weekend
- **K. Hrbáček & T. Jech, *Introduction to Set Theory* (3rd ed., Marcel Dekker, 1999)** — more thorough foundational treatment

## Examples, Applications, and Thought Experiments

- **Equivalence classes as partitions** — "same birthday" partitions a class into equivalence classes; the quotient set is the set of birth dates that appear; every equivalence relation on $A$ produces a partition of $A$ and vice versa
- **The function $f(x) = x^2$ on $\mathbb{R}$** — not injective ($f(-2) = f(2)$) and not surjective (negative reals have no preimage); restricting the domain to $[0,\infty)$ and codomain to $[0,\infty)$ makes it a bijection; adjusting domain and codomain changes the function's properties
- **Composition of bijections is a bijection** — think of bijections as "renamings": if you relabel apples as oranges and oranges as bananas, the net effect is a valid relabeling from apples to bananas; this proves $g \circ f$ is bijective when $f$ and $g$ are
- **The graph of a function** — a subset $\Gamma_f \subseteq A \times B$ is the graph of a function $f: A \to B$ iff every $a \in A$ appears as the first coordinate of exactly one pair; partial functions relax the "every $a$" condition; this makes the set-theoretic definition of function precise

## Exercises

1. Prove the following set identities directly from the definitions (without using Venn diagrams as the proof itself). For each identity, also state and prove its dual, obtained by swapping $\cup \leftrightarrow \cap$ and $\emptyset \leftrightarrow A$ throughout.
   (a) $A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$
   (b) $A \setminus (B \cap C) = (A \setminus B) \cup (A \setminus C)$

2. Define a relation $\sim$ on $\mathbb{Z}$ by $a \sim b$ if and only if $3 \mid (a^2 - b^2)$. Prove that $\sim$ is an equivalence relation. Determine the equivalence classes of $\sim$ explicitly — how many are there, and what are they? Identify which standard equivalence relation on $\mathbb{Z}$ this is.

3. Let $A$ be a non-empty set and let $\mathcal{P}$ be a partition of $A$ (a collection of non-empty, pairwise disjoint subsets of $A$ whose union is $A$). Define a relation $\sim_{\mathcal{P}}$ on $A$ by $a \sim_{\mathcal{P}} b$ if and only if $a$ and $b$ belong to the same part of $\mathcal{P}$. Prove that $\sim_{\mathcal{P}}$ is an equivalence relation, and that the equivalence classes of $\sim_{\mathcal{P}}$ are precisely the parts of $\mathcal{P}$. This establishes one direction of the correspondence between equivalence relations and partitions; state carefully what additional argument would be needed to make the correspondence a bijection.

4. Let $f: A \to B$ and $g: B \to C$ be functions. Prove each of the following:
   (a) If $g \circ f$ is injective, then $f$ is injective.
   (b) If $g \circ f$ is surjective, then $g$ is surjective.
   (c) Provide explicit counterexamples showing that the converses of (a) and (b) can both fail: it is possible for $f$ to be injective without $g \circ f$ being injective, and for $g$ to be surjective without $g \circ f$ being surjective.

5. Prove that a function $f: A \to B$ is bijective if and only if there exists a function $g: B \to A$ such that $g \circ f = \mathrm{id}_A$ and $f \circ g = \mathrm{id}_B$. Prove further that if such a $g$ exists, it is unique, and justify why we are entitled to call it $f^{-1}$.

6. Let $\sim$ be an equivalence relation on $A$, and define the *quotient map* $\pi: A \to A/{\sim}$ by $\pi(a) = [a]$. Prove that $\pi$ is surjective. Now suppose $f: A \to B$ is a function that is *constant on equivalence classes*, meaning $a \sim b \Rightarrow f(a) = f(b)$. Prove that there exists a unique function $\bar{f}: A/{\sim} \to B$ such that $f = \bar{f} \circ \pi$. This is the *universal property of the quotient set*; state in words what it says about functions defined on equivalence classes.

7. Define a relation $\leq$ on the power set $\mathcal{P}(X)$ of a set $X$ by $A \leq B$ iff $A \subseteq B$. Prove that $\leq$ is a partial order on $\mathcal{P}(X)$. Identify which pairs of elements in $\mathcal{P}(X)$ are comparable under this order, and for $X = \{1,2,3\}$, draw the Hasse diagram of $(\mathcal{P}(X), \subseteq)$.

8. (Challenge) Let $f: A \to B$ be a function. Define a relation $\sim_f$ on $A$ by $a \sim_f a'$ iff $f(a) = f(a')$. Prove that $\sim_f$ is an equivalence relation, and that the induced map $\bar{f}: A/{\sim_f} \to B$ defined by $\bar{f}([a]) = f(a)$ is injective. Conclude that every function $f: A \to B$ factors as a surjection followed by an injection: $A \xrightarrow{\pi} A/{\sim_f} \xrightarrow{\bar{f}} B$, and explain why this factorization is unique up to the choice of intermediate set.
