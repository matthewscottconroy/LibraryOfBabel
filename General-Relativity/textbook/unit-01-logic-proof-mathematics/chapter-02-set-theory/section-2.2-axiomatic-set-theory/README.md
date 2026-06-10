# Section 2.2: Axiomatic Set Theory

---

## Section Introduction

Naive set theory, introduced in Section 2.1, treats sets as collections defined by any property whatsoever. This freedom is its appeal — and its fatal flaw. Russell's paradox, derived from the "set of all sets that do not contain themselves," exposes a contradiction at the heart of naive set theory. Something must be done.

The response, developed in the early twentieth century by Zermelo and then refined by Fraenkel and Skolem, was to replace the naive comprehension axiom with a carefully restricted system: the **Zermelo-Fraenkel axioms (ZF)**, with the optional addition of the **Axiom of Choice (AC)**, giving the system **ZFC**. The strategy is defensive: instead of allowing any property to define a set, the axioms specify exactly which set-forming operations are permitted. The contradiction-generating sets simply cannot be formed.

ZFC is the standard foundation for virtually all of mathematics. When a modern mathematician says "prove it from the axioms," they almost always mean "prove it in ZFC." The axioms are carefully chosen to be (apparently) consistent, to be strong enough to derive the mathematics we need, and to be independent enough to avoid paradox. Whether ZFC is truly consistent cannot be proved within ZFC — this is the content of Gödel's second incompleteness theorem — but in decades of use, no contradiction has been found.

The Axiom of Choice deserves special attention. It asserts that given any collection of nonempty sets, there exists a function that selects exactly one element from each. This seems obviously true for finite collections, and for many infinite collections it is provably true without AC. But for uncountable collections of sets with no specified structure, AC cannot be proved from the other ZF axioms (Cohen, 1963). The axiom is independent — it can be consistently added or denied. Most mathematicians accept AC because of the theorems it enables; some constructivists reject it for philosophical reasons. In GR, the Axiom of Choice appears implicitly whenever we assert the existence of bases for function spaces or invoke Zorn's lemma to construct geodesics.

---

## Subsections

- [2.2.1: The ZF Axioms](2.2.1-zf-axioms.md)
- [2.2.2: The Axiom of Choice and Its Equivalents](2.2.2-axiom-of-choice.md)
- [2.2.3: Ordinals and Cardinals in ZFC](2.2.3-ordinals-cardinals.md)
- [2.2.4: Gödel's Incompleteness Theorems](2.2.4-godel.md)
