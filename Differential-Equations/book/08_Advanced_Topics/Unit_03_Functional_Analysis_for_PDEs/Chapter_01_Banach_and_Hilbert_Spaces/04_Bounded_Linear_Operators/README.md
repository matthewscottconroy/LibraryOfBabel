# Bounded Linear Operators

Linear operators between function spaces are the morphisms of functional analysis—the analogues of matrices in finite-dimensional linear algebra. In infinite dimensions, however, a linear operator need not be bounded (continuous), and the distinction between bounded and unbounded operators is crucial. This section develops the theory of bounded linear operators, including the operator norm, adjoints, and the connection to PDE theory via the notions of compact and self-adjoint operators.

## Bounded Linear Operators: Definition and Examples

**Definition.** A linear map $T: X \to Y$ between normed spaces is **bounded** if there exists $C > 0$ with $\|Tx\|_Y \leq C\|x\|_X$ for all $x \in X$. The **operator norm** is:

$$\|T\| = \sup_{\|x\|=1}\|Tx\|_Y = \sup_{x \neq 0} \frac{\|Tx\|_Y}{\|x\|_X} = \inf\{C \geq 0 : \|Tx\| \leq C\|x\| \text{ for all } x\}.$$

**Equivalent characterization.** $T$ is bounded if and only if $T$ is continuous at 0, if and only if $T$ is uniformly continuous (linearity converts these).

**The space $B(X,Y)$.** All bounded linear operators from $X$ to $Y$, with the operator norm, form a normed space. If $Y$ is a Banach space, $B(X,Y)$ is a Banach space.

**Examples:**
- **Multiplication operator:** On $L^2(\Omega)$, $(M_\phi f)(x) = \phi(x)f(x)$ for $\phi \in L^\infty(\Omega)$ is bounded with $\|M_\phi\| = \|\phi\|_{L^\infty}$.
- **Shift operator:** On $\ell^2$, $T(a_1, a_2, \ldots) = (0, a_1, a_2, \ldots)$ is bounded with $\|T\| = 1$.
- **Integral operator:** $(Tf)(x) = \int_\Omega K(x,y)f(y) \, dy$ on $L^2(\Omega)$ is bounded if $K \in L^2(\Omega \times \Omega)$, with $\|T\| \leq \|K\|_{L^2(\Omega\times\Omega)}$ (Hilbert-Schmidt operator).
- **Differential operator:** $d/dx: C^1([0,1]) \to C([0,1])$ is NOT bounded with respect to the sup norm on $C^1$ and sup norm on $C^0$? Actually it is: $\|f'\|_\infty \leq \|f\|_{C^1}$. But $d/dx: C^1 \to C^0$ with $C^1$ given the $C^0$ norm is unbounded: $f_n(x) = \sin(nx)/\sqrt{n}$ has $\|f_n\|_\infty = 1/\sqrt{n} \to 0$ but $\|f_n'\|_\infty = \sqrt{n} \to \infty$.

## The Dual Space and Adjoint

For a Banach space $X$, the **dual space** $X^* = B(X, \mathbb{R})$ consists of all bounded linear functionals. Key examples:
- $(L^p(\Omega))^* = L^q(\Omega)$ for $1 < p < \infty$, $1/p + 1/q = 1$.
- $(L^1(\Omega))^* = L^\infty(\Omega)$.
- $(H^1_0(\Omega))^* = H^{-1}(\Omega)$ (Sobolev dual, essential for PDE theory).

For a bounded linear operator $T: X \to Y$, the **adjoint** $T^*: Y^* \to X^*$ is defined by $(T^*f)(x) = f(Tx)$ for $f \in Y^*$, $x \in X$. It satisfies $\|T^*\| = \|T\|$.

For operators on Hilbert spaces, the adjoint takes a simpler form. For $T: H \to H$ bounded linear on a Hilbert space $H$, the **Hilbert space adjoint** $T^*: H \to H$ is the unique bounded operator satisfying:

$$\langle Tx, y\rangle = \langle x, T^*y\rangle \text{ for all } x, y \in H.$$

(Existence and uniqueness follow from the Riesz representation theorem: $y \mapsto \langle Tx, y\rangle$ is a bounded functional in $x$, so there exists $T^*y$ with $\langle Tx, y\rangle = \langle x, T^*y\rangle$.)

