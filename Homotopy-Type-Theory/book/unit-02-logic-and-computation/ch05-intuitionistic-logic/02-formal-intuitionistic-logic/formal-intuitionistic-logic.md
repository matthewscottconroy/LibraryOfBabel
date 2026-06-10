# Formal Intuitionistic Logic

## From Informal to Formal

The BHK interpretation tells us what constructive proofs *mean* but does not give us a formal proof system. We need to know precisely which propositions are provable constructively, and we need an explicit calculus — a set of rules — that captures exactly the valid constructive arguments.

The formal system is the *Intuitionistic Propositional Calculus* (IPC). It is the natural deduction system from Chapter 4 — with all the introduction and elimination rules for $\wedge$, $\vee$, $\to$, $\neg$, $\bot$, $\top$ — but *without* any of the classical additions (LEM, DNE, Peirce's law, RAA). The system is exactly what results from taking the BHK clauses seriously as constraints on proof.

The key facts about IPC: it proves every classical tautology whose proof does not require LEM or its equivalents; it has the Disjunction Property and Existence Property; and it is decidable (for propositional logic) — there is an algorithm that, given a formula, determines in finite time whether it is an IPC theorem.

## The IPC Proof System

IPC uses the natural deduction rules from Chapter 4 exactly, with $\neg A$ defined as $A \to \bot$. We list the rules for propositional logic; quantifier rules are analogous.

**Conjunction**: $\wedge$I, $\wedge$E$_1$, $\wedge$E$_2$ (as before).

**Implication**: $\to$I (assume $A$, derive $B$, discharge to get $A \to B$); $\to$E (modus ponens).

**Disjunction**: $\vee$I$_1$, $\vee$I$_2$; $\vee$E (case analysis requiring both branches).

**Bottom**: $\bot$E (ex falso quodlibet: from $\bot$, anything).

**Top**: $\top$I (trivial introduction).

**Negation**: Since $\neg A := A \to \bot$, the rules for negation are the rules for implication instantiated with $\bot$.

No other rules. In particular, there is no rule of the form "from $\neg A \vdash \bot$, derive $A$" (that would be RAA) and no axiom $A \vee \neg A$ (that would be LEM).

## Classical Tautologies That Fail in IPC

Which classical theorems are not IPC theorems? The clearest test cases:

**Law of Excluded Middle (LEM):** $A \vee \neg A$.
Not an IPC theorem. A proof would require knowing whether $A$ is provable or refutable for an arbitrary $A$. The Disjunction Property shows this cannot be established in IPC without actually proving one disjunct.

**Double Negation Elimination (DNE):** $\neg\neg A \to A$.
Not an IPC theorem. A proof would require converting a "refutation-of-the-refutation" of $A$ into an actual proof of $A$. No such conversion procedure exists in general.

**Peirce's Law:** $((A \to B) \to A) \to A$.
Not an IPC theorem. This is the type of the call/cc operator — a non-standard control operation. The Curry-Howard reading makes its non-constructive nature clear: there is no function of this type in any normalizing type theory.

**De Morgan's Laws (one direction):** $\neg(A \vee B) \leftrightarrow \neg A \wedge \neg B$ — the direction $\neg(A \vee B) \to \neg A \wedge \neg B$ holds. But $\neg A \wedge \neg B \to \neg(A \vee B)$... also holds! The trouble is with $\neg(A \wedge B) \leftrightarrow \neg A \vee \neg B$. The direction $\neg A \vee \neg B \to \neg(A \wedge B)$ holds. But $\neg(A \wedge B) \to \neg A \vee \neg B$ fails: knowing the conjunction is false doesn't tell us which conjunct fails.

**Classical Tautologies That Hold in IPC:**
- $\neg\neg\neg A \to \neg A$ (triple negation reduces)
- $A \to \neg\neg A$ (double negation introduction)
- $(A \to B) \to (\neg B \to \neg A)$ (contraposition — the forward direction)
- $\neg A \vee B \to (A \to B)$ (from "not-A or B", derive "if A then B")

Wait — $(A \to B) \to (\neg B \to \neg A)$ holds: given $f : A \to B$ and $g : B \to \bot$, produce $g \circ f : A \to \bot$. This is the constructive content of contraposition.

## The Disjunction Property: Formal Statement and Proof

**Theorem (Disjunction Property for IPC).** If $\vdash_\text{IPC} A \vee B$, then $\vdash_\text{IPC} A$ or $\vdash_\text{IPC} B$.

*Proof sketch.* The proof uses normalization: normalize the proof of $A \vee B$. In a normal form derivation with no undischarged hypotheses, the subformula property says every formula is a subformula of $A \vee B$. The only introduction rules that produce $A \vee B$ are $\vee$I$_1$ (from a proof of $A$) and $\vee$I$_2$ (from a proof of $B$). In a normal form proof, the last step must be one of these two introduction rules (since the conclusion $A \vee B$ can only arise from an introduction), giving a proof of $A$ or a proof of $B$. $\square$

Note: this proof fails for classical logic. In classical logic, you can prove $A \vee B$ using LEM (as $A \vee \neg A$, which does not give a proof of $A$ or $\neg A$ separately) or indirect reasoning. The Disjunction Property is a distinctive feature of intuitionistic logic.

## The Existence Property

**Theorem (Existence Property for IPC/IQC).** If $\vdash_\text{IQC} \exists x, P(x)$, then there is a term $t$ such that $\vdash_\text{IQC} P(t)$.

Proof by normalization, analogous to DP. A normal form proof of $\exists x, P(x)$ from no hypotheses must have its last step as $\exists$I (introducing the existential), which requires exhibiting a specific term $t$ and a proof of $P(t)$. $\square$

This property fails classically: classical logic can prove $\exists n, (n = 0 \vee n = 1)$ by reasoning from LEM, without specifying $n = 0$ or $n = 1$.

## The Relationship Between IPC and CPC

Classical Propositional Calculus (CPC) is obtained from IPC by adding any of the following equivalent axioms:

1. LEM: $A \vee \neg A$ for all $A$
2. DNE: $\neg\neg A \to A$ for all $A$
3. Peirce's Law: $((A \to B) \to A) \to A$ for all $A, B$
4. RAA: from $\neg A \vdash \bot$, derive $A$

Any one of these gives all the others over IPC. The resulting system is exactly CPC: the classical propositional calculus.

**IPC is a strict subsystem of CPC.** Every IPC theorem is a CPC theorem (the rules of IPC are a subset of CPC's rules). But not vice versa: LEM is a CPC theorem that is not an IPC theorem. The containment is strict.

**How much of mathematics is constructive?** Surprisingly much. Most of undergraduate algebra — group theory, ring theory, field theory — can be done constructively: the proofs are constructive because they are explicit constructions. Most of combinatorics is constructive: a counting argument that works by finding a bijection is constructive. Most of elementary number theory is constructive.

The classical principles appear mainly in analysis (using the completeness of $\mathbb{R}$, which requires LEM for arbitrary propositions), in topology (when using non-constructive arguments about open sets), and in situations where existence is proved by contradiction without exhibiting a witness.

Bishop's constructive analysis (see Section 5) shows that most of classical analysis can be recovered with only minor modifications. The key technique is to replace "or" with "decidable or" — $P \vee \neg P$ is not assumed for all $P$, but is provable for many specific propositions arising in analysis.

## Propositional Formulas: Which Are IPC Theorems?

For propositional logic (no quantifiers), the question "is $\varphi$ an IPC theorem?" is decidable. We can determine this in finite time.

One algorithm: use the Kripke semantics (see Section 3). A propositional formula is an IPC theorem if and only if it is valid in all Kripke models. There are finitely many Kripke frames up to isomorphism for a given formula, and checking validity is decidable.

Another algorithm: tableaux methods or sequent proof search with the subformula property. Since IPC is a subsystem of CPC, we first check classical validity (easy). If the formula is not a classical tautology, it's not an IPC theorem. If it is a classical tautology, we check whether it requires LEM by attempting a sequent proof in **LJ** (the intuitionistic sequent calculus from Chapter 4), using the subformula property to bound the search space.

For *first-order* intuitionistic logic (IQC), the situation is more subtle: the theory is recursively enumerable (proofs can be enumerated) but not decidable (there is no decision procedure). This parallels first-order classical logic.

## Hereditary Harrop Formulas and Logic Programming

A practically important fragment of IPC is the *hereditary Harrop formulas* (HH formulas), used in the logic programming language $\lambda$Prolog.

HH formulas are defined inductively:
- Atomic formulas are HH.
- $A \wedge B$ and $\forall x, A$ are HH if $A$ and $B$ are HH.
- $G \to A$ is HH if $G$ is a "goal formula" and $A$ is HH.

Goal formulas are: atomic, conjunctions, disjunctions, existentials, and $A \Rightarrow G$ where $A$ is HH and $G$ is a goal.

Proof search in this fragment is well-behaved: backward chaining from a goal always terminates or makes progress in a well-defined sense. This is the basis of Prolog's SLD resolution and its higher-order generalization in $\lambda$Prolog, which uses the intuitionistic sequent calculus as its operational semantics.

## The Decidability of IPC Itself

Although the provability problem for full intuitionistic predicate logic is undecidable (like classical predicate logic), there is a beautiful decision procedure for intuitionistic *propositional* logic:

**PSPACE completeness:** The provability problem for IPC is PSPACE-complete (Statman 1979). This means it can be decided in polynomial space, but not (provably) in polynomial time unless P = PSPACE. The best known algorithms run in exponential time.

By comparison, classical propositional logic (satisfiability / tautology checking) is coNP-complete — much easier than IPC. Classical propositional logic has tautologies with short proofs (polynomial in the formula size); IPC requires longer proofs for some tautologies, which is related to its PSPACE complexity.

The PSPACE complexity of IPC is a striking example of how restricting the logic can make the decision problem *harder*, not easier: intuitionism's stricter demands on proofs make the provability problem more complex.
