# Higher Homotopy Groups

## Beyond Loops

The fundamental group $\pi_1(X, x_0)$ detects 1-dimensional holes in $X$: loops that cannot be contracted to a point. A loop is a map from $S^1$ based at a point. The natural generalization: consider maps from $S^n$ based at a point, for any $n \geq 1$.

**Definition.** The *$n$th homotopy group* $\pi_n(X, x_0)$ is the set of homotopy classes of continuous maps $f : (S^n, *) \to (X, x_0)$ from the $n$-sphere to $X$, preserving basepoints, where homotopies are also required to preserve basepoints.

Equivalently (for computational purposes): $\pi_n(X, x_0)$ is the set of homotopy classes of maps $f : ([0,1]^n, \partial[0,1]^n) \to (X, x_0)$ from the $n$-cube to $X$ that map the entire boundary $\partial[0,1]^n$ to the basepoint $x_0$. The group structure is defined by concatenation in the first coordinate.

For $n = 1$: this recovers the fundamental group.

For $n = 0$: $\pi_0(X, x_0)$ is the set of path-components of $X$ (no group structure in general).

## The Abelian Miracle

The first non-trivial fact about higher homotopy groups is that they are abelian for $n \geq 2$. This is completely different from the fundamental group, which can be any group.

**Theorem (Eckmann-Hilton).** $\pi_n(X, x_0)$ is abelian for all $n \geq 2$.

The proof uses a beautiful argument called the *Eckmann-Hilton argument*:

Consider two maps $f, g : S^2 \to X$ (representing elements of $\pi_2$). In the cube model, these are maps $[0,1]^2 \to X$ that send the boundary to $x_0$. We can compose them in two ways:
- Horizontally: concatenate by the first coordinate.
- Vertically: concatenate by the second coordinate.

Both give a group structure on $\pi_2(X, x_0)$. The Eckmann-Hilton lemma says: if two binary operations on a set both have the same unit and each distributes over the other, then they are equal *and* abelian. Applied here: the horizontal and vertical compositions must both be the same operation, and that operation is commutative.

The Eckmann-Hilton argument is deeply connected to HoTT. In type theory, the identity type $a = a$ (the loop space of $A$ at $a$) has two composition operations: horizontal (by concatenating paths) and vertical (by concatenating homotopies). The Eckmann-Hilton argument shows that paths in the loop space — elements of $\Omega^2(A, a) = \Omega(\Omega(A,a), \mathsf{refl}_a)$ — form an abelian group. This is the type-theoretic statement that $\pi_2(A) = \pi_1(\Omega A)$ is abelian. The HoTT proof of Eckmann-Hilton is a landmark result in the book.

## Key Computations

**$\pi_n(S^n) = \mathbb{Z}$.** The $n$-sphere has $n$th homotopy group $\mathbb{Z}$. The generator is the identity map $\mathsf{id} : S^n \to S^n$, and the integer corresponds to the *degree* of the map (how many times it wraps $S^n$ around itself).

**$\pi_k(S^n) = 0$ for $k < n$.** Maps from lower-dimensional spheres to higher-dimensional ones can always be contracted. (This follows from the cellular approximation theorem: a map from a $k$-dimensional CW complex to an $n$-dimensional CW complex is homotopic to a map that sends cells to cells of the same or lower dimension; if $k < n$, the image misses the top cell of $S^n$, and $S^n$ minus a point is contractible.)

**$\pi_1(S^n) = 0$ for $n \geq 2$.** Higher-dimensional spheres are simply connected. This follows from the same cellular argument.

**$\pi_3(S^2) = \mathbb{Z}$.** This is the first surprising computation: a 2-sphere has a non-trivial third homotopy group. The generator is the Hopf fibration $\eta : S^3 \to S^2$. See Section 5 for details.

The full table of homotopy groups of spheres $\pi_k(S^n)$ is extraordinarily complex and is one of the central open problems of algebraic topology. Even $\pi_k(S^2)$ for large $k$ is not fully computed.

## Eilenberg-MacLane Spaces

Given a group $G$ (abelian if $n \geq 2$) and a positive integer $n$, an *Eilenberg-MacLane space* $K(G, n)$ is a topological space with $\pi_n(K(G,n)) = G$ and $\pi_k(K(G,n)) = 0$ for all $k \neq n$. Such spaces exist and are unique up to homotopy equivalence.

