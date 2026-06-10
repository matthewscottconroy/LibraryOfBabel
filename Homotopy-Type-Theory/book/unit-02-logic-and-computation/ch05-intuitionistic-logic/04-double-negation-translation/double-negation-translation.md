# The Double-Negation Translation

## An Unexpected Containment

Classical and intuitionistic logic appear to be fundamentally different. Classical logic asserts LEM; intuitionistic logic rejects it. Classical logic treats double negation as trivially eliminable; intuitionistic logic does not. The logics prove different theorems.

And yet — this is a theorem, not an intuition — classical propositional logic is *interpretable* inside intuitionistic propositional logic. Every classical theorem translates into an intuitionistic theorem. The translation inserts double negations at strategic points, converting classical reasoning into constructive reasoning. And it does this honestly: the translated theorem is not a watered-down version of the original — it carries exactly the same classical content, expressed in a language that constructive logic can verify.

This is the *double-negation translation*, developed independently by Gödel (1933) and Gentzen (1933, published 1936). It reveals that the classical and intuitionistic systems are not as different as they appear: classical mathematics is *embeddable* in constructive mathematics, just with an altered interpretation of the logical connectives.

## The Translation: $\varphi^N$

The *Gödel-Gentzen translation* $\varphi^N$ of a formula $\varphi$ is defined recursively:

- $p^N = \neg\neg p$ for atomic propositions $p$
- $\bot^N = \bot$
- $\top^N = \top$
- $(A \wedge B)^N = A^N \wedge B^N$
- $(A \vee B)^N = \neg\neg(A^N \vee B^N)$
- $(A \to B)^N = A^N \to B^N$
- $(\neg A)^N = \neg A^N$
- $(\forall x, A)^N = \forall x, A^N$
- $(\exists x, A)^N = \neg\neg \exists x, A^N$

The key features: atomic propositions and conjunctions get double-negated, disjunctions and existentials are wrapped in a double negation, and implications and universal quantifiers are translated component-wise.

The double negations are placed exactly where classical logic would use LEM or DNE — at the points where a commitment (to which disjunct holds, to which witness exists) would be required constructively.

## The Main Theorem

**Theorem (Gödel-Gentzen, 1933).** For every formula $\varphi$:

$$\vdash_\text{CPC} \varphi \iff \vdash_\text{IPC} \varphi^N$$

*Part 1 (soundness of translation):* If $\varphi$ is classically provable, then $\varphi^N$ is intuitionistically provable.

*Proof sketch.* Show by induction that each classical rule, applied to translated premises, produces a translated conclusion that is intuitionistically provable. The key case is LEM: LEM for $\varphi$ translates to $\neg\neg(\varphi^N \vee \neg\varphi^N)$, which *is* intuitionistically provable (the stabilization of LEM: from any proof of $\neg(\varphi^N \vee \neg\varphi^N)$, derive $\bot$, and hence prove $\neg\neg(\varphi^N \vee \neg\varphi^N)$). All other classical rules translate directly to intuitionistic rules. $\square$

*Part 2 (faithfulness):* If $\varphi^N$ is intuitionistically provable, then $\varphi$ is classically provable.

*Proof.* One shows that $\vdash_\text{IPC} \varphi^N \to \varphi$ holds classically (by adding LEM), so if $\varphi^N$ is provable, $\varphi$ follows classically. More carefully: the translation $(\cdot)^N$ commutes with the classical rules in both directions. $\square$

## What the Translation Means

The Gödel-Gentzen theorem has several striking interpretations.

**Classical logic is inside intuitionistic logic.** Every classical theorem can be proved intuitionistically — just in translated form. Intuitionistic logic is not weaker than classical logic in any absolute sense. It is classically equivalent, just using a different interpretation of the connectives.

**The translations of classical theorems have constructive content.** When we translate a classical theorem $\varphi$ to $\varphi^N$ and prove it intuitionistically, we are not simply verifying classical logic. We are discovering a *constructive reading* of the classical result. The double negations track exactly where the classical proof would use non-constructive reasoning.

**The negative translation reveals $\neg\neg$-stable propositions.** A proposition $A$ is *$\neg\neg$-stable* if $\neg\neg A \to A$ holds (intuitionistically). The translation of a classical theorem is always a $\neg\neg$-stable proposition. This gives a criterion for when a classical result has constructive content: exactly when the $\neg\neg$-translated form is provable (which it always is) and when the $\neg\neg$-translation is intuitionistically equivalent to the original (which requires that the original is $\neg\neg$-stable).

