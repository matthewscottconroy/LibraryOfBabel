# Exercises

---

**Exercise 5.1 (BHK Interpretation).** For each of the following propositions, describe explicitly the function (program) that constitutes a BHK proof:

(a) $P \to P$ (identity)

(b) $P \to Q \to P$ (const)

(c) $(P \to Q \to R) \to (P \to Q) \to P \to R$ (S combinator)

(d) $P \wedge Q \to Q \wedge P$ (commutativity of conjunction)

---

**Exercise 5.2 (Intuitionistic Theorems).** Prove in IPC (without DNE or LEM):

(a) $P \to \neg\neg P$

(b) $\neg\neg\neg P \leftrightarrow \neg P$

(c) $\neg(P \vee Q) \leftrightarrow \neg P \wedge \neg Q$ (De Morgan)

(d) $\neg P \vee \neg Q \to \neg(P \wedge Q)$ (one direction of De Morgan — prove the other direction fails)

---

**Exercise 5.3.** Prove in IPC that $\neg\neg(P \vee \neg P)$ is provable even though $P \vee \neg P$ is not. Interpret this computationally: what does a proof of $\neg\neg(P \vee \neg P)$ compute?

---

**Exercise 5.4 (Kripke Models).** 

(a) Construct a Kripke model with 2 worlds showing that $(P \to Q) \vee (Q \to P)$ is not intuitionistically valid. (This is a classical tautology!)

(b) Construct a Kripke model showing that $\neg P \vee \neg Q \to \neg(P \wedge Q)$ is not reversible: find a model where $\neg(P \wedge Q)$ holds but $\neg P \vee \neg Q$ fails.

---

**Exercise 5.5.** Apply the Gödel-Gentzen translation to:

(a) $P \vee \neg P$ (LEM) — write out $(P \vee \neg P)^\circ$ explicitly.

(b) $\neg\neg P \to P$ (DNE) — write out $(\neg\neg P \to P)^\circ$.

Verify that both translations are intuitionistically provable.

---

**Exercise 5.6 (Contrapositive).** 

(a) Prove that $(A \to B) \to (\neg B \to \neg A)$ holds in IPC.

(b) Prove that $(\neg B \to \neg A) \to (A \to B)$ does NOT hold in IPC by constructing a Kripke countermodel.

(c) Using Gödel-Gentzen, show that $(\neg\neg B \to \neg A) \to (A \to \neg\neg B)$ is provable in IPC (the "classical" contrapositive in translated form).

---

**Exercise 5.7 (Peirce's Law).** 

(a) Show that Peirce's Law $((A \to B) \to A) \to A$ implies LEM over IPC.

(b) Find a Kripke model where Peirce's Law fails.

(c) What is the computational interpretation (if any) of Peirce's Law? (Hint: it's related to continuations in programming languages.)

---

**Exercise 5.8 (Markov's Principle).** Assume Markov's Principle: if $P : \mathbb{N} \to \{0,1\}$ is decidable and $\neg\neg\exists n, P(n) = 1$, then $\exists n, P(n) = 1$.

(a) Use Markov's Principle to prove: if a Turing machine is guaranteed not to run forever (proven by contradiction), then it terminates. Explain why this is computationally reasonable.

(b) Show that Markov's Principle is not constructively provable without extra assumptions.

---

**Exercise 5.9 (Bishop's Constructivism).** The classical Intermediate Value Theorem says: if $f : [0,1] \to \mathbb{R}$ is continuous and $f(0) < 0 < f(1)$, then there exists $x \in [0,1]$ with $f(x) = 0$.

(a) Explain why this is not constructively valid as stated. (Hint: what if $f(x) = (x - \pi/4) \cdot (x - \pi/3)$ and you're working computably?)

(b) State Bishop's constructive version: for any $\varepsilon > 0$, there exists $x \in [0,1]$ with $|f(x)| < \varepsilon$.

(c) Prove the constructive version by bisection. Does this proof give a computable algorithm?

---

**Exercise 5.10 (Decidable Equality).** 

(a) Prove that $\mathbb{N}$ has decidable equality (constructively, without LEM).

(b) Prove that if $A$ and $B$ both have decidable equality, so does $A \times B$ and $A + B$.

(c) Prove that if $A$ has decidable equality and $f : B \to A$ is injective, then $B$ has decidable equality.

---

**Exercise 5.11 (Propositional Truncation).** In HoTT, the *propositional truncation* $\|A\|$ of a type $A$ is the mere proposition obtained by "forgetting" the computational content of $A$. It satisfies:
- There is a map $|-| : A \to \|A\|$.
- $\|A\|$ is a mere proposition.
- (Universal property) For any mere proposition $P$ and function $f : A \to P$, there is a unique $g : \|A\| \to P$ with $g \circ |-| = f$.

(a) What is the relationship between $\neg\neg A$ and $\|A\|$? Are they equivalent? (Hint: there are maps in both directions in some cases.)

(b) Under LEM (all propositions are decidable), show that $\|A\| \leftrightarrow \neg\neg A$ for all mere propositions $A$.

(c) In what sense does $\|A\|$ represent "classical existence" of an element of $A$?

---

**Exercise 5.12 (Research).** Kleene's *realizability* interpretation (1945) formalizes BHK using recursive functions:
- A natural number $n$ *realizes* a proposition $P$ if the $n$-th Turing machine provides evidence for $P$.

Look up and describe:

(a) How is the Axiom of Choice interpretable in realizability? (Hint: the "choice function" is just a recursive function.)

(b) Markov's principle holds in the effective topos (the topos of recursive functions). What does this mean?

(c) How does realizability differ from topological/Kripke models of IPC?
