# 1.1 The Simplex Category $\Delta$

## The Building Blocks

Simplicial geometry is built from simplices: points, line segments, triangles, tetrahedra, and their higher-dimensional analogs.

- A **0-simplex** is a point.
- A **1-simplex** is an edge (line segment between two vertices).
- A **2-simplex** is a filled triangle (three vertices, three edges, one interior).
- A **3-simplex** is a solid tetrahedron (four vertices, six edges, four faces, one interior).
- An **$n$-simplex** is the convex hull of $n+1$ vertices in "general position."

The *simplex category* $\Delta$ organizes these shapes and the maps between them in a purely combinatorial way, without reference to geometry.

## The Simplex Category

**Definition 1.1 (Simplex Category $\Delta$).** The simplex category $\Delta$ has:
- **Objects:** The finite non-empty ordinals $[n] = \{0, 1, \ldots, n\}$ for $n \geq 0$
  - $[0] = \{0\}$ (one element — models a point)
  - $[1] = \{0, 1\}$ (two elements — models an edge)
  - $[2] = \{0, 1, 2\}$ (three elements — models a triangle)
  - $[n] = \{0, 1, \ldots, n\}$ ($n+1$ elements — models an $n$-simplex)

- **Morphisms:** Order-preserving (non-decreasing) functions $f : [m] \to [n]$, i.e., functions where $i \leq j \Rightarrow f(i) \leq f(j)$

- **Composition:** Ordinary composition of functions (which preserves order-preservation)

- **Identity:** The identity function $\mathsf{id}_{[n]} : [n] \to [n]$

The key point: the objects are *ordered* sets, and morphisms must preserve the ordering. This encodes the combinatorial structure of simplices: the vertices of a simplex have a natural ordering (vertex 0, vertex 1, ..., vertex $n$), and the maps between simplices must preserve this ordering.

## Face and Degeneracy Maps

The generators of all morphisms in $\Delta$ are two types of maps:

**Face maps (injections):** $d^i : [n-1] \to [n]$ for $0 \leq i \leq n$.

$$d^i(j) = \begin{cases} j & \text{if } j < i \\ j + 1 & \text{if } j \geq i \end{cases}$$

This is the unique injective order-preserving map $[n-1] \to [n]$ that *misses $i$* (its image is $\{0,\ldots,n\} \setminus \{i\}$).

**Intuition:** The face map $d^i$ picks out the "$i$-th face" of an $n$-simplex — the face opposite vertex $i$. When you apply $d^i$ to the vertices of an $(n-1)$-simplex, you get the $i$-th face of the $n$-simplex.

**Degeneracy maps (surjections):** $s^i : [n+1] \to [n]$ for $0 \leq i \leq n$.

$$s^i(j) = \begin{cases} j & \text{if } j \leq i \\ j - 1 & \text{if } j > i \end{cases}$$

This is the unique surjective order-preserving map $[n+1] \to [n]$ that *doubles $i$* (maps both $i$ and $i+1$ to $i$).

**Intuition:** The degeneracy map $s^i$ "collapses" vertex $i$ and vertex $i+1$ together. In a simplicial set, this corresponds to "inserting a degenerate simplex" — a simplex that's really lower-dimensional, just written as a higher-dimensional one.

## Explicit Low-Dimensional Cases

Let's work out the face and degeneracy maps for small $n$ to build intuition.

**$n = 1$ (edges):**
- $d^0 : [0] \to [1]$: maps $\{0\} \to \{1\}$ — picks the "target" vertex
- $d^1 : [0] \to [1]$: maps $\{0\} \to \{0\}$ — picks the "source" vertex
- $s^0 : [1] \to [0]$: collapses everything to $0$ — the "degenerate edge" on a point

**$n = 2$ (triangles):**
- $d^0 : [1] \to [2]$: $0 \mapsto 1, 1 \mapsto 2$ — the face opposite vertex 0 (the "back edge")
- $d^1 : [1] \to [2]$: $0 \mapsto 0, 1 \mapsto 2$ — the face opposite vertex 1 (the "slanted edge")
- $d^2 : [1] \to [2]$: $0 \mapsto 0, 1 \mapsto 1$ — the face opposite vertex 2 (the "front edge")
- $s^0 : [2] \to [1]$: $0 \mapsto 0, 1 \mapsto 0, 2 \mapsto 1$ — collapse $v_0$ and $v_1$ together
- $s^1 : [2] \to [1]$: $0 \mapsto 0, 1 \mapsto 1, 2 \mapsto 1$ — collapse $v_1$ and $v_2$ together

