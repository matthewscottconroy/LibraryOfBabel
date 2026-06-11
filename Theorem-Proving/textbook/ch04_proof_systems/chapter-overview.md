# Chapter 4 Overview: Proof Systems

---

## Central Question

Given that we want to prove things, what is a *proof*? How do we make the notion of proof precise enough to study mathematically? And once we have done so, how do we know that our proof system proves the right things — no more, no less?

---

## Why This Chapter Matters

The choice of proof system is not merely a stylistic matter. Different proof systems have different computational behaviour, different proof sizes, different algorithmic properties, and different connections to computation. Natural deduction corresponds to lambda calculus (Curry-Howard); sequent calculus has the cut-elimination property that drives proof search; Hilbert systems are minimal in inference rules but hard to use for humans. Understanding all three, and their relationships, is essential for Chapter 11 (type theory) and Chapter 13 (formal verification).

---

## Three Proof Systems

### Natural Deduction (Gentzen 1935)

Natural deduction (ND) models how mathematicians actually reason: rules introduce and eliminate each connective. There are no axioms (or only minimal ones); everything is a rule of inference.

**Introduction and elimination rules for propositional connectives:**

Conjunction:
$$\frac{\phi \quad \psi}{\phi \land \psi}\land I \qquad \frac{\phi \land \psi}{\phi}\land E_1 \qquad \frac{\phi \land \psi}{\psi}\land E_2$$

Implication:
$$\frac{[\phi] \quad \vdots \quad \psi}{\phi \to \psi}\to I \qquad \frac{\phi \to \psi \quad \phi}{\psi}\to E\ (\text{modus ponens})$$

The $[\phi]$ notation indicates a *discharged hypothesis*: the rule $\to I$ allows us to assume $\phi$, derive $\psi$, and then conclude $\phi \to \psi$ while *cancelling* (discharging) the assumption of $\phi$.

Negation (classical):
$$\frac{[\neg\phi] \quad \vdots \quad \bot}{\phi}\text{RAA} \quad \text{(Reductio Ad Absurdum)}$$

Disjunction:
$$\frac{\phi}{\phi \lor \psi}\lor I_1 \qquad \frac{[\phi] \quad [\psi] \quad \phi \lor \psi}{\chi}\lor E$$

**For FOL, add:**

Universal quantifier:
$$\frac{\phi[t/x]}{\exists x\, \phi}\exists I \qquad \frac{\exists x\, \phi \quad [\phi[y/x]] \quad \vdots \quad \psi}{\psi}\exists E$$

(where $y$ is a fresh variable in $\exists E$)

$$\frac{\phi}{\forall x\, \phi}\forall I^* \qquad \frac{\forall x\, \phi}{\phi[t/x]}\forall E$$

($\forall I$ requires $x$ not free in any undischarged hypothesis)

### Sequent Calculus (Gentzen's LK, 1935)

A *sequent* is a pair $\Gamma \vdash \Delta$ where $\Gamma$ and $\Delta$ are finite sequences of formulas. Intuitively: "assuming all formulas in $\Gamma$, at least one formula in $\Delta$ holds."

The sequent calculus has structural rules (weakening, contraction, exchange), logical rules (introducing connectives on the left or right of $\vdash$), and the **cut rule**:

$$\frac{\Gamma \vdash \Delta, \phi \quad \phi, \Pi \vdash \Lambda}{\Gamma, \Pi \vdash \Delta, \Lambda}\text{Cut}$$

**Cut elimination theorem (Gentzen 1935).** Every proof with cuts can be transformed into a cut-free proof of the same sequent.

*Significance:* Cut elimination shows that the "detour" through an intermediate lemma $\phi$ can always be eliminated. It implies that sequent calculus proofs satisfy the *subformula property*: every formula appearing in a proof of $\Gamma \vdash \Delta$ is a subformula of some formula in $\Gamma \cup \Delta$. This drastically constrains proof search.

### Hilbert Systems

A Hilbert system consists of a small set of axioms (typically 3-4 schemas) plus a single rule of inference (modus ponens). For propositional logic, a standard set is:

