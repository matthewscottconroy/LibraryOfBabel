# Sequent Calculus

## A More Symmetric Perspective

Natural deduction has an asymmetry. It treats the conclusion specially: there is one formula on the right side of $\Gamma \vdash A$, while the left side may have many. Introduction rules build up the conclusion; elimination rules break down hypotheses. The system is designed around the *conclusion*.

Sequent calculus, Gentzen's second invention in the same 1935 paper, takes a different approach. A *sequent* has the form:

$$\Gamma \Rightarrow \Delta$$

where $\Gamma$ and $\Delta$ are both lists of formulas. Read: "if all formulas in $\Gamma$ hold, then at least one formula in $\Delta$ holds." The left side $\Gamma$ is the *antecedent* (hypotheses); the right side $\Delta$ is the *succedent* (possible conclusions).

Allowing multiple conclusions on the right gives the calculus a left-right symmetry that natural deduction lacks. For classical logic, this symmetry is perfect: the rules for conjunction on the left mirror the rules for disjunction on the right, the rules for implication on the left mirror the rules for implication on the right, and so forth. This symmetry makes certain theoretical arguments much cleaner.

More importantly: sequent calculus is the natural home of Gentzen's *Hauptsatz* — the cut elimination theorem. This theorem says that every proof using the cut rule can be transformed into a proof without cut, at no loss of logical power. It is one of the great theorems of 20th-century logic, and its proof is most naturally carried out in the sequent setting.

## The Sequent and Its Meaning

**Definition.** A *sequent* $\Gamma \Rightarrow \Delta$ consists of a multiset $\Gamma$ of formulas (the antecedent) and a multiset $\Delta$ of formulas (the succedent). Its intended meaning is:

$$\bigwedge_{\varphi \in \Gamma} \varphi \;\implies\; \bigvee_{\psi \in \Delta} \psi$$

Special cases:
- $\Gamma \Rightarrow A$ (single conclusion): "from all of $\Gamma$, conclude $A$." This is the natural deduction judgment.
- $\Rightarrow A$ (empty antecedent): "$A$ is a theorem."
- $\Gamma \Rightarrow$ (empty succedent): "from $\Gamma$, derive a contradiction" — $\Gamma$ is inconsistent.

For intuitionistic logic, we restrict to *single-conclusion sequents* $\Gamma \Rightarrow A$ — the multi-conclusion setting is for classical logic (**LK**), while intuitionistic logic uses **LJ** with at most one formula on the right.

## The Identity Axiom and Structural Rules

**Identity (Axiom):**
$$\overline{A \Rightarrow A}$$

The most primitive sequent: $A$ proves $A$.

**Structural rules** for sequent calculus correspond to the structural rules of natural deduction, but now appear symmetrically on both sides.

**Weakening**:
$$\frac{\Gamma \Rightarrow \Delta}{\Gamma, A \Rightarrow \Delta} \quad (W_L) \qquad \frac{\Gamma \Rightarrow \Delta}{\Gamma \Rightarrow A, \Delta} \quad (W_R)$$

**Contraction**:
$$\frac{A, A, \Gamma \Rightarrow \Delta}{A, \Gamma \Rightarrow \Delta} \quad (C_L) \qquad \frac{\Gamma \Rightarrow \Delta, A, A}{\Gamma \Rightarrow \Delta, A} \quad (C_R)$$

**Exchange** is usually absorbed by treating $\Gamma$ and $\Delta$ as multisets (unordered collections) rather than lists.

## Logical Rules: Left and Right

For each connective, there are *left rules* (rules that put the connective in the antecedent) and *right rules* (rules that put the connective in the succedent). The right rules correspond to introduction rules; the left rules correspond to elimination rules.

**Conjunction:**

$$\frac{A, \Gamma \Rightarrow \Delta}{A \wedge B, \Gamma \Rightarrow \Delta} \quad (\wedge L_1) \qquad \frac{B, \Gamma \Rightarrow \Delta}{A \wedge B, \Gamma \Rightarrow \Delta} \quad (\wedge L_2)$$

$$\frac{\Gamma \Rightarrow \Delta, A \quad \Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \wedge B} \quad (\wedge R)$$

Left: to use $A \wedge B$, use either $A$ or $B$ (you have both). Right: to prove $A \wedge B$, prove both $A$ and $B$.

**Disjunction:**

