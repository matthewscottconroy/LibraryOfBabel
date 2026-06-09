# Part VII — Category Theory

**Chapters 33–37**

---

## What This Part Establishes

Part VII introduces category theory: the language in which all of modern algebra — and much of mathematics — is naturally written. A category is an abstraction of "a collection of mathematical objects and the structure-preserving maps between them." The key insight of category theory is that the morphisms (maps) between objects carry as much or more information than the objects themselves, and that patterns recurring across all of mathematics (products, kernels, free objects, completions) can be captured and reasoned about at this level of abstraction.

Chapter 33 establishes the basic vocabulary: categories, special classes of morphisms (monomorphisms, epimorphisms, isomorphisms), and functors (structure-preserving maps between categories). Chapter 34 introduces natural transformations (maps between functors) and the Yoneda lemma, which shows that every mathematical object is completely determined by the maps into or out of it. Chapter 35 develops adjoint functors — the most pervasive categorical pattern, instantiated by free-forgetful pairs, tensor-hom pairs, and a hundred other examples. Chapter 36 establishes limits and colimits as the universal constructions that generalize products, kernels, pullbacks, and their duals. Chapter 37 introduces abelian categories, the categorical home of exact sequences and homological algebra.

By the end of Part VII, the reader has the categorical language needed for homological algebra (Part VIII), understands the universal properties underlying all constructions in algebra, and sees why category theory is not an abstraction for its own sake but a precision tool for organizing the content of Parts III–VI and beyond.

---

## Internal Dependency Map

```
Ch 33 (Categories, Functors)
         |
         v
Ch 34 (Natural Transformations, Yoneda)
         |
    _____|_____
    |         |
    v         v
 Ch 35      Ch 36
(Adjoints) (Limits/Colimits)
    |         |
    |_________|
         |
         v
      Ch 37
(Abelian Categories)
```

---

## Chapter 33 — Categories and Functors

**What it establishes:** The definition of a category and its basic vocabulary; the notion of a functor as a structure-preserving map between categories; and the first examples showing that the mathematical world is organized by categorical structure.

---

**33.1 Categories**

**33.1.1 Objects and Morphisms; Composition and Identity**
A *category* $\mathcal{C}$ consists of: a collection of *objects*$\mathrm{ob}(\mathcal{C})$; for each pair of objects$A, B$, a set$\mathrm{Hom}_{\mathcal{C}}(A, B)$ of *morphisms* from$A$ to$B$; a *composition law*$\circ: \mathrm{Hom}(B,C) \times \mathrm{Hom}(A,B) \to \mathrm{Hom}(A,C)$ for each triple$A, B, C$; and for each object$A$, an *identity morphism*$\mathrm{id}_A \in \mathrm{Hom}(A,A)$. Composition is associative and the identity morphisms satisfy the unit laws:$f \circ \mathrm{id}_A = f = \mathrm{id}_B \circ f$ for$f: A \to B$.

**33.1.2 The Category Axioms: Associativity and Unit Laws**
The category axioms encode precisely what it means for composition to be well-behaved: $(h \circ g) \circ f = h \circ (g \circ f)$ whenever the composites are defined;$\mathrm{id}_B \circ f = f$ and$f \circ \mathrm{id}_A = f$ for any$f: A \to B$. These two axioms are the category-theoretic analogues of the group axioms of associativity and identity. They ensure that proofs by "diagram chasing" — drawing commutative diagrams of morphisms — are consistent and well-defined.

**33.1.3 Small and Large Categories; the Size Issues**
A category is *small* if its objects form a set (rather than a proper class). Most naturally occurring categories — **Set**, **Grp**, **Ring** — are *large*: their objects (all sets, all groups, all rings) form a proper class. The size issue is genuinely important: allowing "the category of all categories" leads to paradoxes analogous to Russell's paradox. The resolution uses Grothendieck universes (inaccessible cardinals from set theory) or simply works in a metatheory that distinguishes sets from classes.

**33.1.4 Examples: **Set**, **Grp**, **Ring**, $R$-**Mod**, **Top**, Posets, Groups**
The main examples: **Set** (sets and functions); **Grp** (groups and group homomorphisms); **Ring** (unital rings and ring homomorphisms); $R$-**Mod** (left$R$-modules and$R$-linear maps); **Top** (topological spaces and continuous maps); **Vect**$_F$ (vector spaces over$F$ and linear maps). Less obvious examples: any poset$(P, \leq)$ is a category (objects = elements, morphisms = pairs$a \leq b$, at most one morphism between any two objects); any group$G$ is a category with one object and the elements of$G$ as morphisms (composition = group multiplication). These examples demonstrate that categories are not just a language for algebra but encompass all of mathematics.

**33.1.5 Opposite (Dual) Categories**
The *opposite category* $\mathcal{C}^{op}$ has the same objects as$\mathcal{C}$ but all morphisms reversed:$\mathrm{Hom}_{\mathcal{C}^{op}}(A,B) = \mathrm{Hom}_{\mathcal{C}}(B,A)$. Composition in$\mathcal{C}^{op}$ is the reverse of composition in$\mathcal{C}$. The *duality principle*: every theorem about categories has a dual theorem (obtained by replacing all morphisms with their reverses, turning products into coproducts, limits into colimits, etc.). Working in the opposite category is the categorical way of "reversing arrows."

