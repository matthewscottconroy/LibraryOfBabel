# Applications: Higher Category Theory

## 1. Topological Quantum Field Theory and the Cobordism Hypothesis

Physics provides one of the most striking applications of higher category theory. A topological quantum field theory (TQFT) in dimension $n$ assigns algebraic data to manifolds in a way that is compatible with cutting and gluing. The formal framework — due to Atiyah and Segal in the 1980s — defines a TQFT as a symmetric monoidal functor from the category of $n$-dimensional cobordisms to vector spaces.

But the fully extended version — the one that remembers all the structure of manifolds with corners — requires the full machinery of $(\infty, n)$-categories. Lurie's **cobordism hypothesis** (2009) gives the classification: the $(\infty, n)$-category of framed bordisms $\mathbf{Bord}_n^{fr}$ is the free symmetric monoidal $(\infty, n)$-category with duals on a single generator. A fully extended framed TQFT is exactly a choice of fully dualizable object in the target $(\infty, n)$-category.

The practical consequence is decisive. To specify a TQFT, you need only specify what happens to a single point. Everything else — the value on circles, tori, higher-dimensional manifolds — is determined by the duality structure of the single value at a point. This is a classification theorem of extraordinary power: the space of fully extended framed TQFTs valued in $\mathcal{C}$ is equivalent to the space of fully dualizable objects of $\mathcal{C}$.

In practice, this framework has been used to:
- Classify invertible TQFTs (relevant to condensed matter physics and topological phases of matter)
- Formalize the Chern-Simons theory of knot invariants as a 3-dimensional TQFT
- Understand the Witten-Reshetikhin-Turaev invariants of 3-manifolds in higher-categorical terms
- Connect $(\infty, 2)$-categorical structures to the geometry of 2d conformal field theories

The connection to HoTT is direct: the $(\infty, n)$-categories required for extended TQFT are the same structures being developed for the foundations of mathematics. The coherence data for a fully dualizable object — the duality data satisfying the swallowtail coherences — is exactly the kind of higher coherence structure that HoTT is designed to handle natively.

## 2. Derived Algebraic Geometry and ∞-Stacks

Classical algebraic geometry studies schemes — spaces locally modeled on commutative rings. The Grothendieck revolution in the 1960s showed that the "points" of a scheme are really functors: a scheme $X$ is determined by its functor of points $\mathsf{Hom}(-, X) : \mathbf{Ring}^{op} \to \mathbf{Set}$.

Derived algebraic geometry replaces rings with differential graded commutative rings (or simplicial commutative rings) and sets with ∞-groupoids. The derived functor of points sends a derived ring to an ∞-groupoid of solutions, not just a set. The effect is that intersection multiplicities, deformation spaces, and obstruction theories — which are messy in classical geometry — become clean and functorial in the derived setting.

Lurie's program, developed in his thesis (2004) and formalized in *Spectral Algebraic Geometry* (2018), built the foundations of derived algebraic geometry using ∞-categorical methods. The key constructions:

- **Derived schemes**: locally modeled on simplicial commutative rings; the derived tensor product $A \otimes_B^L C$ is computed by a homotopy colimit, not a strict pushout
- **∞-stacks**: functors $(\mathbf{dRing}^{op})^{\simeq} \to \mathcal{S}$ satisfying descent conditions; the moduli stack of elliptic curves becomes a genuine geometric object, not just a quotient
- **Deformation theory**: the cotangent complex $\mathbb{L}_{A/B}$ classifies deformations; in the derived setting, it is a module in the ∞-category of $A$-modules, not just a chain complex

The application to concrete mathematics is substantial. The proof of the Weil conjectures (Deligne, 1974) can be reformulated as a statement about $\ell$-adic sheaves on derived schemes over finite fields. The geometric Langlands program — relating automorphic representations to geometric objects on moduli spaces of bundles — uses ∞-categorical derived geometry as its natural language.

The connection to HoTT: the types in HoTT are ∞-groupoids, and the universe $\mathcal{U}$ is an ∞-stack classifier. Running HoTT "in" a sheaf ∞-topos on a site of derived rings gives internal derived algebraic geometry — the dream of doing algebraic geometry synthetically in a type-theoretic proof assistant.

## 3. Chromatic Homotopy Theory and Structured Ring Spectra

Stable homotopy theory — the study of spectra and their homotopy groups — is the natural home of chromatic homotopy theory. The chromatic filtration organizes stable homotopy groups of spheres by "height" $n$, with the height-0 part being rational homotopy theory, height-1 corresponding to $K$-theory and $J$-theory, and height $n$ governed by the Morava $K$-theories $K(n)$.

The higher-categorical framework is essential for making the chromatic perspective precise. The $(\infty, 1)$-category of spectra $\mathbf{Sp}$ is symmetric monoidal under the smash product $\wedge$. The category of $\mathbb{E}_\infty$-ring spectra — spectra with a coherently homotopy-commutative and associative multiplication — is the $(\infty, 1)$-category of commutative monoids in $\mathbf{Sp}$.

