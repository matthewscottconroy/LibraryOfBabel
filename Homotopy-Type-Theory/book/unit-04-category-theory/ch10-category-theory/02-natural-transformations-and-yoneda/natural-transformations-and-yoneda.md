# Natural Transformations and the Yoneda Lemma

## Natural Transformations: Maps Between Functors

Once you have functors, you naturally ask: what are the maps between functors? The answer — natural transformations — is why Eilenberg and Mac Lane needed category theory in the first place.

**Definition.** Let $F, G : \mathcal{C} \to \mathcal{D}$ be functors. A *natural transformation* $\alpha : F \Rightarrow G$ consists of:
- For each object $A \in \mathcal{C}$, a morphism $\alpha_A : F(A) \to G(A)$ in $\mathcal{D}$ (the *component* at $A$)

such that for every morphism $f : A \to B$ in $\mathcal{C}$, the following *naturality square* commutes:

$$\begin{array}{ccc} F(A) & \xrightarrow{F(f)} & F(B) \\ \downarrow_{\alpha_A} & & \downarrow_{\alpha_B} \\ G(A) & \xrightarrow{G(f)} & G(B) \end{array}$$

That is: $\alpha_B \circ F(f) = G(f) \circ \alpha_A$.

The naturality condition says that the transformation $\alpha$ doesn't depend on which path you take through the square. It commutes with the structure (the functors) on both sides.

**Why naturality matters.** Eilenberg and Mac Lane's original question: the double-dual embedding $\eta_V : V \to V^{**}$ (sending $v$ to the evaluation functional $\phi \mapsto \phi(v)$) is "natural" because it commutes with every linear map: for any $f : V \to W$, the diagram

$$\begin{array}{ccc} V & \xrightarrow{f} & W \\ \downarrow_{\eta_V} & & \downarrow_{\eta_W} \\ V^{**} & \xrightarrow{f^{**}} & W^{**} \end{array}$$

commutes. The single-dual $V \to V^*$ requires a choice of basis for each $V$ — the maps $V \xrightarrow{\sim} V^*$ cannot be assembled into a natural transformation from the identity functor to the dual functor. This is the formal content of "the double dual is canonical but the single dual is not."

## Composition and the Functor Category

Natural transformations compose: if $\alpha : F \Rightarrow G$ and $\beta : G \Rightarrow H$, their *vertical composite* $\beta \circ \alpha : F \Rightarrow H$ has components $(\beta \circ \alpha)_A = \beta_A \circ \alpha_A$. The identity natural transformation on $F$ has components $(\mathsf{id}_F)_A = \mathsf{id}_{F(A)}$.

This means: for any two categories $\mathcal{C}$ and $\mathcal{D}$, there is a *functor category* $[\mathcal{C}, \mathcal{D}]$ (also written $\mathcal{D}^\mathcal{C}$) whose objects are functors $\mathcal{C} \to \mathcal{D}$ and whose morphisms are natural transformations.

**Horizontal composition.** There is also a horizontal composition for natural transformations. If $\alpha : F \Rightarrow G$ (functors $\mathcal{C} \to \mathcal{D}$) and $\beta : H \Rightarrow K$ (functors $\mathcal{D} \to \mathcal{E}$), the horizontal composite $\beta \star \alpha : H \circ F \Rightarrow K \circ G$ has components $(\beta \star \alpha)_A = \beta_{G(A)} \circ H(\alpha_A) = K(\alpha_A) \circ \beta_{F(A)}$ (both are equal by naturality of $\beta$).

Natural transformations, with both kinds of composition, form the morphisms of the 2-category $\mathbf{Cat}$ of categories, functors, and natural transformations. This is the first example of a 2-category we encounter in this curriculum (see Chapter 12 for more).

## Natural Isomorphisms and Equivalence of Categories

A natural transformation $\alpha : F \Rightarrow G$ is a *natural isomorphism* if each component $\alpha_A : F(A) \to G(A)$ is an isomorphism. In this case $F \cong G$.

An *equivalence of categories* consists of functors $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{C}$ together with natural isomorphisms $G \circ F \cong \mathsf{Id}_\mathcal{C}$ and $F \circ G \cong \mathsf{Id}_\mathcal{D}$. Two categories are *equivalent* if such data exists.

