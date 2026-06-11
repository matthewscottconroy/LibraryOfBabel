# Chapter 11 Overview: Type Theory

---

## Central Question

What if propositions were the same as types, and proofs were the same as programs? This identification — the Curry-Howard correspondence — transforms proof theory into programming language theory, and makes formal proof a form of software development.

---

## Why This Chapter Matters

Type theory is the foundation of modern proof assistants. Lean 4 is based on the Calculus of Inductive Constructions; Coq is based on the Calculus of Constructions. Agda is based on Martin-Löf dependent type theory. All are realisations of the Curry-Howard correspondence: a proof of proposition $P$ is a term of type $P$, and type-checking is proof verification. Understanding the theoretical foundations explains why proof assistants work and what their limits are.

---

## Key Definitions

**Type.** In a type system, every term $t$ has a type $A$, written $t : A$. The type constrains the term's behaviour. Types can themselves be terms.

**Simply typed lambda calculus ($\lambda_\to$).** Terms built from variables, abstraction $\lambda x:A. t$ (a function), and application $t_1\ t_2$. Types are built from base types $\iota, o, \ldots$ and the arrow type $A \to B$ (functions from $A$ to $B$). Typing rules:

- Variable: $\Gamma, x:A \vdash x : A$
- Abstraction: $\frac{\Gamma, x:A \vdash t : B}{\Gamma \vdash \lambda x:A. t : A \to B}$
- Application: $\frac{\Gamma \vdash t_1 : A \to B \quad \Gamma \vdash t_2 : A}{\Gamma \vdash t_1\ t_2 : B}$

**System F (polymorphic lambda calculus, Girard-Reynolds 1972).** Extends $\lambda_\to$ with type variables and universal quantification over types: $\forall \alpha. A$ is a type (for all types $\alpha$, here is a term of type $A$). Terms can abstract over types: $\Lambda\alpha. t$ (a term universally quantified over type $\alpha$), with application $t\ [A]$ (instantiating the type variable with $A$).

System F is far more expressive than $\lambda_\to$: it can represent all primitive recursive functions, and its typability problem is undecidable (Urzyczyn 1997).

**Dependent types.** In dependent type theory, types can depend on terms. The *dependent product type* $\Pi x:A. B(x)$ is the type of functions $f$ such that $f(a) : B(a)$ for all $a : A$. The *dependent sum type* $\Sigma x:A. B(x)$ is the type of pairs $(a, b)$ where $a : A$ and $b : B(a)$.

**Propositions as types.** The Curry-Howard correspondence:

| Logic | Type Theory |
|-------|-------------|
| Proposition $P$ | Type $P$ |
| Proof of $P$ | Term $t : P$ |
| Implication $P \to Q$ | Function type $P \to Q$ |
| Conjunction $P \land Q$ | Product type $P \times Q$ |
| Disjunction $P \lor Q$ | Sum type $P + Q$ |
| Negation $\neg P$ | Function type $P \to \bot$ |
| Universal $\forall x:A. P(x)$ | Dependent product $\Pi x:A. P(x)$ |
| Existential $\exists x:A. P(x)$ | Dependent sum $\Sigma x:A. P(x)$ |

**Dependent type theory (Martin-Löf 1975, 1984).** A type theory with:
- $\Pi$-types (dependent products / functions)
- $\Sigma$-types (dependent sums / pairs)
- Inductive types (including $\mathbb{N}$, lists, trees)
- A universe hierarchy $\text{Type}_0 : \text{Type}_1 : \text{Type}_2 : \cdots$

---

## The Curry-Howard Correspondence

### Simply Typed Lambda Calculus and Intuitionistic Propositional Logic

The typing rules of $\lambda_\to$ are in bijective correspondence with the natural deduction rules for intuitionistic propositional logic (without $\lor$ or $\exists$):

- Typing $\Gamma \vdash t : A$ corresponds to a proof of $A$ from hypotheses $\Gamma$.
- Abstraction ($\lambda$) corresponds to implication introduction ($\to I$): assume $A$, derive $B$, conclude $A \to B$.
- Application corresponds to modus ponens ($\to E$): from $A \to B$ and $A$, derive $B$.

**Proof terms as witnesses.** A proof of $A \land B$ in natural deduction is a pair of proofs of $A$ and $B$; the corresponding type-theoretic term is a pair $(t_1, t_2)$ where $t_1 : A$ and $t_2 : B$.

### System F and Second-Order Logic

System F types correspond to second-order intuitionistic propositional logic. The type $\forall \alpha. \alpha \to \alpha$ corresponds to the second-order proposition $\forall P. P \to P$ (the identity). This is a Church encoding of the universal type.

### Dependent Types and FOL

Full dependent type theory encodes all of intuitionistic first-order logic: $\Pi x:A. P(x)$ is $\forall x:A. P(x)$; $\Sigma x:A. P(x)$ is $\exists x:A. P(x)$.

**Proof relevance.** In dependent type theory, a proof of $P$ is a term of type $P$; two different proofs may be distinct terms. This matters for equality: $p : P$ and $q : P$ may or may not be equal as terms, even if both prove $P$. The question "are all proofs of a proposition equal?" is the *proof irrelevance* question, and its treatment differs between intensional and extensional type theories.

---

## Normalisation and Decidability

**Strong normalisation theorem.** In simply typed lambda calculus, every typed term reduces to a unique normal form (no infinite reduction sequences). This means type-checking always terminates, making $\lambda_\to$ a sound but limited computational system.

**System F normalisation.** System F is also strongly normalising, but its expressibility is greater than $\lambda_\to$. The normalisation proof is more complex (Girard's method of reducibility candidates).

**Dependent type theory decidability.** Type-checking in dependent type theory (like CIC, the foundation of Lean 4 and Coq) is decidable, because the definitional equality used in type-checking has a decision procedure (normalisation). This is what makes mechanical proof-checking possible.

---

## Historical Context

**Haskell Curry (1934)** observed the correspondence between types in combinatory logic and propositional formulas.

**William Howard (1969, circulated; published 1980)** identified the full correspondence between natural deduction proofs and lambda terms, making the "propositions as types" slogan precise.

**Jean-Yves Girard (1972)** independently discovered System F (polymorphic lambda calculus) in his proof theory work, while John Reynolds (1974) rediscovered it in the context of programming languages. System F is the type-theoretic account of second-order logic.

**Per Martin-Löf (1975, 1984)** developed intuitionistic type theory, the first type theory rich enough to serve as a foundation for constructive mathematics. His theory introduced dependent types, inductive types, and the universe hierarchy.

**Thierry Coquand and Gérard Huet (1988)** developed the Calculus of Constructions (CoC) and its extension to inductive types (CIC), which is the foundation of Coq.

**Leonardo de Moura et al. (2021)** released Lean 4, based on a dependent type theory similar to CIC, with features for efficient compilation and metaprogramming.

---

## Connections to Other Chapters

- **Chapter 4** presented intuitionistic natural deduction; Chapter 11 reveals this is exactly $\lambda_\to$ in disguise.
- **Chapter 5** presented proof strategies; in Lean 4/Coq, these become *tactics* that manipulate proof terms.
- **Chapter 7** (Induction and Recursion): inductive types in type theory are the type-theoretic realisation of the recursion theorem.
- **Chapter 13** (Formal Verification): Lean 4 and Coq are implementations of dependent type theory; understanding the theory explains the tools.
