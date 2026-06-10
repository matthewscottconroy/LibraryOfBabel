# Chapter 8: Dependent Types

## The Function That Cannot Be Typed in Haskell

Here is a function that is impossible to type in Haskell or Java: a function that takes a natural number n and returns a list guaranteed to have exactly n elements. Not a list that might have n elements — a list where n is in the type itself, where a type-checking error catches the mistake of using a 5-element list where a 3-element list was required. This function has type Π(n:ℕ), Vec n. And Vec n is a type that depends on a value.

Once you allow that, everything changes.

Consider what it means. In Haskell, `[1,2,3]` has type `[Int]`. So does `[1]` and `[1,2,3,4,5]`. The type carries no information about how many elements there are. This is a design choice — a deliberate simplification — and it costs you something. When you write `zip :: [a] -> [b] -> [(a,b)]`, the type does not express the obvious fact that the two input lists should have the same length and the output list will too. You write it in a comment. The compiler ignores it. At runtime, something silently goes wrong.

With dependent types, you write: `zip : Π(n:ℕ). Vec A n → Vec B n → Vec (A × B) n`. The lengths are not a comment. They are the type. The compiler enforces them. A mismatch is a compile error, not a runtime surprise.

This chapter builds dependent type theory from scratch. We start with the most basic concept — a type that is a function of a value — and work up through the full machinery that makes formal mathematics possible.

## What You Will Find Here

**Section 1: Type Families.** A type family is simply a function B : A → Type. This is the key concept: something that takes a value and returns a type. The vector type Vec : ℕ → Type is the canonical example. We examine what type families are, how they interact with the rest of the type theory, and why they are the right generalization of both predicates in logic and fibrations in topology.

**Section 2: Π Types.** Given a type A and a type family B over A, the dependent function type Π(x:A).B(x) is the type of functions that map each a:A to an element of B(a). We develop the four rules — formation, introduction, elimination, computation (FIEC) — in full. We show that non-dependent function types A → B are a special case. We derive the polymorphic identity function and other essential Π-type inhabitants.

**Section 3: Σ Types.** The dependent pair type Σ(x:A).B(x) pairs a value a:A with an element of B(a). It is the type-theoretic rendering of existential quantification. We prove that the ordinary product A × B is a special case. We examine the projections fst and snd, the role of Σ types in encoding subsets and structures, and why the axiom of choice becomes a trivial theorem in MLTT.

**Section 4: Universes.** Types live somewhere. The type of all types — a universe — lets us quantify over types without writing infinitely many copies of every theorem. But Type : Type leads to contradiction (Girard's paradox). We develop the universe hierarchy Type₀ : Type₁ : Type₂ : ..., explain Russell-style and Tarski-style universes, and show why universe polymorphism is needed in practice.

**Section 5: Inductive Types.** The natural numbers ℕ, lists, vectors, booleans, the unit type, and the empty type are all inductive types. We develop the general pattern: an inductive type is specified by its constructors and its elimination principle. The eliminator for ℕ is exactly mathematical induction. We introduce W-types as the universal inductive type that subsumes all the others.

**Section 6: Propositions as Types, Revisited.** With the full machinery of dependent types available, we can now give a complete account of the Curry-Howard correspondence. The universal quantifier ∀x:A.P(x) is the Π type Π(x:A).P(x). The existential quantifier ∃x:A.P(x) is the Σ type Σ(x:A).P(x). We examine what it means for proofs to carry computational content, and why equality — when it becomes proof-relevant — is about to get very interesting.

## Why This Matters for HoTT

HoTT is built entirely within dependent type theory. Every construction — the identity type, the path space, the higher inductive types, univalence — is a type in the system we are building here. The universe is required to state univalence. The Π type is required to state function extensionality. The identity type (Chapter 9) is an inductive type. Without this chapter, there is no HoTT.

There is also a subtler point. Dependent type theory, at its core, treats propositions as types. A proof of P is an element of P. This is not merely a slogan; it is a precise formal identification. When equality becomes a type — when "a equals b" is not a judgment but a type whose elements are proofs — then the question "how many ways can a equal b?" becomes meaningful. The answer, in general, is: more than one. That is where HoTT begins.
