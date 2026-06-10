# Kan Complexes

## The Horn-Filling Condition

A simplicial set can have "gaps" — a collection of simplices where all the lower-dimensional faces are present but the higher-dimensional simplex "filling" them is missing. The Kan condition says: no gaps. Every partial filling can be completed.

More precisely: a horn $\Lambda^n_k \to X$ is a collection of $(n-1)$-simplices of $X$ that form all but one face of a potential $n$-simplex (specifically, all faces except the $k$-th). A horn filling is an $n$-simplex $\Delta[n] \to X$ extending this partial data.

**Definition.** A simplicial set $X$ is a *Kan complex* (or *fibrant simplicial set*) if every horn $\Lambda^n_k \hookrightarrow \Delta[n]$ and every map $\Lambda^n_k \to X$ extends to a map $\Delta[n] \to X$:
$$\forall n \geq 1, 0 \leq k \leq n: \text{every map } \Lambda^n_k \to X \text{ extends to } \Delta[n] \to X$$

This extension need not be unique — only existence is required.

## Inner vs. Outer Horns

The horns $\Lambda^n_k$ for $0 < k < n$ are called *inner horns*. The horns $\Lambda^n_0$ and $\Lambda^n_n$ are the *outer horns*.

The distinction matters for different versions of the condition:
- **Kan complexes** (filling all horns, inner and outer) model $\infty$-groupoids — spaces where all morphisms at all levels are invertible.
- **Quasi-categories** (filling only inner horns) model $(\infty, 1)$-categories — $\infty$-groupoids where 2-morphisms and above are invertible, but 1-morphisms need not be.

For modeling homotopy types (where all morphisms are invertible — paths can be reversed, homotopies can be "inverted"), Kan complexes are the right notion.

## The Geometric Interpretation

What does the Kan condition mean geometrically?

**For $n = 2$:** A 2-horn $\Lambda^2_k \to X$ specifies two edges of a triangle and their shared vertex. The Kan condition asks for a filling of the triangle — a 2-simplex of $X$ with these two edges as faces.

- Horn $\Lambda^2_1$ (inner): specifies edges $\{0,1\}$ and $\{1,2\}$ (sharing vertex 1). Filling asks for the edge $\{0,2\}$ and a 2-simplex filling the triangle. This says: any two composable morphisms (edges) have a composition.

- Horn $\Lambda^2_0$ (outer): specifies edges $\{0,1\}$ and $\{0,2\}$ (sharing vertex 0). Filling asks for the edge $\{1,2\}$ and a 2-simplex. This says: any morphism has a right inverse.

- Horn $\Lambda^2_2$ (outer): specifies edges $\{1,2\}$ and $\{0,2\}$ (sharing vertex 2). Filling says: any morphism has a left inverse.

So the Kan condition for $n=2$ says: any two composable morphisms have a (homotopy) composition, and every morphism has homotopy inverses. This is exactly the condition for an $\infty$-groupoid.

**For $n = 1$:** The horn $\Lambda^1_0$ specifies vertex $1$ (the target); a filling asks for an edge from some vertex to vertex $1$, i.e., a path ending at vertex $1$. The horn $\Lambda^1_1$ specifies vertex $0$ (the source); a filling asks for an edge starting at vertex $0$. The Kan condition for $n = 1$ says the space is "inhabited" in a trivial sense.

**For $n = 3$:** The inner horns $\Lambda^3_1$ and $\Lambda^3_2$ ask for fillings of tetrahedra given three of their four faces. This is the condition that compositions of morphisms are associative up to coherent homotopy.

## The Singular Complex Is Always Kan

**Theorem.** For any topological space $Y$, the singular complex $\text{Sing}(Y)$ is a Kan complex.

*Proof.* A horn $\Lambda^n_k \to \text{Sing}(Y)$ is a continuous map $|\Lambda^n_k| \to Y$ from the geometric realization of the horn. The geometric realization $|\Lambda^n_k|$ is a contractible space (a simplex with one face removed, which deformation retracts to a point). In particular, $|\Lambda^n_k| \hookrightarrow |\Delta^n|$ is a homotopy equivalence, and by the Tietze extension theorem (or the fact that $|\Delta^n|$ is contractible), any continuous map from a retract to $Y$ extends to the larger space. So the horn filler exists.

This theorem is the key: every topological space $Y$ gives rise to a Kan complex $\text{Sing}(Y)$, and the homotopy groups of the Kan complex equal those of the space: $\pi_n(\text{Sing}(Y), v) \cong \pi_n(Y, v)$ for any vertex $v$.

