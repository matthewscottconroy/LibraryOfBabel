# 25.3 Differential Geometry in Cohesive HoTT

## The Revolution in Language

Classical differential geometry is written in the language of coordinates, transition functions, and local charts. A manifold is defined by an atlas — a collection of local homeomorphisms to $\mathbb{R}^n$ that overlap compatibly. A differential form is defined by giving component functions in each coordinate chart and verifying that they transform correctly under coordinate changes. A connection is given by Christoffel symbols or connection 1-forms in each chart.

This language works. The results it produces — Stokes' theorem, the Gauss-Bonnet theorem, the Chern-Weil theory of characteristic classes — are among the deepest in mathematics. But the language is ugly. It obscures the geometry behind a notation designed for calculation rather than understanding.

In cohesive HoTT, the language changes entirely. A smooth manifold is a type. A differential form is a map to a specific cohesive type. A connection is a map satisfying a condition stated using the flat modality. The theorems of differential geometry become propositions in the type theory, and their proofs become functions. No coordinates. No charts. No transition functions.

This is not merely aesthetic. The coordinate-free presentation makes the invariance properties of differential geometry *automatic*: a statement about a manifold $M$ that is expressed using only the cohesive type structure of $M$ is automatically coordinate-invariant, because the coordinate system is not part of the type-theoretic description.

## Smooth Manifolds as Cohesive Types

In cohesive HoTT, we do not define manifolds by atlases. Instead, we assume that certain types (the "smooth manifolds") already exist in our cohesive type theory, and we specify what cohesive structure they have.

Concretely, we work with types that arise from the *smooth sets* model of cohesive HoTT. In this model:
- The type $\mathbb{R}$ is the real line with its standard smooth structure
- Products, function types, and subsets of smooth types are smooth
- Maps between smooth types are smooth functions

A smooth manifold $M$ is a cohesive type such that:
- Every point has a neighborhood equivalent to an open subset of $\mathbb{R}^n$
- The smooth structure is compatible with this local structure

This is still somewhat indirect, but the point is: once we have the real line $\mathbb{R}$ as a cohesive type (with $\int \mathbb{R} \simeq \mathbf{1}$ by real cohesion), everything else follows from the type-theoretic operations.

## Locally Constant Functions and the Flat Modality

The *locally constant* functions on a smooth manifold $M$ with values in a type $V$ are exactly the functions that factor through the flat modality:

$$\mathsf{Loc}(M, V) :\equiv \{ f : M \to V \mid f \text{ factors through } \flat V \}$$

More precisely, a function $f : M \to V$ is locally constant iff it lifts to a map $g : M \to \flat V$ such that $\varepsilon^\flat \circ g = f$:

$$M \xrightarrow{g} \flat V \xrightarrow{\varepsilon^\flat} V$$

**Why this works**: A locally constant function on $M$ sends each connected component of $M$ to a single value. The flat modality $\flat V$ captures exactly the discrete aspects of $V$ — the values, without paths between them. A map $g : M \to \flat V$ is a locally constant function to $V$: it assigns a discrete value to each connected component.

This is the cohesive account of locally constant functions: not defined by "constant on neighborhoods" (a condition involving charts), but defined by the factorization through $\flat V$ (a condition stated using modalities).

## Differential Forms Synthetically

The key insight for defining differential forms synthetically is to use the flat modality to capture the "linearization" of maps.

**Informally**: A $p$-form on $M$ is a smooth alternating multilinear map on tangent vectors. In cohesive HoTT, tangent vectors should be captured by infinitesimal directions — maps from "infinitesimal intervals" — and differential forms should be maps that are "locally linear."

