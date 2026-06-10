# Thought Experiments: The Curry-Howard Correspondence

## 1. The Proof That Is Also a Specification

In Lean 4 or Coq, a theorem statement like `∀ n : ℕ, n + 0 = n` is literally a type, and its proof is literally a term of that type. The proof might look like an induction that produces, for each $n$, a path $p_n : n + 0 = n$.

Now consider: what is the difference between writing a *specification* and writing a *theorem*? In software engineering, a specification is a formal statement of what a program should do. In mathematics, a theorem is a proposition to be proved. Under Curry-Howard, these are the same thing.

If you write a function with type signature `sort : List ℕ → List ℕ` and a proof `∀ l : List ℕ, sorted (sort l) ∧ permutation l (sort l)`, you have simultaneously written a specification (the proposition expressed by the type) and committed to providing a proof (a term of that type, which is the algorithm).

What changes about software development if every function is simultaneously a specification and an implementation, and "testing" is replaced by "type checking"? What are the limits of this approach? What kinds of correctness properties can be expressed as types, and which cannot?

## 2. The Classical Program

The Curry-Howard correspondence for classical logic maps classical proofs to programs with continuations. Specifically, Peirce's law $((A \to B) \to A) \to A$ corresponds to call/cc: a function that takes a "potential continuation" and uses it to jump out of the current context with a value.

Consider: what does it mean to "run" a classical proof? In intuitionistic logic, running a proof (normalizing it) produces a value directly. In classical logic with continuations, running a proof can "escape" the normal evaluation context — think of exceptions in programming, where an exception thrown in a nested function call unwinds the call stack back to a handler.

A classical proof of "there exists an $n$ with property $P$" (using LEM) would, when "executed," search for an $n$ by trying both branches of the disjunction ($P(n)$ or $\neg P(n)$) and "throwing" the witness when found. The computation might not return in the normal sense — it might jump to a continuation.

Is this a reasonable computational interpretation of "existence"? Does it give you an algorithm for finding $n$? Or does it give you something more like a proof of existence without an explicit witness?

## 3. Types as Propositions, All the Way Down

If propositions are types, what about *propositions about propositions*? In propositional logic, we have propositions and proofs. In predicate logic, propositions can quantify over objects. In second-order logic, propositions can quantify over propositions.

Under Curry-Howard, second-order logic corresponds to System F: universal quantification over types. A polymorphic identity function $\Lambda \alpha.\, \lambda x : \alpha.\, x$ has type $\forall \alpha.\, \alpha \to \alpha$, which under Curry-Howard is the second-order tautology $\forall P.\, P \to P$.

But what about propositions about proofs of propositions — meta-level reasoning about the proof system itself? In HoTT, this is captured by the identity type: $p : a =_A b$ is a proposition about the elements $a, b : A$, and if $A$ is itself a type of proofs, then $p$ is a proposition about proofs. The type $p =_{a =_A b} q$ is a proposition about propositions about proofs — a second-level equality.

How far does this tower go? In MLTT with universes, the tower is infinite: $\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \ldots$. Each universe is a "type of types" at the next level. What does it mean for there to be propositions all the way up? Is there a "top" to the tower, or is mathematics irreducibly infinite in its foundational depth?

## 4. The Extracted Program and the Proof

Coq can extract an OCaml program from a constructive proof. The proof of $\forall n : \mathbb{N}, \exists m : \mathbb{N}, m > n$ extracts to a function `find_larger : int -> int`. The proof says such a function exists; the extraction gives you the function.

Now consider: two different constructive proofs of the same theorem extract to different programs. The proof that uses $m = n + 1$ extracts to `fun n -> n + 1`. A more complex proof that uses $m = n + 2$ (while still being a valid proof of the theorem) extracts to `fun n -> n + 2`. Both programs are correct — they satisfy the specification — but they compute different values.

This shows that the proof itself, not just the theorem, carries computational content. Different proofs of the same theorem can produce different algorithms with different efficiency properties.

Now: in classical mathematics, two proofs of the same theorem are considered equivalent (they establish the same thing). Under Curry-Howard, they are genuinely different programs. Does this mean that, from a mathematical perspective, classical mathematics *wastes* information by identifying proofs that compute differently? Is proof relevance (distinguishing different proofs of the same theorem) a mathematical virtue, not just a type-theoretic technicality?

## 5. The Consistency of Mathematics as a Termination Property

The Curry-Howard correspondence converts the logical consistency of type theory into a termination property: the system is consistent if and only if all well-typed terms terminate.

This gives a startling reframing of the foundational question "is mathematics consistent?" as a computational question: "do all programs in this type theory terminate?"

The consistency of Peano Arithmetic (proved by Gentzen, using transfinite induction up to $\varepsilon_0$) corresponds, under Curry-Howard, to the statement that all programs in Gödel's System T (the type theory corresponding to Peano Arithmetic) terminate. This is true, and the termination proof uses the same ordinal analysis that Gentzen used.

The consistency of ZFC set theory — the standard foundation for mathematics — would correspond to the termination of all programs in some type theory corresponding to ZFC. We do not have such a type theory fully worked out, partly because ZFC contains non-constructive principles that lack clean computational interpretations.

Does this framing suggest that the question "is mathematics consistent?" might be better understood as "does computation, properly constrained, always terminate?" And what would it mean for mathematics to be *inconsistent* from a computational perspective?

## 6. Leibniz's Dream and Type Theory

Leibniz imagined a *characteristica universalis* — a universal formal language in which all knowledge could be expressed — and a *calculus ratiocinator* — a mechanical procedure for determining the truth of any statement in that language. With these tools, disputes about truth would be resolved not by argument but by computation: "let us calculate."

The Curry-Howard correspondence is one of the closest realizations of Leibniz's dream. A proposition is a type; a proof is a term; verification is type checking; and type checking is a decidable computation.

But Gödel's theorems show the dream cannot be fully realized: there is no complete and consistent formal system for all of mathematics. Any sufficiently powerful system either has unprovable truths or has provable falsehoods.

Under Curry-Howard, Gödel's first incompleteness theorem becomes: there are types in sufficiently powerful type theories that are inhabited (have terms) but for which no such term can be exhibited by any formal procedure. And the second incompleteness theorem becomes: a sufficiently powerful type theory cannot prove its own strong normalization (termination).

How does the Curry-Howard perspective change the significance of Gödel's theorems? Are they limitations of mathematics, or revelations about the nature of mathematical truth and computation?