## Homotopy Groups of Kan Complexes

For a Kan complex $X$ and a vertex $v \in X_0$, the homotopy groups are defined combinatorially.

A *loop* at $v$ is an edge $\sigma \in X_1$ with $d_0 \sigma = v$ and $d_1 \sigma = v$ (both faces are the basepoint). Two loops $\sigma$ and $\tau$ are *homotopic* (rel basepoint) if there is a 2-simplex $h \in X_2$ with $d_0 h = \tau$, $d_1 h = s_0 v$ (the degenerate edge at $v$), and $d_2 h = \sigma$ — a "homotopy from $\sigma$ to $\tau$ through $v$."

The set $\pi_1(X, v)$ of homotopy classes of loops is a group under the composition defined by inner horn-filling: given loops $\sigma$ and $\tau$ at $v$, the horn $\Lambda^2_1 \to X$ specifying edges $\sigma$ and $\tau$ can be filled (by the Kan condition) to give a 2-simplex $h$, and the third edge $d_1 h$ is the composition $\sigma \cdot \tau$.

Similarly, $\pi_n(X, v)$ is defined for all $n$ using $(n+1)$-simplices with appropriate boundary conditions. The Kan condition ensures these groups are well-defined.

**Theorem.** For a Kan complex $X$ and a vertex $v$, there is a natural isomorphism $\pi_n(X, v) \cong \pi_n(|X|, v)$ — the combinatorial homotopy groups of $X$ equal the classical homotopy groups of its geometric realization.

## Kan Complexes as $\infty$-Groupoids

The fundamental insight of simplicial homotopy theory is that Kan complexes are exactly the right combinatorial model for *$\infty$-groupoids* — structures in which:
- 0-morphisms are objects (vertices)
- 1-morphisms are morphisms between objects (edges)
- 2-morphisms are homotopies between morphisms (triangles)
- $n$-morphisms are $n$-fold homotopies ($n$-simplices)
- All $n$-morphisms for $n \geq 1$ are invertible (up to higher morphisms)

The Kan condition ensures invertibility at each level: inner horns give composition, outer horns give inverses. The higher Kan conditions ensure coherent associativity and all the coherence conditions of an $\infty$-groupoid.

This connection — Kan complexes = $\infty$-groupoids = homotopy types — is Grothendieck's homotopy hypothesis (now a theorem, in various formulations): the homotopy theory of spaces is equivalent to the homotopy theory of $\infty$-groupoids, and Kan complexes are the combinatorial models for both.

## Kan Fibrations

The Kan condition on a simplicial set can be generalized to a condition on a map between simplicial sets.

**Definition.** A map $p : E \to B$ of simplicial sets is a *Kan fibration* if for every horn inclusion $\Lambda^n_k \hookrightarrow \Delta[n]$, every commutative square:
$$\Lambda^n_k \xrightarrow{a} E$$
$$\downarrow \qquad \downarrow p$$
$$\Delta[n] \xrightarrow{b} B$$
has a lifting $\Delta[n] \to E$ making both triangles commute.

A simplicial set $X$ is a Kan complex iff $X \to *$ (the map to the terminal simplicial set) is a Kan fibration.

Kan fibrations are the fibrations in the Quillen model structure on simplicial sets (Section 4). They correspond to Serre fibrations of topological spaces under the adjunction $|-| \dashv \text{Sing}$: a map $p : E \to B$ is a Kan fibration iff $|p| : |E| \to |B|$ is a Serre fibration.

## The Path Space

For a Kan complex $X$ and two vertices $x, y \in X_0$, the *path space* $\text{Path}(X, x, y)$ is the simplicial set whose $n$-simplices are $(n+1)$-simplices $\sigma \in X_{n+1}$ with $d_0^{n+1} \sigma = s_0^n x$ (the source) and $d_1^{n+1} \sigma = s_0^n y$ (the target), where $d_0^{n+1}$ and $d_1^{n+1}$ are appropriate boundary operations.

The geometric realization of $\text{Path}(X, x, y)$ is homotopy equivalent to the classical path space $P(|X|, x, y)$.

The *loop space* $\Omega(X, v) = \text{Path}(X, v, v)$ is the Kan complex of loops at $v$. It is the combinatorial model of the loop space $\Omega(|X|, v)$, and $\pi_n(\Omega(X,v), c_v) \cong \pi_{n+1}(X, v)$ (where $c_v$ is the constant loop at $v$).

This combinatorial path space is the prototype for the identity type in HoTT: the type $a =_A b$ is exactly the path space of the type $A$ from $a$ to $b$.
