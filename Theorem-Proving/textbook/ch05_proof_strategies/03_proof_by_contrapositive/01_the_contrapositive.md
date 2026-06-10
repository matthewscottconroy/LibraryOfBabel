# Proof by Contrapositive

## Overview
The contrapositive of P → Q is ¬Q → ¬P. They are logically equivalent.
Proving ¬Q → ¬P is often easier when P is complex or when Q gives you
a powerful negative assumption to work with.

## Learning Objectives
- State the equivalence P→Q ≡ ¬Q→¬P
- Recognize when contrapositive is the natural strategy
- Distinguish contrapositive from contradiction

## Method
To prove P → Q:
1. Assume ¬Q
2. Derive ¬P
3. Conclude P → Q by the equivalence with ¬Q → ¬P

## Example: If n² is even, then n is even
**Direct approach**: Hard — we would need to factor n² and trace back to n.
**Contrapositive**: Assume n is odd. Then n = 2k+1 for some k. So n² = 4k²+4k+1 = 2(2k²+2k)+1 is odd. □

## Contrapositive vs. Contradiction
Both are classical. But they are different:
- **Contrapositive**: proves P→Q by assuming ¬Q and deriving ¬P (direct proof of the contrapositive)
- **Contradiction**: proves P by assuming ¬P and deriving ⊥

For conditionals, prefer contrapositive over contradiction when it applies —
it is more direct and gives a cleaner proof structure.

## Lean 4
```lean
-- To prove P → Q by contrapositive:
theorem n_sq_even_implies_n_even (n : Int) (h : isEven (n * n)) : isEven n := by
  contrapose! h   -- now goal becomes: ¬isEven n → ¬isEven (n * n)
  sorry
```

## Exercises
See `problems/ch05_proof_strategies/03_contradiction_and_contrapositive.md`
