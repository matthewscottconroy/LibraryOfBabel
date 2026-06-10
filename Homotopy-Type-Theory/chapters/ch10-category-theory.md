# Chapter 10: Category Theory — Structure, Composition, and Universal Properties

## Introduction

Category theory is sometimes called "the mathematics of mathematics" — a language for expressing the structural patterns common to all branches of mathematics. A category consists of objects and morphisms (arrows between objects) satisfying simple composition laws. This abstraction, though initially seeming vacuous, is extraordinarily powerful: it reveals deep commonalities between groups and topological spaces, between logic and type theory, between sets and sheaves.

For our curriculum, category theory plays four roles:
1. **Language:** It gives us a precise vocabulary for structural properties (limits, colimits, adjoints).
2. **Semantics:** The categorical models of type theory (cartesian closed categories, toposes) explain why the formal rules work.
3. **Higher structure:** Higher category theory (∞-categories, ∞-groupoids) provides the homotopy-theoretic interpretation of HoTT.
4. **Practice:** Lean 4's Mathlib is organized categorically; formalization in Mathlib requires fluency with the `CategoryTheory` library.

---

## 1. Categories

### 1.1 The Definition

**Definition 10.1 (Category).** A *category* $\mathcal{C}$ consists of:
- A collection $\mathsf{Ob}(\mathcal{C})$ of *objects* (written $A, B, C, \ldots$)
- For each pair of objects $A, B$, a set $\mathsf{Hom}_\mathcal{C}(A, B)$ of *morphisms* (or *arrows*) from $A$ to $B$ (written $f : A \to B$)
- For each triple $A, B, C$, a *composition* function $\circ : \mathsf{Hom}(B,C) \times \mathsf{Hom}(A,B) \to \mathsf{Hom}(A,C)$
- For each object $A$, an *identity morphism* $\mathsf{id}_A : A \to A$

satisfying:
- **Associativity:** $(h \circ g) \circ f = h \circ (g \circ f)$ for composable $f, g, h$
- **Left unit:** $\mathsf{id}_B \circ f = f$ for $f : A \to B$
- **Right unit:** $f \circ \mathsf{id}_A = f$ for $f : A \to B$

**Remark 10.2.** A category is *small* if both the collection of objects and all hom-sets are actual sets (not proper classes). It is *locally small* if all hom-sets are sets (even if the collection of objects is a proper class).

### 1.2 Examples

**Example 10.3 (Set).** The category $\mathbf{Set}$: objects are sets, morphisms are functions, composition is function composition, identity is the identity function.

**Example 10.4 (Grp).** The category $\mathbf{Grp}$: objects are groups, morphisms are group homomorphisms.

**Example 10.5 (Top).** The category $\mathbf{Top}$: objects are topological spaces, morphisms are continuous maps.

**Example 10.6 (Type, the category of types in MLTT).** Objects are types $A : \mathsf{Type}$; morphisms $A \to B$ are functions $f : A \to B$; composition is function composition; identity is $\lambda x, x$.

**Example 10.7 (Discrete Category).** Given a set $S$, the discrete category on $S$ has elements of $S$ as objects and only identity morphisms. No non-trivial arrows.

**Example 10.8 (Preorder as Category).** A preorder $(P, \leq)$ gives a category where the objects are elements of $P$ and $\mathsf{Hom}(a, b) = \{*\}$ if $a \leq b$, else $\mathsf{Hom}(a, b) = \emptyset$. Composition uses transitivity; identity uses reflexivity. (A *poset* corresponds to a category where each hom-set has at most one element and the category is also skeletal: $a \cong b \Rightarrow a = b$.)

**Example 10.9 (Monoid as Category).** A monoid $(M, \cdot, e)$ is a one-object category $\mathbf{B}M$: the single object is $*$, and $\mathsf{Hom}(*, *) = M$. Composition is monoid multiplication; identity is the unit $e$.

**Example 10.10 (Opposite Category).** For any category $\mathcal{C}$, the *opposite* category $\mathcal{C}^{op}$ has the same objects but all arrows reversed: $\mathsf{Hom}_{\mathcal{C}^{op}}(A, B) = \mathsf{Hom}_\mathcal{C}(B, A)$.

### 1.3 Isomorphisms in a Category

