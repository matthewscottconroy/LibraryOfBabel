# 5.1 Schools of Constructive Mathematics

## Beyond Pure Intuitionistic Logic

Intuitionistic logic (IPC) is a minimum — the common ground of all constructive mathematics. But different constructive mathematicians add different extra principles, leading to different schools with different strengths and different relationships to classical mathematics.

The three main schools:
1. **Brouwer's Intuitionism**: radical, rejects classical mathematics, adds continuity principles
2. **Bishop's Constructivism**: conservative, compatible with classical mathematics, uses only explicit constructions
3. **Markov's Recursive Constructivism** (Russian school): adds computability principles, accepts Markov's principle

Each school represents a different balance between constructive purity and mathematical power.

## Brouwer's Intuitionism

L.E.J. Brouwer (1881–1966) founded intuitionism. His philosophy was radical: mathematics is a "languageless activity of the mind," and mathematical objects exist only as mental constructions. Language, including formal logic, is a secondary description of these mental constructions. Formal systems can never fully capture mathematical truth.

Brouwer accepted *choice sequences*: infinite sequences of natural numbers that are being "freely generated" — not determined in advance but chosen step by step. This led to two notable principles:

**The Fan Theorem (Brouwer):** Any decidable spread (a certain kind of finitely branching tree) has a finite "bar." This is equivalent (over IPC) to the statement that $[0,1]$ is compact in Brouwer's mathematics.

**The Continuity Principle (Brouwer):** Every total function from the Baire space $\mathbb{N}^{\mathbb{N}}$ to $\mathbb{N}$ is *continuous* (in the standard topology on $\mathbb{N}^{\mathbb{N}}$).

This principle has a dramatic consequence: in Brouwer's intuitionism, every real-valued function defined on $[0,1]$ is uniformly continuous. This contradicts classical mathematics, where discontinuous functions exist.

Brouwer's mathematics is *inconsistent with classical mathematics*. You can't add his continuity principles to ZFC without getting a contradiction.

This is not a problem for Brouwer: he rejected ZFC's formalism entirely. But it means Brouwer's mathematics is incompatible with the mainstream. For our purposes (building HoTT as a foundation compatible with classical practice), Brouwer's approach is too radical.

## Bishop's Constructivism

Errett Bishop's 1967 book *Foundations of Constructive Analysis* launched a new approach: constructive mathematics that is *compatible* with both classical and intuitionistic mathematics.

Bishop's strategy: develop mathematics using only explicit constructions and proofs, without adding any axioms beyond standard constructive logic and simple arithmetic. Avoid LEM and the axiom of choice (except the obvious "choice" that's built into dependent type theory). Avoid Brouwer's continuity principles.

Bishop's results are:
- Every theorem of Bishop's mathematics is also a theorem of classical mathematics. (Bishop's mathematics is a *subset* of classical mathematics.)
- Every theorem of Bishop's mathematics is also a theorem of Heyting Arithmetic plus standard type-theoretic axioms. (Bishop's mathematics is *constructive*.)
- The proofs contain algorithmic content. A Bishop proof of $\exists n, P(n)$ gives an algorithm for finding $n$.

Examples of Bishop's constructive theorems:
- The Intermediate Value Theorem fails constructively (there's no general algorithm for finding roots). Bishop proves a *constructive version*: any continuous function with opposite signs at the endpoints has an approximate root within any $\varepsilon$.
- The Bolzano-Weierstrass theorem (every bounded sequence of reals has a convergent subsequence) fails constructively. Bishop proves the constructive version: given an explicit bound and an explicit modulus of compactness, one can extract a convergent subsequence.

Bishop's approach is the gold standard for constructive analysis and forms the basis for *formal constructive mathematics* in proof assistants.

## Markov's Principle and Recursive Constructivism

The Russian school of constructive mathematics, associated with A.A. Markov Jr. (1903–1979), formalizes the BHK interpretation using *recursive functions* (Turing machines). In this school:
- A proof of $\exists n, P(n)$ is a Turing machine that computes such an $n$.
- A proof of $\forall n, \exists m, P(n,m)$ is a Turing machine that, given $n$, computes such an $m$.

The Russian school adds **Markov's Principle (MP)**:

$$\text{If } \forall n, P(n) \vee \neg P(n) \text{ (decidable)} \text{ and } \neg\neg\exists n, P(n), \text{ then } \exists n, P(n).$$

Informally: if $P$ is decidable and it's not the case that no $n$ satisfies $P$, then some $n$ satisfies $P$.

Why is this reasonable? Suppose $P$ is a decidable property (you can check $P(0), P(1), P(2), \ldots$ one by one). If you assume $\neg\neg\exists n, P(n)$ — i.e., it's impossible that all fail — then you can *search* through the naturals: try $P(0)$, then $P(1)$, etc. You know the search must eventually succeed (if it didn't, that would mean all fail, contradicting $\neg\neg\exists n, P(n)$). So you eventually find an $n$ with $P(n)$.

