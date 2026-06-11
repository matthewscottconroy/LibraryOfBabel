# Unit Overview: Advanced Topics in Differential Equations

## Why the Foundations Need Foundations

The techniques developed in the preceding units — separation of variables, eigenfunction expansions, the method of characteristics, numerical methods — are powerful and broadly applicable. But they rest on implicit assumptions that, when examined carefully, turn out to require significant mathematical infrastructure to justify. When does a series of eigenfunctions converge to the function it purports to represent? When does a variational problem ("minimize the energy functional") have a solution? When can one differentiate an integral with respect to a parameter? What does it mean for a function with a jump discontinuity to "satisfy" a PDE?

These questions are not pedantic. They arise directly in the study of PDEs, and their answers determine whether the formal methods of the preceding units are valid, and under what conditions. The three units in this advanced module supply the infrastructure: differential geometry (for equations on curved spaces), distribution theory (for non-smooth solutions), and functional analysis (for the abstract framework that makes existence and uniqueness theory precise).

Beyond their role as justification, these theories are productive in their own right. Distribution theory gives a rigorous meaning to the Dirac delta function and fundamental solutions, opening the way to the theory of Green's functions in full generality. Sobolev spaces and the Lax-Milgram theorem provide the foundation for the finite element method — the most widely used numerical method for PDEs in engineering. Differential geometry and the Laplace-Beltrami operator allow one to formulate and solve the heat equation, wave equation, and Schrödinger equation on curved spaces, including the spacetimes of general relativity.

## Differential Geometry: Equations on Curved Spaces

**Smooth Manifolds.** A smooth $n$-dimensional manifold $M$ is a topological space that is locally homeomorphic to $\mathbb{R}^n$, with smooth ($C^\infty$) transition maps between overlapping coordinate patches. Examples: spheres $S^n$, tori, Lie groups. The curvature of a manifold is an intrinsic property, independent of any embedding in higher-dimensional Euclidean space.

**Tangent Vectors and Differential Forms.** At each point $p \in M$, the tangent space $T_pM$ is an $n$-dimensional vector space of "directional derivatives" (in the abstract sense). The cotangent space $T_p^*M$ is its dual; differential $k$-forms are smooth sections of the $k$-th exterior power of the cotangent bundle.

**Exterior Derivative and Stokes' Theorem.** The exterior derivative $d : \Omega^k(M) \to \Omega^{k+1}(M)$ satisfies $d^2 = 0$ (the fundamental identity). The **Generalized Stokes' Theorem** for a smooth oriented manifold with boundary:
$$\int_M d\omega = \int_{\partial M} \omega.$$
This single formula encodes all the classical integral theorems (Green, Stokes, Divergence) as special cases by choosing $M$ and $\omega$ appropriately.

**Riemannian Geometry.** A Riemannian metric $g$ on $M$ assigns to each $p$ an inner product $g_p : T_pM \times T_pM \to \mathbb{R}$, varying smoothly. The metric determines distances, angles, volumes, and the notion of geodesic (length-minimizing curves). The **Levi-Civita connection** $\nabla$ is the unique torsion-free metric-compatible connection; it provides the notion of parallel transport and covariant derivative.

**Laplace-Beltrami Operator.** The Laplacian $\Delta$ on Euclidean space generalizes to the Laplace-Beltrami operator on a Riemannian manifold:
$$\Delta_g u = \frac{1}{\sqrt{\det g}}\sum_{i,j}\frac{\partial}{\partial x^i}\!\left(\sqrt{\det g}\,g^{ij}\frac{\partial u}{\partial x^j}\right).$$
On a Riemannian manifold $(M,g)$, the heat equation is $u_t = \Delta_g u$ and the wave equation is $u_{tt} = \Delta_g u$. The spectrum of $-\Delta_g$ (its eigenvalues and eigenfunctions) encodes geometric information about $M$: the famous question "Can you hear the shape of a drum?" asks whether two non-isometric Riemannian manifolds can have the same Laplace-Beltrami spectrum.