**Definition 10.11.** A morphism $f : A \to B$ is an *isomorphism* if there exists $g : B \to A$ with $g \circ f = \mathsf{id}_A$ and $f \circ g = \mathsf{id}_B$. We write $A \cong B$.

**Example 10.12.** In $\mathbf{Set}$, isomorphisms are bijections. In $\mathbf{Grp}$, isomorphisms are group isomorphisms. In $\mathbf{Top}$, isomorphisms are homeomorphisms. In a preorder, $a \cong b$ iff $a = b$ (since there can be at most one morphism in each direction).

---

## 2. Functors

**Definition 10.13 (Functor).** A *functor* $F : \mathcal{C} \to \mathcal{D}$ consists of:
- A function on objects: $F : \mathsf{Ob}(\mathcal{C}) \to \mathsf{Ob}(\mathcal{D})$
- A function on morphisms: for each $f : A \to B$ in $\mathcal{C}$, a morphism $F(f) : F(A) \to F(B)$ in $\mathcal{D}$

satisfying:
- **Identity:** $F(\mathsf{id}_A) = \mathsf{id}_{F(A)}$
- **Composition:** $F(g \circ f) = F(g) \circ F(f)$

A *contravariant functor* $F : \mathcal{C} \to \mathcal{D}$ reverses arrows: $F(f) : F(B) \to F(A)$ for $f : A \to B$. Equivalently, it is a functor $\mathcal{C}^{op} \to \mathcal{D}$.

**Example 10.14.** The *forgetful functor* $U : \mathbf{Grp} \to \mathbf{Set}$ sends each group to its underlying set and each group homomorphism to the underlying function.

**Example 10.15.** The *free functor* $F : \mathbf{Set} \to \mathbf{Grp}$ sends each set to the free group on that set (Chapter 2).

**Example 10.16.** The *fundamental group* $\pi_1 : \mathbf{Top}_* \to \mathbf{Grp}$ is a functor from pointed topological spaces to groups (Chapter 14). This is a crucial connection between topology and algebra.

**Example 10.17.** The *Hom functor*: for a fixed $B \in \mathcal{C}$, the functor $\mathsf{Hom}(-, B) : \mathcal{C}^{op} \to \mathbf{Set}$ sends $A$ to $\mathsf{Hom}(A, B)$ and $f : A' \to A$ to the precomposition function $f^* : \mathsf{Hom}(A, B) \to \mathsf{Hom}(A', B)$.

---

## 3. Natural Transformations

**Definition 10.18 (Natural Transformation).** Given functors $F, G : \mathcal{C} \to \mathcal{D}$, a *natural transformation* $\alpha : F \Rightarrow G$ is a family of morphisms:
$$\alpha_A : F(A) \to G(A) \quad \text{for each object } A \in \mathcal{C}$$
satisfying the *naturality condition*: for every $f : A \to B$ in $\mathcal{C}$,
$$G(f) \circ \alpha_A = \alpha_B \circ F(f)$$

This says the following square commutes:
$$\begin{array}{ccc} F(A) & \xrightarrow{\alpha_A} & G(A) \\ F(f)\downarrow & & \downarrow G(f) \\ F(B) & \xrightarrow{\alpha_B} & G(B) \end{array}$$

**Example 10.19.** The *determinant* is a natural transformation: for each field $k$, $\det : \mathsf{GL}_n(k) \to k^*$ is a group homomorphism, and these are natural in $k$ in a suitable sense.

**Example 10.20.** The *double dual* embedding $\eta_V : V \to V^{**}$ (for vector spaces) is a natural transformation $\mathsf{id} \Rightarrow (-)^{**}$.

**Functor categories:** Given categories $\mathcal{C}$ and $\mathcal{D}$, there is a *functor category* $[\mathcal{C}, \mathcal{D}]$ (or $\mathcal{D}^\mathcal{C}$) whose objects are functors $\mathcal{C} \to \mathcal{D}$ and whose morphisms are natural transformations. Composition of natural transformations is pointwise.

---

## 4. The Yoneda Lemma

The Yoneda lemma is one of the most important and deepest theorems in mathematics. It says: a mathematical object is completely determined by its relationships to all other objects.

### 4.1 Representable Functors

**Definition 10.21.** A functor $F : \mathcal{C}^{op} \to \mathbf{Set}$ is *representable* if there exists an object $A \in \mathcal{C}$ and a natural isomorphism $F \cong \mathsf{Hom}(-, A)$. The object $A$ *represents* $F$.

