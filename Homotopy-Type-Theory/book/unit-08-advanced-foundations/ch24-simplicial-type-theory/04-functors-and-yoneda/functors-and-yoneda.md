# 24.4 Functors, Natural Transformations, and the Synthetic Yoneda Lemma

## The Most Remarkable Collapse

In classical category theory, a functor between categories is not just a function. It is a pair of functions — one on objects, one on morphisms — satisfying the functoriality conditions: it maps identities to identities and composition to composition. To verify that something is a functor, you must check these conditions explicitly.

In simplicial type theory, functors between Segal types are just *functions*. No extra conditions. No pairs of functions. No functoriality checks. A function $f : A \to B$ between Segal types is automatically a functor.

This collapse is not a cheap trick. It is a consequence of the Segal condition and the way morphisms are defined in STT. When morphisms are functions from $\mathbf{2}$ and when hom types are extension types, every function between types automatically preserves the categorical structure.

## Functors Are Functions

**Theorem.** If $A$ and $B$ are Segal types, then every function $f : A \to B$ is a functor: it preserves identities and composition.

*Proof sketch.* 

*Identities*: The identity morphism at $a$ is $\mathsf{id}_a = \lambda t. a : \mathsf{hom}_A(a, a)$. The image under $f$ is $f \circ \mathsf{id}_a = \lambda t. f(a) = \mathsf{id}_{f(a)}$. ✓ (This is just function application — no calculation needed.)

*Composition*: A composable pair $(g : \mathsf{hom}_A(a, b), h : \mathsf{hom}_A(b, c))$ has a composite $h \circ g : \mathsf{hom}_A(a, c)$ determined by the Segal condition. The image under $f$ is $(f \circ h) \circ (f \circ g)$, which is also the unique composite of the pair $(f \circ g, f \circ h)$. By uniqueness of composites in $B$, $f(h \circ g) = f(h) \circ f(g)$. ✓

The key: "apply $f$ to a morphism" means "post-compose with $f$" — that is, $f$ acts on $\mathsf{hom}_A(a, b)$ by $f_* : \mathsf{hom}_A(a, b) \to \mathsf{hom}_B(f(a), f(b))$ defined as $f_*(g) = f \circ g$ (viewing $g : \mathbf{2} \to A$ and composing with $f : A \to B$). Functoriality follows from function composition being associative and the identity being $\lambda t. a$.

## Natural Transformations Are Directed Paths in Function Types

**Theorem.** For Segal types $A$ and $B$, a natural transformation between functors $f, g : A \to B$ is an element of $\mathsf{hom}_{B^A}(f, g)$ — a directed path from $f$ to $g$ in the *function type* $A \to B$.

Here $B^A :\equiv (A \to B)$ is the type of functions from $A$ to $B$. The hom type in $B^A$:

$$\mathsf{hom}_{B^A}(f, g) = \{ \alpha : \mathbf{2} \to (A \to B) \mid \alpha(0) = f \text{ and } \alpha(1) = g \}$$

An element $\alpha : \mathsf{hom}_{B^A}(f, g)$ is a function of a directed interval variable $t : \mathbf{2}$, giving a function $\alpha(t) : A \to B$ at each $t$, with $\alpha(0) = f$ and $\alpha(1) = g$.

For each object $a : A$, we get a morphism $\alpha_a :\equiv \lambda t. \alpha(t)(a) : \mathsf{hom}_B(f(a), g(a))$ — the component of the natural transformation at $a$.

**Naturality is automatic.** In classical category theory, a natural transformation must satisfy the naturality condition: for every morphism $h : a \to b$ in $A$, the diagram $g(h) \circ \alpha_a = \alpha_b \circ f(h)$ commutes. In STT, this is automatic from the fact that $\alpha$ is a function out of $\mathbf{2}$ applied to $h : \mathbf{2} \to A$: the square $\alpha \circ h : \mathbf{2}^2 \to B$ witnesses the naturality by a 2-simplex computation.

The collapse: in STT, natural transformations are morphisms in the function type. No naturality condition to check. No commuting squares to verify. They are automatically natural because they are functions.

## The Functor Type Is Segal

**Theorem.** If $B$ is a Segal type, then for any type $A$, the function type $B^A = (A \to B)$ is Segal.

