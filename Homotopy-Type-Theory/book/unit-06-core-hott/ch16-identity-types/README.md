# Chapter 16: Identity Types and Paths

In classical mathematics, equality is boring. Two things are equal or they are not; there is nothing more to say. In homotopy type theory, equality is a space. A proof that a equals b is a path from a to b. And between two paths there can be homotopies — paths of paths. And between those, higher homotopies. The structure never ends, and every level carries information. This is not a complication. It is a discovery. The "boring" equality relation turns out to have been an infinity-groupoid all along.

Let us sit with this for a moment, because it is genuinely surprising.

You have been writing proofs of equality your whole mathematical life. When you proved that 2+2=4, when you proved that the composition of continuous functions is continuous, when you proved that two groups are isomorphic — each of those proofs was, in the classical picture, a yes-or-no answer. The answer was yes. And that was all.

But there is more. When you prove that the composition of two continuous functions is continuous, you typically give a specific argument — an epsilon-delta argument, or a topological argument, or an algebraic argument. These different proofs are *different paths* between the same two points. In classical mathematics, we would say they prove the same thing and ignore the difference. In HoTT, the difference is the thing.

Here is the key question that HoTT answers: if equality is a space, what kind of space is it? The answer is: it depends on what kind of thing you are equating. If you are equating natural numbers, the equality space is discrete — at most one proof, either present or absent. If you are equating types in the universe, the equality space is enormous — it contains one path for each equivalence between those types. If you are equating proofs of equality, you get 2-paths. If you equate those, you get 3-paths. The tower of equality types is the tower of homotopy groups, and every type in HoTT carries this tower.

The technical heart of the chapter is the J rule — path induction. The J rule says: to prove something about all paths p:a=b, it suffices to prove it when b is a and p is the constant path refl_a. This sounds like ordinary induction, but it is something more subtle. It is saying that the based path space — the space of all paths starting at a — is contractible. Every path "comes from" the trivial path, in a specific sense that path induction makes precise. The J rule is not a simplification of equality. It is the precise content of equality's contractibility.

What we prove in this chapter:

- **Section 1** gives the full formation, introduction, elimination, and computation rules for the identity type. We see why refl is the only axiomatically given element, and why the type can still have many elements once we have a non-trivial underlying type.

- **Section 2** shows that paths can be concatenated and inverted, and that the groupoid laws hold — not definitionally, but propositionally. The distinction is crucial: associativity is not built into the definition of path concatenation; it is proved about it.

- **Section 3** goes higher. Between two paths there are 2-paths. The Eckmann-Hilton argument shows that 2-loops are automatically commutative — a purely type-theoretic proof of a classical theorem.

- **Section 4** introduces transport and the functorial action of functions on paths (ap). These are the two workhorses of all subsequent HoTT reasoning. Transport moves elements along paths; ap shows that every function is, automatically, continuous.

- **Section 5** asks: what do paths look like in product types, dependent sums, function types? The answers are clean and satisfying: paths in products are pairs of paths; paths in Sigma-types are paths-plus-transport; paths in function types are homotopies.

Each of these sections reveals something that was always true but never said. Classical mathematics defined equality as a relation and moved on. HoTT refuses to move on until it has understood equality completely. The reward is that "completely" turns out to mean "infinite-dimensionally" — and that infinity is not a problem. It is the source of everything that comes next.
