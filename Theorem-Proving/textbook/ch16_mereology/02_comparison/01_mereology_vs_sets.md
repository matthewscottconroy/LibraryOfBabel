# Mereology vs. Set Theory

Set theory and mereology are both theories of "many collected into one" — sets with their members, fusions with their parts. The resemblance is superficial. The two primitives obey different logical laws, license different existence claims, and, most strikingly, produce theories on opposite sides of the decidability line. This section develops the comparison formally; Chapter 6 supplies the set-theoretic background.

## Membership vs. Parthood

The membership relation $\in$ of ZF is **neither reflexive nor transitive**. Not reflexive: Foundation implies $\forall x\, x \notin x$. Not transitive: $a \in \{a\}$ and $\{a\} \in \{\{a\}\}$, but $a \notin \{\{a\}\}$ — the sole member of $\{\{a\}\}$ is $\{a\}$. Parthood $P$, by contrast, is reflexive, antisymmetric, and transitive by the very axioms of $\mathbf{M}$. (The set-theoretic relation that *is* a partial order is inclusion $\subseteq$ — and indeed $\subseteq$ on nonempty sets is a useful mental model for $P$.)

| | ZF set theory | GEM mereology |
|---|---|---|
| Primitive | $\in$ | $P$ |
| Reflexive / transitive | no / no | yes / yes |
| Empty object | $\varnothing$ exists, unique | no null individual |
| Singleton | $\{a\} \neq a$ | $\sigma x\,(x=a) = a$ |
| Hierarchy | cumulative ranks $V_\alpha$ | flat: fusion of fusions is a fusion |
| Extensionality | all sets | composite objects only |
| First-order theory | undecidable, incomplete | decidable (Tarski) |

The "flatness" row is worth stating precisely: fusion is idempotent in the sense that $\sigma$ applied to fusions of $\phi$-things yields the same object as $\sigma$ applied to the $\phi$-things themselves, and the fusion of a single thing is that thing. Mereology has no analogue of the rank hierarchy $V_0 \subseteq V_1 \subseteq \cdots$ — there is nowhere "up" to go.

## No Empty Object

ZF cannot live without $\varnothing$: it anchors the ordinals and the cumulative hierarchy. The mereological analogue would be a **null individual** $n$ with $\forall x\, P(n,x)$ — a part of everything. Classical mereology does not merely omit it; even weak supplementation *refutes* it in any nontrivial domain.

**Proposition.** In $\mathbf{M} + \mathrm{WSP}$, if some $n$ satisfies $\forall x\, P(n,x)$, then the domain contains exactly one object.

*Proof.* Suppose $\forall x\, P(n,x)$ and, for contradiction, some $y \neq n$. Then $P(n,y)$ and $n \neq y$, so $PP(n,y)$. By WSP there is $z$ with $P(z,y) \land \neg O(z,n)$. But $P(n,z)$ (as $n$ is part of everything) and $P(n,n)$, so $n$ is a common part of $z$ and $n$: $O(z,n)$ — contradiction. $\square$

Intuitively, a part of every material thing at once would have to be nothing at all; formally, a bottom element makes disjointness unsatisfiable and supplementation absurd. Systems that add a null element for algebraic convenience recover exactly Boolean algebras *with* zero — Tarski's theorem read in reverse.

## Singletons

In ZF, $x \neq \{x\}$: Foundation forbids $x \in x$, which $x = \{x\}$ would entail (only non-well-founded set theories admit Quine atoms $x = \{x\}$). So set formation always adds a new layer of structure — $a$, $\{a\}$, $\{\{a\}\}$, ... are all distinct. Mereology has no such layer: the fusion of the condition "being identical to $a$" is just $a$, and more generally $a$ is a mereological part of any fusion containing it — $\phi(a) \to P(a, \sigma x\,\phi(x))$ is a theorem of GEM. What set theory renders as membership in a class, mereology renders as parthood in a fusion; and since parts of parts are parts, the fusion cannot remember which things it was "made from." This is why Lewis called mereological composition **ontologically innocent** — a fusion is no addition of being beyond its parts — and also, as we will see below, why mereology is mathematically weak.

## Extensionality, Twice

$$
\begin{aligned}
&\textbf{ZF:} && \forall x \forall y\,\bigl(\forall z\,(z \in x \leftrightarrow z \in y) \to x = y\bigr)\\
&\textbf{EM:} && \forall x \forall y\,\Bigl(\exists z\, PP(z,x) \to \bigl(\forall z\,(PP(z,x) \leftrightarrow PP(z,y)) \to x = y\bigr)\Bigr)
\end{aligned}
$$