**Curvature.** The Riemann curvature tensor $R^i_{jkl}$ measures the non-commutativity of covariant differentiation: $[\nabla_k, \nabla_l]V^i = R^i_{jkl}V^j$. The Gauss-Bonnet theorem in 2D:
$$\int_M K\,dA + \int_{\partial M} \kappa_g\,ds = 2\pi\chi(M)$$
relates the Gaussian curvature $K$, geodesic curvature $\kappa_g$, and Euler characteristic $\chi(M)$ — a global topological invariant. This theorem is a striking example of how analysis and topology are linked via geometry.

## Distributions and Generalized Functions

Classical analysis requires functions to be differentiable in order to be differentiated. But many functions arising naturally — the Heaviside step function $H(x)$, solutions to PDEs with discontinuous data, fundamental solutions — are not differentiable in the classical sense. The theory of distributions (Laurent Schwartz, 1945–50) resolves this by extending the notion of differentiation.

**Test Functions.** The space $\mathcal{D}(\Omega) = C_c^\infty(\Omega)$ consists of smooth functions with compact support in $\Omega$. These are the "test functions": they can be differentiated arbitrarily many times and are zero outside a compact set.

**Distributions.** A distribution on $\Omega$ is a continuous linear functional $T : \mathcal{D}(\Omega) \to \mathbb{R}$. The value $T[\varphi] = \langle T, \varphi\rangle$ is the "pairing" of $T$ with a test function $\varphi$. Every locally integrable function $f$ defines a distribution $T_f[\varphi] = \int f\varphi\,dx$.

