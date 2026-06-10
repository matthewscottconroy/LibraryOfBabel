# Natural Deduction

## How Mathematicians Actually Reason

Gentzen's great insight was that mathematical reasoning has a natural shape — a shape that formal systems had, up to 1935, largely ignored. The older formalisms, like Hilbert-style axiom systems, reduced all reasoning to a fixed set of axioms and a single rule (modus ponens). This produces valid proofs, but proofs that bear no resemblance to how a mathematician actually thinks.

Natural deduction is different. It aims to capture the actual structure of mathematical argument: you assume things, you derive consequences, you discharge assumptions when they've served their purpose. The system organizes itself not around axioms but around *connectives*, and for each connective, it provides exactly two kinds of rules:

**Introduction rules**: how to *prove* a proposition whose main connective is the given one. How do you prove $A \wedge B$? By proving $A$ and proving $B$. How do you prove $A \to B$? By assuming $A$ and deriving $B$.

**Elimination rules**: how to *use* a proved proposition with the given connective as its main connective. If you have $A \wedge B$, you can extract $A$ or $B$. If you have $A \to B$ and you have $A$, you can derive $B$.

This introduction-elimination structure is the organizing principle of proof theory. It is also the organizing principle of type theory, where introduction rules become constructors and elimination rules become case-analysis or recursion principles. Understanding natural deduction is understanding the logic underlying all of MLTT and HoTT.

## Conjunction: $A \wedge B$

**Introduction** ($\wedge$I): to prove $A \wedge B$, prove $A$ and prove $B$.

$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \quad (\wedge\text{I})$$

**Elimination** ($\wedge$E$_1$, $\wedge$E$_2$): from a proof of $A \wedge B$, extract $A$ or extract $B$.

$$\frac{\Gamma \vdash A \wedge B}{\Gamma \vdash A} \quad (\wedge\text{E}_1) \qquad \frac{\Gamma \vdash A \wedge B}{\Gamma \vdash B} \quad (\wedge\text{E}_2)$$

Under Curry-Howard, $A \wedge B$ is the *product type* $A \times B$. Introduction is forming a pair $(a, b)$; elimination is the projections $\pi_1$ and $\pi_2$.

## Implication: $A \to B$

**Introduction** ($\to$I): to prove $A \to B$, *assume* $A$ and derive $B$ under that assumption, then discharge the assumption.

$$\frac{\Gamma, A \vdash B}{\Gamma \vdash A \to B} \quad (\to\text{I})$$

This is the rule of hypothetical reasoning. Notice what it does: if we can derive $B$ with $A$ as an extra hypothesis, we can conclude $A \to B$ without that hypothesis. The assumption $A$ gets "used up" — discharged — in the process of forming the implication.

In derivation trees using discharge notation, this is sometimes written with the discharged assumption labeled:

$$\frac{\overset{[A]^u}{\vdots} \\ B}{A \to B} \quad \to\text{I}^u$$

The superscript $u$ marks the assumption that is discharged. A single derivation may discharge an assumption multiple times, or not at all — an assumption may remain undischarged in the conclusion.

**Elimination** ($\to$E, *modus ponens*): from a proof of $A \to B$ and a proof of $A$, derive $B$.

$$\frac{\Gamma \vdash A \to B \quad \Gamma \vdash A}{\Gamma \vdash B} \quad (\to\text{E})$$

Under Curry-Howard, $A \to B$ is the *function type*. Introduction is lambda abstraction $\lambda x. t$; elimination is function application $f\, a$.

## Disjunction: $A \vee B$

**Introduction** ($\vee$I$_1$, $\vee$I$_2$): to prove $A \vee B$, either prove $A$ or prove $B$.

$$\frac{\Gamma \vdash A}{\Gamma \vdash A \vee B} \quad (\vee\text{I}_1) \qquad \frac{\Gamma \vdash B}{\Gamma \vdash A \vee B} \quad (\vee\text{I}_2)$$

This is the constructive content of disjunction: a proof of $A \vee B$ says *which* disjunct holds and provides evidence for it. Classically, you can prove $A \vee B$ without knowing which. Constructively, you cannot — a proof must commit.

