# Voevodsky's Simplicial Set Model

## The Problem

Homotopy Type Theory is a formal system with rules for forming types, terms, and proofs. But formal systems must be shown to be consistent — to not prove contradictions — before they can be trusted. And consistency proofs require models: mathematical structures satisfying all the axioms of the system.

For classical type theory (Martin-Löf type theory), models are relatively well understood. But HoTT includes the *univalence axiom* — which is not a theorem of classical type theory — and this axiom needed to be shown consistent. Voevodsky's simplicial set model provides this consistency proof.

The model says: there is a mathematical structure (built out of simplicial sets, which live in ordinary set theory ZFC) in which all the axioms of HoTT hold, including univalence. Since ZFC is (assumed to be) consistent, HoTT is consistent relative to ZFC.

## Types as Kan Complexes

In the simplicial set model:
- A **type** $A$ is a (small) Kan complex.
- A **term** $a : A$ is a vertex of $A$ — an element of $A_0$.
- A **type family** $P : A \to \mathcal{U}$ (a dependent type) is a Kan fibration $p : E \to A$, where the fiber $p^{-1}(a)$ over each vertex $a$ is the Kan complex corresponding to the type $P(a)$.
- The **universe** $\mathcal{U}$ is the Kan complex of (small) Kan complexes.

Let's check each type former:

**$\Sigma$-types.** The type $\sum_{a:A} P(a)$ corresponds to the total space $E = \{(a, e) : a \in A_0, e \in P(a)_0\}$, which is the Kan complex with $n$-simplices being pairs $(\sigma, \tau)$ where $\sigma \in A_n$ and $\tau \in P(\sigma)_n$. The Kan condition on $E$ follows from the fact that $p : E \to A$ is a Kan fibration and $A$ is a Kan complex.

**$\Pi$-types.** The type $\prod_{a:A} P(a)$ corresponds to the Kan complex of *sections* of the fibration $p : E \to A$ — maps $s : A \to E$ with $p \circ s = \mathsf{id}_A$. The section space of a Kan fibration over a Kan complex is a Kan complex.

**Identity types.** The type $a =_A b$ (for $a, b : A$) corresponds to the *path space* Kan complex: the simplicial set whose $n$-simplices are $(n+1)$-simplices of $A$ from $a$ to $b$. This is the fiber of the diagonal map $A \to A \times A$ at the pair $(a, b)$. The path space is always a Kan complex (the diagonal of a Kan complex is a Kan fibration).

**The reflexivity term.** The term $\mathsf{refl}_a : a =_A a$ corresponds to the degenerate simplex $s_0 a \in A_1$ — the degenerate edge at $a$, which is the "constant path" at $a$.

**Path induction (J eliminator).** Given $a : A$ and $P : \prod_{b:A} (a =_A b) \to \mathcal{U}$ and $d : P(a, \mathsf{refl}_a)$, path induction produces a term $\mathsf{J}(a, P, d) : \prod_{b:A} \prod_{p : a=b} P(b, p)$. In the simplicial model, this corresponds to the extension property of the Kan fibration representing $P$ along the inclusion of the vertex $a$ into the path space. The Kan condition ensures such extensions exist.

## The Universe

The universe $\mathcal{U}$ is the Kan complex whose:
- $n$-simplices are families of small Kan complexes parametrized by $\Delta[n]$, i.e., Kan fibrations over $\Delta[n]$.
- Face and degeneracy maps are the obvious pullbacks.

This is the *universal Kan fibration*: the "space of Kan complexes" and their equivalences. Its $0$-simplices are Kan complexes, its $1$-simplices are maps between Kan complexes, and higher simplices are higher homotopies between maps.

**Key property:** $\mathcal{U}$ is itself a Kan complex. This is non-trivial: it requires showing that the horn-filling condition holds for families of Kan complexes parametrized by horns, and that these fillings can be chosen to produce Kan complexes.

The fact that $\mathcal{U}$ is a Kan complex means: the universe itself is a homotopy type. Paths in $\mathcal{U}$ are homotopy equivalences between Kan complexes.

## The Univalence Axiom

The univalence axiom states:
$$\mathsf{ua} : (A \simeq B) \simeq (A =_{\mathcal{U}} B)$$

In words: the type of equivalences between $A$ and $B$ is equivalent to the type of identities (paths) from $A$ to $B$ in the universe.

In the simplicial model, this means:
- A path from $A$ to $B$ in $\mathcal{U}$ is a 1-simplex of $\mathcal{U}$, which is a Kan fibration over $\Delta[1]$ with fiber $A$ over vertex $0$ and fiber $B$ over vertex $1$. In other words, a path from $A$ to $B$ in $\mathcal{U}$ is a Kan fibration whose fibers are $A$ and $B$.
- A homotopy equivalence $f : A \simeq B$ between Kan complexes is a weak homotopy equivalence (by the Quillen model structure, this is the same as a weak equivalence in $\mathbf{sSet}$).

