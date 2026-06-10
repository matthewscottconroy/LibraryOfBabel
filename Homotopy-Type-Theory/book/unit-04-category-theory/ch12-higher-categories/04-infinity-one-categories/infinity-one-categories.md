# (∞,1)-Categories

## Beyond Groupoids: Directed Higher Structure

∞-Groupoids capture homotopy types: spaces where all morphisms are invertible. But many mathematical structures involve *directed* morphisms — morphisms that are not invertible, like functions between types, or morphisms in a category.

An *(∞,1)-category* is the correct generalization: an ∞-category where all $k$-morphisms for $k \geq 2$ are invertible, but 1-morphisms may not be. This is the homotopy-coherent version of an ordinary category.

The key examples:
- The (∞,1)-category of ∞-groupoids (Kan complexes and morphisms between them)
- The (∞,1)-category of chain complexes and quasi-isomorphisms
- The (∞,1)-category of ring spectra
- The (∞,1)-category of topological spaces and continuous maps

These cannot be modeled as ordinary categories because the natural notion of "equality" between morphisms is *homotopy* (not strict equality), and composition is only *associative up to homotopy*.

## The Problem: Homotopy-Coherent Composition

In an ordinary category, composition is strict: $(h \circ g) \circ f = h \circ (g \circ f)$. In an (∞,1)-category, composition is homotopy-coherent: there is a specified 2-morphism (homotopy) $\alpha_{h,g,f} : (h \circ g) \circ f \to h \circ (g \circ f)$, and this 2-morphism satisfies a coherence condition (the pentagon axiom), and the pentagon axiom is satisfied up to a 3-morphism, and so on.

Specifying all this data explicitly — giving the composition, all associators, all higher coherences, all coherences between coherences — is extraordinarily complex. The simplicial approach avoids this by encoding all the coherence data implicitly.

## Quasi-Categories: The Simplicial Model

**Definition.** A *quasi-category* (or *∞-category*, following Lurie) is a simplicial set $\mathcal{C}$ satisfying the *inner horn filling condition*: for every $0 < k < n$ and every map $\Lambda^k[n] \to \mathcal{C}$ (a map from the $k$-th horn of the standard $n$-simplex), there exists an extension to $\Delta[n] \to \mathcal{C}$.

**Unpacking:**
- A simplicial set $\mathcal{C}$ consists of sets $\mathcal{C}_n$ (the $n$-simplices) for each $n \geq 0$, with face maps $d_i : \mathcal{C}_n \to \mathcal{C}_{n-1}$ and degeneracy maps $s_i : \mathcal{C}_n \to \mathcal{C}_{n+1}$ satisfying the simplicial identities.
- The 0-simplices $\mathcal{C}_0$ are the "objects."
- The 1-simplices $\mathcal{C}_1$ are the "morphisms."
- The 2-simplices $\mathcal{C}_2$ are "homotopies" (witnessing that two morphisms are homotopic).
- The inner horn $\Lambda^k[n]$ is the standard $n$-simplex with the $k$-th face and the interior removed.

**What inner horn filling says:**
- The $\Lambda^1[2]$ horn: given $f : a \to b$ and $g : b \to c$, there exists a 2-simplex with faces $g \circ f$ (on the outer face), $f$, and $g$. This witnesses that $g \circ f$ is a *composite* of $f$ and $g$.
- The $\Lambda^2[2]$ horn: similar, giving the composite in the other direction.
- Higher horns: give coherence data for associativity, etc.

The inner horn filling condition ensures that composition is defined (up to homotopy) and satisfies all coherence conditions implicitly. No explicit coherence data is needed.

**Note:** Kan complexes satisfy all horn filling conditions (inner and outer). Quasi-categories only require the *inner* horn condition. The outer horns ($\Lambda^0[n]$ and $\Lambda^n[n]$) are not required to fill, which allows for non-invertible morphisms.

## The Relationship to Kan Complexes

Kan complexes are the ∞-groupoids; quasi-categories are the (∞,1)-categories. The inclusion:

$$\text{Kan complexes} \subset \text{Quasi-categories}$$

reflects the inclusion of ∞-groupoids into (∞,1)-categories. Every Kan complex is a quasi-category, and a quasi-category is a Kan complex iff all its 1-simplices are invertible up to homotopy.

**Key theorem (Joyal).** The category of quasi-categories is a *model category*, and the weak equivalences are the *categorical equivalences* (fully faithful and essentially surjective functors of quasi-categories). This model structure is called the *Joyal model structure*.

