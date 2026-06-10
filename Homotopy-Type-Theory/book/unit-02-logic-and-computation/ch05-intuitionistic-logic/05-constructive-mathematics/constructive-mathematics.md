# Constructive Mathematics

## Mathematics With Your Hands

Errett Bishop's 1967 book *Foundations of Constructive Analysis* opens with a polemic: classical mathematics, he argues, has "degenerated into a sterile game played with meaningless symbols." The statement is provocative, and Bishop intended it to be. His point was not that classical mathematics is false but that it is *empty* — it proves the existence of objects without telling you how to find them, and it regards this as a virtue rather than a defect.

Bishop then proceeds to develop — with complete mathematical rigor and in full technical detail — the entire apparatus of real analysis: the real numbers, continuous functions, differential calculus, measure theory, functional analysis. All of it. Constructively. Every existence proof exhibits an explicit construction. Every function is computable. The theorems are the same classical theorems (or very close to them), but the proofs carry computational content that classical proofs discard.

This was not just a philosophical demonstration. Bishop's constructive analysis is genuinely useful. A constructive proof of "there exists a root of this polynomial" contains an algorithm for approximating the root to arbitrary precision. A classical proof might not. The constructive version is mathematically informative in a way the classical version is not.

## What Constructive Mathematics Requires

The demands of constructive mathematics are not arbitrary. They follow from the BHK interpretation:

**Existence must be witnessed.** A proof of $\exists x, P(x)$ must exhibit a specific $x$ and verify $P(x)$. Non-constructive existence arguments — assume no $x$ satisfies $P$, derive a contradiction, conclude some $x$ must satisfy $P$ — are forbidden unless supplemented by an explicit construction.

**Disjunctions must be decided.** A proof of $A \vee B$ must specify which holds. Proofs that establish "it cannot be the case that neither holds" do not suffice.

**Functions must be computable.** When we say "there exists a function $f$ with property $P$," we must exhibit a procedure that computes $f(n)$ for any given $n$. The function is not just an object asserted to exist; it is an algorithm.

**Reasoning must be finitary.** Infinite sets and sequences must be specified by rules or algorithms, not by quantifying over uncountable collections.

## The Real Numbers, Constructively

The constructive treatment of real numbers illustrates the approach.

A **constructive real number** is a sequence of rationals $(q_n)_{n \in \mathbb{N}}$ together with a modulus of convergence: a function $\mu : \mathbb{N} \to \mathbb{N}$ such that for all $k$ and all $m, n \geq \mu(k)$, we have $|q_m - q_n| < 2^{-k}$.

This is essentially the classical definition of a Cauchy sequence, but with the modulus made explicit. The modulus $\mu$ is a *program* that, given any desired precision $k$, tells you how far into the sequence you need to go to be within $2^{-k}$ of the limit. Without the modulus, you merely know the sequence converges; with the modulus, you know *how fast* it converges, which is computationally essential.

Two constructive reals $(q_n, \mu)$ and $(r_n, \nu)$ are *equal* if $|q_n - r_n| \to 0$ with a modulus of convergence for this difference. This is a computational notion of equality, not just a logical one.

**Classically**, the real numbers form a complete ordered field, and the intermediate value theorem holds: if $f : [0,1] \to \mathbb{R}$ is continuous with $f(0) < 0 < f(1)$, there exists $c \in (0,1)$ with $f(c) = 0$.

**Constructively**, the intermediate value theorem requires a stronger hypothesis: the function $f$ must be uniformly continuous (not just continuous), and even then the theorem as stated may fail. The constructive version says: for every $\varepsilon > 0$, there exists $c$ with $|f(c)| < \varepsilon$. This is the *approximate intermediate value theorem*, and it gives an algorithm: binary search, using the sign of $f$ at each step.

The full intermediate value theorem — finding an exact root — is not constructively provable in general. This is not a limitation of constructive analysis; it is a mathematical fact about the complexity of the problem.

## Bishop's Constructivism

Bishop's constructivism is pragmatic rather than philosophical. He does not defend a particular philosophical position about the nature of mathematics. He simply insists that mathematical proofs should carry computational content, and he shows that most of classical analysis can be reconstructed under this constraint.

Key features of Bishop's mathematics:
- **No axioms beyond intuitionistic logic and basic arithmetic.** Bishop works in a system that is essentially intuitionistic mathematics with countable choice.
- **Countable choice is accepted.** The axiom of choice for $\mathbb{N}$-indexed families is assumed: if for each $n$ there exists $a_n$ with property $P(n, a_n)$, then there is a sequence $(a_n)$ such that $P(n, a_n)$ for all $n$. This is computationally obvious: define the sequence by the algorithm.
- **Dependent choice is often used.** A stronger form: if for each $a$ with $P(a)$ there exists $b$ with $Q(a, b)$, then there is a sequence $a_0, a_1, \ldots$ with $P(a_0)$, $Q(a_i, a_{i+1})$, etc.
- **LEM and full AC are not assumed.** These are the non-constructive principles that Bishop avoids.