### 4.2 The Yoneda Embedding

**Definition 10.22.** The *Yoneda embedding* is the functor $\mathbf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ defined by:
$$\mathbf{y}(A) = \mathsf{Hom}(-, A) \quad \text{(the representable presheaf at } A\text{)}$$

On morphisms: for $f : A \to B$, $\mathbf{y}(f) = f_* : \mathsf{Hom}(-, A) \Rightarrow \mathsf{Hom}(-, B)$ (postcomposition by $f$).

**Theorem 10.23 (Yoneda Lemma).** For any locally small category $\mathcal{C}$, functor $F : \mathcal{C}^{op} \to \mathbf{Set}$, and object $A \in \mathcal{C}$:
$$[\mathcal{C}^{op}, \mathbf{Set}](\mathsf{Hom}(-, A),\, F) \cong F(A)$$
naturally in $A$ and $F$.

*Proof.* We construct the bijection. Given a natural transformation $\alpha : \mathsf{Hom}(-, A) \Rightarrow F$, evaluate at $A$ and at $\mathsf{id}_A$:
$$\Phi(\alpha) = \alpha_A(\mathsf{id}_A) \in F(A).$$

Conversely, given $x \in F(A)$, define a natural transformation $\Psi(x)$ by:
$$\Psi(x)_B : \mathsf{Hom}(B, A) \to F(B), \quad f \mapsto F(f)(x).$$

**Verify naturality of $\Psi(x)$:** For $g : B' \to B$, we need $F(g) \circ \Psi(x)_B = \Psi(x)_{B'} \circ \mathsf{Hom}(g, A)$. This says $F(g)(F(f)(x)) = F(f \circ g)(x)$, which holds since $F$ is a functor.

**Verify $\Phi \circ \Psi = \mathsf{id}$:** $\Phi(\Psi(x)) = \Psi(x)_A(\mathsf{id}_A) = F(\mathsf{id}_A)(x) = \mathsf{id}_{F(A)}(x) = x$.

**Verify $\Psi \circ \Phi = \mathsf{id}$:** Given $\alpha$, check $\Psi(\Phi(\alpha))_B(f) = F(f)(\alpha_A(\mathsf{id}_A)) = \alpha_B(\mathsf{id}_A \circ f)^{\star} = \alpha_B(f)$, where $(*)$ uses naturality of $\alpha$. $\square$

**Corollary 10.24 (Yoneda embedding is fully faithful).** $\mathbf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ is fully faithful: $\mathsf{Hom}_\mathcal{C}(A, B) \cong [\mathcal{C}^{op}, \mathbf{Set}](\mathbf{y}(A), \mathbf{y}(B))$. Equivalently, $A \cong B$ iff $\mathbf{y}(A) \cong \mathbf{y}(B)$.

**What Yoneda means:** An object $A$ is completely determined (up to isomorphism) by the functor $\mathsf{Hom}(-, A)$. To understand $A$, you don't need to know anything "internal" to it — you only need to know how other objects map *into* it.

This is the categorical expression of a mathematical culture: define objects by their relationships (universal properties), not by their internal constitution.

---

## 5. Universal Properties: Limits and Colimits

### 5.1 Terminal and Initial Objects

**Definition 10.25.** An object $\mathbf{1} \in \mathcal{C}$ is *terminal* if for every $A$, there exists a unique morphism $A \to \mathbf{1}$.

An object $\mathbf{0} \in \mathcal{C}$ is *initial* if for every $A$, there exists a unique morphism $\mathbf{0} \to A$.

**Examples:** In $\mathbf{Set}$: terminal is any singleton $\{*\}$; initial is $\emptyset$. In $\mathbf{Grp}$: both terminal and initial is the trivial group $\{e\}$. In a preorder: terminal is the maximum element (if it exists); initial is the minimum.

Terminal/initial objects are unique *up to unique isomorphism*. This is the prototype of uniqueness via universal properties.

### 5.2 Products and Coproducts

**Definition 10.26 (Product).** The *product* of $A$ and $B$ is an object $A \times B$ with projections $\pi_1 : A \times B \to A$ and $\pi_2 : A \times B \to B$ such that: for every $C$ and morphisms $f : C \to A$, $g : C \to B$, there is a unique $\langle f, g \rangle : C \to A \times B$ with $\pi_1 \circ \langle f, g \rangle = f$ and $\pi_2 \circ \langle f, g \rangle = g$.

