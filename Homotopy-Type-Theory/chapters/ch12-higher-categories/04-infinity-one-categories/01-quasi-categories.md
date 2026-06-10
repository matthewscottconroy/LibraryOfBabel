# 4.1 (∞,1)-Categories and Quasi-Categories

## The Need for (∞,1)-Categories

∞-Groupoids capture homotopy types — spaces where you can go "backwards" along any path. But many mathematical structures have directed morphisms that aren't invertible.

Think about the category of chain complexes and chain maps. There's a sensible notion of "homotopy" between chain maps (a chain homotopy), and you can compose chain maps in a homotopy-coherent way. But chain maps aren't invertible in general. So the structure is not an ∞-groupoid but something more general: an ∞-category where 1-morphisms might not be invertible.

More specifically, we want: an **(∞,1)-category** — an ∞-category in which every $k$-morphism for $k \geq 2$ is invertible, but 1-morphisms may not be. The idea is:
- 0-cells: objects
- 1-cells: morphisms (not necessarily invertible)
- 2-cells: homotopies between morphisms (invertible)
- 3-cells: homotopies between homotopies (invertible)
- ...all higher cells invertible

This is the right generalization of ordinary categories to the homotopy-coherent setting.

Key examples:
- The (∞,1)-category of spaces (objects = spaces, morphisms = continuous maps, higher morphisms = homotopies, homotopies of homotopies, ...)
- The (∞,1)-category of chain complexes
- The (∞,1)-category of $E_\infty$-ring spectra
- Any ordinary category (by treating all hom-sets as discrete ∞-groupoids)

## Multiple Models

Just as there are multiple models of ∞-groupoids, there are multiple models of (∞,1)-categories:
1. **Quasi-categories** (Joyal, Lurie) — simplicial sets with inner horn filling
2. **Complete Segal spaces** (Rezk) — simplicial spaces satisfying Segal and completeness conditions
3. **Segal categories** — simplicial spaces where the 0th space is discrete
4. **Relative categories** — categories with a class of "weak equivalences"
5. **$A_\infty$-categories** — categories enriched over chain complexes (for linear settings)

All of these are equivalent (in the appropriate homotopy-theoretic sense), proven by various people over the past 30 years. We'll focus on quasi-categories, which are the most tractable for our purposes.

## Quasi-Categories: Definition

Recall that a simplicial set $X$ is:
- A collection of sets $X_0, X_1, X_2, \ldots$ (the $n$-simplices)
- Face maps $d_i : X_n \to X_{n-1}$ for $0 \leq i \leq n$
- Degeneracy maps $s_i : X_n \to X_{n+1}$ for $0 \leq i \leq n$

satisfying simplicial identities.

A **horn** $\Lambda^n_k$ is the "$(n-1)$-skeleton of $\Delta^n$ minus the $k$-th face" — the simplicial set obtained by taking all faces of the $n$-simplex except the interior and the $k$-th face. There are three types:
- **Outer horn $\Lambda^n_0$:** remove the 0th face (and the interior)
- **Outer horn $\Lambda^n_n$:** remove the $n$th face
- **Inner horn $\Lambda^n_k$ for $0 < k < n$:** remove an interior face

**Definition 4.1 (Quasi-Category).** A simplicial set $X$ is a *quasi-category* (or *weak Kan complex*, or *inner Kan complex*) if it has the **inner horn filling property**: for every $n \geq 2$ and every $0 < k < n$, every map $\Lambda^n_k \to X$ extends to a map $\Delta^n \to X$.

$$\begin{array}{ccc}
\Lambda^n_k & \to & X \\
\downarrow & \nearrow & \\
\Delta^n & &
\end{array}$$

Compare with Kan complexes: a Kan complex fills *all* horns (inner and outer). A quasi-category only fills inner horns.

The outer horns correspond to inverses. A Kan complex has inverses for all morphisms (all outer horns fill). A quasi-category only requires composition (inner horn filling) — morphisms need not be invertible.

## The Meaning of Inner Horn Filling

Let's think about what $\Lambda^2_1 \to X$ means explicitly.

$\Delta^2$ is the standard 2-simplex (a triangle). Its vertices are $0, 1, 2$. Its edges are $01, 12, 02$. Its interior is the unique 2-simplex $012$.

$\Lambda^2_1$ is $\Delta^2$ minus the interior and minus the 1st face (which is the edge $02$ — the edge opposite vertex 1).

So $\Lambda^2_1$ consists of: vertices $0, 1, 2$ and edges $01$ and $12$ (but *not* edge $02$ or the interior).

