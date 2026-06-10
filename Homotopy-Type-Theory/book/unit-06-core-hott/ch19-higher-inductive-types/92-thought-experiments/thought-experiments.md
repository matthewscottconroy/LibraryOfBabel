# Thought Experiments: Higher Inductive Types

## Thought Experiment 1: Constructing the Torus

The torus T^2 can be defined as a HIT in multiple ways. One description: T^2 has one basepoint, two loops (p and q), and one 2-cell t that says p · q = q · p (the loops commute).

A second description: T^2 is the pushout of two cylinders S^1 × I along their boundaries (two copies of S^1).

*Question:* Show that these two descriptions give the same fundamental group. Use van Kampen on the second description. What is π₁(T^2)?

Now generalize: the genus-g surface Σ_g has 2g loops and certain relations among them. What are the relations? What is π₁(Σ_g)?

## Thought Experiment 2: The Infinite-Dimensional Sphere

The infinite-dimensional sphere S^∞ is the colimit of the sequence S^0 → S^1 → S^2 → ... (where each sphere includes into the next as the equatorial sphere).

S^∞ is contractible — it has no homotopy groups. Here is the intuition: any element of the sequence of spheres can be "pushed off" the current sphere into the next, and in the limit, there is nowhere for non-trivial paths to live.

*Question:* In HoTT, how would you define S^∞? (Hint: use a sequential colimit, which is itself a HIT with constructors corresponding to the inclusion maps and path constructors ensuring the sequence is "inductively glued.") Why is S^∞ contractible despite being defined as a colimit of non-contractible spaces?

## Thought Experiment 3: The Suspension as Forcing a Homotopy Group

The suspension ΣA of A "adds a homotopy group" in a certain range. By Freudenthal, if A is (n-1)-connected (π_k(A) = 0 for k < n), then ΣA is n-connected (π_k(ΣA) = 0 for k < n+1).

*Question:* The suspension "pushes" the first non-trivial homotopy group up by one dimension. If A has first non-trivial homotopy group at dimension n (i.e., πₙ(A) ≠ 0, πₖ(A) = 0 for k < n), then ΣA has first non-trivial homotopy group at dimension n+1. What is πₙ₊₁(ΣA) in terms of πₙ(A)?

## Thought Experiment 4: Truncation as Forgetting

The propositional truncation ‖A‖ forgets all elements of A except the fact that it is inhabited. The set truncation ‖A‖₀ forgets all path-structure, keeping only the set of connected components.

*Question:* What information is preserved and what is lost when we truncate the circle S^1 to ‖S^1‖ (propositional truncation)? To ‖S^1‖₀ (set truncation)? To ‖S^1‖₁ (groupoid truncation)? At each level, describe what the resulting type "looks like" and what homotopy information it retains.

## Thought Experiment 5: Eilenberg-MacLane Spaces as Memory

The Eilenberg-MacLane space K(G,n) is the "memory" of the group G at dimension n. Any space X with non-trivial cohomology H^n(X;G) has "holes" in dimension n that the Eilenberg-MacLane space can "detect."

*Question:* Consider the 2-sphere S^2. We know H^2(S^2; Z) = Z (the top cohomology is Z). By the representability of cohomology, maps S^2 → K(Z,2) up to homotopy correspond to elements of Z. The identity map S^2 → K(Z,2) should correspond to the generator 1 ∈ Z.

What does this map look like explicitly? (Hint: K(Z,2) = CP^∞, the infinite complex projective space, which can be defined as the Eilenberg-MacLane space K(Z,2). The generator of H^2(S^2; Z) is the "fundamental class" of S^2.)

## Thought Experiment 6: The Real Projective Plane

The real projective plane RP^2 is the space of lines through the origin in R^3 — equivalently, the space of pairs of antipodal points on S^2.

In HoTT, RP^2 can be defined as the pushout:
```
S^0 ←^{antipodal} S^1 →^{inclusion} D^2
```
where D^2 is the disc (contractible) and the antipodal map sends each point of S^0 to its antipodal point.

Alternatively, RP^2 is the quotient of S^2 by the Z/2Z action (antipodal map).

*Question:* What is π₁(RP^2)? (Hint: it is Z/2Z, the "fundamental group of the projective plane.") Can you see why from the pushout definition? Apply van Kampen.

Also: RP^2 is not orientable. How would you express non-orientability in type-theoretic terms?