$$\frac{A, \Gamma \Rightarrow \Delta \quad B, \Gamma \Rightarrow \Delta}{A \vee B, \Gamma \Rightarrow \Delta} \quad (\vee L)$$

$$\frac{\Gamma \Rightarrow \Delta, A}{\Gamma \Rightarrow \Delta, A \vee B} \quad (\vee R_1) \qquad \frac{\Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \vee B} \quad (\vee R_2)$$

Left: to use $A \vee B$, handle both cases. Right: to prove $A \vee B$, prove one disjunct.

**Implication:**

$$\frac{\Gamma \Rightarrow \Delta, A \quad B, \Pi \Rightarrow \Lambda}{A \to B, \Gamma, \Pi \Rightarrow \Delta, \Lambda} \quad (\to L)$$

$$\frac{A, \Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \to B} \quad (\to R)$$

The left rule for implication is the most complex: to use $A \to B$ in proving something from $\Gamma \cup \Pi$, you must first prove $A$ (from $\Gamma$, contributing to $\Delta$) and then you can use $B$ (from $\Pi$ with $B$, contributing to $\Lambda$).

The right rule is simpler: to prove $A \to B$, add $A$ to the antecedent and prove $B$.

**Negation** (classical LK):

$$\frac{\Gamma \Rightarrow \Delta, A}{\neg A, \Gamma \Rightarrow \Delta} \quad (\neg L) \qquad \frac{A, \Gamma \Rightarrow \Delta}{\Gamma \Rightarrow \Delta, \neg A} \quad (\neg R)$$

The symmetry here is striking: $\neg L$ moves a formula from right to left; $\neg R$ moves it from left to right with a negation attached. In classical logic, the antecedent and succedent are essentially negations of each other.

## The Cut Rule

$$\frac{\Gamma \Rightarrow \Delta, A \quad A, \Pi \Rightarrow \Lambda}{\Gamma, \Pi \Rightarrow \Delta, \Lambda} \quad (\text{Cut})$$

Cut formalizes the use of a *lemma*: prove $A$ in one branch (from $\Gamma$, adding it to $\Delta$), use $A$ in another branch (from $\Pi$ and $A$, deriving $\Lambda$), and the combined proof goes from $\Gamma \cup \Pi$ to $\Delta \cup \Lambda$.

The formula $A$ — the *cut formula* — does not appear in the conclusion. It is an intermediate result: stated, proved, used, and then discarded.

Cut is *analytic-free*: the cut formula $A$ can be arbitrarily complex — much more complex than anything appearing in the conclusion. A proof with cut can use auxiliary formulas from outside the vocabulary of the theorem being proved. This is what makes mathematics efficient: we prove lemmas about one domain to derive results about another.

## The Hauptsatz: Cut Elimination