**Geometric picture:** A triangle $\Delta^2$ has three edges (three 1-faces) and three vertices (three 0-faces). The face maps $d^i : [1] \to [2]$ pick out these edges. The degeneracy maps collapse the triangle onto an edge.

## The Simplicial Identities

The face and degeneracy maps satisfy a collection of identities called the *simplicial identities*. These are crucial because they ensure that the combinatorics is consistent — you get the same simplex no matter which way you traverse a sequence of face/degeneracy operations.

**Face-face:** For $i < j$:
$$d^j \circ d^i = d^i \circ d^{j-1} : [n-2] \to [n]$$

This says: applying face $i$ then face $j$ is the same as applying face $j-1$ then face $i$. Both skip vertices $i$ and $j$.

**Degeneracy-degeneracy:** For $i \leq j$:
$$s^j \circ s^i = s^i \circ s^{j+1} : [n+2] \to [n]$$

**Face-degeneracy:** For $i < j$:
$$s^j \circ d^i = d^i \circ s^{j-1} : [n] \to [n]$$

For $i = j$ or $i = j+1$:
$$s^j \circ d^j = \mathsf{id} = s^j \circ d^{j+1} : [n] \to [n]$$

For $i > j+1$:
$$s^j \circ d^i = d^{i-1} \circ s^j : [n] \to [n]$$

**Verification (face-face):** $d^j \circ d^i$ skips $i$ first (among $[n-2] \to [n-1]$), then skips $j$ (among $[n-1] \to [n]$). Since $i < j$, skipping $j$ in $[n-1]$ corresponds to skipping $j$ in the image. $d^i \circ d^{j-1}$ skips $j-1$ first (among $[n-2] \to [n-1]$), then skips $i$ (among $[n-1] \to [n]$). Both operations skip the same two vertices $i$ and $j$ in $[n]$.

## Every Morphism Factors Uniquely

**Theorem 1.2 (Factorization in $\Delta$).** Every morphism $f : [m] \to [n]$ in $\Delta$ factors uniquely as:
$$f = d^{i_1} \circ d^{i_2} \circ \cdots \circ d^{i_k} \circ s^{j_1} \circ s^{j_2} \circ \cdots \circ s^{j_l}$$
where $i_1 > i_2 > \cdots > i_k$ and $j_1 < j_2 < \cdots < j_l$.

In other words: every morphism in $\Delta$ is uniquely a surjection followed by an injection, where the injections are composed in decreasing order and the surjections in increasing order.

This factorization is the key structural theorem about $\Delta$. It means that face and degeneracy maps generate all of $\Delta$, and the simplicial identities give a complete set of relations.

## The Role of $\Delta$ in Homotopy Theory

Why is $\Delta$ the right category for homotopy theory? Several reasons:

**1. Simplices are the building blocks.** Every topological space can be triangulated (or at least approximated) by simplices. The combinatorial data of $\Delta$ captures the combinatorial structure of triangulations.

**2. The nerve construction.** Every small category $\mathcal{C}$ has a *nerve* $N(\mathcal{C}) : \Delta^{op} \to \mathbf{Set}$, which is a simplicial set capturing the structure of $\mathcal{C}$. This is the bridge between category theory and homotopy theory.

**3. The classifying space.** The geometric realization $|N(\mathcal{C})|$ (the classifying space of $\mathcal{C}$) is a topological space with:
- $\pi_1 =$ the "fundamental groupoid" of $\mathcal{C}$
- For groupoids: $\pi_1 =$ the group of automorphisms

**4. Computational convenience.** The simplex category is simple enough to compute with explicitly, but rich enough to capture all homotopy types. This balance is what makes it ideal for both theoretical and computational purposes.

## Summary

| Object | Meaning | Example |
|---|---|---|
| $[n] \in \Delta$ | An ordered $(n+1)$-element set | $[2] = \{0,1,2\}$ (triangle) |
| $d^i : [n-1] \to [n]$ | Face map (inclusion missing $i$) | $d^1(j) = 0 \mapsto 0, 1 \mapsto 2$ |
| $s^i : [n+1] \to [n]$ | Degeneracy map (collapse $i,i+1$) | $s^0(j) = 0,1 \mapsto 0, 2 \mapsto 1$ |
| Simplicial identity | Consistency of face/degeneracy | $d^j d^i = d^i d^{j-1}$ for $i < j$ |

The simplex category is the combinatorial skeleton of all simplicial geometry. It tells you exactly what the faces and degeneracies of simplices are, in a purely algebraic way. Everything in simplicial homotopy theory is built on top of this simple category.
