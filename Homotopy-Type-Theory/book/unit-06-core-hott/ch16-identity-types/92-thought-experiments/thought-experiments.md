# Thought Experiments: Identity Types and Paths

## Thought Experiment 1: The Proof-Relevant Archivist

Imagine a mathematician who keeps a logbook. Every time she proves that two things are equal, she records not just the fact of equality but *which proof she used*. After many years, she notices something: different proofs of the same equality are useful in different contexts. The proof that 2+2=4 via the Peano axioms is useful when you need to know the proof is inductive. The proof via direct computation is useful when you need to know the result immediately. Two proofs — same conclusion, different information.

The classical mathematician says: "Both proofs are correct, and the conclusion is what matters." The HoTT mathematician says: "The proofs are different elements of the identity type 2+2 = 4. In this particular type, they happen to be equal as paths — the identity type `2+2=4` is a proposition. But in general, proof identity matters."

*Question:* Can you think of a mathematical context where the *identity* of an equality proof — not just the fact of equality — would be mathematically significant? (Hint: think about an algebraic structure where the same element can be equal to another via two different group-theoretic paths.)

## Thought Experiment 2: The Topological Elevator

Consider a building with floors labeled by elements of a type A. You can travel between floors via elevators — but only if there is a path between the corresponding elements of A. The constant path refl_a is the elevator that never moves. A non-trivial path p : a = b is a moving elevator from floor a to floor b.

Now suppose some floors are connected by *two different elevator routes*. You can take the east elevator or the west elevator from floor a to floor b. These are different paths — different elements of `a = b`. They may take you to the same destination but by different routes, and if you are carrying furniture (a family of types P over A), the furniture arrives in different configurations depending on which elevator you took (different transports).

*Question:* If the building has a circular arrangement of floors — elevator routes that, when composed, return to the starting floor but with a net effect (like winding once around the circle) — what type does this building correspond to? What does the "net effect" mean in terms of transport?

## Thought Experiment 3: Definitional vs. Propositional Equality

The right unit law `p · refl = p` holds definitionally — the two sides reduce to the same normal form. The left unit law `refl · p = p` holds only propositionally — you must prove it by J.

This asymmetry disturbs some students. "Shouldn't left and right unit be symmetric?" Yes and no. The asymmetry comes from *how we defined concatenation* — by inducting on the second argument. If we had inducted on the first, the asymmetry would be reversed.

*Question:* Suppose we define concatenation by inducting on the first argument instead. Write out the new computation rule and the new statement of which unit law holds definitionally. Does the choice of definition affect what we can *prove*? (Hint: the two definitions are propositionally equal — related by a 2-path — but they are not definitionally equal.)

## Thought Experiment 4: The Eckmann-Hilton Surprise

The Eckmann-Hilton argument shows that 2-loops (elements of Ω²A) must commute. This is surprising because 1-loops (elements of ΩA) need not commute — the fundamental group of a space can be non-abelian.

Here is a striking consequence. Consider a type A that is a "1-type" — a type where all identity types a = b are sets (have at most one path between any two paths). Such a type corresponds to an ordinary groupoid. The fundamental group π₁(A, a) can be any group — including non-abelian groups.

But the fundamental group π₂(A, a) — the group of 2-loops — is always abelian, by Eckmann-Hilton.

*Question:* Where does the argument break down if you try to apply Eckmann-Hilton to 1-loops? What is the key difference between the 1-dimensional and 2-dimensional cases that makes commutativity fail at dimension 1 but succeed at dimension 2?

## Thought Experiment 5: Transport as Memory

Consider a family P : R → Type where R is some "time line" — a type representing times. An element x : P(t) is the "state at time t." A path p : t₁ = t₂ allows us to transport: transport^P(p, x) is the state x "remembered" at time t₂.

*Question:* What properties of transport ensure that "memory" is consistent? Specifically: (1) No change along the constant path (computation rule). (2) Memory is reversible (transport is an equivalence). (3) Chained memories are consistent (transport over a concatenated path equals the composition of individual transports).

Verify that all three properties hold for the abstract transport function defined by J.

## Thought Experiment 6: Functions are Automatically Continuous

In topology, you must explicitly verify that a function f : X → Y is continuous — that it preserves open sets (or, equivalently, paths). In HoTT, every function is automatically continuous, because every function has an associated ap_f that maps paths to paths.

This is more than a convenience. It means that in HoTT, there is no room for "discontinuous functions." Every definable function is homotopy-invariant. This is sometimes called the *continuity principle*.

*Question:* Does this mean HoTT cannot reason about discontinuous functions? Or does it mean that "discontinuous functions" are simply not the kind of thing that can be expressed as a function between types? Think about what the type of "a function from R to R that is not continuous at 0" would look like in HoTT.
