# 1.2 Semantics: Truth Tables and Logical Validity

## Giving Formulas Meaning

Syntax tells us what formulas exist. Semantics tells us what they *mean* — specifically, when they are true and when they are false. In propositional logic, meaning is given entirely by *truth values*.

The key idea: the truth value of a complex formula is determined entirely by the truth values of its atomic components. This is the *truth-functional* nature of propositional logic. The connectives are functions on truth values.

## Valuations

**Definition (Valuation).** A *valuation* (or *truth assignment* or *interpretation*) is a function:
$$v : \{\text{atomic propositions}\} \to \{\mathbf{T}, \mathbf{F}\}$$

Given a valuation, we extend it to all formulas recursively — this is the *semantic evaluation function*, often written $[\![\varphi]\!]_v$ or just $v(\varphi)$:

- $v(P) = $ whatever $v$ assigns to the atom $P$
- $v(\neg\varphi) = \mathbf{T}$ iff $v(\varphi) = \mathbf{F}$
- $v(\varphi \wedge \psi) = \mathbf{T}$ iff $v(\varphi) = \mathbf{T}$ and $v(\psi) = \mathbf{T}$
- $v(\varphi \vee \psi) = \mathbf{T}$ iff $v(\varphi) = \mathbf{T}$ or $v(\psi) = \mathbf{T}$ (or both)
- $v(\varphi \to \psi) = \mathbf{T}$ iff $v(\varphi) = \mathbf{F}$ or $v(\psi) = \mathbf{T}$
- $v(\varphi \leftrightarrow \psi) = \mathbf{T}$ iff $v(\varphi) = v(\psi)$

The extension is unique: given the values on atoms, there is exactly one way to evaluate every formula. This is proved by structural induction on formulas.

## Truth Tables

A *truth table* displays the evaluation of a formula for every possible valuation of its atoms. For a formula with $n$ distinct atoms, there are $2^n$ valuations, giving $2^n$ rows.

**The Five Connectives:**

| $\varphi$ | $\psi$ | $\neg\varphi$ | $\varphi \wedge \psi$ | $\varphi \vee \psi$ | $\varphi \to \psi$ | $\varphi \leftrightarrow \psi$ |
|:---------:|:-------:|:-------------:|:---------------------:|:-------------------:|:------------------:|:------------------------------:|
| T | T | F | T | T | T | T |
| T | F | F | F | T | F | F |
| F | T | T | F | T | T | F |
| F | F | T | F | F | T | T |

## Understanding Implication

The truth table for $\varphi \to \psi$ surprises many students at first. The row $v(\varphi) = \mathbf{F}$ gives $v(\varphi \to \psi) = \mathbf{T}$ regardless of $\psi$. This is called *vacuous truth*, and it has a logical rationale:

Think of $P \to Q$ as a promise: "if $P$ is true, then $Q$ is true." When is this promise *broken*? Exactly when $P$ is true but $Q$ is false — you made a claim about the case when $P$ holds, and $Q$ didn't hold. When $P$ is false, the promise is never activated. You didn't promise anything about what happens when $P$ is false, so the promise can't be violated.

Examples of vacuously true statements:
- "If 2 is odd, then the moon is made of cheese." (True, because 2 is not odd.)
- "For all $n$, if $n < 0$ and $n \in \mathbb{N}$, then $n^2 < 0$." (Vacuously true because no natural number is negative.)

Vacuous truth is not a bug in logic — it's a feature that makes implication well-behaved for mathematical reasoning. The formula $P \to Q$ is useful precisely because it makes a non-trivial claim only when $P$ holds.

## Tautologies, Contradictions, and Satisfiability

**Definition.** A formula $\varphi$ is:
- A *tautology* (or *logically valid*) if $v(\varphi) = \mathbf{T}$ for every valuation $v$.
- A *contradiction* (or *unsatisfiable*) if $v(\varphi) = \mathbf{F}$ for every valuation $v$.
- *Satisfiable* if there exists some valuation $v$ with $v(\varphi) = \mathbf{T}$.

Tautologies are true "by logic alone," regardless of what the atoms actually mean. Contradictions are false "by logic alone." Satisfiable formulas might or might not be true depending on the situation.

**Key tautologies worth knowing:**

| Formula | Name |
|---------|------|
| $P \vee \neg P$ | Law of excluded middle (LEM) |
| $\neg(P \wedge \neg P)$ | Law of non-contradiction |
| $P \to P$ | Identity |
| $P \to (Q \to P)$ | Weakening |
| $(P \to (Q \to R)) \to ((P \to Q) \to (P \to R))$ | Frege / S combinator |
| $(P \to Q) \wedge (Q \to R) \to (P \to R)$ | Hypothetical syllogism |
| $(\neg Q \to \neg P) \to (P \to Q)$ | Contrapositive |
| $\neg\neg P \to P$ | Double negation elimination (classical only) |
| $(P \to Q) \to (\neg Q \to \neg P)$ | Modus tollens |
| $P \wedge Q \to P$ | And-elimination (left) |
| $P \to P \vee Q$ | Or-introduction (left) |