---

**33.2 Special Morphisms**

**33.2.1 Monomorphisms (Generalized Injections)**
A morphism $f: A \to B$ is a *monomorphism* if it is left-cancellable:$f \circ g = f \circ h$ implies$g = h$ for any$g, h: C \to A$. In **Set**, monomorphisms = injections. In **Grp**, monomorphisms = injective group homomorphisms. In **Ring**, monomorphisms = injective ring homomorphisms. Monomorphisms are the categorical generalization of injectivity that works in any category — no reference to elements is needed.

**33.2.2 Epimorphisms (Generalized Surjections)**
A morphism $f: A \to B$ is an *epimorphism* if it is right-cancellable:$g \circ f = h \circ f$ implies$g = h$. In **Set**, epimorphisms = surjections. However, in **Ring**, the inclusion$\mathbb{Z} \hookrightarrow \mathbb{Q}$ is an epimorphism that is not surjective: any two ring maps$\mathbb{Q} \to S$ that agree on$\mathbb{Z}$ must agree everywhere (since every rational is a quotient of integers). This shows that categorical epimorphisms can differ from surjections in non-set-like categories — a distinction that matters for sheaf theory and localizations.

**33.2.3 Isomorphisms; Automorphism Groups**
A morphism $f: A \to B$ is an *isomorphism* if it has a two-sided inverse$g: B \to A$ with$g \circ f = \mathrm{id}_A$ and$f \circ g = \mathrm{id}_B$. Objects$A$ and$B$ are *isomorphic* if an isomorphism exists. The *automorphism group*$\mathrm{Aut}(A)$ of an object$A$ is the group of all isomorphisms$A \to A$ under composition. In **Grp**,$\mathrm{Aut}(A)$ is the usual automorphism group of the group$A$; in **Vect**,$\mathrm{Aut}(V) \cong GL(V)$.

**33.2.4 Examples Distinguishing Categorical from Set-Theoretic Notions**
The distinction between categorical and set-theoretic properties matters. In **Ring**: the inclusion $\mathbb{Z} \to \mathbb{Q}$ is an epimorphism but not surjective (as above). In the category of Hausdorff topological spaces: the inclusion of a dense subspace is an epimorphism. In general: monic + epic$\not\Rightarrow$ iso (in **Ring**, the inclusion$\mathbb{Z} \hookrightarrow \mathbb{Q}$ is both monic and epic but not an isomorphism). Isomorphisms are morphisms that are bijective in the categorical sense — having a two-sided inverse — not just injective and surjective in the set-theoretic sense.

---

**33.3 Functors**

**33.3.1 Covariant Functors: Objects to Objects, Morphisms to Morphisms**
A *covariant functor* $F: \mathcal{C} \to \mathcal{D}$ assigns to each object$A \in \mathcal{C}$ an object$F(A) \in \mathcal{D}$ and to each morphism$f: A \to B$ a morphism$F(f): F(A) \to F(B)$, preserving composition:$F(g \circ f) = F(g) \circ F(f)$ and$F(\mathrm{id}_A) = \mathrm{id}_{F(A)}$. A functor is a "structure-preserving map" between categories, analogous to a homomorphism between groups. Every time one passes from a mathematical structure to an associated structure (e.g., from a topological space to its fundamental group), one is defining a functor.

**33.3.2 Contravariant Functors: Arrow Reversal**
A *contravariant functor* $F: \mathcal{C} \to \mathcal{D}$ reverses arrows: it sends morphisms$f: A \to B$ to morphisms$F(f): F(B) \to F(A)$, and satisfies$F(g \circ f) = F(f) \circ F(g)$. Equivalently, a contravariant functor from$\mathcal{C}$ is a covariant functor from$\mathcal{C}^{op}$. The paradigm example: the contravariant hom-functor$\mathrm{Hom}(-, M): R\text{-Mod} \to \text{Ab}$, which sends a module$N$ to$\mathrm{Hom}_R(N, M)$ and a linear map$f: N' \to N$ to the "precompose with$f$" map$\mathrm{Hom}_R(N,M) \to \mathrm{Hom}_R(N',M)$.

**33.3.3 Full, Faithful, and Essentially Surjective Functors**
A functor $F: \mathcal{C} \to \mathcal{D}$ is: *full* if$F: \mathrm{Hom}(A,B) \to \mathrm{Hom}(F(A), F(B))$ is surjective for all$A, B$; *faithful* if these maps are all injective; *essentially surjective* if every$D \in \mathcal{D}$ is isomorphic to$F(C)$ for some$C \in \mathcal{C}$. A functor that is full, faithful, and essentially surjective is an *equivalence of categories* — the categorical notion of "the same structure" for categories. Equivalences are weaker than isomorphisms of categories (which require an exact inverse functor) but are the correct notion in practice.

