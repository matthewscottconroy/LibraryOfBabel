# Applications: Categorical Logic

## 1. Forcing and Independence Results in Set Theory

The most famous application of topos theory to foundational mathematics: forcing, as pioneered by Paul Cohen (1963) for set theory, can be understood as a passage to a different topos.

Cohen proved the independence of the Continuum Hypothesis (CH) from ZFC by constructing a "forcing extension" of the set-theoretic universe. In category-theoretic terms: a forcing extension is a passage from the topos $\mathbf{Set}$ (the ambient set theory) to a sheaf topos $\mathsf{Sh}(P, J)$ over a *forcing poset* $P$ with a specific Grothendieck topology $J$.

The key theorem: if $\phi$ is a sentence in the language of set theory, then $\phi$ holds in the forcing extension iff it holds in the sheaf topos $\mathsf{Sh}(P, J)$ (under the translation given by the internal language). Cohen's proof that CH is independent of ZFC then corresponds to finding a specific forcing poset $P$ and topology $J$ such that $\mathsf{Sh}(P, J)$ satisfies "the cardinality of $\mathbb{R}$ is $\aleph_2$" (violating CH).

The topos-theoretic formulation (Tierney, Barr) makes the structure transparent: different toposes model different set-theoretic universes, and independence results correspond to finding toposes that validate or invalidate specific statements. The techniques of algebraic topology (sheaves, sites, Grothendieck topologies) become tools for proving logical independence.

For HoTT: analogous techniques apply to the independence of HoTT axioms. The groupoid model proves UIP is independent; the simplicial set model proves univalence is consistent; the effective topos provides a realizability model where Church's Thesis holds.

## 2. Domain Theory and Denotational Semantics

Domain theory — the mathematics of fixed points and continuous functions for computer science — has a categorical formulation in terms of *Scott domains* and the *Scott topology*.

A Scott domain is a partially ordered set with a specific completeness property (directed complete partial order with a least element). The continuous functions between Scott domains are the monotone functions that preserve directed joins. The category of Scott domains and continuous functions, $\mathbf{Dom}$, is a CCC (cartesian closed category): products and function spaces exist and satisfy the CCC axioms.

This makes $\mathbf{Dom}$ a model of STLC with fixpoints: there is a fixpoint combinator $Y_A : (A \to A) \to A$ for every type $A$ (since every endomorphism has a least fixed point in a domain). The CCC structure gives the denotational semantics: each STLC type is a domain, each term is a continuous function, and the $\beta$-reduction rules are equalities of continuous functions.

Dana Scott's construction of a domain $D$ satisfying $D \cong [D, D]$ (a domain isomorphic to its own function space) gives a denotational semantics for the *untyped* lambda calculus: the type $D$ models "all programs," and the self-referential isomorphism gives meaning to self-application.

