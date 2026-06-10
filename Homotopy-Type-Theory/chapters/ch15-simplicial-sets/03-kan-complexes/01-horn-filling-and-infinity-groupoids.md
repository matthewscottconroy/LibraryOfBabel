# 3.1 Kan Complexes: Horn Filling and ∞-Groupoids

## The Key Condition

We've defined simplicial sets as purely combinatorial objects: sets of simplices organized by face and degeneracy maps. But a general simplicial set is just an arbitrary combinatorial structure — it might look nothing like a topological space.

To get simplicial sets that *model* topological spaces, we need an additional condition. The condition is: **horns can be filled**.

**Definition 3.1 (Kan Complex).** A simplicial set $X$ is a *Kan complex* if for every $n \geq 1$ and $0 \leq k \leq n$, every horn inclusion admits an extension:

$$\text{Every map } \Lambda^n_k \to X \text{ extends to a map } \Delta[n] \to X$$

$$\begin{array}{ccc}
\Lambda^n_k & \hookrightarrow & \Delta[n] \\
\downarrow & \nearrow & \\
X & &
\end{array}$$

In words: given all but one face of an $n$-simplex (a horn), you can always fill in the missing face and the interior.

This single condition captures an enormous amount of structure. Let's see why.

## What Horn Filling Means Dimension by Dimension

**Dimension 1 ($n=1$):**
- $\Lambda^1_0$: the horn consisting of just the vertex $\{1\}$ (source missing). Filling gives an edge $0 \to 1$ for any target vertex $1$.
- $\Lambda^1_1$: the horn consisting of just $\{0\}$. Filling gives an edge for any source vertex.
- These are trivial: given any two vertices, fill with any edge between them.

**Dimension 2 ($n=2$):**
- $\Lambda^2_1$ (inner horn): Two edges $f : 0 \to 1$ and $g : 1 \to 2$ (composable). Filling gives a triangle, i.e., an edge $h : 0 \to 2$ and a 2-simplex witnessing that $h \simeq g \circ f$. **Filling $\Lambda^2_1$ means: composition exists.**

- $\Lambda^2_0$ (outer horn): Edges $f : 0 \to 1$ and $h : 0 \to 2$. Filling gives an edge $g : 1 \to 2$ such that $h \simeq g \circ f$, i.e., $g \simeq h \circ f^{-1}$. **Filling $\Lambda^2_0$ means: "right division" (composition with inverse) exists** — hence inverses exist.

- $\Lambda^2_2$ (outer horn): Edges $g : 1 \to 2$ and $h : 0 \to 2$. Filling gives $f : 0 \to 1$ with $h \simeq g \circ f$, i.e., $f \simeq g^{-1} \circ h$. **Filling $\Lambda^2_2$ means: "left division" exists** — also requiring inverses.

So: inner horn filling = composition; outer horn filling = inverses. Together, all horn filling = ∞-groupoid!

**Higher dimensions ($n \geq 3$):**
- Inner horns at higher dimensions give "higher composition" and coherence conditions
- Outer horns give "higher inverses" and their coherence
- Together: all the coherence conditions for an ∞-groupoid

## Why Kan Complexes are ∞-Groupoids

The following argument is informal but captures the key idea.

A Kan complex $X$ has:
- **Objects:** 0-simplices $x \in X_0$
- **Morphisms:** 1-simplices $f \in X_1$ with source $\partial_1 f$ and target $\partial_0 f$
- **Composition:** By $\Lambda^2_1$ horn filling: given $f : x \to y$ and $g : y \to z$, there's a 2-simplex with faces $f$, $g$, and a composite $h : x \to z$
- **Inverses:** By $\Lambda^2_0$ and $\Lambda^2_2$ filling: every morphism has an inverse

But composition is not unique (the horn-filling is not required to be unique). So we have a *space* of composites, and this space is contractible (any two composites are homotopic via higher horn filling). This is exactly the "composition up to contractible choice" of ∞-categories.

The higher simplices encode all the coherences:
- 2-simplices are "homotopies" (fillings witnessing that two composites agree)
- 3-simplices are "homotopies between homotopies" (coherences between composites)
- etc.

**Theorem 3.2 (Informal: Kan complexes = ∞-groupoids).** Kan complexes model ∞-groupoids. The correspondence:
- 0-simplices: objects
- 1-simplices: morphisms
- 2-simplices: homotopies between morphisms
- $n$-simplices: $n$-morphisms (all invertible, by outer horn filling)

## $\mathsf{Sing}(X)$ is a Kan Complex

The most important source of Kan complexes: singular complexes of topological spaces.

**Theorem 3.3.** For any topological space $X$, $\mathsf{Sing}(X)$ is a Kan complex.

*Proof.* We need to fill horns. A horn $\Lambda^n_k \to \mathsf{Sing}(X)$ is a collection of continuous maps from the faces of the geometric horn $|\Lambda^n_k|$ to $X$, compatible on their intersections.

Geometrically: $|\Lambda^n_k|$ is a deformation retract of $|\Delta^n|$ (the horn retracts to the filled simplex). So we can extend any map $|\Lambda^n_k| \to X$ to a map $|\Delta^n| \to X$ by composing with the retraction. (More precisely: the inclusion $|\Lambda^n_k| \hookrightarrow |\Delta^n|$ is a cofibration-acyclic, and by the HEP for ANR spaces, it has the extension property.) $\square$

