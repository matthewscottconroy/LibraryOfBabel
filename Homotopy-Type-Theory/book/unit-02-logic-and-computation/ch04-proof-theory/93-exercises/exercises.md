# Exercises: Proof Theory

## Section 1: Judgments and Derivations

**Exercise 1.** Write out (in words and symbols) what each of the following judgments means. Indicate whether each is a theorem (provable from no hypotheses) or requires hypotheses.

(a) $P, Q \vdash P \wedge Q$
(b) $\vdash P \to P$
(c) $P \to Q, Q \to R \vdash P \to R$
(d) $\vdash (P \to Q) \to P \to Q$
(e) $P \vdash Q$

**Exercise 2.** State the structural rules of natural deduction (Identity, Weakening, Contraction, Exchange) using formal inference rule notation. For each rule, give a concrete example of when you would use it in an informal mathematical proof.

**Exercise 3.** In classical mathematics, we freely use any hypothesis as many times as we like. Which structural rule formalizes this? Explain what happens to a proof system if you *remove* this rule. Give a specific example of a valid classical argument that would fail in the resulting system.

**Exercise 4.** Explain the difference between:
(a) $\Gamma \vdash A$ (syntactic provability)
(b) $\Gamma \models A$ (semantic entailment)

What does the Soundness Theorem say? What does the Completeness Theorem say? For which logics does completeness hold?

**Exercise 5.** Draw a full derivation tree (with all rules labeled) for each of the following:

(a) $P \wedge Q \vdash Q \wedge P$ (commutativity of conjunction)
(b) $P \to Q, Q \to R, P \vdash R$ (transitivity of implication)
(c) $\vdash P \to (Q \to P)$ (weakening as a theorem)

**Exercise 6.** A formal derivation is an *inductive* structure. What does it mean to do induction on the *height* of a derivation? What does it mean to do induction on the *structure* (last rule applied) of a derivation? When would you use one versus the other?

## Section 2: Natural Deduction

**Exercise 7.** Derive (with a formal derivation tree) each of the following. Label every rule application.

(a) $\vdash A \to A$ (identity)
(b) $\vdash A \wedge B \to B \wedge A$ (commutativity of conjunction)
(c) $\vdash (A \to B \to C) \to (A \wedge B \to C)$ (uncurrying)
(d) $\vdash (A \wedge B \to C) \to (A \to B \to C)$ (currying)
(e) $\vdash A \to \neg\neg A$ (double negation introduction)

**Exercise 8.** Which of the following can be proved in *intuitionistic* natural deduction? For those that cannot, explain why not (what would a proof require?). For those that can, give the derivation.

(a) $\vdash A \vee \neg A$ (law of excluded middle)
(b) $\vdash \neg\neg A \to A$ (double negation elimination)
(c) $\neg\neg A \vdash \neg\neg\neg\neg A$ (quadruple negation introduction)
(d) $A \vee B, \neg A \vdash B$ (disjunctive syllogism)
(e) $\vdash \neg(A \wedge \neg A)$ (non-contradiction)
(f) $\vdash \neg\neg(A \vee \neg A)$ (stability of LEM)

**Exercise 9.** The classical rule of *reductio ad absurdum* (RAA) states: if $\Gamma, \neg A \vdash \bot$, then $\Gamma \vdash A$. Show that adding RAA to intuitionistic natural deduction allows you to prove the law of excluded middle $A \vee \neg A$.

**Exercise 10.** State the natural deduction rules for universal quantification ($\forall$) and existential quantification ($\exists$). For each rule, state the freshness condition if applicable and explain why it is necessary. Give an example of a proof that would be unsound if the freshness condition were dropped.

**Exercise 11.** Consider the following informal argument: "Let $n$ be any natural number. Then $n \geq 0$. Therefore, every natural number is $\geq 0$." Translate this into a formal derivation using the $\forall$I rule. Identify where the freshness condition is used.

**Exercise 12.** Under the Curry-Howard correspondence, what term (in the $\lambda$-calculus) corresponds to each of the following proofs?

(a) The proof of $A \to A$ (identity)
(b) The proof of $A \wedge B \to B \wedge A$
(c) The proof of $(A \to B) \to (B \to C) \to (A \to C)$
(d) The proof of $(A \to B) \wedge A \to B$

## Section 3: Normalization

**Exercise 13.** Identify all $\beta$-redexes in the following derivation and carry out one reduction for each:

"Prove $A$ from $A \wedge B$: first prove $A \wedge B$ (already assumed), extract $A$ (by $\wedge$E$_1$), then prove $A \wedge B$ again by pairing $A$ (just extracted) with $B$ (extracted by $\wedge$E$_2$), and finally extract $A$ again (by $\wedge$E$_1$)."

