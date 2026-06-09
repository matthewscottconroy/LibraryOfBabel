# 5.4 Operators on Hilbert Spaces

In dynamics, the natural operators — Koopman operators, transfer operators, Perron-Frobenius operators — act on infinite-dimensional function spaces. To study them, we need the theory of bounded linear operators on Hilbert spaces, including their spectra, adjoints, and spectral decompositions.

## 5.4.1 Bounded Linear Operators

**Definition 5.4.1.** A linear map $T: H \to K$ between Hilbert spaces is *bounded* if $\|T\| = \sup_{\|x\|=1} \|Tx\| < \infty$. The space $\mathcal{B}(H, K)$ of bounded linear operators is a Banach space. $\mathcal{B}(H) = \mathcal{B}(H,H)$ is a *C*-algebra*.

Boundedness is the infinite-dimensional analog of continuity: a linear map is continuous iff it's bounded. The operator norm $\|T\|$ is the best Lipschitz constant of $T$.

**Definition 5.4.2.** The *adjoint* $T^*: K \to H$ of a bounded operator $T: H \to K$ is the unique bounded operator satisfying $\langle Tx, y \rangle_K = \langle x, T^*y \rangle_H$ for all $x \in H$, $y \in K$.

The adjoint exists and is unique by the Riesz Representation Theorem (applied to the functional $y \mapsto \langle Tx, y \rangle$). In finite dimensions, for matrices with the standard inner product, $T^* = T^H$ (the conjugate transpose).

The important special cases:
- $T$ is *self-adjoint* if $T = T^*$ — the infinite-dimensional analog of Hermitian matrices.
- $T$ is *unitary* if $T^*T = TT^* = I$ — an isomorphism that preserves the inner product.
- $T$ is *normal* if $T^*T = TT^*$ — the class to which the spectral theorem extends.
- $T$ is a *projection* if $T^2 = T = T^*$ — the infinite-dimensional analog of the projection matrices from linear algebra.

**Theorem 5.4.3 (Closed Graph Theorem).** If $T: H \to K$ is a closed linear operator (its graph $\{(x, Tx)\}$ is closed in $H \times K$), then $T$ is bounded.

The Closed Graph Theorem is one of the fundamental results of functional analysis. It says you can prove an operator is bounded by checking a weaker condition — closedness of the graph — which is often easier to verify.

## 5.4.2 The Spectrum of a Bounded Operator

In finite dimensions, the spectrum of a matrix is just its set of eigenvalues. In infinite dimensions, the spectrum is richer:

**Definition 5.4.4.** For $T \in \mathcal{B}(H)$, the *resolvent set* is $\rho(T) = \{\lambda \in \mathbb{C} : T - \lambda I \text{ is invertible}\}$. The *spectrum* is $\sigma(T) = \mathbb{C} \setminus \rho(T)$, decomposed as:
- *Point spectrum* $\sigma_p(T)$: eigenvalues ($\ker(T-\lambda I) \neq 0$)
- *Continuous spectrum* $\sigma_c(T)$: $T - \lambda I$ injective with dense but not closed range
- *Residual spectrum* $\sigma_r(T)$: $T - \lambda I$ injective but range not dense

In finite dimensions, every $\lambda \in \sigma(T)$ is an eigenvalue — the spectrum is the point spectrum. In infinite dimensions, the continuous spectrum appears when $T - \lambda I$ fails to be invertible for a subtler reason: it's injective (no eigenvectors) but its range isn't all of $H$.

**Theorem 5.4.5.** For bounded $T$, $\sigma(T)$ is a nonempty compact subset of $\mathbb{C}$, contained in the disk $\{|\lambda| \leq \|T\|\}$.

**Application.** The *spectral radius* $r(T) = \sup\{|\lambda| : \lambda \in \sigma(T)\} = \lim_n \|T^n\|^{1/n}$. For the transfer operator of a dynamical system, the spectral gap — the difference between the leading eigenvalue $1 = r(T)$ and the next-largest spectral radius — controls the exponential rate of mixing. A large spectral gap means fast mixing; no spectral gap means slow or no mixing.

For the Koopman operator $U_f$ of an ergodic measure-preserving map, the spectral theory sorts out: which functions $\varphi \in L^2(\mu)$ show periodic behavior (eigenfunctions), which show weakly mixing behavior (continuous spectrum), and which show the strongest mixing (Lebesgue spectrum). This spectral classification of ergodic systems is one of the central achievements of 20th-century mathematics.
