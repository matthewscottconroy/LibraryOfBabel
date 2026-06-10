# Exercises: Intuitionistic Logic

## Section 1: The BHK Interpretation

**Exercise 1.** State the BHK interpretation for each of the following:

(a) $P \wedge Q$
(b) $P \vee Q$
(c) $P \to Q$
(d) $\neg P$
(e) $\forall x : A, P(x)$
(f) $\exists x : A, P(x)$

For each, explain what computational object a proof would be.

**Exercise 2.** Under the BHK interpretation, construct explicit proof objects for the following tautologies:

(a) $P \to P$
(b) $P \wedge Q \to Q \wedge P$
(c) $(P \to Q) \to (Q \to R) \to P \to R$
(d) $(P \to Q \to R) \to (P \wedge Q \to R)$
(e) $P \to \neg\neg P$

**Exercise 3.** Explain, in BHK terms, why the following cannot have a constructive proof:

(a) $P \vee \neg P$ (for an arbitrary $P$)
(b) $\neg\neg P \to P$ (for an arbitrary $P$)
(c) $\neg(P \wedge Q) \to \neg P \vee \neg Q$ (compare with the direction that is provable)

**Exercise 4.** The BHK interpretation says a proof of $P \to Q$ is a *function* from proofs of $P$ to proofs of $Q$. What kind of function? In informal constructivism (Brouwer), it is a "mental construction." In realizability (Kleene), it is a recursive function. In type theory (Martin-Löf), it is a term of the function type $P \to Q$.

For each of the following, state what the BHK proof objects are in the type-theoretic interpretation:

(a) A proof of $A \wedge B$
(b) A proof of $A \vee B$ (when the left disjunct holds)
(c) A proof of $\exists n : \mathbb{N}, n > 5$

**Exercise 5.** Prove constructively (by exhibiting explicit constructions) that:

(a) $\neg\neg\neg A \leftrightarrow \neg A$
(b) $\neg(A \vee B) \leftrightarrow \neg A \wedge \neg B$
(c) $\neg A \vee \neg B \to \neg(A \wedge B)$ (and show the converse fails constructively)

## Section 2: Formal Intuitionistic Logic

**Exercise 6.** For each of the following formulas, determine whether it is an IPC theorem, a CPC theorem but not IPC, or neither:

(a) $A \vee \neg A$
(b) $\neg\neg A \to A$
(c) $A \to \neg\neg A$
(d) $\neg(A \wedge B) \to \neg A \vee \neg B$
(e) $\neg A \vee \neg B \to \neg(A \wedge B)$
(f) $(A \to B) \vee (B \to A)$
(g) $\neg A \vee B \to (A \to B)$
(h) $(A \to B) \to \neg A \vee B$

For CPC-only theorems, explain what classical principle is required.

**Exercise 7.** Prove formally in IPC (with a derivation tree) that each of the following holds:

(a) $A \vee B, \neg A \vdash B$ (disjunctive syllogism — note this is IPC-valid!)
(b) $\neg A \wedge \neg B \vdash \neg(A \vee B)$
(c) $A \to B \vdash \neg B \to \neg A$ (contraposition)
(d) $A \to B, B \to C \vdash \neg C \to \neg A$

**Exercise 8.** Show that the following are equivalent over IPC (i.e., adding any one of them to IPC gives classical logic):

