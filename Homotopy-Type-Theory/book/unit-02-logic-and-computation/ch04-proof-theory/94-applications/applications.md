# Applications: Proof Theory

## 1. Automated Theorem Proving and Proof Search

The sequent calculus is the backbone of automated theorem provers. The subformula property, which follows from cut elimination, means that proof search for propositional logic is finite: every formula in a cut-free proof is bounded in complexity by the conclusion, so the search space is finite and can be explored systematically.

Modern propositional satisfiability solvers (SAT solvers) use variants of resolution, which is essentially the cut rule applied to clausal normal form. The DPLL algorithm and its successors (CDCL — Conflict-Driven Clause Learning) can be understood as implicit cut-elimination strategies: they search for refutations (proofs of $\bot$ from the negated goal) in a clause-normal-form sequent calculus, using heuristics to guide the order of resolution steps.

For first-order logic, proof search is semi-decidable: if a sequent is valid, a systematic search will eventually find a cut-free proof, but the search may not terminate if the sequent is invalid. The sequent calculus structure guides *focused* proof search strategies (Liang and Miller's focused sequent calculus) that reduce non-determinism by committing to the "polarity" of each formula — whether it should be decomposed eagerly (negative connectives) or lazily (positive connectives). Focused proof search underlies the Twelf system, which uses sequent-calculus-based logical frameworks to specify and check program logics.

## 2. Compiler Correctness via Program Logic

The Curry-Howard correspondence turns proof theory into a tool for verifying programs. Hoare logic — the standard framework for proving program correctness — can be understood as a variant of sequent calculus where the "formulas" are program states and the "proofs" are program executions.

The Iris framework for program verification (used in Coq) is a concurrent separation logic whose proof rules are natural deduction rules for a custom modal logic. The derivability relation $\{P\} \, e \, \{Q\}$ (Hoare triple: if precondition $P$ holds, executing $e$ produces postcondition $Q$) is literally a judgment in a sequent-like system. The cut rule corresponds to composing program components: if $\{P\} \, e_1 \, \{Q\}$ and $\{Q\} \, e_2 \, \{R\}$, then $\{P\} \, e_1; e_2 \, \{R\}$.

Normalization theorems for these program logics correspond to safety properties: if the program logic is strongly normalizing (as a proof system), then the verification process terminates, and there are no circular or redundant verification steps. The Iris team used this connection to verify key components of the Rust standard library.

## 3. Type Checking in Production Programming Languages

Every modern statically typed programming language — Rust, Haskell, OCaml, TypeScript, Swift, Kotlin — implements a judgment of the form $\Gamma \vdash e : T$ (term $e$ has type $T$ in context $\Gamma$). The type checking algorithm is exactly a proof search in the natural deduction system for the type theory underlying the language.

The Hindley-Milner type inference algorithm, used in Haskell and OCaml, is a decision procedure for the judgment $\vdash e : ?$ (infer the type of $e$) in System F restricted to rank-1 polymorphism (Damas-Milner types). The algorithm works by generating and solving unification constraints — which corresponds to backwards proof search in the typing rules, using unification to handle unknown types.

When type checking fails, the error message is essentially a statement about which derivation tree cannot be completed: which rule failed to apply, which type mismatch occurred. Understanding proof theory makes type errors legible — they are not arbitrary failures but principled diagnoses of where the derivation tree breaks down.

## 4. Linear Logic and Resource-Sensitive Type Systems

Girard's linear logic, which restricts the structural rules of contraction and weakening, has direct applications in programming language design wherever resources must be tracked.

Rust's ownership system is the industrial-scale application: the borrow checker enforces that each value is either moved (consumed once, corresponding to linear use) or borrowed (used without consuming, corresponding to the exponential modality $!A$ in linear logic). Memory-safety bugs — use-after-free, double-free, data races — are type errors in this linear type system. The Rust compiler's borrow checker is essentially a proof checker for the linear logic judgment $\Gamma \vdash e : T$ where $\Gamma$ is a *linear* context tracking resource ownership.

Session types for concurrent programming — used in languages like Links, Idris (with session type libraries), and various research languages — formalize communication protocols using linear sequent calculus. The sequent $\Gamma \Rightarrow A$ represents a process that consumes the channels in $\Gamma$ and produces the channel $A$. Cut elimination corresponds to connecting two processes: if one process produces a channel and another consumes it, the composition is well-typed. Deadlock-freedom follows from cut elimination: a well-typed system of processes has no circular dependencies.

## 5. Proof Complexity and the P vs NP Question

The efficiency of proof systems — how short a proof of a given tautology must be — is directly connected to computational complexity theory. This connection is the subject of *proof complexity*, pioneered by Cook and Reckhow.

A *proof system* for a set of tautologies is a polynomial-time relation between formulas and proofs (checking a proof is efficient). A tautology has a *short proof* if it has one of polynomial length in the size of the formula. The central open question: does every tautology have a short proof in some fixed proof system?

If $\mathsf{P} = \mathsf{NP}$, then yes: there is a proof system where every tautology has a polynomial-length proof (because NP contains coNP, and the non-existence of a proof would be in coNP). If $\mathsf{P} \neq \mathsf{NP}$, then no fixed proof system can have polynomial proofs for all tautologies.

The resolution proof system — the basis of SAT solvers — has been extensively studied. There exist families of tautologies (the pigeonhole principles, parity formulas) for which resolution proofs must be exponentially long. This lower bound on resolution proof length is related to the Frege proof system (natural deduction), and separating these systems is an active research frontier. The proof complexity of the sequent calculus and of Frege systems with counting axioms is connected to the polynomial hierarchy and to circuit complexity.

## 6. Certified Cryptography

Formal methods for cryptography use proof-theoretic techniques to certify that cryptographic protocols achieve their security goals. The EasyCrypt tool formalizes game-based security proofs (standard reductions in cryptography) as derivations in a probabilistic relational Hoare logic — a logic whose proof rules are typed natural deduction for a language with probabilistic semantics.

A "proof" that a cryptographic scheme is IND-CPA secure (indistinguishable under chosen plaintext attack) is a derivation in this system that shows the probability of any efficient adversary breaking the scheme is negligible. The normalization theorem for this logic corresponds to a compositionality property: security proofs compose without loss of rigor, and the proof system does not introduce new circular dependencies.

The Coq proof assistant, which is based on the Calculus of Inductive Constructions (a dependent type theory built on the foundation of this chapter), has been used to give machine-verified proofs of cryptographic correctness: the TLS 1.3 protocol implementation in the HACL* library was verified using a combination of F* (a proof assistant in the MLTT tradition) and Coq, giving proofs of memory safety, protocol correctness, and cryptographic security simultaneously.

## 7. Foundations of Mathematics: Ordinal Analysis

Proof theory provides a fine-grained analysis of the "strength" of formal mathematical theories via *ordinal analysis*. Gentzen's original result assigned the ordinal $\varepsilon_0$ to Peano Arithmetic: this is the smallest ordinal such that if you accept transfinite induction up to $\varepsilon_0$, you can prove the consistency of Peano Arithmetic.

Stronger theories — such as second-order arithmetic or subsystems of set theory — have larger *proof-theoretic ordinals*. The Bachmann-Howard ordinal characterizes the strength of the system ATR$_0$ (arithmetic transfinite recursion); the Takeuti-Feferman-Buchholz ordinals characterize stronger systems.

This analysis has practical consequences: it tells us exactly which induction principles are needed to prove which theorems, which theorems cannot be proved without which set-theoretic assumptions, and how to calibrate the strength of foundational theories for specific mathematical domains. The program of *reverse mathematics* (Harvey Friedman, Stephen Simpson) uses proof-theoretic techniques to determine exactly which axioms are equivalent to which theorems over a weak base theory — discovering that most of classical analysis requires only a handful of comprehension axioms, and that these axioms are linearly ordered by proof-theoretic strength.
