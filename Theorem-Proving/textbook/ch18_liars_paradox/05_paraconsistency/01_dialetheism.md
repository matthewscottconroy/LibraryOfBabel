# Dialetheism and Paraconsistency

What if the Liar is *both true and false*? Every other response spends heavily to avoid that verdict — hierarchies, gaps, context-shifts — and each, we saw, ends up unable to state its own theory. The dialetheist proposal: accept the contradiction, and change the logic so that one contradiction does not poison everything.

## Paraconsistency: Rejecting Explosion

Classical (and intuitionistic) logic validates **explosion** (*ex contradictione quodlibet*):
$$\phi, \neg\phi \vdash \psi \qquad \text{for arbitrary } \psi.$$
A classical derivation: from $\phi$ infer $\phi \vee \psi$ ($\vee$I); from $\phi \vee \psi$ and $\neg\phi$ infer $\psi$ (disjunctive syllogism). A consequence relation is **paraconsistent** iff explosion fails: some contradiction does not entail everything. Paraconsistency is a property of the *logic*; **dialetheism** — the thesis that some contradictions are actually *true* — is a stronger, philosophical claim. One can use paraconsistent logic merely as insulation against inconsistent data without believing any contradiction (and Section "Automated Reasoning" below is exactly that use).

## LP: The Logic of Paradox

Priest's **LP** (1979) has three values $\{\mathbf{T}, \mathbf{B}, \mathbf{F}\}$ — true only, both true and false, false only — with **designated** values $\mathcal{D} = \{\mathbf{T}, \mathbf{B}\}$: to be assertable is to be *at least* true, possibly false as well. The tables are exactly Strong Kleene's (Section 3) with $\mathbf{B}$ in the middle role; the entire difference from $K_3$ is that the middle value is designated.

| $\phi$ | $\neg\phi$ |
|:---:|:---:|
| $\mathbf{T}$ | $\mathbf{F}$ |
| $\mathbf{B}$ | $\mathbf{B}$ |
| $\mathbf{F}$ | $\mathbf{T}$ |

| $\wedge$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{T}$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |
| $\mathbf{B}$ | $\mathbf{B}$ | $\mathbf{B}$ | $\mathbf{F}$ |
| $\mathbf{F}$ | $\mathbf{F}$ | $\mathbf{F}$ | $\mathbf{F}$ |

| $\vee$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{T}$ | $\mathbf{T}$ | $\mathbf{T}$ | $\mathbf{T}$ |
| $\mathbf{B}$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{B}$ |
| $\mathbf{F}$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |

| $\to$ ($:= \neg\phi \vee \psi$) | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{T}$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{F}$ |
| $\mathbf{B}$ | $\mathbf{T}$ | $\mathbf{B}$ | $\mathbf{B}$ |
| $\mathbf{F}$ | $\mathbf{T}$ | $\mathbf{T}$ | $\mathbf{T}$ |

**Validity**: $\Gamma \models_{LP} \phi$ iff every valuation designating all of $\Gamma$ designates $\phi$ — preservation of designated value, not of $\mathbf{T}$.

**The Liar gets value $\mathbf{B}$.** Let $\lambda = \neg T(\ulcorner\lambda\urcorner)$, with the transparency constraint $v(T(\ulcorner\phi\urcorner)) = v(\phi)$. Setting $v(\lambda) = \mathbf{B}$ is coherent: then $v(T(\ulcorner\lambda\urcorner)) = \mathbf{B}$, so $v(\neg T(\ulcorner\lambda\urcorner)) = \neg\mathbf{B} = \mathbf{B} = v(\lambda)$ — the fixed-point equation is *satisfied*, not violated. The T-schema instance $T(\ulcorner\lambda\urcorner) \leftrightarrow \lambda$ evaluates to $(\mathbf{B} \to \mathbf{B}) \wedge (\mathbf{B} \to \mathbf{B}) = \mathbf{B}$: designated. Even $\lambda \leftrightarrow \neg\lambda$ is designated. The Liar is a **dialetheia** — an ordinary citizen of the language, both true and false, and nothing further follows from it. Formally this is Kripke's construction with gaps replaced by gluts: the same monotone machinery yields fixed points where $E$ and $A$ are permitted to *overlap*.

**Theorem (LP is paraconsistent: explosion fails).** $P, \neg P \not\models_{LP} Q$.

*Proof.* Countermodel: $v(P) = \mathbf{B}$, $v(Q) = \mathbf{F}$. Then $v(P) = \mathbf{B} \in \mathcal{D}$ and $v(\neg P) = \mathbf{B} \in \mathcal{D}$, but $v(Q) = \mathbf{F} \notin \mathcal{D}$. $\square$

**Theorem (material modus ponens fails in LP).** $P, P \to Q \not\models_{LP} Q$.

*Proof.* Same valuation: $v(P) = \mathbf{B}$, $v(Q) = \mathbf{F}$. Then $v(P \to Q) = v(\neg P \vee Q) = \max(\mathbf{B}, \mathbf{F}) = \mathbf{B} \in \mathcal{D}$, and $v(P) \in \mathcal{D}$, but $v(Q) \notin \mathcal{D}$. $\square$

The same countermodel kills disjunctive syllogism ($P \vee Q, \neg P \not\models_{LP} Q$) — which is precisely the step in the classical explosion derivation that LP rejects.