Markov's principle is a form of the classical argument "if not all naturals fail, then some succeed," made constructively acceptable by adding the computability assumption (the search terminates in finite time).

**Properties of MP:**
- MP is *not* provable in pure IPC or in Bishop's constructivism.
- MP is *consistent* with IPC (there are models where it holds).
- MP is *not* consistent with Brouwer's continuity principle.
- MP holds in Kleene's realizability interpretation (where functions are Turing machines).

For HoTT: MP is neither assumed nor refuted. In the "effective" models of HoTT (where types are interpreted as sets of programs), MP holds. In the "spatial" models (where types are topological spaces), MP may fail.

## Constructive Mathematics in HoTT

HoTT is built on **Martin-Löf Type Theory (MLTT)**, which is a constructive system:
- LEM is not provable in MLTT.
- The axiom of choice (in the "propositional" form) is not provable.
- Markov's principle is not provable.

But these principles can be *added* consistently:
- Adding LEM to MLTT gives classical type theory.
- Adding function extensionality plus LEM plus the axiom of choice gives a system roughly equivalent to ZFC.
- The Univalence Axiom can be combined with LEM (though some consequences change).

**Lean 4 and Mathlib** import classical axioms by default: `Classical.propDecidable` gives a decision procedure for all propositions, equivalent to LEM. This makes Lean suitable for mainstream mathematics but means proofs don't automatically have computational content.

**Agda** can be used in a "classical mode" (with `--safe` flag off) or in a purely constructive mode. HoTT-Agda libraries work constructively, without classical axioms.

**Coq** distinguishes between `Prop` (proof-irrelevant propositions, classical if LEM is imported) and `Type` (computational content preserved). Classical proofs in Coq are proofs in `Prop` and don't have extractable programs.

The key takeaway for the working type theorist: understanding which axioms are needed for which theorems, and what their computational cost is, is essential for formal mathematics.

## The Spectrum: From Constructive to Classical

Here's a helpful way to think about the landscape:

**Most constructive** (strongest computational content):
- Pure IPC + basic arithmetic
- Bishop's constructivism
- MLTT without extra axioms

**Intermediate** (some classical principles):
- MLTT + function extensionality
- MLTT + propositional univalence
- MLTT + Markov's principle

**Mostly classical** (classical principles, fewer constructive guarantees):
- MLTT + LEM
- HoTT + LEM

**Fully classical** (classical mathematics):
- ZFC
- Lean 4 + Mathlib (with classical axioms)

The beauty of HoTT is that it sits near the middle: it's constructive by default but flexible enough to accommodate classical reasoning when needed, without losing the rich homotopical structure.

## Practical Implications

When working in a type-theoretic proof assistant:

1. **Constructive proofs are better** when you care about computation. A constructive proof of "there exists a program with property $P$" gives you the program. A classical proof doesn't.

2. **Classical proofs are acceptable** when you're just doing mathematics without caring about algorithmic content. Most of Mathlib in Lean 4 is classical.

3. **The Axiom of Choice** in its propositional form (choosing from non-empty types where you have a proof they're non-empty) is not automatically available constructively. In many practical situations, you have the explicit choice function anyway (because you constructed the type by exhibiting elements). The "hard" AC is for arbitrary infinite collections.

4. **Function extensionality** (two functions that agree on all inputs are equal) is not provable in pure MLTT. It's an axiom in HoTT (and follows from Univalence). Lean 4 has function extensionality by default.

5. **Propositional extensionality** (two propositions that are logically equivalent are equal) is not provable in pure MLTT. It follows from LEM or from Univalence (for propositions at the appropriate h-level).

Understanding these distinctions is one of the practical payoffs of studying intuitionistic logic carefully.
