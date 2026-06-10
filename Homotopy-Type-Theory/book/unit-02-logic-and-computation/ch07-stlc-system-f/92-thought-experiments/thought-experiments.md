# Thought Experiments: STLC and System F

## 1. The Self-Application Problem

In the untyped lambda calculus, the term $\omega = \lambda x.\, x\, x$ is syntactically valid and semantically meaningful: it takes an argument and applies that argument to itself. The combination $\Omega = \omega\, \omega$ is the canonical infinite loop.

In STLC, $\omega$ cannot be typed: $x$ would need to have type $A \to B$ (to be applied) and also type $A$ (to be the argument), requiring $A = A \to B$, which has no finite solution in a simple type system.

Now ask: what if we *wanted* self-application? What if some computation naturally requires a function to operate on itself? Recursive programs require this: a function that calls itself is applying itself to arguments (in some sense). How does STLC handle this?

The answer is: STLC doesn't. There is no recursive function in STLC. All STLC programs terminate because STLC excludes self-reference. To get recursion back, we either:
(a) Add a fixed-point combinator $Y$ as a primitive (at which point STLC is no longer strongly normalizing), or
(b) Use inductive types with structural recursion (the approach of MLTT and proof assistants).

Option (b) preserves strong normalization because structural recursion requires the recursive call to be on a structurally smaller input — the recursion is guaranteed to terminate. This is how Coq, Agda, and Lean handle recursion: all recursive functions must be structurally recursive (or provably terminating by some other criterion).

The impossibility of self-application in STLC is not a bug — it is a feature. Self-application is exactly what creates non-termination, and non-termination is logical inconsistency. Types rule out the dangerous cases.

## 2. What Free Theorems Really Mean

Reynolds' parametricity theorem says: a term of type $\forall \alpha.\, \alpha \to \alpha$ must be the identity function. There is no other possibility.

But what does "must be" mean here? It means: in any model of System F satisfying the relational interpretation, the term is the identity. In the denotational model (sets and functions), the only element of the interpretation of $\forall \alpha.\, \alpha \to \alpha$ is the identity function. In the operational model (beta reduction), any term of this type reduces to the identity.

Now ask: is parametricity a constraint on programs, or a fact about programs?

It is both, in different senses. As a *language design* fact: in a well-designed polymorphic language, you cannot write a term of type $\forall \alpha.\, \alpha \to \alpha$ that is not the identity — the language rules it out. As a *semantic* fact: in the mathematical model of the language, the only element of that type is the identity.

Consider: what about a language that allows reflection — the ability to inspect the type at runtime? In such a language, you could write $\lambda \alpha.\, \lambda x : \alpha.\, \mathsf{if}\, (\alpha = \mathbb{N})\, \mathsf{then}\, 0\, \mathsf{else}\, x$. This has the "right" type $\forall \alpha.\, \alpha \to \alpha$ if you squint, but it is not the identity — for $\alpha = \mathbb{N}$, it returns $0$ instead of $x$.

Parametricity fails in the presence of type reflection. This is why Java's generics are not genuinely parametric (erasure at runtime, reflection available) while Haskell's polymorphism is parametric (types are erased, no runtime reflection). Haskell's free theorems hold; Java's don't in general.

What design choices preserve parametricity? And what is the relationship between parametricity and program security — does parametricity prevent certain information leaks?

## 3. The Logical Strength of System F

Girard's theorem says: a function $f : \mathbb{N} \to \mathbb{N}$ is definable in System F if and only if its totality is provable in second-order arithmetic. System F sits strictly between first-order arithmetic (Gödel's System T) and the full Calculus of Constructions.

This means System F can define the Ackermann function (which is not definable in first-order arithmetic) but cannot solve the halting problem (which is not solvable by any computable total function). System F occupies a precise position in the logical and computational hierarchy.

Now ask: what does the *boundary* look like? What function is just barely beyond System F's reach — the "simplest" function whose totality requires more than second-order arithmetic? And what does this boundary tell us about the relationship between logical strength and computational expressiveness?

The answer connects to ordinal analysis: System F corresponds to the proof-theoretic ordinal of second-order arithmetic (around $\Phi_1(0)$ in Veblen notation, or $\varepsilon_{\varepsilon_0}$ in some counting). Functions just beyond this boundary require principles provable in third-order arithmetic but not second-order.

Is there a natural mathematical function that witnesses this boundary — a function that is computable, provably total in third-order arithmetic, but not definable in System F? Paris-Harrington-style principles suggest such functions exist, but their explicit description is highly non-elementary.

## 4. Hindley-Milner and the LISP Revelation

In the 1950s, LISP was the first language to support a form of polymorphism: a list function like `car` (take the first element) worked on lists of any type. But LISP was dynamically typed — types were checked at runtime, not compile time. Type errors appeared only when a program was executed.

