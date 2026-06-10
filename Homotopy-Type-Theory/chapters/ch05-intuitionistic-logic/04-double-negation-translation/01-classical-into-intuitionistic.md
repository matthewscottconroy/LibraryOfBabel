# 4.1 The Double-Negation Translation: Classical Inside Intuitionistic

## The Relationship Between Classical and Intuitionistic Logic

Classical and intuitionistic logic aren't two completely separate universes. Every theorem of IPC is a theorem of CPC (since IPC has fewer axioms). The question is: how do theorems of CPC relate to IPC?

The answer is elegant: every classical theorem can be "embedded" into intuitionistic logic by a *double-negation translation*. The translation transforms a classically provable formula into an intuitionistically provable formula that is equivalent over CPC.

This shows that classical mathematics is *conservative* over intuitionistic mathematics in a specific sense: classical theorems don't introduce new inconsistencies from an intuitionistic standpoint — they're just harder to interpret constructively.

## The Gödel-Gentzen Translation

**Definition.** The *Gödel-Gentzen translation* $\varphi \mapsto \varphi^\circ$ is defined by:
- $P^\circ = \neg\neg P$ for atomic propositions $P$
- $\bot^\circ = \bot$
- $(\varphi \wedge \psi)^\circ = \varphi^\circ \wedge \psi^\circ$
- $(\varphi \vee \psi)^\circ = \neg\neg(\varphi^\circ \vee \psi^\circ)$
- $(\varphi \to \psi)^\circ = \varphi^\circ \to \psi^\circ$
- $(\neg\varphi)^\circ = \neg\varphi^\circ$

**Theorem (Gödel 1933, Gentzen 1933).** CPC proves $\varphi$ if and only if IPC proves $\varphi^\circ$.

*Proof sketch.* Two directions:

($\Leftarrow$) Since IPC $\subseteq$ CPC and CPC proves $\neg\neg A \leftrightarrow A$ (by DNE), if IPC $\vdash \varphi^\circ$ then CPC $\vdash \varphi^\circ$ and classically $\varphi^\circ \leftrightarrow \varphi$, so CPC $\vdash \varphi$.

($\Rightarrow$) By induction on the classical proof. The key cases:
- LEM: $\neg\neg(A^\circ \vee \neg A^\circ)$ is provable in IPC (as we showed earlier — $\neg\neg(P \vee \neg P)$ is an IPC theorem).
- Modus ponens: if IPC proves $(\varphi \to \psi)^\circ = \varphi^\circ \to \psi^\circ$ and IPC proves $\varphi^\circ$, then IPC proves $\psi^\circ$. $\square$

## What the Translation Does

The translation inserts double negations at "classical points" — places where classical logic uses information that IPC doesn't have.

For atoms: $P^\circ = \neg\neg P$ says "it's not the case that $P$ has no proof." This is a weaker (but constructively available) surrogate for $P$.

For disjunction: $(\varphi \vee \psi)^\circ = \neg\neg(\varphi^\circ \vee \psi^\circ)$ — we can't assert which disjunct holds, but we can assert that not both can be false.

For implication: $(\varphi \to \psi)^\circ = \varphi^\circ \to \psi^\circ$ — no extra double negation needed! Functions (implications) are already constructive.

**The key insight:** Classical proofs can be constructivized *except* at "decision points" where you use LEM or DNE. The translation marks all such points with $\neg\neg$, making them constructively accessible (at the cost of weaker statements).

## Why Negations Don't Need Double Negation

Notice that $(\neg\varphi)^\circ = \neg\varphi^\circ$ — negations are not double-negated. This is because negation is $\varphi \to \bot$, and implication is preserved by the translation: $(\varphi \to \bot)^\circ = \varphi^\circ \to \bot^\circ = \varphi^\circ \to \bot = \neg\varphi^\circ$.

