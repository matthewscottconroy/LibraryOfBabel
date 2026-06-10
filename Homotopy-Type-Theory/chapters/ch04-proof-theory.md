# Chapter 4: Proof Theory — The Structure of Formal Derivation

## Introduction

Proof theory is the mathematical study of *proofs as objects*. Rather than asking "what is true?" it asks "how is it proved?" and "what structure does the proof have?" This shift of perspective — from semantics (meaning) to syntax (form) — is one of the most productive moves in modern logic.

For our curriculum, proof theory is essential for three reasons. First, it makes precise the rules of inference that type theory will later inherit. Second, the phenomenon of *normalization* (simplifying proofs to their canonical form) turns out to be the same as *computation* — this is the Curry-Howard correspondence in action. Third, the discipline of specifying formal proof systems is exactly what proof assistants implement: when we write a proof in Lean or Agda, we are constructing a formal derivation in an underlying proof system.

We focus on two complementary formalisms: natural deduction and sequent calculus.

---

## 1. Judgments and the Proof-Theoretic Perspective

### 1.1 What is a Judgment?

In natural language, we make *assertions*: "it is raining," "2 + 2 = 4." In formal systems, assertions are replaced by *judgments* — statements that are either derivable or not within a formal system.

The most basic judgment in propositional logic is:

$$\Gamma \vdash \varphi$$

Read: "from the hypotheses $\Gamma$, the formula $\varphi$ is derivable." Here $\Gamma$ is a (possibly empty) set of formulas (the *context* or *hypotheses*), and $\varphi$ is the *conclusion*.

This notation $\Gamma \vdash \varphi$ is called a *sequent*. When $\Gamma = \emptyset$ we write $\vdash \varphi$ (or sometimes just $\varphi$), meaning $\varphi$ is derivable from no assumptions — it is a *theorem*.

**Remark 4.1.** There is a sharp distinction between:
- $\Gamma \models \varphi$ (semantic entailment, Chapter 0): every valuation making $\Gamma$ true makes $\varphi$ true.
- $\Gamma \vdash \varphi$ (syntactic derivability): there exists a formal proof of $\varphi$ from $\Gamma$.

The *soundness* theorem says $\Gamma \vdash \varphi \Rightarrow \Gamma \models \varphi$. The *completeness* theorem says $\Gamma \models \varphi \Rightarrow \Gamma \vdash \varphi$. Together they establish that these two notions coincide — for classical propositional logic. For intuitionistic logic, they *separate* (more on this in Chapter 5).

---

## 2. Natural Deduction

Natural deduction (Gentzen, 1935) is a proof system designed to mimic the structure of actual mathematical reasoning. Each connective has *introduction rules* (how to prove it) and *elimination rules* (how to use it).

### 2.1 Rules for Conjunction ($\wedge$)

**Introduction ($\wedge$-I):** To prove $\varphi \wedge \psi$, prove $\varphi$ and prove $\psi$.
$$\frac{\Gamma \vdash \varphi \quad \Gamma \vdash \psi}{\Gamma \vdash \varphi \wedge \psi} (\wedge\text{-I})$$

**Eliminations ($\wedge$-E):** From a proof of $\varphi \wedge \psi$, you can extract either component.
$$\frac{\Gamma \vdash \varphi \wedge \psi}{\Gamma \vdash \varphi} (\wedge\text{-E}_1) \qquad \frac{\Gamma \vdash \varphi \wedge \psi}{\Gamma \vdash \psi} (\wedge\text{-E}_2)$$

### 2.2 Rules for Implication ($\to$)

**Introduction ($\to$-I):** To prove $\varphi \to \psi$, assume $\varphi$ and prove $\psi$ under that assumption. The assumption is then *discharged*.
$$\frac{\Gamma, \varphi \vdash \psi}{\Gamma \vdash \varphi \to \psi} (\to\text{-I})$$

**Elimination ($\to$-E, modus ponens):** From $\varphi \to \psi$ and $\varphi$, derive $\psi$.
$$\frac{\Gamma \vdash \varphi \to \psi \quad \Gamma \vdash \varphi}{\Gamma \vdash \psi} (\to\text{-E})$$

### 2.3 Rules for Disjunction ($\vee$)

**Introductions ($\vee$-I):** Either component proves the disjunction.
$$\frac{\Gamma \vdash \varphi}{\Gamma \vdash \varphi \vee \psi} (\vee\text{-I}_1) \qquad \frac{\Gamma \vdash \psi}{\Gamma \vdash \varphi \vee \psi} (\vee\text{-I}_2)$$