**What survives?** Everything, at the level of logical truth:

**Theorem.** $\phi$ is an LP-tautology iff $\phi$ is a classical tautology.

*Proof idea.* ($\Rightarrow$) Classical valuations are LP-valuations with no $\mathbf{B}$'s. ($\Leftarrow$) Given an LP-valuation $v$, define the classical $v_c(p) = \mathbf{T}$ iff $v(p) \in \{\mathbf{T},\mathbf{B}\}$. Induction on $\phi$ shows: $v_c(\phi) = \mathbf{T}$ implies $v(\phi) \in \{\mathbf{T},\mathbf{B}\}$, and $v_c(\phi) = \mathbf{F}$ implies $v(\phi) \in \{\mathbf{F},\mathbf{B}\}$ (with $\mathbf{T} > \mathbf{B} > \mathbf{F}$, $\wedge = \min$ and $\vee = \max$ make the steps mechanical). So if $\phi$ is classically valid, every LP-valuation designates it. $\square$

Thus excluded middle and even $\neg(\phi \wedge \neg\phi)$ are LP-valid — the law of non-contradiction is a *logical truth* of LP, some of whose instances are also false. What changes is the consequence relation: LP is classical logic minus the inferences (MP, DS, explosion) that leak through $\mathbf{B}$.

## The Curry Problem

Here is the deep constraint. LP's material-MP failure is what blocks Curry's paradox (Section 1): the derivation's detachment steps are simply invalid, so $\kappa$ is one more $\mathbf{B}$-valued oddity and $0=1$ is not forthcoming. But a logic whose conditional never detaches is crippled — ordinary deductive practice needs *some* $\to$ with $\phi, \phi \to \psi \vdash \psi$. Add a detachable conditional, and the Curry derivation runs again *unless contraction fails*: the culprit steps are $\to$I with a doubly-used assumption (equivalently the axiom $(\phi \to (\phi \to \psi)) \to (\phi \to \psi)$, or the structural rule of contraction). The design space is a trilemma: **detachment, contraction, unrestricted self-reference — any two, not all three** (keeping non-triviality). Priest's own conditional for dialetheic truth theory is detachable but contraction-free; substructural "Curry-paranoid" logics take the same exit. Note well: negation appears nowhere in this. Curry, not the Liar, is the hard boundary of the subject.

## Dialetheism and Its Discontents

**Dialetheism** (Priest, *In Contradiction*, 1987; Beall, *Spandrels of Truth*, 2009) holds that dialetheia are real but *quarantined*: the true contradictions arise only at the semantic (and perhaps set-theoretic) diagonal — Liar sentences, Russell sets — as inevitable by-products ("spandrels") of a transparent truth predicate. Empirical and mathematical discourse is assumed consistent, and **classical recapture** theorems back this up: over premise sets whose models can be taken minimally inconsistent, LP-consequence coincides with classical consequence (Priest's minimally inconsistent LP), so the dialetheist reasons classically everywhere except at the paradoxical fringe. The advertised payoff is Section 4's prize: expressive completeness with no revenge, since there is no forbidden classification whose expression would explode.

Objections. (1) **The "just true" problem**: the dialetheist seems unable to say that $\phi$ is true *and not also false* — any candidate ("$\phi$ is just true") might itself be a dialetheia; the classification "consistent" plays the role of everyone else's revenge-vocabulary. (2) **The exclusion problem**: asserting $\neg\phi$ no longer rules out $\phi$ (both may be designated), so what speech act *rejects* a claim? Priest answers with a primitive act of denial distinct from asserting the negation — critics reply that denial then does the exclusionary work negation was supposed to do, smuggling consistency back in.

**Beyond LP.** LP is one point in a large family. **Relevant logics** ($\mathbf{B}$, $\mathbf{R}$, $\mathbf{E}$; Anderson–Belnap) impose variable-sharing between premises and conclusion and are paraconsistent with a genuinely detachable (ternary-relation) conditional. **FDE** (first-degree entailment) adds a fourth value $\mathbf{N}$ (neither), combining gaps and gluts in one lattice. **da Costa's C-systems** ($C_1, C_2, \dots, C_\omega$) add a consistency operator $\phi^\circ$ ("$\phi$ behaves classically"), letting explosion apply exactly where consistency is asserted — the ancestor of modern *logics of formal inconsistency*.

## Paraconsistency in Automated Reasoning

Explosion is an engineering disaster wherever knowledge bases are large, multi-sourced, and imperfect: one contradictory pair anywhere — two sensors disagreeing, two regulations conflicting — and a classical reasoner will cheerfully derive every query. Paraconsistent consequence keeps inference *local* to consistent regions. The same imperative drives belief revision (AGM contraction before incorporation), paraconsistent description logics for ontology merging, and, in the SMT world, reasoning with soft constraints and MaxSAT/unsat-core extraction (Chapter 13): rather than deriving everything from an inconsistent specification, the solver isolates minimal conflicting subsets and reasons from the rest. Whether or not any contradiction is *true*, systems that must act on inconsistent information need logics in which contradictions do not explode — the Liar's most practical legacy.

## Exercises
See [problems/ch18_liars_paradox/](../../../problems/ch18_liars_paradox/)