The definition of $\mathbb{E}_\infty$ requires ∞-categorical machinery: an $\mathbb{E}_\infty$-ring is an algebra for the $\mathbb{E}_\infty$-operad in $\mathbf{Sp}$, where the operad is an ∞-operad in Lurie's sense. This is not a property of a spectrum but a structure, and the space of $\mathbb{E}_\infty$-ring structures on a given spectrum can be non-trivial.

Concrete results that depend on this framework:
- The **Goerss-Hopkins-Miller theorem**: the Morava $E$-theories $E_n$ carry a canonical $\mathbb{E}_\infty$-ring structure, functorial in the formal group; this is the foundational result underlying the chromatic splitting conjecture
- **Topological modular forms (tmf)**: a global section object in the ∞-categorical sheaf of $\mathbb{E}_\infty$-rings on the moduli stack of elliptic curves; its homotopy groups compute exotic torsion in stable homotopy theory
- **The telescope conjecture**: recently resolved (Burklund-Hahn-Levy-Schlank, 2023), with the disproof using ∞-categorical methods for controlling the chromatic filtration at height 2

For HoTT and formal verification: several computations in chromatic homotopy theory have been formalized or partially formalized using Agda with the HoTT library. The ∞-categorical coherence machinery needed for $\mathbb{E}_\infty$-rings — which is difficult to handle in classical set theory because of the proliferating coherence conditions — becomes manageable in HoTT because coherence data lives in contractible spaces.

## 4. Formal Verification of Algebraic Topology in Proof Assistants

One of the most direct applications of higher category theory to computer science is the formalization of algebraic topology in proof assistants. The HoTT Book (2013) demonstrated that homotopy theory is not just analogous to type theory — the two are literally the same thing, interpreted correctly. This opened the possibility of machine-verified proofs of topological theorems.

**Cubical Agda** (using the CCHM cubical type theory model) has enabled:
- Formal verification of $\pi_1(S^1) = \mathbb{Z}$ (the encode-decode proof, using the winding number fiber sequence)
- Verification of the Blakers-Massey connectivity theorem (Favonia, Finster, Lumsdaine, Licata)
- Verification of the freudenthal suspension theorem $\pi_n(S^n) = \mathbb{Z}$
- Verification of the Seifert-van Kampen theorem for fundamental groupoids
- Computation of $\pi_4(S^3) = \mathbb{Z}/2$ (using the Brunerie number proof, with computational verification in cubical Agda)

The **Lean 4 mathlib** project has increasingly incorporated ∞-categorical language, with work on formalizing the theory of model categories, simplicial homotopy theory, and the Dold-Kan correspondence.