**Elimination** ($\vee$E, *proof by cases*): from a proof of $A \vee B$, and proofs of $C$ under each disjunct separately, derive $C$.

$$\frac{\Gamma \vdash A \vee B \quad \Gamma, A \vdash C \quad \Gamma, B \vdash C}{\Gamma \vdash C} \quad (\vee\text{E})$$

To use $A \vee B$: handle both cases. In case $A$ holds, derive $C$ from $A$. In case $B$ holds, derive $C$ from $B$. In either case, $C$ follows.

Under Curry-Howard, $A \vee B$ is the *coproduct type* $A + B$ (tagged union). Introduction is $\mathsf{inl}(a)$ or $\mathsf{inr}(b)$; elimination is pattern matching.

## Falsehood: $\bot$

$\bot$ ("bottom") is the proposition that is always false. There is no proof of $\bot$ — that is its defining property. Accordingly, there is *no introduction rule* for $\bot$. The only rule is elimination:

**Elimination** ($\bot$E, *ex falso quodlibet*): from a proof of $\bot$, derive anything.

$$\frac{\Gamma \vdash \bot}{\Gamma \vdash A} \quad (\bot\text{E})$$

This is "from false, anything follows." If you ever derive $\bot$ — a contradiction — the system allows you to conclude any proposition whatsoever. This is not a defect; it is the logical content of "if the system is inconsistent, all propositions are provable." (A consistent system, of course, never actually produces a proof of $\bot$.)

Under Curry-Howard, $\bot$ is the *empty type* $\mathbf{0}$, which has no elements. The elimination rule is pattern matching on all zero cases — vacuously, any output type is acceptable.

**Negation** is defined: $\neg A \;\equiv\; A \to \bot$. To prove $\neg A$, assume $A$ and derive a contradiction. To use $\neg A$ together with a proof of $A$, apply modus ponens to get $\bot$, then use ex falso.

## Truth: $\top$

$\top$ ("top") is the proposition that is always true. It has exactly one proof — the trivial proof — and that proof carries no information. The introduction rule:

$$\frac{}{\Gamma \vdash \top} \quad (\top\text{I})$$

There is no elimination rule, because nothing can be extracted from the trivial proof. Under Curry-Howard, $\top$ is the *unit type* $\mathbf{1}$, which has exactly one element $\star$.

## Universal Quantification: $\forall x : A, B(x)$

**Introduction** ($\forall$I): to prove $\forall x : A, B(x)$, prove $B(a)$ for a *fresh* variable $a$ — one not mentioned in the context or conclusion.

$$\frac{\Gamma \vdash B(a)}{\Gamma \vdash \forall x : A, B(x)} \quad (\forall\text{I}) \quad [a \text{ fresh}]$$

The freshness condition is crucial. If $a$ appeared in $\Gamma$, the proof would be using specific information about $a$, not proving the result for all values. Freshness ensures we have made no assumptions about $a$ — it is truly arbitrary.

**Elimination** ($\forall$E, *universal instantiation*): from a proof of $\forall x : A, B(x)$ and a specific term $t$ of type $A$, derive $B(t)$.

$$\frac{\Gamma \vdash \forall x : A, B(x)}{\Gamma \vdash B(t)} \quad (\forall\text{E})$$

Under Curry-Howard, $\forall x : A, B(x)$ is the *dependent product type* $\Pi_{x:A} B(x)$: the type of functions that take any $a : A$ and return an element of $B(a)$.

## Existential Quantification: $\exists x : A, B(x)$

**Introduction** ($\exists$I): to prove $\exists x : A, B(x)$, exhibit a specific term $t : A$ and prove $B(t)$.

$$\frac{\Gamma \vdash t : A \quad \Gamma \vdash B(t)}{\Gamma \vdash \exists x : A, B(x)} \quad (\exists\text{I})$$

This is the constructive heart of existential quantification. The witness $t$ is part of the proof. You cannot prove "something exists" without saying what that thing is.

**Elimination** ($\exists$E): from a proof of $\exists x : A, B(x)$ and a proof of $C$ using an arbitrary element with property $B$, derive $C$.