**Theorem (Gentzen's Hauptsatz, 1935).** Every sequent provable in **LK** (classical sequent calculus) or **LJ** (intuitionistic sequent calculus) has a cut-free proof.

This is Gentzen's central theorem, and its proof occupies a substantial portion of his 1935 paper. The proof is a double induction: on the *complexity* of the cut formula (measured by the number of logical connectives) and the *height* of the proof above the cut.

**Proof idea.** We show that every application of the cut rule can be "pushed up" toward the leaves of the derivation tree, reducing the complexity of the cut formula at each step until it disappears.

The key case is when the cut formula is *principal* in both premises — introduced by a right rule on the left and a left rule on the right. For example, a cut on $A \wedge B$:

$$\frac{
  \dfrac{\Gamma \Rightarrow \Delta, A \quad \Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \wedge B}
  \quad
  \dfrac{A, \Pi \Rightarrow \Lambda}{A \wedge B, \Pi \Rightarrow \Lambda}
}{\Gamma, \Pi \Rightarrow \Delta, \Lambda}$$

This can be replaced by a cut on $A$ alone:

$$\frac{\Gamma \Rightarrow \Delta, A \quad A, \Pi \Rightarrow \Lambda}{\Gamma, \Pi \Rightarrow \Delta, \Lambda}$$

The cut formula has shrunk: $A \wedge B$ replaced by $A$. By induction on formula complexity, eventually the cut formula is atomic. An atomic cut can be eliminated by a different argument (essentially, substituting one derivation into another at the leaf level).

The proof is long but mechanical: each connective case is handled separately, and the induction is carefully maintained. Gentzen's original proof is fully rigorous.

## Why Cut Elimination Is Profound

Cut elimination is not just a technical result. Its consequences reveal the deep structure of logical provability.

**1. Consistency.** If $\vdash \bot$ were provable, its cut-free proof would contain only subformulas of $\bot$. But $\bot$ has no subformulas and no right rules. The only leaf in a cut-free derivation is the axiom $A \Rightarrow A$, but no such axiom produces $\Rightarrow \bot$ (we cannot have $\bot \Rightarrow \bot$ if $\bot$ is meant to be unprovable). Contradiction: no such proof exists.

**2. The subformula property.** Every formula in a cut-free proof is a subformula of the conclusion or hypotheses. Proofs are *analytic*: they work only with the vocabulary of the statement being proved.

**3. Separation of classical and intuitionistic logic.** In **LJ** (intuitionistic sequent calculus with single-conclusion sequents), cut-free proofs respect the single-conclusion restriction. In **LK** (classical with multiple conclusions), cut-free proofs use the double-conclusion rules. Cut elimination shows that these are genuinely different systems — their provable sequents differ, and the difference shows up at the level of cut-free proofs.

**4. Herbrand's Theorem.** For first-order classical logic: if $\vdash \exists x, A(x)$, then there exist finitely many terms $t_1, \ldots, t_n$ such that $\vdash A(t_1) \vee \cdots \vee A(t_n)$. This follows from cut elimination plus the structure of quantifier rules. It gives a bridge between provability and explicit witnesses.

**5. Craig Interpolation.** If $A \vdash B$, there exists an *interpolant* $I$ — a formula using only vocabulary common to $A$ and $B$ — such that $A \vdash I$ and $I \vdash B$. The proof uses cut elimination: a cut on $I$ connects the two halves, and the subformula property ensures $I$ uses only common vocabulary.

## The Cost of Cut Elimination: Complexity

Cut elimination can cause an *exponential blowup* in proof size.

A proof with cut can be polynomial in the size of the conclusion. The corresponding cut-free proof can be exponential. This is related to fundamental questions in computational complexity.

Specifically: the number of uses of the cut rule needed to prove tautologies is connected to the strength of proof systems. Systems requiring many cuts for short proofs of simple statements are "proof-complexity hard." This is an active research area connecting logic and complexity theory.

In practice, mathematics works with cuts constantly — every use of a lemma is a cut. Cut-free proofs exist but are often impractical to write. Sequent calculus with cut is a model of mathematical practice; sequent calculus without cut is a model of the underlying logical structure.

## Proof Search and Backwards Chaining

Sequent calculus is particularly natural for *proof search*: finding a proof of a given sequent by working backwards from the goal.

In a cut-free proof, every rule is *invertible* in a weak sense: given a goal sequent, each rule specifies what sub-goals must be established. Working backwards from $\Gamma \Rightarrow A \wedge B$, the only applicable right rule is $\wedge R$, giving sub-goals $\Gamma \Rightarrow A$ and $\Gamma \Rightarrow B$. This is deterministic.

For propositional logic, the subformula property bounds the search space: all formulas in a cut-free proof are bounded in complexity by the goal. Search terminates. This gives a decision procedure for intuitionistic propositional logic.

For first-order logic, the quantifier rules complicate this: the $\forall R$ rule must choose a fresh variable, and $\exists R$ must choose an appropriate instantiation term. But even here, cut elimination guides efficient proof search strategies — this is the basis of Prolog's resolution proof search and the connection to logic programming.

## Looking Forward: From Sequents to Types

Sequent calculus has a Curry-Howard correspondence of its own, though more complex than for natural deduction.

The sequent $\Gamma \Rightarrow A$ corresponds to a typing judgment $\Gamma \vdash t : A$. The cut rule corresponds to *let-binding* or *substitution*: `let x = t in s`. Cut elimination corresponds to substitution of closed terms — beta reduction in a language with explicit let.

The resulting calculus — the *sequent calculus term assignment* — makes proof composition into first-class computation. This perspective is developed in work on *polarized type theory* and *focused proof search*, where the antecedent-succedent distinction corresponds to the negative-positive type distinction in modern dependent type theories.

In HoTT, the sequent calculus perspective resurfaces in the study of *judgmental equality*: the definitional equality of the type theory is determined by the reduction rules, which are exactly the cut-elimination steps. Understanding cut elimination is understanding computation in MLTT.