**33.3.4 Examples: Forgetful, Free, Hom, Power Set, Fundamental Group**
*Forgetful functors*: **Grp** → **Set** (send a group to its underlying set, a homomorphism to the underlying function); similarly for rings, modules. *Free functors*: **Set** → **Grp** (send a set $S$ to the free group$F_S$, a function$f$ to the induced homomorphism). *Hom functors*:$\mathrm{Hom}(A, -)$ and$\mathrm{Hom}(-, A)$ for any fixed object$A$. *Power set*$\mathcal{P}$: **Set** → **Set** (either covariant via image or contravariant via preimage). *Fundamental group*$\pi_1$: pointed topological spaces → **Grp**. These examples show that functors are everywhere in mathematics.

**33.3.5 Compositions of Functors; the Category of Small Categories**
Functors can be composed: if $F: \mathcal{C} \to \mathcal{D}$ and$G: \mathcal{D} \to \mathcal{E}$ are functors, then$G \circ F: \mathcal{C} \to \mathcal{E}$ is a functor (apply$F$ then$G$). There is an identity functor$\mathrm{Id}_{\mathcal{C}}: \mathcal{C} \to \mathcal{C}$. The *category of small categories* **Cat** has small categories as objects and functors as morphisms — a "meta-category." In this category, natural transformations (Chapter 34) are the 2-morphisms, making **Cat** the first example of a *2-category*.

---

## Chapter 34 — Natural Transformations and the Yoneda Lemma

**What it establishes:** Natural transformations as the "morphisms between functors"; the Yoneda lemma as the deepest foundational result in category theory, showing that every object is completely determined by the maps into or out of it.

---

**34.1 Natural Transformations**

**34.1.1 Definition: Commuting Squares for Every Morphism**
A *natural transformation* $\eta: F \Rightarrow G$ between functors$F, G: \mathcal{C} \to \mathcal{D}$ is a family of morphisms$\eta_A: F(A) \to G(A)$ in$\mathcal{D}$, one for each object$A \in \mathcal{C}$, such that for every morphism$f: A \to B$ in$\mathcal{C}$, the *naturality square* commutes:$\eta_B \circ F(f) = G(f) \circ \eta_A$. This says the components$\eta_A$ "intertwine" the functors$F$ and$G$ in a coherent way — the transformation is compatible with all structure in$\mathcal{C}$.

**34.1.2 Natural Isomorphisms; Canonical vs. Non-Canonical Maps**
A natural transformation $\eta: F \Rightarrow G$ is a *natural isomorphism* if each component$\eta_A$ is an isomorphism in$\mathcal{D}$. Natural isomorphisms are the correct notion of "isomorphism between functors." The conceptual content: a *canonical* isomorphism between two objects is one that is natural — it does not depend on any arbitrary choice. The map$V \cong V^{**}$ (vector space$\cong$ double dual) is natural; the isomorphism$V \cong V^*$ is not (it requires a choice of inner product or basis).

**34.1.3 The Category $[\mathcal{C}, \mathcal{D}]$ of Functors**
For small $\mathcal{C}$ and any$\mathcal{D}$, the collection of all functors$\mathcal{C} \to \mathcal{D}$ with natural transformations as morphisms forms a *functor category*$[\mathcal{C}, \mathcal{D}]$ (or$\mathcal{D}^{\mathcal{C}}$). Composition of natural transformations is "vertical composition" (compose components). There is also "horizontal composition" of natural transformations, making **Cat** into a 2-category. Functor categories are pervasive: presheaves (Chapter 58) are functors$\mathcal{C}^{op} \to \mathbf{Set}$, forming a functor category.

**34.1.4 Examples: Double Dual, Determinant, Abelianization**
*Double dual:* the natural isomorphism $\mathrm{id}_{\mathbf{Vect}} \Rightarrow (-)^{**}$ (identity functor naturally isomorphic to double dual, canonically). *Determinant:* a natural transformation from$GL_n: \mathbf{CRing} \to \mathbf{Grp}$ (general linear group functor) to$(-)^*$ (units functor). *Abelianization:* the natural surjection$\mathrm{id}_{\mathbf{Grp}} \Rightarrow \mathrm{Ab}$ (group to its abelianization$G/[G,G]$) is a natural transformation. These examples show that naturality is a ubiquitous property of mathematical constructions.

---

**34.2 The Yoneda Lemma**

**34.2.1 Representable Functors and the Hom-Functor $\mathrm{h}^A$**
For a locally small category $\mathcal{C}$ and an object$A \in \mathcal{C}$, the *representable functor*$\mathrm{h}^A = \mathrm{Hom}_{\mathcal{C}}(A, -): \mathcal{C} \to \mathbf{Set}$ sends each object$B$ to the set$\mathrm{Hom}(A, B)$ and each morphism$f: B \to C$ to the post-composition map$f_*: \mathrm{Hom}(A,B) \to \mathrm{Hom}(A,C)$. A functor$F: \mathcal{C} \to \mathbf{Set}$ is *representable* if$F \cong \mathrm{h}^A$ for some$A$. Representable functors capture "the set of maps from$A$" and are the crucial link between abstract category theory and concrete mathematical objects.