## Which Propositions Are $\neg\neg$-Stable?

$\neg\neg$-stable propositions — those satisfying $\neg\neg A \to A$ — are exactly those for which the double-negation translation is transparent. They are the propositions whose truth can be determined once we know they're not disprovable.

Examples of $\neg\neg$-stable propositions:
- $\bot$: $\neg\neg\bot \to \bot$ is $\neg\bot \to \bot$, which is provable from $\neg\bot := \bot \to \bot$ (trivially), so yes.
- $\neg A$: negations are always $\neg\neg$-stable, since $\neg\neg\neg A \to \neg A$ (using the IPC theorem $\neg\neg\neg A \to \neg A$).
- Atomic propositions with decidable truth values.
- Propositions of the form $A = B$ in many type theories (when $A$ and $B$ have decidable equality).

Examples of propositions that are *not* $\neg\neg$-stable (in general):
- $A \vee B$: $\neg\neg(A \vee B) \to A \vee B$ fails (this would give LEM by taking $B = \neg A$).
- $\exists x, P(x)$: $\neg\neg \exists x, P(x) \to \exists x, P(x)$ fails (this would give the existence property for $\neg\neg$, which fails).

In HoTT, $\neg\neg$-stable propositions are closely related to *h-propositions* (Propositions in the sense of HoTT) — types that are "mere propositions" where all elements are propositionally equal. The $\neg\neg$-sheafification of a type corresponds to its propositional truncation $\|A\|_{-1}$.

## The Friedman A-Translation

The Gödel-Gentzen translation has a generalization due to Harvey Friedman (1978): the *A-translation*, which translates classical arithmetic into intuitionistic arithmetic *with an additional axiom* $A$.

**Definition.** For a fixed sentence $A$, the $A$-translation $\varphi^A$ is defined like the $\neg\neg$-translation but replacing $\bot$ with $A$ everywhere:

- $p^A = p \vee A$ for atomic $p$
- $\bot^A = A$
- $(B \to C)^A = B^A \to C^A$
- etc.

**Theorem (Friedman).** If $\varphi$ is provable in classical arithmetic, then $\varphi^A$ is provable in intuitionistic arithmetic for any $A$.

The significance: if we take $A$ to be some "oracle" or "additional axiom" of interest, the A-translation tells us that classical arithmetic is interpretable in intuitionistic arithmetic *plus* that axiom. This is the foundation of realizability semantics for classical arithmetic and the technique of *proof mining* — extracting computational content from classical proofs.

## Classical Logic as a Sheaf

The double-negation translation has a clean category-theoretic interpretation. Consider the category of propositions in IPC as a Heyting algebra $\mathcal{H}$. The double-negation operation $\neg\neg : \mathcal{H} \to \mathcal{H}$ is a *Lawvere-Tierney topology* on $\mathcal{H}$ — a closure operator satisfying certain axioms.

The $\neg\neg$-stable propositions form a *sublocale* (a quotient Heyting algebra) that is a *Boolean algebra* — the Boolean algebra of propositions that have definite classical truth values. Classical logic is the internal logic of this Boolean sublocale.

This means: inside the topos of IPC-valued propositions, there is a sub-topos whose internal logic is classical. The double-negation translation is the functor sending a classical proposition to its translation in this sub-topos.

In HoTT, this corresponds to the *propositional truncation* and the *law of excluded middle for mere propositions*: in HoTT, LEM for *h-propositions* is consistent (it does not affect the homotopy structure), and many classical results can be recovered by working in the sub-$\infty$-topos of $(-1)$-truncated types.

## Practical Consequences

The double-negation translation has practical consequences for mathematics and computer science:

**Program extraction from classical proofs.** A classical proof of $\exists x, P(x)$ translates to an intuitionistic proof of $\neg\neg \exists x, P^N(x)$. This is weaker than an outright witness — it does not directly give a program. But using additional techniques (like the "modified realizability" of Kreisel), one can often extract programs from classical proofs by analyzing the double-negation translation.

**The classical-constructive divide in practice.** When a classical proof translates to a $\neg\neg$-stable formula, it can often be made constructive directly, using the classical proof as a guide. The double-negation translation identifies the exact points where non-constructive reasoning is used and suggests where witnesses could be extracted.

**Formal verification.** Proof assistants like Isabelle/HOL work in classical logic. When a result proved in Isabelle is needed constructively (for program extraction in Coq or Agda), the double-negation translation provides a map: the Isabelle proof's structure guides where to insert explicit witnesses in the constructive proof.
