# Introduction to Manifolds

A smooth manifold is a space that locally looks like Euclidean space. The surface of a sphere is not itself $\mathbb{R}^2$ — it curves, it wraps around, it has no global coordinate system — but any small patch of the sphere is indistinguishable from a flat piece of $\mathbb{R}^2$. The implicit function theorem provides the rigorous foundation for this idea: the level set of a smooth map is a smooth manifold near any regular point. This section introduces manifolds in the concrete setting where they arise naturally in multivariable calculus — as subsets of $\mathbb{R}^n$ defined by smooth equations — and connects the abstract definition to the geometric intuition built throughout this unit.

## Level Sets as Manifolds

**Definition.** A subset $M\subseteq\mathbb{R}^n$ is a **smooth $k$-dimensional submanifold** of $\mathbb{R}^n$ if for every point $\mathbf{p}\in M$, there is an open neighborhood $U\ni\mathbf{p}$ in $\mathbb{R}^n$ and a smooth map $F: U\to\mathbb{R}^{n-k}$ such that:
1. $F(\mathbf{p}) = \mathbf{0}$.
2. $M\cap U = \{\mathbf{x}\in U: F(\mathbf{x}) = \mathbf{0}\}$.
3. $J_F(\mathbf{p})$ has rank $n-k$ (the Jacobian is surjective).

**Theorem (Regular Level Set Theorem).** Let $F: D\subseteq\mathbb{R}^n\to\mathbb{R}^m$ be $C^1$ with $m < n$. If $\mathbf{c}\in\mathbb{R}^m$ is a **regular value** of $F$ (meaning $J_F(\mathbf{p})$ has rank $m$ for every $\mathbf{p}\in F^{-1}(\mathbf{c})$), then $M = F^{-1}(\mathbf{c})$ is a smooth $(n-m)$-dimensional submanifold of $\mathbb{R}^n$.

**Proof.** At any $\mathbf{p}\in M$ with $J_F(\mathbf{p})$ of rank $m$, the implicit function theorem guarantees that locally, the equation $F(\mathbf{x})=\mathbf{c}$ can be solved for $m$ of the variables as $C^1$ functions of the remaining $n-m$ variables. This gives a local parameterization of $M$ near $\mathbf{p}$ by $n-m$ free parameters.

## Examples of Manifolds

**Sphere $S^{n-1}$:** $F(\mathbf{x}) = \|\mathbf{x}\|^2 - 1 = 0$. $J_F = 2\mathbf{x}^T$, which has rank 1 for all $\mathbf{x}\neq\mathbf{0}$. Every point of $S^{n-1}$ satisfies $\mathbf{x}\neq\mathbf{0}$, so $1$ is a regular value. The sphere $S^{n-1}$ is a smooth $(n-1)$-dimensional manifold.

**Torus:** The torus in $\mathbb{R}^3$ can be written as $F(x,y,z) = (\sqrt{x^2+y^2}-R)^2+z^2-r^2=0$ for $R>r>0$. The gradient $\nabla F$ is nonzero on the torus, so the torus is a smooth 2-dimensional manifold in $\mathbb{R}^3$.

**Curve as 1-manifold:** A smooth regular curve $\mathbf{r}(t)$ in $\mathbb{R}^3$ traces a 1-dimensional manifold in $\mathbb{R}^3$. Locally, near any point, the curve looks like a short segment of $\mathbb{R}$.

**Graph as $n$-manifold:** The graph of any smooth function $f:\mathbb{R}^n\to\mathbb{R}$, $\{(x,y): y=f(x)\}\subset\mathbb{R}^{n+1}$, is an $n$-dimensional manifold. The defining map is $F(x,y) = y-f(x)$, and $F_y = 1\neq 0$ always.

## Tangent Spaces

At each point $\mathbf{p}$ of a smooth manifold $M$, there is a **tangent space** $T_\mathbf{p}M$: the set of all tangent vectors to smooth curves in $M$ passing through $\mathbf{p}$.

**Theorem.** For $M = F^{-1}(\mathbf{0})\subset\mathbb{R}^n$, the tangent space at $\mathbf{p}$ is the kernel of $J_F(\mathbf{p})$:

$$T_\mathbf{p}M = \ker J_F(\mathbf{p}) = \{\mathbf{v}\in\mathbb{R}^n: J_F(\mathbf{p})\mathbf{v} = \mathbf{0}\}.$$

**Proof.** If $\boldsymbol{\gamma}(t)$ is a curve in $M$ with $\boldsymbol{\gamma}(0)=\mathbf{p}$, then $F(\boldsymbol{\gamma}(t))=\mathbf{0}$ for all $t$. Differentiating: $J_F(\mathbf{p})\boldsymbol{\gamma}'(0)=\mathbf{0}$. So every tangent vector lies in $\ker J_F(\mathbf{p})$. The converse — that every vector in $\ker J_F(\mathbf{p})$ is tangent to some curve in $M$ — follows from the implicit function theorem.

**Example.** For $M = S^{n-1}$, $F(\mathbf{x})=\|\mathbf{x}\|^2-1$, $J_F(\mathbf{p}) = 2\mathbf{p}^T$. Then $T_\mathbf{p}S^{n-1} = \ker(2\mathbf{p}^T) = \{\mathbf{v}: \mathbf{p}\cdot\mathbf{v}=0\}$, the orthogonal complement of $\mathbf{p}$. The tangent space to the sphere at $\mathbf{p}$ is the plane (or hyperplane) through $\mathbf{p}$ perpendicular to the radius — exactly the geometric intuition.

## Local Coordinates and Charts

A **chart** on a manifold $M$ at $\mathbf{p}$ is a smooth bijection $\phi: U\cap M\to V\subseteq\mathbb{R}^k$ (where $U$ is a neighborhood of $\mathbf{p}$ in $\mathbb{R}^n$ and $V$ is open in $\mathbb{R}^k$) with smooth inverse. The implicit function theorem guarantees the existence of such charts: it says that $k = n-m$ coordinates can be solved from the other $n-k$, providing a local coordinate system on $M$.

**Example.** On the sphere $S^2$ near the north pole $(0,0,1)$: the implicit function theorem solves $x^2+y^2+z^2=1$ for $z$ as a function of $(x,y)$ near $(0,0,1)$: $z=\sqrt{1-x^2-y^2}$. So $(x,y)$ serve as local coordinates near the north pole. This is stereographic projection (approximately).

## Why Manifolds Matter

Manifolds are the natural setting for physics and geometry. Configuration spaces of mechanical systems (the set of all possible positions) are manifolds. Phase space (positions and momenta) is a manifold. In general relativity, spacetime is a 4-dimensional Lorentzian manifold. Lie groups (smooth manifolds with group structure) are the symmetry groups of physics.

In the context of differential equations, solutions to autonomous ODEs $\dot{\mathbf{x}} = \mathbf{F}(\mathbf{x})$ trace curves on the manifold of initial conditions. Conservation laws define invariant submanifolds (level sets of conserved quantities). The study of how solutions behave on and near these manifolds — stable manifold theory, center manifold theory — is a major subject in the global theory of differential equations.

## Connection Forward

The language introduced here — manifolds, tangent spaces, charts — is the foundation for differential geometry, and it reappears throughout the course in the study of surfaces, vector fields, and differential forms. The key takeaway: the implicit function theorem shows that the "nice" subsets of $\mathbb{R}^n$ (those defined by regular level sets of smooth maps) are locally indistinguishable from flat Euclidean space, which makes calculus on them possible.
