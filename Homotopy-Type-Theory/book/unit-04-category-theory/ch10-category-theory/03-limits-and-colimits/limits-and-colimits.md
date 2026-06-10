# Limits and Colimits

## Universal Constructions

The deepest idea in category theory is the *universal property*: instead of constructing an object by specifying its internal structure, define it by specifying how it relates to all other objects. The object defined by a universal property is unique up to unique isomorphism — it is the "best" object satisfying the property, from any direction.

Limits and colimits are the two fundamental types of universal construction. Every object defined by a universal property is, in a precise sense, either a limit or a colimit. Products, terminal objects, pullbacks, and equalizers are all limits. Coproducts, initial objects, pushouts, and coequalizers are all colimits. These appear throughout mathematics under different names, but they are instances of a single general concept.

## Diagrams and Cones

**Definition.** A *diagram* of shape $\mathcal{J}$ in $\mathcal{C}$ is a functor $D : \mathcal{J} \to \mathcal{C}$.

The category $\mathcal{J}$ is the *index category* — it specifies the "shape" of the diagram. It can be any small category.

**Definition.** A *cone* over $D : \mathcal{J} \to \mathcal{C}$ with *vertex* $C$ is a family of morphisms $(\phi_j : C \to D(j))_{j \in \mathcal{J}}$ that is *compatible* with the diagram: for every morphism $f : j \to k$ in $\mathcal{J}$, the triangle $D(f) \circ \phi_j = \phi_k$ commutes.

Equivalently, a cone is a natural transformation from the constant functor $\Delta_C : \mathcal{J} \to \mathcal{C}$ (sending every object to $C$, every morphism to $\mathsf{id}_C$) to $D$.

**Definition.** A *limit* of $D : \mathcal{J} \to \mathcal{C}$ is a *universal cone*: a cone $(L, (\ell_j)_{j \in \mathcal{J}})$ such that any other cone $(C, (\phi_j)_j)$ over $D$ factors uniquely through $L$. That is: there exists a unique morphism $u : C \to L$ in $\mathcal{C}$ such that $\ell_j \circ u = \phi_j$ for all $j$.

The limit, when it exists, is unique up to unique isomorphism.

**Cocones and colimits.** A *cocone* over $D$ with vertex $C$ is a family of morphisms $(\psi_j : D(j) \to C)_j$ compatible with the diagram. A *colimit* is a universal cocone: a cocone $(K, (k_j)_j)$ through which every other cocone factors uniquely.

Limits and colimits are dual: a colimit in $\mathcal{C}$ is a limit in $\mathcal{C}^{op}$.

## Terminal and Initial Objects

The empty diagram (empty index category $\mathcal{J} = \emptyset$) has a special limit and colimit.

**Terminal object.** The limit of the empty diagram (if it exists) is a *terminal object* $1 \in \mathcal{C}$: an object such that for every $C \in \mathcal{C}$, there is exactly one morphism $C \to 1$. Terminal objects are unique up to unique isomorphism.

Examples: $\{*\}$ in $\mathbf{Set}$; the trivial group $\{e\}$ in $\mathbf{Grp}$; a one-point space in $\mathbf{Top}$; the unit type $\mathbf{1}$ in type theory.

**Initial object.** The colimit of the empty diagram is an *initial object* $0 \in \mathcal{C}$: an object with exactly one morphism to every other object. Dual to terminal objects.

Examples: $\emptyset$ in $\mathbf{Set}$; the trivial group in $\mathbf{Grp}$ (also a terminal object — groups have a "zero" object); the empty type $\mathbf{0}$ in type theory.

In type theory: the unit type $\mathbf{1}$ (with its unique term $\star$) is terminal because for any type $A$, there is exactly one function $A \to \mathbf{1}$ (sending everything to $\star$). The empty type $\mathbf{0}$ is initial because for any type $A$, the function $\mathbf{0} \to A$ exists (by the eliminator for $\mathbf{0}$) and is unique.

## Products and Coproducts

