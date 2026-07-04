# Tarski's Elementary Geometry: Complete and Decidable

Here is the chapter's logical payoff, and one of the most surprising results in metamathematics. Gödel had shown (Chapter 10) that any consistent, recursively axiomatized theory able to interpret arithmetic is **incomplete** and **undecidable**. One might expect geometry — soaked in the continuum, home of $\pi$ and $\sqrt 2$ — to be worse. Alfred Tarski proved the opposite: the first-order theory of elementary Euclidean geometry is **consistent, complete, and decidable**. For every geometric sentence $\sigma$ the theory settles $\sigma$ or $\neg\sigma$, and an algorithm computes which. Geometry, apparently richer than arithmetic, is *tame* where arithmetic is wild — and understanding why is understanding what incompleteness really is.

## A First-Order Language of Two Primitives

Tarski's crucial decision was to make geometry **first-order and point-based**: one sort of variable (points), no quantification over sets, lines, or regions. Just two primitive predicates suffice:

- **Betweenness** $B(x,y,z)$: point $y$ lies on the segment from $x$ to $z$ (endpoints allowed).
- **Congruence** $D(x,y,z,w)$, written $xy \equiv zw$: segment $xy$ is congruent to segment $zw$.

Everything classical is definable. Collinearity is $\operatorname{Col}(x,y,z) := B(x,y,z) \lor B(y,z,x) \lor B(z,x,y)$; midpoints, perpendiculars, angle congruence, and circles ($D(c,x,c,p)$) all reduce to $B$ and $\equiv$. No separate primitive for angles is needed.

## The Axioms

Tarski's system — call it $\mathcal{E}_2$ — has about a dozen axioms plus one schema:

- reflexivity, symmetry, transitivity, and identity of congruence ($xy \equiv zz \to x=y$);
- **identity of betweenness**: $B(x,y,x) \to x=y$;
- **segment construction**: for any $x,y$ and segment $ab$, some $z$ has $B(x,y,z)$ and $yz \equiv ab$;
- the **five-segment axiom**, a first-order encoding of **SAS** using five points;
- **Pasch's axiom** (inner form) and **Euclid's axiom** (a first-order parallel postulate);
- **lower and upper dimension axioms**, fixing the dimension at exactly $2$.

And the axiom carrying the weight — the **continuity schema**: for all first-order $\varphi(x)$, $\psi(y)$ (with parameters), the universal closure of
$$\exists a\,\forall x\,\forall y\,\bigl(\varphi(x) \land \psi(y) \to B(a,x,y)\bigr) \;\to\; \exists b\,\forall x\,\forall y\,\bigl(\varphi(x) \land \psi(y) \to B(x,b,y)\bigr).$$
If the $\varphi$-points all precede the $\psi$-points, some $b$ lies between all of them: a **Dedekind cut restricted to first-order-definable cuts** — a schema, one axiom per pair of formulas, exactly like Separation in ZF or Induction in PA.

## Elementary vs. Full Continuity

Everything turns on the word *elementary*. Hilbert's V.2 quantified over **arbitrary** sets of points — second-order, categorical, unique model $\mathbb{R}^2$. Tarski's schema quantifies only over **definable** cuts, and the price is categoricity: $\mathcal{E}_2$ has many non-isomorphic models — the planes over $\mathbb{R}$, over the **real algebraic numbers** $\mathbb{R}_{\mathrm{alg}}$, over the real closure of $\mathbb{Q}(t)$, and over non-Archimedean real-closed fields, where a cut defined by no formula may lack a filling point. The reward is immense: $\mathcal{E}_2$ is a *bona fide first-order theory*, subject to completeness and compactness (Chapter 9) — and, it turns out, to a decision procedure.

## Geometry Is Real-Closed Fields

The engine is a first-order sharpening of Hilbert's segment arithmetic ([Section 3](../03_hilbert/01_hilbert_axioms.md)).

**Definition.** A **real-closed field (RCF)** is an ordered field in which every positive element has a square root and every odd-degree polynomial has a root. $\mathbb{R}$ and $\mathbb{R}_{\mathrm{alg}}$ are real-closed; $\mathbb{Q}$ is not.

**Theorem (Coordinatization).** Every model of $\mathcal{E}_2$ is definably isomorphic to the plane $F^2$ over some real-closed field $F$; conversely every $F^2$ (with the natural $B$ and $D$) models $\mathcal{E}_2$.

So $\mathcal{E}_2$ and the theory of real-closed fields are **bi-interpretable**: betweenness and congruence translate into polynomial equalities and inequalities over $F$, and back. Whatever is decidable about one is decidable about the other.

## Quantifier Elimination and the Decision Theorem

