# Chapter 15: Simplicial Sets

Vladimir Voevodsky spent much of his career working on motivic cohomology, a subject so technical that fewer than a hundred people in the world understood it fully. In 2006, he discovered a serious error in a proof he had published. Not a fixable error — a fundamental one. He concluded that mathematics needed a new foundation: one where proofs could be machine-verified, where the computer could check what human experts had missed. The foundation he chose was Homotopy Type Theory, and the model he used to show it was consistent was built out of simplicial sets. To understand why simplicial sets work — why they provide a model of HoTT, why Voevodsky could show that the univalence axiom holds in this model, why the proof is convincing — you have to understand what simplicial sets are.

## The Combinatorial Revolution

Topology is built on continuous spaces: sets with topological structure, points, and the continuous maps between them. This is powerful but also technically demanding. Every argument requires tracking convergence, continuity, and the topology of the spaces involved.

Homotopy theory, by contrast, only cares about spaces up to homotopy equivalence. Once you decide to work up to homotopy equivalence, you might ask: do you really need the full machinery of continuous spaces? Or is there a purely combinatorial — algebraic, discrete, finitary — way to capture the homotopy-theoretic information?

The answer is yes, and the objects that do the job are simplicial sets.

A simplicial set is a purely combinatorial object: a collection of "simplices" (vertices, edges, triangles, tetrahedra, and their higher-dimensional analogs) organized by *face maps* (telling you which face of a simplex is which) and *degeneracy maps* (allowing you to view a lower-dimensional simplex as a degenerate higher-dimensional one). No metric. No topology. No continuity. Just sets and maps between sets, organized in a pattern dictated by the simplex category $\Delta$.

And yet — the key theorem — certain simplicial sets (the *Kan complexes*) encode exactly the same homotopy-theoretic information as topological spaces. Quillen proved in 1967 that the homotopy theory of Kan complexes is equivalent to the homotopy theory of topological spaces. You can do all of homotopy theory without ever leaving the combinatorial world.

## Why This Matters for HoTT

Voevodsky's consistency model for HoTT is built in this combinatorial world:
- **Types** are Kan complexes.
- **Terms** are vertices of Kan complexes.
- **Identity types** $a =_A b$ are the path spaces of Kan complexes: the Kan complex of paths from $a$ to $b$.
- **The universe** $\mathcal{U}$ is a Kan complex whose vertices are (small) Kan complexes.
- **Univalence** holds because paths in the universe correspond to homotopy equivalences of Kan complexes.

This model is the mathematical proof that HoTT is consistent relative to ZFC (the standard set-theoretic foundation). If ZFC has a model (which it does, assuming ZFC is consistent — something we cannot prove within ZFC but have good reason to believe), then so does HoTT, with all its type-theoretic structure including the univalence axiom.

The simplicial set model is also the explanation of *why* HoTT's type theory behaves the way it does. The path type $a =_A b$ is a space of paths because it interprets as the path space of a Kan complex. The identity type is not just a proposition (a flat set) but a type with its own identity types — because path spaces have their own path spaces, all the way up. This is the $\infty$-groupoid structure of types, and it arises from the simplicial structure.

## What This Chapter Covers

**Section 1 (The Simplex Category)** introduces $\Delta$: the category whose objects are finite ordered sets $[n] = \{0, 1, \ldots, n\}$ and whose morphisms are order-preserving maps. The face maps (injections missing one element) and degeneracy maps (surjections doubling one element) generate all morphisms and satisfy the simplicial identities. The simplex category is the combinatorial skeleton of all simplicial geometry.

**Section 2 (Simplicial Sets)** defines simplicial sets as functors $\Delta^{op} \to \mathbf{Set}$. The standard simplex $\Delta[n] = \text{Hom}_\Delta(-, [n])$ is the representable functor. The geometric realization $|X|$ takes a simplicial set to a topological space; the singular complex $\text{Sing}(Y)$ takes a topological space to a simplicial set. These functors form an adjunction $|-| \dashv \text{Sing}$.

**Section 3 (Kan Complexes)** introduces the horn-filling condition: every horn $\Lambda^n_k \to X$ extends to a simplex $\Delta[n] \to X$. Kan complexes are exactly the simplicial sets satisfying this condition, and they are exactly the combinatorial models of $\infty$-groupoids (spaces where all morphisms at all levels are invertible). The singular complex of any topological space is a Kan complex.

**Section 4 (The Model Structure)** develops the Quillen model structure on simplicial sets: cofibrations are monomorphisms, fibrations are Kan fibrations (satisfying the horn-filling condition on morphisms), and weak equivalences are maps inducing isomorphisms on all homotopy groups. Quillen proved this model structure is equivalent to the standard model structure on topological spaces.

**Section 5 (The Voevodsky Model)** explains Voevodsky's simplicial set model of HoTT, showing how each type-theoretic construction (Π-types, Σ-types, identity types, universes) corresponds to a simplicial construction, and why the univalence axiom holds.

## The Three-World Picture

The deepest insight of simplicial homotopy theory is that three worlds — topological spaces, simplicial sets, and types in HoTT — all carry equivalent homotopy-theoretic information:

$$\text{Topological Spaces} \underset{\text{Quillen equivalence}}{\longleftrightarrow} \text{Simplicial Sets (Kan complexes)} \underset{\text{Voevodsky model}}{\longleftrightarrow} \text{Types in HoTT}$$

The left equivalence (Quillen, 1967) allows you to translate freely between continuous spaces and combinatorial simplicial sets, preserving all homotopy-theoretic information. The right equivalence (Voevodsky, 2009) shows that HoTT's types capture exactly the homotopy types of spaces, and that HoTT's axioms (including univalence) are true in the simplicial set model.

Together: HoTT is an axiomatic system whose models are homotopy types. Every theorem of HoTT is a theorem about homotopy types of spaces — and every homotopy-theoretic theorem that can be stated in HoTT has a proof in HoTT. This is what it means for HoTT to be the internal language of $\infty$-toposes.