So negative statements translate cleanly. This is related to the fact that *negative* classical reasoning is more constructive than *positive* reasoning:
- "This algorithm doesn't solve the halting problem" is a clean negative statement, constructively accessible.
- "Some algorithm solves this problem" requires exhibiting the algorithm.

## The Kuroda Translation

Another translation, by Kuroda (1951), inserts $\neg\neg$ at every universally quantified formula and at every atomic subformula:

$$P^* = \neg\neg P$$
$$(\forall x, \varphi)^* = \forall x, \neg\neg\varphi^*$$
$$(other\ connectives)^* = \text{same recursion as Gödel-Gentzen}$$

This works better for first-order logic.

## The Friedman Translation

A more powerful translation for arithmetic uses Friedman's *A-translation* (1978), where each atomic formula $P$ is replaced by $P \vee A$ for a fixed sentence $A$. This allows transferring classical arithmetic proofs to constructive proofs with explicit witnesses, enabling the *proof mining* program in mathematical logic.

Proof mining: given a classical proof of $\forall x, \exists y, P(x, y)$ (a $\Pi_2$ statement), extract a computable function $f$ such that $P(x, f(x))$ holds constructively. This program has produced effective bounds in analysis, algebra, and combinatorics.

## Consistency Relative to IPC

An important corollary of the double-negation translation:

**Corollary.** CPC is consistent if IPC is consistent.

More precisely: if IPC is consistent (has no proof of $\bot$), then CPC is also consistent.

*Proof.* If CPC $\vdash \bot$, then by the translation, IPC $\vdash \bot^\circ = \bot$, contradicting consistency of IPC. $\square$

This shows classical and intuitionistic propositional logic have the same consistency strength.

For arithmetic: if Heyting Arithmetic (HA) is consistent, so is Peano Arithmetic (PA). And the converse also holds (obviously, since PA is stronger). So CPC and IPC are equiconsistent.

## What the Translation Tells Us About Classical Reasoning

The translation reveals exactly what makes classical reasoning "non-constructive":

1. **LEM** becomes $\neg\neg(P \vee \neg P)$ — constructively available, but only as a doubly-negated statement. Classical reasoning "decides" the disjunct; constructive reasoning only rules out both being false.

2. **Proof by contradiction** for positive conclusions: to prove $P$, assume $\neg P$ and derive $\bot$. Under the translation, this becomes a proof of $\neg\neg P$, not of $P$ itself. Classical logic silently applies DNE ($\neg\neg P \to P$) to complete the argument.

3. **Non-constructive existence proofs** like "some function with property $Q$ exists" (proven by assuming all functions fail $Q$ and deriving a contradiction) become proofs of $\neg\neg(\exists f, Q(f))$ — existence in the double-negation sense.

## The Double-Negation Monad

From a type-theoretic perspective, the double-negation translation defines a *monad* on the category of propositions:

- Unit: $P \to \neg\neg P$ (double negation introduction)
- Multiplication: $\neg\neg\neg\neg P \to \neg\neg P$ (follows from $\neg\neg\neg P \to \neg P$)
- Kleisli extension: from $P \to \neg\neg Q$, derive $\neg\neg P \to \neg\neg Q$

The double-negation monad "embeds" classical logic into intuitionistic logic as a "doubly-negated" fragment. Classical proofs live in the Kleisli category of this monad.

In homotopy type theory, the *propositional truncation* $\|A\|$ (also written $\|-\|_{-1}$, the "mere proposition" or "squash" type) plays a similar role: it strips off computational content and treats a type as a "bare proposition." Under this view:
- $\neg\neg A$ corresponds to "A is non-empty" (the double negation monad).
- $\|A\|$ corresponds to "A is inhabited" (propositional truncation).

In classical logic, these coincide. In HoTT, they may differ: $\|A\|$ has an elimination principle (you can eliminate into other mere propositions), while $\neg\neg A$ may not.

This relationship between the classical and constructive world, mediated by double negation and propositional truncation, is one of the subtle themes that will recur throughout the HoTT chapters.