Equivalence is the right notion of "sameness" for categories — much weaker than isomorphism (which requires the composites to be literally equal), and corresponding to the mathematical practice of treating isomorphic objects as identical. The theorem characterizing equivalences: $F : \mathcal{C} \to \mathcal{D}$ is an equivalence if and only if $F$ is fully faithful and essentially surjective.

In HoTT, equivalences of categories play the role that isomorphisms of sets play in classical mathematics. The formalization of category theory in HoTT (following the *univalent* approach) takes equivalences as the fundamental notion and defines "equality of categories" as equivalence — implementing the univalence philosophy for categories.

## Representable Functors

For any locally small category $\mathcal{C}$ and any object $A \in \mathcal{C}$, the *representable functor* $\mathsf{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$ sends each object $B$ to the set $\mathsf{Hom}(A, B)$ of morphisms from $A$ to $B$.

A functor $F : \mathcal{C} \to \mathbf{Set}$ is *representable* if there exists an object $A$ and a natural isomorphism $\mathsf{Hom}(A, -) \cong F$. The object $A$ is called the *representing object*.

Examples of representable functors:
- The underlying set functor $U : \mathbf{Grp} \to \mathbf{Set}$ is represented by the free group on one generator $\mathbb{Z}$: $\mathsf{Hom}_\mathbf{Grp}(\mathbb{Z}, G) \cong U(G)$. (A group homomorphism from $\mathbb{Z}$ to $G$ is completely determined by where the generator $1$ goes — any element of $G$.)
- The tangent space functor on smooth manifolds: $T_p M$ is represented by the dual numbers $\mathbb{R}[\varepsilon]/(\varepsilon^2)$.
- In type theory: the function type $[A, B]$ represents the functor $\mathsf{Hom}(-, B)$ at the object $A$, via the bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$ (currying).

## The Yoneda Lemma

The Yoneda lemma is one of the most important theorems in mathematics. It says: an object is completely determined by its relationships to other objects.

**Theorem (Yoneda Lemma).** Let $\mathcal{C}$ be a locally small category, $F : \mathcal{C} \to \mathbf{Set}$ a functor, and $A \in \mathcal{C}$ an object. Then there is a bijection, natural in both $F$ and $A$:

$$\mathsf{Nat}(\mathsf{Hom}(A, -), F) \cong F(A)$$

On the left: the set of natural transformations from the representable functor $\mathsf{Hom}(A, -)$ to $F$. On the right: the set $F(A)$.

**Proof.** Given a natural transformation $\alpha : \mathsf{Hom}(A, -) \Rightarrow F$, the bijection sends $\alpha$ to $\alpha_A(\mathsf{id}_A) \in F(A)$.

Given $x \in F(A)$, the bijection constructs the natural transformation $\alpha^x : \mathsf{Hom}(A, -) \Rightarrow F$ with components $\alpha^x_B(f) = F(f)(x)$ for each $f : A \to B$.

**That these are inverse:** $\alpha \mapsto \alpha_A(\mathsf{id}_A) \mapsto [\alpha^x]^B(f) = F(f)(\alpha_A(\mathsf{id}_A)) = \alpha_B(F(A)(f)(\mathsf{id}_A))$... actually, by naturality of $\alpha$: the square $\alpha_A(\mathsf{id}_A) \mapsto F(f)(\alpha_A(\mathsf{id}_A))$ equals $\alpha_B(\mathsf{Hom}(A,f)(\mathsf{id}_A)) = \alpha_B(f \circ \mathsf{id}_A) = \alpha_B(f)$. So $\alpha^{\alpha_A(\mathsf{id}_A)} = \alpha$.

The other direction: $x \mapsto \alpha^x \mapsto \alpha^x_A(\mathsf{id}_A) = F(\mathsf{id}_A)(x) = \mathsf{id}_{F(A)}(x) = x$. $\square$

**What the Yoneda Lemma means.** Every element $x \in F(A)$ corresponds to a "window" through which $A$ sees $F$: the natural transformation $\alpha^x$ tells you, for each $f : A \to B$, the element $F(f)(x) \in F(B)$ you get by "following $f$ from $A$." Knowing all these elements determines $x$ uniquely.

More dramatically: a natural transformation from the representable functor $\mathsf{Hom}(A, -)$ to any functor $F$ is completely determined by a single element of $F(A)$ — the image of $\mathsf{id}_A$. All naturality conditions are forced by this single choice.

## The Yoneda Embedding

**Corollary.** The *Yoneda embedding* $\mathsf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$, sending each object $A$ to the presheaf $\mathsf{Hom}(-, A)$ (the contravariant hom-functor), is fully faithful.

*Proof.* By Yoneda applied to the contravariant case: $\mathsf{Nat}(\mathsf{Hom}(-, A), \mathsf{Hom}(-, B)) \cong \mathsf{Hom}(B, A)$... wait, that's wrong. Let's use $\mathsf{Nat}(\mathsf{Hom}(-, A), \mathsf{Hom}(-, B)) \cong \mathsf{Hom}(A, B)$ from the covariant version. More carefully: $\mathsf{Nat}(\mathsf{y}(A), \mathsf{y}(B)) = \mathsf{Nat}(\mathsf{Hom}(-, A), \mathsf{Hom}(-, B)) \cong \mathsf{Hom}(A, B)$ by applying Yoneda to $F = \mathsf{Hom}(-, B)$ (a contravariant functor, i.e., a presheaf). The Yoneda bijection gives $\mathsf{Nat}(\mathsf{Hom}(-, A), F) \cong F(A) = \mathsf{Hom}(A, B)$.

So the Yoneda embedding $\mathsf{y}$ is fully faithful: every morphism $f : A \to B$ in $\mathcal{C}$ corresponds to a unique natural transformation $\mathsf{y}(f) : \mathsf{y}(A) \Rightarrow \mathsf{y}(B)$, and this correspondence is a bijection. $\square$

**The conceptual content.** Any category $\mathcal{C}$ embeds fully faithfully into the presheaf category $[\mathcal{C}^{op}, \mathbf{Set}]$. Presheaves on $\mathcal{C}$ are "generalized objects" — and the actual objects of $\mathcal{C}$ sit inside this larger category, recoverable from their representability property.

This is the mathematical content of the slogan "an object is determined by its relationships." The presheaf $\mathsf{Hom}(-, A)$ encodes all the information about $A$: for each object $B$, it tells you the set of maps $B \to A$. Knowing all these sets (with their natural functorial structure) determines $A$ up to isomorphism.

## The Yoneda Lemma in Type Theory

The Yoneda lemma has a direct incarnation in type theory: the *Yoneda principle* in HoTT.

For a type $A$ and a type family $B : A \to \mathcal{U}$, the Yoneda lemma gives a natural equivalence:

$$\Big(\prod_{x:A} (a = x) \to B(x)\Big) \simeq B(a)$$

for any $a : A$. The left side is the type of natural transformations from the "representable" type family $\lambda x. (a = x)$ to $B$. The right side is $B(a)$.

The proof: given a function $f : \prod_{x:A} (a = x) \to B(x)$, the bijection sends $f$ to $f(a, \mathsf{refl}_a) : B(a)$. Given $b : B(a)$, we construct $f_b(x, p) = \mathsf{transport}^B(p, b)$. These are inverse. This is path induction in disguise: the $J$ eliminator is the Yoneda lemma for the identity type.

This is not a coincidence. The identity type $a =_A b$ is the hom-type of $A$ viewed as an ∞-groupoid; the Yoneda lemma for ∞-groupoids gives path induction. The two concepts are the same at different levels of generality.

## The Density Theorem

A stronger consequence of Yoneda: every presheaf is a colimit of representable presheaves. Specifically, for any presheaf $F : \mathcal{C}^{op} \to \mathbf{Set}$:

$$F \cong \mathsf{colim}_{(A, x) \in \mathsf{el}(F)} \mathsf{Hom}(-, A)$$

where $\mathsf{el}(F)$ is the *category of elements* of $F$ (objects are pairs $(A, x)$ with $x \in F(A)$; morphisms $(A, x) \to (B, y)$ are $f : A \to B$ with $F(f)(y) = x$).

This says: you can always approximate any generalized object (presheaf) as a colimit of actual objects (representables). The representables generate the entire presheaf category.

This density theorem is the starting point for the theory of sketches, of locally presentable categories, and ultimately for the theory of ∞-toposes: in a topos, sheaves are colimits of representables satisfying a descent condition.
