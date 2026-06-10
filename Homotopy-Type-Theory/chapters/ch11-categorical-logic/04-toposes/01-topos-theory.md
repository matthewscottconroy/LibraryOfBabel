# 4.1 Toposes

## What a Topos Is

A topos (from the Greek "place" — as in the "place" where mathematical logic lives) is a category that has enough structure to do most of mathematics internally, with its own "internal logic."

The key property: a topos has a *subobject classifier* $\Omega$ that plays the role of the "type of propositions." Every predicate on $A$ (every subtype of $A$) corresponds to a morphism $A \to \Omega$, just as every subset of a set $A$ corresponds to its characteristic function $A \to \{\mathsf{true}, \mathsf{false}\}$.

## The Definition

**Definition.** An *elementary topos* $\mathcal{E}$ is a category with:
1. **Finite limits** (equivalently: terminal object $\mathbf{1}$ and all pullbacks)
2. **Exponentials** (for any $A, B$: an object $B^A$ representing functions $A \to B$)
3. **A subobject classifier:** an object $\Omega$ and a morphism $\mathsf{true} : \mathbf{1} \to \Omega$ such that for every monomorphism $m : S \hookrightarrow A$, there exists a unique *characteristic morphism* $\chi_m : A \to \Omega$ making the square

$$\begin{array}{ccc} S & \to & \mathbf{1} \\ m\downarrow & & \downarrow \mathsf{true} \\ A & \xrightarrow{\chi_m} & \Omega \end{array}$$

a pullback.

A monomorphism $m : S \hookrightarrow A$ is the categorical analog of an injective function — it represents a "subobject" of $A$. The subobject classifier $\Omega$ is the type of truth values.

**Properties of toposes:**
- Every topos is a CCC (from conditions 1 + 2)
- Every topos has all finite colimits (provable from the three conditions)
- Every topos has an *internal language* — an intuitionistic higher-order logic
- Every topos has its own "internal set theory"

## Examples

**$\mathbf{Set}$.** The archetypal topos.
- Terminal object: any singleton
- Exponentials: function sets $B^A$
- Subobject classifier: $\Omega = \{\mathsf{true}, \mathsf{false}\}$
- Characteristic morphism: the characteristic function of a subset

The internal logic of $\mathbf{Set}$ is classical higher-order logic (since $\Omega$ is Boolean: $\Omega = \mathbf{2}$, with $\mathsf{true} \neq \mathsf{false}$ and $\mathsf{true} \lor \mathsf{false} = \mathsf{true}$, i.e., LEM holds).

**Presheaf toposes $\widehat{\mathcal{C}} = [\mathcal{C}^{op}, \mathbf{Set}]$.** For any small category $\mathcal{C}$:
- Terminal object: the constant functor at $\{*\}$
- Exponentials: $[F, G](c) = [\mathcal{C}^{op}, \mathbf{Set}](\mathbf{y}(c) \times F, G)$ (using Yoneda)
- Subobject classifier: $\Omega(c) = \{\text{sieves on } c\}$ (a sieve on $c$ is a collection of morphisms into $c$ closed under precomposition)

The internal logic of $\widehat{\mathcal{C}}$ is intuitionistic (LEM fails for non-trivial $\mathcal{C}$). This is the standard presheaf model of intuitionistic logic.

**Sheaf toposes $\mathbf{Sh}(X)$.** For a topological space $X$, sheaves on $X$:
- Objects: sheaves (presheaves satisfying the gluing axiom)
- $\Omega$: the sheaf of open subsets of $X$
- Internal logic: "local" reasoning about $X$

In $\mathbf{Sh}(X)$, a proposition can be "true on the open set $U$" — the truth value is an open set of $X$. This models local/geometric truth.

**The effective topos.** Hyland's effective topos has objects "realized by programs":
- Internal logic: constructive/computable mathematics
- LEM fails (there are undecidable propositions)
- Church's thesis holds internally (every function is computable)

Different toposes model different logical universes. This is the power of topos theory.

## The Internal Language of a Topos

Every topos has an *internal language* — a type theory or logical system whose models are the objects of the topos. The internal language of an elementary topos is a *higher-order intuitionistic type theory* (Mitchell-Bénabou language):

- Types: objects of $\mathcal{E}$
- Propositions about $A$: subobjects of $A$ (or equivalently, morphisms $A \to \Omega$)
- $\mathsf{true}$ and $\mathsf{false}$: the maps $\mathbf{1} \to \Omega$ (true = $\mathsf{true}$; false = the characteristic of $\emptyset$)
- Conjunction $P \land Q$: intersection of subobjects
- Disjunction $P \lor Q$: union of subobjects
- Implication $P \Rightarrow Q$: the exponential $P^Q$ in the poset of subobjects (internal hom)
- Universal quantification $\forall x : A, P(x)$: the image of $P \hookrightarrow A$ under $\Pi_{\pi_A}$
- Existential quantification $\exists x : A, P(x)$: the image of $P \hookrightarrow A$ under $\Sigma_{\pi_A}$