**34.2.2 Statement: $\mathrm{Nat}(\mathrm{h}^A, F) \cong F(A)$, Naturally**
*The Yoneda Lemma:* For any locally small category $\mathcal{C}$, any object$A \in \mathcal{C}$, and any functor$F: \mathcal{C} \to \mathbf{Set}$, there is a bijection
$$\mathrm{Nat}(\mathrm{h}^A, F) \cong F(A)$$
(natural transformations from $\mathrm{h}^A$ to$F$ are in bijection with elements of$F(A)$), and this bijection is natural in both$A$ and$F$. In words: a natural transformation$\mathrm{h}^A \Rightarrow F$ is completely determined by where it sends the identity morphism$\mathrm{id}_A \in \mathrm{Hom}(A, A) = \mathrm{h}^A(A)$.

**34.2.3 Proof of the Yoneda Lemma**
The Yoneda bijection sends a natural transformation $\eta: \mathrm{h}^A \Rightarrow F$ to the element$\eta_A(\mathrm{id}_A) \in F(A)$. The inverse sends$x \in F(A)$ to the natural transformation$\eta^x: \mathrm{h}^A \Rightarrow F$ with$\eta^x_B(f) = F(f)(x)$ for$f: A \to B$. Naturality of$\eta^x$ (the commutativity of the naturality squares) follows from the definition of$F$ as a functor. This is one of the purest proofs in mathematics: the proof writes itself from the definitions, with no choices to make.

**34.2.4 The Yoneda Embedding: $\mathcal{C} \hookrightarrow [\mathcal{C}^{op}, \mathbf{Set}]$**
The *Yoneda embedding* is the functor $Y: \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ sending$A$ to$\mathrm{h}_A = \mathrm{Hom}(-, A)$ (the contravariant representable functor). The Yoneda lemma implies$Y$ is fully faithful:$\mathrm{Hom}_{\mathcal{C}}(A, B) \cong \mathrm{Nat}(\mathrm{h}_B, \mathrm{h}_A)$ naturally. The Yoneda embedding identifies$\mathcal{C}$ with a full subcategory of the *presheaf category*$[\mathcal{C}^{op}, \mathbf{Set}]$. This is the categorical version of Cayley's theorem: every category embeds into a category of set-valued functors.

**34.2.5 Consequences: Objects Determined by Maps into/out of Them**
The Yoneda lemma's philosophical content: an object $A$ is completely determined (up to isomorphism) by the functor$\mathrm{Hom}(A, -)$ or$\mathrm{Hom}(-, A)$ — by all the maps into or out of$A$. Two objects are isomorphic iff they represent the same functor. *Universal properties* (free objects, tensor products, limits) are expressed as representability of functors: an object$T$ "is" the tensor product iff$\mathrm{Hom}(T, -) \cong$ bilinear maps from$M \times N$ to$(-)$.

---

## Chapter 35 — Adjoint Functors

**What it establishes:** The adjunction — the most pervasive categorical pattern — and its ubiquitous instantiations across algebra, topology, and logic; the key properties that right adjoints preserve limits and left adjoints preserve colimits.

---

**35.1 Definition and Examples**

**35.1.1 The Hom-Set Bijection: $\mathrm{Hom}(FA, B) \cong \mathrm{Hom}(A, GB)$, Naturally**
Functors $F: \mathcal{C} \to \mathcal{D}$ and$G: \mathcal{D} \to \mathcal{C}$ form an *adjunction* (written$F \dashv G$, "$F$ is left adjoint to$G$") if there is a natural bijection$\mathrm{Hom}_{\mathcal{D}}(FA, B) \cong \mathrm{Hom}_{\mathcal{C}}(A, GB)$ for all$A \in \mathcal{C}$,$B \in \mathcal{D}$. Naturality means the bijection is compatible with all morphisms in both$A$ and$B$. The functor$F$ is the *left adjoint* and$G$ is the *right adjoint*.

**35.1.2 Left and Right Adjoints; Notation $F \dashv G$**
The bijection $\mathrm{Hom}(FA, B) \cong \mathrm{Hom}(A, GB)$ has a direction: maps$FA \to B$ (in$\mathcal{D}$) correspond to maps$A \to GB$ (in$\mathcal{C}$). The left adjoint$F$ "does the work" of constructing objects in$\mathcal{D}$ from objects in$\mathcal{C}$, while the right adjoint$G$ "forgets structure" by taking$\mathcal{D}$-objects to$\mathcal{C}$-objects. This asymmetry — left adjoints build, right adjoints forget — is the intuition behind the free-forgetful paradigm.

