# Boolos's Plural Quantification

George Boolos (1984, 1985) developed plural logic as a response to a philosophical puzzle about second-order logic and its apparent commitment to Platonic abstract objects (sets, classes, properties).

## The Geach-Kaplan Sentence

Peter Geach observed that some English sentences resist first-order formalization:

> "Some critics admire only one another."

A first-order attempt requires introducing a set-variable, quantifying over sets. Boolos argued this is wrong — English speakers use this sentence without any concept of sets. The correct formalization is plural:

```
∃xx [∃x (x ≺ xx) ∧ ∀x (x ≺ xx → Critic(x)) ∧
     ∀x∀y (x ≺ xx ∧ Admires(x,y) → y ≺ xx ∧ x ≠ y)]
```

"There are some xx such that: at least one of them exists; each of them is a critic; and for any of them x, if x admires y, then y is one of them and x ≠ y."

No sets. No abstract objects. Just individuals quantified plurally.

## The Formal Language PFO

Plural First-Order Logic (PFO) extends FOL with:

**Syntax**:
- Plural variables: xx, yy, ...
- Plural quantifiers: ∃xx, ∀xx
- Membership predicate: x ≺ xx
- (Optionally) plural predicates: plural terms as arguments

**Semantics**: Interpret plural variables as *pluralities* — non-empty collections of individuals from the domain. But these "collections" are not set-theoretic objects; they are merely the individuals themselves, referred to plurally.

## Comprehension for Plurals

A key axiom schema: for any condition φ(x) with a free singular variable x, if some x satisfies φ, then there are some xx that are exactly the φ-things:

```
∃x φ(x) → ∃xx ∀x (x ≺ xx ↔ φ(x))
```

This is analogous to separation in set theory — but without creating a set object, just acknowledging a plurality.

## Expressive Power

PFO has the same expressive power as *monadic second-order logic* (MSO) — second-order logic where set quantifiers range only over sets of individuals (not sets of sets, relations, etc.).

This is stronger than FOL:
- PFO can define finiteness (a property not expressible in FOL by Löwenheim-Skolem)
- PFO can characterize the natural numbers categorically (unlike FOL, which has non-standard models)

Yet PFO is arguably more ontologically innocent than full second-order logic: it adds no abstract objects to the domain, only plural ways of referring to the existing individuals.

## Philosophical Significance

Boolos's work opened a debate: is second-order logic logic, or set theory in disguise? If plural interpretations give second-order logic an ontologically innocent reading, perhaps it is genuinely logic — not a commitment to abstract objects.

This connects to *neo-logicism*: the project (Hale, Wright) of showing that mathematics reduces to logic plus definitions. If plural logic is "pure logic," and arithmetic reduces to plural logic, then arithmetic is logic — a vindication of Frege's original program without its paradoxes.
