# Tarski's World Problems

## Section 1: Evaluating Sentences (★)

**Problem Setup**: Consider a world with four blocks:
- a: small cube at position (1,1)
- b: large tetrahedron at position (3,1)
- c: medium cube at position (2,3)
- d: small tetrahedron at position (4,3)

(Position (col, row); col 1 is leftmost, row 1 is front.)

**1.** Evaluate each sentence as True or False in this world:
  a. `Cube(a)`
  b. `LeftOf(a, b)`
  c. `∀x(Small(x) → Tet(x))`
  d. `∃x(Cube(x) ∧ Large(x))`
  e. `∀x∀y(Cube(x) ∧ Tet(y) → LeftOf(x,y))`  [hint: is a left of d?]

## Section 2: Building Worlds (★★)

**2.** For each pair of sentences, either:
  (a) Build a world where both are true, or
  (b) Prove the two sentences cannot both be true.

  i.  `∀x Cube(x)` and `∃x Tet(x)`
  ii. `∀x(Cube(x) → Large(x))` and `∃x(Cube(x) ∧ Small(x))`
  iii.`∀x∀y(x=y ∨ ¬SameSize(x,y))` and `∃x∃y(x≠y)` (all blocks different sizes + 2+ blocks)

## Section 3: Game of Logic (★★)

**3.** The **Ehrenfeucht-Fraïssé (EF) game** on two structures M and N:
  Spoiler picks an element in M or N; Duplicator picks an element in the other.
  After k rounds, Duplicator wins if the chosen elements satisfy the same atomic sentences.
  Duplicator wins the k-round game iff M and N agree on all FOL sentences of quantifier depth ≤ k.

  Play a 2-round EF game between:
  - M: {a, b} with a LeftOf b, both cubes, a small, b large
  - N: {c, d} with c LeftOf d, both cubes, c medium, d large

  Can Spoiler win? What does this tell us about distinguishing M from N in FOL?

## Section 4: Challenge (★★★)

**4.** Write a sentence that is true in your world from Q1 but would be false in *any* world
with only one block. What quantifier structure is needed?

**5.** Is there a first-order sentence that is true in all finite worlds but false in some
infinite world? (Hint: think about the compactness theorem from ch09.)
