# Applications: Simplicial Sets

## 1. Machine-Verified Mathematics and Proof Assistants

The most direct application of simplicial set theory is the one that motivated Voevodsky: machine-verified mathematics. Proof assistants like Coq, Agda, and Lean implement type theory as their foundation, and with HoTT as an extension, they can formalize mathematics with a level of rigor that human mathematicians cannot match.

The simplicial set model is what makes HoTT a trustworthy foundation for this enterprise. When a proof assistant checks a proof in HoTT, it is implicitly checking that the proof corresponds to a valid construction in the simplicial set model — a construction in ordinary set theory that can, in principle, be verified by hand. The machine-verification step replaces the "in principle" with "in practice."

Voevodsky's UniMath library (developed at IAS in Coq) contains formalized mathematics at the level of graduate algebra, topology, and category theory. The HoTT library (in Coq and Agda) contains synthetic proofs of classical theorems: $\pi_1(S^1) = \mathbb{Z}$, the Seifert-van Kampen theorem, the Blakers-Massey theorem. These are not just translations of classical proofs but new synthetic proofs that are cleaner, more general, and machine-verified.

The connection to simplicial sets: the type theory implemented in the proof assistant is sound relative to the simplicial set model. Every theorem proved in the proof assistant holds in the simplicial set model, hence in classical mathematics. The model provides the semantic guarantee.

## 2. Homotopy Theory Made Computable

Before simplicial sets, homotopy theoretic computations required continuous maps, integral formulas, and analytic arguments. After simplicial sets, many computations reduce to finite combinatorics: counting simplices, computing face and degeneracy maps, and solving linear systems.

The key: for spaces that are presented as CW complexes or simplicial sets with finitely many non-degenerate simplices, the cellular chain complex is finite and explicitly computable. Homotopy groups can be computed (for low-dimensional spaces and low homotopy groups) by algorithms based on Postnikov sections and simplicial group theory.

The software package *Kenzo* (Rubio, Sergeraert, and collaborators) implements these algorithms, computing homotopy groups of spaces presented as simplicial sets. It has computed $\pi_k(S^n)$ for small $k$ and $n$, verified classical theorems about spectral sequences, and extended classical results to new cases.

The theoretical foundation: simplicial sets provide a finitary, algebraic representation of topological spaces. Algorithms operating on this representation can compute topological invariants. This is applied algebraic topology, with simplicial sets as the lingua franca.

## 3. Topological Data Analysis: Simplicial Complexes as Approximations

Persistent homology (mentioned in Chapter 13 applications) computes the homology of a nested sequence of simplicial complexes built from data. The simplicial complex at each scale is a special case of a simplicial set — one with only finitely many simplices in each degree.

The computational engine of persistent homology is linear algebra applied to the boundary matrices of simplicial chain complexes. Each simplicial complex in the filtration has an associated chain complex $C_* = (C_n, \partial_n)$, and the persistence algorithm tracks how the homology $H_n(C_*)$ changes as the complex grows.

The mathematics behind this: the Dold-Kan correspondence (Exercise 15.9) establishes a precise relationship between simplicial abelian groups and chain complexes. The homology of a simplicial complex, as computed by persistent homology algorithms, is exactly the homology of the normalized chain complex via Dold-Kan.

Recent work extends persistent homology from homology (which is homotopy-invariant but relatively coarse) to homotopy groups and other invariants. The simplicial set framework — richer than simplicial complexes because of degeneracy maps — is the natural setting for these extensions.

## 4. Higher Gauge Theory and Physics

In mathematical physics, gauge theories describe the interactions of fundamental particles via connection forms on principal bundles. Classical gauge theory is based on Lie groups and differential geometry. *Higher gauge theory* extends this to higher-categorical structures, where gauge transformations have gauge-of-gauge transformations, and so on up the tower.

Simplicial sets provide the combinatorial framework for higher gauge theory. A *simplicial principal bundle* over a simplicial set $B$ is a Kan fibration $E \to B$ with fiber a simplicial group $G$. Higher gauge fields are simplicial 1-cocycles with values in a simplicial group. The gauge transformations are 0-cochains, and the gauge-of-gauge transformations are simplicial homotopies.

