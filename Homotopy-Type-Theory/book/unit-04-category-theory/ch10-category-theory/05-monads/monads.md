# Monads

## From Adjunctions to Monads

Every adjunction $F \dashv G$ gives a monad. This is not a construction — it is a theorem. And its converse is also true: every monad arises from an adjunction (in fact, from two canonical ones). Monads and adjunctions are two perspectives on the same structure.

**Definition.** A *monad* on a category $\mathcal{C}$ is a triple $(T, \eta, \mu)$ where:
- $T : \mathcal{C} \to \mathcal{C}$ is a functor (the *underlying functor*)
- $\eta : \mathsf{Id}_\mathcal{C} \Rightarrow T$ is a natural transformation (the *unit*)
- $\mu : T^2 \Rightarrow T$ is a natural transformation (the *multiplication*, where $T^2 = T \circ T$)

satisfying:
- **Associativity:** $\mu \circ T\mu = \mu \circ \mu T$ (both give $T^3 \Rightarrow T$)
- **Left unit:** $\mu \circ T\eta = \mathsf{id}_T$
- **Right unit:** $\mu \circ \eta T = \mathsf{id}_T$

These axioms are exactly the axioms of a monoid: if you think of $\eta$ as the unit element and $\mu$ as multiplication, a monad is a "monoid in the category of endofunctors." The slogan (from Mac Lane) is apt.

## Monads from Adjunctions

Given an adjunction $F \dashv G$ with $F : \mathcal{C} \to \mathcal{D}$, $G : \mathcal{D} \to \mathcal{C}$, unit $\eta$, and counit $\varepsilon$, define:

- $T = G \circ F : \mathcal{C} \to \mathcal{C}$
- Unit of the monad: $\eta : \mathsf{Id}_\mathcal{C} \Rightarrow G \circ F$ (the adjunction unit)
- Multiplication: $\mu_A = G(\varepsilon_{F(A)}) : G(F(G(F(A)))) \to G(F(A))$ (i.e., $G \varepsilon F : T^2 \Rightarrow T$)

The monad axioms follow from the triangular identities of the adjunction.

**Examples:**
- Free group $\dashv$ forgetful gives the monad $T$ on $\mathbf{Set}$ sending a set $S$ to the underlying set of the free group $F(S)$: $T(S) = F(S)$ as a set.
- Product $(-) \times A \dashv [A, -]$ gives the monad $T = [A, A \times -]$ on $\mathcal{C}$.
- Propositional truncation gives the monad $T = \|-\|$ on $\mathcal{U}$: $T(A) = \|A\|$.

## Algebras for a Monad

Given a monad $(T, \eta, \mu)$ on $\mathcal{C}$, a *$T$-algebra* is a pair $(A, \alpha)$ where $A \in \mathcal{C}$ is an object and $\alpha : T(A) \to A$ is a morphism (the *structure map*) satisfying:
- $\alpha \circ \eta_A = \mathsf{id}_A$ (unit law)
- $\alpha \circ \mu_A = \alpha \circ T(\alpha)$ (associativity law)

A *morphism* of $T$-algebras $(A, \alpha) \to (B, \beta)$ is a morphism $f : A \to B$ in $\mathcal{C}$ with $f \circ \alpha = \beta \circ T(f)$. The category of $T$-algebras and their morphisms is the *Eilenberg-Moore category* $\mathcal{C}^T$.

**The key theorem (Eilenberg-Moore):** Given any adjunction $F \dashv G$ with induced monad $T$, there is a comparison functor $K : \mathcal{D} \to \mathcal{C}^T$ making everything commute. The adjunction "factors through" the Eilenberg-Moore category in a universal way.

**The converse (existence of adjoints):** Given a monad $(T, \eta, \mu)$ on $\mathcal{C}$, there exists an adjunction $F^T \dashv U^T$ (between $\mathcal{C}$ and $\mathcal{C}^T$) with induced monad exactly $(T, \eta, \mu)$. So every monad comes from an adjunction.

## Monads in Algebra

The Eilenberg-Moore theorem makes precise the slogan "monads are algebraic theories."

**Groups as algebras for a monad.** The free group monad $T$ on $\mathbf{Set}$ has algebras that are exactly groups. A $T$-algebra $(A, \alpha)$ is a set $A$ with a structure map $\alpha : T(A) \to A$ (taking a "free group expression" over $A$ to an element of $A$) satisfying the monad laws. These laws force $\alpha$ to be an associative, unital multiplication — exactly a group structure.

More generally: any algebraic structure (monoid, group, ring, module, lattice) that can be specified by operations and equations is the category of algebras for some monad on $\mathbf{Set}$. This is the precise content of "monads are algebraic theories."

**Monoids.** The free monoid monad $T(S) = S^* = \bigsqcup_{n \geq 0} S^n$ (lists over $S$) has algebras that are monoids. The structure map $\alpha : S^* \to A$ says: "evaluate a list of elements of $A$ under the monoid operation."

