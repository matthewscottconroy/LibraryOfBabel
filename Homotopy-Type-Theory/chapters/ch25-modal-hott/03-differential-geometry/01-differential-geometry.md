# 3.1 Synthetic Differential Geometry via Cohesion

## The Goal

Classical differential geometry studies smooth manifolds using coordinates, differential forms, connections, curvature. These constructions are powerful but technically intensive: they require charts, transition functions, compatibility conditions.

The synthetic approach to differential geometry — pioneered by Lawvere, extended by Kock, and formalized in cohesive HoTT by Schreiber — replaces this machinery with axioms. Instead of building differential geometry from scratch, we axiomatize the key properties and derive the classical constructions synthetically.

In cohesive HoTT, smooth types are just types with cohesive structure. Differential forms are elements of specific cohesive types. Connections and curvature follow from the cohesion axioms.

## Differential Forms

In classical differential geometry, a *differential $n$-form* on a manifold $M$ is a smooth antisymmetric multilinear map $\omega : TM^n \to \mathbb{R}$, where $TM$ is the tangent bundle.

In cohesive HoTT, the tangent space is defined using the infinitesimal interval:

**The infinitesimal interval $D$.** In the synthetic setting (following Kock-Lawvere synthetic differential geometry), the infinitesimal interval $D$ is the subtype of $\mathbb{R}$ satisfying $x^2 = 0$:

$$D :\equiv \{ x : \mathbb{R} \mid x^2 = 0 \}$$

**Tangent vectors.** A *tangent vector* at $a : M$ is a map $D \to M$ sending $0$ to $a$. The tangent space is:

$$T_a M :\equiv \{ v : D \to M \mid v(0) = a \}$$

**Differential forms.** A *differential 1-form* on $M$ is a function $\omega : M \to (TM \to \mathbb{R})$ — for each point and each tangent vector, a real number.

This is the beginning of synthetic differential geometry. In cohesive HoTT, the infinitesimal interval $D$ is defined using the *infinitesimal shape modality* $\Im$ (part of differential cohesion — a refinement of cohesive HoTT with infinitesimal structure).

## The De Rham Complex

The *de Rham complex* of a smooth type $A$ is a sequence of types:

$$\Omega^0(A) \xrightarrow{d} \Omega^1(A) \xrightarrow{d} \Omega^2(A) \xrightarrow{d} \cdots$$

where $\Omega^n(A)$ is the type of $n$-forms and $d$ is the exterior derivative.

**In cohesive HoTT:**
- $\Omega^0(A) = A \to \mathbb{R}$ (smooth functions)
- $\Omega^1(A) = A \to (TA \to \mathbb{R})$ (1-forms)
- The exterior derivative $d : \Omega^n(A) \to \Omega^{n+1}(A)$ is defined using the infinitesimal structure

**The de Rham cohomology:**
$$H^n_{dR}(A) :\equiv \ker(d : \Omega^n \to \Omega^{n+1}) / \mathsf{im}(d : \Omega^{n-1} \to \Omega^n)$$

**The de Rham theorem (synthetic):** From the cohesion axioms:
$$H^n_{dR}(A) \simeq H^n(\int A, \mathbb{R})$$

The de Rham cohomology depends only on the shape $\int A$ — the underlying homotopy type — and equals the ordinary singular cohomology with real coefficients.

## Connections on Principal Bundles

A *connection* on a principal $G$-bundle $P \to M$ is a choice of "how to lift paths from $M$ to $P$" — a splitting of the bundle projection at the tangent space level.

In cohesive HoTT, this is elegantly captured by the modalities.

**Principal bundles.** A *principal $G$-bundle on $M$* is a map $M \to \mathbf{B}G$ where $\mathbf{B}G$ is the classifying type of $G$-bundles (the HIT with one point and one path for each element of $G$).

**Bundles with connection.** The classifying type for *principal $G$-bundles with connection* is $\mathbf{B}G_\nabla$ — a refinement of $\mathbf{B}G$ that remembers the connection data.

A principal bundle with connection is a map $M \to \mathbf{B}G_\nabla$.

**The forgetful map.** There is a map $\mathbf{B}G_\nabla \to \mathbf{B}G$ forgetting the connection. A bundle with connection $M \to \mathbf{B}G_\nabla$ maps to a bundle $M \to \mathbf{B}G$ by this forgetful map.

**Flat connections.** A connection is *flat* if it factors through the flat modality:
$$M \to \mathbf{B}G_\nabla \text{ is flat} \iff \text{the map factors through } \flat \mathbf{B}G$$

