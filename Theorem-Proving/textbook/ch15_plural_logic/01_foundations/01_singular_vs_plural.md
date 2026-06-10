# Singular vs. Plural Reference

In ordinary language, we distinguish singular and plural reference:
- "The book is on the shelf." — singular reference to one object
- "The books are on the shelf." — plural reference to multiple objects

First-order logic handles singular reference naturally (variables refer to one object). But it handles plural reference awkwardly: to say "the books are heavy," we need either a set (the set of books is heavy) or a predicate (everything that is a book is heavy).

## The Problem with Sets

Consider: "Some critics admire only one another."

In first-order logic over individuals, this requires existential quantification over a *set* of critics:
```
∃S (S is a nonempty set of critics ∧ ∀x∈S ∀y (x admires y → y∈S ∧ x≠y))
```

But we've introduced a set S — a mathematical object that wasn't in our original domain. We've changed the ontological commitments of our logic.

George Boolos (1984) observed this is unnecessary. Ordinary English handles the sentence without sets:

"There are some critics who admire only one another."

The "some critics" refers plurally to several individuals simultaneously — not to a set.

## Plural Variables and Quantification

Plural logic introduces:
- **Plural variables**: xx, yy, zz (ranging over *many* individuals)
- **Plural quantifiers**: ∃xx ("there are some things xx such that..."), ∀xx ("for any things xx...")
- **Plural membership**: x ≺ xx ("x is one of the xx")

Reading: ∃xx φ(xx) means "there are some things (the xx) such that φ holds of them."

Key properties:
- xx is not a set — it's a plural term denoting individuals plurally
- "x ≺ xx" is not set membership — it's a logical primitive
- We do not commit to the existence of any object beyond the individuals

## Boolos's Insight

Boolos showed that second-order logic (which quantifies over sets/relations) can be interpreted as plural logic — avoiding the ontological commitment to sets as objects.

"There are some sets such that..." becomes "there are some things xx such that each of them is a set..."

This gives a nominalist-friendly interpretation of second-order quantification: instead of quantifying over abstract set-objects, we quantify over pluralities of concrete individuals.
