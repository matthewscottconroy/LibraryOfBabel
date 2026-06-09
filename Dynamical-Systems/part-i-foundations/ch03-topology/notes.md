# Chapter 3 — Notes

---

For foundational point-set topology, Munkres' *Topology* (Chapters 2, 3, 7, 9) is the standard reference and deservedly so — it's clear, systematic, and covers everything you'll need. The chapters on compactness and connectedness are particularly well done.

For smooth manifolds, Lee's *Introduction to Smooth Manifolds* is the definitive modern text. It's thorough and precise, and the second edition has excellent exercises. Guillemin and Pollack's *Differential Topology* offers a more geometric and problem-centered approach — it's shorter and develops a strong intuition for transversality and degree theory. Milnor's *Topology from the Differentiable Viewpoint* is a beautiful 50-page essay covering transversality, degree theory, and the Hopf theorem; if you haven't read it, put it on your list immediately. It's one of those books that changes how you think.

For algebraic topology (fundamental groups, higher homotopy groups, cohomology), Hatcher's *Algebraic Topology* is freely available online and is the contemporary standard. Hatcher is less formal than some alternatives, which some people find liberating and others find frustrating. Bredon's *Topology and Geometry* is a more structured alternative that covers both smooth manifolds and algebraic topology in a unified framework.

Poincaré-Hopf (Section 3.7.3) connects topology to dynamics by counting zeros of vector fields with signs. The Lefschetz theorem (Theorem 3.7.6) is the bridge to algebraic topology: it counts fixed points of maps via cohomology. The Nielsen fixed-point theorem, not covered here, gives a stronger count using the fundamental group — it's relevant for surface diffeomorphisms and appears in Chapters 6 and 9.

For the connection to HoTT mentioned in Remark 3.3.6: every manifold is a type, paths are homotopies, and the homotopy groups $\pi_n(M)$ are the higher inductive types of that type. The de Rham complex is a model for the cohomology of the $\infty$-topos of $M$. This connection is explored much further in the research literature on synthetic differential geometry; if you're interested, start with the HoTT book (available free at homotopytypetheory.org).

The Poincaré-Bendixson theorem (Section 3.8) is proved in most ODE textbooks. The most satisfying proof, in my opinion, uses the Jordan curve theorem directly and makes the topological constraint explicit. See Hirsch, Smale, and Devaney's *Differential Equations, Dynamical Systems, and an Introduction to Chaos* for a clean presentation.