This is why $\mathsf{Sing}(X)$ captures all the homotopy-theoretic information about $X$: it's a Kan complex with the same homotopy groups.

## Homotopy Groups of Kan Complexes

Since Kan complexes model topological spaces (in the homotopy-theoretic sense), they should have homotopy groups. Here's how to define them.

**Definition 3.4 (Homotopy Groups of a Kan Complex).** For a Kan complex $X$ with basepoint $x_0 \in X_0$:

**$\pi_0(X)$:** The set of connected components. Two 0-simplices $x, y$ are in the same component if there's a 1-simplex $f : x_0 \to y$ (more precisely: if they're connected by a sequence of 1-simplices).

**$\pi_1(X, x_0)$:** Homotopy classes of loops at $x_0$. A loop is a 1-simplex $f$ with $\partial_0 f = \partial_1 f = x_0$. Two loops $f, g$ are homotopic if there's a 2-simplex with faces $f, g$, and a degenerate edge (the constant loop). Composition: given loops $f$ and $g$, fill the horn $\Lambda^2_1$ (with $f$ and $g$ as two sides) to get a composite loop.

**$\pi_n(X, x_0)$:** Homotopy classes of maps $(\Delta[n], \partial\Delta[n]) \to (X, x_0)$ — maps sending the entire boundary $\partial\Delta[n]$ to $x_0$. With appropriate group structure from horn filling.

**Theorem 3.5.** For a topological space $Y$ with basepoint $y_0$:
$$\pi_n(\mathsf{Sing}(Y), y_0) \cong \pi_n(Y, y_0)$$

The homotopy groups of the Kan complex $\mathsf{Sing}(Y)$ equal the classical homotopy groups of $Y$.

## The Quillen Equivalence

The fundamental theorem connecting simplicial sets to topology:

**Theorem 3.6 (Quillen, 1967).** There is a Quillen equivalence between:
- The category of simplicial sets, with the Kan-Quillen model structure
- The category of topological spaces (with a suitable model structure)

The adjunction is $|{-}| \dashv \mathsf{Sing}$.

This is a precise statement: the two model categories are "homotopy equivalent" in the sense that the derived functors of $|{-}|$ and $\mathsf{Sing}$ are inverse equivalences of homotopy categories.

**Corollary:** For "nice" spaces $X$ (CW complexes), $|\mathsf{Sing}(X)| \simeq X$ (homotopy equivalence). For Kan complexes $K$, $\mathsf{Sing}(|K|) \simeq K$ (weak equivalence of simplicial sets).

**Interpretation:** Kan complexes and topological spaces are two different presentations of the same mathematical objects — homotopy types. You can freely translate between them.

## Weak Kan Complexes (Quasi-Categories)

Recall from Chapter 12: a quasi-category satisfies the inner horn filling condition but not necessarily the outer horn conditions.

**Inner horn filling:** $\Lambda^n_k \to X$ extends to $\Delta[n] \to X$ for all $0 < k < n$

**All horn filling (Kan):** $\Lambda^n_k \to X$ extends for all $0 \leq k \leq n$

The difference:
- Kan complex: all morphisms invertible (∞-groupoid)
- Quasi-category: only higher morphisms ($k \geq 2$) invertible ((∞,1)-category)

**Nerve characterization:**
- $N(\mathcal{C})$ is a Kan complex iff $\mathcal{C}$ is a groupoid
- $N(\mathcal{C})$ satisfies inner horn filling iff $\mathcal{C}$ is a category (inner horn filling uniquely means ordinary category)
- General quasi-categories generalize ordinary categories in the homotopy-coherent setting

## Kan Complexes in HoTT

The simplicial set model of HoTT (Voevodsky's model):
- Types are interpreted as Kan complexes
- The identity type $a =_A b$ is interpreted as a path in the Kan complex (a 1-simplex from $a$ to $b$)
- J rule = Kan horn filling
- Universes = specific Kan complexes satisfying univalence

The Kan condition is exactly the J rule! Here's why:

The J rule says: to prove $P(a, b, p)$ for all $b : A$ and $p : a = b$, it suffices to prove $P(a, a, \mathsf{refl}_a)$. Categorically, this is: any map from the basepoint $a$ (sitting at the "tip" of the horn $\Lambda^n_k$ in dimension 1) extends to a map from the full simplex.

More precisely:
- Inner horn filling in dimension 2 = composition of paths (the J rule used to define path concatenation)
- Outer horn filling = inverses of paths (the J rule used to define path inversion)
- Higher horn filling = coherences (the J rule used inductively)

So: Kan complexes model HoTT's identity type structure exactly, and the J rule is the combinatorial counterpart of the Kan horn-filling condition.

## Summary

| Property | Means | For ∞-groupoids |
|---|---|---|
| Inner $\Lambda^2_1$ fills | Composition exists | Path concatenation |
| Outer $\Lambda^2_0$ fills | Right inverse exists | Path inversion |
| Outer $\Lambda^2_2$ fills | Left inverse exists | Path inversion |
| Higher inner horns fill | Higher composition | Associativity, etc. |
| Higher outer horns fill | Higher inverses | Coherence |
| All horns fill (Kan) | Full ∞-groupoid structure | Identity types in HoTT |

Kan complexes are the combinatorial model of ∞-groupoids. The horn-filling condition encodes exactly the structure needed for identity types in HoTT: composition (J rule for concatenation), inversion (J rule for symmetry), and all higher coherences. This is why Voevodsky's model of HoTT uses Kan complexes.