Set extensionality is unconditional; it entails that all memberless objects collapse into one, which is precisely why $\varnothing$ is unique (and why *urelements* require modifying the axiom). Mereological extensionality must carry the proviso: all atoms alike have no proper parts, yet distinct atoms abound. The asymmetry runs deep — set theory individuates from below without exception; mereology individuates composites from below but takes atoms as brute.

## Lewis: *Parts of Classes*

David Lewis (1991) showed how much of set theory is *already* mereology. His thesis: the parts of a class are exactly its nonempty subclasses. The class of cats literally has the class of black cats as a part; a class is the fusion of the singletons of its members. What remains irreducibly set-theoretic is only the **singleton function** $x \mapsto \{x\}$: membership factors as
$$x \in y \;\equiv\; P(\{x\},\, y) \quad \text{for classes } y,$$
so *set theory = mereology + singleton*. All the mystery of the universe of sets concentrates in the single step from $x$ to $\{x\}$ — a step Lewis found "unintelligible" and ultimately treated structurally: in *Mathematics is Megethology* (1993), mereology plus plural quantification (Chapter 15) simulates the singleton function up to isomorphism, given hypotheses about how many atoms there are. Mathematics reduces to **megethology** — the theory of size.

## Composition as Identity

If a fusion is "nothing over and above" its parts, is it *identical* to them? Donald Baxter defends the strong thesis: the whole just is the parts, counted differently. Lewis stopped short at "almost identity": composition is *analogous* to identity (innocent, unmysterious) but not identity itself. The strong thesis strains logic: identity is one–one, while composition relates one whole to many parts, so even stating $u = xx$ requires plural terms (Chapter 15), and Leibniz's law then misfires — the parts are many, the fusion is not. The debate matters here because strong composition-as-identity would make unrestricted fusion trivially true: the fusion's existence is just the parts' existence, redescribed.

## The Decidability Asymmetry

Now the sharpest formal contrast.

**Theorem.** The first-order theory of atomistic GEM (GEM + Atomicity) is decidable.

*Proof idea.* By Tarski's representation theorem, models of atomistic GEM are, up to isomorphism, atomic Boolean algebras with the zero removed; the translation $P \mapsto \leq$, relativized to nonzero elements, interprets the mereological theory in the elementary theory of atomic Boolean algebras. That theory is decidable — Tarski announced the decidability of the full elementary theory of Boolean algebras in 1949, refining Skolem's 1919 quantifier elimination for the atomic case. A decision procedure for the host theory plus a computable interpretation yields one for the guest. $\square$

Atomless GEM is decidable for an even cleaner reason: countable atomless Boolean algebras are unique up to isomorphism (a back-and-forth argument), so the theory is $\omega$-categorical, hence complete, hence decidable. Even full GEM, neutral about atoms, is decidable (Tsai 2013) — while, curiously, the *weak* systems $\mathbf{M}$, MM, and EM are undecidable: $\mathbf{M}$ is just the theory of partial orders, which interprets enough graph theory to be undecidable. Strengthening the axioms *tames* the models until decision becomes possible.

Set theory sits on the other side of the line. ZF interprets Robinson arithmetic $Q$, so it is **essentially undecidable** (Tarski–Mostowski–Robinson): no consistent extension of ZF is decidable, and by Gödel–Rosser every consistent recursively axiomatized extension is incomplete. Two theories of "collections," one primitive apiece — and one is decidable while the other cannot even be completed. The reason is expressive: $\in$ builds the cumulative hierarchy, encodes pairs, sequences, and arithmetic; $P$ collapses all structure into a Boolean algebra, which Tarski-style quantifier elimination can exhaustively analyze. (Decidable is not cheap: the theory of Boolean algebras has provably exponential complexity — but it is a far cry from unsolvable.)

## What Mereology Cannot Say

The price of decidability is expressive poverty — in particular, **cardinality talk**.

Fusion forgets count: if $a, b$ are disjoint, the two things $a, b$ and the three things $a, b, a+b$ have the very same fusion, so "the number of things $z$ was fused from" is not well defined. In an atomistic setting one can count atoms — "$z$ has exactly $n$ atomic parts" is first-order expressible for each fixed $n$, and a finite model of atomistic GEM with $n$ atoms has exactly $2^n - 1$ objects. But no first-order mereological theory can say "there are *finitely many* atoms" (compactness: a theory with arbitrarily large finite atom counts has a model with infinitely many), and in gunk there are no atoms to count at all. Genuine cardinality comparisons — "there are more cats than dogs" — require either coding by atoms or **plural quantification**, which is exactly the route Lewis's megethology takes; see [Chapter 15: Plural Logic](../../ch15_plural_logic/README.md). Set theory, by contrast, was *built* for cardinality: that is Cantor's inheritance, and mereology has no share in it.

## Exercises
See [problems/ch16_mereology/](../../../problems/ch16_mereology/)
