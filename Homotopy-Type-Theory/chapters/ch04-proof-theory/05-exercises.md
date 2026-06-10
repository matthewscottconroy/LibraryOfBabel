# Exercises

---

**Exercise 4.1.** Construct natural deduction proofs (derivation trees) of each of the following sequents. Use only the rules from Section 2, starting with intuitionistic rules (no classical axioms):

(a) $P \to Q, P \vdash Q$  
(b) $\vdash P \to (Q \to P)$  
(c) $P \to Q, Q \to R \vdash P \to R$  
(d) $P \wedge Q \vdash Q \wedge P$  
(e) $P \to (Q \to R) \vdash (P \wedge Q) \to R$

---

**Exercise 4.2.** 
(a) Construct a proof of $\vdash \neg\neg(P \vee \neg P)$ in intuitionistic logic. (Double negation of excluded middle is intuitionistically valid!)

(b) Show that $\vdash P \vee \neg P$ is not provable in intuitionistic logic by giving a Heyting algebra semantics in which it fails. (Hint: Let the Heyting algebra be the open sets of $\mathbb{R}$, and evaluate $P$ as $(0, \infty)$.)

---

**Exercise 4.3.** 
(a) Show that $P \to Q \vdash \neg Q \to \neg P$ (contrapositive) is provable in intuitionistic logic.

(b) Show that $\neg Q \to \neg P \vdash P \to Q$ is NOT provable in intuitionistic logic, but IS provable in classical logic (by adding DNE).

This shows contrapositive is not an intuitionistic equivalence, only a one-way implication.

---

**Exercise 4.4.** The following are all equivalent in classical logic (each implies the others, over minimal intuitionistic logic):
- LEM: $A \vee \neg A$
- DNE: $\neg\neg A \to A$
- Peirce's Law: $((A \to B) \to A) \to A$

(a) Prove DNE $\to$ LEM in intuitionistic + DNE.

(b) Prove LEM $\to$ DNE in intuitionistic + LEM.

(c) Prove that Peirce's Law follows from LEM. (Hint: case split on $A$.)

---

**Exercise 4.5 (Curry-Howard).** Identify the type-theoretic interpretation of each connective and rule:

| Logic | Type Theory |
|---|---|
| $A \wedge B$ | ? |
| $A \to B$ | ? |
| $A \vee B$ | ? |
| $\bot$ | ? |
| $\neg A$ | ? |
| $\forall x : D, P(x)$ | ? |
| $\exists x : D, P(x)$ | ? |
| Proof of $A \wedge B$ | ? |
| Proof of $A \to B$ | ? |
| $\to$E (modus ponens) | ? |

---

**Exercise 4.6.** Identify each β-redex in the following derivation tree and carry out the β-reduction(s):

$$\frac{\dfrac{[P]^u \quad [Q]^v}{P \wedge Q}\wedge\text{I}}{Q}\wedge\text{E}_2$$

How many steps does it take to reach normal form? What is the normal form derivation?

---

**Exercise 4.7.** 
(a) A derivation is in normal form iff it contains no β-redexes. Show that the derivation for $P, Q \vdash P \wedge Q$ (proving a conjunction from both conjuncts) is already in normal form.

(b) Give an example of a derivation that is NOT in normal form, and normalize it.

---

**Exercise 4.8.** The *subformula property* says every formula in a normal form derivation is a subformula of the conclusion or hypotheses.

(a) Identify all subformulas of $P \to (Q \to (P \wedge Q))$.

(b) Give the normal form proof of $\vdash P \to (Q \to (P \wedge Q))$ and verify that every formula in the derivation tree is a subformula.

---

**Exercise 4.9.** Translate the following natural deduction proofs into sequent calculus derivations:

(a) The proof of $P \to Q, P \vdash Q$ (modus ponens).

(b) The proof of $\vdash P \to P$ (self-implication).

---

**Exercise 4.10.** Carry out a single cut elimination step on the following sequent calculus derivation:

$$\frac{\dfrac{}{\Rightarrow P \quad \Rightarrow Q}}{\Rightarrow P \wedge Q}\wedge R \qquad \frac{P, Q \Rightarrow R}{P \wedge Q \Rightarrow R}\wedge L$$

$$\text{Cut on } P \wedge Q: \quad \Rightarrow R$$

Eliminate the cut by replacing it with cuts on smaller formulas.

---

**Exercise 4.11.** The cut elimination procedure can cause an exponential blowup in proof length.

(a) Construct a family of proofs $(\pi_n)_{n \geq 1}$ of length $O(n)$ (each using $n$ cuts) such that any cut-free version has length $\geq 2^n$.

*Hint:* Use a chain of lemmas: prove $A_1$ from hypotheses, then use $A_1$ to prove $A_2$, etc., where each step uses the previous result. Eliminating cuts requires "inlining" all the lemma proofs.

(b) Explain why this exponential blowup doesn't contradict the usefulness of cut elimination.

---

**Exercise 4.12 (Sequent Calculus for Classical Logic).** In classical sequent calculus LK, the succedent can contain multiple formulas: $\Gamma \Rightarrow A_1, A_2, \ldots, A_n$ means "from $\Gamma$, at least one of $A_i$ holds."

(a) Show how to derive $\Rightarrow P \vee \neg P$ (LEM) in LK.

*Hint:* The proof uses the identity axiom $P \Rightarrow P$, weakening, and the $\vee$-rules.

(b) Show that the LK proof of LEM cannot be translated to a cut-free LJ (intuitionistic sequent calculus) proof.

---

**Exercise 4.13 (Research).** Gentzen's consistency proof for Peano Arithmetic (PA) uses transfinite induction up to $\epsilon_0$. Look up a description and answer:

(a) What is $\epsilon_0$? (In terms of the ordinal hierarchy $\omega, \omega^\omega, \omega^{\omega^\omega}, \ldots$)

(b) Gödel's second incompleteness theorem says PA cannot prove its own consistency. How does Gentzen's proof avoid contradicting this? (Hint: Gentzen uses a principle not provable in PA.)

(c) What does it mean for a consistency proof to be "stronger" or "weaker" than another?

---

**Exercise 4.14 (Conceptual — Proofs in HoTT).** In HoTT, the identity type $\text{Id}_A(a, b)$ plays the role of "proofs that $a = b$." Two proofs of the same equality are themselves equal only if there's a path (homotopy) between them.

(a) In natural deduction proof theory, two proofs of the same proposition are both "valid" but might be structurally different. How is this different from HoTT's treatment of identity proofs?

(b) In the interpretation of HoTT as homotopy theory: if proofs are paths, what is a "proof that two proofs are equal"?

(c) The Axiom K (or uniqueness of identity proofs, UIP) says: all proofs of $a = b$ are equal (the identity type is a proposition). Why does HoTT reject this axiom? (Hint: it would collapse all the higher homotopical structure.)
