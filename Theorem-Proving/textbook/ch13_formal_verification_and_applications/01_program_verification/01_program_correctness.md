# Program Correctness

## What Does It Mean for a Program to Be Correct?

A program is **correct** if it computes the right output for every valid input. But "right output" requires a specification — a precise statement of what the program *should* do.

**Informal specification**: "Sort the input list." Clear in intent, but ambiguous in edge cases (empty lists? duplicate elements? stability?).

**Formal specification**: A mathematical predicate relating inputs and outputs:
$$\text{sorted}(l') \wedge \text{permutation}(l, l')$$

Given input $l$, the output $l'$ must be sorted and contain exactly the same elements as $l$.

Formal specifications are the foundation of program verification — without them, "correctness" has no precise meaning.

## Varieties of Correctness

**Partial correctness**: If the program terminates with input satisfying the precondition, the output satisfies the postcondition. Does not guarantee termination.

**Total correctness**: The program terminates AND the output satisfies the postcondition for every valid input.

**Safety properties**: Something bad never happens ("the array index is always in bounds").

**Liveness properties**: Something good eventually happens ("the lock is eventually released").

## Approaches to Verification

| Approach | Method | Guarantees |
|----------|--------|-----------|
| **Testing** | Run on sample inputs | Bugs found, not absence of bugs |
| **Hoare logic** | Manual proof of triples $\{P\}C\{Q\}$ | Full correctness (with loop invariants) |
| **Model checking** | Exhaustive state-space search | Full correctness for finite systems |
| **Type checking** | Static type inference | Type safety (limited class of errors) |
| **SMT solving** | Automatic Z3/CVC5 proof search | Full correctness for bounded programs |

## Historical Context

Edsger Dijkstra (1970s) argued that programs should be developed together with their correctness proofs — not tested after the fact. His structured programming movement emphasized designing programs whose correctness is evident by construction.

Tony Hoare's axiomatic semantics (1969) provided the formal tool: Hoare logic. The vision of provably correct software has taken 50 years to become practical — modern proof assistants like Lean, Coq, and Dafny are bringing it closer to reality.

## Exercises
See [problems/ch13_applications/01_hoare_logic_problems.md](../../../problems/ch13_applications/01_hoare_logic_problems.md)