**Elimination ($\vee$-E, case analysis):** If $\varphi \vee \psi$ holds, and $\chi$ follows from both $\varphi$ and from $\psi$, then $\chi$ follows.
$$\frac{\Gamma \vdash \varphi \vee \psi \quad \Gamma, \varphi \vdash \chi \quad \Gamma, \psi \vdash \chi}{\Gamma \vdash \chi} (\vee\text{-E})$$

### 2.4 Rules for Negation and False

**$\bot$-Introduction ($\bot$-I):** If we have both $\varphi$ and $\neg\varphi$, we have a contradiction.
$$\frac{\Gamma \vdash \varphi \quad \Gamma \vdash \neg\varphi}{\Gamma \vdash \bot} (\bot\text{-I})$$

**$\bot$-Elimination (ex falso):** From a contradiction, anything follows.
$$\frac{\Gamma \vdash \bot}{\Gamma \vdash \varphi} (\bot\text{-E})$$

**Negation introduction ($\neg$-I):** To prove $\neg\varphi$, assume $\varphi$ and derive $\bot$.
$$\frac{\Gamma, \varphi \vdash \bot}{\Gamma \vdash \neg\varphi} (\neg\text{-I})$$

**Classical vs. intuitionistic:** The above rules (without any additional rule) define *intuitionistic* propositional logic. Classical logic adds one more rule:

**Double negation elimination (DNE) / Law of excluded middle:**
$$\frac{\Gamma \vdash \neg\neg\varphi}{\Gamma \vdash \varphi} (\text{DNE}) \qquad \text{or equivalently} \quad \Gamma \vdash \varphi \vee \neg\varphi \quad (\text{LEM})$$

The choice of whether to include this rule is the fundamental divide between classical and intuitionistic logic. We explore this deeply in Chapter 5.

### 2.5 Derivation Trees

A *derivation* (or *proof tree*) in natural deduction is a finite tree of sequents, where each node is the conclusion of a rule application and its children are the premises.

**Example 4.2.** Here is a derivation of $\varphi \to \psi \to \varphi$ (a tautology reading "if $\varphi$, then if $\psi$, then $\varphi$"):

```
           [φ]    (hypothesis, discharged later)
           ───────────── (→-I, discharging [ψ])
           ψ → φ
     ───────────────── (→-I, discharging [φ])
     φ → (ψ → φ)
```

More formally:
1. Assume $[\varphi]^1$ (an undischarged hypothesis labeled 1).
2. Assume $[\psi]^2$ (an undischarged hypothesis labeled 2).
3. From step 1, $\varphi$ is available. By $\to$-I discharging $[\psi]^2$: $\psi \to \varphi$ (label 2 is now discharged).
4. By $\to$-I discharging $[\varphi]^1$: $\varphi \to (\psi \to \varphi)$.

**Example 4.3.** Derivation of $(P \to Q) \to (Q \to R) \to (P \to R)$ (transitivity of implication):
1. Assume $[P \to Q]^1$, $[Q \to R]^2$, $[P]^3$.
2. By $\to$-E on step 1 and step 3: $Q$.
3. By $\to$-E on step 2 and step 4: $R$.
4. By $\to$-I discharging $[P]^3$: $P \to R$.
5. By $\to$-I discharging $[Q \to R]^2$: $(Q \to R) \to (P \to R)$.
6. By $\to$-I discharging $[P \to Q]^1$: $(P \to Q) \to (Q \to R) \to (P \to R)$.

---

## 3. Normal Forms and Normalization

The central theorem of proof theory for natural deduction is the *normalization theorem*. A proof is in *normal form* if it contains no *detours* — sequences of steps where you introduce a connective only to immediately eliminate it.

### 3.1 Detours (Redexes)

A *detour* (or *reduction redex*) has the pattern:

$$\frac{\dfrac{\Gamma, \varphi \vdash \psi}{\Gamma \vdash \varphi \to \psi} (\to\text{-I}) \quad \Gamma \vdash \varphi}{\Gamma \vdash \psi} (\to\text{-E})$$

This says: "I prove $\varphi \to \psi$ by assuming $\varphi$ and proving $\psi$; then I use $\varphi \to \psi$ with a proof of $\varphi$ to get $\psi$." But I could have just substituted the proof of $\varphi$ directly into the subproof and gotten $\psi$ without the detour!

