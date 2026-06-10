# Section 1.2: Predicate Logic

---

## Section Introduction

Propositional logic is powerful, but it has a fundamental limitation: it cannot talk about the *internal structure* of propositions. It can tell us that "P and Q implies R" is a valid form of argument, but it cannot tell us whether "every even integer greater than 2 is the sum of two primes" is provable from anything. The statement has the internal structure of a universal claim about all objects of a certain kind, and propositional logic has no way to express or reason about that structure.

**Predicate logic** (also called *first-order logic* or *quantificational logic*) fills this gap. It introduces two new elements: **predicates**, which express properties of or relations among objects, and **quantifiers**, which make claims about how many objects satisfy a predicate. With these tools, we can express virtually every mathematical claim: "for every ε > 0 there exists δ > 0 such that...," "there exists a prime between n and 2n for all n ≥ 1," "every continuous function on a closed interval achieves its maximum."

The language of predicate logic is the language in which physics is ultimately expressed. When Einstein wrote "spacetime is a four-dimensional Lorentzian manifold with a metric satisfying the field equations," every one of those terms is defined via predicate logic: "for all manifolds M, if M satisfies condition X, then Y," and so forth. The chain of definitions eventually bottoms out in set theory (Chapter 2), which is itself expressed in predicate logic.

Mastery of predicate logic — and especially of how to correctly negate quantified statements — is one of the most practically useful skills in this book.

---

## Subsections

- [1.2.1: Predicates and Quantifiers](1.2.1-predicates-and-quantifiers.md)
- [1.2.2: Nested Quantifiers](1.2.2-nested-quantifiers.md)
- [1.2.3: Negating Quantified Statements](1.2.3-negating-quantified-statements.md)
- [1.2.4: Free and Bound Variables](1.2.4-free-and-bound-variables.md)
