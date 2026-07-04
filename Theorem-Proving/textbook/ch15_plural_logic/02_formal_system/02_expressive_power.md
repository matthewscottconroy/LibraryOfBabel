# Second-Order Logic and Expressive Power

Plural logic's importance rests on a single structural fact: it has the expressive strength of second-order logic while — if the innocence thesis holds — the ontology of first-order logic. This section proves the equivalence with monadic second-order logic, draws out the consequences (finiteness, categorical arithmetic) that lie beyond first-order reach, and then weighs the objections to the claim that all this power is ontologically free.

## The Interpretation of Monadic Second-Order Logic

**Monadic second-order logic (MSO)** extends first-order logic with variables $X, Y, \dots$ ranging over *subsets* of the domain, an atomic formula $X(t)$ ("$t$ is in $X$"), and second-order quantifiers $\exists X, \forall X$, governed by the comprehension schema $\exists X\,\forall x\,(X(x) \leftrightarrow \phi(x))$ for each $\phi$ not containing $X$ free.

**Definition (Plural translation).** Map each MSO formula $\sigma$ to a plural formula $\sigma^{\ast}$ by:
$$x = y \mapsto x = y, \quad R(\bar t) \mapsto R(\bar t), \quad X(t) \mapsto t \prec xx_X, \quad (\cdot)^\ast \text{ commutes with } \neg, \land, \lor, \to,$$
$$(\exists x\,\sigma)^\ast = \exists x\,\sigma^\ast, \quad (\exists X\,\sigma)^\ast = \exists xx_X\,\sigma^\ast, \quad (\forall X\,\sigma)^\ast = \forall xx_X\,\sigma^\ast,$$
where each monadic variable $X$ is assigned a distinct plural variable $xx_X$.

**Theorem (Boolos 1984).** Over any domain, an MSO sentence $\sigma$ and its translation $\sigma^\ast$ have the same truth value, once the empty subset is accommodated. Hence MSO and plural logic are **expressively equivalent**.

*Proof.* By induction on $\sigma$. Atomic, boolean, and first-order-quantifier cases are immediate since $(\cdot)^\ast$ leaves them fixed up to renaming. For the second-order quantifier: a subset $S \subseteq A$ that interprets $X$ corresponds to the plurality with exactly the members of $S$, and $X(a)$ holds iff $a \in S$ iff $a \prec xx_X$ under the matching plural assignment. Thus $\exists X\,\sigma$ is true (some subset $S$ verifies $\sigma$) iff some plurality verifies $\sigma^\ast$, i.e. $\exists xx_X\,\sigma^\ast$. The MSO comprehension instance for $\phi$ translates to the plural comprehension instance for $\phi$, which is P-Comp. $\square$

