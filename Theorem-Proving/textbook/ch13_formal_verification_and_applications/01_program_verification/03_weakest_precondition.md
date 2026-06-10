# Weakest Preconditions

> "The weakest precondition transformer turns program verification into a problem of mathematical proof."
> — Dijkstra, *A Discipline of Programming*, 1976

## From Hoare Logic to Predicate Transformers

Hoare logic gives us rules for deriving Hoare triples $\{P\} C \{Q\}$. But using the rules directly requires *choosing* intermediate conditions — particularly loop invariants — which is a creative, non-mechanical task.

Edsger Dijkstra's **weakest precondition calculus** (1976) provides a systematic, calculational approach: given a postcondition $Q$ and a command $C$, compute the **weakest precondition** $\text{wp}(C, Q)$ — the least restrictive condition on the initial state that guarantees $Q$ after $C$ runs (assuming $C$ terminates).

The key property: $\{P\} C \{Q\}$ (in the partial correctness sense) if and only if $P \implies \text{wp}(C, Q)$.

## Computing wp: The Rules

**Assignment**:
$$\text{wp}(x := E, Q) = Q[E/x]$$

Substitute the right-hand side expression $E$ for occurrences of $x$ in $Q$. The weakest precondition for an assignment is obtained by "working backwards."

**Example**: $\text{wp}(x := x + 1,\; x > 5) = (x + 1 > 5) = (x > 4)$.

"To ensure $x > 5$ after incrementing, we need $x > 4$ beforehand."

**Sequence**:
$$\text{wp}(C_1; C_2, Q) = \text{wp}(C_1, \text{wp}(C_2, Q))$$

Work backwards through the program, right to left.

**Example**: $\text{wp}(x := x + 1;\; y := x \cdot 2,\; y = 10)$.

First: $\text{wp}(y := x \cdot 2,\; y = 10) = (x \cdot 2 = 10) = (x = 5)$.
Then: $\text{wp}(x := x + 1,\; x = 5) = (x + 1 = 5) = (x = 4)$.

So: starting with $x = 4$, after incrementing and doubling, we get $y = 10$.

**Conditional**:
$$\text{wp}(\text{if } B \text{ then } C_1 \text{ else } C_2, Q) = (B \implies \text{wp}(C_1, Q)) \wedge (\neg B \implies \text{wp}(C_2, Q))$$

Equivalently: $(B \wedge \text{wp}(C_1, Q)) \vee (\neg B \wedge \text{wp}(C_2, Q))$.

**While loop**:
$$\text{wp}(\text{while } B \text{ do } C, Q) = I$$

where $I$ is the *loop invariant* satisfying:
1. $I \implies B \implies \text{wp}(C, I)$ (invariant preserved by body)
2. $I \wedge \neg B \implies Q$ (invariant implies postcondition on exit)
3. Some measure $V$ (variant) satisfying $I \wedge B \implies V \geq 0 \wedge \text{wp}(C, V < V_0)$ (termination)

The while rule is the hardest: finding the invariant remains creative. But once the invariant is found, the wp calculus turns verification into mechanical calculation.

## A Complete Example: Max of Two

```
{true}
if x ≥ y then
  max := x
else
  max := y
{max = max(x, y)}
```

Apply the conditional rule to the postcondition $\text{max} = \max(x, y)$:

$\text{wp}(\text{if } x \geq y \ldots, \text{max} = \max(x,y))$
$= (x \geq y \implies \text{wp}(\text{max} := x, \text{max} = \max(x,y)))$
$\wedge (x < y \implies \text{wp}(\text{max} := y, \text{max} = \max(x,y)))$
$= (x \geq y \implies x = \max(x,y))$
$\wedge (x < y \implies y = \max(x,y))$

Both conjuncts are tautologies (by definition of max). So $\text{wp}(\ldots) = \text{true}$.

The program is correct for any initial values.

## The Predicate Transformer View

The function $\text{wp}(C, -)$ is a **predicate transformer**: a function from postconditions (predicates on states) to preconditions (predicates on states). This functional view has elegant mathematical properties:

- $\text{wp}(C, \text{true}) = \text{true}$: if no postcondition is required, any precondition suffices
- $\text{wp}(C, Q_1 \wedge Q_2) = \text{wp}(C, Q_1) \wedge \text{wp}(C, Q_2)$: wp distributes over conjunction
- $\text{wp}(C, Q_1 \vee Q_2) \supseteq \text{wp}(C, Q_1) \vee \text{wp}(C, Q_2)$: wp is monotone

These properties make wp a *monotone function on the lattice of predicates* — allowing the loop invariant to be computed as a *greatest fixed point* in certain settings.

## Connection to Verification Condition Generators

Modern program verification tools use wp (or the closely related **strongest postcondition**) to generate **verification conditions** — logical formulas that, if provable, certify program correctness.

The workflow:
1. Annotate the program with preconditions, postconditions, and loop invariants
2. Run the VC generator: it applies the wp rules to produce a set of formulas
3. Pass the formulas to an SMT solver (Z3, CVC5) or proof assistant
4. If all formulas are proved, the program is verified

**Tools**:
- **Dafny** (Microsoft): A programming language with built-in Hoare specifications; uses Z3 for automatic verification
- **Why3**: A platform for deductive verification; generates VCs for many proof backends
- **Frama-C** with **WP plugin**: Verifies C programs annotated with ACSL (ANSI/ISO C Specification Language)
- **Lean 4 + Mathlib**: Manual verification using the `sorry`-free proof discipline

**Example in Dafny**:
```dafny
method Max(x: int, y: int) returns (m: int)
  ensures m == if x >= y then x else y
{
  if x >= y {
    m := x;
  } else {
    m := y;
  }
}
```

Dafny's VC generator produces the verification conditions we computed above and Z3 proves them automatically.

## Exercises
See [problems/ch13_applications/05_weakest_precondition.md](../../../problems/ch13_applications/05_weakest_precondition.md)
