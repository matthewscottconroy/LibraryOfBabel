# The Spectral Theorem for Self-Adjoint Operators

The spectral theorem for symmetric matrices states that every real symmetric matrix $A$ can be diagonalized by an orthogonal change of basis: there exist orthonormal eigenvectors $e_1, \ldots, e_n$ with real eigenvalues $\lambda_1, \ldots, \lambda_n$ such that $A = \sum_i \lambda_i \langle \cdot, e_i\rangle e_i$. The spectral theorem for self-adjoint operators on Hilbert spaces is the infinite-dimensional generalization. For compact operators, the analogue is exact (as shown in Section 1). For unbounded operators like the Laplacian, the spectrum may be continuous and the statement requires the language of spectral measures.

## Bounded Self-Adjoint Operators

**Definition.** A bounded operator $T: H \to H$ on a Hilbert space is **self-adjoint** if $\langle Tx, y\rangle = \langle x, Ty\rangle$ for all $x, y \in H$.

**Properties:**
- All eigenvalues are real: if $Tx = \lambda x$ and $\lambda = a + bi$, then $b\|x\|^2 = \text{Im}\langle Tx, x\rangle = \text{Im}(\lambda\|x\|^2) = b\|x\|^2$... wait: $\langle Tx, x\rangle = \langle x, Tx\rangle$ implies $\langle Tx, x\rangle$ is real. So $\lambda\|x\|^2 = \langle Tx, x\rangle \in \mathbb{R}$, giving $\lambda \in \mathbb{R}$.
- Eigenvectors for distinct eigenvalues are orthogonal.
- $\|T\| = \sup_{\|x\|=1}|\langle Tx, x\rangle|$ (equality of operator norm and spectral radius for self-adjoint operators).

**Spectrum.** For a bounded self-adjoint $T$, the spectrum $\sigma(T) \subset [\min_{\|x\|=1}\langle Tx,x\rangle, \max_{\|x\|=1}\langle Tx,x\rangle] \subset \mathbb{R}$.

## Spectral Theorem: Bounded Case

**Theorem (Spectral Theorem for Bounded Self-Adjoint Operators).** Let $T: H \to H$ be a bounded self-adjoint operator on a Hilbert space. There exists a unique **projection-valued measure** (or **spectral measure**) $E: \mathcal{B}(\mathbb{R}) \to B(H)$ (from Borel subsets of $\mathbb{R}$ to orthogonal projections on $H$) such that:

1. $E(\mathbb{R}) = I$, $E(\emptyset) = 0$.
2. $E(\sigma(T)^c) = 0$ (the measure is supported on $\sigma(T)$).
3. $E(A \cap B) = E(A)E(B)$ for Borel sets $A, B$.
4. $T = \int_{\sigma(T)} \lambda \, dE(\lambda)$ (spectral decomposition).

The integral is the Stieltjes integral: $\langle Tu, v\rangle = \int_{\sigma(T)} \lambda \, d\langle E(\lambda)u, v\rangle$ for all $u, v \in H$.

For functions $f: \sigma(T) \to \mathbb{R}$, the **functional calculus** defines $f(T) = \int f(\lambda) \, dE(\lambda)$, a bounded operator when $f$ is bounded.

**For compact self-adjoint operators**, the spectral measure is discrete: $E(\{\lambda_n\}) = \langle\cdot, e_n\rangle e_n$ (rank-1 projection onto $e_n$) and $T = \sum_n \lambda_n \langle\cdot, e_n\rangle e_n$.

## Unbounded Self-Adjoint Operators

Many important operators in PDE—the Laplacian $-\Delta$, the Schrödinger operator $-\Delta + V$, the Sturm-Liouville operator—are unbounded. The spectral theory requires more careful definitions.

**Definition.** An unbounded operator $T: \mathcal{D}(T) \subset H \to H$ (defined on a dense domain $\mathcal{D}(T)$) is:
- **Symmetric** if $\langle Tx, y\rangle = \langle x, Ty\rangle$ for all $x, y \in \mathcal{D}(T)$.
- **Self-adjoint** if it is symmetric and $\mathcal{D}(T) = \mathcal{D}(T^*)$ (where $\mathcal{D}(T^*) = \{y : x\mapsto\langle Tx,y\rangle$ is bounded on $\mathcal{D}(T)\}$).

