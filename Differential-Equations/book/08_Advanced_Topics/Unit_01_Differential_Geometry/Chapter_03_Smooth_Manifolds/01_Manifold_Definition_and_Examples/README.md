# Manifold Definition and Examples

A manifold is a topological space that is locally indistinguishable from Euclidean space. The globally interesting geometry comes from how the local patches are assembled. Making this precise requires the concept of an atlas of coordinate charts, and smoothness requires that the transitions between charts be smooth maps. This section gives the rigorous definition and a collection of fundamental examples.

## Definition

**Definition.** An **$n$-dimensional smooth manifold** is a pair $(M, \mathcal{A})$ where:

1. $M$ is a topological space (Hausdorff and second-countable).
2. $\mathcal{A} = \{(U_\alpha, \phi_\alpha)\}$ is a **smooth atlas**: a collection of pairs (**charts**) where $\{U_\alpha\}$ is an open cover of $M$ and each $\phi_\alpha: U_\alpha \to \phi_\alpha(U_\alpha) \subset \mathbb{R}^n$ is a homeomorphism onto an open subset of $\mathbb{R}^n$.
3. Whenever $U_\alpha \cap U_\beta \neq \emptyset$, the **transition map** $\phi_\beta \circ \phi_\alpha^{-1}: \phi_\alpha(U_\alpha \cap U_\beta) \to \phi_\beta(U_\alpha \cap U_\beta)$ is a smooth ($C^\infty$) diffeomorphism.

Two atlases are **equivalent** if their union is also a smooth atlas. A smooth manifold is a topological space together with an equivalence class of atlases (a **smooth structure**).

A function $f: M \to \mathbb{R}$ is **smooth** if $f \circ \phi_\alpha^{-1}: \phi_\alpha(U_\alpha) \to \mathbb{R}$ is smooth for every chart $(U_\alpha, \phi_\alpha)$. A map $F: M \to N$ between manifolds is smooth if its coordinate representations are smooth.

## Examples

**Euclidean space $\mathbb{R}^n$.** The single chart $(\mathbb{R}^n, \text{id})$ gives $\mathbb{R}^n$ a smooth manifold structure. Every open subset of $\mathbb{R}^n$ is a smooth manifold (with the induced atlas).

**The $n$-sphere $S^n$.** Define $S^n = \{x \in \mathbb{R}^{n+1} : |x| = 1\}$. Two charts cover $S^n$:

- Stereographic projection from the north pole $N = (0,\ldots,0,1)$: $\phi_N(x_1,\ldots,x_{n+1}) = (x_1,\ldots,x_n)/(1-x_{n+1})$, defined on $S^n \setminus \{N\}$.
- Stereographic projection from the south pole: similarly.

The transition map $\phi_S \circ \phi_N^{-1}: \mathbb{R}^n \setminus \{0\} \to \mathbb{R}^n \setminus \{0\}$ is $x \mapsto x/|x|^2$ (inversion), which is smooth. So $S^n$ is a smooth $n$-manifold.

**The $n$-torus $T^n$.** $T^n = S^1 \times S^1 \times \cdots \times S^1$ ($n$ factors) is a smooth $n$-manifold with the product smooth structure. As a quotient, $T^n = \mathbb{R}^n / \mathbb{Z}^n$: points of $\mathbb{R}^n$ are identified if they differ by an integer vector. Local charts are provided by small open sets in $\mathbb{R}^n$.

**Real projective space $\mathbb{RP}^n$.** Points of $\mathbb{RP}^n$ are lines through the origin in $\mathbb{R}^{n+1}$, represented as equivalence classes $[x_0 : x_1 : \cdots : x_n]$ (homogeneous coordinates). The $n+1$ charts are $U_i = \{[x] : x_i \neq 0\}$ with $\phi_i([x]) = (x_0/x_i, \ldots, \widehat{x_i/x_i}, \ldots, x_n/x_i) \in \mathbb{R}^n$. Transition maps are smooth, giving $\mathbb{RP}^n$ a smooth structure.

**Matrix groups: $GL(n, \mathbb{R})$, $O(n)$, $SL(n,\mathbb{R})$, $SO(n)$.** The general linear group $GL(n,\mathbb{R}) = \{A \in M_{n\times n}(\mathbb{R}) : \det A \neq 0\}$ is an open subset of $\mathbb{R}^{n^2}$, hence a smooth manifold of dimension $n^2$. The orthogonal group $O(n) = \{A : A^TA = I\}$ is a smooth manifold of dimension $n(n-1)/2$, by the regular value theorem applied to $F(A) = A^TA$ at the regular value $I$.

**Regular surfaces in $\mathbb{R}^3$.** As established in Chapter 1, every regular surface is a 2-dimensional smooth manifold (the local parametrizations are the chart maps).

## Submanifolds

A subset $N \subset M$ is a **smooth submanifold** if it is a manifold in its own right such that the inclusion map $N \hookrightarrow M$ is a smooth embedding (injective, smooth, with injective derivative). Equivalently, near each point $p \in N$, there is a chart $(U, \phi)$ of $M$ such that $\phi(N \cap U) = \{x \in \phi(U) : x_{k+1} = \cdots = x_n = 0\}$—the submanifold appears as a "slice" in local coordinates.

**Regular value theorem.** If $F: M \to N$ is smooth and $q \in N$ is a **regular value** (meaning $dF_p$ is surjective for all $p \in F^{-1}(q)$), then $F^{-1}(q)$ is a smooth submanifold of $M$ of dimension $\dim M - \dim N$. This is the generalization of the implicit function theorem to manifolds and produces most of the important examples of manifolds: spheres, orthogonal groups, special linear groups, and solution sets of regular systems of equations.

## Smooth Maps and Diffeomorphisms

A map $F: M \to N$ between smooth manifolds is **smooth** if its local coordinate representations are smooth. A smooth bijection with smooth inverse is a **diffeomorphism**. Two manifolds are diffeomorphic if there is a diffeomorphism between them; they are then geometrically identical.

A surprising fact is that topologically equivalent manifolds need not be diffeomorphic. In 1956, Milnor discovered exotic smooth structures on $S^7$: manifolds homeomorphic to the 7-sphere but not diffeomorphic to it. In dimension 4, there are uncountably many exotic smooth structures on $\mathbb{R}^4$. These exotic phenomena are special to smooth manifold theory and have no analogue in the classical theory of surfaces.

## Partitions of Unity

One of the key tools on manifolds is the existence of **partitions of unity**: a collection of smooth functions $\{\psi_\alpha\}$ with $\psi_\alpha \geq 0$, $\text{supp}(\psi_\alpha) \subset U_\alpha$, and $\sum_\alpha \psi_\alpha = 1$. Partitions of unity allow local constructions (defined in individual charts) to be patched together into global objects. They are used to construct Riemannian metrics, to define integration of differential forms, and to prove many existence theorems in global analysis.
