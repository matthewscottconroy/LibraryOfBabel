# 5.1 Eilenberg-MacLane Spaces

## The Fundamental Building Blocks

Every topological space can be decomposed, via its Postnikov tower, into a sequence of "layers" — each layer capturing one homotopy group. The individual layers are the *Eilenberg-MacLane spaces* $K(G, n)$.

**Definition 5.1 (Eilenberg-MacLane space).** For a group $G$ (abelian if $n \geq 2$) and $n \geq 1$, the Eilenberg-MacLane space $K(G, n)$ is a connected type with:
$$\pi_k(K(G, n)) = \begin{cases} G & k = n \\ 0 & k \neq n, k \geq 1 \end{cases}$$

(and $\pi_0 = \mathbf{1}$, i.e., $K(G,n)$ is connected).

**Uniqueness.** $K(G, n)$ is unique up to homotopy equivalence (given $G$ and $n$). This uniqueness is the key property: there is exactly one "space" with the given homotopy group in dimension $n$ and trivial everywhere else.

**Why they matter.** Any space $X$ fits into a Postnikov tower:
$$X \to \cdots \to X[2] \to X[1] \to X[0]$$

where $X[n]$ is the $n$-truncation $\|X\|_n$ of $X$, and the fiber of $X[n] \to X[n-1]$ is a $K(\pi_n(X), n)$. The space $X$ is "built from" its Eilenberg-MacLane spaces via this tower.

## Constructing K(G, 1)

For $n = 1$, $K(G, 1)$ is the classifying space of $G$.

**The HIT construction.** Define $K(G, 1)$ as:
- Point constructor: $\mathsf{pt} : K(G, 1)$
- Path constructors: $g : \mathsf{pt} = \mathsf{pt}$ for each $g : G$ (one loop for each group element)
- Path coherences: $\mathsf{mul}(g, h) : g \cdot h = gg_h$ in the loop space, encoding the group multiplication
- Truncation: $K(G, 1)$ is a 1-type (so 2-paths are unique)

More precisely, the 2-path constructors say that the loops satisfy the group multiplication law. The 1-truncation ensures there are no interesting 2-paths (making $\pi_2 = 0$).

**The loop space.** By construction:
$$\Omega(K(G, 1), \mathsf{pt}) = (\mathsf{pt} = \mathsf{pt}) \simeq G$$

The loop space of $K(G, 1)$ at its basepoint is (equivalent to) $G$.

**Example: $K(\mathbb{Z}, 1) = S^1$.** The circle has $\pi_1 = \mathbb{Z}$ and $\pi_k = 0$ for $k \geq 2$ (a theorem in synthetic homotopy theory). So $S^1 = K(\mathbb{Z}, 1)$.

This is a fundamental identification: the circle is the "universal space for integer winding numbers."

**Example: $K(\mathbb{Z}/2\mathbb{Z}, 1) = \mathbb{RP}^\infty$.** The infinite real projective space has fundamental group $\mathbb{Z}/2\mathbb{Z}$ and trivial higher homotopy groups.

## Constructing K(G, n) for Higher n

For $n \geq 2$, $G$ must be abelian (since $\pi_n$ is abelian for $n \geq 2$ by Eckmann-Hilton), and $K(G, n)$ is constructed iteratively using the delooping operation.

**The delooping (= classifying space) operation.** Given a pointed connected type $X$ with $\Omega X \simeq G$, we say $X$ is a *delooping* or *classifying space* of $G$ (as a type, not as a group).

**Iterative construction:**
$$K(G, 0) :\equiv G \quad (\text{as a discrete type})$$
$$K(G, n+1) :\equiv BK(G, n) \quad (\text{the classifying space / delooping})$$

Starting from $G$ (a discrete type), each delooping raises the "relevant dimension" by 1.

**In HoTT notation:**
$$K(G, n) \simeq \Omega^n K(G, 0) \quad \text{(loop space)} \quad K(G, n) = B^n G \quad \text{(iterated classifying space)}$$

Wait — this reverses: $K(G, 0) = G$, and $K(G, n) = B^n G$ where $B$ is the delooping.

**The delooping in practice.** In HoTT, the delooping of a group $G$ can be constructed as a HIT:
- One point $* : BG$
- For each group element $g : G$: a loop $g : * = *$
- For each multiplication: a 2-path $\mathsf{mul}(g, h)$ saying $g \cdot h =$ (their concatenation)
- Truncation to 1-type