**35.1.3 Unit and Counit of an Adjunction**
The adjunction $F \dashv G$ is equivalently specified by two natural transformations: the *unit*$\eta: \mathrm{id}_{\mathcal{C}} \Rightarrow GF$ (corresponding to the identity map$FA \to FA$ under the bijection) and the *counit*$\varepsilon: FG \Rightarrow \mathrm{id}_{\mathcal{D}}$ (corresponding to$B \to B$), satisfying the *triangle identities*:$(G\varepsilon) \circ (\eta G) = \mathrm{id}_G$ and$(\varepsilon F) \circ (F\eta) = \mathrm{id}_F$. The unit is the "canonical map of$A$ into$GFA$" (the element$A$ maps into the free$G$-structure generated by it), and the counit is the "canonical evaluation map$FGB \to B$."

**35.1.4 Examples: Free–Forgetful, Tensor–Hom, Abelianization–Inclusion**
*Free–Forgetful:* $F_{\text{free}}: \mathbf{Set} \to \mathbf{Grp}$ (free group) is left adjoint to the forgetful functor$U: \mathbf{Grp} \to \mathbf{Set}$: homomorphisms$F(S) \to G$ correspond to functions$S \to U(G)$. *Tensor–Hom:*$- \otimes_R N \dashv \mathrm{Hom}_R(N, -)$ as shown in Chapter 28. *Abelianization:* the abelianization functor$\mathrm{Ab}: \mathbf{Grp} \to \mathbf{Ab}$ (sending$G \mapsto G/[G,G]$) is left adjoint to the inclusion$\mathbf{Ab} \hookrightarrow \mathbf{Grp}$. Every universal construction in algebra is an adjunction.

---

**35.2 Properties of Adjoints**

**35.2.1 Adjoints Are Unique up to Natural Isomorphism**
If $F \dashv G$ and$F \dashv G'$, then$G \cong G'$ naturally. Similarly, left adjoints are unique. This uniqueness means that universal properties — which express functors as adjoints — determine their values up to canonical isomorphism. The free group on a set, the tensor product of modules, the product of topological spaces — all are determined up to isomorphism by their universal property.

**35.2.2 Right Adjoints Preserve Limits; Left Adjoints Preserve Colimits**
*RAPL (Right Adjoints Preserve Limits):* If $G: \mathcal{D} \to \mathcal{C}$ is right adjoint to some functor, then$G$ preserves all limits that exist in$\mathcal{D}$. *LAPC (Left Adjoints Preserve Colimits):* If$F: \mathcal{C} \to \mathcal{D}$ is left adjoint to some functor, then$F$ preserves all colimits that exist in$\mathcal{C}$. These are among the most useful theorems in category theory. Examples: the forgetful functor preserves limits (products of groups, kernels, etc.); tensor product preserves colimits (right-exactness, coproducts); free group functor sends coproducts of sets to free products of groups.

**35.2.3 Reflective Subcategories; Localizations**
A subcategory $\mathcal{D} \hookrightarrow \mathcal{C}$ is *reflective* if the inclusion functor has a left adjoint$L: \mathcal{C} \to \mathcal{D}$ (the *reflector* or *localization functor*). The unit$\eta_A: A \to L(A)$ is the "best approximation of$A$ in$\mathcal{D}$" — the *reflection* of$A$ into$\mathcal{D}$. Examples: the abelianization$G \mapsto G/[G,G]$ is the reflector from **Grp** into **Ab**; the sheafification functor is the reflector from presheaves to sheaves; the category of fractions$S^{-1}\mathcal{C}$ is a reflective localization. Reflective subcategories are the categorical context for localizations in algebra.

**35.2.4 Adjoint Functor Theorems (Statement)**
*Freyd's Adjoint Functor Theorem:* A functor $G: \mathcal{D} \to \mathcal{C}$ has a left adjoint iff it preserves all limits and satisfies a "solution set condition" (a smallness condition). The *Special Adjoint Functor Theorem* removes the solution set condition under suitable completeness assumptions on$\mathcal{D}$. These theorems guarantee the existence of left adjoints (free objects, tensor products, etc.) without constructing them explicitly — reducing the question of existence to preservation of limits, which is often easy to check.

---

## Chapter 36 — Limits and Colimits

**What it establishes:** The universal constructions generalizing products, kernels, pullbacks, inverse limits, and their duals — coproducts, cokernels, pushouts, direct limits — as special cases of a single categorical concept.

---

**36.1 Diagrams and Cones**

**36.1.1 Diagrams as Functors $D: \mathcal{J} \to \mathcal{C}$**
A *diagram* of shape $\mathcal{J}$ in$\mathcal{C}$ is a functor$D: \mathcal{J} \to \mathcal{C}$, where$\mathcal{J}$ is a small "index category" encoding the shape of the diagram. For example: a diagram of shape$\bullet \rightrightarrows \bullet$ (two parallel morphisms) is a pair of parallel morphisms; a diagram of shape$\bullet \to \bullet \leftarrow \bullet$ is a pair of morphisms with a common codomain (the "span" shape for pullbacks). Every commutative diagram one draws is a functor from the underlying graph-category.

