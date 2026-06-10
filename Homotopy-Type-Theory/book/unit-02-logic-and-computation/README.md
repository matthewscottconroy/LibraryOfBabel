# Unit 02: Logic and Computation

## The Question Nobody Asked

Here is a question that sounds like it belongs in a philosophy seminar, not a mathematics course: what is a proof?

Not "how do you write one" or "what makes one convincing." What *is* a proof — what kind of object is it, what structure does it have, and what does its internal anatomy tell us about the proposition it establishes?

For most of mathematics, this question is ignored. You prove theorems; they are either right or wrong; a proof is a convincing argument addressed to a community of peers. The structure of the proof itself, beyond its logical validity, is unimportant. Two different proofs of the same theorem are equally good proofs — they establish the same thing, and that is all that matters.

This unit is about why that last claim is false, and why recognizing its falsity unlocks the entire framework of Homotopy Type Theory.

## The Revolution in Four Chapters

The story this unit tells runs from 1935 to the present, through four mathematical systems that look, on the surface, like separate subjects: proof theory, intuitionistic logic, the Curry-Howard correspondence, and typed lambda calculi. They are not separate subjects. They are four angles on a single discovery: that proofs and programs are the same mathematical object.

**Chapter 4: Proof Theory.** In 1935, Gerhard Gentzen asked: can we give proofs a precise internal structure — not just "is this derivation valid" but "what is the shape of this derivation, and can it always be simplified?" His answer introduced natural deduction and sequent calculus, two formalisms for making proofs into explicit mathematical objects. His key theorem — that every proof can be reduced to a normal form with no redundant steps — is the first bridge between logic and computation. A proof in normal form is a proof in which nothing is done unless necessary. The process of normalization is, we will come to understand, exactly the process of running a program.

**Chapter 5: Intuitionistic Logic.** In the 1920s, L.E.J. Brouwer declared that classical logic was wrong — not wrong in some subtle philosophical sense, but actually incorrect in its treatment of existence and disjunction. A proof of "there exists an $x$ with property $P$" should *contain* the $x$, not just argue abstractly that one must exist. A proof of "$P$ or $Q$" should *specify which one* holds, not merely establish that at least one does. This constructive demand sounds restrictive, but it turns out to be liberating: constructive proofs carry computational content that classical proofs destroy. This is not a philosophical preference. It is the condition under which the Curry-Howard correspondence becomes a genuine identity, not merely an analogy.

**Chapter 6: The Curry-Howard Correspondence.** In 1958, Haskell Curry noticed that the axioms of combinatory logic had the same shapes as tautologies of propositional logic. In 1969, William Howard made this precise: natural deduction proofs and typed lambda terms are literally the same formal structures, described in different notation. Propositions are types. Proofs are programs. Proof normalization is computation. This is not an analogy or a metaphor. It is an identification — a demonstration that two mathematical theories, developed independently for entirely different purposes, are presentations of the same underlying mathematics. The Curry-Howard correspondence is why proof assistants exist, why type-checking is proof-checking, and why HoTT's identity types are simultaneously logical and topological objects.

**Chapter 7: STLC and System F.** The lambda calculus, without types, is a chaotic and inconsistent thing: every term can be applied to every other term, programs can loop forever, and there is no distinction between a valid proof and nonsense. Adding types restores order. The Simply Typed Lambda Calculus is the minimal typed system corresponding to propositional logic. System F extends it with polymorphism — quantification over types — corresponding to second-order logic. Reynolds' parametricity theorem shows that this quantification has remarkable consequences: the type of a polymorphic program tells you, for free, theorems about its behavior that you never had to prove. System F is the foundation of Haskell and ML, and the stepping stone to dependent types, which await in Unit 3.

## Why This Matters for HoTT

Homotopy Type Theory is a type theory. Its objects are types, its inhabitants are terms, and its fundamental operation is forming the identity type $a =_A b$ — the type of proofs that $a$ and $b$ are equal. To understand why identity types behave the way they do, why they have higher-dimensional structure, why univalence is a natural axiom rather than an arbitrary postulate — you need to understand the Curry-Howard correspondence at a deep level.

In HoTT, a proof of $p : a =_A b$ is itself a term with a type. It can be a witness that participates in further constructions. Two proofs of the same equality can be non-equal. A proof that two proofs of the same equality are equal is a term of type $p =_{a=_Ab} q$. This infinite tower — identity types all the way up — makes sense only if you have already internalized that proofs are objects, not mere certificates, and that two proofs of the same proposition can be genuinely different mathematical objects.

That understanding begins here.

## Prerequisites

Unit 01 (Foundations): comfort with set theory, first-order logic, and informal mathematical proof. You should be able to read and write proofs by induction, understand the meaning of quantifiers, and have some intuition for functions and relations. No prior exposure to type theory, lambda calculus, or formal proof systems is assumed.

## A Note on Notation

This unit uses mathematical notation that may be unfamiliar. We introduce it carefully. The key symbols: $\Gamma \vdash \varphi$ (context $\Gamma$ proves $\varphi$), $\lambda x. t$ (the function sending $x$ to $t$), $\Pi_{x:A} B(x)$ (dependent function type), $\Sigma_{x:A} B(x)$ (dependent pair type). If a symbol appears without introduction, that is an error in the text — please flag it.