For dependent type theory: *realizability models* (like Hyland's effective topos) provide categorical models where the types are not just sets but "assemblies" — sets equipped with a realizability structure. This gives semantics for dependent type theory in a computationally meaningful setting.

## 3. Proof Irrelevance and Propositions in Coq

In Coq (and other proof assistants based on CIC), there are two universes: `Type` (for data types and computational types) and `Prop` (for propositions, proof-irrelevant types). The distinction is:

- `A : Prop` means $A$ is a proposition: any two proofs of $A$ are definitionally equal. Proof irrelevance holds for `Prop`.
- `A : Type` means $A$ is a data type: elements can be distinct and computationally meaningful.

Categorically: `Prop` corresponds to the subobject classifier $\Omega$ in the underlying topos, while `Type` corresponds to the full universe of types. The inclusion `Prop → Type` is the *subobject classifier morphism* in the categorical semantics.

The distinction has practical consequences: functions $f : A \to B$ where $A : Prop$ can be eliminated (since we can "use" the proof), but functions $g : A \to P$ where $P : Prop$ cannot depend on which specific proof of $A$ was provided (since proofs of $P$ are proof-irrelevant, $g$ must produce the same result for any proof).

This is the categorical content of *proof irrelevance*: propositions in the `Prop` universe are modeled by subobjects of $1$ (subterminal objects), and functions into subterminal objects don't depend on the specific element of the domain.

HoTT replaces `Prop` with h-props ($(-1)$-truncated types) and makes proof relevance the default: identity proofs are not necessarily irrelevant, and the distinction between "propositions" and "types" is internal to the type theory (not imposed by two separate universes).

## 4. Realizability and Verified Algorithm Extraction

The *realizability interpretation* of constructive type theory gives a computational interpretation of formal proofs: every proof of a theorem $P$ provides a *realizer* — a computable function that witnesses the truth of $P$.

Formally: in the effective topos $\mathsf{Eff}$, every object $A$ is an assembly $(|A|, E_A)$ where $|A|$ is a set of elements and $E_A : |A| \to \mathcal{P}(\mathbb{N})$ assigns to each element a nonempty set of "realizers" (natural numbers that witness membership in $A$). A proof of $P$ in the internal logic of $\mathsf{Eff}$ comes equipped with a realizer — a Turing machine that computes the proof.

The application: if you formalize a theorem in a constructive type theory (like Coq with intuitionistic logic), the proof term itself is a program. The *extraction* mechanism in Coq translates the proof into an OCaml program that computes the result. The correctness of the extracted program follows from the soundness of the type theory.

This has been applied to algorithms:
- **Sorting algorithms**: A proof that a function $\mathsf{sort} : \mathsf{List}\, \mathbb{N} \to \mathsf{List}\, \mathbb{N}$ produces a sorted permutation of its input, together with a constructive proof of the existence of such a function, gives an extracted OCaml implementation that is *provably correct by construction*.
- **Number theory**: Constructive proofs of the Chinese Remainder Theorem, Bézout's identity, and other number-theoretic results extract to certified implementations of the corresponding algorithms.
- **Formal verification of compilers** (CompCert): The CompCert C compiler is formally verified in Coq, extracting to a real C compiler with machine-checked correctness. The categorical semantics ensures that the compilation preserves the semantics of C programs.

## 5. Geometric Logic and Database Coherence

*Geometric logic* is the fragment of first-order logic generated by $\top, \bot, \wedge, \vee$ (including infinitary $\bigvee$) and $\exists$ — without $\forall$, $\Rightarrow$, or $\neg$. This is the logic preserved by *geometric morphisms* (functors between toposes that preserve finite limits and all colimits).

Every Grothendieck topos is the classifying topos of a *geometric theory* — a theory in geometric logic. The internal logic of the topos is exactly the theory being classified.

Applications to databases: a relational database schema can be expressed as a geometric theory. A database *instance* is a model of the theory (a set-valued functor). Database *morphisms* (schema evolution, data migration) are geometric morphisms between classifying toposes.

The coherence: geometric morphisms preserve all constructions expressible in geometric logic. This means that data migrations that are "geometric" (preserve the schema structure) automatically preserve all the relational constraints — and this preservation is a theorem, not just an engineering convention.

The Gabriel-Ulmer duality provides a precise correspondence between geometric theories and their classifying toposes, making this framework useful for automated verification of database schema coherence.

## 6. Homotopy-Theoretic Consistency Proofs

Voevodsky's simplicial set model is not just a consistency proof — it provides a *computational* interpretation of HoTT.

The *canonicity* theorem (Huber, 2016, for cubical type theory): every closed term of type $\mathbb{N}$ (natural numbers) in HoTT + Univalence *computes* to a numeral $\underline{n}$. This means:
1. The type theory is consistent: there is no closed term of type $\mathbf{0}$ (the empty type)
2. The computation is effective: there is a algorithm that reduces any term to its normal form

Canonicity was an open problem for HoTT (with Univalence as an axiom) for several years. The resolution came through *cubical type theory* (Cohen-Coquand-Huber-Mörtberg, 2015): by replacing the Univalence Axiom with a *computational* version (the Glue type), all terms including univalence-using terms can be reduced to normal form.

The categorical model underlying this is the *cubical set model* (presheaves on the cube category). Unlike the simplicial set model (where univalence holds only semantically), the cubical model validates univalence *definitionally* — the computation of $\mathsf{ua}(f)(a)$ is a term in the type theory, not just a semantic value.

This is the deepest application of categorical logic to HoTT: not just a consistency proof, but a computational realization of the entire theory.