*Proof sketch.* Given a composable pair of natural transformations $(\alpha : \mathsf{hom}_{B^A}(f, g), \beta : \mathsf{hom}_{B^A}(g, h))$, the composite $\beta \circ \alpha$ is defined pointwise: $(\beta \circ \alpha)_a :\equiv \beta_a \circ \alpha_a$ for each $a : A$. Since $B$ is Segal, the composite $\beta_a \circ \alpha_a$ is uniquely defined, and the assignment $a \mapsto \beta_a \circ \alpha_a$ gives a natural transformation $A \to B$. The uniqueness follows from uniqueness in $B$ applied pointwise. $\square$

This means: the "category of functors from $A$ to $B$" is itself a Segal type, with composition inherited from $B$. No coherence conditions needed — just pointwise composition.

## The Yoneda Lemma: Classical Statement

The classical Yoneda lemma states: for a locally small category $\mathcal{C}$, an object $c \in \mathcal{C}$, and a functor $F : \mathcal{C} \to \mathsf{Set}$:

$$\mathsf{Nat}(\mathsf{Hom}(c, -), F) \cong F(c)$$

The set of natural transformations from the representable functor $\mathsf{Hom}(c, -)$ to $F$ is in bijection with the elements of $F(c)$.

The bijection is: given $\alpha \in \mathsf{Nat}(\mathsf{Hom}(c, -), F)$, take the element $\alpha_c(\mathsf{id}_c) \in F(c)$ (apply the component at $c$ to the identity morphism). Conversely, given $x \in F(c)$, define $\alpha_d(f) = F(f)(x)$ for each $d$ and $f : \mathsf{Hom}(c, d)$.

The classical proof is a calculation. It works, but it requires setting up all the machinery of categories, functors, and natural transformations first.

## The Synthetic Yoneda Lemma

**Theorem (Synthetic Yoneda, Riehl-Shulman).** For a Segal type $A$, an object $a : A$, and a covariant fibration $C : A \to \mathsf{Type}$:

$$\mathsf{hom}_{(A \to \mathsf{Type})}(\mathsf{hom}_A(a, -), C) \simeq C(a)$$

The equivalence is given by the evaluation map:
$$\mathsf{ev}_a : \mathsf{hom}_{(A \to \mathsf{Type})}(\mathsf{hom}_A(a, -), C) \to C(a)$$
$$\mathsf{ev}_a(\alpha) = \alpha_a(\mathsf{id}_a)$$

*Proof in STT.*

We need to show $\mathsf{ev}_a$ is an equivalence. The key is to construct its quasi-inverse.

**Construction of the inverse**: Given $x : C(a)$, define $\Phi(x) : \mathsf{hom}_A(a, -) \Rightarrow C$ by:
$$\Phi(x)_b(f) :\equiv f_*(x)$$
where $f_*(x) : C(b)$ is the transport of $x$ along $f : \mathsf{hom}_A(a, b)$ in the covariant fibration $C$.

(The covariant fibration structure of $C$ provides exactly this transport: for each morphism $f$, there is an induced map $f_* : C(a) \to C(b)$.)

**Checking $\Phi(\mathsf{ev}_a(\alpha)) = \alpha$**: For each $b$ and $f : \mathsf{hom}_A(a, b)$:
$$\Phi(\mathsf{ev}_a(\alpha))_b(f) = f_*(\alpha_a(\mathsf{id}_a)) = \alpha_b(f \circ \mathsf{id}_a) = \alpha_b(f)$$
using the naturality of $\alpha$ (automatic in STT) and the identity law for $f$.

**Checking $\mathsf{ev}_a(\Phi(x)) = x$**:
$$\mathsf{ev}_a(\Phi(x)) = \Phi(x)_a(\mathsf{id}_a) = (\mathsf{id}_a)_*(x) = x$$
since transport along an identity morphism is the identity (from the identity law for covariant fibrations). $\square$

**What "synthetic" means**: This proof does not mention simplicial sets, doesn't require choosing specific models of functors, doesn't check any coherence conditions. It works entirely within the type theory — using only the Segal condition, the covariant fibration property, and the basic rules of function types. This is what synthetic means: the proof machinery is built into the language.

## Comparing Classical and Synthetic Yoneda