**Products.** Let $\mathcal{J} = \{*, *'\}$ be the discrete two-object category (no non-identity morphisms). A diagram $D : \mathcal{J} \to \mathcal{C}$ is just a pair $(A, B)$ of objects. A cone over $(A, B)$ with vertex $C$ is a pair of morphisms $(\phi : C \to A, \psi : C \to B)$.

The limit of $(A, B)$ — when it exists — is the *product* $A \times B$, with *projections* $\pi_1 : A \times B \to A$ and $\pi_2 : A \times B \to B$ such that every pair $(\phi : C \to A, \psi : C \to B)$ factors uniquely: there exists a unique $\langle \phi, \psi \rangle : C \to A \times B$ with $\pi_1 \circ \langle \phi, \psi \rangle = \phi$ and $\pi_2 \circ \langle \phi, \psi \rangle = \psi$.

This is the universal property of the product: a map *into* $A \times B$ is the same as a *pair* of maps — one into $A$, one into $B$.

In type theory: the product type $A \times B$ (or more generally $A \mathbin{\times} B$ for non-dependent pair) satisfies exactly this universal property.

**Coproducts.** The dual: the colimit of $(A, B)$ is the *coproduct* $A + B$, with *injections* $\iota_1 : A \to A + B$ and $\iota_2 : B \to A + B$ such that every pair $(\phi : A \to C, \psi : B \to C)$ factors uniquely through $A + B$.

A map *out of* $A + B$ is the same as a *pair* of maps — one out of $A$, one out of $B$. This is case analysis.

In type theory: the sum type $A + B$ satisfies this universal property. The eliminator for $A + B$ is exactly the factoring: to build a function $A + B \to C$, you provide a function $A \to C$ and a function $B \to C$.

## Equalizers and Coequalizers

**Equalizers.** Let $\mathcal{J} = \{\bullet \rightrightarrows \bullet\}$ (two objects, two parallel morphisms). A diagram is a pair of parallel morphisms $f, g : A \rightrightarrows B$. The limit — the *equalizer* — is an object $E$ with $e : E \to A$ such that $f \circ e = g \circ e$, and with the universal property that any $h : C \to A$ with $f \circ h = g \circ h$ factors uniquely through $E$.

The equalizer "picks out the subobject of $A$ where $f$ and $g$ agree."

In $\mathbf{Set}$: the equalizer of $f, g : A \to B$ is the subset $\{a \in A : f(a) = g(a)\}$.

In type theory: the equalizer of $f, g : A \to B$ is the type $\{a : A \mid f(a) = g(a)\} = \sum_{a:A} (f(a) = g(a))$ — the subtype of elements equalized by $f$ and $g$.

**Coequalizers.** Dual: the *coequalizer* of $f, g : A \rightrightarrows B$ is the object $Q$ with $q : B \to Q$ such that $q \circ f = q \circ g$. It is the "quotient" of $B$ that identifies $f(a)$ and $g(a)$ for all $a \in A$.

Coequalizers are how category theory formalizes quotient constructions.

## Pullbacks and Pushouts

**Pullbacks.** Let $\mathcal{J}$ be the shape $\{A \to C \leftarrow B\}$ (a cospan). A pullback is the limit of this diagram: an object $P$ with maps $p_1 : P \to A$ and $p_2 : P \to B$ such that $f \circ p_1 = g \circ p_2$ (the square commutes), universal among all such squares.

In $\mathbf{Set}$: the pullback of $f : A \to C$ and $g : B \to C$ is $\{(a, b) \in A \times B : f(a) = g(b)\}$ — the "fiber product" of $A$ and $B$ over $C$.

Pullbacks are ubiquitous in mathematics:
- In algebra: fiber products of rings, groups, modules
- In topology: a homotopy pullback (in the homotopy-theoretic sense)
- In type theory: the dependent sum $\sum_{a:A} (f(a) = g(b))$ for varying $b$; and more fundamentally, *substitution* in context

**Substitution as pullback.** In the categorical semantics of type theory, a type $B$ in context $\Gamma, x:A$ is modeled as a morphism $B \to A$ in the slice category $\mathcal{C}/\Gamma$. Substituting a term $a : A$ for $x$ in $B$ is the pullback of $B \to A$ along $a : \Gamma \to A$. This is the fundamental operation connecting type theory to category theory.

**Pushouts.** The colimit of a span $\{A \leftarrow C \to B\}$: an object $P$ with maps $i_1 : A \to P$ and $i_2 : B \to P$ such that $i_1 \circ f = i_2 \circ g$.

In $\mathbf{Set}$: the pushout is the disjoint union $A \sqcup B$ with the equivalence relation generated by $f(c) \sim g(c)$ for all $c \in C$.

In topology: the pushout of $A \leftarrow C \to B$ is the space obtained by gluing $A$ and $B$ together along $C$.

In type theory: pushouts are **higher inductive types** — types with both point constructors and path constructors. The pushout of $f : C \to A$ and $g : C \to B$ is the type with constructors $\mathsf{inl} : A \to P$, $\mathsf{inr} : B \to P$, and $\mathsf{glue} : \prod_{c:C} (\mathsf{inl}(f(c)) = \mathsf{inr}(g(c)))$. This is one of the most important HITs in HoTT.

## Limits in Type Theory

The categorical notions of limit appear throughout type theory:

| Categorical Limit | Type Theory |
|---|---|
| Terminal object | Unit type $\mathbf{1}$ |
| Product $A \times B$ | Pair type $A \times B$ |
| Equalizer of $f, g : A \to B$ | $\sum_{a:A} (f(a) = g(a))$ |
| Pullback of $f : A \to C$, $g : B \to C$ | $\sum_{a:A} \sum_{b:B} (f(a) = g(b))$ |
| Limit of a diagram | Dependent function type (in the right context) |

And colimits:

| Categorical Colimit | Type Theory |
|---|---|
| Initial object | Empty type $\mathbf{0}$ |
| Coproduct $A + B$ | Sum type $A + B$ |
| Coequalizer | Quotient type (requires HIT) |
| Pushout | Pushout HIT |
| Colimit of a diagram | HIT with path constructors |

The colimits — especially coequalizers and pushouts — require higher inductive types. This is why HITs are necessary for HoTT to have all homotopy-theoretic constructions: classical type theory can do limits, but colimits require the extra path constructor machinery.

## Preservation of Limits and Colimits

The fundamental theorem relating adjunctions to limits and colimits:

**Theorem.** Right adjoints preserve limits. Left adjoints preserve colimits.

This has many corollaries:
- The forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ (right adjoint to free) preserves products: the product of groups has the same underlying set as the product of the underlying sets.
- The free functor $\mathbf{Set} \to \mathbf{Grp}$ (left adjoint to forgetful) preserves coproducts: $F(S \sqcup T) \cong F(S) * F(T)$ (free product).
- In type theory: the product functor $(-) \times A$ (left adjoint to $[A, -]$) preserves coproducts: $(B + C) \times A \simeq (B \times A) + (C \times A)$.
- In HoTT: propositional truncation $\|-\|$ (left adjoint to inclusion of propositions) preserves coproducts: $\|A + B\| \simeq \|A\| \vee \|B\|$ (where $\vee$ is disjunction).

These are not arbitrary facts — they are all instances of the single theorem that adjoints preserve (co)limits.

## Complete and Cocomplete Categories

A category is *complete* if all small limits exist; *cocomplete* if all small colimits exist. Categories that are both are called *bicomplete*.

Examples:
- $\mathbf{Set}$ is bicomplete
- $\mathbf{Grp}$, $\mathbf{Top}$, $\mathbf{Vect}_k$ are bicomplete
- The category of types in HoTT is bicomplete (assuming univalence and HITs)

Bicompleteness is a strong condition: it means you can take all limits and colimits, do all universal constructions, and the result always exists.

A fundamental theorem: a locally small, complete, well-powered category with a *generating set* of objects is also cocomplete. This is the *adjoint functor theorem* in disguise — it says that completeness and certain smallness conditions force the existence of left adjoints, and left adjoints produce colimits.

In the ∞-categorical setting, the analogous notion is an *∞-topos*, which is a certain kind of bicomplete ∞-category. HoTT is designed to be the internal language of ∞-toposes — and the existence of all HITs corresponds to the cocompleteness of the ∞-topos.
