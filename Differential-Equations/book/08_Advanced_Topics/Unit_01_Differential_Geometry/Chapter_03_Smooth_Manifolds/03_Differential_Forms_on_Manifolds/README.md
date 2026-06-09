# Differential Forms on Manifolds

Differential forms are the natural objects to integrate on manifolds. A $k$-form assigns to each point an alternating multilinear functional on the $k$-fold product of tangent spaces—exactly what is needed to integrate over $k$-dimensional submanifolds. The exterior derivative $d$ connects forms of different degrees, and the condition $d^2 = 0$ underlies the de Rham cohomology, which encodes global topological information. Stokes' theorem, in its manifold formulation, unifies all the classical theorems of vector calculus.

## Alternating Multilinear Forms

Recall: a $k$-linear map $\omega: V^k \to \mathbb{R}$ is **alternating** if $\omega(\ldots, v, \ldots, v, \ldots) = 0$ whenever two arguments coincide (equivalently, it changes sign when any two arguments are transposed). The space of alternating $k$-linear forms on $V$ is $\Lambda^k(V^*)$.

For a vector space of dimension $n$, $\dim \Lambda^k(V^*) = \binom{n}{k}$. In particular, $\Lambda^0(V^*) = \mathbb{R}$, $\Lambda^1(V^*) = V^*$, and $\Lambda^n(V^*) = \mathbb{R}$ (one-dimensional: the volume forms). $\Lambda^k(V^*) = 0$ for $k > n$.

If $\{e_1, \ldots, e_n\}$ is a basis for $V$ with dual basis $\{e^1, \ldots, e^n\}$, then $\{e^{i_1} \wedge e^{i_2} \wedge \cdots \wedge e^{i_k} : i_1 < i_2 < \cdots < i_k\}$ is a basis for $\Lambda^k(V^*)$, where the **wedge product** (exterior product) is:

$$(e^{i_1} \wedge \cdots \wedge e^{i_k})(v_1, \ldots, v_k) = \det\begin{pmatrix} e^{i_1}(v_1) & \cdots & e^{i_1}(v_k) \\ \vdots & & \vdots \\ e^{i_k}(v_1) & \cdots & e^{i_k}(v_k) \end{pmatrix}.$$

## Differential $k$-Forms

A **differential $k$-form** on a smooth manifold $M$ is a smooth section of $\Lambda^k(T^*M)$: an assignment $\omega: M \to \Lambda^k(T^*M)$ that is smooth. In a local chart $(U, x^1, \ldots, x^n)$:

$$\omega = \sum_{1 \leq i_1 < \cdots < i_k \leq n} \omega_{i_1 \cdots i_k}(x) \, dx^{i_1} \wedge \cdots \wedge dx^{i_k},$$

where $\omega_{i_1\cdots i_k}: U \to \mathbb{R}$ are smooth functions. The space of $k$-forms on $M$ is denoted $\Omega^k(M)$.

Special cases:
- $\Omega^0(M) = C^\infty(M)$ (smooth functions).
- $\Omega^1(M)$ (1-forms, or covector fields): in coordinates, $\omega = \sum_i f_i \, dx^i$.
- $\Omega^n(M)$ (top-degree forms): in coordinates, $\omega = f \, dx^1 \wedge \cdots \wedge dx^n$, used for integration.

## The Exterior Derivative

The **exterior derivative** $d: \Omega^k(M) \to \Omega^{k+1}(M)$ is the unique $\mathbb{R}$-linear map satisfying:
1. $df = \sum_i \frac{\partial f}{\partial x^i} dx^i$ for $f \in \Omega^0(M)$.
2. $d(\omega \wedge \eta) = d\omega \wedge \eta + (-1)^k \omega \wedge d\eta$ (graded Leibniz rule, where $k = \deg\omega$).
3. $d^2 = 0$ (nilpotency).

In coordinates, for $\omega = \sum_{I} \omega_I \, dx^I$ (where $I = (i_1, \ldots, i_k)$ is a multi-index):