**36.1.2 Cones over a Diagram; the Category of Cones**
A *cone* over a diagram $D: \mathcal{J} \to \mathcal{C}$ with *apex*$N$ consists of a collection of morphisms$\psi_j: N \to D(j)$ (one for each object$j \in \mathcal{J}$) such that for every morphism$f: j \to k$ in$\mathcal{J}$, the triangle$N \xrightarrow{\psi_j} D(j) \xrightarrow{D(f)} D(k)$ equals$N \xrightarrow{\psi_k} D(k)$. A cone is a compatible family of maps from a single object to all objects in the diagram. The collection of all cones over$D$ forms a category.

**36.1.3 Limits as Terminal Cones; Universal Property**
A *limit* of a diagram $D: \mathcal{J} \to \mathcal{C}$ is a *terminal cone*: a cone$(\varprojlim D, \psi_j)$ such that for any other cone$(N, \phi_j)$, there exists a unique morphism$u: N \to \varprojlim D$ with$\psi_j \circ u = \phi_j$ for all$j$. Limits generalize all "best approximations from above" in a diagram. The universal property of a limit is exactly the statement that it is the terminal object in the category of cones, which by the Yoneda lemma determines it uniquely up to isomorphism.

---

**36.2 Specific Limits**

**36.2.1 Terminal Objects; Products; Equalizers**
*Terminal objects* are limits over the empty diagram: an object $T$ with a unique morphism$A \to T$ for all$A$ (in **Set**,$T$ is any one-element set; in **Grp**,$T$ is the trivial group). *Products* are limits of discrete diagrams (no non-identity morphisms):$A \times B$ with projections$\pi_A, \pi_B$ and the universal property that any pair of maps$f: C \to A$,$g: C \to B$ factors through$A \times B$. *Equalizers* are limits of pairs of parallel morphisms:$\mathrm{Eq}(f, g)$ is the subobject of$A$ where$f$ and$g$ agree.

**36.2.2 Pullbacks (Fibered Products); General Limits from Products and Equalizers**
The *pullback* (or *fibered product*) of $f: A \to C$ and$g: B \to C$ is the limit of the diagram$A \xrightarrow{f} C \xleftarrow{g} B$: it is$A \times_C B = \{(a,b) : f(a) = g(b)\}$ in **Set** (with the obvious maps). Pullbacks generalize fiber products in topology and algebraic geometry. *Key theorem:* Every limit can be constructed from products and equalizers, so a category with all products and equalizers has all small limits. This reduces "does the limit exist" to two questions.

**36.2.3 Inverse (Projective) Limits; $p$-adic Numbers as an Inverse Limit**
An *inverse (projective) limit* $\varprojlim_i A_i$ is the limit of a diagram indexed by a directed poset (ordered by$\geq$): a system of objects$A_i$ and morphisms$\phi_{ij}: A_j \to A_i$ for$i \leq j$. Elements of$\varprojlim A_i$ are "compatible sequences"$(a_i)_{i}$ with$\phi_{ij}(a_j) = a_i$. Example: the$p$-adic integers$\mathbb{Z}_p = \varprojlim \mathbb{Z}/p^n\mathbb{Z}$ is the inverse limit of the system$\cdots \to \mathbb{Z}/p^3 \to \mathbb{Z}/p^2 \to \mathbb{Z}/p$. Inverse limits are ubiquitous in topology (completions), algebraic geometry (formal completions), and number theory ($p$-adic numbers, profinite groups).

---

**36.3 Colimits**

**36.3.1 Cocones; Colimits as Initial Cocones**
A *colimit* is the dual of a limit: a *cocone* under a diagram $D: \mathcal{J} \to \mathcal{C}$ with apex$N$ consists of morphisms$\psi_j: D(j) \to N$ compatible with the diagram. A colimit is an *initial cocone* — a cocone through which every other cocone factors uniquely. Colimits generalize "best approximations from below" and include coproducts, pushouts, coequalizers, and direct limits.

**36.3.2 Initial Objects; Coproducts; Coequalizers; Pushouts**
*Initial objects* (colimits over the empty diagram): $\emptyset$ in **Set**, the trivial group$\{e\}$ in **Grp**, the zero ring in **Ring**. *Coproducts*: in **Set**,$A \sqcup B$ (disjoint union); in **Grp**, the free product$A * B$; in **Ab**,$A \oplus B$; in **Ring**,$A \otimes_{\mathbb{Z}} B$. *Coequalizers*: the quotient of$A$ by the equivalence relation generated by$f(a) \sim g(a)$ for a pair$f, g: B \to A$. *Pushouts*: dual to pullbacks; in **Grp**, the amalgamated free product$A *_C B$.

**36.3.3 Direct (Inductive) Limits; Algebraic Closure as a Colimit**
A *direct (inductive) limit* $\varinjlim_i A_i$ is the colimit of a directed system (objects$A_i$ with maps$\phi_{ij}: A_i \to A_j$ for$i \leq j$). In **Set**, it is the disjoint union modulo the equivalence relation generated by$\phi_{ij}$. The algebraic closure$\bar F$ of a field$F$ is the colimit of the directed system of all finite extensions of$F$:$\bar F = \varinjlim [E:F]<\infty$. Direct limits preserve exactness (in the abelian setting) and commute with tensor products.