**Differentiation of Distributions.** The derivative $T'$ is defined by $T'[\varphi] = -T[\varphi']$. This is motivated by integration by parts: $\int f'\varphi = -\int f\varphi'$ when $f$ is smooth. The definition extends differentiation to all distributions, automatically.

**Theorem.** Every distribution has a derivative of every order, and differentiation is a continuous operation on distributions.

**The Dirac Delta.** $\delta(x)$ is the distribution $\delta[\varphi] = \varphi(0)$. It has derivatives: $\delta'[\varphi] = -\varphi'(0)$, $\delta^{(n)}[\varphi] = (-1)^n\varphi^{(n)}(0)$. The identity $x\delta(x) = 0$ holds in the distributional sense. The Dirac delta is not a function; it is a measure (in fact, the point mass at the origin), but the distributional framework treats it on the same footing as functions.

**Fundamental Solutions.** A fundamental solution (Green's function for the whole space) for a differential operator $L$ is a distribution $E$ satisfying $LE = \delta$. If $E$ is known, the solution to $Lu = f$ is $u = E * f$ (distributional convolution). Examples:
- For $L = -d^2/dx^2$ on $\mathbb{R}$: $E(x) = x/2$ (checked: $-E'' = -\delta/2 \cdot 2 = -\delta$... more carefully: $E(x) = |x|/2$, so $E'(x) = \text{sgn}(x)/2$, $E''(x) = \delta(x)$, so $-\Delta E = -\delta$... let $E(x) = -|x|/2$).
- For $L = -\Delta$ on $\mathbb{R}^3$: $E(\mathbf{x}) = 1/(4\pi|\mathbf{x}|)$.
- For $L = \partial_t - \kappa\partial_{xx}$ (heat operator): $E(x,t) = K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$ for $t > 0$, extended by zero for $t \leq 0$.

**Tempered Distributions and Fourier Transform.** The Schwartz space $\mathcal{S}(\mathbb{R}^n)$ consists of smooth functions that, together with all their derivatives, decay faster than any polynomial. Its dual $\mathcal{S}'(\mathbb{R}^n)$ is the space of tempered distributions. The Fourier transform extends to a continuous bijection on $\mathcal{S}'$: $\hat{\delta} = 1$ (a constant function), consistent with the interpretation that a point mass has a flat spectrum.

## Functional Analysis for PDEs

Functional analysis is, at its core, infinite-dimensional linear algebra: the study of vector spaces of functions together with the linear operators acting on them.

**Banach Spaces.** A Banach space is a complete normed vector space. Examples: $C[a,b]$ with sup norm; $L^p(\Omega)$ for $1 \leq p \leq \infty$; Sobolev spaces $W^{k,p}(\Omega)$.

**Hilbert Spaces.** A Hilbert space is a complete inner product space; its geometry is closest to $\mathbb{R}^n$. Examples: $L^2(\Omega)$, Sobolev space $H^k(\Omega) = W^{k,2}(\Omega)$. In a Hilbert space, the Projection Theorem holds: every closed subspace has an orthogonal complement, and every element has a unique best approximation in any closed convex set.

**Sobolev Spaces.** For $k \geq 0$ an integer and $1 \leq p < \infty$:
$$W^{k,p}(\Omega) = \{u \in L^p(\Omega) : D^\alpha u \in L^p(\Omega) \text{ for all } |\alpha| \leq k\}$$
with norm $\|u\|_{W^{k,p}} = \left(\sum_{|\alpha|\leq k}\|D^\alpha u\|_{L^p}^p\right)^{1/p}$. Here derivatives are taken in the distributional sense.

The Sobolev space $H^1(\Omega) = W^{1,2}(\Omega)$ is the natural space for solutions of second-order elliptic PDEs, since existence of the weak derivative $u_x \in L^2$ is precisely what is needed to make the energy integral $\int|\nabla u|^2$ finite.

**Theorem (Sobolev Embedding).** For $\Omega \subset \mathbb{R}^n$ with smooth boundary, if $k > n/p$ (or $kp > n$), then $W^{k,p}(\Omega) \hookrightarrow C(\Omega)$ (Sobolev functions with enough regularity are continuous). More generally, $W^{k,p} \hookrightarrow W^{j,q}$ for appropriate relations among $k, j, p, q, n$.

The Sobolev embedding theorem connects regularity (having $L^p$ derivatives of order $k$) to classical smoothness (being a continuous function), which is essential for showing that weak solutions are actually classical solutions under appropriate conditions.

**Theorem (Lax-Milgram).** Let $H$ be a Hilbert space, $B : H \times H \to \mathbb{R}$ a bilinear form that is:
- Bounded: $|B(u,v)| \leq M\|u\|\|v\|$ for all $u, v$
- Coercive: $B(u,u) \geq \alpha\|u\|^2$ for all $u$, with $\alpha > 0$.

Then for every bounded linear functional $F : H \to \mathbb{R}$, there exists a unique $u \in H$ with $B(u,v) = F(v)$ for all $v \in H$.

The Lax-Milgram theorem is the fundamental existence and uniqueness theorem for elliptic PDEs in variational form. The weak formulation of $-\Delta u = f$ on $\Omega$ with zero Dirichlet boundary condition is: find $u \in H_0^1(\Omega)$ with $\int_\Omega \nabla u \cdot \nabla v = \int_\Omega fv$ for all $v \in H_0^1(\Omega)$. Here $B(u,v) = \int\nabla u\cdot\nabla v$ is bounded by the Cauchy-Schwarz inequality and coercive by the Poincaré inequality, so Lax-Milgram applies.

**Spectral Theory of Self-Adjoint Operators.** For a bounded self-adjoint operator $T : H \to H$ on a Hilbert space, the spectral theorem gives $T = \int \lambda\,dE(\lambda)$ for a projection-valued measure $E$. For an unbounded self-adjoint operator (such as $-\Delta$ on $L^2$), a more delicate version holds: the operator has a complete orthonormal system of generalized eigenfunctions (possibly in the distributional sense) with real eigenvalues. This is the infinite-dimensional analogue of the Spectral Theorem for symmetric matrices, and it is the rigorous foundation for the eigenfunction expansion methods used throughout the course.

## Worked Examples

### Example 1: Distributional Derivative

Compute the distributional derivative of $H(x)$ (Heaviside function: $H(x) = 0$ for $x < 0$, $H(x) = 1$ for $x > 0$).

$H'[\varphi] = -H[\varphi'] = -\int_0^\infty \varphi'(x)\,dx = -[\varphi(\infty) - \varphi(0)] = \varphi(0) = \delta[\varphi]$.

So $H' = \delta$ in the distributional sense — the derivative of the Heaviside function is the Dirac delta. This is consistent with the physical interpretation: the step function has a "spike" of infinite height and infinitesimal width at $x=0$.

### Example 2: Weak Solution

A function $u \in H^1(\Omega)$ is a weak solution of $-\Delta u = f$ (with $u = 0$ on $\partial\Omega$) if
$$\int_\Omega \nabla u \cdot \nabla v\,dx = \int_\Omega fv\,dx \quad \text{for all } v \in H_0^1(\Omega).$$

This is derived from the strong form by multiplying by a test function $v \in H_0^1$ and integrating by parts: $-\int_\Omega v\Delta u = \int_\Omega \nabla u\cdot\nabla v - \oint_{\partial\Omega} v\frac{\partial u}{\partial n} = \int_\Omega \nabla u\cdot\nabla v$ (since $v = 0$ on $\partial\Omega$).

The weak formulation requires only that $u$ have first-order distributional derivatives in $L^2$, which is much weaker than requiring $u \in C^2(\Omega)$.

### Example 3: Poincaré Inequality

**Theorem (Poincaré).** There exists $C = C(\Omega) > 0$ such that $\|u\|_{L^2(\Omega)} \leq C\|\nabla u\|_{L^2(\Omega)}$ for all $u \in H_0^1(\Omega)$.

**Proof sketch:** Suppose not. Then for each $n$, there exists $u_n \in H_0^1(\Omega)$ with $\|u_n\|_{L^2} = 1$ and $\|\nabla u_n\|_{L^2} \leq 1/n$. By Rellich-Kondrachov compactness ($H^1 \hookrightarrow\hookrightarrow L^2$), a subsequence converges in $L^2$ to some $u$ with $\|u\|_{L^2} = 1$ and $\|\nabla u\|_{L^2} = 0$. But $\nabla u = 0$ on a connected domain implies $u = \text{const}$; with $u = 0$ on $\partial\Omega$ (in the trace sense), $u = 0$, contradiction.

The Poincaré inequality is what ensures coercivity of the bilinear form in the Lax-Milgram theorem: $B(u,u) = \|\nabla u\|_{L^2}^2 \geq C\|u\|_{H^1}^2$ for all $u \in H_0^1(\Omega)$.

## Historical Notes

**Élie Cartan (1869–1951)** developed the theory of differential forms and exterior calculus that is now the foundation of differential geometry. His work on Lie groups, connections, and holonomy unified differential geometry, group theory, and topology, and is indispensable for the modern formulation of gauge theories in physics.

**Hermann Weyl (1885–1955)** proved that the eigenvalues of the Dirichlet Laplacian on a bounded domain $\Omega \subset \mathbb{R}^n$ satisfy $\lambda_k \sim (4\pi)^{d/2}\Gamma(d/2+1)^{-1}(\text{Vol}(\Omega))^{-1} k^{2/n}$ as $k \to \infty$ (Weyl's law, 1911). He also made foundational contributions to the spectral theory of differential operators on manifolds, and to the general theory of representations of Lie groups.

**Laurent Schwartz (1915–2002)** created the theory of distributions in the 1940s, providing a rigorous home for the Dirac delta and other generalized functions that physicists had been using informally since the 1920s. Schwartz received the Fields Medal in 1950, the first year the medal was awarded, in part for this work.

**Sergei Sobolev (1908–1989)** introduced the Sobolev spaces $W^{k,p}$ in the 1930s in connection with his work on the Cauchy problem for hyperbolic equations. Sobolev's embedding theorems made precise the relationship between the integrability of derivatives and the classical smoothness of functions, and his spaces became the standard setting for the modern theory of PDEs.

**Stefan Banach (1892–1945)** laid the foundations of functional analysis with his *Théorie des opérations linéaires* (1932), introducing Banach spaces, the open mapping theorem, the closed graph theorem, and the uniform boundedness principle. These tools are now standard in the analysis of PDEs.

**Peter Lax (1926–) and Arthur Milgram (1912–1961)** proved the Lax-Milgram theorem in 1954. Lax later received the Abel Prize (2005) for fundamental contributions to PDE theory and numerical analysis.

**Vladimir Maz'ya (1937–)** has made deep contributions to Sobolev space theory, including sharp constants in Sobolev inequalities and the behavior of solutions to elliptic PDEs near singularities.

## Connections to Other Units

**Prerequisites:**
- Unit 00 (Foundations): completeness (Hilbert and Banach spaces are defined by completeness); inner product spaces; eigenvalue theory.
- Unit 01 (Multivariable Calculus): partial derivatives, change of variables, the Inverse Function Theorem (used in defining smooth manifolds).
- Unit 02 (Vector Calculus): the Generalized Stokes' Theorem unifies all the integral theorems of Unit 02.
- Units 03–05: ODEs and PDEs supply the motivation and the applications for every abstract concept in this unit.

**Completion of the Course:**
This unit closes the course by providing the theoretical infrastructure that makes rigorous everything that came before. The reader who completes this unit can read Evans' *Partial Differential Equations*, Taylor's *Partial Differential Equations*, or do Carmo's *Riemannian Geometry* — standard first-year graduate texts — with confidence.

**Research frontiers:**
- Nonlinear elliptic PDEs and the calculus of variations (regularity theory, Morrey spaces, De Giorgi-Nash-Moser theory).
- Geometric analysis: the Ricci flow (used by Perelman to prove the Poincaré conjecture), harmonic maps, Yang-Mills theory.
- Microlocal analysis: pseudodifferential operators, the propagation of singularities for hyperbolic PDEs.
- Stochastic PDEs: when noise enters the equation, Itô calculus and Gaussian measures on function spaces.

## Key Theorems at a Glance

1. **Generalized Stokes' Theorem:** $\int_M d\omega = \int_{\partial M}\omega$ — unifies all integral theorems via exterior calculus.
2. **Gauss-Bonnet Theorem:** $\int_M K\,dA = 2\pi\chi(M)$ for a closed surface — connects curvature (analysis) to topology.
3. **Distributional Differentiation:** Every distribution has derivatives of all orders; $H' = \delta$ (Heaviside derivative is Dirac delta).
4. **Fundamental Solution:** $LE = \delta$ gives the solution to $Lu = f$ as $u = E * f$; explicit in terms of the Green's function for the operator $L$.
5. **Fourier Transform of Tempered Distributions:** Extends to a continuous bijection on $\mathcal{S}'$; $\hat{\delta} = 1$.
6. **Sobolev Embedding Theorem:** $W^{k,p}(\Omega) \hookrightarrow C(\Omega)$ when $kp > n$ — sufficient Sobolev regularity implies classical continuity.
7. **Rellich-Kondrachov Compactness:** The inclusion $H^1(\Omega) \hookrightarrow L^2(\Omega)$ is compact for bounded $\Omega$ with smooth boundary.
8. **Poincaré Inequality:** $\|u\|_{L^2} \leq C\|\nabla u\|_{L^2}$ for $u \in H_0^1(\Omega)$ — controls function norm by gradient norm.
9. **Lax-Milgram Theorem:** Bounded coercive bilinear form on Hilbert space $\Rightarrow$ unique solution to variational problem; foundation of finite element method.
10. **Spectral Theorem for Self-Adjoint Operators:** Unbounded self-adjoint operator on $L^2$ (such as $-\Delta$ on a bounded domain) has a complete orthonormal system of eigenfunctions with real eigenvalues — the rigorous foundation for all eigenfunction expansion methods.
