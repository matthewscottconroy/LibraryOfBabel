# Identity Types Categorically

## The Problem with Naive Interpretations

In a CCC or LCCC, there is a naive interpretation of the identity type: $a =_A b$ as the equalizer of the two projections from the "pair type" $A \times A$. But this gives the wrong thing.

The equalizer of $\pi_1, \pi_2 : A \times A \rightrightarrows A$ is the diagonal $\Delta(A) = \{(a,a) : a \in A\} \hookrightarrow A \times A$, with characteristic morphism $\chi_= : A \times A \to \Omega$ sending $(a, b)$ to $\top$ iff $a = b$. This is a *proposition* (in the set-theoretic sense): either $a = b$ or not. The identity type has at most one proof.

But this forces UIP: any two proofs of $a = b$ are equal (since there is at most one proof). The naive interpretation validates UIP, so it cannot model MLTT in the full HoTT sense — where the identity type can have multiple distinct proofs (paths of different lengths, windings, etc.).

The correct categorical interpretation requires *homotopical* structure: path objects, factorization systems, and ultimately the ∞-groupoid structure of types.

## Path Objects and Weak Factorization Systems

The Awodey-Warren approach (2009) models the identity type using *path objects*.

**Definition.** In a category $\mathcal{C}$ with a *weak factorization system* $(\mathcal{L}, \mathcal{R})$ (left maps and right maps), a *path object* for $A$ is an object $\mathsf{Path}(A)$ with a factorization of the diagonal:

$$A \xrightarrow{r} \mathsf{Path}(A) \xrightarrow{(s,t)} A \times A$$

where $r \in \mathcal{L}$ (an acyclic cofibration) and $(s,t) \in \mathcal{R}$ (a fibration).

Intuitively: $\mathsf{Path}(A)$ is the "space of paths in $A$." The map $r : A \to \mathsf{Path}(A)$ sends each point to the constant path at that point. The map $(s,t) : \mathsf{Path}(A) \to A \times A$ sends each path to its endpoints.

The identity type $a =_A b$ is then the *fiber* of $(s, t)$ over the point $(a, b) \in A \times A$: the type of paths from $a$ to $b$.

## The J Eliminator as a Lifting Property

The $J$ eliminator (path induction) in MLTT says: to prove a property $P$ about all paths, it suffices to prove $P$ about reflexivity paths.

Formally: given a type family $P : \prod_{x,y:A} (x =_A y) \to \mathcal{U}$ and a proof $d : \prod_{a:A} P(a, a, \mathsf{refl}_a)$, there is a term $J(P, d) : \prod_{x,y:A} \prod_{p:x=y} P(x, y, p)$.

Categorically, this is a *lifting property*: given the diagram

$$\begin{array}{ccc} A & \xrightarrow{d} & E \\ \downarrow_r & & \downarrow_q \\ \mathsf{Path}(A) & & B \end{array}$$

where $r : A \to \mathsf{Path}(A)$ is the reflexivity map (an acyclic cofibration in $\mathcal{L}$) and $q : E \to B$ is a fibration (in $\mathcal{R}$), the weak factorization system gives a diagonal filler: a morphism $J : \mathsf{Path}(A) \to E$ making both triangles commute.

The $J$ eliminator is this diagonal filler. The computation rule ($J(P, d)(a, a, \mathsf{refl}) = d(a)$) is the commutativity of the upper triangle.

**This explains why $J$ has the form it does:** the lifting property of a weak factorization system is exactly the statement of path induction.

## The Groupoid Model: UIP Fails

Hofmann and Streicher (1994) constructed the *groupoid model* of MLTT — the first model in which UIP fails.

**The model:** Interpret types as *groupoids* (categories where all morphisms are invertible). Interpret type families $B : A \to \mathcal{U}$ as *functors* $B : A \to \mathbf{Gpd}$ (groupoid-valued functors). Interpret terms $\Gamma \vdash t : A$ as *functors* from the groupoid $\llbracket \Gamma \rrbracket$ to the groupoid $\llbracket A \rrbracket$.

**Identity types:** The identity type $a =_A b$ is the *hom-set* $\mathsf{Hom}_A(a, b)$ — the set of morphisms from $a$ to $b$ in the groupoid $A$.