The result is a mathematics that is simultaneously constructive (every proof carries computational content) and mainstream (the theorems are recognizable as the classical theorems, often with minor modifications).

## Markov's Principle

Markov's principle (MP) is a principle intermediate between pure intuitionism and classical logic:

**MP:** If it is not the case that a Turing machine $M$ never terminates (i.e., $\neg\neg \exists n, M \text{ halts at step } n$), then $M$ does halt (i.e., $\exists n, M \text{ halts at step } n$).

More abstractly: for a decidable property $P$ of natural numbers, if $\neg\neg \exists n, P(n)$ then $\exists n, P(n)$.

MP says: for decidable predicates, double negation elimination holds. If we know it's not the case that no $n$ satisfies $P$, we can actually find an $n$ satisfying $P$ — by running the search.

MP is weaker than full DNE ($\neg\neg A \to A$ for all $A$) but stronger than pure intuitionism (which cannot prove it). It is accepted by the Russian school of constructivism (Markov, Shanin) because it is computationally justified: if $M$ doesn't definitely fail to halt, try running it. But Bishop's school and the Swedish school (Martin-Löf) do not accept MP — it uses an infinite search that, while always terminating (if MP's hypothesis holds), gives no advance bound on how long the search takes.

In formal terms, MP is *realizable* (it has a computational interpretation) but not *provable* in IPC or pure Bishop constructivism. It is an example of a principle that is computationally true but not constructively valid in the proof-theoretic sense.

In HoTT, Markov's principle is related to *omniscience* principles for the natural numbers. The LLPO (Lesser Limited Principle of Omniscience) and LPO (Limited Principle of Omniscience) are weaker versions of LEM that are consistent with some models of HoTT (constructive models) but not with others (classical models).

## MLTT as a Foundation for Constructive Mathematics

Martin-Löf Type Theory (MLTT) provides a rigorous foundation for constructive mathematics. Its key features:

**Types are mathematical constructions.** The natural numbers $\mathbb{N}$ are defined by their constructors (zero and successor) and their elimination principle (recursion). Real numbers can be defined as types of Cauchy sequences with moduli.

**Proofs are computations.** Every proof in MLTT is a program. The proof of "every even number is the sum of two numbers" is a function that, given an even number $n$, computes the two summands. The computation is part of the proof.

**Dependent types capture quantification.** $\forall x : A, P(x)$ is the type $\Pi_{x:A} P(x)$, and $\exists x : A, P(x)$ is the type $\Sigma_{x:A} P(x)$. These types encode both the logical content and the computational content of the quantifiers.

**Identity types capture equality.** Proofs of $a = b$ are elements of the identity type $a =_A b$. In ordinary constructive mathematics, this is a proposition (at most one proof). In HoTT, the identity type can have multiple distinct elements — paths — and this is the source of the higher-dimensional structure.

MLTT is more expressive than Bishop's constructivism and provides a language in which constructive mathematics can be fully formalized and computer-checked. The major constructive proof assistants — Agda, Coq (without classical axioms), Lean 4 (without classical axioms) — are all implementations of MLTT or closely related type theories.

## The Constructive Content of Proofs

The most practically significant fact about constructive mathematics: **constructive proofs can be automatically converted into programs**.

This is *program extraction*. Given a constructive proof of $\forall x : A, \exists y : B, P(x, y)$ — "for every input of type $A$, there exists an output of type $B$ satisfying $P$" — we can extract a program $f : A \to B$ such that $P(x, f(x))$ holds for all $x$. The program is the computational content of the proof.

In Coq, this is done with the `Extraction` command: write a constructive proof in Coq's type theory, and Coq can extract a verified OCaml or Haskell program implementing the computed function. The extracted program is correct by construction — it is derived from the proof, not written independently and then verified.

This approach has been used to extract:
- Sorting algorithms from proofs of the proposition "every list has a sorted permutation."
- Number-theoretic algorithms from proofs of theorems in combinatorics.
- Parsing algorithms from proofs of grammar properties.
- Cryptographic protocols from game-based security proofs.

The connection to HoTT: in HoTT, the extraction of computational content is enriched by the homotopy structure. A proof of equivalence $A \simeq B$ in HoTT includes the functions back and forth, their homotopies, and the proof that these are actual inverses. This is not just logical content — it is computational content, and it can be extracted as a verified program that witnesses the equivalence.
