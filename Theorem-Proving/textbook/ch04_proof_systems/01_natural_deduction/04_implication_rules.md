# Implication Rules in Natural Deduction

## The Heart of Logic

The conditional (implication, →) is arguably the most important connective in logic. It is the vehicle for all deduction: "from hypothesis, derive conclusion." Its natural deduction rules are elegant — and deeply connected to the lambda calculus.

**Implication Introduction (→I):**

$$\frac{\Gamma, \varphi \vdash \psi}{\Gamma \vdash \varphi \to \psi} \; (\to I)$$

To prove "if φ then ψ": *assume* φ (temporarily add it to your hypotheses) and derive ψ. Then discharge the assumption — φ is no longer a hypothesis of the final conclusion.

**Implication Elimination (→E, also called Modus Ponens):**

$$\frac{\Gamma \vdash \varphi \to \psi \qquad \Gamma \vdash \varphi}{\Gamma \vdash \psi} \; (\to E)$$

From "if φ then ψ" and "φ," conclude "ψ." This is modus ponens — the oldest named inference rule in logic.

## The Lambda Calculus Connection

The →I rule is exactly **lambda abstraction**:
- "Assume x : φ, derive e : ψ" corresponds to "λx. e : φ → ψ"

The →E rule is exactly **function application**:
- "f : φ → ψ" and "a : φ" gives "f a : ψ"

This is the Curry-Howard correspondence at its most direct. Every →I/→E proof in natural deduction corresponds to a lambda term, and every lambda term corresponds to a natural deduction proof.

## Worked Example: Hypothetical Syllogism

**Claim**: $\varphi \to \psi,\; \psi \to \chi \vdash \varphi \to \chi$ (transitivity of implication)

```
1. φ → ψ          [assumption]
2. ψ → χ          [assumption]
3.   φ            [assume φ for →I]
4.   ψ            [→E on 1, 3]
5.   χ            [→E on 2, 4]
6. φ → χ          [→I, discharging assumption 3]
```

In Lean:
```lean
theorem impl_trans (h1 : P → Q) (h2 : Q → R) : P → R :=
  fun hp => h2 (h1 hp)
-- This is literally function composition: h2 ∘ h1
```

## Discharging Assumptions

The →I rule involves a subtle but important operation: **discharging assumptions**. When we write:

$$\frac{[\varphi], \ldots \vdash \psi}{\varphi \to \psi}$$

The brackets on $[\varphi]$ indicate that this occurrence of $\varphi$ as a hypothesis is discharged — it is consumed by the →I step and no longer appears as a live hypothesis in the conclusion.

This is what makes natural deduction "natural": it mirrors how mathematicians actually reason. When proving "if P then Q," you say "suppose P..." and then "therefore Q," and the proof ends there — the "suppose P" is discharged, having served its temporary purpose.

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)
