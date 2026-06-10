# Temporal Logic and Model Checking

## Automatic Verification of Systems

**Model checking** (Clarke, Emerson, Sifakis — Turing Award 2007) is the technique of:
1. Modeling a system as a finite-state Kripke structure
2. Specifying a property in CTL or LTL
3. **Automatically** verifying whether the model satisfies the property — or producing a counterexample

Unlike theorem proving (which requires creative proof search), model checking is fully automatic. The tradeoff: it requires the state space to be finite (or abstractable to finite).

## The Approach

Given model $\mathcal{M} = (S, S_0, R, L)$ and CTL formula $\varphi$:

**Recursive labeling**: For each subformula $\psi$ of $\varphi$, compute $\text{Sat}(\psi) = \{s \in S \mid \mathcal{M}, s \models \psi\}$.

Base cases: atoms use the labeling $L$.
Recursive cases: use set operations and graph reachability.

The model satisfies $\varphi$ iff $S_0 \subseteq \text{Sat}(\varphi)$.

## The State Explosion Problem and Fixes

**Problem**: With $n$ Boolean state variables, the state space is $2^n$ — exponential in the number of variables. A circuit with 100 state variables has $2^{100}$ states.

**Symbolic model checking** (McMillan 1993): represent state sets as BDDs (Binary Decision Diagrams) — a compact, canonical representation. Many large sets can be represented by small BDDs.

**Bounded Model Checking**: Check if the property can be violated within $k$ steps. Encode as SAT formula, solve with DPLL. Finds bugs efficiently; cannot prove correctness (only up to bound).

**Counterexample-Guided Abstraction Refinement (CEGAR)**: Build a coarse abstract model, check it. If it fails with a spurious counterexample, refine the abstraction. Repeat until either property is proved or a real counterexample is found.

## Industrial Applications

- **Intel**: Model checking for floating-point units since the Pentium FDIV disaster
- **NASA**: Verification of spacecraft software
- **AWS, Microsoft Azure**: Distributed protocol verification with TLA+
- **Hardware synthesis tools**: Verify generated hardware against specifications before fabrication

Model checking has transitioned from academic technique to industrial standard over 30 years — one of the greatest success stories of formal methods.

## Exercises
See [problems/ch12_modal_logic/02_temporal_exercises.md](../../../problems/ch12_modal_logic/02_temporal_exercises.md)
