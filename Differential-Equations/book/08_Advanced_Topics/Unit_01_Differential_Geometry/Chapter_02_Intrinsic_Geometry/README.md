# Chapter 2: Intrinsic Geometry

The first fundamental form captures how a surface measures distances and angles from the inside. But surfaces embedded in $\mathbb{R}^3$ also bend and curve as they sit in the ambient space. The **second fundamental form** encodes this bending; from it one derives the principal curvatures and, most importantly, the Gaussian curvature. The revolutionary discovery—Gauss's Theorema Egregium—is that the Gaussian curvature is actually intrinsic: it can be computed from the first fundamental form alone, without any knowledge of the embedding. This chapter develops these ideas and culminates in the Gauss-Bonnet theorem, which connects local curvature to global topology.

## The Second Fundamental Form

The second fundamental form measures how a surface deviates from its tangent plane as one moves along the surface. For an orientable surface with unit normal $\hat{N}$, define the **shape operator** (Weingarten map) $S: T_pS \to T_pS$ by:

$$S(\mathbf{v}) = -D_{\mathbf{v}} \hat{N},$$

the negative of the covariant derivative of the normal field in direction $\mathbf{v}$. The shape operator is self-adjoint with respect to the first fundamental form. The **second fundamental form** is:

$$II(\mathbf{u}, \mathbf{v}) = I(S(\mathbf{u}), \mathbf{v}).$$

In coordinates: $II = L \, du^2 + 2M \, du \, dv + N \, dv^2$ where $L = \hat{N} \cdot \mathbf{r}_{uu}$, $M = \hat{N} \cdot \mathbf{r}_{uv}$, $N = \hat{N} \cdot \mathbf{r}_{vv}$.

## Principal Curvatures and Gaussian Curvature

Since $S$ is self-adjoint, it has two real eigenvalues $\kappa_1, \kappa_2$ (the **principal curvatures**) with orthogonal eigenvectors (the **principal directions**). The maximum and minimum of the normal curvature $\kappa_n(\mathbf{v}) = II(\mathbf{v}, \mathbf{v})/I(\mathbf{v}, \mathbf{v})$ over unit tangent vectors are exactly $\kappa_1$ and $\kappa_2$.

The **Gaussian curvature** is $K = \kappa_1 \kappa_2 = \det(h)/\det(g)$, where $h = \begin{pmatrix} L & M \\ M & N \end{pmatrix}$ and $g = \begin{pmatrix} E & F \\ F & G \end{pmatrix}$. The **mean curvature** is $H = (\kappa_1 + \kappa_2)/2 = \text{tr}(S)/2$.

## Theorema Egregium and Gauss-Bonnet

The central result of Chapter 2 is:

**Theorem (Gauss, Theorema Egregium).** The Gaussian curvature $K$ is an intrinsic invariant: it can be expressed entirely in terms of the coefficients $E, F, G$ of the first fundamental form and their derivatives.

The proof exhibits an explicit (if complicated) formula for $K$ in terms of $E, F, G$. In conformal coordinates ($F = 0$, $E = G = e^{2\phi}$), the formula simplifies to $K = -e^{-2\phi}\Delta\phi$.

**Theorem (Gauss-Bonnet, Local).** For a region $R$ with smooth boundary $\partial R$ and external angles $\theta_i$ at any corners:

$$\iint_R K \, dA + \int_{\partial R} \kappa_g \, ds + \sum_i \theta_i = 2\pi,$$

where $\kappa_g$ is the geodesic curvature of $\partial R$.

**Theorem (Gauss-Bonnet, Global).** For a compact orientable surface $S$ without boundary:

$$\iint_S K \, dA = 2\pi\chi(S),$$

where $\chi(S)$ is the Euler characteristic. This remarkable identity shows that the total curvature is a topological invariant.

## Chapter Structure

Section 1 develops the second fundamental form and the shape operator in detail, with examples. Section 2 treats Gaussian and mean curvature, including the classification of surfaces by sign of $K$. Section 3 proves the Theorema Egregium. Section 4 proves the Gauss-Bonnet theorem, first locally (using geodesic triangles) and then globally by triangulation.