Hindley-Milner (Damas-Milner) type inference changed this: static type inference for polymorphic programs, without requiring type annotations. The programmer writes `fun x -> x` (the identity function in OCaml syntax) without specifying its type, and the type checker infers `'a -> 'a` (for all `'a`, take `'a`, return `'a`).

This is remarkable: the programmer's intended polymorphism is inferred mechanically. The type checker "discovers" that the function works for all types by observing that it makes no assumptions about the type of its argument.

Now ask: why does this work? What property of ML programs makes type inference complete? The answer is *rank-1 polymorphism*: in Hindley-Milner, $\forall$ quantifiers can only appear at the outermost level. A function can be polymorphic, but its arguments cannot themselves require polymorphic functions. Rank-1 restriction makes the unification problem underlying type inference decidable (and polynomial-time completable).

What fails for higher-rank polymorphism (rank-2 and above)? And why does Haskell require explicit type annotations for rank-2 polymorphism (`forall a. (forall b. b -> a) -> a`) while rank-1 is inferred? What does the lambda cube perspective tell us about where the undecidability boundary lies?

## 5. Type Erasure and the Meaning of Types

In most typed programming languages — Haskell, OCaml, Java (after compilation), Standard ML — the types are *erased* at runtime. The compiled program contains no type information. Types are purely a compile-time checking mechanism; they have no runtime cost.

This is the erasure principle: types are logical propositions (in the Curry-Howard sense), and once the proof (type-checking) is done, the proposition itself need not be carried around. The program is the proof term; the type is the proposition; and propositions-as-types says that once you have the proof, you have the evidence — you don't need to remember the statement.

But some type systems require *type evidence* at runtime: GADT pattern matching in Haskell, type class instances, or reified generics in Java (.NET). In these cases, the type information is part of the program and cannot be erased.

Under Curry-Howard: requiring type evidence at runtime corresponds to *proof-relevant* reasoning — the proof that a type satisfies a constraint is a computational object that must be explicitly manipulated. In HoTT, this is the norm: all proofs are terms, all equalities are paths, and identity proofs cannot be erased.

What does it mean for mathematics to be "proof-relevant" — to carry proofs as first-class objects rather than mere certificates to be discarded? This is the question HoTT answers, and the tension between erasure (classical proofs as certificates) and retention (constructive proofs as computations) is one of its central themes.

## 6. The Scott vs. Church Divide

Church encodings represent data by their *iteration* (or *fold*) principles: a natural number is "iterate a function $n$ times." Scott encodings represent data by their *pattern-matching* principles: a natural number is "am I zero, or am I a successor of something?"

These two representations have different computational properties:
- Church numerals support addition and multiplication efficiently (by composition of iteration), but predecessor is expensive ($O(n)$ time).
- Scott numerals support predecessor efficiently (constant time), but arithmetic is recursive and harder.

Under Curry-Howard, the Church numeral $\underline{n}$ has type $\forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$, which is the induction principle. The Scott numeral has a type corresponding to the *case analysis* principle. These are logically equivalent (both characterize natural numbers up to isomorphism), but computationally very different.

In HoTT, inductive types (like $\mathbb{N}$) come with both a recursion principle (Church-style) and a case analysis principle (Scott-style), unified into a single eliminator. The eliminator allows the return type to depend on the value being eliminated — giving dependent elimination that neither Church nor Scott encodings can express.

What does this tell us about the relationship between logical characterizations of data types and their computational efficiency? And why does dependent elimination require proper inductive types rather than lambda-encoded ones?

## 7. Parametricity and Relational Reasoning in HoTT

Reynolds' parametricity theorem is proved by a logical relations argument: for each type $A$, define a relation $\mathcal{R}(A)$ on pairs of elements, and prove that all terms satisfy the relational property.

In HoTT, the identity type $a =_A b$ provides a canonical notion of "being related" for any type $A$: elements $a$ and $b$ are related if $a =_A b$ is inhabited. Transport along paths gives a way to move evidence along equalities.

The connection: in HoTT, univalence implies a form of parametricity. Any type-theoretic construction that respects equivalences (homotopy invariant) is "parametric" in the homotopy-theoretic sense: it doesn't distinguish between equivalent types, only between genuinely inequivalent ones.

This suggests a deep connection between Reynolds' parametricity (a property of System F) and HoTT's univalence (a property of the identity type). Research by Nuyts, Vezzosi, and Devriese (among others) has begun to make this connection precise: parametricity and univalence are related via the theory of *modal type theory* and *relational models* of dependent types.

Does this mean parametricity is a consequence of univalence, or that univalence is a strengthening of parametricity? What is the precise relationship between being "uniform across type instantiations" (Reynolds) and "preserving type equivalences" (Voevodsky)?