Examples:
- $K(\mathbb{Z}, 1) = S^1$: the circle has $\pi_1 = \mathbb{Z}$ and $\pi_k = 0$ for $k \geq 2$ (the latter uses the universal cover $\mathbb{R}$, which is contractible, so $\pi_k(S^1) = \pi_k(\mathbb{R}) = 0$ for $k \geq 2$ by the homotopy-lifting property).
- $K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$: the infinite complex projective space.
- $K(\mathbb{Z}/2\mathbb{Z}, 1) = \mathbb{RP}^\infty$: the infinite real projective space.
- $K(G, 1)$ for any group $G$: the classifying space $BG$.

Eilenberg-MacLane spaces are fundamental building blocks of homotopy theory. Every simply-connected space $X$ can be reconstructed (up to homotopy equivalence) from its Postnikov tower: a sequence of fibrations $\cdots \to P_3(X) \to P_2(X) \to P_1(X) = K(\pi_1(X), 1)$ where each $P_n(X)$ has homotopy groups $\pi_k(P_n(X)) = \pi_k(X)$ for $k \leq n$ and $0$ otherwise. The space $X$ is the homotopy limit of this tower.

In HoTT, Eilenberg-MacLane spaces are defined using higher inductive types and truncations. The type $K(G, n)$ is the $n$-fold loop space of the delooping of $G$, or equivalently the $n$-fold suspension of the type $BG$ constructed from $G$ as a HIT. This is one of the central constructions of synthetic homotopy theory.

## The Hurewicz Theorem

The relationship between homotopy groups and homology groups is given by the Hurewicz theorem.

**Theorem (Hurewicz).** If $X$ is $(n-1)$-connected (i.e., $\pi_k(X) = 0$ for $k < n$, with $n \geq 2$), then:
$$H_k(X) = 0 \text{ for } 1 \leq k < n$$
and there is an isomorphism
$$\pi_n(X, x_0) \cong H_n(X)$$

The Hurewicz map $h : \pi_n(X) \to H_n(X)$ sends a map $f : S^n \to X$ to the image of the fundamental class $[S^n] \in H_n(S^n) = \mathbb{Z}$.

For $n = 1$ (the base case): $\pi_1(X)^{ab} \cong H_1(X)$, where $\pi_1(X)^{ab}$ is the abelianization of the fundamental group.

The Hurewicz theorem is the bridge between homotopy and homology: it gives a systematic way to compute the first non-trivial homology group from the first non-trivial homotopy group. For simply-connected spaces, it makes homotopy and homology interchangeable in low degrees.

## The Long Exact Sequence of a Pair

For a pair $(X, A)$ with $A \subseteq X$, the *relative homotopy groups* $\pi_n(X, A, x_0)$ fit into a long exact sequence:
$$\cdots \to \pi_n(A, x_0) \xrightarrow{i_*} \pi_n(X, x_0) \xrightarrow{j_*} \pi_n(X, A, x_0) \xrightarrow{\partial} \pi_{n-1}(A, x_0) \to \cdots$$

The boundary map $\partial : \pi_n(X, A) \to \pi_{n-1}(A)$ takes a relative map (a map from $(D^n, S^{n-1})$ to $(X, A)$) and records where the boundary sphere $S^{n-1}$ maps in $A$.

The long exact sequence is the main tool for computing homotopy groups from simpler pieces. When $A \hookrightarrow X$ is a fibration (Section 5), the sequence specializes to the long exact sequence of a fibration — the most powerful computational tool in homotopy theory.

## HoTT: The Eckmann-Hilton Argument in Type Theory

In HoTT, the Eckmann-Hilton argument has an elegant formulation. Consider a type $A$ and a term $a : A$. The loop space $\Omega(A, a) = (a =_A a)$ has a multiplication (path concatenation) and is a group. The second loop space $\Omega^2(A, a) = \Omega(\Omega(A, a), \mathsf{refl}_a) = (\mathsf{refl}_a =_{(a=a)} \mathsf{refl}_a)$ has two multiplications:
- Horizontal: concatenation of paths in $a = a$ (treating them as loops in $A$).
- Vertical: concatenation of homotopies between loops (paths in the loop space).

The Eckmann-Hilton theorem in HoTT says: both multiplications are equal, and the resulting group is abelian. This is proved by a series of path algebra manipulations involving the interchange law for concatenation in two directions.

The key consequence: $\pi_2(A) = \pi_1(\Omega A) = \pi_0(\Omega^2 A)$ is abelian for any type $A$. This matches the classical statement that $\pi_n$ is abelian for $n \geq 2$.