Flat connections have trivial holonomy — their curvature vanishes.

## Curvature

The *curvature* of a connection $\nabla : M \to \mathbf{B}G_\nabla$ is a differential 2-form $F_\nabla : M \to \Omega^2_G$ measuring the failure of $\nabla$ to be flat.

In cohesive HoTT, the curvature appears as the obstruction to lifting $\nabla$ through $\flat \mathbf{B}G$:

$$\begin{array}{ccc}
& & \flat \mathbf{B}G \\
& & \downarrow \\
M & \xrightarrow{\nabla} & \mathbf{B}G_\nabla \xrightarrow{curv} \Omega^2_G
\end{array}$$

The curvature map $\mathsf{curv} : \mathbf{B}G_\nabla \to \Omega^2_G$ extracts the curvature from a connection.

**Chern-Weil theory.** The Chern classes of a principal bundle are computed from the curvature via Chern-Weil theory. In cohesive HoTT, this becomes a natural transformation:

$$\mathsf{ch} : (M \to \mathbf{B}G_\nabla) \to \prod_n H^{2n}_{dR}(M)$$

**Theorem (Chern-Weil, synthetic).** For any principal $G$-bundle with connection on $M$, the Chern classes $\mathsf{ch}(\nabla) \in H^{2n}_{dR}(M)$ depend only on the underlying bundle (not the choice of connection).

The proof: the Chern classes are in the image of $H^{2n}(M, \mathbb{Z}) \to H^{2n}_{dR}(M)$ (they're integral classes), and by the de Rham theorem, $H^{2n}_{dR}(M) \simeq H^{2n}(\int M, \mathbb{R})$ depends only on the topology.

## Hodge Theory

The Hodge theorem, in classical geometry: on a compact Riemannian manifold, every cohomology class has a unique harmonic representative.

In cohesive HoTT with a Riemannian structure (which adds a metric to the cohesive structure):
- A harmonic form is a $d$-closed and $d^*$-closed form (where $d^*$ is the Hodge dual of $d$)
- The Hodge theorem says: $H^n_{dR}(M) \simeq \ker(d) \cap \ker(d^*) = \mathsf{Harm}^n(M)$

This is currently beyond what is fully formalized in cohesive HoTT, but it is within the scope of the theory.

## Differential Cohesion: Infinitesimals

*Differential cohesion* extends cohesive HoTT with additional modalities capturing infinitesimal structure:

**The infinitesimal shape modality $\Im$.** $\Im A$ is $A$ with all paths collapsed to be infinitesimally close. An element of $\Im A$ is an equivalence class of elements of $A$ under "infinitesimal proximity."

**The de Rham stack modality $\mathcal{R}$.** $\mathcal{R} A$ is $A$ with all infinitesimal paths identified. This is the "de Rham space" in the sense of algebraic geometry.

**The jet monad.** For a smooth map $f : E \to M$, the $\infty$-jet bundle $J^\infty E$ is the type of formal Taylor series of sections of $E$. A section of $J^\infty E$ is a formal solution to a PDE on $E$.

In differential cohesion:
$$J^\infty E :\equiv (x : M) \to \Im_x(E)$$
where $\Im_x(E)$ is the infinitesimal neighborhood of the fiber over $x$.

## The Comparison: Classical vs. Cohesive

| Classical | Cohesive HoTT |
|-----------|---------------|
| Smooth manifold $M$ | A cohesive type $M$ |
| Tangent vector at $x$ | Map $D \to M$ sending $0 \mapsto x$ |
| Differential $k$-form | Element of $\Omega^k(M)$ |
| Exterior derivative $d$ | Defined from infinitesimal structure |
| De Rham cohomology | $H^n_{dR}(M) \simeq H^n(\int M, \mathbb{R})$ |
| Principal bundle | Map $M \to \mathbf{B}G$ |
| Connection | Map $M \to \mathbf{B}G_\nabla$ |
| Flat connection | Connection factoring through $\flat \mathbf{B}G$ |
| Curvature | Natural transformation $\mathbf{B}G_\nabla \to \Omega^2_G$ |
| Chern-Weil homomorphism | Natural map from connections to de Rham classes |

The cohesive approach doesn't just translate classical geometry — it *clarifies* it. The distinction between flat and non-flat connections, the relationship between cohomology and de Rham cohomology, the role of the shape modality — all of these become structurally clear in cohesive HoTT.