## The (∞,1)-Category of ∞-Groupoids

The most important (∞,1)-category for HoTT is the (∞,1)-category of ∞-groupoids, $\mathcal{S}$ (also called the "∞-category of spaces").

**Objects:** Kan complexes (∞-groupoids / homotopy types)
**Morphisms:** Simplicial maps between Kan complexes
**2-Morphisms:** Homotopies between simplicial maps
**And so on...**

$\mathcal{S}$ is itself a quasi-category (the coherent nerve of the simplicial category of Kan complexes). It is the "base" ∞-category for homotopy theory: all other homotopy-theoretic structures are built over $\mathcal{S}$.

**The ∞-categorical Yoneda lemma.** For any quasi-category $\mathcal{C}$ and any object $X \in \mathcal{C}$, the "representable" functor $\mathsf{Hom}(X, -) : \mathcal{C} \to \mathcal{S}$ satisfies the Yoneda lemma: $\mathsf{Nat}(\mathsf{Hom}(X, -), F) \simeq F(X)$ (a natural equivalence of ∞-groupoids). The Yoneda lemma holds at the ∞-categorical level.

In HoTT, the Yoneda lemma corresponds to path induction: the type $\prod_{x:A} (a = x) \to B(x)$ is equivalent to $B(a)$ (by sending $f$ to $f(a, \mathsf{refl})$). This is the type-theoretic Yoneda lemma.

## (∞,1)-Functors and Natural Transformations

A *functor* between quasi-categories $F : \mathcal{C} \to \mathcal{D}$ is a simplicial map: it sends $n$-simplices of $\mathcal{C}$ to $n$-simplices of $\mathcal{D}$, preserving the simplicial structure. Composition of simplicial maps gives composition of functors.

A *natural transformation* $\alpha : F \Rightarrow G$ (for functors $F, G : \mathcal{C} \to \mathcal{D}$) is a simplicial map $H : \mathcal{C} \times \Delta[1] \to \mathcal{D}$ extending $F$ and $G$. (Here $\Delta[1]$ is the standard 1-simplex, the "interval".) This is the homotopy-coherent version of a natural transformation.

Natural transformations between functors form an ∞-groupoid (the Kan complex of natural transformations). This is the morphism space in the functor quasi-category $\mathsf{Fun}(\mathcal{C}, \mathcal{D})$.

## ∞-Limits and ∞-Colimits

An (∞,1)-category $\mathcal{C}$ has *∞-limits* and *∞-colimits* if certain extension problems can be solved.

**∞-limit:** An ∞-limit of a diagram $D : \mathcal{J} \to \mathcal{C}$ is a terminal object in the ∞-category of *cones* over $D$.

**∞-colimit:** Dual: an initial object in the ∞-category of *cocones* under $D$.

In $\mathcal{S}$ (the ∞-category of Kan complexes):
- Products are Cartesian products of Kan complexes
- Coproducts are disjoint unions
- Pullbacks are homotopy pullbacks
- Pushouts are homotopy pushouts (which model HITs in HoTT)

The key difference from 1-categorical limits: ∞-limits are invariant under equivalence. A diagram that is equivalent to another gives the same ∞-limit. This is the "homotopy invariance" of ∞-categorical constructions.

In HoTT, all constructions are automatically homotopy-invariant (because equivalences are equalities, by univalence). This is why HoTT is the "internal language" of ∞-toposes: HoTT-provable statements are automatically homotopy-invariant.

## (∞,1)-Toposes

An *(∞,1)-topos* (Lurie) is an (∞,1)-category satisfying:
1. It is presentable (generated by compact objects under colimits)
2. Colimits are universal (stable under pullback)
3. It has "enough" object classifiers

The prototypical example: the ∞-category $\mathcal{S}$ of Kan complexes (∞-groupoids). More generally, any ∞-category of sheaves on a site.

The connection to HoTT: the Univalence Axiom (in HoTT) says the universe $\mathcal{U}$ is an object classifier in the ∞-topos. This is exactly the property that makes $\mathcal{S}$ an ∞-topos: the object classifier in $\mathcal{S}$ is the "∞-groupoid of small ∞-groupoids."

In HoTT: the universe $\mathcal{U}$ (the type of all small types) plays the role of the object classifier. Univalence says the "path space" of $\mathcal{U}$ is the space of equivalences. This is the statement that $\mathcal{U}$ classifies objects (types) in the ∞-topos sense.
