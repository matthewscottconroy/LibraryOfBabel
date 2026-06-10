# 2.1 Simplicial Sets as Presheaves

## The Definition

A simplicial set is a functor. That's the clean categorical definition. But let's unpack what this means concretely.

**Definition 2.1 (Simplicial Set).** A *simplicial set* $X$ is a functor $X : \Delta^{op} \to \mathbf{Set}$.

Unpacking:
- For each object $[n] \in \Delta$, a set $X_n = X([n])$ (the *$n$-simplices*)
- For each morphism $f : [m] \to [n]$ in $\Delta$, a function $X(f) : X_n \to X_m$ (going *backwards* because it's a functor on $\Delta^{op}$)
- Functoriality: $X(g \circ f) = X(f) \circ X(g)$ and $X(\mathsf{id}) = \mathsf{id}$

The specific morphisms we care about are the face and degeneracy maps:
- **Face maps:** $\partial_i = X(d^i) : X_n \to X_{n-1}$ for $0 \leq i \leq n$
- **Degeneracy maps:** $\sigma_i = X(s^i) : X_n \to X_{n+1}$ for $0 \leq i \leq n$

These inherit the simplicial identities from $\Delta$:
- $\partial_i \partial_j = \partial_{j-1} \partial_i$ for $i < j$ (face maps compose as "face of a face")
- $\sigma_i \sigma_j = \sigma_{j+1} \sigma_i$ for $i \leq j$
- And the mixed relations

**Morphisms between simplicial sets:** A *map* $f : X \to Y$ of simplicial sets is a natural transformation: a collection of functions $f_n : X_n \to Y_n$ commuting with all face and degeneracy maps.

The category of simplicial sets is denoted $\mathbf{sSet}$.

## The Intuition: Combinatorial Spaces

Think of a simplicial set as a "combinatorial space" built from simplices:
- Elements of $X_0$ are *vertices* (0-dimensional points)
- Elements of $X_1$ are *edges* (1-dimensional segments)
- Elements of $X_2$ are *triangles* (2-dimensional faces)
- Elements of $X_n$ are *$n$-simplices*

The face maps tell you the boundary of each simplex:
- $\partial_0(\sigma) \in X_{n-1}$: the "0th face" of $\sigma$ (the face opposite vertex 0)
- $\partial_1(\sigma) \in X_{n-1}$: the "1st face" of $\sigma$
- etc.

For an edge $e \in X_1$:
- $\partial_1(e)$ is the *source* (start) of $e$
- $\partial_0(e)$ is the *target* (end) of $e$

For a triangle $t \in X_2$:
- $\partial_0(t)$ is the "opposite edge" from vertex 0 (the edge between vertices 1 and 2)
- $\partial_1(t)$ is the edge between vertices 0 and 2
- $\partial_2(t)$ is the edge between vertices 0 and 1

The degeneracy maps give *degenerate simplices*:
- $\sigma_i(x)$ for $x \in X_n$ gives a degenerate $(n+1)$-simplex that's "really just $x$ in a higher dimension"
- Degenerate simplices are like "identity" simplices — they carry no new topological information

## Key Examples

### The Standard Simplex $\Delta[n]$

The representable simplicial set:
$$\Delta[n] = \mathsf{Hom}_\Delta(-, [n]) : [m] \mapsto \mathsf{Hom}_\Delta([m], [n])$$

$\Delta[n]_m$ = the set of order-preserving maps $[m] \to [n]$ = non-decreasing sequences $(a_0, a_1, \ldots, a_m)$ with $0 \leq a_i \leq n$.

- $\Delta[n]_0 = \{0, 1, \ldots, n\}$ — the $n+1$ vertices
- $\Delta[n]_1$ = edges: pairs $(i,j)$ with $i \leq j$ — there are $\binom{n+2}{2}$ of them
- The unique non-degenerate $n$-simplex: $(0, 1, \ldots, n)$

By the Yoneda lemma: $\mathsf{Hom}_{\mathbf{sSet}}(\Delta[n], X) \cong X_n$ naturally. So $n$-simplices in $X$ correspond to maps from $\Delta[n]$ to $X$.

### The Boundary $\partial\Delta[n]$

The simplicial subset $\partial\Delta[n] \subseteq \Delta[n]$ consisting of all *non-surjective* order-preserving maps $[m] \to [n]$ — all maps that miss at least one element of $[n]$.

Geometrically: $\partial\Delta[n]$ is the "boundary sphere" — the $n$-simplex without its interior, just its $(n-1)$-dimensional faces.

- $|\partial\Delta[0]| = \emptyset$ (the boundary of a point is empty)
- $|\partial\Delta[1]| = \{0,1\}$ (the boundary of an edge is two points)
- $|\partial\Delta[2]| \cong S^1$ (the boundary of a triangle is a circle)
- $|\partial\Delta[n]| \cong S^{n-1}$ (in general)

### Horns $\Lambda^n_k$

The horn $\Lambda^n_k \subseteq \partial\Delta[n]$ is the simplicial subset obtained by removing the $k$-th face of $\partial\Delta[n]$. Concretely:

$\Lambda^n_k$ consists of all non-surjective maps $[m] \to [n]$ whose image also omits at least one element *other than $k$* — that is, maps whose image is not $[n] \setminus \{k\}$.

Geometrically: $\Lambda^n_k$ is the "horn" — the boundary of an $n$-simplex with one face (the $k$-th) removed.

- $|\Lambda^2_1|$: a triangle with its interior face and one edge removed — just two edges forming a "V" shape. This is contractible.
- $|\Lambda^n_k|$ is contractible for all $n, k$ (this is what makes horn filling a useful condition).

The two types of horns:
- **Inner horns** $\Lambda^n_k$ for $0 < k < n$: missing an interior face
- **Outer horns** $\Lambda^n_0$ and $\Lambda^n_n$: missing an outer (end) face

### The Nerve of a Category

For a small category $\mathcal{C}$, the *nerve* $N(\mathcal{C})$ is the simplicial set:
$$N(\mathcal{C})_n = \{\text{composable chains } A_0 \xrightarrow{f_1} A_1 \xrightarrow{f_2} \cdots \xrightarrow{f_n} A_n \text{ in } \mathcal{C}\}$$

Face maps:
- $\partial_0(A_0 \to \cdots \to A_n) = (A_1 \to \cdots \to A_n)$ (remove first object)
- $\partial_n(A_0 \to \cdots \to A_n) = (A_0 \to \cdots \to A_{n-1})$ (remove last object)
- $\partial_i(A_0 \to \cdots \to A_n) = (A_0 \to \cdots \to A_{i-1} \xrightarrow{f_{i+1} \circ f_i} A_{i+1} \to \cdots \to A_n)$ (compose $f_i$ and $f_{i+1}$)

Degeneracy maps:
- $\sigma_i(A_0 \to \cdots \to A_n) = (A_0 \to \cdots \to A_i \xrightarrow{\mathsf{id}} A_i \to \cdots \to A_n)$ (insert identity)

The nerve is a simplicial set whose simplices are sequences of composable morphisms. It's the fundamental bridge between category theory and simplicial homotopy theory.

### The Singular Complex

For a topological space $X$, the *singular simplicial set* $\mathsf{Sing}(X)$:
$$\mathsf{Sing}(X)_n = C(|\Delta^n|, X)$$

(continuous maps from the geometric $n$-simplex to $X$). Face and degeneracy maps come from the geometric face and degeneracy maps on $|\Delta^n|$.

$\mathsf{Sing}(X)$ is a Kan complex (this is a key theorem, proved using the homotopy extension property of geometric horns). It encodes all the homotopy-theoretic information about $X$.

## Geometric Realization

Going the other way: given a simplicial set $X$, we build a topological space $|X|$.

**Definition 2.2 (Geometric Realization).** The *geometric realization* of $X$ is:
$$|X| = \left(\bigsqcup_{n \geq 0} X_n \times |\Delta^n|\right) / \sim$$

where $\sim$ is the equivalence relation generated by:
- $(X(d^i)(\sigma), x) \sim (\sigma, |d^i|(x))$ for face maps (identifies the face of a simplex with the corresponding face of the geometric simplex)
- $(X(s^i)(\sigma), x) \sim (\sigma, |s^i|(x))$ for degeneracy maps

**Theorem 2.3.** Geometric realization and singular complex are adjoint:
$$\mathsf{Hom}_{\mathbf{Top}}(|X|, Y) \cong \mathsf{Hom}_{\mathbf{sSet}}(X, \mathsf{Sing}(Y))$$

This adjunction $|-| \dashv \mathsf{Sing}$ is the bridge between the combinatorial and topological worlds.

**Key computations:**
- $|\Delta[n]| \cong \Delta^n$ (the geometric $n$-simplex)
- $|\partial\Delta[n]| \cong S^{n-1}$ (the $(n-1)$-sphere)
- $|N(\mathcal{C})|$ = the *classifying space* $B\mathcal{C}$ of the category $\mathcal{C}$
- $|\mathsf{Sing}(X)| \simeq X$ for "nice" spaces $X$ (CW complexes)

## Summary

| Concept | Definition | Example |
|---|---|---|
| Simplicial set | Functor $\Delta^{op} \to \mathbf{Set}$ | $\mathsf{Sing}(X)$, $N(\mathcal{C})$ |
| $n$-simplex | Element of $X_n$ | Path in $\mathsf{Sing}(X)$ |
| Face map $\partial_i$ | $X(d^i) : X_n \to X_{n-1}$ | Boundary of a triangle |
| Degeneracy $\sigma_i$ | $X(s^i) : X_n \to X_{n+1}$ | Degenerate simplex |
| Standard simplex $\Delta[n]$ | Representable: $\mathsf{Hom}_\Delta(-,[n])$ | Models an $n$-simplex |
| Boundary $\partial\Delta[n]$ | Remove interior | Models $S^{n-1}$ |
| Horn $\Lambda^n_k$ | Remove one face too | Contractible sub-sphere |
| Nerve $N(\mathcal{C})$ | Composable chains | Classifying space of $\mathcal{C}$ |

Simplicial sets are the right combinatorial framework for homotopy theory. They're simple to define (just a functor), rich in examples (topological spaces, categories, groups), and powerful enough to model all homotopy types (via Kan complexes).
