# 1.1 The BHK Interpretation: What Is a Constructive Proof?

## The Central Question

Before we write down any formal rules, let's try to understand constructive logic from first principles. The key question is: **what is a proof?**

In classical logic, a proof is a finite derivation that establishes the truth of a proposition. The truth of the proposition is taken as an absolute fact: either the proposition is true or it's false, and a proof is a certificate of truth.

In constructive logic, the view is different: a proof is a **construction** — an explicit witness, method, or procedure. A proposition is "true" only insofar as we have a construction for it.

This shift in perspective gives rise to the **Brouwer-Heyting-Kolmogorov (BHK) interpretation**, named after the three mathematicians who articulated it.

## The BHK Clauses

The BHK interpretation specifies what it means to *have a proof* of each kind of proposition. It's recursive: the clauses for compound propositions reduce to clauses for their components.

---

**A proof of $P \wedge Q$** is a pair $(p, q)$ where $p$ is a proof of $P$ and $q$ is a proof of $Q$.

This is unambiguous and entirely natural. To prove "it will rain and it will snow," you need both a proof that it will rain and a proof that it will snow.

---

**A proof of $P \vee Q$** is either:
- A tagged pair $\mathsf{inl}(p)$ where $p$ is a proof of $P$, or
- A tagged pair $\mathsf{inr}(q)$ where $q$ is a proof of $Q$.

A proof of a disjunction must say *which* disjunct holds and provide evidence for it. This is stronger than the classical case, where "P or Q is true" doesn't specify which.

