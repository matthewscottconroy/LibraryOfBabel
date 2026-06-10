# Toposes

## Categories That Behave Like Sets

A *topos* is a category that behaves like the category of sets — but in which "sets" can have geometric content (spatial structure), logical content (sheaves of truth values), or computational content (realizability). The category $\mathbf{Set}$ is the initial example; but there are many toposes, and each has its own internal logic.

The key insight: the properties of $\mathbf{Set}$ that make it a foundation for mathematics can be axiomatized categorically. Once axiomatized, you can study them in any topos, and theorems that hold in all toposes are "topos-valid" — true in a logical sense that doesn't depend on the specific "set theory" you're working with.

For HoTT, toposes matter in two ways: (1) every elementary topos gives a model of MLTT + HOL (higher-order logic), and (2) every ∞-topos gives a model of HoTT. Understanding ordinary toposes is the prerequisite for understanding ∞-toposes.

## Elementary Toposes

**Definition.** An *elementary topos* is a category $\mathcal{E}$ with:

1. **Finite limits:** terminal object $1$, binary products $A \times B$, and equalizers (hence all finite limits).

2. **Exponentials:** for each $A, B$, an exponential object $B^A$ with the natural bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, B^A)$. (This makes $\mathcal{E}$ a CCC.)

3. **A subobject classifier:** an object $\Omega$ with a morphism $\top : 1 \to \Omega$ such that for every monomorphism $m : B \hookrightarrow A$, there exists a unique morphism $\chi_m : A \to \Omega$ making the following square a pullback:

$$\begin{array}{ccc} B & \xrightarrow{!} & 1 \\ \downarrow_m & & \downarrow_\top \\ A & \xrightarrow{\chi_m} & \Omega \end{array}$$

The morphism $\chi_m$ is the *characteristic morphism* or *classifying map* of $m$. The subobject classifier $\Omega$ is the "object of truth values."

**Size note.** Unlike Grothendieck toposes (which additionally satisfy smallness conditions), elementary toposes are defined by a finite list of axioms. They include Grothendieck toposes as special cases but also the effective topos (which is not a Grothendieck topos).

## The Subobject Classifier

The subobject classifier $\Omega$ is the central new ingredient over the CCC structure. In $\mathbf{Set}$: $\Omega = \{0, 1\} = \{\mathsf{false}, \mathsf{true}\}$, and the characteristic function $\chi_S : A \to \{0,1\}$ of a subset $S \subseteq A$ is $\chi_S(a) = 1$ iff $a \in S$.

The topos axiom generalizes this: in any topos, every "subobject" (monomorphism $B \hookrightarrow A$) corresponds uniquely to a morphism $A \to \Omega$. So $\Omega$ classifies all subobjects.

The logical content: $\Omega$ is the "object of propositions." A predicate on $A$ is a morphism $\phi : A \to \Omega$. The subset $\{a \in A : \phi(a)\}$ is defined categorically as the pullback of $\top : 1 \to \Omega$ along $\phi$.

**Logical operations on $\Omega$.** Since $\Omega$ classifies propositions, the logical connectives correspond to morphisms on $\Omega$:

- $\wedge : \Omega \times \Omega \to \Omega$ (conjunction: the characteristic morphism of $\{(p,q) : p = \top \text{ and } q = \top\}$)
- $\vee : \Omega \times \Omega \to \Omega$ (disjunction)
- $\Rightarrow : \Omega \times \Omega \to \Omega$ (implication)
- $\neg : \Omega \to \Omega$ (negation: $\neg = (\Rightarrow) \circ (\mathsf{id} \times (\bot : 1 \to \Omega))$)
- $\top : 1 \to \Omega$ and $\bot : 1 \to \Omega$ (truth and falsity)

These make $\mathsf{Hom}(A, \Omega)$ into a Heyting algebra for each object $A$. The internal logic of any topos is *intuitionistic*: it satisfies all of intuitionistic higher-order logic, but not necessarily classical logic (unless $\Omega \cong 1 + 1$, i.e., unless the topos is Boolean).

**Quantifiers in a topos.** The universal and existential quantifiers are defined as morphisms:

$$\forall : \Omega^A \to \Omega \quad \text{and} \quad \exists : \Omega^A \to \Omega$$

where $\Omega^A$ is the exponential (the "object of propositions over $A$"). The morphism $\forall_A : \Omega^A \to \Omega$ sends a predicate $\phi : A \to \Omega$ to the proposition $\forall a \in A. \phi(a)$. In type theory: $\forall_A$ sends $\phi : A \to \Omega$ to $\Pi_{a:A} \phi(a)$ (which is a proposition since $\Omega = \mathsf{Prop}$).

## Examples of Toposes

**$\mathbf{Set}$:** The paradigmatic topos. $\Omega = \{0,1\}$. The logic is classical. Every topos has a unique *geometric morphism* $\mathcal{E} \to \mathbf{Set}$ (the "global sections" functor), so $\mathbf{Set}$ is the terminal topos.

**Presheaf toposes $[\mathcal{C}^{op}, \mathbf{Set}]$:** For any small category $\mathcal{C}$, the category of presheaves is a (Grothendieck) topos. The subobject classifier is the presheaf $\Omega$ sending each object $C$ to the set of sieves on $C$ (downward-closed subcategories of $\mathcal{C}/C$). Presheaf toposes model *variable set theory*: sets that vary continuously over the objects of $\mathcal{C}$.

