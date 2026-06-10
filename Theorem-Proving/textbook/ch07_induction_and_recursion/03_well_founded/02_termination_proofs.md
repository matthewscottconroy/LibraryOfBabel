# Termination Proofs

## Proving Programs Terminate

**Total correctness** of a program requires two things:
1. *Partial correctness*: if the program terminates, the result is correct
2. *Termination*: the program always terminates

Proving termination requires exhibiting a well-founded **measure** — a quantity that:
- Takes values in a well-ordered set (e.g., natural numbers, lexicographic tuples)
- Strictly decreases at each recursive call or loop iteration

## Example: Collatz Sequence

The Collatz conjecture: define $f(n) = n/2$ if $n$ is even, $3n+1$ if $n$ is odd. Starting from any positive integer and repeatedly applying $f$, does the sequence always reach 1?

This is an **open problem** — we do not know a termination proof. Every computed sequence reaches 1, but no one has proved this for all $n$. The Collatz function *might* not terminate for some (yet-undiscovered) $n$.

This illustrates why termination proofs are non-trivial: for complex programs, finding the right measure is the challenge.

## Example: Merge Sort Termination

```python
def merge_sort(l: list) -> list:
    if len(l) <= 1:
        return l
    mid = len(l) // 2
    return merge(merge_sort(l[:mid]), merge_sort(l[mid:]))
```

**Measure**: $|l|$ (the length of the list).

**Decreasing**: In each recursive call, $|l[:mid]| = \lfloor |l|/2 \rfloor < |l|$ and $|l[mid:]| = \lceil |l|/2 \rceil < |l|$ (for $|l| \geq 2$).

**Well-founded**: $|\mathbb{N}|$ with $<$ is well-founded.

Therefore merge sort terminates.

## In Lean 4: Termination Tactic

```lean
-- Lean requires termination evidence for all recursive functions
-- The `termination_by` clause provides the measure

def merge_sort_len : List α → ℕ := List.length

-- Lean verifies the measure decreases at each call
def mergeSort (l : List α) (ord : α → α → Bool) : List α :=
  match l with
  | [] | [_] => l
  | _ =>
    let mid := l.length / 2
    let left  := mergeSort (l.take mid) ord
    let right := mergeSort (l.drop mid) ord
    merge ord left right
termination_by l.length
```

## Exercises
See [problems/ch07_induction/03_well_founded_exercises.md](../../../problems/ch07_induction/03_well_founded_exercises.md)