A map $\Lambda^2_1 \to X$ gives:
- A vertex $x_0 = f(0)$
- A vertex $x_1 = f(1)$
- A vertex $x_2 = f(2)$
- A 1-simplex $f_{01} : x_0 \to x_1$ (a morphism from $x_0$ to $x_1$)
- A 1-simplex $f_{12} : x_1 \to x_2$ (a morphism from $x_1$ to $x_2$)

In other words: two composable morphisms $x_0 \xrightarrow{f_{01}} x_1 \xrightarrow{f_{12}} x_2$.

The inner horn filling condition says: this extends to a full 2-simplex $\Delta^2 \to X$. This 2-simplex has:
- The same vertices and edges $f_{01}, f_{12}$
- An edge $f_{02} : x_0 \to x_2$ (the composite)
- A 2-simplex (witnessing the composition)

So filling the inner horn $\Lambda^2_1$ is exactly *providing a composite* of $f_{01}$ and $f_{12}$.

The composite is not unique on the nose — there may be many ways to fill the horn. But in a quasi-category, the space of fillings is *contractible* (there's essentially one composite up to homotopy). This is what "composition up to contractible choice" means.

## Objects, Morphisms, and Composition

In a quasi-category $X$:

**Objects:** 0-simplices, elements of $X_0$.

**Morphisms:** 1-simplices $f \in X_1$, with source $d_1(f) \in X_0$ and target $d_0(f) \in X_0$. We write $f : a \to b$ where $a = d_1(f)$ and $b = d_0(f)$.

**Identity morphisms:** For any object $a \in X_0$, the degenerate 1-simplex $s_0(a) \in X_1$ is the identity morphism $\mathsf{id}_a : a \to a$.

**Composition:** Given composable morphisms $f : a \to b$ and $g : b \to c$, fill the horn $\Lambda^2_1 \to X$ (given by $f$ and $g$) to get a 2-simplex. The remaining face is the composite $g \circ f : a \to c$.

The composite is *not unique* — there may be many fillings. But they're all homotopic (connected by 2-simplices), so up to homotopy, composition is well-defined.

**2-Morphisms:** 2-simplices $\sigma \in X_2$ with boundary edges $d_2(\sigma), d_1(\sigma), d_0(\sigma)$. A 2-simplex with faces $f, h, g$ can be read as a *homotopy* from $h$ to $g \circ f$ — it witnesses that $g \circ f \simeq h$.

**Higher morphisms:** $n$-simplices for $n \geq 2$ are the higher homotopies, all of which are invertible (this is a consequence of the Kan condition for these higher simplices... or rather, we need a slightly more careful argument here about what "all 2-cells and above are invertible" means for quasi-categories).

## Homotopy Category of a Quasi-Category

Every quasi-category $X$ has an underlying ordinary category $hX$ (its *homotopy category*):
- Objects: 0-simplices of $X$
- Morphisms: equivalence classes of 1-simplices, where $f \sim g$ if there's a 2-simplex with faces $f$, $g$, and $\mathsf{id}$ (i.e., $f$ and $g$ are homotopic)
- Composition: given by horn filling, well-defined on equivalence classes

The homotopy category $hX$ is an ordinary category. It loses the higher homotopy information but captures the "coarse" structure of $X$.

**Example.** If $X$ is the quasi-category of spaces, $hX$ is the ordinary homotopy category of spaces (spaces and homotopy classes of maps).

## The Nerve of an Ordinary Category

Every ordinary category $\mathcal{C}$ gives a quasi-category, its *nerve* $N(\mathcal{C})$.

**Definition 4.2 (Nerve).** The nerve of a category $\mathcal{C}$ is the simplicial set $N(\mathcal{C})$ with:
- 0-simplices: objects of $\mathcal{C}$
- 1-simplices: morphisms $f : A \to B$
- 2-simplices: composable pairs $(f, g)$ with a chosen composite $g \circ f$ — or more precisely, commutative triangles $A \xrightarrow{f} B \xrightarrow{g} C$ with composite $h = g \circ f$
- $n$-simplices: composable chains $A_0 \xrightarrow{f_1} A_1 \to \cdots \xrightarrow{f_n} A_n$

The face maps delete objects at the ends or compose adjacent morphisms. The degeneracy maps insert identity morphisms.

**Theorem 4.3.** $N(\mathcal{C})$ is a quasi-category. In fact, it fills inner horns *uniquely* — compositions are unique, as expected for ordinary categories.

The nerve construction embeds ordinary categories fully and faithfully into quasi-categories. Ordinary categories are the quasi-categories where horn-filling is unique (composition is strict).

**Theorem 4.4 (Characterization).** A quasi-category $X$ is (isomorphic to the nerve of) an ordinary category if and only if all inner horns fill uniquely.

## Lurie's $\infty$-Category Theory

The systematic development of (∞,1)-category theory via quasi-categories is due largely to Jacob Lurie, in his landmark books *Higher Topos Theory* (2009) and *Higher Algebra* (2017). These are enormous works (HTT is ~1000 pages) that establish the entire foundations of ∞-categorical algebra.

Key developments in Lurie's program:
- **Limits and colimits** in quasi-categories (defined via mapping spaces)
- **Adjunctions** between quasi-categories
- **∞-toposes** (quasi-categories satisfying topos-like axioms)
- **Presentable ∞-categories** (the ∞-categorical analog of locally presentable categories)
- **Stable ∞-categories** (the home of spectra and homological algebra)
- **The ∞-category of ∞-categories** (quasi-categories form a quasi-category themselves)

The key theorem connecting to HoTT:

**Theorem 4.5 (Lurie).** Every ∞-topos is the ∞-category of sheaves on some ∞-groupoid (with respect to some Grothendieck topology). The ∞-topos of spaces (Kan complexes) is the initial ∞-topos.

And the connection to HoTT (due to Shulman, building on Lurie):

**Theorem 4.6 (Shulman, informal).** The internal language of the ∞-topos of spaces is HoTT + Univalence. More generally, HoTT is the internal language of ∞-toposes.

This is the deepest theorem in the chapter — and it's still being fully formalized. But it gives the conceptual picture: HoTT is to ∞-toposes as intuitionistic higher-order logic is to 1-toposes.

## Why Inner (Not Outer) Horn Filling?

It's worth pausing to understand why inner horn filling gives (∞,1)-categories while all horn filling gives ∞-groupoids.

The outer horn $\Lambda^n_0$ represents a sequence of morphisms with the *last* morphism missing and its *inverse* present. Filling $\Lambda^n_0$ gives the last morphism — which is "composing with an inverse." In a groupoid, you can always find an inverse, so outer horns fill. In a general (∞,1)-category, not all morphisms are invertible, so you can't always fill outer horns.

More precisely:
- **$\Lambda^1_0 \to X$:** A single vertex (the target of a morphism). Filling gives a morphism from some source to this vertex — that's always possible (just take the identity). Actually $\Lambda^1_0$ to any simplicial set is just picking a vertex.
- **$\Lambda^2_0 \to X$:** Edges $01$ and $02$ present (i.e., morphisms $f: 0 \to 1$ and $g: 0 \to 2$). Filling gives an edge $12$ (a morphism $1 \to 2$), which would be $g \circ f^{-1}$. This requires $f$ to be invertible!
- Similarly for $\Lambda^2_2$: this requires a morphism $0 \to 1$ given morphisms $0 \to 2$ and $1 \to 2$, which would be $g^{-1} \circ f$.

So outer horn filling corresponds to inverses existing, and inner horn filling corresponds to composition existing. Quasi-categories (inner horn filling) are exactly the (∞,1)-categories with composition but not necessarily inverses.

## The (∞,1)-Category of Spaces

The most important example of a quasi-category is the (∞,1)-category **Spc** of spaces.

One way to construct it: take $\mathsf{Sing}(X)$ for each topological space $X$ (the singular complex, which is a Kan complex), and form a quasi-category whose:
- 0-simplices are Kan complexes (or topological spaces)
- 1-simplices are maps between them (homotopy-coherent maps)
- Higher simplices are homotopies between homotopies, etc.

In **Spc**:
- Objects are homotopy types
- 1-morphisms are maps (continuous maps, up to homotopy)
- 2-morphisms are homotopies between maps
- 3-morphisms are homotopies between homotopies
- etc.

The automorphism group of a space $X$ in **Spc** is the loop space $\Omega X$ (in some sense). For $X = S^1$, this is $\pi_1(S^1) = \mathbb{Z}$ in the appropriate sense.

**Spc** is the terminal ∞-topos. Every other ∞-topos has a geometric morphism to **Spc**. This is the ∞-categorical analog of "every topos has a unique geometric morphism to Set."

## Summary

| Structure | Horn Filling | Morphisms Invertible? |
|---|---|---|
| Ordinary category (nerve) | All inner, uniquely | No |
| Quasi-category | All inner | No (2-cells and up: yes) |
| Kan complex | All (inner and outer) | Yes |
| Ordinary groupoid (nerve) | All, uniquely | Yes |

Quasi-categories are the "just right" generalization of ordinary categories to the homotopy-coherent setting. They allow composition without inverses, but in a way that's only well-defined up to homotopy. The difference between quasi-categories and Kan complexes is precisely the invertibility of 1-morphisms — quasi-categories are (∞,1)-categories, Kan complexes are ∞-groupoids.