**Sheaf toposes $\mathsf{Sh}(X)$:** For a topological space $X$, sheaves on $X$ form a topos. The subobject classifier sends an open set $U$ to the set of open subsets of $U$. The internal logic is the logic of *spatial reasoning*: a proposition can be "locally true" on an open set but fail on a larger open set. This is the logic appropriate for continuous variation.

Example: in the sheaf topos $\mathsf{Sh}(\mathbb{R})$, the "real numbers" (the sheaf sending $U$ to the set of continuous functions $U \to \mathbb{R}$) form a ring in which every element is either zero or invertible — a field — but it need not be the case that every element is zero or nonzero (because "zero on this open set but nonzero on that one" is possible). The Law of Excluded Middle fails in this topos.

**The effective topos $\mathsf{Eff}$:** Hyland's effective topos (1982) is the topos of "realizability." Its objects are assemblies: sets equipped with a "realizability" relation telling which computable functions witness the elements. The internal logic is the logic where a proposition $P$ holds iff it is *computably provable* — there is a Turing machine that computes a proof.

In $\mathsf{Eff}$: Church's Thesis (all total functions $\mathbb{N} \to \mathbb{N}$ are computable) holds. The Axiom of Choice is equivalent to Markov's Principle. The Law of Excluded Middle fails: there is no computable decision procedure for all propositions.

**The simplicial set model:** The category $\mathbf{sSet}$ of simplicial sets is a Grothendieck topos (the presheaf topos on the simplex category $\Delta$). Its objects are simplicial sets, its subobject classifier is the simplicial set of "open simplices." This is Voevodsky's model of HoTT (once restricted to Kan complexes and equipped with a universe).

## Internal Logic and the Mitchell-Bénabou Language

Every topos $\mathcal{E}$ has an *internal language* — the *Mitchell-Bénabou language* — which is a form of higher-order logic:

- *Types* are objects of $\mathcal{E}$
- *Terms* of type $A$ in context $\Gamma$ are morphisms $\Gamma \to A$
- *Propositions* are terms of type $\Omega$
- *Proofs* of propositions are terms of type $1$ that compose correctly

The logical connectives are defined using the structure of $\mathcal{E}$ (the morphisms on $\Omega$ described above). Quantifiers use the exponential: $\forall x:A. \phi(x)$ is interpreted using $\forall_A : \Omega^A \to \Omega$.

**Sound and complete:** The internal logic of $\mathcal{E}$ is sound (every formula provable in the logic holds in $\mathcal{E}$) and complete (every formula true in $\mathcal{E}$ is provable in the logic). This means the logic *exactly* captures what is true in the topos.

**Axioms vary.** Different toposes validate different additional axioms:
- $\mathbf{Set}$: validates classical logic (LEM) and the axiom of choice
- $\mathsf{Sh}(X)$ for non-trivial $X$: validates intuitionistic logic but not LEM
- $\mathsf{Eff}$: validates Church's thesis and Markov's principle
- $\mathbf{sSet}$ (with Kan fibration model structure): validates MLTT and (with the right interpretation) the univalence axiom

The multiplicity of toposes is the categorical explanation of why different logical axioms are independent: different toposes validate different axioms, so no single axiom can be deduced from the topos axioms alone.

## Independence Results via Topos Theory

The most important application of topos theory to logic: proving *independence* of axioms.

**LEM is independent of intuitionistic logic.** The sheaf topos $\mathsf{Sh}(\mathbb{R})$ validates intuitionistic logic but not LEM (as shown above). So LEM cannot be proved from the intuitionistic axioms.

**UIP is independent of MLTT.** The groupoid model (Hofmann-Streicher 1994) validates MLTT but not UIP. So UIP cannot be proved from the MLTT axioms. (This independence result is proved using a *fibered* version of topos theory, but the structure is the same.)

**Univalence is independent of MLTT.** The set model validates MLTT but not univalence (since in the set model, the only "paths" between types are set-theoretic equalities, which don't include non-trivial equivalences). So univalence cannot be proved from MLTT alone — it must be added as an axiom (or as a theorem of cubical type theory).

## From Toposes to ∞-Toposes

The elementary topos axioms capture what is needed for ordinary higher-order logic over "set-like" objects. For HoTT, we need the ∞-version: an ∞-topos, where the "sets" are replaced by "homotopy types" (∞-groupoids).

An *∞-topos* (Lurie) is an (∞,1)-category satisfying:
1. It is presentable (generated by a small set of compact objects under colimits)
2. It satisfies *descent*: colimits are universal (stable under base change)
3. It is "locally" an ∞-groupoid: the slice categories $\mathcal{H}/X$ are ∞-toposes

The prototypical ∞-topos is the ∞-category of ∞-groupoids (or Kan complexes). Every Grothendieck topos gives an ∞-topos by taking its ∞-categorical enhancement.

HoTT is the *internal language* of ∞-toposes: a statement in HoTT (with univalence and HITs) is true in every ∞-topos, and conversely, every statement true in all ∞-toposes is provable in HoTT. This is the deepest version of the Curry-Howard-Lambek correspondence.
