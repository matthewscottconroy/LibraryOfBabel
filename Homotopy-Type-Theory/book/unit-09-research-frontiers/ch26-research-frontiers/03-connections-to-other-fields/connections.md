# 03 — Connections to Other Fields

## The Shape of Cross-Disciplinary Mathematics

There is a difference between an analogy and a connection. An analogy is a structural resemblance that helps you think but does not generate theorems. A connection is a functor, a formal map from one domain to another that preserves structure, carries theorems across, and can be used to transport insights in both directions.

HoTT's connections to other fields are, in the best cases, genuine connections: formal correspondences that have already produced theorems and that promise to produce more. In other cases, they are deep analogies that are still being made precise — programs in the mathematical sense, ongoing attempts to find the right definitions that would turn an analogy into a theorem.

What follows is a survey of both. Where the connection is a theorem, the theorem is stated. Where the connection is a program, the program is described precisely enough that you can assess its current status and potential.

---

## Algebraic K-Theory

**The connection.** Algebraic K-theory assigns to a ring R a sequence of abelian groups K₀(R), K₁(R), K₂(R), ..., which measure the "stable isomorphism classes" of projective R-modules. In HoTT, this has a natural formulation: K₀(R) is the group completion of the monoid of isomorphism classes of finitely generated projective R-modules, where "isomorphism class" is handled by the univalent treatment of mathematical structures (isomorphic structures are equal).

More precisely: define the type of projective R-modules and the equivalence relation of isomorphism. The group completion of the resulting monoid is K₀(R). Because univalence makes isomorphic modules equal, this construction is cleaner in HoTT than in set theory: you do not need to quotient explicitly.