The algebraic side is Tarski's other great theorem, from *A Decision Method for Elementary Algebra and Geometry* (RAND, 1948; expounded in "What is Elementary Geometry?", 1959).

**Theorem (Tarski).** $\mathrm{RCF}$ in the language $\{+,\cdot,<,0,1\}$ admits **quantifier elimination**: every formula is provably equivalent to a quantifier-free Boolean combination of polynomial equalities and inequalities.

From QE the payoff cascades:

1. **Completeness.** A quantifier-free *sentence* is a ground combination of facts like $1+1>0$, decided outright; so $\mathrm{RCF}$ proves or refutes every sentence. Equivalently all real-closed fields are elementarily equivalent — $\mathbb{R}$, $\mathbb{R}_{\mathrm{alg}}$, and every non-Archimedean RCF satisfy the same first-order sentences (Tarski's **transfer principle**).
2. **Decidability.** The elimination is *effective*: strip the quantifiers, evaluate the ground formula.
3. **Transfer.** Via bi-interpretation, $\mathcal{E}_2$ inherits both. **Elementary Euclidean geometry is complete and decidable.**

> **Corollary.** Every elementary statement of plane geometry — the medians are concurrent, the perpendicular bisectors meet at the circumcenter, the nine-point circle exists — is algorithmically decidable, and every true one is provable from Tarski's axioms.

**Quantifier elimination in miniature.** The heart of QE is one example: eliminate the quantifier from $\exists x\,(a x^2 + b x + c = 0)$ over an RCF with $a \ne 0$. A real root exists iff the discriminant is non-negative, so the formula is equivalent to the quantifier-free
$$b^2 - 4ac \ge 0.$$
Full QE handles $a=0$, systems, and strict inequalities by tracking the *signs* of a family of polynomials — but the idea is always: an existential over the reals reduces to finitely many sign conditions on coefficients.

## Why Geometry Escapes Gödel

Now the conceptual heart. How can geometry be decidable when arithmetic is not? Gödel's incompleteness needs a theory that can **interpret arithmetic** — encode syntax, define $+$ and $\times$ on a copy of $\mathbb{N}$, and diagonalize. Robinson arithmetic $Q$ suffices, and $Q$ is interpretable in ZF and PA — but *not* in $\mathrm{RCF}$.

**The obstruction: $\mathbb{Z}$ is not definable in the real field.** By quantifier elimination, every definable subset of the line in a real-closed field is a finite union of points and intervals — a **semialgebraic** set. But $\mathbb{Z}$ is infinite and discrete with no interval, so **no first-order formula in $\{+,\cdot,<\}$ defines the integers**. One therefore cannot express "$x$ is a natural number," cannot quantify over integers, cannot build Gödel codes or the diagonal sentence. The engine of incompleteness has no fuel.

> **Incompleteness is not a symptom of richness, size, or difficulty. It is the specific mark of theories that can define the integers and thereby interpret arithmetic.** A theory of the *continuum* can be complete and decidable; a theory of the *discrete* natural numbers cannot. Geometry is decidable precisely because it cannot count.

This mirrors [Chapter 16](../../ch16_mereology/02_comparison/01_mereology_vs_sets.md), where classical mereology is decidable for the same species of reason — its models are Boolean algebras, tamed by quantifier elimination, with no room to encode arithmetic — while set theory, which builds $\mathbb{N}$, is essentially undecidable. Add just enough to $\mathrm{RCF}$ to pin down $\mathbb{Z}$ — a predicate for the integers, or the sine function (whose zeros are $\pi\mathbb{Z}$) — and decidability collapses back into Gödelian undecidability. Tameness lives on a knife's edge.

## Decidable Is Not Feasible

A logical triumph need not be a computational one. The decision problem for $\mathrm{RCF}$ is provably **hard**: Fischer and Rabin gave a doubly-exponential lower bound, and Davenport–Heintz showed quantifier elimination is **doubly exponential in the number of variables**. Tarski's original procedure was non-elementary; the practical workhorse is George Collins's **Cylindrical Algebraic Decomposition (CAD)** (1975), enough for real problems in robot motion planning and CAD but hopeless for large formulas, with modern SMT solvers (Z3's `nlsat`) and tools like QEPCAD and Redlog pushing the envelope. So "geometry is decidable" asserts an algorithm exists, not that it is efficient — but it is that existence which shattered the Gödelian expectation that sufficiently rich mathematics must forever elude mechanical decision. Mechanizing that decision is [Section 5](../05_lean/01_geometry_lean.md).

## Exercises
See [problems/ch20_geometry_and_logic/](../../../problems/ch20_geometry_and_logic/)
