# SAT Encoding Exercises

## Section 1: Propositional Encodings (★)

**1.** Encode each constraint as a set of clauses (CNF):
  a. "At least one of x₁, x₂, x₃ is true"
  b. "At most one of x₁, x₂, x₃ is true"
  c. "Exactly one of x₁, x₂, x₃ is true"
  d. "If x₁ is true, then x₂ or x₃ must be true"

## Section 2: Planning as SAT (★★)

**2.** Blocks World planning: encode the problem of moving blocks A, B, C from:
```
Initial:  [A on B, B on table, C on table]
Goal:     [B on A, A on table, C on table]
```
as a SAT instance with time steps 0, 1, 2. Variables: `On(X,Y,t)` for each step t.
Constraints: physics (one block per location), legal moves, initial/goal conditions.

## Section 3: Graph Problems (★★)

**3.** Use Z3 (Python) to find a 3-coloring of the Petersen graph, or prove none exists.
The Petersen graph has vertices {0..9} and edges:
  Outer: (0,1),(1,2),(2,3),(3,4),(4,0)
  Inner: (5,7),(7,9),(9,6),(6,8),(8,5)
  Spokes: (0,5),(1,6),(2,7),(3,8),(4,9)

## Section 4: Formal Verification Tools (★★★)

**4.** Install and use one of:
  - Dafny (Microsoft Research): write a verified linked list
  - F* (Microsoft Research): verify a simple cryptographic operation
  - Why3: verify a sorting algorithm
  - SPIN: model-check a mutual exclusion protocol

Write a short report (1 page): what did you verify, what specification language did you
use, what was the hardest part?

## Section 5: Cryptographic SAT Attack (★★★)

**5.** Research and describe (with pseudocode) how SAT solvers can attack stream ciphers:
  a. What is a "SAT attack" on a cipher?
  b. How is the cipher's keystream generator encoded as a SAT instance?
  c. What property of modern ciphers makes this attack impractical?
