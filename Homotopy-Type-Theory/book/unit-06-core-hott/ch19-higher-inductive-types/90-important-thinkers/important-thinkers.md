# Important Thinkers: Higher Inductive Types

## Peter Lumsdaine and Mike Shulman

Lumsdaine and Shulman are primarily responsible for the foundational theory of HITs. Their 2017 paper "Semantics of Higher Inductive Types" established the precise semantics for HITs in homotopy-theoretic models, solving the fundamental problem of how to interpret HITs in simplicial sets and other model categories.

The challenge is that HITs introduce path constructors — elements of identity types — as part of the type definition, and ensuring that such definitions are consistent and have the right properties in a model requires careful work. Lumsdaine and Shulman's solution uses the theory of "algebraically cofibrant" objects in a model category, giving a clean semantic account.

Their work established that HITs are legitimate type-theoretic constructions with well-defined semantics, removing any doubt about their formal status.

## Dan Licata and Guillaume Brunerie

Licata and Brunerie were key contributors to the use of HITs in synthetic homotopy theory. Licata's work (with collaborators Finster, Harper, and others) developed the theory of HITs in Agda and Lean, making it possible to actually compute with higher inductive types in a proof assistant.

Brunerie's 2016 PhD thesis is a landmark: it proves that π₄(S³) = Z/2Z entirely within HoTT, using HITs (the circle, spheres, the Hopf fibration) and the truncation machinery. This is the first calculation of a non-trivial homotopy group of a sphere inside a proof assistant — a calculation that required significant ingenuity and established HoTT as a viable tool for cutting-edge homotopy theory.

## Samuel Eilenberg and Saunders Mac Lane (Classical)

Eilenberg and Mac Lane invented the spaces K(G,n) that bear their names in the 1940s and 1950s, as part of their foundational work on cohomology theory. Their paper "Relations between homology and homotopy groups of spaces" (1945) introduced these spaces and showed they represent cohomology.

The discovery that cohomology is representable — that there exist spaces whose maps classify cohomology groups — transformed algebraic topology and led to the development of generalized cohomology theories, spectra, and much of modern homotopy theory. In HoTT, the definition of K(G,n) as a HIT and its role in synthetic cohomology is the direct descendant of Eilenberg and Mac Lane's work.

## Thierry Coquand and Anders Mortberg

Coquand and Mortberg's work on cubical type theory (with collaborators) gave HITs a *computational* status. In cubical type theory, HITs are genuine computational objects: the circle S^1 has a `base` that computes definitionally, and the `loop` path constructor is a path in the computational sense (a path in the interval object of cubical type theory).

This means that programs involving HITs can be *run*, not just verified. The winding number of a loop in S^1 can be computed by running the encode function on the loop. This computational status is not available in Book HoTT (where HITs are axioms), and it is one of the main advantages of cubical type theory over the axiomatic approach.

## The Princeton IAS Group (2012-2013)

The HITs chapter of the HoTT Book was developed collectively during the IAS special year. Key contributors included Licata, Brunerie, Lumsdaine, and others who worked out the definitions, elimination principles, and computation rules for the main HITs (circle, spheres, suspensions, pushouts, truncations) and proved the main theorems about them (van Kampen, Freudenthal, π₁(S¹) = Z).

The collective achievement of the HoTT Book's HIT material represents one of the most rapid developments of a new mathematical theory in recent history — a year of concentrated work that established the foundations of synthetic homotopy theory.