**36.3.4 Filtered and Sifted Colimits**
A *filtered colimit* is a colimit over a filtered index category (one where any two objects have a common "future" object and any two parallel morphisms are equalized somewhere ahead). Filtered colimits in **Set** and **Ab** commute with finite limits — a fundamental exactness property. *Sifted colimits* generalize filtered colimits and directed colimits; they commute with finite products. These notions are essential for the theory of algebraic theories (Lawvere theories) and accessible categories.

---

**36.4 Preservation and Creation of Limits**

**36.4.1 Continuous and Cocontinuous Functors**
A functor $F: \mathcal{C} \to \mathcal{D}$ *preserves limits* (is *continuous*) if$F(\varprojlim D) \cong \varprojlim (F \circ D)$ for all small diagrams$D$. It *preserves colimits* (is *cocontinuous*) if$F(\varinjlim D) \cong \varinjlim (F \circ D)$. Preservation of specific limits (products, equalizers) is preservation of limits of specific shapes.

**36.4.2 Right Adjoints Are Continuous; Left Adjoints Are Cocontinuous**
As noted in Chapter 35: right adjoints preserve all limits (RAPL); left adjoints preserve all colimits (LAPC). This is one of the most useful theorems in category theory. *Examples:* the forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ is right adjoint to the free group functor, so it preserves limits: products, equalizers, and all limits in **Grp** are computed on underlying sets. The free functor (left adjoint) preserves colimits: the free group on a coproduct of sets is the free product of the free groups.

**36.4.3 Representable Functors Are Continuous**
For any object $A$ in a locally small category$\mathcal{C}$, the representable functor$\mathrm{Hom}(A, -): \mathcal{C} \to \mathbf{Set}$ preserves all limits:$\mathrm{Hom}(A, \varprojlim D_j) \cong \varprojlim \mathrm{Hom}(A, D_j)$. This is a consequence of the Yoneda lemma and the fact that$\mathrm{Hom}(A, -)$ is right adjoint to the "cotensor" functor (or directly by the universal property of limits). The preservation of products by$\mathrm{Hom}(A, -)$ is the statement that$\mathrm{Hom}(A, B \times C) \cong \mathrm{Hom}(A,B) \times \mathrm{Hom}(A,C)$.

---

## Chapter 37 — Abelian Categories

**What it establishes:** The axiomatics of abelian categories as the correct categorical home for homological algebra; exact sequences in the abstract setting; and the embedding theorems that justify "working element-wise" in any abelian category.

---

**37.1 Additive Categories**