Then $B^n G = K(G, n)$ has exactly one non-trivial homotopy group in dimension $n$.

## K(G, n) and Cohomology

The main application of Eilenberg-MacLane spaces:

**Theorem 5.2 (Brown Representability, classical).** For any generalized cohomology theory $h^*$, there exist Eilenberg-MacLane spaces $E_n$ such that:
$$h^n(X) \cong [X, E_n]$$

(homotopy classes of maps from $X$ to $E_n$).

For ordinary cohomology with coefficients in an abelian group $G$:
$$H^n(X; G) \cong [X, K(G, n)]$$

**In HoTT:** This theorem can be stated and (partially) proved synthetically. Maps from $X$ to $K(G, n)$ correspond to $n$-dimensional cohomology classes of $X$ with coefficients in $G$.

**Cup products.** The cohomology ring structure (cup product $H^m \times H^n \to H^{m+n}$) corresponds to the *join product* on maps to Eilenberg-MacLane spaces:
$$[X, K(G, m)] \times [X, K(G, n)] \to [X, K(G, m+n)]$$

In HoTT, this can be defined using the smash product $X \wedge Y$ and the Eilenberg-MacLane space structure.

**Brunerie's computation.** The computation $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ by Brunerie uses the cohomology of $K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$ and cup products in a key way. The "Brunerie number" is essentially a cohomology class in $H^4(K(\mathbb{Z}, 2); \mathbb{Z}/2\mathbb{Z})$.

## Spectra and Stabilization

Eilenberg-MacLane spaces give the simplest examples of *spectra*:

**Definition 5.3 (Spectrum).** A spectrum $E$ is a sequence of pointed types $(E_n)_{n \geq 0}$ with equivalences:
$$E_n \simeq \Omega E_{n+1}$$

(each type is the loop space of the next).

**The Eilenberg-MacLane spectrum.** For an abelian group $G$, define:
$$HG_n = K(G, n)$$

The equivalences $K(G, n) \simeq \Omega K(G, n+1)$ give the Eilenberg-MacLane spectrum $HG$.

The cohomology theory represented by $HG$ is ordinary cohomology with coefficients in $G$:
$$H^n(X; G) = \pi_0(\mathsf{Maps}(X, K(G, n))) = [X, K(G, n)]$$

**The sphere spectrum.** The sphere spectrum $(S^n)_{n \geq 0}$ (with the stabilization maps $S^n \to \Omega S^{n+1}$) represents stable homotopy theory. Its homotopy groups are the stable homotopy groups of spheres $\pi_k^s = \lim_{n \to \infty} \pi_{n+k}(S^n)$.

Computing the stable homotopy groups of spheres is one of the central problems of algebraic topology — a problem that HoTT contributes to via synthetic methods.

## HITs as a Lingua Franca

Higher inductive types provide a uniform language for:

| Classical concept | HIT |
|---|---|
| $S^n$ | Suspension of $S^{n-1}$ |
| $K(G, 1)$ | One point plus group loops plus 1-truncation |
| $K(G, n)$ | Iterated delooping |
| Pushout | Two inclusions plus gluing paths |
| Truncation $\|A\|_n$ | Inclusion plus truncation constructors |
| Classifying space $BG$ | One point plus $G$-loops plus 1-truncation |
| Colimit | Point constructors for nodes, path constructors for edges |

HITs are the type-theoretic version of CW complexes (cell complexes) — you build spaces by attaching cells in increasing dimensions:
- 0-cells: point constructors
- 1-cells: path constructors
- 2-cells: 2-path constructors
- etc.

This is the deep connection: HITs are the type-theoretic CW complex construction, and the homotopy type of the resulting type is exactly the homotopy type of the corresponding CW complex.

## Looking Forward

The HITs introduced in this chapter — circles, spheres, pushouts, truncations, Eilenberg-MacLane spaces — form the toolkit for synthetic homotopy theory (Chapter 20). The key theorems:
- $\pi_1(S^1) = \mathbb{Z}$ (encode-decode on the circle HIT)
- Seifert-van Kampen (universal property of pushout HIT)
- Freudenthal suspension (suspension HIT + Blakers-Massey)
- Hopf fibration (join construction + $K(\mathbb{Z},1) = S^1$)

All of these are proved by working directly with the HIT constructors and eliminators, using path induction, transport, and the techniques developed throughout the book.
