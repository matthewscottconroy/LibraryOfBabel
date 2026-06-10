# Model Theory: Structure Construction

## Section 1: Building Models (★)

**1.** For each first-order sentence, either build a model (structure) satisfying it,
or explain why no model exists:
  a. `∀x∀y(x = y)`  (one-element domain)
  b. `∃x∃y(x ≠ y) ∧ ∀z(z = x ∨ z = y)`  (exactly two elements)
  c. `∀x∀y(P(x,y) → P(y,x)) ∧ ∃x¬P(x,x)` (symmetric but not reflexive)
  d. `∀x P(x) ∧ ∀x ¬P(x)`  (impossible)

## Section 2: Isomorphism (★★)

**2.** For each pair of structures, determine if they are isomorphic. If yes, exhibit an
isomorphism. If no, find a first-order sentence true in one but not the other.

  a. M₁ = ({0,1,2}, <) and M₂ = ({a,b,c}, <) where < in M₂ is a<b, b<c
  b. M₁ = (ℕ, <) and M₂ = (ℤ, <)
  c. M₁ = ({1,2,3,4}, divides) and M₂ = (𝒫({a,b}), ⊆)

## Section 3: Compactness Applications (★★★)

**3.** Use compactness to prove: if a first-order sentence σ is true in every finite model,
then σ has an infinite model.
(Hint: add constants c₁ ≠ c₂, c₂ ≠ c₃, ...; apply compactness.)

**4.** Use compactness to show that the property "the domain has exactly n elements" is not
expressible by a single first-order sentence for any fixed n.

**5. Challenge**: Show that the property "the graph is connected" is not expressible in FOL.
(Hint: build a family of disconnected graphs satisfying any finite set of sentences that
all connected graphs satisfy, then use compactness.)