The distinction between symmetric and self-adjoint is subtle but crucial: a symmetric operator may fail to be self-adjoint because the domains don't match. The operator $-d^2/dx^2$ on $L^2([0,1])$ with different boundary conditions can be symmetric but not self-adjoint (if the boundary conditions do not make the adjoint operator's domain equal to the operator's domain).

## Spectral Theorem: Unbounded Case

**Theorem (Spectral Theorem for Unbounded Self-Adjoint Operators).** Let $T: \mathcal{D}(T) \subset H \to H$ be self-adjoint. There exists a unique spectral measure $E: \mathcal{B}(\mathbb{R}) \to B(H)$ such that:

$$\langle Tu, v\rangle = \int_{-\infty}^\infty \lambda \, d\langle E(\lambda)u, v\rangle \quad \text{for all } u \in \mathcal{D}(T), v \in H,$$

where $\mathcal{D}(T) = \{u \in H : \int \lambda^2 \, d\|E(\lambda)u\|^2 < \infty\}$ (elements for which the spectral integral converges).

**Functional calculus.** For a measurable function $f: \mathbb{R} \to \mathbb{R}$, $f(T) = \int f(\lambda) \, dE(\lambda)$ is a (generally unbounded) self-adjoint operator with domain $\{u : \int f(\lambda)^2 \, d\|E(\lambda)u\|^2 < \infty\}$.

## The Spectrum: Point, Continuous, and Residual

The spectrum $\sigma(T) = \{\lambda \in \mathbb{C} : (T - \lambda I) \text{ is not bijective}\}$ decomposes:

- **Point spectrum** $\sigma_p(T)$: eigenvalues ($(T-\lambda I)$ not injective, has nontrivial kernel).
- **Continuous spectrum** $\sigma_c(T)$: $(T-\lambda I)$ is injective, has dense range, but is not surjective.
- **Residual spectrum** $\sigma_r(T)$: $(T-\lambda I)$ is injective but range is not dense. (For self-adjoint operators, $\sigma_r(T) = \emptyset$.)

For self-adjoint operators, $\sigma(T) = \sigma_p(T) \cup \sigma_c(T) \subset \mathbb{R}$.

**Example: $-d/dx^2$ on $L^2([0,\infty))$ with Dirichlet condition at 0.** The spectrum is purely continuous: $\sigma(-d^2/dx^2) = [0,\infty)$. There are no $L^2$ eigenfunctions (generalized eigenfunctions $e^{i\lambda x}$ exist but are not in $L^2$).

**Example: $-d^2/dx^2$ on $L^2([0,L])$ with Dirichlet.** Purely discrete: $\lambda_n = (n\pi/L)^2$, eigenfunctions $\phi_n(x) = \sqrt{2/L}\sin(n\pi x/L)$.

## The Laplacian on a Riemannian Manifold

For a compact Riemannian manifold $(M,g)$ without boundary, the Laplace-Beltrami operator $\Delta_g$ is an unbounded self-adjoint operator on $L^2(M)$ with domain $H^2(M)$. By the theory of compact operators (the resolvent $(-\Delta_g + I)^{-1}: L^2 \to H^2 \hookrightarrow L^2$ is compact by Rellich-Kondrachov), $-\Delta_g$ has a discrete spectrum $0 = \lambda_0 < \lambda_1 \leq \lambda_2 \leq \cdots \to \infty$ with complete orthonormal eigenbasis $\{\phi_n\}$ in $L^2(M)$.

The heat kernel $p_t(x,y)$ is then:

$$p_t(x,y) = \sum_{n=0}^\infty e^{-\lambda_n t} \phi_n(x)\phi_n(y),$$

convergent in $L^2(M \times M)$, with the property that $u(x,t) = \int_M p_t(x,y) u_0(y) \, d\text{vol}(y)$ solves the heat equation $\partial_t u = \Delta_g u$ on $M$.