**Higher K-groups.** The higher K-groups Kₙ(R) have a beautiful HoTT formulation: Kₙ(R) = πₙ(K(R)), where K(R) is the K-theory spectrum. In HoTT, the K-theory spectrum is built using the delooping machinery for the ∞-group GL_∞(R)⁺ (the infinite general linear group with Quillen's plus construction). Each delooping gives one K-group: Kₙ(R) = πₙ(BGL_∞(R)⁺).

**Current status.** The definitions of K₀ and K₁ in Cubical Agda are accessible in principle; the group completion HIT and the basic properties of projective modules are within the library's current reach. K₁(R) = π₁(BGL(R)) has been stated but the full formalization is not complete. Higher K-groups require the spectrification machinery, which is present in outline but not fully developed. This is a long-term formalization project but a feasible one for a researcher with the background from this curriculum.

**Why it matters for HoTT.** Algebraic K-theory is one of the richest areas of algebraic topology. Its connection to L-functions (via the Birch-Swinnerton-Dyer conjecture and its relatives), to algebraic geometry (via Grothendieck's original K₀), and to number theory (via Milnor K-theory and Galois cohomology — Voevodsky's Fields Medal work) means that formalizing K-theory in HoTT would bring HoTT into contact with the deepest parts of modern mathematics. This is not a peripheral application; it is the direction Voevodsky himself was heading when he started the Univalent Foundations program.

---

## Topological Field Theories

**The connection.** A topological field theory (TFT) in the Atiyah-Segal sense is a symmetric monoidal functor from a category of cobordisms to a category of vector spaces. The cobordism hypothesis (Baez-Dolan 1995, proved by Lurie 2009) classifies fully extended TFTs: they are determined by their value on a point, which must be a fully dualizable object in the target ∞-category.

In HoTT, this classification has a synthetic reformulation. The ∞-category of fully extended TFTs is equivalent (by the cobordism hypothesis) to the ∞-groupoid of fully dualizable objects in the target. In HoTT's language, "classifying fully extended TFTs" means computing the type of fully dualizable objects — a purely type-theoretic computation.

**The Baez-Dolan cobordism hypothesis in HoTT.** The cobordism hypothesis is not yet proved in HoTT. Proving it synthetically would require:
(a) A definition of the ∞-category of cobordisms as a Segal type in simplicial type theory
(b) A definition of full dualizability as a property of objects in a symmetric monoidal Segal type
(c) A proof that the evaluation functor (restricting a TFT to its value on a point) is an equivalence of ∞-groupoids

This is a long-term project at the interface of simplicial type theory and physics. It has not been attempted in any proof assistant.

**Why it matters.** The cobordism hypothesis is one of the central results of modern mathematical physics. A synthetic proof in HoTT would not just verify Lurie's proof (which is over 100 pages and not easily checked by hand) but potentially illuminate it — by identifying which properties of the cobordism category drive the result and which are incidental. The synthetic proof would also be machine-verifiable, a significant gain for a result that has been difficult to check independently.

---

## Chromatic Homotopy Theory

**The connection.** Chromatic homotopy theory organizes the stable homotopy groups of spheres by "chromatic height" — a notion coming from the theory of formal group laws. The chromatic filtration decomposes the sphere spectrum as:

    𝕊 = L₀𝕊 ← L₁𝕊 ← L₂𝕊 ← ...

where Lₙ is the Bousfield localization at Morava K-theory K(n). The Morava K-theories K(n) detect the "height n" information in stable homotopy: they see the relationship between stable homotopy and formal group laws of height n, which are classified by fields of characteristic p.

**HoTT and p-localization.** The p-localization modality from modal HoTT (Chapter 25) is the natural setting for the chromatic story at the first level (n = 0: rational homotopy; n = 1: p-local homotopy via K(1)). Constructing the higher Morava K-theories as modalities in HoTT would require new modal axioms, describing the formal group law structure.

**What is known.** The p-localization modality is well-understood in HoTT and has been formalized in Cubical Agda. The relationship between p-localization and the first level of the chromatic filtration (the Adams summand, K(1)-localization) is understood at the level of definitions. Higher chromatic levels (K(n) for n ≥ 2) require the theory of formal group laws of height n, which is an algebraic input not yet integrated into any HoTT library.

**Why this is the frontier.** Chromatic homotopy theory sits at the intersection of algebraic topology, number theory, and algebraic geometry. The connections are deep — the chromatic filtration corresponds to the stratification of the moduli stack of formal groups by height, which connects to the arithmetic of p-adic L-functions via the work of Devinatz-Hopkins-Smith. Bringing this into the synthetic HoTT setting would be a major development.

---

## Condensed Mathematics and Cohesive Type Theory

**The connection.** Condensed mathematics (Clausen-Scholze, 2019–2021) is a new approach to algebra and topology that replaces topological spaces with "condensed sets" — sheaves of sets on the site of profinite sets. The motivation is to find an algebraic foundation for analysis that behaves better than classical topological algebra (in particular, that has good homological properties, like enough projective objects).

Cohesive HoTT (Chapter 25) is a type theory with modalities that describe the relationship between "discrete" and "continuous" mathematics. The shape modality ʃ extracts the homotopy type of a space; the flat modality ♭ gives the underlying discrete set; the sharp modality # gives the codiscrete (indiscrete) structure.

**The formal connection.** Scholze's condensed sets are sheaves on the category of profinite spaces. Profinite spaces are the pro-objects of finite discrete spaces — the completions with respect to all finite quotients. The "condensed" structure on a topological group G is a sheaf that assigns to each profinite space S the set of continuous maps S → G.

In cohesive HoTT, the shape modality generates something analogous: the "shape" of a type is its underlying homotopy type, obtained by inverting the paths that come from the cohesive structure. The pyknotic sets of Barwick-Haine and the condensed sets of Clausen-Scholze are different concretizations of this idea.

**Pyknotic objects and HoTT.** Barwick and Haine have developed a framework for "pyknotic objects" (the condensed sets of profinite sets) that is more explicitly connected to higher topos theory and thus to HoTT. Their work suggests that cohesive HoTT with the right pyknotic axioms could provide a synthetic foundation for condensed mathematics.

**Why this matters.** The Scholze-Clausen program is one of the most exciting developments in modern mathematics. Liquid tensor experiments (the formalization of a key Scholze theorem in Lean 4 by a team including Scholze himself) demonstrated that formalization of condensed mathematics is feasible. Connecting the condensed setting to cohesive HoTT would give a type-theoretic foundation for this entire program.

---

## Programming Language Theory: Modal and Graded Types

**The connection.** Dependent type theory is already the foundation for programming language semantics (the Curry-Howard correspondence, the propositions-as-types doctrine). HoTT extends this in a specific direction: it adds homotopy-theoretic content to types, making identity types into path spaces and allowing types to have non-trivial higher structure.

Current PL theory research is developing several extensions of type theory that connect naturally to HoTT:

**Graded types and linear types.** Linear type theory tracks resource usage: a linear variable is used exactly once. Graded type theory generalizes this: a type variable has a "grade" (from a semiring R) that specifies how many times it can be used. The semiring can be ℕ (for counting uses), or {0, 1, ω} (for specifying whether a value is erased, used once, or used freely), or more exotic structures.

The connection to HoTT: the identity type has a natural "grade" interpretation. A proof of a = b uses the identity path, which has a direction; a proof of a = b = c uses two paths; composition is linear in the paths. Graded types could make this explicit, giving a framework where "path composition" is a typed operation with resource tracking.

**Cohesive modalities and effects.** Effect systems in PL track what a computation does — reads, writes, exceptions, non-termination. A cohesive modality (♭, #) can be interpreted as tracking whether a computation accesses the ambient cohesive structure (is it a "pure" discrete computation, or does it depend on continuous structure?). This gives a type-theoretic effect system for geometry.

**Applications.** Graded modal type theory (GMTT) is an active research area. Bernardy, Boespflug, Newton, Jones, and Spiwack (2018) developed a system for tracking linearity in Haskell. Atkey (2018) showed how graded types subsume many ad hoc type system extensions. Connecting this to HoTT's path algebra could produce type theories where resource tracking and homotopy-theoretic structure coexist.

**What would make this connection a theorem.** A model of a graded version of HoTT in which:
(a) The grade semiring tracks path-algebraic complexity (the number of compositions, the dimension of the paths)
(b) The univalence axiom is compatible with the grading
(c) HITs can be specified with grades that describe the dimension of their constructors

Such a model would simultaneously be a foundation for higher-dimensional rewriting theory and a model of a resource-aware homotopy type theory — connecting PL theory and pure mathematics in a new way.

---

## How These Connections Generate New Mathematics

The connections described above are not merely inspirational. They point at specific mathematical work:

**K-theory → HoTT:** Formalizing the Quillen Q-construction or the Waldhausen S-construction in Cubical Agda would be a direct contribution, connecting to existing library infrastructure on group completions and spectra.

**TFTs → simplicial type theory:** Defining the cobordism Segal type in Rzk is a tractable formalization project. The ingredients — Rezk types, symmetric monoidal structure, dualizability — are partly available in sHoTT.

**Chromatic homotopy → modal HoTT:** The p-localization modality is already formalized. Extending it to K(1)-localization (the Adams summand) requires specifying the correct cofiber sequence and checking that it has the right properties — a specific, defined research task.

**Condensed mathematics → cohesive HoTT:** David Jaz Myers' work on cohesion (arXiv:2102.05848) already sets up the framework. Connecting Myers' cohesive HoTT to the condensed setting of Clausen-Scholze is a specific research program, not a vague aspiration.

**Graded types → HoTT:** The graded modal type theory of Moon, Farka, and Orchard (2021) provides a starting point for connecting grade structures to path algebras. A theorem connecting their grading to cubical intervals would be a genuine new result.

The field is young enough that these connections, when made precise, will be new theorems — not exercises in translating known results, but genuine discoveries about the relationship between different parts of mathematics. That is the reward for doing the translation work carefully.
