# 2.1 Intuitionistic Propositional Calculus and Its Properties

## Formalizing Intuitionistic Logic

We've seen the BHK interpretation informally. Now we formalize it.

**Intuitionistic Propositional Calculus (IPC)** is the natural deduction system from Chapter 4, restricted to the following rules (no classical axioms like DNE or LEM):
- Introduction and elimination rules for $\wedge$, $\vee$, $\to$, $\bot$ (as in Section 4.2)
- No double negation elimination ($\neg\neg A \to A$)
- No law of excluded middle ($A \vee \neg A$)
- No Peirce's law ($((A \to B) \to A) \to A$)

Alternatively, IPC has a Hilbert-style axiomatization with axiom schemes and modus ponens. Here are the axioms (you can check each has a BHK proof):

1. $A \to (B \to A)$ — if you have $A$, you can trivially construct $B \to A$ (ignore $B$)
2. $(A \to (B \to C)) \to ((A \to B) \to (A \to C))$ — distribution of implication
3. $A \wedge B \to A$ — first projection
4. $A \wedge B \to B$ — second projection
5. $A \to (B \to A \wedge B)$ — pairing
6. $A \to A \vee B$ — left injection
7. $B \to A \vee B$ — right injection
8. $(A \to C) \to (B \to C) \to (A \vee B \to C)$ — case analysis
9. $\bot \to A$ — ex falso (from a proof of $\bot$, anything follows)

**Note:** $\neg A$ is defined as $A \to \bot$.

To get *classical* propositional logic (CPC), add any one of:
- DNE: $\neg\neg A \to A$
- LEM: $A \vee \neg A$
- Peirce: $((A \to B) \to A) \to A$

These are classically equivalent (each implies the others over IPC), but none is derivable in IPC.

## Theorems of IPC (not in CPC)

Some key theorems hold in IPC but have different status in CPC:

**Theorems of IPC:**
- $A \to \neg\neg A$ (double negation introduction)
- $\neg\neg\neg A \leftrightarrow \neg A$
- $\neg(A \vee B) \leftrightarrow \neg A \wedge \neg B$ (De Morgan)
- $\neg A \vee \neg B \to \neg(A \wedge B)$ (but not the converse)
- $\neg\neg(A \vee \neg A)$ (doubly negated LEM)
- $(A \to B) \to (\neg B \to \neg A)$ (contrapositive)

**NOT theorems of IPC (but theorems of CPC):**
- $\neg\neg A \to A$ (double negation elimination)
- $A \vee \neg A$ (excluded middle)
- $\neg(A \wedge B) \to \neg A \vee \neg B$ (strong De Morgan)
- $(\neg B \to \neg A) \to (A \to B)$ (inverse contrapositive)
- $(A \to B) \vee (B \to A)$ — this classical tautology fails in many Kripke models

The contrasts illuminate what classical reasoning actually does: it fills in missing information by deciding each proposition one way or the other.

## The Disjunction Property

The most important property of IPC is:

**Theorem (Disjunction Property).** If IPC proves $\varphi \vee \psi$, then IPC proves $\varphi$ or IPC proves $\psi$.

This fails for CPC (classical propositional logic): CPC proves $A \vee \neg A$ without proving $A$ or proving $\neg A$.