**Important:** The tautologies listed above are all *classical* tautologies — they hold in standard propositional logic. Some, like $P \vee \neg P$ and $\neg\neg P \to P$, are *not* valid in *intuitionistic* logic. We will return to this distinction in Chapter 5. For now, be aware that not all mathematicians accept all these principles.

## Semantic Entailment

**Definition (Semantic Entailment).** A set of formulas $\Gamma$ *semantically entails* $\varphi$, written $\Gamma \models \varphi$, if:

For every valuation $v$ that makes every formula in $\Gamma$ true, $v(\varphi) = \mathbf{T}$.

We write $\models \varphi$ (with empty $\Gamma$) to mean $\varphi$ is a tautology.

**Examples:**

- $\{P, P \to Q\} \models Q$ — modus ponens (if you know $P$ and $P \to Q$, you know $Q$)
- $\{P \to Q, \neg Q\} \models \neg P$ — modus tollens
- $\{P \vee Q, \neg P\} \models Q$ — disjunctive syllogism
- $\{P \wedge Q\} \models P$ — conjunction elimination

To *verify* $\Gamma \models \varphi$, you check every row of the truth table where all formulas in $\Gamma$ are true, and confirm $\varphi$ is also true in those rows.

## Logical Equivalence

**Definition.** Two formulas $\varphi$ and $\psi$ are *logically equivalent*, written $\varphi \equiv \psi$, if $\models (\varphi \leftrightarrow \psi)$ — i.e., they have the same truth value under every valuation.

Logical equivalence is a congruence: replacing any subformula by an equivalent one yields an equivalent formula.

**Useful equivalences:**

| Equivalence | Name |
|-------------|------|
| $\neg\neg P \equiv P$ | Double negation (classical) |
| $P \to Q \equiv \neg P \vee Q$ | Material implication |
| $P \to Q \equiv \neg Q \to \neg P$ | Contrapositive |
| $\neg(P \wedge Q) \equiv \neg P \vee \neg Q$ | De Morgan (1) |
| $\neg(P \vee Q) \equiv \neg P \wedge \neg Q$ | De Morgan (2) |
| $P \wedge (Q \vee R) \equiv (P \wedge Q) \vee (P \wedge R)$ | Distributivity |
| $P \vee (Q \wedge R) \equiv (P \vee Q) \wedge (P \vee R)$ | Distributivity |
| $P \wedge Q \equiv Q \wedge P$ | Commutativity of $\wedge$ |
| $P \vee Q \equiv Q \vee P$ | Commutativity of $\vee$ |

## Normal Forms

Every propositional formula can be converted to a *normal form* — a standardized syntactic shape. Two useful normal forms:

**Conjunctive Normal Form (CNF):** A conjunction ($\wedge$) of *clauses*, where each clause is a disjunction ($\vee$) of *literals* (atoms or negated atoms). Example: $(P \vee \neg Q) \wedge (Q \vee R) \wedge \neg P$.

**Disjunctive Normal Form (DNF):** A disjunction ($\vee$) of *terms*, where each term is a conjunction ($\wedge$) of literals. Example: $(P \wedge Q) \vee (\neg P \wedge R)$.

Every formula is equivalent to a formula in CNF and to one in DNF. These forms are useful for algorithmic reasoning (SAT solvers work with CNF).

## The Satisfiability Problem (SAT)

Given a propositional formula, determining whether it is satisfiable is the *Boolean Satisfiability Problem* (SAT). For $n$ variables, the naive approach checks all $2^n$ valuations.

SAT was the first problem proven to be NP-complete (Cook, 1971; Levin, 1973). This means:
- No known polynomial-time algorithm solves all instances.
- If you could solve SAT in polynomial time, you could solve every NP problem in polynomial time (P = NP).

Despite this worst-case hardness, modern SAT solvers (using DPLL, CDCL algorithms) can handle formulas with millions of variables in practice. They are widely used in hardware verification, automated reasoning, and proof assistants.

## The Connection to Proof: Soundness and Completeness

A key theorem of classical propositional logic:

**Theorem (Soundness and Completeness of Propositional Logic).** There exists a proof system for propositional logic such that $\Gamma \vdash \varphi$ (provability) if and only if $\Gamma \models \varphi$ (semantic entailment).

- *Soundness* ($\Gamma \vdash \varphi \Rightarrow \Gamma \models \varphi$): everything provable is true.
- *Completeness* ($\Gamma \models \varphi \Rightarrow \Gamma \vdash \varphi$): everything true is provable.

This is a remarkable alignment between the syntactic world (formal proofs) and the semantic world (truth tables). We'll encounter the proof system side in Section 2 (natural deduction), and we'll see in Chapter 5 that *intuitionistic* logic deliberately breaks completeness with respect to the classical truth-table semantics — but is complete with respect to a different, constructive semantics.
