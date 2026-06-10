# 4.1 Sequent Calculus and Cut Elimination

## From Natural Deduction to Sequent Calculus

Natural deduction gives a beautiful account of proofs as tree-structured derivations with introduction and elimination rules. But it has some asymmetries: introduction rules "build up" connectives (going from smaller formulas to larger), while elimination rules "break down" connectives (going from larger to smaller). This asymmetry makes some theoretical analyses more complex.

Gentzen's second invention, *sequent calculus*, is designed to be more symmetric. It makes the structure of proofs easier to analyze, and it's the natural setting for proving cut elimination.

## Sequents

**Definition.** A *sequent* is an expression of the form:
$$\Gamma \Rightarrow \Delta$$

where $\Gamma$ and $\Delta$ are finite (possibly empty) lists of formulas. Read: "From all of $\Gamma$, at least one of $\Delta$" or formally "if $\bigwedge \Gamma$ then $\bigvee \Delta$."

$\Gamma$ is the *antecedent* (hypotheses) and $\Delta$ is the *succedent* (conclusions).

This is the key difference from natural deduction: in sequent calculus, the right-hand side $\Delta$ is a *list of possible conclusions*, not a single formula. The multi-conclusion setting gives the calculus additional symmetry.

**Special cases:**
- $\Gamma \Rightarrow A$: the usual judgment "from $\Gamma$, derive $A$."
- $\Gamma \Rightarrow$: from $\Gamma$, derive a contradiction (the succedent is empty).
- $\Rightarrow A$: $A$ is a theorem.

For intuitionistic logic, we restrict to *single-conclusion sequents*: $\Gamma \Rightarrow A$ where $\Delta$ has exactly one formula. Classical logic allows multiple conclusions.

## Structural Rules