**The empty-set caveat.** Plural variables range over *nonempty* pluralities, whereas $X$ may be assigned $\varnothing$. The gap is closed uniformly: relativize each plural quantifier so that the empty case is handled by an extra disjunct, translating $\exists X\,\sigma(X)$ as $\sigma^\ast[\bot] \lor \exists xx\,\sigma^\ast$, where $\sigma^\ast[\bot]$ replaces every "$t \prec xx$" by $\bot$ (nothing is one of the empty plurality). With this patch the equivalence is exact. Some presentations instead admit an empty plural term (Oliver–Smiley's "zilch") and drop the patch.

## What This Buys: Beyond the First Order

Plural logic thereby inherits the expressive power of monadic second-order logic, which strictly exceeds first-order logic. And with one further ingredient it reaches *full* second-order logic. MSO quantifies only over subsets (monadic properties), not over binary relations; to code a relation $R \subseteq A \times A$ as a *single* plurality one needs a **pairing function** $\langle \cdot,\cdot\rangle : A \times A \rightarrowtail A$ in the first-order base, so that $R$ is represented by the plurality of its codes $\langle a,b\rangle$. In any setting that supplies pairing — arithmetic, or a domain with enough structure — plural quantification then simulates quantification over relations, i.e. full second-order logic. This is exactly the setting of Boolos's *Nominalist Platonism*: second-order arithmetic, read plurally.

Two payoffs stand out, both provably impossible in first-order logic (Chapter 9):

- **Defining finiteness.** With pairing available, a plurality $xx$ is **Dedekind-infinite** iff there are some pairs coding an injection of $xx$ into a *proper* sub-plurality of itself; $xx$ is **finite** iff not. First-order logic cannot express "there are finitely many $F$s": by compactness, any theory with arbitrarily large finite models has an infinite model, so "finite" has no first-order definition. Plurals express it outright.
- **Categorical arithmetic.** The second-order Dedekind–Peano axioms — with induction stated as *"for any things $xx$, if $0 \prec xx$ and $xx$ is closed under successor, then every number is one of the $xx$"* — are **categorical**: all their models are isomorphic to $\langle \mathbb{N}, 0, S\rangle$. First-order Peano arithmetic, by Löwenheim–Skolem, has non-standard models of every infinite cardinality. The plural induction axiom pins down $\mathbb{N}$ up to isomorphism, quantifying over *pluralities of numbers* rather than sets of numbers. Boolos's point: this categoricity, long taken to show that second-order logic is set theory in disguise, needs only plural quantification over the numbers themselves.

The plural rendering of the **ancestral** (Frege's "following in a $\phi$-series") is the same move in miniature: "$b$ is an $R$-ancestor of $a$" iff $b$ is one of *every* plurality that contains $a$ and is closed under $R$ — a plural $\forall xx$ replacing Frege's quantification over concepts. Transitive closure, well-foundedness, and reachability (Chapter 14's model-checking properties) are all plural-definable in the same way.

## The Innocence Debate

Everything above assumed Boolos's prize: that plural quantification is *ontologically innocent*, so this second-order strength costs no new objects. The thesis is contested.

- **Quine's challenge**, inherited from his verdict on second-order logic, is that plural logic is "set theory in sheep's clothing": the plural comprehension schema is an existence axiom as substantive as Separation, so pluralities are sets under another name. Boolos replies that $\prec$ is provably not $\in$ (there is a universal plurality but no universal set; no plurality is a member of anything; the paradoxes do not arise), and that the values of plural variables are individuals, not a new domain of collections.
- **Resnik (1988), "Second-Order Logic Still Wild,"** presses that using set-valued *semantics* to prove the metatheorems (soundness, the interpretation theorem above) re-imports commitment to sets. The Boolosian reply distinguishes object language from metalanguage and gives the metatheory itself plurally (McKay, Rayo), so no set is asserted to exist by the theory whose innocence is at issue.
- **Parsons and Linnebo** sharpen the question rather than dismiss it. Linnebo, in *Plural Quantification Exposed* (2003) and the Stanford Encyclopedia article *Plural Quantification*, distinguishes several theses that "innocence" might mean — no commitment to sets, no commitment to *any* collective entities, no new *ideology*, epistemic parity with first-order logic — and argues that plural logic is innocent in some of these senses but not all: P-Comp remains a genuine, non-trivial existence principle about pluralities, even if pluralities are not objects. On this diagnosis plurals are cheaper than sets but not free.

The stakes are the ones flagged in [Section 2](../01_foundations/02_boolos_plural.md): if plurals are innocent, second-order logic is genuine logic and Frege's logicism can be revived without paradox; if not, the reduction merely relocates the ontological cost. Either way the *expressive* facts of this section stand — plural logic reaches where first-order logic cannot — and it is the expressive reach, harnessed to mereology and higher-order plurals, that the [final section](../03_developments/01_higher_order_and_foundations.md) turns to the foundations of mathematics.

## Exercises
See [problems/ch15_plural_logic/](../../../problems/ch15_plural_logic/)
