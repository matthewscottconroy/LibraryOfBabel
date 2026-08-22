# Categories and Functors

## The Definition

A *category* is almost nothing: objects, arrows, and a rule for composing arrows. The power is in what you can do with this almost-nothing.

**Definition.** A *category* $\mathcal{C}$ consists of:
- A collection $\mathsf{Ob}(\mathcal{C})$ of *objects*
- For each pair $(A, B)$ of objects, a set $\mathsf{Hom}_{\mathcal{C}}(A, B)$ of *morphisms* from $A$ to $B$, written $f : A \to B$
- For each triple $(A, B, C)$, a *composition* map $\mathsf{Hom}(A,B) \times \mathsf{Hom}(B,C) \to \mathsf{Hom}(A,C)$, written $(f, g) \mapsto g \circ f$
- For each object $A$, an *identity morphism* $\mathsf{id}_A : A \to A$

satisfying:
- **Associativity:** $(h \circ g) \circ f = h \circ (g \circ f)$ for all composable $f, g, h$
- **Left unit:** $\mathsf{id}_B \circ f = f$ for all $f : A \to B$
- **Right unit:** $f \circ \mathsf{id}_A = f$ for all $f : A \to B$

The definition is small. Three pieces of data, three axioms. But the abstraction is doing real work: it strips away everything about the *nature* of the objects and retains only the *compositional structure* of the maps between them.

## Size and Set-Theoretic Subtleties

A category is *small* if both its collection of objects and all its hom-sets are sets (as opposed to proper classes). It is *locally small* if all hom-sets $\mathsf{Hom}(A, B)$ are sets, even if the collection of objects is a proper class.

