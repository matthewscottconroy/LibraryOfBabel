# Categorical Semantics of Type Theory

## The Internal Language Correspondence

Every sufficiently structured category carries a type theory — its *internal language* — and every type theory has an associated category of models. This bidirectional correspondence is not just an analogy: it is a precise mathematical theorem, established through the machinery of categorical logic.

The key insight is that the rules of type theory are not arbitrary choices. They are exactly the rules that hold in every category of a certain kind. When you prove the $\beta$-reduction rule $(\lambda x. t)a = t[a/x]$, you are proving the universal property of the exponential. When you prove the $\eta$-rule $\lambda x. (f\, x) = f$, you are proving the uniqueness clause of the adjunction. The logic and the category theory are the same thing, expressed in different languages.

## The Hierarchy: From STLC to HoTT

The correspondence proceeds in steps, each adding structure to the category and axioms to the type theory:

$$\text{Cartesian Closed Category (CCC)} \longleftrightarrow \text{STLC}$$
$$\text{Locally Cartesian Closed Category (LCCC)} \longleftrightarrow \text{Dependent type theory}$$
$$\text{LCCC + path objects} \longleftrightarrow \text{MLTT (with identity types)}$$
$$\text{Topos} \longleftrightarrow \text{MLTT + higher-order logic}$$
$$\text{$\infty$-Topos} \longleftrightarrow \text{HoTT}$$

Each step adds structure. We trace through them.

## Cartesian Closed Categories and STLC

**Definition.** A *cartesian closed category* (CCC) is a category $\mathcal{C}$ with:
1. A terminal object $1$
2. Binary products: for each $A, B$, an object $A \times B$ with projections and universal property
3. Exponentials: for each $A, B$, an object $[A, B]$ with the natural bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$

Examples: $\mathbf{Set}$, $\mathbf{Vect}_k$, every topos, the category of types in STLC.

**The correspondence.** The internal language of a CCC is exactly STLC:
- Objects of $\mathcal{C}$ ↔ types of STLC
- Morphisms $f : A \to B$ ↔ terms $x : A \vdash f(x) : B$ (in context $x:A$)
- Terminal object $1$ ↔ unit type $\mathbf{1}$
- Product $A \times B$ ↔ product type $A \times B$
- Exponential $[A, B]$ ↔ function type $A \to B$
- Composition ↔ substitution: $f \circ g$ corresponds to $\lambda x. f(g(x))$

The bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$ is the categorical formulation of currying: a morphism $C \times A \to B$ (a function in context $C$ and with argument $A$) corresponds to a morphism $C \to [A, B]$ (a function in context $C$ that returns a function from $A$ to $B$).

**The soundness theorem:** Any STLC term $\Gamma \vdash t : A$ has an interpretation $\llbracket t \rrbracket : \llbracket \Gamma \rrbracket \to \llbracket A \rrbracket$ in any CCC, and if $t \to t'$ by $\beta$-reduction, then $\llbracket t \rrbracket = \llbracket t' \rrbracket$. Types and terms are interpreted as objects and morphisms; reduction is interpreted as equality of morphisms.

**Completeness:** Conversely, the *classifying CCC* of STLC is the CCC $\mathcal{C}_\mathsf{STLC}$ whose objects are types, morphisms are $\beta\eta$-equivalence classes of terms, and products and exponentials are given by the type constructors. Any model of STLC in a CCC is a structure-preserving functor from $\mathcal{C}_\mathsf{STLC}$.

## Locally Cartesian Closed Categories and Dependent Types

**Definition.** A *locally cartesian closed category* (LCCC) is a category $\mathcal{C}$ where every slice category $\mathcal{C}/A$ is a CCC.

Recall: the slice category $\mathcal{C}/A$ has objects $(B, f)$ where $f : B \to A$ is a morphism in $\mathcal{C}$, and morphisms are commuting triangles. In an LCCC, each slice $\mathcal{C}/A$ has products, a terminal object, and exponentials.

Examples: $\mathbf{Set}$, every topos, the category of types in dependent type theory (if we take morphisms into context as the slice).