The *reduction* for this detour: substitute the proof of $\varphi$ for the assumption $[\varphi]$ in the proof of $\psi$. This is *exactly* the $\beta$-reduction rule of lambda calculus.

**Definition 4.4.** A natural deduction proof is in *normal form* (or *normalized*) if it contains no detours.

**Theorem 4.5 (Normalization Theorem, Prawitz 1965).** Every proof in intuitionistic natural deduction can be reduced, by a finite sequence of detour eliminations, to a proof in normal form.

**Theorem 4.6 (Strong Normalization).** Every sequence of reductions terminates — there are no infinite reduction sequences.

*Proof idea.* Assign to each proof a complexity measure (e.g., the sum of "formula sizes" at all detour points). Each reduction strictly decreases this measure, so the process must terminate. A rigorous proof uses a method called *candidates of reducibility* or *logical relations*. $\square$

### 3.2 The Subformula Property

**Theorem 4.7 (Subformula Property).** Every formula in a normal proof is a subformula of either the conclusion or one of the hypotheses.

This is profound: a normal proof of $\varphi$ from $\Gamma$ is *analytic* — it never introduces "detour formulas" not present in $\varphi$ or $\Gamma$. Normal proofs are the most economical proofs.

**Example 4.8.** The proof of $P \to P$ in normal form is just: assume $[P]^1$; conclude $P \to P$ by $\to$-I. Every formula ($P$ and $P \to P$) is a subformula of the conclusion.

### 3.3 The Curry-Howard Correspondence: First Glimpse

The normalization theorem reveals a deep connection: proof reductions *are* computation steps.

| Proof Theory | Computation |
|---|---|
| Formula | Type |
| Proof | Program / Term |
| Hypothesis $[\varphi]$ | Variable $x : A$ |
| $\to$-I (discharging $[\varphi]$) | $\lambda$-abstraction $\lambda x, t$ |
| $\to$-E (modus ponens) | Function application $f(x)$ |
| Detour (introduce then eliminate) | $\beta$-redex $(\lambda x, t)(s)$ |
| Reduction of detour | $\beta$-reduction $t[s/x]$ |
| Normal form proof | Normal form (value) |

This table is the beginning of the Curry-Howard correspondence, which we develop fully in Chapter 6.

---

## 4. Sequent Calculus

Natural deduction mirrors mathematical proof. Sequent calculus (also Gentzen, 1935) is more symmetric and better suited for proof-theoretic analysis.

### 4.1 Sequents and Structural Rules

In sequent calculus, proofs manipulate *sequents* $\Gamma \vdash \Delta$ where both $\Gamma$ (the *antecedent*) and $\Delta$ (the *succedent*) are finite multisets of formulas. The intuitive reading: "from all formulas in $\Gamma$, at least one of the formulas in $\Delta$ is provable."

(In the *intuitionistic* version: $\Gamma \vdash \varphi$ with a single formula on the right. In the *classical* version: $\Gamma \vdash \Delta$ with a multiset on the right.)

**Structural rules** govern the management of formulas in $\Gamma$ and $\Delta$:

$$\frac{\Gamma \vdash \Delta}{\Gamma, \varphi \vdash \Delta}(\text{Weak}_L) \qquad \frac{\Gamma \vdash \Delta}{\Gamma \vdash \Delta, \varphi}(\text{Weak}_R)$$
(Weakening: hypotheses can be added, succedents can be expanded.)

$$\frac{\Gamma, \varphi, \varphi \vdash \Delta}{\Gamma, \varphi \vdash \Delta}(\text{Contr}_L) \qquad \frac{\Gamma \vdash \Delta, \varphi, \varphi}{\Gamma \vdash \Delta, \varphi}(\text{Contr}_R)$$
(Contraction: duplicate formulas can be merged.)

**The identity axiom:**
$$\overline{\varphi \vdash \varphi} (\text{Id})$$

### 4.2 Logical Rules for $\to$

$$\frac{\Gamma \vdash \varphi, \Delta \quad \Gamma', \psi \vdash \Delta'}{\Gamma, \Gamma', \varphi \to \psi \vdash \Delta, \Delta'} ({\to}_L) \qquad \frac{\Gamma, \varphi \vdash \psi}{\Gamma \vdash \varphi \to \psi} ({\to}_R)$$

The left rule says: to use $\varphi \to \psi$ (in the context), we must both prove $\varphi$ and use $\psi$.

### 4.3 The Cut Rule

