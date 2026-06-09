# Compact Operators

Compact operators are the operators in infinite-dimensional functional analysis that most closely resemble matrices. A compact operator maps bounded sets to relatively compact (precompact) sets, restoring the Bolzano-Weierstrass property that fails for general bounded operators in infinite dimensions. The spectral theory of compact operators—the Riesz-Schauder theory—is complete, beautiful, and directly applicable to the analysis of integral equations and the eigenvalue problems arising from elliptic PDEs.

## Definition and First Properties

**Definition.** A bounded linear operator $T: X \to Y$ between Banach spaces is **compact** if for every bounded sequence $(x_n)$ in $X$, the sequence $(Tx_n)$ has a convergent subsequence in $Y$.

Equivalently: $T$ is compact if $T$ maps the unit ball $B_X = \{x : \|x\| \leq 1\}$ to a set with compact closure in $Y$.

**Examples:**
- Every finite-rank operator (image has finite dimension) is compact.
- The identity $I: X \to X$ is compact if and only if $X$ is finite-dimensional.
- Hilbert-Schmidt operators on $L^2(\Omega)$ (integral operators with $L^2$ kernels) are compact.
- The embedding $H^1(\Omega) \hookrightarrow L^2(\Omega)$ for bounded $\Omega$ is compact (Rellich-Kondrachov).

**Properties:**
- The composition of a compact operator with a bounded operator (in either order) is compact.
- The limit (in operator norm) of a sequence of compact operators is compact.
- On a Hilbert space, $T^*$ is compact iff $T$ is compact.

## The Fredholm Alternative

The Fredholm alternative governs the solvability of equations of the form $(I - T)u = f$ where $T$ is compact.

**Theorem (Fredholm Alternative).** Let $X$ be a Banach space and $T: X \to X$ compact. For any $\lambda \neq 0$:

Either:
1. The equation $(\lambda I - T)u = f$ has a unique solution $u \in X$ for every $f \in X$ (and the operator $(\lambda I - T)^{-1}$ is bounded), OR
2. The homogeneous equation $(\lambda I - T)u = 0$ has a nontrivial solution (i.e., $\lambda$ is an eigenvalue of $T$). In this case, the equation $(\lambda I - T)u = f$ is solvable if and only if $f$ is orthogonal to all solutions of $(\bar\lambda I - T^*)v = 0$.

The two alternatives are mutually exclusive and exhaustive.

**Proof sketch.** Case 1 is the invertibility case; the key is that $\lambda I - T$ is a Fredholm operator (kernel and cokernel are finite-dimensional) because $T$ is compact. The kernel $\ker(\lambda I - T)$ is finite-dimensional (the operator is bijective on a finite-dimensional subspace). The range is closed (by a general Banach space argument). The alternative then follows from the closed range theorem. $\square$

**Application to integral equations.** Consider $u(x) - \lambda\int_\Omega K(x,y)u(y) \, dy = f(x)$. Writing $T: u \mapsto \lambda\int K(\cdot,y)u(y) \, dy$, this is $(I - T)u = f$. If $K \in L^2(\Omega \times \Omega)$, then $T$ is a Hilbert-Schmidt operator (hence compact), and the Fredholm alternative applies. Either the integral equation has a unique solution, or the homogeneous equation has nontrivial solutions and solvability requires a compatibility condition.

## Spectral Theory of Compact Operators

**Theorem (Riesz-Schauder).** Let $T: X \to X$ be compact on a Banach space $X$. Then:

1. The spectrum $\sigma(T) = \{\lambda \in \mathbb{C} : (\lambda I - T) \text{ is not invertible}\}$ is at most countable, with no accumulation points except possibly at $\lambda = 0$.
2. Every $\lambda \in \sigma(T) \setminus \{0\}$ is an eigenvalue of $T$ with finite-dimensional eigenspace.
3. If $X$ is infinite-dimensional, then $0 \in \sigma(T)$.

**For compact self-adjoint operators on Hilbert spaces** (the case most relevant to PDE), the theory is complete:

**Theorem.** Let $T: H \to H$ be compact and self-adjoint. Then:
1. All eigenvalues are real.
2. Eigenvectors for distinct eigenvalues are orthogonal.
3. The eigenvalues $(\lambda_n)$, listed with multiplicity and in decreasing order of $|\lambda_n|$, satisfy $\lambda_n \to 0$.
4. There exists a complete orthonormal system $\{e_n\}$ for $H$ consisting of eigenvectors of $T$: $Te_n = \lambda_n e_n$.
5. The spectral decomposition holds: $T = \sum_n \lambda_n \langle \cdot, e_n\rangle e_n$ (convergent in operator norm).

**Proof sketch.** The existence of at least one eigenvalue uses the extremal characterization: $\lambda_1 = \sup_{\|x\|=1}|\langle Tx, x\rangle|$ is attained (by compactness, the supremum is achieved). Then $\lambda_1$ is an eigenvalue. Removing $\text{span}(e_1)$ and repeating gives the full spectrum inductively. $\square$

## Variational Characterization: Min-Max Principle

The eigenvalues of a compact self-adjoint operator $T$ have a variational characterization:

$$\lambda_n = \max_{\substack{V \leq H \\ \dim V = n}} \min_{\substack{v \in V \\ \|v\|=1}} \langle Tv, v\rangle = \min_{\substack{V \leq H \\ \dim V = n-1}} \max_{\substack{v \perp V \\ \|v\|=1}} \langle Tv, v\rangle.$$

(Assuming eigenvalues are ordered $\lambda_1 \geq \lambda_2 \geq \cdots$.) This is the **Courant-Fischer min-max theorem** for compact self-adjoint operators.

For the Dirichlet Laplacian: the eigenvalues of $(-\Delta)^{-1}$ (the Green's operator) are $1/\lambda_n$ where $\lambda_n$ are the Dirichlet eigenvalues of $-\Delta$. The min-max principle gives:

$$\lambda_1(-\Delta) = \min_{u \in H^1_0(\Omega), u \neq 0} \frac{\int_\Omega |\nabla u|^2}{\int_\Omega u^2} \quad \text{(Rayleigh quotient)}.$$

This characterization of the lowest eigenvalue (the ground state energy) is useful both analytically and numerically.

## The Spectrum of the Laplacian on a Bounded Domain

**Theorem.** Let $\Omega \subset \mathbb{R}^n$ be a bounded open set with $C^1$ boundary. The Dirichlet Laplacian $-\Delta: H^2(\Omega) \cap H^1_0(\Omega) \subset L^2(\Omega) \to L^2(\Omega)$ has a discrete spectrum $\{\lambda_n\}_{n=1}^\infty$ with:

1. $0 < \lambda_1 \leq \lambda_2 \leq \cdots \leq \lambda_n \to \infty$.
2. Each $\lambda_n$ is an eigenvalue with finite-dimensional eigenspace.
3. The eigenfunctions $\{\phi_n\}$ form a complete orthonormal basis for $L^2(\Omega)$.

**Proof.** The Green's operator $G = (-\Delta)^{-1}: L^2(\Omega) \to L^2(\Omega)$ is compact (by elliptic regularity: $G: L^2 \to H^2$ is bounded, and $H^2 \hookrightarrow L^2$ is compact by Rellich-Kondrachov). It is self-adjoint (since $-\Delta$ is self-adjoint). By the spectral theorem for compact self-adjoint operators, $G$ has eigenvalues $\mu_n \to 0$ with orthonormal eigenfunctions $\phi_n$. The eigenfunctions of $G$ are eigenfunctions of $-\Delta$ with eigenvalues $\lambda_n = 1/\mu_n \to \infty$. $\square$

**Weyl's law.** The asymptotic distribution of eigenvalues follows Weyl's law:

$$\lambda_n \sim \frac{4\pi^2}{\omega_n^{2/n}} \left(\frac{n}{|\Omega|}\right)^{2/n} n^{2/n} \quad \text{as } n \to \infty,$$

where $|\Omega|$ is the volume and $\omega_n$ is the volume of the unit ball. The leading asymptotic depends only on the volume, confirming that the spectrum encodes geometric information.