| Aspect | Classical Yoneda | Synthetic Yoneda |
|--------|------------------|------------------|
| Setting | Category with Hom-sets | Segal type with hom types |
| Functors | Defined with axioms | Just functions |
| Nat. trans. | Defined with naturality | Morphisms in function type |
| Representable $\mathsf{hom}(c,-)$ | A Set-valued functor | A covariant fibration |
| $F(c)$ | A set | A type (possibly with homotopy structure) |
| The bijection | A calculation | An equivalence (from extension types) |
| Naturality check | Required explicitly | Automatic from the type theory |

The synthetic version is not just cleaner — it is *stronger*: the bijection is an *equivalence of types* (not just a bijection of sets), and it works for *space-valued* functors (covariant fibrations with fibers that are arbitrary types, not just sets).

## Adjunctions in STT

With the Yoneda lemma in place, adjunctions between Segal types can be defined and developed:

**Definition.** A functor $L : A \to B$ is a *left adjoint* to $R : B \to A$ if there is an equivalence:
$$\mathsf{hom}_B(L(a), b) \simeq \mathsf{hom}_A(a, R(b))$$
natural in $a : A$ and $b : B$.

In STT, this is stated as an equivalence of types, and "natural in $a$" means the equivalence is given by a morphism in the appropriate function type — automatic from the Segal structure.

The Yoneda lemma is used to prove that adjoints are unique (up to unique isomorphism): if $L$ and $L'$ are both left adjoints to $R$, then there is a unique isomorphism $L \simeq L'$ in the functor type $A \to B$.

## Limits and Colimits

The machinery is now in place for a synthetic theory of limits and colimits:

**Limits** of a functor $F : J \to A$ (where $J$ is a Segal type) are defined via the representability of the limit functor. A limit of $F$ is an object $\varprojlim F : A$ such that:
$$\mathsf{hom}_A(c, \varprojlim F) \simeq (J \to A)[F \Rightarrow \mathsf{const}(c)]$$

(the type of natural transformations from $F$ to the constant functor at $c$). This is a limit-as-representing-object, the synthetic version of the classical definition.

In Rzk, such definitions are being formalized and their properties (existence, universality, stability under equivalences) are being proved.

## The Yoneda Embedding

The classical Yoneda embedding is the functor $\mathsf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathsf{Set}]$ defined by $\mathsf{y}(c) = \mathsf{Hom}(-, c)$. It is fully faithful: $\mathsf{Nat}(\mathsf{Hom}(-, c), \mathsf{Hom}(-, d)) \cong \mathsf{Hom}(c, d)$.

In STT, the synthetic Yoneda embedding is:
$$\mathsf{y}_A : A \to (A^{op} \to \mathsf{Type})$$
$$\mathsf{y}_A(a) :\equiv \mathsf{hom}_A(-, a)$$

(where $A^{op}$ is $A$ with the direction of morphisms reversed). The synthetic Yoneda lemma implies that $\mathsf{y}_A$ is fully faithful: for $a, b : A$,
$$\mathsf{hom}_{A^{op} \to \mathsf{Type}}(\mathsf{y}_A(a), \mathsf{y}_A(b)) \simeq \mathsf{hom}_A(a, b)$$

This is the foundation for *∞-categorical presheaf theory* in STT: a Segal type $A$ can be embedded into the "∞-category of presheaves on $A$," which is itself a Segal type. The embedding is fully faithful, allowing $A$ to be studied via its presheaves.

## The State of the Art in Rzk

As of 2024, the Rzk formalization includes:

- Segal types, Rezk types, their basic properties
- Functors (as functions), natural transformations (as hom-morphisms)
- The synthetic Yoneda lemma and its consequences
- Adjoint functors: definition, unit-counit characterization, uniqueness
- The beginnings of limit theory

The Riehl-Shulman paper proves significantly more: the twisted arrow category, the Joyal model structure on simplicial sets interprets STT, and several results from quasicategory theory are reproved synthetically. But the full formalization in Rzk is ongoing.

The significance: for the first time, there is a type theory in which ∞-category theory can be done *natively* — where the statements of theorems look like category theory, not like complicated constructions in model category theory or simplicial set theory. The Yoneda lemma in Rzk looks like the Yoneda lemma. It is not encoded or translated. It is said directly, in the language of the theory.