The most important rule in sequent calculus is the *cut rule*:

$$\frac{\Gamma \vdash \varphi, \Delta \quad \Gamma', \varphi \vdash \Delta'}{\Gamma, \Gamma' \vdash \Delta, \Delta'} (\text{Cut})$$

Cut says: if we can prove $\varphi$, we can use it as a *lemma*. It is the formal incarnation of the mathematical practice of citing lemmas.

**Theorem 4.8 (Cut Elimination, Gentzen's Hauptsatz 1935).** Every sequent provable with the Cut rule is provable without it.

This is one of the most important theorems in mathematical logic. Its consequences:
- **Consistency:** A formula and its negation cannot both be provable (since there is no Cut-free proof of $\vdash \bot$).
- **Subformula property:** Every formula in a Cut-free proof is a subformula of the endsequent.
- **Decidability:** For propositional logic, the Cut-free proof search is finite and terminating.

*Proof sketch.* By induction on a complexity measure of the proof, show that any application of Cut can be eliminated — either by removing it entirely (if the cut formula is not used) or by replacing a large cut by several smaller cuts, until cuts can be eliminated completely. This is Gentzen's "ordinal analysis" for predicate logic (requires transfinite induction up to $\varepsilon_0$). $\square$

---

## 5. Proof Theory in Type Theory

In Martin-Löf type theory and HoTT, the type theory itself is a proof system. The "derivations" are *typing judgments*:

$$\Gamma \vdash t : A$$

Read: "in context $\Gamma$, the term $t$ has type $A$." Here:
- $\Gamma$ is a list of variable declarations $x_1 : A_1, \ldots, x_n : A_n$.
- $t$ is a term (program).
- $A$ is a type (proposition).

The rules of type theory correspond exactly to the rules of natural deduction, via Curry-Howard. The normalization theorem for type theory says every well-typed term reduces to a unique normal form — this is both *computational* (programs terminate) and *proof-theoretic* (proofs simplify).

Cut elimination in type theory is *substitution elimination*: any term of the form $(\lambda x : A, t)(s)$ reduces to $t[s/x]$. The cut formula $\varphi$ corresponds to the type $A$ of the substituted term.

---

## Exercises

**4.1.** Construct a natural deduction proof of each of the following:
  - $\varphi \wedge \psi \to \psi \wedge \varphi$ (commutativity of conjunction)
  - $(\varphi \to \psi) \to (\varphi \to \chi) \to \varphi \to \psi \wedge \chi$
  - $(\varphi \to \chi) \to (\psi \to \chi) \to \varphi \vee \psi \to \chi$

**4.2.** Show that $\neg\neg P \to P$ requires the classical rule DNE. Specifically, exhibit an intuitionistic model (a Kripke frame, as in Chapter 5) where $\neg\neg P$ holds but $P$ does not.

**4.3.** The following formulas are *not* intuitionistically valid. For each, explain which step in a potential proof requires excluded middle:
  - $(P \to Q) \vee (Q \to P)$
  - $\neg(P \wedge Q) \to \neg P \vee \neg Q$ (de Morgan's law for $\wedge$)

**4.4.** Carry out the detour elimination for the following proof step by step. Show that the result is a valid proof in normal form:
  - Proof of $Q$ from $P$ and $P \to Q$: `(→-I, discharging P)` applied to assumption $Q$, then `→-E` applied to the result and assumption $P$.

**4.5.** Prove cut elimination for the propositional fragment of sequent calculus. (For the full proof, consult Troelstra-Schwichtenberg Chapter 3; write the key inductive argument.)

**4.6.** Verify the subformula property for the normal proofs you constructed in Exercise 4.1.

**4.7.** The *de Morgan dualities* for classical logic: show that in classical natural deduction, $\neg(\varphi \vee \psi) \leftrightarrow \neg\varphi \wedge \neg\psi$ and $\neg(\varphi \wedge \psi) \leftrightarrow \neg\varphi \vee \neg\psi$. Show that the second implication ($\neg(\varphi \wedge \psi) \to \neg\varphi \vee \neg\psi$) is not intuitionistically provable.

**4.8 (Challenge).** The *consistency* of propositional natural deduction: prove that there is no derivation of $\vdash \bot$ (false from no hypotheses) in intuitionistic propositional natural deduction. (*Hint:* Use the subformula property — $\bot$ has no subformulas, so a normal proof of $\bot$ from nothing would have to be empty, but there are no 0-premise rules with conclusion $\bot$.)