Most naturally occurring categories — $\mathbf{Set}$, $\mathbf{Grp}$, $\mathbf{Top}$ — are locally small but not small. The category $\mathbf{Set}$ has all sets as objects; the collection of all sets is not itself a set (Russell's paradox). But for any two sets $A$ and $B$, the set of functions $A \to B$ is a perfectly good set.

In HoTT, the "type of all types" lives in a universe $\mathcal{U}$, which itself lives in a higher universe $\mathcal{U}_1$, and so on. This universe hierarchy is the type-theoretic solution to the same size problem.

## Examples from Mathematics

The power of the categorical framework is the universality of its examples. The same formal structure appears in radically different mathematical domains.

**$\mathbf{Set}$:** Objects are sets; morphisms are functions; composition is function composition; identity is the identity function. This is the paradigmatic example. Every other example is meant to generalize or refine this one.

**$\mathbf{Grp}$:** Objects are groups; morphisms are group homomorphisms. A group homomorphism $f : G \to H$ satisfies $f(g_1 \cdot g_2) = f(g_1) \cdot f(g_2)$. The categorical structure captures the structure-preserving nature of the maps.

**$\mathbf{Top}$:** Objects are topological spaces; morphisms are continuous maps. Continuity is the right notion of "structure-preserving" for topological spaces.

**$\mathbf{Vect}_k$:** Objects are vector spaces over a field $k$; morphisms are linear maps. This is where the original Eilenberg-Mac Lane question arose: what does it mean for the double-dual isomorphism to be "natural"?

**Preorders as categories.** A preorder $(P, \leq)$ gives a category: objects are elements; $\mathsf{Hom}(a, b) = \{*\}$ if $a \leq b$, else $\mathsf{Hom}(a, b) = \emptyset$. Transitivity gives composition; reflexivity gives identity. Every hom-set has at most one element. A morphism from $a$ to $b$ is a proof that $a \leq b$; composition is the proof of transitivity.

This example reveals something important: categories can encode *proof-relevant* reasoning. The category of a preorder has at most one proof between any two propositions. A more general category might have many distinct proofs — and the difference between these proofs might matter.

**Monoids as categories.** A monoid $(M, \cdot, e)$ gives a one-object category $\mathbf{B}M$: the single object is $\star$; $\mathsf{Hom}(\star, \star) = M$; composition is monoid multiplication; identity is $e$. A monoid is a category with one object. A group is a one-object category where every morphism is an isomorphism.

This is characteristic of categorical thinking: monoids and groups are not separate species — they are special cases of the same general concept.

**$\mathbf{Type}$ in MLTT.** Objects are types $A : \mathcal{U}$; morphisms from $A$ to $B$ are functions $f : A \to B$; composition is function composition $g \circ f = \lambda x. g(f(x))$; identity is $\lambda x. x$. Type theory forms a category. Every dependent type theory gives a category of types and functions.

The subtlety: in MLTT, two functions are equal only if they are definitionally equal (or propositionally equal by a proof). The categorical structure of $\mathbf{Type}$ depends on which notion of equality you use. This is one reason why the categorical semantics of type theory is more delicate than the categorical semantics of set-based mathematics.

**The opposite category.** For any category $\mathcal{C}$, the *opposite category* $\mathcal{C}^{op}$ has the same objects but reversed morphisms: $\mathsf{Hom}_{\mathcal{C}^{op}}(A, B) = \mathsf{Hom}_{\mathcal{C}}(B, A)$. Composition reverses: $f \circ_{op} g = g \circ f$.

The opposite category underlies the *duality principle* of category theory: every theorem has a dual obtained by reversing all arrows. The dual of "product" is "coproduct"; the dual of "limit" is "colimit"; the dual of "left adjoint" is "right adjoint."

## Isomorphisms

**Definition.** A morphism $f : A \to B$ is an *isomorphism* if there exists $g : B \to A$ such that $g \circ f = \mathsf{id}_A$ and $f \circ g = \mathsf{id}_B$.

In $\mathbf{Set}$: isomorphisms are bijections. In $\mathbf{Grp}$: isomorphisms are group isomorphisms. In $\mathbf{Top}$: isomorphisms are homeomorphisms. In a preorder: $a$ and $b$ are isomorphic iff $a \leq b$ and $b \leq a$.

**Uniqueness of inverses.** If $g$ and $g'$ are both inverses of $f$, then $g = g'$. Proof: $g = g \circ \mathsf{id}_B = g \circ (f \circ g') = (g \circ f) \circ g' = \mathsf{id}_A \circ g' = g'$. The inverse, when it exists, is unique.

**Categorical principle:** Two objects $A$ and $B$ in a category are *the same* (categorically) if they are isomorphic. Anything expressible in categorical terms that holds for $A$ holds for $B$ and vice versa. This is the formal content of "isomorphic objects are indistinguishable by categorical means."

In HoTT, this becomes the Univalence Axiom: isomorphic types are equal. The categorical principle becomes a theorem of the type theory.

## Functors

A *functor* is a map between categories that preserves the categorical structure.

**Definition.** A *functor* $F : \mathcal{C} \to \mathcal{D}$ consists of:
- A function on objects: for each $A \in \mathsf{Ob}(\mathcal{C})$, an object $F(A) \in \mathsf{Ob}(\mathcal{D})$
- A function on morphisms: for each $f : A \to B$ in $\mathcal{C}$, a morphism $F(f) : F(A) \to F(B)$ in $\mathcal{D}$

satisfying:
- *Preservation of identities:* $F(\mathsf{id}_A) = \mathsf{id}_{F(A)}$ for all objects $A$
- *Preservation of composition:* $F(g \circ f) = F(g) \circ F(f)$ for all composable $f, g$

A functor sends objects to objects and morphisms to morphisms, respecting the structure. The two axioms say: functors don't destroy identities or break the composition law.

**Covariant vs. contravariant.** A functor as defined is *covariant*: it preserves the direction of arrows. A *contravariant functor* reverses arrows: $F(f) : F(B) \to F(A)$ for $f : A \to B$. Formally, a contravariant functor from $\mathcal{C}$ to $\mathcal{D}$ is a covariant functor from $\mathcal{C}^{op}$ to $\mathcal{D}$.

## Examples of Functors

**Forgetful functors.** The functor $U : \mathbf{Grp} \to \mathbf{Set}$ sends each group to its underlying set, and each group homomorphism to the underlying function of sets. It "forgets" the group structure. More generally, any map that forgets structure is a functor.

**Free functors.** The functor $F : \mathbf{Set} \to \mathbf{Grp}$ sends each set $S$ to the free group $F(S)$ generated by $S$, and each function $\phi : S \to T$ to the unique group homomorphism $F(\phi) : F(S) \to F(T)$ extending $\phi$ on generators. This functor "adds" free structure.

The forgetful and free functors are related by an adjunction (Section 4): $F \dashv U$.

**Fundamental group.** The functor $\pi_1 : \mathbf{Top}_* \to \mathbf{Grp}$ sends each pointed space $(X, x_0)$ to its fundamental group $\pi_1(X, x_0)$, and each based continuous map to the induced group homomorphism. Functoriality is the statement that $\pi_1(g \circ f) = \pi_1(g) \circ \pi_1(f)$: "passing to fundamental groups respects composition." This is how topology connects to algebra.

**The hom-functor.** For a fixed object $A \in \mathcal{C}$, the *representable functor* $\mathsf{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$ sends each object $B$ to the set $\mathsf{Hom}(A, B)$, and each morphism $f : B \to C$ to the postcomposition function $f_* : \mathsf{Hom}(A, B) \to \mathsf{Hom}(A, C)$, $g \mapsto f \circ g$.

Similarly, $\mathsf{Hom}(-, B) : \mathcal{C}^{op} \to \mathbf{Set}$ sends $A$ to $\mathsf{Hom}(A, B)$ and $f : A' \to A$ to precomposition $f^* : \mathsf{Hom}(A, B) \to \mathsf{Hom}(A', B)$.

**$\mathsf{ap}_f$ in type theory.** In MLTT, if $f : A \to B$ is a function and $p : a =_A b$ is a path, then $\mathsf{ap}_f(p) : f(a) =_B f(b)$ is a path in $B$. This is exactly the functor action of $f$ on the morphisms of the fundamental ∞-groupoid of $A$. Every function in MLTT is a functor between ∞-groupoids.

## The Category of Categories

Functors between categories compose in an obvious way: if $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{E}$, then $G \circ F : \mathcal{C} \to \mathcal{E}$ sends $A \mapsto G(F(A))$ and $f \mapsto G(F(f))$. Composition of functors is associative, and there is an identity functor $\mathsf{Id}_{\mathcal{C}} : \mathcal{C} \to \mathcal{C}$ sending everything to itself.

So categories and functors form a category $\mathbf{Cat}$. (There are set-theoretic subtleties here — $\mathbf{Cat}$ is not small — but this is the right intuition.)

What is an isomorphism in $\mathbf{Cat}$? It is a functor $F : \mathcal{C} \to \mathcal{D}$ that has an inverse functor $G : \mathcal{D} \to \mathcal{C}$ with $G \circ F = \mathsf{Id}_{\mathcal{C}}$ and $F \circ G = \mathsf{Id}_{\mathcal{D}}$. This is an *isomorphism of categories* — strict equality of composite functors.

But this is too strong. In practice, the right notion of "sameness" for categories is *equivalence* (defined in Section 2, via natural transformations). An equivalence of categories requires only that $G \circ F \cong \mathsf{Id}_{\mathcal{C}}$ and $F \circ G \cong \mathsf{Id}_{\mathcal{D}}$ — that the composites are naturally isomorphic to the identity, not literally equal. This is why natural transformations are essential.

## Fully Faithful and Essentially Surjective Functors

**Definition.** A functor $F : \mathcal{C} \to \mathcal{D}$ is:
- *Faithful* if $F$ is injective on hom-sets: if $F(f) = F(g)$ then $f = g$
- *Full* if $F$ is surjective on hom-sets: every $h : F(A) \to F(B)$ in $\mathcal{D}$ is $F(f)$ for some $f : A \to B$ in $\mathcal{C}$
- *Fully faithful* if it is both full and faithful — a bijection on each hom-set
- *Essentially surjective* if every $D \in \mathsf{Ob}(\mathcal{D})$ is isomorphic to $F(A)$ for some $A \in \mathsf{Ob}(\mathcal{C})$

A functor that is fully faithful and essentially surjective is an *equivalence of categories*. Equivalent categories are indistinguishable by categorical means — they have the same objects (up to isomorphism) and the same morphisms between them.

The Yoneda embedding (Section 2) will be fully faithful. This means: every category embeds faithfully into a functor category, and the category can be completely recovered from the functor category's restriction. This is one of the deepest consequences of the Yoneda lemma.

## Structure vs. Property

One consequence of the categorical perspective: isomorphism is a *structure* (a pair of inverse morphisms satisfying equations), not just a *property* (a proposition asserting existence of an inverse). Two objects might be isomorphic in many different ways, and these different isomorphisms can be distinct.

In HoTT, this distinction is formalized by the difference between $\Sigma$ types (existence with structure) and propositional truncations $\|-\|$ (mere existence). The type of isomorphisms $A \cong B$ is a $\Sigma$ type — it carries the explicit data of the inverse and the two identities. This is proof-relevant: different isomorphisms between $A$ and $B$ are different elements of $A \cong B$.

Univalence then says: the type of equalities $A = B$ (in the universe) is equivalent to the type of equivalences $A \simeq B$. Equalities between types carry the same data as equivalences — they are not merely propositions that an equivalence exists.
