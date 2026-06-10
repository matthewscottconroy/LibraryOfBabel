# Chapter 17: H-Levels and Truncations

There is a question that sounds philosophical but is mathematically precise: how complicated can equality be?

Think about what it would mean for equality to be complicated. In a simple type — the booleans, the natural numbers — equality is a decision: either n = m or n ≠ m, and there is exactly one way for things to be equal. In a more complex type — the circle, the universe — equality carries genuine structure. Two loops at the basepoint of S^1 are equal iff they have the same winding number. Two types in the universe are equal iff they are equivalent, and the path between them encodes the specific equivalence. The equality of types is not a yes-or-no question; it is a rich mathematical object.

The h-level hierarchy is the precise measure of this complexity.

At h-level -2: contractible types. One element, up to equality. Everything is equal to the center. There is no question to ask.

At h-level -1: propositions. Any two elements are equal. Equality is trivial once it exists — but it may or may not exist. These are the truth values of constructive logic.

At h-level 0: sets. Elements may be unequal, but when they are equal, there is exactly one proof of their equality. The equality relation is a property, not a structure. These are the types of ordinary mathematics: natural numbers, integers, reals, groups (if you look only at their underlying sets).

At h-level 1: groupoids. Elements can be connected by paths, and between two paths there can be at most one 2-path. The type looks locally like a set, but globally has non-trivial loop structure.

At h-level n: n-groupoids. The tower of identity types is non-trivial up to level n and trivially simple above.

And then there are the types that sit at no finite h-level: the circle, the spheres, the universe. Types where the tower of identity types never stabilizes, where every level carries new information.

Why does this hierarchy matter?

It matters first because it tells us when proof terms are irrelevant. Propositions are where proof doesn't matter: all proofs are equal, so the specific proof you used carries no information. When you use a proposition in a proof, you can freely "forget" the specific evidence you found. This is the formal content of the phrase "the proposition is true" — it means we have evidence, but the evidence itself is trivial.

It matters second because it identifies where classical mathematics lives. Sets — h-level 0 — are the home of ordinary mathematics. Natural numbers, polynomials, topological spaces (as sets with extra structure), groups (as sets with operations) — all of these live at h-level 0 or, counting the structure, at h-level 1. The homotopy type theory of sets is essentially the same as classical mathematics. The novelty of HoTT appears when we go higher.

It matters third because it tells us the shape of the universe. The Univalence Axiom (Chapter 18) implies that the universe Type is not a set — its path type is equivalent to the type of equivalences, which can be rich. Type is at least a groupoid and probably sits at no finite h-level. The hierarchy tells us what kind of mathematical object the universe itself is.

This chapter develops the hierarchy systematically. We start at the bottom, with contractible types, and work upward. At each level we give examples, prove closure properties, and explain the mathematical significance.

The last section introduces truncations — the operations that *force* a type down to a given h-level. Given any type A, we can form its propositional truncation ‖A‖ (which is a proposition), its set truncation ‖A‖₀ (which is a set), and so on. These truncations are higher inductive types — a preview of Chapter 19 — and they provide the formal apparatus for "forgetting" higher homotopy structure when we don't need it.

The h-level hierarchy is one of the organizing principles of the entire theory. Every definition, every theorem, every construction in HoTT has an h-level — the level at which it naturally lives. Learning to read these h-levels is as fundamental as learning to check types. It is, in a sense, checking types at the next level up.
