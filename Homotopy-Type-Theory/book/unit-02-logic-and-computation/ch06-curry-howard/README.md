# Chapter 6: The Curry-Howard Correspondence

## The Accident That Changed Everything

In 1958, Haskell Curry was not trying to connect logic and programming. He was studying the structure of combinatory logic — an algebraic system for representing functions — and he noticed something that puzzled him enough to write it down: the types of the basic combinators looked exactly like the axioms of implicational logic.

The identity combinator $I = \lambda x.\, x$ has type $A \to A$, corresponding to the axiom "A implies A." The constant combinator $K = \lambda x.\, \lambda y.\, x$ has type $A \to B \to A$, corresponding to the classical axiom "if A, then (if B, then A)." The composition combinator $S$ has type $(A \to B \to C) \to (A \to B) \to A \to C$, corresponding to a logical schema. These correspondences were too systematic to be accidents.

Curry noted this in a 1958 paper and did not pursue it further. He had no framework for thinking about it precisely.

In 1969, William Howard did. Howard was a logician, not a computer scientist, and he was thinking about the formal structure of natural deduction proofs. He noticed — working out Curry's observation in full generality — that the typing rules of the simply typed lambda calculus were *formally identical* to the natural deduction rules for intuitionistic logic. Not similar, not analogous. Identical in the sense that if you replace every type label with a proposition label, you get exactly the inference rules of Section 4.2.

Howard circulated a manuscript making this precise. The manuscript was not published until 1980, but it was widely read and is now famous.

The discovery is called the Curry-Howard correspondence, or the propositions-as-types correspondence, or (more dramatically) the Curry-Howard isomorphism. It states: intuitionistic natural deduction proofs and typed lambda terms are the same mathematical object, described in two different notations. A proof of $A$ in natural deduction is the same thing as a term of type $A$ in the simply typed lambda calculus. Proof normalization (removing detours) is the same thing as beta reduction (simplifying applications). Proof checking is the same thing as type checking.

This is not an analogy. It is an identity.

## Why This Is Not Obvious

The reason the Curry-Howard correspondence is surprising — why it is considered a major discovery rather than a trivial observation — is that logic and programming theory were developed independently, for entirely different purposes, by people who were not in communication with each other. Natural deduction was developed by Gentzen in 1935 to understand the structure of mathematical proofs. The lambda calculus was developed by Church in the 1930s to study computability. Typed lambda calculi were developed by computer scientists in the 1950s and 1960s to make programs safer. None of these researchers had the other application in mind.

The correspondence emerged anyway. This is the sign of a deep mathematical truth: that proofs and programs are not just analogous but genuinely the same kind of object, viewed from different perspectives.

## The Dictionary

The Curry-Howard correspondence sets up a dictionary:

| Logic | Type Theory |
|---|---|
| Proposition $A$ | Type $A$ |
| Proof of $A$ | Term $t : A$ |
| Hypotheses $\Gamma$ | Context $\Gamma$ (typed variables) |
| $A \wedge B$ | $A \times B$ (product type) |
| $A \vee B$ | $A + B$ (coproduct type) |
| $A \to B$ | $A \to B$ (function type) |
| $\bot$ | $\mathbf{0}$ (empty type) |
| $\top$ | $\mathbf{1}$ (unit type) |
| $\neg A$ | $A \to \mathbf{0}$ |
| $\forall x : A, B(x)$ | $\Pi_{x:A} B(x)$ (dependent product) |
| $\exists x : A, B(x)$ | $\Sigma_{x:A} B(x)$ (dependent sum) |
| Introduction rule | Constructor / $\lambda$-abstraction |
| Elimination rule | Eliminator / pattern-matching |
| $\beta$-reduction | Computation |
| Normal form | Value |
| Consistency | Strong normalization |

Each entry in the dictionary is not a loose correspondence — it is a formal theorem, provable by inspection of the inference rules.

## The Sections of This Chapter

**Section 1: Propositions as Types.** We work through the dictionary in detail, seeing each correspondence spelled out with explicit inference rules. We prove that the natural deduction rules and the typing rules are formally identical.

**Section 2: Simply Typed Lambda Calculus.** We develop the formal system on the programming side: STLC's syntax, typing rules, reduction rules, and the properties of well-typed terms. We prove the basic metatheorems: preservation (types are preserved by reduction) and progress (well-typed terms are not stuck).

**Section 3: Normalization and Consistency.** We prove strong normalization for STLC using logical relations, and derive the consistency of intuitionistic propositional logic as a corollary. This closes the loop: programs compute, proofs simplify, and neither can diverge.

**Section 4: Extending to Dependent Types.** The STLC-logic correspondence handles propositional logic. To handle predicate logic — with $\forall$ and $\exists$ — we need dependent types: types that depend on values. We preview $\Pi$ and $\Sigma$ types and the identity type.

**Section 5: Proof Assistants.** We see the correspondence in practice. Automath, Coq, Agda, Lean 4: each is an implementation of Curry-Howard at a different scale. What does it mean to write a formal proof? We look at the Flyspeck project, the Liquid Haskell verifier, and the industrial applications.

## The Connection to HoTT

The Curry-Howard correspondence is the conceptual foundation of HoTT. In HoTT, the dictionary continues:

| Proposition | Type |
|---|---|
| Proof of $a = b$ | Path $p : a =_A b$ |
| Proof of equality of proofs | Homotopy $h : p =_{a=_Ab} q$ |
| Equivalence $A \simeq B$ | Type $A \simeq B$ |
| $A \simeq B$ implies $A = B$ | Univalence axiom |

The identity type $a =_A b$ is the type of proofs that $a$ and $b$ are equal. A proof of this type is not just a certificate — it is a path in the space $A$, with its own internal structure. Two paths can be themselves equal (homotopic), giving a path-of-paths. This is the higher-dimensional extension of Curry-Howard, and it is the heart of HoTT.

Everything in this chapter is preparation for that extension.