**Why intuitionistic?** In a general topos, $\Omega$ might not be Boolean. The negation $\neg P = P \Rightarrow \mathbf{false}$ might not satisfy $\neg\neg P = P$ (double negation elimination). So the internal logic is intuitionistic in general.

A topos satisfies LEM (law of excluded middle) internally if and only if $\Omega$ is a Boolean object ($P \lor \neg P = \mathsf{true}$ for all $P$). The topos $\mathbf{Set}$ satisfies LEM; most presheaf and sheaf toposes do not.

## The Diaconescu Theorem

**Theorem (Diaconescu 1975).** In the internal logic of any topos, the Axiom of Choice (every surjection splits) implies LEM.

This is a remarkable result: in a topos, classical logic is equivalent to the axiom of choice (in an appropriate internal sense). Constructive mathematicians who reject LEM must also reject AC.

In HoTT: the "external" axiom of choice (which says choice holds in $\mathbf{Set}$ externally) is fine. But the "internal" axiom of choice ($\prod_{x:A} \|B(x)\| \to \|\prod_{x:A} B(x)\|$) implies LEM in the presence of univalence. This is why HoTT is careful about the distinction between the computational axiom of choice (Section 3.1 of Chapter 8, which is always available) and the classical axiom of choice.

## Presheaf Models of Type Theory

Presheaf categories are particularly important for modeling type theory:

**The cubical model.** Cubical type theory (Chapter 23) is modeled in the presheaf category $[\Box^{op}, \mathbf{Set}]$ where $\Box$ is the *cube category* — a category whose objects are $\{0,1\}^n$ (the vertices of $n$-dimensional cubes) and morphisms are face maps and degeneracies.

In the cubical model:
- Types are presheaves (cubical sets)
- The identity type is the "path object" $A^{[0,1]}$ (the presheaf of "paths in $A$")
- Univalence holds by construction (the identity type of the universe has the right shape)
- Funext is provable without axioms

**The simplicial model.** Voevodsky's model lives in $[\Delta^{op}, \mathbf{Set}]$ (simplicial sets), the presheaf category over the simplex category $\Delta$. Here:
- Types are Kan simplicial sets (satisfying horn-filling conditions)
- The identity type is the simplicial path space
- Univalence is a theorem (by computing the identity type of the universe)

The simplicial model is the foundation of HoTT's consistency.

## Boolean Toposes and Classical Logic

A topos where $\Omega = \mathbf{2} = \{0, 1\}$ (the two-element Boolean algebra) is a *Boolean topos*. In a Boolean topos, the internal logic is classical:
- LEM holds: $P \lor \neg P = \mathsf{true}$ for all propositions $P$
- AC implies choice in the usual sense
- The power object $\mathcal{P}(A) = \Omega^A = 2^A$ is the set of subsets

$\mathbf{Set}$ is a Boolean topos. The internal logic of $\mathbf{Set}$ is classical higher-order logic — the logic of ZFC set theory.

Non-Boolean toposes model intuitionistic logic. Topos theory shows that constructive mathematics is not a restriction but a *generalization*: it works in all toposes, while classical mathematics only works in Boolean ones.

## Toposes and HoTT

The connection between toposes and HoTT is through *$\infty$-toposes*. An ordinary (1-categorical) topos models higher-order intuitionistic logic. An $\infty$-topos models HoTT:
- Objects are homotopy types (not just sets)
- The subobject classifier $\Omega$ becomes the "type of propositions" (h-propositions in HoTT)
- The "universe of types" $\mathsf{Type}$ is not the subobject classifier but a higher "object classifier"
- Univalence is the statement about the object classifier in an $\infty$-topos

The passage from 1-categorical toposes to $\infty$-toposes is the mathematical content of "doing homotopy theory internally" — replacing sets by homotopy types throughout.

Ordinary toposes (1-categorical) give models of HoTT where all types are 0-truncated (sets). $\infty$-toposes give models where types can have arbitrary homotopy structure. The slogan: toposes = constructive mathematics; $\infty$-toposes = constructive homotopy mathematics = HoTT.

## Summary

| Topos | Internal Logic | Key Feature |
|---|---|---|
| $\mathbf{Set}$ | Classical HOL | Boolean $\Omega$ |
| $\widehat{\mathcal{C}}$ (presheaves) | Intuitionistic HOL | Sieve-valued $\Omega$ |
| $\mathbf{Sh}(X)$ (sheaves) | Intuitionistic, local | Open-set $\Omega$ |
| Effective topos | Constructive/computable | Church's thesis holds |
| $\infty$-topos | HoTT | Homotopy types |

Topos theory unifies logic, set theory, and geometry into a single framework. The objects are spaces (not necessarily sets); the propositions are subobjects (not just subsets); the logic is intuitionistic (not necessarily classical). HoTT lives at the top of this hierarchy, where objects are full homotopy types.