**The correspondence.** The internal language of an LCCC is dependent type theory:
- Objects $A$ ↔ types $A : \mathcal{U}$
- Morphisms $f : B \to A$ (objects of $\mathcal{C}/A$) ↔ type families $B : A \to \mathcal{U}$
- Terminal object in $\mathcal{C}/A$ ↔ unit type in context $A$
- Products in $\mathcal{C}/A$ ↔ dependent pair types: $(B, f) \times_{/A} (C, g) = (\Sigma_{a:A} B(a) \times C(a), \ldots)$
- Exponentials in $\mathcal{C}/A$ ↔ dependent function types: $[(B, f), (C, g)]_{/A} = (\Pi_{a:A} B(a) \to C(a), \ldots)$

**Substitution as pullback.** The fundamental operation: substituting a term $t : \Gamma \to A$ in a type family $B : A \to \mathcal{U}$ gives $B[t] : \Gamma \to \mathcal{U}$. Categorically: pullback along $t : \Gamma \to A$ gives a functor $t^* : \mathcal{C}/A \to \mathcal{C}/\Gamma$.

The functoriality of substitution ($t^* \circ s^* = (s \circ t)^*$) is the categorical statement of the substitution lemma.

**Dependent quantifiers as adjoints.** Substitution $t^*$ has both a left adjoint $\Sigma_t$ and a right adjoint $\Pi_t$:

$$\Sigma_t \dashv t^* \dashv \Pi_t$$

- $\Sigma_t(B, f) = (\Sigma_{a:A} B(a), \ldots)$ is dependent sum
- $\Pi_t(B, f) = (\Pi_{a:A} B(a), \ldots)$ is dependent product

The adjunctions capture the universal properties: a function out of a $\Sigma$ type is the same as a function taking pairs; a function into a $\Pi$ type is the same as a function that works for each value of the parameter.

This is why the rules for $\Sigma$ and $\Pi$ types have the form they do: they are the rules of adjunctions in slice categories.

## Path Objects and the Identity Type

To model the identity type, we need more structure: *path objects*.

**Definition.** A category $\mathcal{C}$ has *path objects* if for each object $A$, there is an object $\mathsf{Path}(A)$ with maps $r_A : A \to \mathsf{Path}(A)$ (reflexivity) and $(s, t) : \mathsf{Path}(A) \to A \times A$ (source and target), such that $(s, t) \circ r_A = (\mathsf{id}_A, \mathsf{id}_A)$ and a suitable lifting property (weak factorization system).

In MLTT, the identity type $a =_A b$ corresponds to the fiber of $\mathsf{Path}(A) \to A \times A$ over $(a, b)$. The reflexivity proof $\mathsf{refl}_a : a =_A a$ is the image of $a$ under $r_A : A \to \mathsf{Path}(A)$.

**The Awodey-Warren theorem (2009):** Any category with path objects (satisfying appropriate conditions) models MLTT with the identity type. The $J$ eliminator corresponds to a lifting property in the factorization system.

**Independence of UIP.** The *groupoid model* (Hofmann-Streicher, 1994) provides a model of MLTT in the category of groupoids where UIP fails: in a groupoid, the identity type of $a$ and $b$ is the set of morphisms from $a$ to $b$, which can have multiple elements. Since groupoids model MLTT without UIP, UIP is not provable in MLTT.

## Toposes and Higher-Order Logic

**Definition.** An *elementary topos* is a category $\mathcal{C}$ with:
1. Finite limits
2. Exponential objects (making $\mathcal{C}$ a CCC)
3. A *subobject classifier* $\Omega$: an object with a morphism $\top : 1 \to \Omega$ such that every mono $m : B \hookrightarrow A$ is the pullback of $\top$ along a unique morphism $\chi_m : A \to \Omega$

The subobject classifier $\Omega$ is the "object of truth values." In $\mathbf{Set}$, $\Omega = \{0, 1\}$ (true and false). In the topos of sheaves on a topological space $X$, $\Omega$ is the sheaf of open sets — truth values are now open subsets, not just $\{0, 1\}$.