**Proof.** We prove this using Kripke semantics (which we'll develop in Section 3). The key idea is to exhibit a Kripke frame where $A$ and $\neg A$ are both "undecided" — not forced at the initial world. Then $A \vee \neg A$ is not forced, so it cannot be a theorem (by soundness). More generally, for any pair $\varphi, \psi$: if neither is a theorem, we can construct a Kripke model where both are "undecided," so their disjunction fails, and hence the disjunction is not a theorem of IPC.

The formal proof uses model theory. Completeness gives: if IPC does not prove $\varphi$, there is a Kripke model where $\varphi$ fails. If IPC does not prove $\psi$, there's another Kripke model where $\psi$ fails. Combining these models gives one where both fail (taking a "disjoint sum" of the models, or more carefully, a product of the Kripke frames). In this model, $\varphi \vee \psi$ fails. So IPC does not prove $\varphi \vee \psi$. $\square$

**Why this matters.** The disjunction property means IPC is *constructive* in a strong sense: when you prove a disjunction, you know *which side you're on*. This is exactly the BHK interpretation.

For type theory: the disjunction property corresponds to *canonicity* for coproduct types: every closed term of a sum type $A + B$ reduces to either $\mathsf{inl}(t)$ for some $t : A$ or $\mathsf{inr}(s)$ for some $s : B$.

## The Existence Property

For first-order intuitionistic arithmetic (HA, Heyting Arithmetic):

**Theorem (Existence Property).** If HA proves $\exists n : \mathbb{N}, P(n)$, then there exists a numeral $\bar{k}$ such that HA proves $P(\bar{k})$.

This fails for classical arithmetic (PA). One can prove $\exists n, (n = 0 \vee n = 1)$ by taking $n = 0$ — this is specific. A harder example: PA proves $\exists n \in \{0, 1\}, \text{GoldbachFails}(n) \vee \neg\text{GoldbachFails}(n)$ trivially (by LEM), but no specific $n$ is determined by the proof structure.

**For type theory:** Canonicity for $\Sigma$-types (dependent sums): every closed term of $\Sigma_{n:\mathbb{N}} P(n)$ reduces to a pair $(\bar{k}, p)$ for some specific numeral $\bar{k}$ and proof $p : P(\bar{k})$.

## The Relationship Between IPC and CPC

IPC and CPC are closely related but different. Here's how they compare:

**Provability comparison:**
- Every theorem of IPC is a theorem of CPC (IPC $\subseteq$ CPC).
- Some theorems of CPC are not theorems of IPC (CPC $\supsetneq$ IPC).
- IPC and CPC prove the same *negations* of atomic formulas.

**The key missing link** is the classical principles (LEM, DNE, etc.), which IPC lacks. Everything else is the same.

**Double negation translation (Gödel-Gentzen):** CPC proves $\varphi$ if and only if IPC proves $\varphi^\circ$ (where $^\circ$ is the double-negation translation). So classical logic can be "simulated" inside intuitionistic logic by double-negating everything. This is why classical mathematics is "safe" from an intuitionistic standpoint — the classical results don't introduce any inconsistency; they're just not all accessible without the extra principles.

## Decidability in IPC

**Theorem.** IPC is decidable: there is an algorithm that, given a propositional formula $\varphi$, decides whether IPC $\vdash \varphi$.

*Proof.* By completeness with respect to Kripke semantics: IPC $\vdash \varphi$ iff $\varphi$ is valid in all finite Kripke models. Finite Kripke models are finite, so we can check validity by enumeration. (More precisely: for a formula with $n$ propositional variables, we need only check Kripke frames of size at most $2^n$ or so — there's a bound.) $\square$

This is in contrast to first-order intuitionistic logic: the first-order case is undecidable (by reduction from the halting problem), just as classical first-order logic is undecidable. The decidability of the propositional fragment is a nice feature.

**Complexity.** Deciding IPC provability is PSPACE-complete — the same complexity as classical propositional logic (SAT). This might be surprising, given that IPC seems "harder" (more restrictive). But the complexity is the same; what differs is which formulas are provable, not the difficulty of deciding.

## A Deeper Look: The Propositions-as-Types Lens

All of these properties of IPC — disjunction property, existence property, decidability — have elegant explanations when viewed through the Curry-Howard lens:

- **Disjunction property** = canonicity for coproducts
- **Existence property** = canonicity for $\Sigma$-types
- **Decidability of propositional IPC** = decidability of type checking for simply typed $\lambda$-calculus
- **Strong normalization for IPC** = strong normalization for STLC

The correspondence between logical and computational properties is not coincidental — they're the same theorem, viewed from different angles. This theme will recur throughout the curriculum.