The technical reason higher category theory matters here: the proof of topological facts often requires juggling coherence conditions that proliferate in homotopy theory. In a set-theoretic proof assistant, you need to track these coherences by hand. In HoTT/cubical type theory, the contractibility of the space of coherences means that the proof assistant handles them automatically — any two coherence witnesses are definitionally equal (or propositionally equal in a type that you don't need to unpack).

The long-term vision: a library of formalized algebraic topology, from basic homotopy groups through the Adams spectral sequence, all machine-verified. The higher-categorical foundation makes this feasible in a way that set-theoretic foundations do not.

## 5. Condensed Mathematics and the Clausen-Scholze Program

In 2019, Clausen and Scholze introduced *condensed mathematics* — a new foundation for functional analysis and $p$-adic geometry that resolves long-standing problems with the interplay between algebra and topology.

The classical problem: the category of topological abelian groups is not abelian (the cokernel of a morphism of topological groups is not well-behaved). This failure prevents the standard tools of homological algebra (derived categories, Ext groups) from being applied directly. Condensed mathematics fixes this by replacing topological spaces with *condensed sets* — sheaves of sets on the site of profinite sets with the coherent topology.

The condensed category $\mathbf{Cond}(\mathbf{Ab})$ of condensed abelian groups is an abelian category with enough projectives. Scholze's *analytic geometry* program then constructs a subcategory of "solid" condensed modules and "liquid" vector spaces, allowing the construction of derived categories of $p$-adic modules with the right mapping properties.

The ∞-categorical structure is essential:
- The derived ∞-category $D(\mathbf{Cond}(\mathbf{Ab}))$ is the natural home for the derived functors of condensed homological algebra
- The $\mathbb{E}_1$-ring structure of condensed $\mathbb{Z}_p$ (the $p$-adic integers) is the right framework for $p$-adic Hodge theory
- The theory of *pyknotic objects* (Barwick-Haine) gives an ∞-categorical alternative to condensed sets, making the ∞-categorical structure more explicit

The connection to HoTT: condensed mathematics can be formalized internally in a sheaf ∞-topos on the site of profinite sets. Running HoTT in this ∞-topos gives a synthetic account of condensed mathematics, where the analytic structure (the profinite topology on the site) is encoded in the ∞-topos rather than added as external data.

Scholze explicitly cited higher categorical methods in his 2022 Fields Medal work (on $p$-adic Langlands and perfectoid spaces), with condensed mathematics as the unifying framework.

## 6. K-Theory, Traces, and the Dennis Trace

Algebraic K-theory is the study of the functors $K_n : \mathbf{Ring} \to \mathbf{Ab}$ that generalize projective modules ($K_0$), automorphisms of free modules ($K_1$), and Milnor K-theory ($K_2$). Quillen's higher K-groups are defined via the plus construction or the Q-construction, but the ∞-categorical perspective (due to Barwick, 2016) is more fundamental: $K$-theory is a functor from the $(\infty, 1)$-category of stable ∞-categories to spectra.

The **Dennis trace** is a natural map $K(A) \to \mathsf{THH}(A)$ from algebraic K-theory to topological Hochschild homology. Its factorizations — through topological cyclic homology $\mathsf{TC}(A)$ and topological periodic cyclic homology $\mathsf{TP}(A)$ — are the trace methods used to compute K-groups.

The ∞-categorical framework (Nikolaus-Scholze, 2018) gave a clean construction of $\mathsf{TC}(A)$ as the fixed points of the $S^1$-action on $\mathsf{THH}(A)$, where $S^1$ acts via the cyclotomic structure. The crucial step uses the ∞-categorical characterization of $\mathsf{THH}(A)$ as the geometric realization of the cyclic bar construction — a colimit in the ∞-categorical sense.

Concrete computations enabled by this approach:
- $K(\mathbb{Z}) = \{0, \mathbb{Z}, \mathbb{Z}/2, \mathbb{Z}/48, 0, 0, \mathbb{Z}, \mathbb{Z}/240, \ldots\}$ (computed partially using trace methods)
- $K(\mathbb{F}_q)$ for finite fields (computed via the trace map to $\mathsf{THH}$, which is computable by Bökstedt's theorem)
- $K(ku)$ and $K(KU)$ for complex K-theory spectra (Ausoni-Rognes program, using trace methods with structured ring spectra)

The connection to HoTT: the cyclic bar construction that defines $\mathsf{THH}$ is a colimit over the cyclic category $\Lambda$. In HoTT, this colimit can be computed as a higher inductive type — the cyclic bar construction is a HIT with generators corresponding to the simplices of the bar construction and the cyclic symmetry. This gives a synthetic formulation of topological Hochschild homology that is directly formalizable in a proof assistant.

## 7. ∞-Categorical Methods in the Langlands Program

The Langlands program — connecting automorphic representations of reductive groups to Galois representations — is one of the deepest programs in contemporary mathematics. Its geometric version (the geometric Langlands correspondence) has been recast in higher-categorical terms, making it more tractable and revealing new structure.

The **geometric Langlands conjecture** (proved by Gaitsgory and collaborators, 2024, in a series of papers totaling thousands of pages) states: for a smooth projective curve $X$ over $\mathbb{C}$ and a reductive group $G$, there is an equivalence of $(\infty, 1)$-categories:

$$\mathsf{D\text{-}mod}(\mathsf{Bun}_G) \simeq \mathsf{IndCoh}_\mathsf{Nilp}(\mathsf{LocSys}_{\check{G}})$$

where:
- $\mathsf{Bun}_G$ is the moduli stack of $G$-bundles on $X$
- $\mathsf{LocSys}_{\check{G}}$ is the moduli stack of $\check{G}$-local systems (Langlands dual)
- $\mathsf{D\text{-}mod}$ denotes the $(\infty, 1)$-category of D-modules
- $\mathsf{IndCoh}_\mathsf{Nilp}$ denotes ind-coherent sheaves with nilpotent singular support

The proof uses the full machinery of ∞-categorical derived algebraic geometry: $(\infty, 1)$-categories of sheaves on derived stacks, adjoint functor theorems in the ∞-categorical setting, and ∞-categorical descent for sheaves.

The reason ∞-categories are necessary: the moduli stacks involved ($\mathsf{Bun}_G$, $\mathsf{LocSys}_{\check{G}}$) are not classical stacks but derived stacks, and the functors between them are not exact functors between abelian categories but left-exact or right-exact functors between stable ∞-categories. The coherence of the equivalence — that both sides are compatible with all the additional structure (factorization algebras, Hecke correspondences, spectral decomposition) — is exactly the kind of multi-level coherence that ∞-category theory handles natively.

For HoTT: the ∞-toposes appearing in geometric Langlands (the ∞-topos of sheaves on $\mathsf{Bun}_G$) are exactly the kind of structure for which HoTT was designed to be the internal language. A long-term goal is to formalize the geometric Langlands correspondence synthetically — stating the equivalence as an internal statement in an appropriate ∞-topos and verifying it in a proof assistant. This goal is currently out of reach, but the ∞-categorical foundations are the bridge between the mathematics and its eventual formalization.