**Internal logic.** Every topos has an internal higher-order intuitionistic logic. Propositions are morphisms $\phi : A \to \Omega$; their interpretation is "the subobject $\{a \in A : \phi(a)\}$." Logical connectives correspond to morphisms in the topos:
- $\wedge : \Omega \times \Omega \to \Omega$ (intersection of subobjects)
- $\vee : \Omega \times \Omega \to \Omega$ (union)
- $\Rightarrow : \Omega \times \Omega \to \Omega$ (implication)
- $\forall_{A/\Omega} : \Omega^A \to \Omega$ (universal quantification)
- $\exists_{A/\Omega} : \Omega^A \to \Omega$ (existential quantification)

The internal logic of a topos is intuitionistic (not classical) in general. It becomes classical exactly when $\Omega = \{0, 1\}$ — when truth values are just "true" and "false."

**Examples of toposes:**
- $\mathbf{Set}$: the canonical topos; its internal logic is classical.
- Presheaf toposes $[\mathcal{C}^{op}, \mathbf{Set}]$: for any small category $\mathcal{C}$.
- Sheaves $\mathsf{Sh}(X)$ on a topological space $X$: truth values are open sets of $X$; spatial reasoning is internalized.
- The effective topos (Hyland): the topos whose internal logic captures realizability (Turing-computable truth).

## The Simplicial Set Model and HoTT Consistency

The most important model for our purposes: the simplicial set model, constructed by Voevodsky (2006–2010).

**Simplicial sets** are presheaves on the simplex category $\Delta$: functors $\Delta^{op} \to \mathbf{Set}$. The category of simplicial sets $\mathbf{sSet}$ is a topos (and more: it has a Quillen model structure whose fibrant objects are Kan complexes).

Voevodsky showed:
1. MLTT can be interpreted in $\mathbf{sSet}$: types are Kan complexes (fibrant simplicial sets), type families are fibrations.
2. The identity type of $A$ is the path space $\mathsf{Path}(A)$: the space of homotopies in $A$.
3. The Univalence Axiom holds in this model: the universe (fibrant replacement of $\mathbf{sSet}$) satisfies $(A = B) \simeq (A \simeq B)$.

Consequence: HoTT + Univalence is consistent — it has a model. The model is constructive mathematics formalized in simplicial sets.

This is the foundational result: HoTT is not just a formal game. It has a mathematical model, and that model is the heart of classical homotopy theory.

## ∞-Toposes and HoTT

The full story, still being developed:

**Definition (Lurie, 2009).** An *$\infty$-topos* is an $(\infty, 1)$-category satisfying certain exactness conditions (descent, universal colimits, generation by a set of compact objects).

The theorem: HoTT (with Univalence and HITs) is the internal language of $\infty$-toposes. Every ∞-topos provides a model of HoTT, and the HoTT rules axiomatize exactly what is true in all ∞-toposes.

Examples of ∞-toposes:
- The ∞-category of ∞-groupoids (spaces): the "canonical" ∞-topos, modeled by Kan complexes
- Slice ∞-toposes $\mathcal{H}/X$ for any ∞-topos $\mathcal{H}$ and object $X$
- The ∞-topos of parametrized spectra (for stable homotopy)
- ∞-Toposes over specific sites (étale, Zariski) for algebraic geometry

The ∞-topos perspective explains why HITs exist and what they mean: they are homotopy colimits in the ∞-topos. The circle $S^1$ as a HIT (with one point constructor and one path constructor) corresponds to the geometric realization of the simplicial circle. The fundamental theorem $\pi_1(S^1) = \mathbb{Z}$ is a theorem in every ∞-topos, proved using the synthetic reasoning of HoTT.

This is the endpoint of the hierarchy: not just "HoTT is consistent" (which follows from the simplicial set model), but "HoTT is the internal language of ∞-toposes" — meaning that HoTT-provable statements are true in every ∞-topos, and ∞-topos-valid statements are HoTT-provable. The two frameworks are equivalent.
