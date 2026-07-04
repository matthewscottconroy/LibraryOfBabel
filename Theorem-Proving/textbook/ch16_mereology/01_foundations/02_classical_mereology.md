# Classical Mereology

Ground mereology $\mathbf{M}$ says that parthood is a partial order — nothing more. It cannot prove that a whole with a proper part has any *other* part, that objects are determined by their parts, or that any composite objects exist at all. Classical mereology — **General Extensional Mereology (GEM)**, essentially Leśniewski's system and Tarski's algebraic reconstruction of it — adds two ingredients: *supplementation* and *unrestricted fusion*. The resulting theory is remarkably strong and remarkably well behaved: its models are, up to isomorphism, complete Boolean algebras with the zero deleted.

## Supplementation Principles

If $x$ is a proper part of $y$, must $y$ contain something more — a *remainder*?

**Axiom (Weak Supplementation, WSP).**
$$PP(x,y) \to \exists z\,\bigl(P(z,y) \land \neg O(z,x)\bigr)$$

Whenever $x$ falls short of $y$, some part of $y$ is disjoint from $x$. WSP rules out the pathological one-proper-part model of the previous section. (The witness $z$ is automatically a *proper* part of $y$: if $z = y$ then $D(y,x)$, contradicting $O(x,y)$, which follows from $P(x,y)$.)

**Axiom (Strong Supplementation, SSP).**
$$\neg P(y,x) \to \exists z\,\bigl(P(z,y) \land \neg O(z,x)\bigr)$$

Whenever $y$ is not part of $x$ — even if the two overlap — some part of $y$ avoids $x$ entirely.

**Theorem.** In $\mathbf{M}$, SSP implies WSP.

*Proof.* Assume $PP(x,y)$. Then $P(x,y)$ and $x \neq y$, so by antisymmetry $\neg P(y,x)$. Apply SSP: some $z$ has $P(z,y) \land \neg O(z,x)$, which is WSP's consequent. $\square$

The converse fails. Consider the model $\mathfrak{N}$ with domain $\{a, b, c_1, c_2\}$, where $P$ is the reflexive closure of: $a$ and $b$ are each parts of both $c_1$ and $c_2$.

```
   c1      c2
   |  \  /  |
   |   \/   |
   |   /\   |
   a --/  \-- b
```

WSP holds: the only proper parthood pairs are $a, b$ under $c_1, c_2$, and each time the *other* atom is a part of the whole disjoint from the first. But SSP fails: $\neg P(c_1, c_2)$, yet every part of $c_1$ (namely $a$, $b$, $c_1$ itself) overlaps $c_2$. Note also that $c_1$ and $c_2$ have exactly the same proper parts yet $c_1 \neq c_2$ — extensionality fails in $\mathfrak{N}$. This is no accident:

- $\mathbf{M} + \mathrm{WSP}$ is called **Minimal Mereology (MM)**;
- $\mathbf{M} + \mathrm{SSP}$ is called **Extensional Mereology (EM)** — because of the following theorem.

## Extensionality of Parthood

**Theorem (Extensionality).** In EM, if $x$ or $y$ has a proper part, then
$$x = y \;\leftrightarrow\; \forall z\,\bigl(PP(z,x) \leftrightarrow PP(z,y)\bigr).$$

*Proof.* Left to right is trivial. For right to left, suppose $x$ and $y$ have the same proper parts and, without loss of generality, $PP(w,x)$ for some $w$; then also $PP(w,y)$.

*Claim: $P(x,y)$.* Suppose not. By SSP there is $z$ with $P(z,x) \land \neg O(z,y)$. If $z = x$: then $\neg O(x,y)$; but $P(w,x)$ and $P(w,y)$, so $w$ witnesses $O(x,y)$ — contradiction. If $z \neq x$: then $PP(z,x)$, hence $PP(z,y)$ by hypothesis, hence $P(z,y)$, and $z$ witnesses $O(z,y)$ — contradicting $\neg O(z,y)$. So $P(x,y)$.

By the symmetric argument (using $PP(w,y)$ in the first case), $P(y,x)$. By antisymmetry, $x = y$. $\square$

The proviso "has a proper part" is essential: distinct *atoms* have the same proper parts (none) vacuously. So EM says composite objects are individuated by their proper parts — the mereological analogue of set-theoretic extensionality, and the focus of the constitution debate (see [Physical Objects](../03_applications/01_physical_objects.md)).

A companion result, whose contrapositive is exactly SSP, is worth recording:

**Lemma (Overlap criterion).** In EM, $P(x,y) \leftrightarrow \forall z\,\bigl(O(z,x) \to O(z,y)\bigr)$; hence $x = y \leftrightarrow \forall z\,\bigl(O(z,x) \leftrightarrow O(z,y)\bigr)$.

*Proof sketch.* Left to right holds already in $\mathbf{M}$ by transitivity. Right to left: if $\neg P(x,y)$, SSP yields a part $z$ of $x$ with $O(z,x)$ but $\neg O(z,y)$. $\square$

So in EM an object is completely determined by its overlap profile — indeed $P$ could be *defined* from $O$ by the lemma.

## Fusion

Classical mereology's final ingredient asserts that composites exist in the greatest possible generality. First we say what a fusion is.

**Definition (Fusion).** For a formula $\phi(x)$,
$$\mathrm{Fu}(z, \phi) \;\equiv\; \forall x\,\bigl(\phi(x) \to P(x,z)\bigr) \;\land\; \forall w\,\Bigl(\forall x\,\bigl(\phi(x) \to P(x,w)\bigr) \to P(z,w)\Bigr).$$