(a) $A \vee \neg A$ (LEM)
(b) $\neg\neg A \to A$ (DNE)
(c) $((A \to B) \to A) \to A$ (Peirce's Law)
(d) $\neg A \vee A$ (LEM, symmetric form)

*Hint:* Show each implies all the others over IPC.

**Exercise 9.** Prove the Disjunction Property for IPC formally: use normalization to argue that a closed IPC proof of $A \vee B$ must end with a $\vee$I rule. Which properties of normal form proofs are needed?

**Exercise 10.** The *Gödel-Dummett logic* (LC) extends IPC with the axiom $(A \to B) \vee (B \to A)$. This is an intermediate logic between IPC and CPC.

(a) Show that this axiom is not IPC-valid (construct a Kripke countermodel).
(b) Show that it is not CPC-equivalent (by finding a CPC tautology not in LC, if one exists).
(c) What does the axiom $(A \to B) \vee (B \to A)$ mean in BHK terms?

## Section 3: Kripke Semantics

**Exercise 11.** Let $W = \{w_0, w_1, w_2\}$ with $w_0 \leq w_1$, $w_0 \leq w_2$, and $w_1$ and $w_2$ incomparable. Let $V(p, w_1) = 1$, $V(p, w_2) = 0$, and $V(q, w) = 0$ for all $w$.

(a) Compute $w_0 \Vdash p \vee \neg p$.
(b) Compute $w_0 \Vdash \neg\neg p$.
(c) Compute $w_0 \Vdash \neg\neg p \to p$.
(d) Compute $w_0 \Vdash (p \to q) \vee (q \to p)$.

**Exercise 12.** Prove the Monotonicity Lemma for Kripke semantics: if $w \Vdash A$ and $w \leq v$, then $v \Vdash A$. Do this by induction on the structure of $A$.

**Exercise 13.** Construct a Kripke model falsifying each of the following:

(a) LEM: $A \vee \neg A$
(b) DNE: $\neg\neg A \to A$
(c) De Morgan: $\neg(A \wedge B) \to \neg A \vee \neg B$

**Exercise 14.** Show that the following are valid in all Kripke models (and hence are IPC theorems):

(a) $\neg\neg(A \vee \neg A)$ (stability of LEM)
(b) $\neg(A \wedge \neg A)$ (non-contradiction)
(c) $\neg A \vee \neg B \to \neg(A \wedge B)$

**Exercise 15.** The topological semantics interprets IPC using open sets of a topological space. Show that:

(a) The interpretation of $A \to B$ is $\text{Int}(A^c \cup B)$ (interior of the implication).
(b) The interpretation of $\neg A$ is $\text{Int}(A^c)$ (interior of the complement).
(c) LEM fails in general because $\text{Int}(A^c \cup A)$ need not equal the whole space.

Give a specific topological space and open set $A$ where $A \cup \text{Int}(A^c) \neq X$.

## Section 4: The Double-Negation Translation

**Exercise 16.** Compute the Gödel-Gentzen translation $\varphi^N$ for:

(a) $p \vee \neg p$ (LEM)
(b) $\neg\neg p \to p$ (DNE)
(c) $\exists x, P(x)$
(d) $\forall x, P(x) \vee \neg P(x)$ (decidability of $P$)

**Exercise 17.** Prove that $\varphi^N$ is always $\neg\neg$-stable: $\vdash_\text{IPC} \neg\neg(\varphi^N) \to \varphi^N$.

**Exercise 18.** Show that $\vdash_\text{IPC} (A \vee \neg A)^N$. That is, the double-negation translation of LEM is an IPC theorem. *Hint:* What is $(A \vee \neg A)^N$? It should be $\neg\neg(\neg\neg A \vee \neg\neg\neg A)$, or something equivalent. Prove this holds in IPC.

**Exercise 19.** Explain the following apparent paradox: the double-negation translation shows that every classical theorem is intuitionistically provable (when translated). But IPC is a *strict subsystem* of CPC (it proves fewer things). How can classical theorems translate to intuitionistic theorems if classical logic is stronger?

**Exercise 20.** A proposition $A$ is called *stable* if $\neg\neg A \to A$ holds intuitionistically. Show that:

(a) $\neg A$ is always stable (for any $A$).
(b) $A \wedge B$ is stable if both $A$ and $B$ are stable.
(c) $A \vee B$ need not be stable even if both $A$ and $B$ are stable.
(d) $\forall x, A(x)$ is stable if $A(x)$ is stable for all $x$.

## Section 5: Decidability and Constructive Mathematics

**Exercise 21.** Show that equality of natural numbers is decidable: prove $\forall m, n : \mathbb{N}, (m = n) \vee \neg(m = n)$ by induction.

**Exercise 22.** Suppose $P$ and $Q$ are decidable propositions. Show that the following are also decidable:

(a) $P \wedge Q$
(b) $P \vee Q$
(c) $P \to Q$
(d) $\neg P$

**Exercise 23.** Show that if $P : \mathbb{N} \to \text{Prop}$ is decidable and $n : \mathbb{N}$, then $\exists k \leq n, P(k)$ is decidable. (*Hint:* Check $P(0), \ldots, P(n)$ in sequence.)

**Exercise 24.** Explain Hedberg's Theorem: if a type $A$ has decidable equality, then it is an h-set. Sketch the proof. (*Hint:* Given decidable equality, for each $a, b : A$, either we have a canonical proof of $a = b$ or a proof of $\neg(a = b)$. Use this canonical proof to contract the space of all proofs of $a = b$.)

**Exercise 25.** In HoTT, the axiom of propositional LEM (pLEM) asserts: for every mere proposition $P$, $P \vee \neg P$. Explain why pLEM is consistent with univalence but full LEM (for all types, not just mere propositions) would be inconsistent with the higher-dimensional structure.

## Proof-Level Exercises

**Exercise 26.** Prove the *Existence Property* for IQC using normalization: if $\vdash_\text{IQC} \exists x : A, P(x)$, then there exists a closed term $t$ with $\vdash_\text{IQC} P(t)$. Your proof should identify what a normal form proof of $\exists x : A, P(x)$ (from no hypotheses) must look like.

**Exercise 27.** Prove, using Kripke semantics, that IPC is strictly weaker than CPC. Specifically, exhibit a formula that is CPC-valid but not IPC-valid, and construct a Kripke model falsifying it.

**Exercise 28.** Prove the *Gödel-Gentzen theorem* in one direction: if $\vdash_\text{CPC} \varphi$, then $\vdash_\text{IPC} \varphi^N$. Do this by showing each CPC rule preserves the translated form. The key case is LEM: what does $(A \vee \neg A)^N$ reduce to, and is it IPC-valid?

**Exercise 29.** Bishop's constructive intermediate value theorem says: if $f : [0,1] \to \mathbb{R}$ is uniformly continuous with $f(0) < 0 < f(1)$, then for every $\varepsilon > 0$ there exists $c \in (0,1)$ with $|f(c)| < \varepsilon$. Sketch why this can be proved constructively using binary search, and explain what additional assumption (uniform continuity vs. pointwise continuity) makes the argument work.

**Exercise 30.** Markov's Principle (MP) for decidable predicates states: if $P : \mathbb{N} \to \text{Prop}$ is decidable and $\neg\neg \exists n, P(n)$, then $\exists n, P(n)$. Explain why MP is *computationally justified* (there is a program that, if it terminates, gives the witness) but *not provable in IPC*. Construct a Kripke model falsifying MP.