**Exercise 14.** Consider the derivation: assume $A \to B$ and $A$; apply modus ponens to get $B$; then use $\to$I to re-prove $A \to B$ (discharging the hypothesis $A$, deriving $B$ from $A \to B$ and $A$). Identify this as a $\beta$-redex (or not), and explain whether a reduction applies.

**Exercise 15.** Define what it means for a derivation to be in *normal form*. Give an example of a derivation in normal form and one not in normal form. Is every derivation in normal form minimal (shortest possible proof of the same conclusion from the same hypotheses)?

**Exercise 16.** State and prove (in outline) the **Subformula Property** for normal form derivations in intuitionistic natural deduction. That is: prove that in a normal form derivation of $\Gamma \vdash A$, every formula appearing in the derivation is a subformula of $A$ or of some $\varphi \in \Gamma$.

**Exercise 17.** Show that the consistency of intuitionistic natural deduction follows from the Normalization Theorem: that is, $\not\vdash \bot$ (there is no closed proof of $\bot$). Your argument should be purely syntactic (no appeal to semantics).

**Exercise 18.** The Normalization Theorem for STLC is typically proved using *logical relations* (Tait's method). Explain, in your own words:

(a) What the reducibility predicate $\text{Red}(A)$ is for base types and function types.
(b) Why CR1 (elements of $\text{Red}(A)$ are strongly normalizing) is the key property.
(c) What the main lemma says and why it implies strong normalization.

**Exercise 19.** The *eta-long* normal form of a term of type $A \to B$ is always a lambda abstraction $\lambda x.\, t$, even if the original term was not. Explain why eta-expansion is important for the structure of normal forms. When does eta-expansion change the behavior of a term versus only its notation?

## Section 4: Sequent Calculus

**Exercise 20.** Translate each of the following natural deduction rules into their sequent calculus equivalents:

(a) $\wedge$I (conjunction introduction)
(b) $\to$I (implication introduction)
(c) $\vee$E (disjunction elimination)

**Exercise 21.** In sequent calculus (**LK**), derive (with a full sequent derivation) the following:

(a) $A \wedge B \Rightarrow B \wedge A$
(b) $\Rightarrow A \to A$
(c) $A \to B, B \to C \Rightarrow A \to C$

**Exercise 22.** State Gentzen's Hauptsatz (Cut Elimination Theorem). What does it say? Why is it not obvious? Sketch the key step of the proof: what happens when the cut formula is *principal* on both sides?

**Exercise 23.** Use cut elimination to prove the consistency of **LK**: that $\Rightarrow$ (the empty sequent, representing a proof of $\bot$) is not derivable. Your argument should use the subformula property that follows from cut elimination.

**Exercise 24.** The sequent $A, \neg A \Rightarrow B$ is derivable in classical logic (**LK**) but its intuitionistic counterpart requires some care. Derive $A, A \to \bot \Rightarrow B$ in the intuitionistic sequent calculus **LJ** (single-conclusion version). Identify every rule used.

**Exercise 25.** In **LK** (classical sequent calculus), the succedent $\Delta$ is a multiset of formulas. In **LJ** (intuitionistic sequent calculus), $\Delta$ has at most one formula. Explain why this restriction captures the essence of intuitionistic logic. What classical theorem fails if we allow multiple conclusions on the right in **LJ**?

## Proof-Level Exercises

**Exercise 26.** Prove that the following rule is *admissible* in intuitionistic natural deduction — that is, whenever the premises are derivable, the conclusion is derivable, even though the rule itself is not one of the primitive rules:

$$\frac{\Gamma \vdash A \to B \quad \Gamma \vdash A \to C}{\Gamma \vdash A \to B \wedge C}$$

**Exercise 27.** Prove that the *Weakening* rule is admissible in natural deduction: if $\Gamma \vdash A$, then $\Gamma, B \vdash A$ for any $B$. Do this by induction on the structure of the derivation of $\Gamma \vdash A$.

**Exercise 28.** Define a measure of "size" for natural deduction derivations. Prove that every $\beta$-reduction strictly decreases the size of the derivation. Conclude that there are no infinite $\beta$-reduction sequences (giving a direct proof of normalization by well-founded induction, without using logical relations).

*Hint:* Consider the sum of the sizes of all formulas appearing as detour points.

**Exercise 29.** Prove that the disjunction property holds for intuitionistic propositional logic: if $\vdash P \vee Q$ is provable in IPC, then either $\vdash P$ or $\vdash Q$ is provable. Use the Normalization Theorem. (*Hint:* What does a normal form proof of $\vdash P \vee Q$ look like, given the subformula property?)

**Exercise 30.** Consider *minimal logic*: intuitionistic logic without the $\bot$E rule (ex falso quodlibet). Show that $\neg P \to P \to Q$ is provable in intuitionistic logic but not in minimal logic. (*Hint:* In minimal logic, $\neg P$ is an abbreviation for $P \to \bot$, but $\bot$ is just an atomic proposition with no special rules.)
