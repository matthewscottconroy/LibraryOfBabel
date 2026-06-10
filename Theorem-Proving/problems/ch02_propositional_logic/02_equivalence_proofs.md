# Propositional Equivalence Proofs

## Section 1: Prove by Truth Table (★)

**1.** Prove each equivalence by truth table:
  a. `¬(p ∨ q) ≡ ¬p ∧ ¬q`  (De Morgan 2)
  b. `p → q ≡ ¬p ∨ q`
  c. `p ↔ q ≡ (p → q) ∧ (q → p)`

## Section 2: Algebraic Proof (★★)

**2.** Prove each equivalence using only the named equivalences from ch02 section 3
(show each step and name the rule used):
  a. `p → (q → r) ≡ (p ∧ q) → r`  (exportation/importation)
  b. `¬(p → q) ≡ p ∧ ¬q`
  c. `(p → q) → p ≡ p`  (Peirce's law; harder, may need cases)

## Section 3: Natural Deduction Proofs (★★)

**3.** Prove each in Lean 4 or Coq (or via Fitch/Carnap):
  a. `p ∧ q → q ∧ p`
  b. `(p → r) ∧ (q → r) → (p ∨ q → r)`
  c. `p → ¬¬p`  (double negation introduction; should work intuitionistically)
  d. `¬¬p → p`  (double negation elimination; requires classical logic — why?)

## Section 4: Resolution Proofs (★★)

**4.** Use the resolution method to prove each:
  a. From `{¬p ∨ q, ¬q ∨ r, p}` prove `r`
  b. From `{p ∨ q, ¬p ∨ r, ¬q ∨ r}` prove `r`

## Section 5: Challenge (★★★)

**5.** Prove that the following five axioms (plus modus ponens) are sufficient for
propositional logic (Hilbert system):
  - K: `p → (q → p)`
  - S: `(p → (q → r)) → ((p → q) → (p → r))`
  - B: `(q → r) → ((p → q) → (p → r))`
  - C: `(p → (q → r)) → (q → (p → r))`
  - I: `p → p`

Show that you can derive modus tollens from K and S plus modus ponens.