$$\begin{array}{ccc} & C & \\ f\swarrow & \downarrow \exists!\langle f,g\rangle & \searrow g \\ A & \leftarrow \pi_1 \quad A \times B \quad \xrightarrow{\pi_2} & B \end{array}$$

**Definition 10.27 (Coproduct).** The *coproduct* $A + B$ (or $A \sqcup B$) is the dual: injections $\iota_1 : A \to A + B$, $\iota_2 : B \to A + B$, with a unique copairing $[f, g] : A + B \to C$ for any $f : A \to C$, $g : B \to C$.

**Examples:** In $\mathbf{Set}$: products are Cartesian products; coproducts are disjoint unions. In $\mathbf{Grp}$: products are direct products; coproducts are free products. In a preorder: products are meets (greatest lower bounds); coproducts are joins.

### 5.3 Equalizers, Pullbacks, and the General Notion of Limit

**Definition 10.28 (Equalizer).** The *equalizer* of $f, g : A \to B$ is an object $E$ with a morphism $e : E \to A$ such that $f \circ e = g \circ e$, and for any $h : C \to A$ with $f \circ h = g \circ h$, there is a unique $C \to E$ making the triangle commute.

**Definition 10.29 (Pullback).** The *pullback* of $f : A \to C$ and $g : B \to C$ is an object $A \times_C B$ with morphisms to $A$ and $B$ making the square commute, universal among all such cones.

**General limits and colimits** unify all these constructions: a *limit* is a universal cone over a diagram; a *colimit* is a universal cocone. Products, equalizers, and pullbacks are all special cases of limits.

---

## 6. Adjunctions

Adjunctions are "the most important concept in category theory" (Riehl). They formalize the idea that two operations are "inverse to each other in a weak sense."

### 6.1 Definition

**Definition 10.30 (Adjunction).** An *adjunction* $F \dashv G$ (read "$F$ is left adjoint to $G$") between functors $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{C}$ is a natural bijection:
$$\mathsf{Hom}_\mathcal{D}(F(A), B) \cong \mathsf{Hom}_\mathcal{C}(A, G(B))$$
for all $A \in \mathcal{C}$ and $B \in \mathcal{D}$.

The bijection is natural in both $A$ and $B$.

### 6.2 Unit and Counit

The adjunction is equivalently given by:
- The *unit*: a natural transformation $\eta : \mathsf{id}_\mathcal{C} \Rightarrow G \circ F$ (with components $\eta_A : A \to G(F(A))$)
- The *counit*: a natural transformation $\varepsilon : F \circ G \Rightarrow \mathsf{id}_\mathcal{D}$ (with components $\varepsilon_B : F(G(B)) \to B$)

satisfying the *triangular identities*:
$$(\varepsilon_{F(A)}) \circ F(\eta_A) = \mathsf{id}_{F(A)} \qquad G(\varepsilon_B) \circ \eta_{G(B)} = \mathsf{id}_{G(B)}$$

### 6.3 Examples of Adjunctions

**Example 10.31 (Free-Forgetful).** Free group $\dashv$ Forgetful: $F : \mathbf{Set} \to \mathbf{Grp}$ is left adjoint to $U : \mathbf{Grp} \to \mathbf{Set}$. A group homomorphism $F(S) \to G$ is the same as a function $S \to U(G)$.

**Example 10.32 (Product-Exponential).** In $\mathbf{Set}$: $(-) \times A \dashv A^{(-)}$ (or $[A, -]$). A function $B \times A \to C$ is the same as a function $B \to C^A = [A, C]$. This is *currying*.

**Example 10.33 (Limits and Colimits).** Left adjoints preserve colimits; right adjoints preserve limits. In particular, the product functor $(-) \times A$ is a left adjoint (to the exponential), so it preserves coproducts: $(B + C) \times A \cong (B \times A) + (C \times A)$.

**Example 10.34 (Curry-Howard).** In the internal logic of a cartesian closed category: the introduction rule for $\to$ is an adjunction. In type theory: the typing rule $\Gamma, x : A \vdash t : B$ iff $\Gamma \vdash \lambda x, t : A \to B$ is an adjunction between context extension and the function type.

