# The BHK Interpretation

## Before the Formalism

Before writing down inference rules, before defining models, before any of the machinery of formal logic — there is an *informal* account of what constructive proofs are. This informal account is the Brouwer-Heyting-Kolmogorov interpretation, named for the three mathematicians who articulated it across the 1920s through the 1940s.

The BHK interpretation is deliberately informal. It does not define a proof system or a model. It gives a *meaning*: an explanation of what it would mean to have a constructive proof of each kind of proposition. Its great virtue is that once you understand it, everything else — the formal rules, the Curry-Howard correspondence, the type-theoretic interpretation — follows naturally.

The central idea is this: a proof is not merely a certificate that a proposition is true. A proof is a *construction* — an explicit mathematical object whose existence establishes the proposition. The proposition and its proof are not separated, as in classical logic, into "the claim" and "the evidence." They are intertwined: the proof *contains* the information that the proposition asserts to hold.

## The BHK Clauses

We specify, for each kind of proposition, what it means to have a constructive proof of it.

**A proof of $P \wedge Q$** is a pair $(p, q)$ where $p$ is a proof of $P$ and $q$ is a proof of $Q$.

Nothing subtle here. To prove a conjunction, prove both conjuncts and give both proofs. The pair $(p, q)$ carries both pieces of evidence.

**A proof of $P \vee Q$** is either a pair $(0, p)$ where $p$ is a proof of $P$, or a pair $(1, q)$ where $q$ is a proof of $Q$.

This is where constructive logic diverges from classical. A classical proof of $P \vee Q$ can be achieved by proving $\neg(\neg P \wedge \neg Q)$ — showing it's impossible for both to fail — without ever specifying which one holds. The BHK interpretation forbids this. A constructive proof of $P \vee Q$ must specify which disjunct holds and provide evidence for it. The "tag" (0 or 1) is part of the proof.

Consider the proposition "either there are infinitely many twin primes, or there are only finitely many." Classically, this is trivially true — it is an instance of LEM. But constructively, to prove it, you must either exhibit an algorithm generating infinitely many twin pairs, or give a proof that beyond some point no twin primes exist. We have neither. The classical truth does not give us a constructive proof.

**A proof of $P \to Q$** is a function $f$ that converts any proof $p$ of $P$ into a proof $f(p)$ of $Q$.

This is the heart of the BHK interpretation. An implication is not a relationship between truth values — it is a *procedure*. To prove "if it's raining, then the ground is wet," you must specify: given any evidence that it's raining, here is how to produce evidence that the ground is wet. The proof is the procedure.

The function $f$ must be *effective*: given an actual proof of $P$, it actually produces a proof of $Q$. This is not just an assertion that a proof of $Q$ could be found if one had a proof of $P$ — it is an actual algorithm for the conversion.

Under the Curry-Howard correspondence, this clause is the key: $P \to Q$ is the type of functions from proofs of $P$ to proofs of $Q$. A proof of $P \to Q$ is a $\lambda$-term of that type.

**$\bot$ has no proof.** There is no construction for absurdity. This is the definition of $\bot$ — the proposition for which no evidence exists.

**A proof of $\neg P$** is a function that converts any proof of $P$ into a proof of $\bot$.

Since $\neg P$ is defined as $P \to \bot$, this follows from the previous clause. But notice what it means: $\neg P$ asserts "any alleged proof of $P$ is self-defeating — it would allow you to derive a contradiction." This is a *refutation procedure*, not simply the claim that $P$ is false.

Classically, $\neg P$ means $P$ is false — a statement about truth values. Constructively, $\neg P$ means we have a method for turning any proof of $P$ into a proof of $\bot$. These coincide when we have enough information (if we know $P$ is false, we can derive a contradiction from any proof of it), but they diverge when we don't: if we have no information about $P$ either way, classically we can still assert $P \vee \neg P$ (one of them must be true), but constructively we cannot prove either.

**A proof of $\forall x : A, P(x)$** is a function $f$ that, given any element $a : A$, produces a proof $f(a)$ of $P(a)$.

This is a *dependent* function: the output type $P(a)$ depends on the input $a$. Under Curry-Howard, $\forall x : A, P(x)$ is the dependent product type $\Pi_{x:A} P(x)$.

A proof of Goldbach's conjecture — "every even number $n > 2$ is the sum of two primes" — would be a function that, given any specific even number $n > 2$, produces two primes summing to $n$. We don't have this function. We cannot simply assert that such a function exists; we must exhibit it.

**A proof of $\exists x : A, P(x)$** is a pair $(a, p)$ where $a : A$ is the *witness* and $p$ is a proof of $P(a)$.

The witness is part of the proof. A constructive existence proof is not merely evidence that something exists somewhere — it contains the actual thing that exists.

This is the most striking divergence from classical practice. Classical mathematics often proves existence by contradiction: assume nothing satisfies $P$, derive a contradiction, conclude something must satisfy $P$. The constructive interpretation forbids this if it produces no witness. The constructive proof must produce the witness explicitly.

## The Disjunction Property and Existence Property

The BHK clauses immediately imply two key properties:

**Disjunction Property (DP):** If there is a constructive proof of $P \vee Q$, then there is a constructive proof of $P$ or a constructive proof of $Q$.