This framework has been used in:
- The Dijkgraaf-Witten topological quantum field theory: a topological field theory defined using representations of finite groups and simplicial cochains.
- String theory backgrounds: higher gauge fields on spacetime, modeled as simplicial 2-cocycles with values in simplicial 2-groups.
- M-theory: the $C_3$ field of M-theory is a higher gauge field whose proper mathematical treatment requires 2-group bundles or simplicial bundles.

The simplicial set framework makes these constructions precise and computable. The physics requires the mathematics of simplicial sets; the mathematics is clarified by the physical motivation.

## 5. Derived Algebraic Geometry

Grothendieck's algebraic geometry studies varieties and schemes via their functor of points — the functors they represent on rings. Derived algebraic geometry extends this to the "derived" level: instead of rings, one uses differential graded algebras or simplicial commutative rings, and instead of sets of points, one uses spaces (simplicial sets) of points.

A *derived scheme* is a functor $X$ from simplicial commutative rings to simplicial sets, satisfying suitable sheaf conditions. The geometric realization $|X(R)|$ for a specific ring $R$ gives the classical scheme's points. But the derived information — the higher homotopy groups of $X(R)$ — captures intersection multiplicities, deformation spaces, and other data that classical algebraic geometry misses.

Lurie's *Derived Algebraic Geometry* program (in his books *Higher Topos Theory* and *Spectral Algebraic Geometry*) develops this framework. The key objects are $\infty$-categories of derived schemes, where the morphisms between derived schemes are homotopy classes of maps, and the higher-category structure tracks homotopies between homotopies.

Applications include:
- The geometric Satake correspondence (relating representations of Langlands dual groups to perverse sheaves on affine Grassmannians).
- The Geometrization conjecture of the Langlands program.
- The theory of virtual fundamental classes in Gromov-Witten theory.

In each case, simplicial sets provide the combinatorial model for the "spaces" of algebraic geometry, and the model structure on $\mathbf{sSet}$ provides the homotopy theory needed to make the constructions work.

## 6. Homotopy Type Theory as a Programming Language

HoTT is not just a mathematical foundation — it is a programming language in the Curry-Howard sense. Every proof in HoTT is a program, and every type is a specification. The simplicial set model ensures that the programs compute correctly: every term in HoTT has a computational behavior that corresponds to the geometric behavior of the corresponding simplicial set construction.

The programming language aspect has been developed in:
- *HoTT-Agda*: the Agda proof assistant with HoTT axioms added. Programs in HoTT-Agda correspond to constructions in the simplicial set model.
- *Cubical Agda*: an extension with computational univalence. Programs run; type checking terminates; computations produce results.
- *Lean 4* with HoTT libraries: a newer proof assistant platform.

Applications in computing:
- *Homotopy-theoretic program verification*: using the path types of HoTT to verify that programs satisfy specification (two programs have the same behavior if and only if they are homotopic — related by a continuous path in the space of programs).
- *Certified compilation*: using type theory to prove that a compiler preserves the meaning of programs.
- *Formalized mathematics*: using proof assistants based on HoTT to verify mathematical theorems automatically.

The simplicial set model is the semantic guarantee: it shows that the type theory is consistent, that programs cannot produce contradictions, and that the verification results are trustworthy.

## 7. Category Theory Internalized

Simplicial sets are the foundation for higher category theory. A small category can be represented as its nerve (a simplicial set); higher categories are simplicial sets with various "composition" structures. The theory of $(\infty, 1)$-categories (Lurie's *Higher Topos Theory*) uses simplicial sets (quasi-categories) as its fundamental objects.

This has applications in:
- *Homotopical algebra*: the theory of derived functors in non-abelian settings, using model categories and $\infty$-categories.
- *Topological field theory*: $(\infty, 1)$-categorical structures appear naturally in the classification of extended topological field theories (Baez-Dolan-Lurie cobordism hypothesis).
- *Algebraic K-theory*: Waldhausen's S-construction and its higher analogues are naturally expressed using simplicial sets.

The connection to HoTT: HoTT is the internal language of $(\infty, 1)$-toposes, which are the high-dimensional versions of Grothendieck toposes. Every $(\infty, 1)$-topos has an internal language that is a form of HoTT. The simplicial set model is the $(\infty, 1)$-topos of simplicial sets, and HoTT is its internal language. This is the mathematical meaning of Voevodsky's program: HoTT captures the internal logic of the most fundamental $(\infty, 1)$-topos, and theorems of HoTT are theorems about all $(\infty, 1)$-toposes simultaneously.
