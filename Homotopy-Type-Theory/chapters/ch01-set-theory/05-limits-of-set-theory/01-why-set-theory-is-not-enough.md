# 5.1 Why Set Theory Is Not Enough

## The Success Story

ZFC is an extraordinary achievement. Starting from a small collection of axioms, it can encode essentially all of classical mathematics:
- Number theory (arithmetic, primes, Diophantine equations)
- Analysis (real numbers, limits, derivatives, integration)
- Algebra (groups, rings, fields, modules)
- Geometry (Euclidean and non-Euclidean)
- Topology (general, algebraic, differential)
- Probability theory
- Combinatorics and graph theory

This universality is not trivial. For most of the 19th century, different branches of mathematics had their own foundations. Set theory unified them under a single language and axiom system.

So why isn't ZFC the end of the story?

## Problem 1: The Identity Problem

**The issue.** In ZFC, two objects are equal if and only if they are the same set. But mathematicians routinely work with objects that are "the same" in a weaker sense: *isomorphic*.

- The cyclic group $\mathbb{Z}/2\mathbb{Z}$ and the group $(\{0,1\}, \oplus)$ (XOR) are isomorphic groups. Mathematicians treat them as "the same group" — they have identical algebraic properties, and every statement in the language of group theory is true of one iff it's true of the other. But in ZFC, they are different sets.

- The real numbers constructed as Dedekind cuts and as equivalence classes of Cauchy sequences are different sets. Yet mathematicians use either construction interchangeably.

- When we say "the fundamental group of the circle is $\mathbb{Z}$," we mean it is *isomorphic* to $\mathbb{Z}$ — not that it *is* $\mathbb{Z}$ (which would require specifying which $\mathbb{Z}$ and what the isomorphism is).

ZFC handles this by convention: mathematicians agree to treat isomorphic structures as interchangeable, without this being formalized in the logic.

**The type-theoretic resolution.** The *Univalence Axiom* in HoTT says: for types $A$ and $B$, the type $A = B$ (proofs that $A$ equals $B$) is equivalent to the type $A \simeq B$ (type equivalences, i.e., bijections respecting structure). Isomorphic mathematical objects are literally equal.

This makes the mathematical practice of "treating isomorphic objects as the same" into a formal theorem. It resolves the identity problem by design.

## Problem 2: Computational Content

**The issue.** A proof in ZFC establishes that something is true, but it may not give any algorithm for computing it.

*Example 1:* The Axiom of Choice proves that every surjection $f : A \twoheadrightarrow B$ has a right inverse (a function $g : B \to A$ with $f(g(b)) = b$). But the proof gives no algorithm for finding $g$.

*Example 2:* Brouwer's fixed-point theorem (a continuous function $f : D^n \to D^n$ on the $n$-ball has a fixed point) is provable in ZFC but the proof is non-constructive. Given $f$, there is no algorithm that always computes the fixed point.

*Example 3:* The completeness theorem for first-order logic says that any consistent theory has a model. The proof uses Zorn's Lemma (equivalent to AC) in an essential way, and the model constructed has no computable description in general.

For the foundations of computer science, this is a problem. We want proofs to have *computational content*: a proof of $\exists n, P(n)$ should give an algorithm for computing $n$. ZFC makes no such promise.

**The type-theoretic resolution.** In dependent type theory, the Curry-Howard correspondence says: *proofs are programs, propositions are types*. A proof of $\exists n, P(n)$ is a term of the type $\Sigma_{n:\mathbb{N}} P(n)$ — a pair of a witness $n$ and a proof that $P(n)$ holds. The witness is computationally accessible.

This makes every constructive proof into an algorithm. Proof assistants like Lean can extract runnable programs from proofs.

## Problem 3: Type Safety

**The issue.** In ZFC, everything is a set, and the membership relation $\in$ is defined for all pairs. This means questions like:

- "Is the number 2 a subset of the sine function?"
- "Does the integer 5 belong to the group $S_3$?"
- "Is $\pi$ a member of the Euclidean plane?"

are syntactically well-formed in ZFC (they have truth values — all false in the standard encoding). But they are *semantically nonsensical* — they mix apples and oranges.

The fact that ZFC assigns truth values to these questions is not a feature but a defect. It means the formalism cannot detect "category errors" that any mathematician would immediately flag as absurd.

**The type-theoretic resolution.** Type theory introduces *types*, which are the fundamental ontological category. Every term has a unique type, and operations only apply to terms of appropriate types. The question "does the integer 5 belong to the group $S_3$?" cannot even be *written* in type theory — "belongs to" (set membership) only makes sense when the left-hand side has the element type of the right-hand side.

Types provide a static discipline that catches category errors before any reasoning begins.

## Problem 4: The Formalization Gap

**The issue.** There is a vast gap between "informal mathematical proof" and "ZFC-formal proof." When a mathematician writes a proof in a journal, it is technically a proof in ZFC, but:
- The encoding of mathematical objects as sets is left implicit.
- Many steps that formally require invoking axioms are left as "clearly" or "obviously."
- The isomorphism conventions are used freely without justification.

The formal ZFC proof of a theorem might be thousands of times longer than the informal proof. This gap means that "mathematically verified" and "formally verified" are very different things.

**The type-theoretic resolution.** Proof assistants based on type theory (Lean 4, Coq, Agda) have a much smaller gap between informal and formal proof. The type system enforces type safety automatically. Isomorphism is handled by Univalence. The notation can be made to look very close to standard mathematical notation.

Formalizing mathematics in Lean is not easy, but it is significantly less painful than formalizing in ZFC-as-a-proof-assistant.

## What Set Theory Gets Right

Before finishing, it's worth acknowledging what set theory does well, to avoid caricature:

1. **Classical mathematics works perfectly well in ZFC.** The vast majority of published mathematics can be formalized in ZFC, and the formalization is usually straightforward (if verbose).

2. **ZFC is extremely well-studied.** Independence results, consistency strengths, forcing — the mathematical logic of set theory is rich and well-understood.

3. **Many mathematicians don't need the alternatives.** If you're not interested in computational content, don't care about foundations, and are happy with the isomorphism conventions, ZFC causes no problems.

4. **Separation of concerns.** Using ZFC as a foundation separates "what's true" from "how to compute it." This can be useful when you're doing existence proofs and don't care about algorithms.

## The Transition

The rest of this curriculum develops the type-theoretic alternative:
- **Chapter 5:** Intuitionistic logic — what changes when you don't assume excluded middle.
- **Chapter 6:** The Curry-Howard correspondence — proofs as programs.
- **Chapter 8:** Dependent types — the basic machinery of type theory.
- **Chapter 9:** Martin-Löf Type Theory — the foundation underlying Lean and Agda.
- **Chapter 16:** Identity types — the type-theoretic replacement for set-theoretic equality.
- **Chapter 18:** The Univalence Axiom — isomorphism as equality.

Each of these chapters addresses one or more of the problems identified here. By the end of the curriculum, you will have a complete picture of why the type-theoretic approach to foundations is not just different but genuinely *better* for the goals of this curriculum.
