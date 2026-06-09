# 9.3 Anosov Diffeomorphisms

An Anosov diffeomorphism is a hyperbolic system with no "room to breathe" — the entire manifold is hyperbolic, with no neutral directions anywhere. Every tangent vector is either contracted or expanded, uniformly across the whole space.

**Definition 9.3.1.** A $C^1$ diffeomorphism $f: M \to M$ of a compact manifold is an *Anosov diffeomorphism* if $M$ itself is a hyperbolic set: for all $x \in M$, $T_xM = E^s(x) \oplus E^u(x)$ with uniform expansion in $E^u$ and contraction in $E^s$.

**Examples 9.3.2.**
- *Linear toral automorphisms*: $f_A: {\mathbb T}^n \to {\mathbb T}^n$ for $A \in GL(n, {\mathbb Z})$ with no eigenvalue on the unit circle. The splitting is constant: $E^s = $ span of eigenvectors with $|\lambda| < 1$, $E^u = $ span with $|\lambda| > 1$.
- The Arnold cat map $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix}$ on ${\mathbb T}^2$ with eigenvalues $\lambda_+ = (3+\sqrt{5})/2 > 1$ and $\lambda_- = (3-\sqrt{5})/2 < 1$.
- It is unknown whether Anosov diffeomorphisms exist on manifolds other than infranilmanifolds (a major open problem).

The linear toral automorphisms are the canonical examples. For the cat map $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix}$ on ${\mathbb T}^2$, the eigenvectors of $A$ are the stable and unstable directions, and these are constant across the whole torus (because the map is linear). The contraction and expansion rates are the eigenvalues $\lambda_\pm = (3 \pm \sqrt{5})/2$.

The open problem about which manifolds support Anosov diffeomorphisms is one of the central mysteries of the subject. It's known that the torus and more general "infranilmanifolds" support them; it's conjectured that nothing else does. But no proof exists.

**Theorem 9.3.3 (Anosov).** Every Anosov diffeomorphism is topologically transitive (has a dense orbit) and has a dense set of periodic points.

Anosov proved this in the 1960s. The key ingredients are the invariant foliations and a geometric argument showing that unstable manifolds are dense. The density of periodic orbits follows from the symbolic coding via Markov partitions (Section 9.5).

**Theorem 9.3.4.** For an Anosov diffeomorphism $f$:
1. The stable and unstable foliations $\mathcal{W}^s$, $\mathcal{W}^u$ are well-defined, $f$-invariant, and continuous (though generally not smooth as foliations).
2. $f$ is *structurally stable*: any $C^1$-perturbation $g$ of $f$ is topologically conjugate to $f$.

Structural stability is a striking property. Most dynamical systems change their qualitative behavior under perturbation — a small perturbation can create or destroy orbits, change the number of fixed points, alter the topology of the attractor. But Anosov diffeomorphisms are immune to this: any $C^1$-perturbation is topologically conjugate to the original. The orbit structure is rigid.

This rigidity doesn't mean the system is simple — Anosov diffeomorphisms are chaotic. It means their chaos is *stable*: you can't perturb it away.