**Theorem (Univalence in the simplicial model).** A path in $\mathcal{U}$ from $A$ to $B$ corresponds to a homotopy equivalence from $A$ to $B$.

*Proof sketch.* A path from $A$ to $B$ in $\mathcal{U}$ is a Kan fibration $p : E \to \Delta[1]$ with $p^{-1}(0) = A$ and $p^{-1}(1) = B$. By the theory of Kan fibrations, the inclusion of the fiber $A \hookrightarrow E$ is a weak homotopy equivalence (the fiber inclusion into a path object is always a weak equivalence). Composing with the weak equivalence $E \to B$ (by pulling back along the other vertex), we get a weak equivalence $A \to B$. Conversely, given a weak equivalence $f : A \to B$, we can form the *mapping cylinder* (a Kan fibration over $\Delta[1]$ with fibers $A$ and $B$), giving a path in $\mathcal{U}$.

The precise verification uses the Quillen model structure and the fact that the model structure is compatible with the universe structure. The key technical result is that the "straightening/unstraightening" construction of Lurie makes the correspondence between Kan fibrations over $\Delta[1]$ and maps between their fibers an equivalence.

## What the Model Tells Us

The Voevodsky simplicial set model is more than a consistency proof. It tells us *what HoTT is about*:

**1. Types have homotopy types.** Every type $A$ in HoTT is, in the simplicial model, a Kan complex — a homotopy type. The identity types $a =_A b$ are path spaces; higher identity types are spaces of homotopies. This is why types behave like spaces.

**2. HoTT is the internal language of an $\infty$-topos.** The simplicial set model is an $\infty$-topos, and HoTT is its internal language. Every construction in HoTT has a geometric interpretation in the $\infty$-topos of simplicial sets, and every geometric construction in the $\infty$-topos can be expressed in HoTT.

**3. Univalence is a theorem of homotopy theory.** The univalence axiom, added to Martin-Löf type theory as an axiom, is actually a theorem in the simplicial model — it follows from the theory of Kan fibrations. This suggests that in the "right" type theory (one that builds in univalence at a deeper level), univalence might not be an axiom at all, but a consequence. This is the motivation for cubical type theory.

**4. Classical homotopy theory is a subset of HoTT.** Every theorem of classical homotopy theory that can be stated in HoTT has a proof in HoTT (though the proof may need to be synthetic). The simplicial model ensures that the synthetic statements are correct — they reflect genuine facts about homotopy types.

## The Simplicial Set Model vs. Cubical Set Model

Voevodsky's simplicial set model shows HoTT is consistent, but it does not give a *computational* model — one where you can run computations on terms. The reason: in the simplicial set model, the univalence axiom is an axiom with no computational reduction rule.

The *cubical set model* (Bezem-Coquand-Huber, 2014; Cohen-Coquand-Huber-Mörtberg, 2016) provides an alternative model built out of *cubical sets* (presheaves on a category of cubes rather than simplices). In the cubical model:
- The univalence axiom has a direct computational interpretation.
- Identity types have a direct cubical description (as "path types" — dependent types over the interval $I = [0,1]$ as an abstract interval object).
- The model gives rise to *Cubical Agda* and *HoTT-Agda*, proof assistants where computations can actually be run.

The cubical model is now preferred for computational purposes; the simplicial model remains fundamental for the mathematical theory, as it connects HoTT to the classical theory of simplicial sets and $\infty$-toposes.

## Summary: The Three-World Triangle

| Construction | Topology | Simplicial Sets | HoTT |
|---|---|---|---|
| A space/type | Topological space | Kan complex | Type |
| A point | Point of a space | Vertex ($0$-simplex) | Term |
| A path from $a$ to $b$ | Continuous path $[0,1] \to X$ | 1-simplex from $a$ to $b$ | Term of $a =_A b$ |
| A homotopy | Homotopy $H : [0,1]^2 \to X$ | 2-simplex | Term of $p =_{a=b} q$ |
| A fibration | Serre fibration | Kan fibration | Dependent type family |
| The fiber | Fiber of a fibration | Fiber Kan complex | The type $P(a)$ |
| Equivalence | Homotopy equivalence | Weak equivalence | Term of $A \simeq B$ |
| Identity of types | Homeomorphism (wrong!) | Homotopy equivalence | Term of $A = B$ in $\mathcal{U}$ |
| Univalence | (no classical analogue) | Paths in the universe = equivalences | $A \simeq B \simeq A =_{\mathcal{U}} B$ |