**37.1.1 Enrichment over Abelian Groups: $\mathrm{Hom}(A,B)$ Is an Abelian Group**
An *additive category* is a category in which each hom-set $\mathrm{Hom}(A, B)$ carries the structure of an abelian group, composition is bilinear (distributive over addition), and the category has a zero object$0$ (an object that is simultaneously initial and terminal). Additive categories are "categories with addition of morphisms." Examples:$R$-**Mod**, **Ab**, the category of chain complexes. Non-examples: **Set**, **Grp** (hom-sets don't have a natural group structure).

**37.1.2 Biproducts: Products = Coproducts**
In an additive category, products and coproducts coincide: there are *biproducts* $A \oplus B$ with both projection morphisms$\pi_A, \pi_B$ and inclusion morphisms$\iota_A, \iota_B$ satisfying$\pi_A \circ \iota_A = \mathrm{id}_A$,$\pi_B \circ \iota_B = \mathrm{id}_B$,$\pi_A \circ \iota_B = 0$,$\pi_B \circ \iota_A = 0$, and$\iota_A \circ \pi_A + \iota_B \circ \pi_B = \mathrm{id}_{A \oplus B}$. In$R$-**Mod**, the biproduct is the direct sum$M \oplus N$. Biproducts make$A \oplus B$ simultaneously a product and a coproduct — a feature unique to additive categories.

**37.1.3 The Zero Object; Zero Morphisms**
The *zero object* $0$ is characterized by$\mathrm{Hom}(0, A) = \{*\}$ and$\mathrm{Hom}(A, 0) = \{*\}$ for all$A$ (it is both initial and terminal). For any$A, B$, the *zero morphism*$0_{AB}: A \to B$ is the composition$A \to 0 \to B$. In the abelian group structure on$\mathrm{Hom}(A,B)$,$0_{AB}$ is the additive identity. Zero morphisms give additive categories their additive flavor and are the morphism-level counterpart of the zero module.

---

**37.2 Abelian Categories**

**37.2.1 The Axioms: Kernels, Cokernels, and the Isomorphism Condition**
An *abelian category* is an additive category in which: (i) every morphism has a kernel and a cokernel; (ii) every monomorphism is a kernel (of its cokernel) and every epimorphism is a cokernel (of its kernel). These axioms make the "image" well-defined and force the first isomorphism theorem: every morphism $f: A \to B$ factors as$A \twoheadrightarrow \mathrm{im}(f) \hookrightarrow B$ where the first map is an epimorphism and the second a monomorphism. The abelian category axioms capture exactly what is needed for homological algebra.

**37.2.2 Images and Coimages; the Canonical Factorization**
The *image* of $f: A \to B$ is the kernel of the cokernel of$f$:$\mathrm{im}(f) = \ker(\mathrm{coker}(f))$. The *coimage* is the cokernel of the kernel:$\mathrm{coim}(f) = \mathrm{coker}(\ker(f))$. In an abelian category, the canonical map$\mathrm{coim}(f) \to \mathrm{im}(f)$ is always an isomorphism — this is one of the abelian category axioms. The factorization$A \to \mathrm{coim}(f) \cong \mathrm{im}(f) \to B$ is the first isomorphism theorem in this setting.

**37.2.3 Examples: **Ab**, $R$-**Mod**, Sheaves of Abelian Groups**
The category **Ab** of abelian groups is abelian. The category $R$-**Mod** of left modules over any ring$R$ is abelian (kernels and cokernels exist; the abelian category axioms are all verified). The category$\mathrm{Sh}(X, \mathbf{Ab})$ of sheaves of abelian groups on a topological space$X$ is abelian. These are the main examples in which homological algebra (Part VIII) is developed. Non-examples: **Top** (topological spaces) and **Grp** (groups) are not abelian (the isomorphism condition fails).

**37.2.4 The Freyd–Mitchell Embedding Theorem**
*Freyd–Mitchell Theorem:* Every small abelian category embeds fully and faithfully into $R$-**Mod** for some ring$R$, and this embedding preserves exact sequences. This means that in any abelian category, one can argue as if working with modules over a ring: "elements" of objects, "pointwise" computations. The theorem justifies the standard practice of proving categorical statements about abelian categories by "element-chasing" in modules, then invoking the embedding to transfer to the general setting.

---

**37.3 Exact Sequences in Abelian Categories**

**37.3.1 Exact Sequences; Exactness at an Object**
A sequence of morphisms $A \xrightarrow{f} B \xrightarrow{g} C$ is *exact at$B$* if$\mathrm{im}(f) = \ker(g)$. A sequence$A_0 \to A_1 \to A_2 \to \cdots$ is *exact* if it is exact at every object (except possibly the first and last). In$R$-**Mod**, exactness at$B$ means: every element of$\ker(g)$ is the image of some element of$A$ under$f$. Exact sequences are the fundamental notion of homological algebra; they encode and measure the "failure" of maps to be bijective.

**37.3.2 Short Exact Sequences and Split Sequences**
A *short exact sequence* $0 \to A \xrightarrow{f} B \xrightarrow{g} C \to 0$ in an abelian category means:$f$ is a monomorphism,$g$ is an epimorphism, and$\mathrm{im}(f) = \ker(g)$. Equivalently,$A$ embeds in$B$ and$C \cong B/A$. The sequence *splits* if$B \cong A \oplus C$ — the embedding of$A$ into$B$ has a retraction. In$R$-**Mod**, short exact sequences classify extensions of$C$ by$A$; the Ext group$\mathrm{Ext}^1(C, A)$ classifies all short exact sequences up to isomorphism.

**37.3.3 Left Exact and Right Exact Functors**
A functor $F$ between abelian categories is *left exact* if it preserves short exact sequences at the left:$0 \to A \to B \to C \to 0$ exact$\Rightarrow$ $0 \to F(A) \to F(B) \to F(C)$ exact. It is *right exact* if it preserves at the right:$F(A) \to F(B) \to F(C) \to 0$ exact. An *exact* functor preserves all short exact sequences. The functors$\mathrm{Hom}(M, -)$ and$\mathrm{Hom}(-, M)$ are left exact;$- \otimes M$ is right exact. These failures of exactness motivate the derived functors Ext and Tor (Chapter 40).

**37.3.4 The Five Lemma and the Snake Lemma**
The *Five Lemma*: given a commutative diagram with exact rows and four of the five vertical maps being isomorphisms, the fifth is also an isomorphism. The *Snake Lemma*: given a commutative diagram with exact rows, there is a natural connecting homomorphism $\partial: \ker(c) \to \mathrm{coker}(a)$ and the sequence$0 \to \ker(a) \to \ker(b) \to \ker(c) \xrightarrow{\partial} \mathrm{coker}(a) \to \mathrm{coker}(b) \to \mathrm{coker}(c) \to 0$ is exact. The Snake Lemma is the prototype for all long exact sequences in homological algebra and is proved by (justified) "diagram chasing."

---

*Next: [Part VIII — Homological Algebra](part-VIII-homological-algebra.md)*

*Prerequisites satisfied: Part I (logic, sets), Part II (linear algebra, especially exact sequences), Part III (groups, normal subgroups, quotients), Part IV (ring theory, ideals), Part V (modules — the primary examples), Parts III–VI provide the examples; this part provides the language.*
