# Tautology Identification and SAT

## Section 1: Tautology Check (★)

**1.** Determine by inspection (then verify by truth table or Z3) whether each is a tautology:
  a. `(p → q) ∧ (q → r) → (p → r)`
  b. `p → p ∧ p`
  c. `(p ∨ q) → (p ∧ q)`
  d. `((p → q) → p) → p`  (Peirce's law — this is classically valid but NOT intuitionistically)
  e. `¬¬p → p`  (classical, not intuitionistic)

## Section 2: Z3 SAT Solving (★★)

**2.** Use Z3 (via Python) to:
  a. Find a satisfying assignment for `(p ∨ q ∨ r) ∧ (¬p ∨ q) ∧ (¬q ∨ r) ∧ ¬r`
  b. Verify that `(p → q) → ((q → r) → (p → r))` is a tautology
  c. Find all satisfying assignments for `(p ∧ q) ∨ (¬p ∧ ¬q)`

## Section 3: SAT Encoding (★★★)

**3.** Encode the following as SAT instances and solve with Z3:

  **Graph coloring**: Can the following graph be 3-colored?
  Vertices: {1, 2, 3, 4, 5}
  Edges: {(1,2), (1,3), (2,3), (2,4), (3,5), (4,5)}

  Use variables `color_i_j` (vertex i has color j) and constraints:
  - Each vertex has exactly one color
  - Adjacent vertices have different colors

**4.** Encode and solve the 4×4 Sudoku below using Z3:
```
  . . | 1 .
  . 1 | . .
  ----+----
  . . | . 2
  2 . | . .
```