*Proof from BHK:* A proof of $P \vee Q$ is either $(0, p)$ (giving a proof of $P$) or $(1, q)$ (giving a proof of $Q$). Reading off which tag is present tells us which disjunct is proved. $\square$

Classical logic fails DP: $\vdash_\text{cl} P \vee \neg P$ for every $P$, but neither $\vdash_\text{cl} P$ nor $\vdash_\text{cl} \neg P$ in general.

**Existence Property (EP):** If there is a constructive proof of $\exists x : A, P(x)$, then there is a specific $a : A$ with a constructive proof of $P(a)$.

*Proof from BHK:* A proof of $\exists x : A, P(x)$ is a pair $(a, p)$. Reading off the first component gives the witness $a$. $\square$

Classical logic fails EP: one can prove $\exists n : \mathbb{N}, (n = 0 \vee n = 1)$ by an instance of LEM without specifying $n$.

## Why LEM Fails Constructively

The Law of Excluded Middle asserts: $\vdash P \vee \neg P$ for every proposition $P$.

A constructive proof of $P \vee \neg P$ must, by the disjunction property, either prove $P$ or prove $\neg P$. But this is a claim about *every* proposition $P$ — for every mathematical statement, we can decide whether it holds or fails.

This is a universal decision procedure. No such procedure exists.

More concretely: let $P$ be "Turing machine $M$ halts on input $w$." The halting problem is undecidable: no algorithm can decide $P$ for all $M$ and $w$. Any function $f$ that, given any $P$, produces a proof of $P$ or a proof of $\neg P$ would solve the halting problem. No such $f$ exists.

This is not a philosophical objection to LEM. It is a mathematical theorem: LEM implies a universal decision procedure, which does not exist. Therefore LEM is not constructively valid.

## Double Negation: An Asymmetry

Despite rejecting LEM, intuitionistic logic accepts the weaker claim: $P \to \neg\neg P$.

*Proof:* Given a proof $p$ of $P$, we want to prove $\neg\neg P$, which is $(P \to \bot) \to \bot$. Given a function $f : P \to \bot$, apply $f$ to $p$ to get $f(p) : \bot$. So $\lambda f.\, f(p)$ is a proof of $\neg\neg P$. $\square$

But the converse, $\neg\neg P \to P$, is not constructively valid. A proof of $\neg\neg P$ — a function that takes any $f : P \to \bot$ and derives $\bot$ — does not give us a proof of $P$. We know that $\neg P$ is impossible, but we cannot extract a proof of $P$ from this impossibility without some additional information.

This asymmetry — $P \to \neg\neg P$ but not $\neg\neg P \to P$ — is characteristic of intuitionistic logic. It reflects the difference between "it's impossible for $P$ to be disprovable" and "we have an actual proof of $P$."

## Connection to Computability

The BHK interpretation connects, more than coincidentally, to the theory of computation.

Kleene's *realizability* (1945) makes the BHK interpretation precise by identifying "proofs" with recursive functions (Turing-computable programs) and "elements" with natural numbers. A proposition $P$ is *realizable* if there is a Turing machine that computes a witness for it from any input satisfying $P$'s hypotheses.

Under realizability:
- LEM is not realizable: no Turing machine decides all propositions.
- The axiom of choice for $\mathbb{N}$ *is* realizable: given a function that chooses witnesses, compose it with the existential statement.
- *Markov's principle* (if a Turing machine is not provably non-terminating, try all inputs in sequence) is realizable, using the dovetailing trick.

Realizability connects the constructive content of proofs to actual computation. A constructive proof of $\forall n, \exists m, P(n, m)$ is a program that, given $n$, computes $m$ and a witness for $P(n, m)$. The program is extracted from the proof structure, following the BHK clauses.

This connection is the foundation of *program extraction from proofs* — the technique of writing formal proofs in constructive type theory (Coq, Agda, Lean) and extracting verified programs from them. The programs are guaranteed correct because they are the computational content of the proofs, and the proofs have been type-checked.

## BHK and Type Theory

The BHK interpretation is the informal core of the Curry-Howard correspondence. Every BHK clause corresponds to a type constructor:

| BHK clause | Type constructor |
|---|---|
| Proof of $P \wedge Q$ is a pair | $P \times Q$ (product type) |
| Proof of $P \vee Q$ is a tagged sum | $P + Q$ (coproduct type) |
| Proof of $P \to Q$ is a function | $P \to Q$ (function type) |
| $\bot$ has no proof | $\mathbf{0}$ (empty type) |
| Proof of $\forall x, P(x)$ is a dependent function | $\Pi_{x:A} P(x)$ |
| Proof of $\exists x, P(x)$ is a dependent pair | $\Sigma_{x:A} P(x)$ |

The BHK interpretation is not just a philosophical gloss — it is the *content* of the type-theoretic interpretation of logic. Understanding it makes the Curry-Howard correspondence feel inevitable rather than surprising.

In HoTT, the BHK interpretation extends further: a proof of $a = b$ is a path from $a$ to $b$ in the type $A$. The "function" that converts a proof of $a = b$ into a proof of $P(a) \to P(b)$ is the transport operation. The higher structure of identity types — homotopies between paths, paths between homotopies — is the extension of BHK to propositions about propositions.