Example: A classical proof of "either there are infinitely many twin primes or there are not" is trivially valid (it's an instance of LEM). But a constructive proof would require you to either exhibit an algorithm producing infinitely many twin primes or to prove that no such algorithm exists. We don't have either.

---

**A proof of $P \to Q$** is a **function**: a method $f$ that converts any proof $p$ of $P$ into a proof $f(p)$ of $Q$.

This is the heart of the BHK interpretation. An implication is not just a statement about truth values — it's a computational procedure. To prove "if it's raining, then the ground is wet," you must specify: given any evidence that it's raining, how would you produce evidence that the ground is wet?

This function $f$ should be *effective*: given a proof of $P$, it actually produces a proof of $Q$, not just proves that one exists.

---

**$\bot$ (false) has no proof.** There is no construction for falsehood.

---

**A proof of $\neg P$** (which is $P \to \bot$) is a function that converts any proof of $P$ into a proof of $\bot$.

Since $\bot$ has no proof, this means: $\neg P$ asserts that $P$ can have no proof (any alleged proof of $P$ would let you construct an impossible object).

This is a weaker statement than classical negation! Classically, $\neg P$ means $P$ is false. Constructively, $\neg P$ means "I have a method showing that a proof of $P$ would be contradictory." These coincide when we have enough information, but they diverge in cases of ignorance.

---

**A proof of $\forall x : A, P(x)$** is a function $f$ such that, for every element $a : A$, $f(a)$ is a proof of $P(a)$.

This is a *dependent* function: the output type depends on the input. For a proof of "every even number is the sum of two primes" (Goldbach's conjecture), you'd need a function that, given any even number $n$, produces two primes that sum to $n$. We don't have this.

---

**A proof of $\exists x : A, P(x)$** is a pair $(a, p)$ where $a : A$ is the witness and $p$ is a proof of $P(a)$.

Crucially: the witness $a$ is part of the proof. A constructive existence proof is not just evidence that something exists somewhere — it contains the *actual thing* that exists. This is why constructive existence proofs have algorithmic content.

Compare:
- Classical: "There exists a real solution to $x^2 - 2 = 0$." Proof: assume no solution exists, derive a contradiction using properties of $\mathbb{R}$.
- Constructive: "There exists a real solution to $x^2 - 2 = 0$." Proof: exhibit $\sqrt{2}$ and verify it satisfies the equation. The proof contains the solution.

---

## The Disjunction Property

The BHK interpretation immediately implies a key property of constructive logic:

**Disjunction Property:** If there is a BHK proof of $P \vee Q$, then there is a BHK proof of $P$ or a BHK proof of $Q$ (and the proof of the disjunction tells us which).

This is obvious from the definition: a proof of $P \vee Q$ is either $\mathsf{inl}(p)$ (giving a proof of $P$) or $\mathsf{inr}(q)$ (giving a proof of $Q$).

Classical logic does not have this property. Classical logic proves $P \vee \neg P$ for every $P$, without proving $P$ or proving $\neg P$.

## The Existence Property

Similarly:

**Existence Property:** If there is a BHK proof of $\exists x : A, P(x)$, then there exists a specific element $a : A$ and a BHK proof of $P(a)$.

Again obvious from the definition: a proof of $\exists x : A, P(x)$ contains a witness $a$ and a proof of $P(a)$.

Classical logic does not have this property: you can classically prove "there exists an $n$ such that $n = 0$ or $n = 1$" without specifying which one. (The proof: $0 = 0$ implies $\exists n, n = 0 \vee n = 1$ by taking $n = 0$. But that's specific — the real example is more subtle: use LEM on $P$ to get $P \vee \neg P$, then in each case find an $n$ that might differ.)

## Why the Law of Excluded Middle Fails

The BHK interpretation immediately shows why LEM ($P \vee \neg P$) is not constructively valid in general.

A BHK proof of $P \vee \neg P$ would be: for every proposition $P$, either a proof of $P$ or a proof of $\neg P$. This means: we have a *decision procedure* for all propositions. For every mathematical statement, we can decide (in finite time) whether it's true or false.

But this is clearly false: we cannot decide whether the Riemann Hypothesis is true. We cannot decide whether Goldbach's conjecture holds. We cannot even decide whether an arbitrary Turing machine halts (the halting problem is undecidable).

The BHK interpretation connects LEM to decidability: LEM would give us a universal decision procedure, which doesn't exist.

## Double Negation: The Classical Embedding

Despite rejecting LEM, constructive logic does accept weaker forms:

**$P \to \neg\neg P$:** From a proof $p$ of $P$, we can prove $\neg\neg P$ by the function $\lambda f: (P \to \bot). f(p)$. Given any proof of $\neg P$ (i.e., any function $f: P \to \bot$), we apply $f$ to $p$ to get a proof of $\bot$. This is a BHK proof of $\neg\neg P$.

So: every truth can be doubly-negated. But the converse, $\neg\neg P \to P$, is not constructively valid. A proof of $\neg\neg P$ says "any proof of $\neg P$ leads to contradiction" — but this doesn't give us a proof of $P$ directly.

**$\neg\neg(P \vee \neg P)$:** We can prove this! Let $f: (P \vee \neg P) \to \bot$ be a hypothetical proof of $\neg(P \vee \neg P)$. Then $\lambda p: P. f(\mathsf{inl}(p))$ is a proof of $\neg P$, so $f(\mathsf{inr}(\lambda p: P. f(\mathsf{inl}(p))))$ gives $\bot$. Thus we have a proof of $\neg(P \vee \neg P) \to \bot$, i.e., $\neg\neg(P \vee \neg P)$.

We can't construct a proof of $P \vee \neg P$, but we can construct a proof that it's not false. This asymmetry is characteristic of intuitionistic logic.

## The BHK Interpretation and Type Theory

The BHK interpretation is informal — it talks about "proofs" and "functions" without specifying what these are. The Curry-Howard correspondence makes it precise by identifying:
- Proofs with terms in a typed system.
- Propositions with types.
- The BHK clauses with the introduction rules of the corresponding types.

Under this identification:
- A proof of $P \wedge Q$ is a term of the product type $P \times Q$.
- A proof of $P \vee Q$ is a term of the coproduct type $P + Q$.
- A proof of $P \to Q$ is a term of the function type $P \to Q$ (a $\lambda$-abstraction).
- A proof of $\exists x : A, P(x)$ is a term of the dependent pair type $\Sigma_{x:A} P(x)$.
- A proof of $\forall x : A, P(x)$ is a term of the dependent function type $\Pi_{x:A} P(x)$.

The BHK interpretation is not just a philosophical position — it's the foundation of the type-theoretic approach to foundations.

## Beyond BHK: Realizability

The BHK interpretation is deliberately informal. Different constructive systems make it precise in different ways.

*Kleene's realizability* (1945) identifies "functions" with recursive functions (Turing-computable procedures). A proposition $P$ is realizable if there is a Turing machine that, given any input satisfying $P$'s hypotheses, produces a witness for $P$'s conclusion. Under realizability:
- LEM is not realizable (no Turing machine decides all propositions).
- The axiom of choice for natural numbers *is* realizable (it's computationally obvious).
- Markov's principle is realizable (if a program is not provably non-terminating, you can try all natural numbers in sequence).

In HoTT, realizability is replaced by a richer interpretation using $\infty$-groupoids and homotopy theory. The "functions" in BHK become continuous maps, and "proofs" become points in a topological space. This is the motivating picture of HoTT.