The sequent calculus begins with structural rules (which we've seen before):

**Identity (Axiom):**
$$\overline{A \Rightarrow A}$$

A single formula on each side — the most basic sequent.

**Weakening (left and right):**
$$\frac{\Gamma \Rightarrow \Delta}{A, \Gamma \Rightarrow \Delta} \quad W_L \qquad \frac{\Gamma \Rightarrow \Delta}{\Gamma \Rightarrow \Delta, A} \quad W_R$$

**Contraction (left and right):**
$$\frac{A, A, \Gamma \Rightarrow \Delta}{A, \Gamma \Rightarrow \Delta} \quad C_L \qquad \frac{\Gamma \Rightarrow \Delta, A, A}{\Gamma \Rightarrow \Delta, A} \quad C_R$$

**Exchange (left and right):**
$$\frac{\Gamma, A, B, \Delta \Rightarrow \Theta}{\Gamma, B, A, \Delta \Rightarrow \Theta} \quad E_L \qquad \frac{\Gamma \Rightarrow \Delta, A, B, \Theta}{\Gamma \Rightarrow \Delta, B, A, \Theta} \quad E_R$$

(Often absorbed into a multiset convention where order doesn't matter.)

## Logical Rules in Sequent Calculus

For each connective, there are **left rules** (rules that introduce the connective in the antecedent) and **right rules** (rules that introduce the connective in the succedent).

**Conjunction:**
$$\frac{A, \Gamma \Rightarrow \Delta}{A \wedge B, \Gamma \Rightarrow \Delta} \quad \wedge L_1 \qquad \frac{B, \Gamma \Rightarrow \Delta}{A \wedge B, \Gamma \Rightarrow \Delta} \quad \wedge L_2$$

$$\frac{\Gamma \Rightarrow \Delta, A \qquad \Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \wedge B} \quad \wedge R$$

**Disjunction:**
$$\frac{A, \Gamma \Rightarrow \Delta \qquad B, \Gamma \Rightarrow \Delta}{A \vee B, \Gamma \Rightarrow \Delta} \quad \vee L$$

$$\frac{\Gamma \Rightarrow \Delta, A}{\Gamma \Rightarrow \Delta, A \vee B} \quad \vee R_1 \qquad \frac{\Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \vee B} \quad \vee R_2$$

**Implication:**
$$\frac{\Gamma \Rightarrow \Delta, A \qquad B, \Gamma \Rightarrow \Delta}{A \to B, \Gamma \Rightarrow \Delta} \quad \to L$$

$$\frac{A, \Gamma \Rightarrow \Delta, B}{\Gamma \Rightarrow \Delta, A \to B} \quad \to R$$

The symmetry is now visible: for each connective, there is a left rule (how to use the connective from the left) and a right rule (how to prove the connective on the right). The left rules correspond to elimination rules in natural deduction; the right rules correspond to introduction rules.

## The Cut Rule

$$\frac{\Gamma \Rightarrow \Delta, A \qquad A, \Pi \Rightarrow \Lambda}{\Gamma, \Pi \Rightarrow \Delta, \Lambda} \quad \text{Cut}$$

Cut formalizes "lemma use": prove $A$ in one branch, use $A$ in another branch, and combine.

The formula $A$ that is introduced in the right premise and used in the left premise is called the *cut formula*. It does not appear in the conclusion.

**Why cut is interesting.** The cut formula $A$ can be *any* formula — it doesn't need to be a subformula of the conclusion. This is what makes cut "unanalytic": it allows the proof to use auxiliary formulas not directly related to what's being proved.

The cut rule is clearly sound (if you can prove $A$ and prove $\psi$ from $A$, you can prove $\psi$). The deep question is: is it necessary?

## Cut Elimination: The Hauptsatz

**Theorem (Gentzen's Hauptsatz, 1935).** Every sequent provable in LK (classical sequent calculus) or LJ (intuitionistic sequent calculus) with cut has a cut-free proof.

In other words: cut is *admissible* — every provable sequent has a proof that doesn't use cut.

This is Gentzen's central theorem. It took him years to prove, and the proof is highly technical (a double induction on the complexity of the cut formula and the complexity of the proof above each cut). But the consequences are enormous.

**Proof idea.** The proof proceeds by showing that each occurrence of the cut rule can be "pushed up" toward the leaves of the derivation tree, where it can be eliminated by direct inspection. The critical case is when the cut formula is "principal" in both premises (introduced by a right rule on one side and a left rule on the other). In this case, the cut can be replaced by smaller cuts on the subformulas.

For example, if a cut on $A \wedge B$ arises from a $\wedge R$ on the right and a $\wedge L_1$ on the left:

$$\frac{\Gamma \Rightarrow A \quad \Gamma \Rightarrow B}{\Gamma \Rightarrow A \wedge B} \wedge R \qquad \frac{A, \Pi \Rightarrow \Lambda}{A \wedge B, \Pi \Rightarrow \Lambda} \wedge L_1$$

$$\text{Cut on } A \wedge B: \quad \frac{\Gamma \Rightarrow A \wedge B \qquad A \wedge B, \Pi \Rightarrow \Lambda}{\Gamma, \Pi \Rightarrow \Lambda}$$

This can be replaced by a cut on $A$ alone (a smaller formula):

$$\frac{\Gamma \Rightarrow A \qquad A, \Pi \Rightarrow \Lambda}{\Gamma, \Pi \Rightarrow \Lambda} \text{Cut on }A$$

The cut formula has been reduced in complexity. Iterating this process (with careful bookkeeping about the size of cuts and the complexity of the proof above) eventually eliminates all cuts. $\square$

## Consequences of Cut Elimination

Cut elimination has sweeping consequences:

**1. Consistency.** If we prove $\bot$ (inconsistency), the proof uses only formulas that are subformulas of $\bot$. But $\bot$ has no subformulas, and there are no axioms of the form $\Gamma \Rightarrow \bot$. So no cut-free proof of $\bot$ exists, and by cut elimination, no proof at all. The system is consistent.

This is one of the cleanest consistency proofs in logic. It follows purely from the syntactic properties of the proof system.

**2. The subformula property.** Every formula appearing in a cut-free proof is a subformula of the conclusion or hypotheses. Proofs are analytic.

**3. Separation.** Classical logic (**LK**) and intuitionistic logic (**LJ**) are different systems with different provable sequents. Cut elimination makes this clear: a cut-free proof in **LK** uses double negation elimination at the leaf level (classical axioms), while a cut-free proof in **LJ** doesn't.

**4. Herbrand's theorem.** For first-order logic: if $\vdash \exists x, A(x)$, then there exist finitely many terms $t_1, \ldots, t_n$ with $\vdash A(t_1) \vee \cdots \vee A(t_n)$. This follows from cut elimination plus the structure of quantifier rules.

**5. Craig's interpolation theorem.** If $\Gamma \vdash \varphi$ and $\psi$ is any formula, there is an interpolant $\chi$ (using only the vocabulary common to $\Gamma$ and $\varphi$) with $\Gamma \vdash \chi$ and $\chi \vdash \varphi$. This has applications in model theory, complexity theory, and verification.

**6. Decidability.** For propositional logic, cut elimination gives a decision procedure: search for cut-free proofs by backward chaining from the goal (since all formulas in a cut-free proof are subformulas of the goal, the search space is finite).

## The Cost of Cut Elimination

There's a price: cut elimination can cause an *exponential blowup* in proof length.

A proof with cut can be polynomial in the size of the theorem. The corresponding cut-free proof might be exponential. This is related to the $\mathsf{P} \neq \mathsf{NP}$ question: verifying a proof with cuts is fast (polynomial), but finding a cut-free proof might require exponential search.

More formally: the cut-free proof system **LK** characterizes non-deterministic exponential time (**NEXP**), while the number of cut-rule applications needed to prove tautologies in polynomial-size proofs is related to the complexity of the polynomial hierarchy.

This connection between cut elimination and computational complexity is an active area of research in *proof complexity*.

## Curry-Howard for Sequent Calculus

The Curry-Howard correspondence extends to sequent calculus, giving:
- Sequents $\Gamma \Rightarrow A$ correspond to typing judgments $x_1 : A_1, \ldots \vdash t : A$.
- The cut rule corresponds to substitution / let-binding.
- Cut-free proofs correspond to programs in a particular *normal form*.

The resulting calculus is called the *sequent calculus term assignment* or the *λ-calculus with explicit substitutions*.

Cut elimination then corresponds to reduction in this term calculus — another facet of the computation-proof correspondence.

## Looking Forward: From Proof Theory to Type Theory

Proof theory gives us the foundational perspective on formal proofs:
- Proofs are structured, tree-shaped objects.
- They have normal forms (no redundant reasoning).
- The structure of normal form proofs is constrained to the vocabulary of the conclusion.
- The system's consistency follows from the structure of proofs.

Type theory takes this one step further: it doesn't just study proofs as formal objects, but *internalizes* proofs as terms of a system. In type theory:
- Propositions are types.
- Proofs are terms.
- Equality of proofs (propositional equality) is a type.
- Higher equalities (paths between paths) are also types.

This is the Curry-Howard correspondence, which we develop fully in the next chapter. The sequent calculus perspective continues to inform type theory through the lens of *focused proof search* and *polarized type theories*, which structure the type theory to mirror the analytic structure of cut-free sequent proofs.

The key table to keep in mind:

| Proof Theory | Type Theory |
|---|---|
| Proposition $A$ | Type $A$ |
| Proof of $A$ | Term $t : A$ |
| Hypothesis $A$ | Variable $x : A$ |
| Introduction rule | Constructor / $\lambda$-abstraction |
| Elimination rule | Eliminator / application |
| β-reduction | Computation (definitional equality) |
| Normalization | Strong normalization (all programs terminate) |
| Cut | Substitution |
| Cut elimination | Subject reduction (types are preserved by computation) |
| Consistency | Canonicity (every term of boolean type is true or false) |

This correspondence is the backbone of the rest of the curriculum.