**Vector spaces.** Over a field $k$, the free vector space monad $T(S) = k^{(S)}$ (formal $k$-linear combinations of elements of $S$) has algebras that are $k$-vector spaces.

## Monads in Computer Science: Computational Effects

In functional programming (Haskell, OCaml, Lean 4), monads are the abstract interface for *computational effects*: operations that go beyond pure function application.

A monad in this sense is:
- A type constructor $M : \mathsf{Type} \to \mathsf{Type}$ (the "effectful" version of the type)
- $\mathsf{return} : A \to M(A)$ (the unit: embedding a pure value in the effectful world)
- $(\mathbin{>\!\!>\!=}) : M(A) \to (A \to M(B)) \to M(B)$ (bind: sequencing effectful computations)

satisfying the monad laws (left unit, right unit, associativity). This is exactly the Haskell `Monad` class.

**Examples from Haskell:**
- *Maybe monad:* $M(A) = A + \{\mathsf{Nothing}\}$; models computations that may fail. $\mathsf{return}\, a = \mathsf{Just}\, a$; bind propagates `Nothing`.
- *List monad:* $M(A) = A^*$ (lists over $A$); models nondeterminism. $\mathsf{return}\, a = [a]$; bind applies a computation to each element and concatenates.
- *State monad:* $M(A) = S \to (A \times S)$ for a state type $S$; models stateful computation.
- *IO monad:* in Haskell, the type of computations with I/O effects.

The categorical structure ensures that these monadic computations compose correctly. The monad laws are precisely the conditions needed for sequential composition to be associative and have identity elements.

**In Lean 4:** The `Monad` type class in Lean 4 is exactly the categorical monad, applied to the universe of types. `do` notation is syntactic sugar for monadic bind.

## Monads in HoTT: Modalities

In HoTT, monads on the universe $\mathcal{U}$ that satisfy an *idempotency* condition $T(T(A)) \simeq T(A)$ are called *modalities*. They formalize the idea of "modes of truth."

**Propositional truncation $\|-\|$.** The map $A \mapsto \|A\|$ (propositional truncation) is a monad: $\mathsf{return} : A \to \|A\|$; bind sends a proof that $\|A\|$ is inhabited and a function $A \to \|B\|$ to a proof that $\|B\|$ is inhabited. Idempotency: $\|\|A\|\| \simeq \|A\|$.

**$n$-Truncation $\|-\|_n$.** More generally, for each $n \geq -2$, the $n$-truncation is a monad. For $n = -2$: contractibilization. For $n = -1$: propositional truncation. For $n = 0$: set-truncation. These are all idempotent monads.

**Shape modality.** In cohesive HoTT (Chapter 25), the shape modality $\flat$ is a monad capturing the "underlying discrete space" of a geometric type. The comonad $\flat$ extracts the discrete points.

The general theory of modalities in HoTT (Rijke, Shulman, Spitters) shows that every modality gives rise to a factorization system on $\mathcal{U}$: the connected-modal factorization. This is the type-theoretic analogue of the *epi-mono factorization* in category theory.

## The Beck Monadicity Theorem

The question "when does a functor $G : \mathcal{D} \to \mathcal{C}$ come from an adjunction $F \dashv G$ where $G$ is the Eilenberg-Moore category of some monad?" is answered by Beck's monadicity theorem:

**Theorem (Beck).** A functor $G : \mathcal{D} \to \mathcal{C}$ is monadic (equivalent to an Eilenberg-Moore category of a monad on $\mathcal{C}$) if and only if:
1. $G$ has a left adjoint $F$
2. $G$ reflects isomorphisms: if $G(f)$ is an isomorphism, then $f$ is
3. $\mathcal{D}$ has and $G$ preserves coequalizers of $G$-split coequalizer pairs

Beck's theorem is one of the deepest tools in category theory. It characterizes exactly which categories are "categories of algebras for an algebraic theory." In HoTT, analogues of Beck's theorem characterize when a fibration of ∞-groupoids is equivalent to a pullback — this is the descent condition that characterizes ∞-toposes.

## Monads and Logic

The connection between monads and logic is fundamental. Every modality in modal logic corresponds to a monad on the category of propositions.

In the Curry-Howard picture:
- A monad $T$ on types corresponds to a modality $\Box$ in modal logic
- $\eta : A \to T(A)$ corresponds to the necessitation rule: $A \vdash \Box A$
- The monad laws correspond to the modal axioms

Specifically:
- The Identity monad corresponds to $\Box A \leftrightarrow A$ (trivial modality)
- Propositional truncation $\|-\|$ corresponds to classical double-negation $\neg\neg$ (it turns any type into a proposition)
- The $n$-truncation corresponds to the $n$-th level of the h-level hierarchy

The connection runs deep: the theory of modalities in HoTT is a synthesis of modal logic, the categorical theory of monads, and the homotopy-theoretic theory of ∞-connected maps.
