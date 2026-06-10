# Applications — Chapter 24: Simplicial Type Theory

## Application 1: Formal Verification of ∞-Categorical Theorems in Rzk

**The context.** Classical ∞-category theory (as developed by Joyal, Lurie, and others) contains hundreds of theorems that have never been formally verified. The proofs often involve intricate arguments about simplicial sets, horn-filling conditions, and model-category machinery. A single error could propagate through the theory.

**The application.** Rzk provides a formal verification framework for ∞-categorical results:

The Riehl-Weinberger program has formalized in Rzk:
- The Yoneda lemma for Segal types and covariant fibrations
- The theory of adjunctions: unit-counit characterization, triangular identities, uniqueness
- The theory of (co)cartesian fibrations over Segal types
- Slice Segal types and over-categories

Each formalization is a formal proof that the result holds in the type-theoretic model of ∞-categories. Since the type theory has a semantic model in the Joyal model structure on simplicial sets, these formal proofs correspond to valid theorems about quasi-categories.

**The significance.** This is the first large-scale formal verification of ∞-categorical results. Previous verification of category theory (in Lean, Coq, Agda) was restricted to 1-categories, where coherence conditions are manageable. For ∞-categories, the coherence problem made formal verification practically impossible before STT.

## Application 2: Categorical Semantics of Programming Languages

**The context.** Every programming language has a semantics — a mathematical account of what programs mean. The correct mathematical framework for type-theoretic programming languages is not set theory or simple type theory; it is ∞-category theory.

**The application.** Simplicial type theory provides tools for the categorical semantics of programming languages:

*Dependent types and fibrations*: The semantics of dependent types (types that can depend on terms) uses *fibrations* — Grothendieck fibrations in the 1-categorical case, and covariant/contravariant fibrations in the ∞-categorical case. STT provides a native language for expressing and verifying these fibration conditions.

*Denotational semantics with full coherence*: The denotational semantics of a programming language assigns a morphism in some category to each program. For languages with effects (monads), higher-order functions (cartesian-closed categories), and dependent types (locally cartesian-closed ∞-categories), the semantic categories are ∞-categories. STT allows the semantics to be stated and verified without the coherence overhead of classical higher category theory.

*The ∞-categorical Scott domain theory*: The classical Scott domain theory for the denotational semantics of recursive programs uses partial orders and least fixed points. An ∞-categorical version would use Segal types with appropriate completeness conditions. The morphisms in the semantic Segal type would be the (ω-continuous) functions, and the initial algebra theorem would give the recursion semantics.

## Application 3: Synthetic Algebraic Topology

**The context.** Algebraic topology studies spaces using algebraic invariants — homotopy groups, homology, cohomology. The classical approach builds these invariants from specific models (simplicial sets, topological spaces, CW complexes). The synthetic approach builds them from the type-theoretic structure.

**The application.** STT enables synthetic algebraic topology at the ∞-categorical level:

*The ∞-categorical Seifert-van Kampen theorem*: Classically, van Kampen computes $\pi_1$ of a pushout. The ∞-categorical version computes the *fundamental ∞-groupoid* (the full homotopy type) of a pushout. In STT, this is a statement about colimits in the Segal type of ∞-groupoids — provable using the Yoneda lemma and adjoint functor theory.

*Sheaves and local systems*: A local system on a space $X$ is a covariant fibration $L : \Pi_1(X) \to \mathsf{Vect}$ (in the 1-categorical case) or $L : X \to \mathsf{Type}$ (in the ∞-categorical case). STT's native language for covariant fibrations makes this definition immediate and the parallel transport construction automatic.

*Spectra as Segal types*: In stable homotopy theory, spectra are the fundamental objects. A spectrum can be defined as a Segal type with specific loop space conditions. In STT, the connection between spectra and ∞-categories is direct: spectra are ∞-groupoids with stable (invertible suspension) structure.

## Application 4: Formal Verification of Categorical Constructions in Mathematics

**The context.** Contemporary pure mathematics uses ∞-categorical constructions extensively — derived categories in homological algebra, ∞-toposes in geometry, ∞-operads in homotopy theory. These constructions are rarely fully verified.

**The application.** STT enables formal verification of specific ∞-categorical constructions:

*Derived categories in algebraic geometry*: The derived category of coherent sheaves on an algebraic variety is an ∞-category (a stable ∞-category). Proofs of equivalences between derived categories (e.g., the Fourier-Mukai transform) are equivalences of Segal types. In STT, the Rezk condition for the universe of stable ∞-categories would make such equivalences provably unique.

*The ∞-categorical Künneth formula*: The Künneth formula in homological algebra computes $H_*(X \times Y) \cong H_*(X) \otimes H_*(Y)$ for chain complexes with field coefficients. The ∞-categorical version uses the tensor product of stable ∞-categories. In STT, this becomes a statement about the Segal structure of the ∞-category of stable ∞-categories.