**The synthetic approach** (following Schreiber's program):

**0-forms** (smooth functions): $\Omega^0(M) :\equiv (M \to \mathbb{R})$ — smooth functions from $M$ to $\mathbb{R}$.

**1-forms**: $\Omega^1(M) :\equiv (M \to \flat \mathbb{R}) \to (M \to \mathbb{R})$ — not quite, but the idea: a 1-form is something that takes a tangent vector (a path up to first order) and gives a real number.

More precisely, using the *deRham stack* approach: define the deRham stack $M_{\mathsf{dR}} :\equiv \int M$ (the shape of $M$) — but this is the *topological* deRham, not the smooth one. For the smooth deRham theory, we need a different construction.

**The precise definition** uses the *jet bundle* approach in cohesive HoTT:
- The *infinitesimal interval* $D$ is defined using the nilsquare property: $D = \{x : \mathbb{R} \mid x^2 = 0\}$ (in appropriate models, this is non-empty)
- A *tangent vector* at $p : M$ is a map $v : D \to M$ with $v(0) = p$
- A 1-form on $M$ is a smooth map $\omega : TM \to \mathbb{R}$ (where $TM = \Sigma_{p:M} (D \to M)_p$) satisfying linearity

For differential forms via the flat modality specifically:

**de Rham complex via flat sections**: The $n$-th de Rham cohomology is captured by:
$$H^n_{\mathsf{dR}}(M) :\equiv \pi_n(\mathsf{Map}(M, \flat B\mathbb{R}))$$

where $\flat B\mathbb{R}$ is the flat classifying space of the circle. This is the cohesive definition of de Rham cohomology: $n$-forms modulo exact forms are classified by maps to $\flat B\mathbb{R}$.

**The de Rham theorem in cohesive HoTT**:
$$H^n_{\mathsf{dR}}(M) \simeq H^n(\int M, \mathbb{R})$$

The de Rham cohomology of $M$ depends only on its shape $\int M$ — the homotopy invariants. This is the de Rham theorem, stated as a consequence of the cohesion axioms: the shape modality exactly captures the de Rham-relevant information.

## The Poincaré Lemma Synthetically

**Theorem (Poincaré Lemma in cohesive HoTT).** For any cohesively contractible space $M$ (i.e., $\int M \simeq \mathbf{1}$):
$$H^n_{\mathsf{dR}}(M) = 0 \text{ for } n > 0$$

*Proof.* Since $\int M \simeq \mathbf{1}$, the de Rham theorem gives:
$$H^n_{\mathsf{dR}}(M) \simeq H^n(\int M, \mathbb{R}) \simeq H^n(\mathbf{1}, \mathbb{R}) = 0 \text{ for } n > 0$$

This is the Poincaré lemma: all closed forms on a contractible space are exact. The proof is immediate from the de Rham theorem and the cohesion axiom $\int \mathbb{R}^n \simeq \mathbf{1}$. No calculation with explicit primitives or potentials required. $\square$

## The de Rham Differential

The *de Rham differential* $d : \Omega^n(M) \to \Omega^{n+1}(M)$ is the map that differentiates forms. In the synthetic setting:

**Using the flat modality**: The de Rham differential arises from the counit $\varepsilon^\flat : \flat A \to A$ and the infinitesimal structure. For a 0-form $f : M \to \mathbb{R}$ (a smooth function), the differential $df$ measures how $f$ changes in each direction.

In the *Dubuc-Kock-Lawvere* approach (synthetic differential geometry inside cohesive HoTT):
- The *Weil algebra* of $\mathbb{R}$ is the ring $\mathbb{R}[\epsilon]/(\epsilon^2)$ (the dual numbers)
- A tangent vector at $p : M$ is a $\mathbb{R}$-algebra map $T_p M = \mathsf{Der}_{\mathbb{R}}(C^\infty(M)_p, \mathbb{R})$
- The differential $df : TM \to \mathbb{R}$ sends a tangent vector $v$ to $v(f)$

The key axiom of synthetic differential geometry — the *Kock-Lawvere axiom* — says: every map $D \to \mathbb{R}$ (where $D = \{x \mid x^2 = 0\}$) is of the form $x \mapsto a + bx$ for unique $a, b : \mathbb{R}$.

In cohesive HoTT, the Kock-Lawvere axiom holds in the smooth sets model, making the differential synthetically definable.

## Connections and Parallel Transport

A *connection* on a bundle $P \to M$ is a way to define "parallel transport" — a rule for moving elements of the fiber of $P$ along paths in $M$ consistently.

In cohesive HoTT, connections on a trivial bundle $M \times G \to M$ (a principal $G$-bundle) are captured by maps:

$$\nabla : P \to \flat P$$

such that $\varepsilon^\flat \circ \nabla = \mathsf{id}_P$ (a section of the flat modality applied to $P$). This is the *flat connection* on $P$ — the connection for which parallel transport is trivial.

A general connection is a map that differs from the flat map by a "connection form" — a Lie algebra-valued 1-form:
$$A \in \Omega^1(M, \mathfrak{g})$$

The holonomy of the connection along a path $\gamma : [0,1] \to M$ is the element of $G$ obtained by integrating the connection form along $\gamma$. In cohesive HoTT, this is the image of $\gamma$ under the parallel transport map.

**The curvature** of a connection $A$ is the 2-form $F_A = dA + \frac{1}{2}[A, A]$. A connection is *flat* (has trivial holonomy around contractible loops) iff $F_A = 0$.

In cohesive type theory, flat connections are literally maps to $\flat P$ — the flat modality captures the notion of "flatness" definitionally.

## The Cohomology Triangle

The fundamental relationship in differential cohomology is a "cohomology triangle" connecting three invariants of a smooth space $M$:

$$\begin{array}{ccc}
\Omega^n_{\text{closed}}(M) & \to & \hat{H}^n(M, \mathbb{Z}) \\
\downarrow & & \downarrow \\
H^n(M, \mathbb{R}) & \leftarrow & H^n(M, \mathbb{Z})
\end{array}$$

- $\Omega^n_{\text{closed}}(M)$: closed $n$-forms on $M$ (de Rham data)
- $H^n(M, \mathbb{Z})$: integral cohomology (topological data)
- $H^n(M, \mathbb{R})$: real cohomology (de Rham cohomology)
- $\hat{H}^n(M, \mathbb{Z})$: differential cohomology (combines both)

In cohesive HoTT:
$$\hat{H}^n(M, \mathbb{Z}) :\equiv H^n(\flat M, \mathbb{Z}) \times_{H^n(\int M, \mathbb{Z})} H^n_{\mathsf{dR}}(M)$$

The pullback expresses the compatibility between the flat part (integral cohomology, from the discrete shadow of $M$) and the de Rham part (cohomology of smooth forms), meeting in the real cohomology.

This triangle is not a theorem in cohesive HoTT — it is the *definition* of differential cohomology in the cohesive setting. The fact that it correctly captures the classical notion is a consequence of the cohesion axioms and the de Rham theorem.

## Why This Matters: Geometry Without Coordinates

The synthesis of differential geometry within cohesive HoTT achieves something that coordinate geometry cannot: it separates what is *intrinsic* to a space from what is an artifact of the coordinate presentation.

In coordinate geometry, proving a result is coordinate-independent requires an explicit calculation showing that the result is unchanged under coordinate transformations. This can be tedious and error-prone.

In cohesive HoTT, there are no coordinates in the language. Every type-theoretic construction is automatically invariant. The differential, the curvature, the de Rham complex — all of these are defined using the modalities and the type structure, not using charts or transition functions. Coordinate independence is not a theorem to prove; it is a consequence of the language.

This is the same move that was made in abstract algebra: instead of defining a group as a set of matrices with multiplication and inversion, you define it abstractly as a set with an operation satisfying axioms. The abstract definition is coordinate-free, and all theorems about groups proved abstractly are automatically valid for any concrete representation. Cohesive HoTT makes differential geometry coordinate-free in the same way.
