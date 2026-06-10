# Induction as a Proof Strategy

## When to Use Induction

Induction is the right strategy when you are proving a property $P(n)$ that:
1. Is parameterized by a natural number $n$ (or a recursively defined structure)
2. Has an obvious base case
3. Has a step from smaller to larger that can be captured formally

More broadly: whenever your proof has "and this continues for all $n$..." or "by the same argument for any length list...", induction is probably the right tool.

## The Strategy in Practice

**Step 1**: Identify the induction variable and what $P(n)$ says.

**Step 2**: Check: is the base case obvious? If not, you might have the wrong induction statement.

**Step 3**: Write the inductive step:
- **Assume** $P(k)$ (the IH)
- **Prove** $P(k+1)$
- **Identify** where the IH is used — if you never use it, something is wrong

**Step 4**: Check your proof handles all edge cases. Are there cases where the IH does not apply?

## Common Pitfalls

**The IH is not strong enough**: Sometimes $P(n)$ is true but proving $P(n+1)$ from $P(n)$ alone is impossible. The fix: strengthen $P(n)$ to $Q(n)$ (a stronger statement that implies $P(n)$ but is also induction-friendly).

**Off-by-one in the base case**: If your inductive step only works for $n \geq 2$ (needing $P(n-1)$ and $P(n-2)$), you need two base cases ($n = 0$ and $n = 1$).

**Wrong induction variable**: Sometimes induction on $n$ doesn't work but induction on $2n$ or some derived quantity does.

## Lean 4 Tactic

```lean
theorem example_induction (n : ℕ) : 0 + n = n := by
  induction n with
  | zero     => rfl
  | succ k h => simp [Nat.add_succ, h]
```

The `induction n with` tactic splits the goal into base case (`zero`) and step (`succ k h`), where `h : 0 + k = k` is the induction hypothesis.

## Exercises
See [problems/ch07_induction/](../../../problems/ch07_induction/)
