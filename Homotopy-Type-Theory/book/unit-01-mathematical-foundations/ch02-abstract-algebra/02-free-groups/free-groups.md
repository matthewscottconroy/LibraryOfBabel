# Free Groups

## Words and Reduction

Let S be a set, which we call the set of *generators*. A *word* over S is a finite sequence of *letters*, where each letter is either a generator s ∈ S or a *formal inverse* s⁻¹ for some s ∈ S. Examples with S = {a, b}:

- ε (the empty word)
- a
- ab
- a⁻¹ba
- aab⁻¹a⁻¹bb

A word is *reduced* if it contains no adjacent letters of the form ss⁻¹ or s⁻¹s — no generator immediately followed by its inverse, or vice versa. The words above are all reduced; the word aa⁻¹b is not (it reduces to b).

**Reduction.** Every word can be reduced to a unique reduced word by canceling adjacent inverses:
- aa⁻¹b → b
- ab⁻¹ba → aa (cancel b⁻¹b in the middle)
- aba⁻¹a → ab (cancel a⁻¹a)

The reduction process terminates (the word gets shorter with each cancellation) and the final result is independent of the order of cancellations (this requires a small proof, using the Church-Rosser property of the reduction system).

**Definition.** The *free group* F(S) on the generating set S is the set of all reduced words over S, with multiplication given by concatenation followed by reduction.

## The Group Laws

We verify that F(S) is indeed a group.

**Identity.** The empty word ε is the identity: concatenating ε with any word w gives w, and concatenation followed by reduction leaves w unchanged.

**Inverses.** The inverse of a word w = s₁^{ε₁} s₂^{ε₂} ... sₙ^{εₙ} (where εᵢ ∈ {+1,-1}) is the word sₙ^{-εₙ} ... s₂^{-ε₂} s₁^{-ε₁}: reverse the word and flip all exponents. Concatenating w with w⁻¹ gives a word that fully reduces to ε.

**Associativity.** Concatenation of words is associative on the nose (before reduction). After reduction, associativity holds because reduction is confluent.

**Example.** F({a}) = {ε, a, a⁻¹, a², a⁻², ...} ≅ ℤ. The word aⁿ corresponds to the integer n. Multiplication is addition. F on one generator is the integers.

**Example.** F({a, b}) is the *free group on two generators*: words like ε, a, b, ab, ba, a⁻¹, aba⁻¹, b²a⁻¹b³. This group is *infinite and non-abelian*: ab ≠ ba (neither can be reduced to the other). F₂ is one of the most important groups in mathematics — it is the fundamental group of the figure eight (two circles joined at a point).

## The Universal Property

Here is what makes free groups foundational: they are characterized, up to isomorphism, by a *universal property*.

**Theorem (Universal Property of Free Groups).** Let S be a set and F(S) the free group on S, with inclusion map i: S → F(S) (sending each generator to itself as a one-letter word). For any group G and any function f: S → G (not necessarily a homomorphism — just a function), there exists a *unique* group homomorphism φ: F(S) → G such that φ ∘ i = f (i.e., φ(s) = f(s) for each generator s ∈ S).

```
    i
S ----→ F(S)
 \       |
  f      | φ (unique homomorphism)
   \     |
    ↘   ↓
       G
```

*Proof.* Given f: S → G, define φ on a reduced word s₁^{ε₁} ... sₙ^{εₙ} by:
φ(s₁^{ε₁} ... sₙ^{εₙ}) = f(s₁)^{ε₁} · f(s₂)^{ε₂} · ... · f(sₙ)^{εₙ}

This is a homomorphism (concatenation of words maps to multiplication in G), and φ(s) = f(s) for s ∈ S. It is unique because a homomorphism from F(S) is determined by its values on the generators, and those values must equal f. □

**Consequence.** A homomorphism from F(S) to any group G is determined *entirely* by where the generators go. This is the "free" property: F(S) has no relations, so there are no constraints on where the generators are sent (beyond the group axioms in G).

This universal property is the right way to *define* the free group, not just to characterize it. In category theory, F(S) is the left adjoint to the forgetful functor from groups to sets: groups can be "forgotten" to their underlying sets, and F is the "best" way to turn a set into a group.

In HoTT, higher inductive types have exactly this structure: they are defined by their *introduction rules* (generators and path-constructors) and characterized by their *elimination rules* (universal properties). The free group F(S) is the HIT defined by generators S and path-constructors encoding the group axioms. The elimination rule is exactly the universal property above.

## Group Presentations

**Definition.** A *group presentation* ⟨S | R⟩ consists of a generating set S and a set R of *relations* — words in F(S) that we decree to be equal to the identity. The presented group is F(S)/N(R), where N(R) is the *normal closure* of R: the smallest normal subgroup containing all the words in R.

Elements of N(R) are products of conjugates of elements of R and their inverses.

**Examples:**

⟨a | aⁿ = e⟩ = ℤ/nℤ. The cyclic group of order n: one generator, one relation.

⟨a, b | ab = ba⟩ = ℤ × ℤ. The free abelian group on two generators: two generators with commutativity.

⟨r, s | rⁿ = e, s² = e, srs = r⁻¹⟩ = D_n. The dihedral group: a rotation r of order n, a reflection s of order 2, and the conjugation relation.

⟨a, b | ⟩ = F₂. No relations: the free group on two generators. The "most general" group with two generators.

**Every group has a presentation.** Take S = G (the group itself as generating set) and R = all products that are the identity in G. This is a valid but trivial presentation. Useful presentations are ones where S and R are small.

**Van Kampen's theorem.** This theorem computes the fundamental group of a pushout of spaces. Suppose X = A ∪_C B (where C = A ∩ B is the intersection). Then:

π₁(X) = π₁(A) *_{π₁(C)} π₁(B)

This is the *amalgamated free product*: take the free product of π₁(A) and π₁(B) (combine the generators), then impose the relations that the images of π₁(C) in π₁(A) and π₁(B) are equal.

**Example.** The figure eight is S¹ ∨ S¹ (the wedge sum of two circles, joined at one point). The intersection is a single point with trivial fundamental group. Van Kampen gives: π₁(S¹ ∨ S¹) = ℤ * ℤ = F₂. The free group on two generators.

This is the algebraic statement corresponding to the topological fact that the figure eight has two independent loops, with no relation between them.

## Free Groups and Equality in HoTT

The connection between free groups and HoTT is precise and deep.

In HoTT, the *circle* S¹ is defined as a higher inductive type:
```
data S¹ : Type where
  base : S¹
  loop : base = base
```
One point, one non-trivial loop. The identity type `base = base` — the type of all paths from base to base — is the loop space Ω(S¹, base).

The theorem π₁(S¹) ≅ ℤ is a major result in HoTT: the loop space of the circle is equivalent to the integers. Its proof uses the universal property of the circle type (which is its induction principle) exactly as the universal property of the free group on one generator identifies F({a}) with ℤ.

The wedge of two circles S¹ ∨ S¹ is similarly a HIT with two generators:
```
data WedgeCircles : Type where
  base : WedgeCircles
  loop₁ : base = base
  loop₂ : base = base
```
And π₁(S¹ ∨ S¹) ≅ F₂ (the free group on two generators).

The free group is not just a piece of algebraic theory we study before the real topic. It *is* the real topic, seen from the algebraic angle. Every HIT circle gives you a free group. Every presentation of a group corresponds to a CW complex. Every consequence of the universal property of free groups is a computation about paths and loops in HoTT.