$$\frac{\Gamma \vdash \exists x : A, B(x) \quad \Gamma, a : A, B(a) \vdash C}{\Gamma \vdash C} \quad (\exists\text{E}) \quad [a \text{ fresh}]$$

The freshness condition prevents the conclusion $C$ from depending on the specific witness — we are working with "whatever the witness is," not a specific named element.

Under Curry-Howard, $\exists x : A, B(x)$ is the *dependent sum type* $\Sigma_{x:A} B(x)$: the type of pairs $(t, p)$ where $t : A$ and $p : B(t)$.

## Classical vs. Intuitionistic Logic

The rules above are *intuitionistic*. Neither the Law of Excluded Middle (LEM) nor Double Negation Elimination (DNE) is derivable from them.

**LEM:** $\vdash A \vee \neg A$ — for every $A$, either $A$ or its negation.

**DNE:** $\neg\neg A \vdash A$ — from a double negation, extract the original proposition.

**Peirce's Law:** $((A \to B) \to A) \to A$ — a classical tautology with no obvious intuitionistic content.

These three are equivalent over intuitionistic logic: adding any one of them to the intuitionistic rules gives classical logic. To obtain classical logic, add any of the following to the intuitionistic rules:

$$\frac{\Gamma, \neg A \vdash \bot}{\Gamma \vdash A} \quad (\text{RAA — reductio ad absurdum})$$

This is "strong" proof by contradiction: to prove $A$, it suffices to derive a contradiction from $\neg A$. In intuitionistic logic, we have only the "weak" version: from $A$ we can derive $\neg\neg A$, but not conversely.

The Curry-Howard correspondence illuminates why classical logic lacks clean computational content. LEM would require a term of type $A + (A \to \mathbf{0})$ — but for an arbitrary type $A$, no such term exists in a normalizing type theory. Classical logic corresponds to programs with *call/cc* (call-with-current-continuation), a non-standard control operator. This is valid computation, but it doesn't "return" in the ordinary sense — it jumps to a saved context. The classical theorem "every program is either terminating or non-terminating" would, if internalized as a type, require a runtime oracle.

## A Full Worked Example

Let us derive the theorem: $\vdash (A \to B) \to (B \to C) \to A \to C$ — function composition.

We build the derivation step by step:

1. Hypothesis: $A \to B, B \to C, A \vdash A$ (by Hyp)
2. Hypothesis: $A \to B, B \to C, A \vdash A \to B$ (by Hyp)
3. From (2) and (1) by $\to$E: $A \to B, B \to C, A \vdash B$
4. Hypothesis: $A \to B, B \to C, A \vdash B \to C$ (by Hyp)
5. From (4) and (3) by $\to$E: $A \to B, B \to C, A \vdash C$
6. By $\to$I (discharging $A$): $A \to B, B \to C \vdash A \to C$
7. By $\to$I (discharging $B \to C$): $A \to B \vdash (B \to C) \to A \to C$
8. By $\to$I (discharging $A \to B$): $\vdash (A \to B) \to (B \to C) \to A \to C$

Under Curry-Howard, this derivation *is* the program $\lambda f.\, \lambda g.\, \lambda x.\, g\,(f\, x)$ — function composition. The proof tree and the program are the same object.

## The Subformula Property (Preview)

A striking fact about normal form derivations — proofs with no detours — is that every formula appearing in the derivation is a *subformula* of the conclusion or one of the undischarged hypotheses. No "external" formulas enter.

This is the *subformula property*, and it follows from normalization. We state it here and prove it in Section 3. Its significance: a proof of $A$ that makes no assumptions can only use formulas that appear in $A$ itself. To prove a purely propositional statement, you only need propositional reasoning about the propositions that appear in it.

This has immediate consequences for consistency (if there were a proof of $\bot$, its normal form would use only subformulas of $\bot$, but $\bot$ has no introduction rule, contradiction) and decidability (normal proofs are bounded in size by the formula, making proof search finite).

The subformula property is one of the deepest structural facts about proofs. Its proof — the normalization theorem — is the subject of the next section.
