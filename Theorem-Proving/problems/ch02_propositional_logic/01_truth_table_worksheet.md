# Propositional Logic: Truth Table Worksheet

## Section 1: Building Truth Tables (★)

**1.** Construct truth tables for each formula:
  a. `¬(p ∨ q)`
  b. `¬p ∧ ¬q`
  c. `p → (q → p)`
  d. `(p → q) ∧ (q → p)`
  e. `p ↔ (p ∨ p)`

**2.** For formulas (a) and (b) above: are they logically equivalent? Prove it using your
truth tables.

## Section 2: Classification (★)

**3.** For each formula, determine whether it is a tautology, contradiction, or contingency:
  a. `p ∨ ¬p`
  b. `p ∧ ¬p`
  c. `(p → q) → (¬q → ¬p)`
  d. `(p → q) ∧ (¬p → q) → q`
  e. `(p ∧ q) → p`
  f. `p → (p ∧ q)`

## Section 3: Entailment (★★)

**4.** Use truth tables to determine if each entailment holds:
  a. `{p → q, ¬q}  ⊨  ¬p`
  b. `{p ∨ q, ¬p}  ⊨  q`
  c. `{p → q, q → r}  ⊨  p → r`
  d. `{p}  ⊨  p ∧ p`
  e. `{p ∨ q}  ⊨  p`  (valid or not?)

## Section 4: Python Challenge (★★)

**5.** Modify `textbook/ch02_propositional_logic/02_truth_tables_and_semantics/03_truth_tables_in_python.py`
to:
  a. Check whether any two given formulas are logically equivalent
  b. Find all satisfying valuations of a given formula
  c. Count the number of satisfying valuations

**6.** Use your script to verify De Morgan's laws computationally.

## Section 5: CNF Conversion (★★)

**7.** Convert each formula to CNF (show all steps):
  a. `¬(p → q)`
  b. `p ↔ q`
  c. `(p ∧ q) ∨ (r ∧ s)`

**8.** What is the relationship between the number of clauses in a CNF formula and the
number of rows in its truth table that evaluate to False?