### 6.4 The Fundamental Theorem

**Theorem 10.35.** Left adjoints preserve colimits. Right adjoints preserve limits.

*Proof.* If $F \dashv G$ and $D : \mathcal{J} \to \mathcal{C}$ is a diagram with colimit $\text{colim}\, D$, then $F(\text{colim}\, D) \cong \text{colim}\, F \circ D$. This follows from the natural bijection and the universal property of colimits. $\square$

This theorem is constantly used in mathematics: it explains why direct limits commute with tensor products (tensor product is a left adjoint), why inverse limits commute with Hom (Hom is a right adjoint), etc.

---

## 7. Monads

**Definition 10.36 (Monad).** A *monad* on $\mathcal{C}$ is a triple $(T, \eta, \mu)$ where $T : \mathcal{C} \to \mathcal{C}$ is a functor, $\eta : \mathsf{id} \Rightarrow T$ (unit), and $\mu : T^2 \Rightarrow T$ (multiplication), satisfying:
- $\mu \circ T\eta = \mu \circ \eta T = \mathsf{id}_T$ (unit laws)
- $\mu \circ \mu T = \mu \circ T\mu$ (associativity)

**Example 10.37.** Every adjunction $F \dashv G$ gives a monad $T = G \circ F$ on $\mathcal{C}$, with $\eta$ the unit of the adjunction and $\mu = G(\varepsilon_F)$.

**In programming:** Monads (in Haskell) are exactly monads in the category of types and functions. The `Maybe` monad, the list monad, the IO monad all arise from specific adjunctions or are monads in the categorical sense.

**In type theory:** The propositional truncation $\| - \|$ is a monad on the category of types in HoTT.

---

## 8. Connection to Type Theory

Category theory and type theory are deeply connected through the *internal language* correspondence.

| **Category** | **Type Theory** |
|---|---|
| Cartesian closed category (CCC) | Simply typed lambda calculus |
| Locally cartesian closed category (LCCC) | Dependent type theory |
| Topos | Higher-order intuitionistic logic |
| Presheaf category $[\mathcal{C}^{op}, \mathbf{Set}]$ | A model of type theory |
| Kan complex | A model of HoTT (via $\infty$-groupoids) |

Every model of type theory is a category of a specific kind. Every CCC gives a model of STLC. Every LCCC gives a model of dependent type theory. The Yoneda embedding provides a canonical way to build models.

---

## Exercises

**10.1.** Verify that the following are categories by checking associativity and identity:
  - The category of matrices: objects are natural numbers, morphisms $n \to m$ are $m \times n$ matrices over $\mathbb{R}$, composition is matrix multiplication.
  - The category of relations: objects are sets, morphisms $A \to B$ are relations $R \subseteq A \times B$, composition is relational composition.

**10.2.** Show that the hom functor $\mathsf{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$ is indeed a functor (verify the functor axioms).

**10.3.** Prove the Yoneda lemma in detail. In particular, verify that $\Psi(\Phi(\alpha)) = \alpha$ using the naturality of $\alpha$.

**10.4.** Show that terminal objects are unique up to unique isomorphism.

**10.5.** Show that in any category, isomorphisms satisfy: if $f$ and $g$ are isomorphisms, so is $g \circ f$; and $(g \circ f)^{-1} = f^{-1} \circ g^{-1}$.

**10.6.** Prove that in $\mathbf{Set}$:
  - Products are Cartesian products
  - Equalizers of $f, g : A \to B$ are $\{a \in A \mid f(a) = g(a)\}$
  - Pullbacks are $\{(a, b) \in A \times B \mid f(a) = g(b)\}$

**10.7.** The *opposite duality*: For any statement in category theory that holds for all categories, its *dual* (obtained by reversing all arrows) also holds. State and prove the dual of: "products are characterized by their universal property."

**10.8.** In Lean 4's `CategoryTheory` library: state and use the Yoneda lemma. Identify the types `Hom`, `functor`, and `NatTrans` in Mathlib.

**10.9 (Challenge).** Prove that limits can be built from products and equalizers: given any diagram $D : \mathcal{J} \to \mathcal{C}$, the limit of $D$ (if it exists) can be computed as an equalizer of two maps between products. (This is the standard construction of general limits from finite limits.)
