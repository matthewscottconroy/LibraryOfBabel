# Important Thinkers: H-Levels and Truncations

## Vladimir Voevodsky (1966–2017)

The h-level hierarchy is Voevodsky's invention, though the ideas behind it draw on classical homotopy theory. Voevodsky observed that the complexity of types in HoTT is measured by the complexity of their identity types, and that this complexity forms a hierarchy perfectly analogous to the Postnikov tower in algebraic topology.

His key insight was that most mathematics that mathematicians actually care about lives at h-level 0 or 1 — the level of sets and groupoids — and that the Univalence Axiom places the universe at a higher (and non-finite) h-level. This stratification resolved a puzzle: why is HoTT not just ordinary type theory? Because the universe is genuinely a space, not a set, and this spatial structure is captured by the h-level hierarchy.

Voevodsky also introduced the *univalent foundations* program, of which the h-level hierarchy is a central organizing principle: mathematics should be done in a way that is automatically invariant under equivalences within each h-level. This is the correct generalization of "invariant under bijection" from the set-theoretic world.

## Thomas Jech and the Set-Theoretic Tradition

The mathematical tradition of set theory provides the backdrop against which the h-level hierarchy is most clearly understood. Jech's "Set Theory" and related works formalize the classical view of equality as a proposition — the view that h-level 0 is the default. The h-level hierarchy shows that this is a special case: the default equality of set theory is the discrete equality of sets, which happens to be propositional.

This situating of classical set theory as the h-level 0 case of HoTT is one of the most important insights of the univalent foundations program. Classical mathematics is not wrong — it is working in the "sets" layer of a richer hierarchy. HoTT provides the layer above.

## Nicolas Brunerie and Guillaume Brunerie's Number

One striking application of the h-level hierarchy is the computation of homotopy groups of spheres. Brunerie proved (in his 2016 PhD thesis, using Agda) that π₄(S³) = Z/2Z — a classical result, but now with a computer-verifiable proof that explicitly uses the h-level structure of spheres and the machinery of truncations.

The key technical tool: the Freudenthal Suspension Theorem guarantees that π₃(S²) = Z, and the Hopf fibration gives a map S³ → S² whose homotopy fiber is S¹, leading to the computation of π₄(S³). All of this reasoning depends critically on knowing that S^n is an n-type but not an (n-1)-type — precisely the h-level classification.

## Dan Licata and Robert Harper

The computational theory of h-levels — how to work with them efficiently in proof assistants, how to define the h-level hierarchy inductively, and how to verify h-level claims automatically — was developed significantly by Licata, Harper, and their collaborators at Carnegie Mellon University. Their work on "computational higher type theory" and the implementation of HoTT in proof assistants provided the practical tools that make the h-level hierarchy usable in actual mathematical development.

## Michael Shulman

Shulman has written extensively on the precise relationship between the h-level hierarchy and the n-category theory that underlies it. His papers on "Homotopy type theory: the logic of space" and related work clarify how the h-level hierarchy corresponds to the Postnikov-tower decomposition of spaces, and how truncations correspond to n-connected maps. His contributions to the theoretical underpinnings of the hierarchy are essential for understanding why it takes the form it does.