For a general groupoid, $\mathsf{Hom}_A(a, b)$ can have multiple elements. For example, if $A = \pi_1(S^1)$ (the fundamental groupoid of the circle), then $\mathsf{Hom}_A(\star, \star) = \mathbb{Z}$ (the integers, one for each winding number). So the identity type $\star =_{S^1} \star$ has $\mathbb{Z}$-many proofs.

**UIP fails:** In the groupoid model, UIP would say: any two elements of $\mathsf{Hom}_A(a, b)$ are equal. But $\mathsf{Hom}_A(\star, \star) = \mathbb{Z}$ has many distinct elements. So UIP is false in the groupoid model.

**MLTT holds:** All the MLTT rules (J eliminator, substitution, $\Pi$ and $\Sigma$ types) hold in the groupoid model. The groupoid model is a sound model of MLTT.

**The independence result:** Since UIP holds in the set model (where all identity types are trivial — at most one element) and fails in the groupoid model, UIP is *independent* of MLTT. It is neither provable nor refutable from the MLTT axioms alone.

This independence result was the catalyst for HoTT: if UIP is independent, then identity types carry more information than classical mathematics assumed. The "higher groupoid structure" of identity types is genuine mathematical content.

## The Simplicial Set Model: Univalence Holds

Voevodsky's simplicial set model (2006–2010) is the model in which the Univalence Axiom holds.

**The model:** Interpret types as *Kan complexes* (fibrant simplicial sets satisfying the horn-filling condition). Interpret type families as *Kan fibrations*. Interpret terms as *sections*.

**Identity types:** The identity type $a =_A b$ is the *space of paths* from $a$ to $b$ in the Kan complex $A$. A path is a map $\Delta[1] \to A$ (a 1-simplex) with boundary at $a$ and $b$.

For a general Kan complex, the space of paths $a =_A b$ is itself a Kan complex. Its elements are 1-simplices; its morphisms are 2-simplices (homotopies between paths); and so on. This gives the full ∞-groupoid structure.

**Univalence:** Voevodsky proved that in the simplicial set model, the natural map $(A = B) \to (A \simeq B)$ (sending equalities to equivalences) is an equivalence of Kan complexes. This is the Univalence Axiom.

The proof uses:
1. The universe $\mathcal{U}$ is a Kan complex (this requires fibrant replacement techniques)
2. The identity type of $\mathcal{U}$ at $(A, B)$ is the space of equivalences $A \simeq B$ (not just bijections)
3. The Kan filling conditions ensure all the required coherences hold

The simplicial set model proves: HoTT + Univalence is *consistent*. Voevodsky's proof transformed HoTT from a speculative proposal into a verified mathematical theory.

## The Tower of Models

The independence results define a tower of type theories and their models:

| Type Theory | Categorical Model | What's True | What Fails |
|---|---|---|---|
| MLTT | Sets | UIP, LEM (classically) | Univalence |
| MLTT | Groupoids | MLTT rules | UIP |
| MLTT + UIP | Sets | Classical math | HoTT |
| MLTT + Univalence | Kan simplicial sets | HoTT | (Unknown failures) |
| HoTT (MLTT + UA + HITs) | ∞-Toposes | Synthetic homotopy theory | — |

Each model reveals what is independent: the groupoid model shows UIP is independent, the set model shows univalence is independent, the simplicial set model shows HoTT is consistent.

## Consequences for HoTT

The categorical story of identity types has several direct consequences for HoTT:

1. **Proof relevance:** Identity proofs are not mere propositions but can carry genuine mathematical content (e.g., the winding number of a path in $S^1$). This is forced by the groupoid model.

2. **Higher structure:** The identity type of an identity type ($p =_{a=a'} q$ for paths $p, q$) models homotopies between paths. The full ∞-groupoid structure emerges from the iterated identity types.

3. **Univalence is essential:** The failure of univalence in the set model means that classical set-theoretic reasoning cannot detect equivalences that HoTT treats as equalities. Univalence is the axiom that forces the type-theoretic universe to behave like an ∞-groupoid of types.

4. **HITs are natural:** Higher inductive types are the colimit constructions in the ∞-topos structure. The path constructors in HITs (like $\mathsf{loop} : \mathsf{base} = \mathsf{base}$ in $S^1$) are elements of identity types — they are paths in the ∞-groupoid structure of the HIT.

The categorical perspective on identity types is not just a consistency check. It is the mathematical explanation of what HoTT is *about*.