That is: $z$ is an upper bound of the $\phi$-ers, and a part of every upper bound — their *least upper bound* under $P$. (An alternative definition used in the literature is $\mathrm{Fu}_O(z,\phi) \equiv \forall y\,(O(y,z) \leftrightarrow \exists x(\phi(x) \land O(x,y)))$: $z$ overlaps exactly what some $\phi$-er overlaps. In the presence of SSP the two characterize the same objects; in weaker theories they come apart.)

**Axiom Schema (Unrestricted Fusion).** For every formula $\phi(x)$ of $\mathcal{L}_P$ (possibly with parameters):
$$\exists x\, \phi(x) \;\to\; \exists z\, \mathrm{Fu}(z, \phi)$$

Any instantiated condition whatsoever — however scattered or gerrymandered its instances — has a fusion. As with Separation in ZF, this is an axiom *schema*, one axiom per formula; the first-order theory can only fuse *definable* collections, while the second-order version quantifies over all subsets of the domain.

**Proposition (Uniqueness).** In $\mathbf{M}$, fusions are unique: $\mathrm{Fu}(z,\phi) \land \mathrm{Fu}(z',\phi) \to z = z'$.

*Proof.* Each of $z, z'$ is an upper bound, so leastness gives $P(z,z')$ and $P(z',z)$; apply antisymmetry. $\square$

We may therefore write $\sigma x\, \phi(x)$ for *the* fusion of the $\phi$s, and define the **binary sum** $x + y := \sigma z\,(P(z,x) \lor P(z,y))$ and the **universe** $u := \sigma x\,(x = x)$, of which everything is a part. There is deliberately no dual "zero": fusion requires a *nonempty* condition, and classical mereology admits no null individual that is part of everything (Section 3 proves this from WSP alone).

## GEM: The Full System

**General Extensional Mereology (GEM)** is the first-order theory:

1. **(M1)** $\forall x\, P(x,x)$
2. **(M2)** $\forall x \forall y\,(P(x,y) \land P(y,x) \to x=y)$
3. **(M3)** $\forall x \forall y \forall z\,(P(x,y) \land P(y,z) \to P(x,z))$
4. **(SSP)** $\forall x \forall y\,(\neg P(y,x) \to \exists z\,(P(z,y) \land \neg O(z,x)))$
5. **(Fusion schema)** $\exists x\,\phi(x) \to \exists z\,\mathrm{Fu}(z,\phi)$, for each $\phi$

(WSP, extensionality, uniqueness of fusions are theorems.) GEM licenses *scattered* objects — the fusion of Napoleon's left hand and the Eiffel Tower is a perfectly good individual — and is neutral on atomicity: both GEM + Atomicity and GEM + Atomlessness are consistent. Whether such arbitrary fusions really *exist* is the Special Composition Question, taken up in the applications section.

## The Boolean Algebra Theorem

**Theorem (Tarski, 1935).** The models of GEM (with fusion for arbitrary collections) are exactly the structures $\langle B \smallsetminus \{0\}, \leq \rangle$ where $B$ is a complete Boolean algebra — a complete Boolean algebra with the bottom element removed.

*Proof idea.* Given a complete Boolean algebra $B$, delete $0$ and read $P$ as $\leq$: M1–M3 are immediate; for SSP, if $y \not\leq x$ then $y \wedge \neg x \neq 0$ is a part of $y$ sharing no nonzero lower bound with $x$; fusions are the (complete) joins $\bigvee$. Conversely, given a GEM model, adjoin a fresh zero $0$ and define: join $=$ fusion, $x \wedge y := x \sqcap y$ (the product below) when $O(x,y)$ and $0$ otherwise, and complement $\neg x := \sigma z\, D(z,x)$ for $x \neq u$ (nonempty by SSP applied to $\neg P(u,x)$), with $\neg u := 0$. The overlap criterion — each element determined by its overlap set — yields distributivity via the embedding $x \mapsto \{z : O(z,x)\}$ into a field of sets, and unrestricted fusion supplies completeness. $\square$

The excised bottom is precisely the absent null individual. In the first-order setting the correspondence is with Boolean algebras whose *definable* nonempty subsets have suprema — a point that matters for the decidability results of the [comparison section](../02_comparison/01_mereology_vs_sets.md).

## Products from Overlap

The meet operation promised above is the *product* — and its existence is a theorem, not an axiom.

**Theorem (Products).** In GEM, if $O(x,y)$ then there exists a greatest common part $x \sqcap y$ of $x$ and $y$.

*Proof sketch.* Let $\phi(z) \equiv P(z,x) \land P(z,y)$. Since $O(x,y)$, $\phi$ is instantiated, so the fusion $f = \sigma z\,\phi(z)$ exists; it is an upper bound of all common parts and least among such bounds. The delicate point is that $f$ is itself a common part: supposing $\neg P(f,x)$, SSP yields $k$ with $P(k,f) \land \neg O(k,x)$; but every part of a fusion of common parts overlaps some common part, and a part of a common part of $x$ is a part of $x$ — so $O(k,x)$ after all, a contradiction. Hence $P(f,x)$, and symmetrically $P(f,y)$; leastness makes $f$ the *greatest* common part. $\square$

The full formal proof is worked out step by step in [proofs/09_mereology/overlap_implies_product/paper_proof.md](../../../proofs/09_mereology/overlap_implies_product/paper_proof.md). Together with fusions (arbitrary joins) and differences (from SSP), products give GEM the entire Boolean toolkit — except that meet and complement are *partial* operations, defined only when $O(x,y)$ and $x \neq u$ respectively. That partiality is exactly the shadow cast by the missing zero.

## Exercises
See [problems/ch16_mereology/](../../../problems/ch16_mereology/)