An operator is **self-adjoint** if $T = T^*$, i.e., $\langle Tx, y\rangle = \langle x, Ty\rangle$. Self-adjoint operators have real spectrum and form the focus of spectral theory.

## Compact Operators

A bounded operator $T: X \to Y$ is **compact** if it maps bounded sets to relatively compact sets (sets with compact closure). Equivalently, for every bounded sequence $(x_n)$ in $X$, $(Tx_n)$ has a convergent subsequence in $Y$.

Compact operators are important because they restore some finite-dimensional behavior to infinite-dimensional spaces:

**Theorem (Spectral theorem for compact self-adjoint operators).** Let $T: H \to H$ be a compact self-adjoint operator on a Hilbert space. Then:
1. All eigenvalues of $T$ are real.
2. Eigenvectors corresponding to different eigenvalues are orthogonal.
3. The eigenvalues form a sequence (or finite set) $\lambda_1 \geq \lambda_2 \geq \cdots \geq 0 \geq \cdots$ with $\lambda_n \to 0$ (if there are infinitely many).
4. There exists a complete orthonormal system $\{e_n\}$ of eigenvectors: $Te_n = \lambda_n e_n$.
5. $T = \sum_n \lambda_n \langle \cdot, e_n\rangle e_n$ (spectral decomposition, convergent in operator norm).

This is the infinite-dimensional generalization of the spectral theorem for symmetric matrices.

**Examples of compact operators:**
- Hilbert-Schmidt integral operators (kernel in $L^2(\Omega \times \Omega)$).
- Embedding $i: H^1(\Omega) \to L^2(\Omega)$ on bounded $\Omega$ (Rellich-Kondrachov theorem).
- Resolvents $(A - \lambda I)^{-1}$ of elliptic operators with discrete spectrum.

## Connection to Elliptic PDEs

For the Laplacian $-\Delta$ on a bounded domain $\Omega$ with Dirichlet boundary conditions, the **Green's operator** $G: L^2(\Omega) \to L^2(\Omega)$ maps $f$ to the solution $u$ of $-\Delta u = f$, $u|_{\partial\Omega} = 0$. Properties:
- $G: L^2 \to H^2 \cap H^1_0$ is bounded (elliptic regularity).
- The inclusion $H^2 \hookrightarrow L^2$ is compact (Rellich-Kondrachov).
- Therefore $G: L^2 \to L^2$ is compact.
- $G$ is self-adjoint (since $-\Delta$ is self-adjoint with Dirichlet conditions).

By the spectral theorem, $G$ has a complete orthonormal system of eigenfunctions $\{\phi_n\}$ with eigenvalues $\mu_n \to 0$:

$$G\phi_n = \mu_n \phi_n \iff -\Delta\phi_n = \frac{1}{\mu_n}\phi_n = \lambda_n\phi_n.$$

The eigenvalues $\lambda_n = 1/\mu_n \to \infty$ form an increasing sequence, and the eigenfunctions form a basis for $L^2(\Omega)$. This is the eigenfunction expansion used in the separation of variables method for the heat and wave equations on bounded domains.

## Unbounded Operators and Their Domains

Many important operators in PDE theory are unbounded: the differential operator $\partial/\partial x: L^2 \to L^2$ is not bounded (as shown above). Unbounded operators are defined on a **domain** $\mathcal{D}(T) \subset X$, a dense subspace where the operator is well-defined.

For an unbounded operator $T: \mathcal{D}(T) \subset H \to H$ on a Hilbert space, the **adjoint** $T^*$ is defined on $\mathcal{D}(T^*) = \{y \in H : x \mapsto \langle Tx, y\rangle \text{ is bounded on } \mathcal{D}(T)\}$. The operator is **symmetric** if $\langle Tx, y\rangle = \langle x, Ty\rangle$ for all $x, y \in \mathcal{D}(T)$, and **self-adjoint** if additionally $\mathcal{D}(T) = \mathcal{D}(T^*)$.

The Laplacian $-\Delta: H^2(\Omega) \cap H^1_0(\Omega) \to L^2(\Omega)$ is self-adjoint (as an unbounded operator on $L^2$), and its spectral theory (via unbounded self-adjoint operator theory) produces the eigenfunction expansions. This is developed further in Chapter 3.