- (H1) $\phi \to (\psi \to \phi)$
- (H2) $(\phi \to (\psi \to \chi)) \to ((\phi \to \psi) \to (\phi \to \chi))$
- (H3) $(\neg\phi \to \neg\psi) \to (\psi \to \phi)$

With modus ponens: $\frac{\phi \to \psi \quad \phi}{\psi}$.

Hilbert systems are minimal and conceptually clean, but practically difficult to use: proofs that are one line in natural deduction may require hundreds of steps in a Hilbert system. They are important for theoretical analysis (proving completeness, studying consistency) but not for proof assistants.

---

## Soundness and Completeness

**Soundness.** A proof system is sound for a semantics if: whenever $\Gamma \vdash \phi$, we have $\Gamma \vDash \phi$. 

*Proof for natural deduction (sketch).* By induction on proof structure. Each introduction rule is semantically sound (e.g., if $v(\phi)=1$ and $v(\psi)=1$ then $v(\phi \land \psi)=1$). Each elimination rule is semantically sound. The RAA rule is sound under the classical semantics (double negation elimination). $\square$

**Completeness.** A proof system is complete if: whenever $\Gamma \vDash \phi$, we have $\Gamma \vdash \phi$.

The completeness proof is deeper. The standard strategy is the **Henkin construction** (Henkin 1949):

1. Assume $\Gamma \not\vdash \phi$, i.e., $\Gamma \cup \{\neg\phi\}$ is consistent.
2. Extend $\Gamma \cup \{\neg\phi\}$ to a *maximal consistent set* (a consistent set not extendable while remaining consistent): use Zorn's Lemma or enumerate all sentences.
3. The maximal consistent set defines a "Henkin model" (the model's domain is the set of constant terms, with equality determined by the set).
4. Show $\Gamma \cup \{\neg\phi\}$ is satisfiable in this model, contradicting $\Gamma \vDash \phi$.

**Gödel's Completeness Theorem (1930)** states this for first-order logic: if $\Gamma \vDash \phi$, then $\Gamma \vdash \phi$ (in any sound proof system).

---

## Intuitionistic vs. Classical Logic

Classical logic validates the law of excluded middle (LEM): $\vDash \phi \lor \neg\phi$.

Intuitionistic (or constructive) logic does not. In natural deduction, removing the RAA rule and adding only the Intuitionistic Reductio:

$$\frac{[\phi] \quad \vdots \quad \bot}{\neg\phi}\neg I$$

gives *intuitionistic natural deduction*. This corresponds (via Curry-Howard) to the simply typed lambda calculus.

Many theorems valid classically fail intuitionistically:
- $\neg\neg\phi \to \phi$ (double negation elimination) — fails
- $\phi \lor \neg\phi$ (LEM) — fails
- $(\phi \to \psi) \to (\neg\phi \lor \psi)$ — fails

But: every classically provable formula is also provable intuitionistically after translating using the **double-negation translation** (Gödel-Gentzen translation): replace $\phi$ by $\neg\neg\phi$ and each atomic formula $p$ by $\neg\neg p$.

---

## Historical Context

**Gerhard Gentzen (1909–1945)** invented both natural deduction and the sequent calculus in his 1935 doctoral thesis. He also proved cut elimination and used it to prove the consistency of Peano arithmetic (relative to a transfinite induction principle). Gentzen's contributions are foundational for all of proof theory.

**David Hilbert (1862–1943)** proposed the Hilbert program: axiomatise all of mathematics in a formal system, then prove that system consistent by finitary means. Gödel's theorems (Chapter 10) showed this program cannot be completed as envisioned.

**Leon Henkin (1949)** gave the first completeness proof for type theory and the Henkin construction for FOL completeness that is now standard.

**William Howard (1969, circulated; published 1980)** identified the correspondence between natural deduction proofs and lambda calculus terms (the Curry-Howard correspondence, Chapter 11).

---

## Connections to Other Chapters

- **Chapter 2** uses propositional natural deduction as its proof system.
- **Chapter 5** presents proof strategies within the context of natural deduction.
- **Chapter 11** develops the Curry-Howard correspondence: proofs are programs, propositions are types.
- **Chapter 13** uses Lean 4 and Coq, which are based on dependent type theories that extend intuitionistic natural deduction.