*∞-Toposes in homotopy theory*: Lurie's ∞-topos theory is the foundation for derived algebraic geometry. In STT, an ∞-topos is a Segal type satisfying additional exact-sequence conditions. The Giraud axioms for ∞-toposes can be stated using extension types and the Segal/Rezk conditions.

## Application 5: Type-Theoretic Foundations of Derived Algebraic Geometry

**The context.** Derived algebraic geometry (DAG) — initiated by Toën-Vezzosi and Lurie — extends classical algebraic geometry by replacing rings with ring spectra (simplicial commutative rings or $E_\infty$-rings). The geometric objects are *derived schemes* and *derived stacks*. The morphisms are maps of ∞-toposes.

**The application.** STT provides a native framework for DAG:

*Simplicial commutative rings as Segal types*: A simplicial commutative ring can be understood as a Segal type with compatible ring structure. The comparison with classical algebraic geometry is: ordinary schemes correspond to 0-truncated Segal types (sets with ring structure), while derived schemes correspond to general Segal types.

*Spectral algebraic geometry in STT*: The Riehl-Shulman program includes (in future work) spectral algebraic geometry: a type-theoretic formulation of Lurie's spectral algebraic geometry. The key objects — spectral Deligne-Mumford stacks, sheaves of $E_\infty$-rings — would be specific Segal types with structure.

*Moduli problems as covariant fibrations*: A moduli problem in algebraic geometry is a functor $M : \mathsf{Schemes}^{op} \to \mathsf{Groupoids}$ (or its ∞-categorical version $M : \mathsf{DerivedSchemes}^{op} \to \mathsf{Spaces}$). In STT, this is a covariant fibration over the Segal type of derived schemes. The representability theorem (Artin's representability theorem) becomes a statement about when such a fibration is representable by an ∞-groupoid.

## Application 6: Synthetic Higher Category Theory in Rzk — The Ongoing Project

**The context.** The Riehl-Shulman synthetic program is an ongoing research project, not a completed edifice. New results are being proved synthetically as the Rzk implementation matures.

**The application.** Current and near-future developments in the Rzk program:

*The comprehension theorem*: Every covariant fibration over a Segal type $A$ corresponds, via the Grothendieck construction, to a functor $A \to \mathsf{Type}$. This should be a synthetic theorem provable in Rzk. (In the classical setting, this is a theorem about quasi-categories requiring significant simplicial set machinery.)

*The Barwick-Schommer-Pries theorem*: The ∞-category of ∞-categories has a specific universal property. In STT, this would be a statement about the Segal type $\mathsf{Segal}$. The proof would use the Yoneda lemma and the theory of adjunctions.

*Synthetic ∞-operads*: ∞-Operads govern multiplicative algebraic structures (such as $E_n$-algebras). In STT, they should be Segal types with additional structure encoding the "colors" of inputs and outputs. A synthetic theory of ∞-operads would unlock ∞-algebra and higher string topology.

*Connections to cohesive type theory*: The interaction between simplicial type theory and cohesive HoTT (Chapter 25) is largely unexplored. Cohesion provides smooth structure; simplicial type theory provides categorical structure. A unified theory would support smooth ∞-categories — the setting for modern mathematical physics.

## Application 7: Categorical Semantics of Dependent Type Theories

**The context.** Dependent type theories (MLTT, HoTT, Cubical TT) have categorical semantics in specific ∞-categories — *models* of the type theory. The connection between the type theory and its model is the *initiality conjecture*: the syntactic category is the initial model.

**The application.** STT provides the correct framework for the categorical semantics of dependent type theories:

*Locally cartesian-closed ∞-categories as models of MLTT*: Martin-Löf type theory is modeled by locally cartesian-closed categories. In the ∞-categorical setting, the models should be locally cartesian-closed ∞-categories — and STT provides a native language for expressing and verifying this.

*The initiality conjecture*: The syntactic ∞-category of HoTT should be the initial model. Proving this requires a precise definition of "model" as a Segal type with additional structure, and a construction of the syntactic Segal type from the type theory. In STT, both can be stated clearly, and the proof is accessible (though non-trivial).

*Parametricity in STT*: The *Reynolds parametricity theorem* in programming languages theory says that polymorphic programs satisfy certain naturality conditions. In the ∞-categorical setting, parametricity corresponds to the Yoneda condition: polymorphic functions are natural transformations. STT's synthetic Yoneda lemma gives a new proof of parametricity, valid in the ∞-categorical setting.

The connection between STT, programming language theory, and category theory is one of the most fertile research directions currently open. The tools of Chapter 24 are the foundation for all of it.