$$d\omega = \sum_I \sum_j \frac{\partial \omega_I}{\partial x^j} dx^j \wedge dx^I.$$

**Example.** For $\omega = P \, dx + Q \, dy$ on $\mathbb{R}^2$:

$$d\omega = dP \wedge dx + dQ \wedge dy = \left(\frac{\partial P}{\partial y}dy\wedge dx + \frac{\partial Q}{\partial x}dx\wedge dy\right) = \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dx \wedge dy.$$

This is exactly the curl of $(P, Q)$.

## Connection to Vector Calculus

In $\mathbb{R}^3$, the exterior derivative gives:
- $d(f) = \nabla f \cdot d\mathbf{r}$ (gradient) for 0-forms.
- $d(P \, dx + Q \, dy + R \, dz) = (\nabla \times F) \cdot dA$ (curl) for 1-forms.
- $d(F_1 \, dy\wedge dz + F_2 \, dz\wedge dx + F_3 \, dx\wedge dy) = (\nabla \cdot F) \, dx\wedge dy\wedge dz$ (divergence) for 2-forms.

The identity $d^2 = 0$ encodes: $\nabla \times \nabla f = 0$ and $\nabla \cdot (\nabla \times F) = 0$.

## Closed and Exact Forms

A form $\omega$ is **closed** if $d\omega = 0$ and **exact** if $\omega = d\eta$ for some form $\eta$. Every exact form is closed ($d^2 = 0$), but not every closed form is exact on a manifold with nontrivial topology.

**Poincaré lemma.** On a contractible open set $U \subset \mathbb{R}^n$, every closed $k$-form ($k \geq 1$) is exact.

The failure of closed forms to be exact on a manifold is measured by the **de Rham cohomology**:

$$H^k_{dR}(M) = \frac{\ker(d: \Omega^k(M) \to \Omega^{k+1}(M))}{\text{im}(d: \Omega^{k-1}(M) \to \Omega^k(M))}.$$

**de Rham's theorem** states that $H^k_{dR}(M) \cong H^k(M; \mathbb{R})$ (singular cohomology with real coefficients). This isomorphism equates the analytic (differential forms) and topological (singular homology) theories, and is one of the foundational results in differential topology.

## Integration of Differential Forms

A top-degree form $\omega \in \Omega^n(M)$ on an oriented $n$-manifold $M$ can be integrated: in each chart $(U, \phi)$, $\omega|_U = f \, dx^1 \wedge \cdots \wedge dx^n$, and the integral is $\int_U f \circ \phi^{-1} \, d^n x$ (ordinary Lebesgue integral in $\mathbb{R}^n$). Orientation ensures consistency across overlapping charts (the Jacobian determinant is positive).

More generally, a $k$-form can be integrated over an oriented $k$-dimensional submanifold.

## Stokes' Theorem

**Theorem (Stokes).** Let $M$ be a compact oriented smooth $n$-manifold with boundary $\partial M$ (oriented by the induced orientation). For any $\omega \in \Omega^{n-1}(M)$:

$$\int_M d\omega = \int_{\partial M} \omega.$$

Special cases:
- $n = 1$, $M = [a,b]$: $\int_a^b f' \, dx = f(b) - f(a)$ (fundamental theorem of calculus).
- $n = 2$, $M$ is a region in $\mathbb{R}^2$: Green's theorem.
- $n = 3$, $M$ is a surface in $\mathbb{R}^3$: classical Stokes' theorem ($\int_S \nabla \times F \cdot d\mathbf{A} = \oint_{\partial S} F \cdot d\mathbf{r}$).
- $n = 3$, $M$ is a region in $\mathbf{R}^3$: divergence theorem.
- Gauss-Bonnet theorem is also a consequence (via the connection 1-form).

Stokes' theorem is the master identity of integration on manifolds, unifying an enormous range of classical results into a single conceptual statement.
