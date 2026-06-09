# Chapter 5 — Advanced Linear Algebra and Spectral Theory

> *Eigenvalues determine stability. The spectrum of a linear operator is its dynamical fingerprint. Learning to read that fingerprint is the prerequisite for everything that follows.*

**Prerequisites:** Undergraduate linear algebra, Chapter 1 (Banach/Hilbert spaces).

**What this chapter builds:** Jordan canonical form and matrix functions (needed for linear ODE solutions); the spectral theorem for normal operators (needed for Koopman operator analysis); SVD and low-rank approximations (needed for data-driven dynamics); bounded operators on Hilbert spaces (needed for transfer operators and quantum information theory); and compact operators (needed for the Perron-Frobenius theorem).

---

## 5.1 Jordan Canonical Form

### 5.1.1 Generalized Eigenspaces

**Definition 5.1.1.** Let $A \in M_n({\mathbb C})$. For an eigenvalue $\lambda$ of $A$, the *generalized eigenspace* is
$$V_\lambda = \ker(A - \lambda I)^n = \{v : (A - \lambda I)^k v = 0 \text{ for some } k\}.$$

*Note:* Since $n = \dim({\mathbb C}^n)$, we have $(A-\lambda I)^n = 0$ on $V_\lambda$ for sure.

**Theorem 5.1.2 (Primary Decomposition).** ${\mathbb C}^n = \bigoplus_{\lambda \in \sigma(A)} V_\lambda$ where $\sigma(A)$ is the spectrum of $A$. Each $V_\lambda$ is $A$-invariant and $A|_{V_\lambda} = \lambda I + N_\lambda$ where $N_\lambda$ is nilpotent.

### 5.1.2 Jordan Blocks and the Jordan Normal Form

**Definition 5.1.3.** A *Jordan block* of size $k$ for eigenvalue $\lambda$ is the $k \times k$ matrix
$$J_k(\lambda) = \begin{pmatrix} \lambda & 1 & 0 & \cdots & 0 \\ 0 & \lambda & 1 & \cdots & 0 \\ \vdots & & \ddots & \ddots & \vdots \\ 0 & \cdots & 0 & \lambda & 1 \\ 0 & \cdots & 0 & 0 & \lambda \end{pmatrix}.$$

**Theorem 5.1.4 (Jordan Canonical Form).** Every $A \in M_n({\mathbb C})$ is similar to a block-diagonal matrix
$$J = \text{diag}(J_{k_1}(\lambda_1), J_{k_2}(\lambda_2), \ldots, J_{k_r}(\lambda_r))$$
where the $\lambda_i$ are eigenvalues (not necessarily distinct) and $\sum k_i = n$. This form is unique up to reordering of blocks.

**Example 5.1.5.** If $A$ has distinct eigenvalues, all Jordan blocks are $1 \times 1$ and $J = \text{diag}(\lambda_1, \ldots, \lambda_n)$ (diagonalization). The Jordan form captures the failure of diagonalizability.

### 5.1.3 The Matrix Exponential Revisited

For a Jordan block $J_k(\lambda)$:
$$e^{tJ_k(\lambda)} = e^{\lambda t} \begin{pmatrix} 1 & t & t^2/2! & \cdots & t^{k-1}/(k-1)! \\ 0 & 1 & t & \cdots & t^{k-2}/(k-2)! \\ \vdots & & \ddots & & \vdots \\ 0 & 0 & \cdots & 1 & t \\ 0 & 0 & \cdots & 0 & 1 \end{pmatrix}.$$

This shows that the growth rate of $e^{tA}$ is determined by the real parts of eigenvalues, while Jordan blocks of size $>1$ contribute *polynomial* growth at the same exponential rate — hence the subtlety in the stability classification.

**Example 5.1.6.** If $A = \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix}$ (Jordan block with $\lambda = 0$), then $e^{tA} = \begin{pmatrix} 1 & t \\ 0 & 1 \end{pmatrix}$. Solutions grow linearly even though the eigenvalue is zero — this is why zero-eigenvalue fixed points require center manifold analysis.

---

## 5.2 The Spectral Theorem

### 5.2.1 Normal Operators

**Definition 5.2.1.** A matrix $A \in M_n({\mathbb C})$ is *normal* if $A^*A = AA^*$ (where $A^* = \bar{A}^T$). Important special cases:
- *Self-adjoint (Hermitian)*: $A = A^*$
- *Unitary*: $A^*A = AA^* = I$
- *Anti-Hermitian*: $A^* = -A$ (these are infinitesimal generators of unitary groups)
- *Skew-symmetric* real matrices (anti-Hermitian over ${\mathbb R}$)

**Theorem 5.2.2 (Spectral Theorem for Normal Matrices).** $A$ is normal if and only if $A$ is unitarily diagonalizable: there exists a unitary $U$ with $U^*AU = D = \text{diag}(\lambda_1, \ldots, \lambda_n)$.

- If $A$ is Hermitian: all $\lambda_i \in {\mathbb R}$.
- If $A$ is unitary: all $|\lambda_i| = 1$.
- If $A$ is positive semi-definite ($\langle Av, v \rangle \geq 0$ for all $v$): all $\lambda_i \geq 0$.

**Proof (for Hermitian matrices):** Induction on $n$. Any Hermitian matrix has at least one real eigenvalue $\lambda$ with unit eigenvector $v$. The orthogonal complement $v^\perp$ is $A$-invariant (since $A$ is Hermitian), so restrict to $v^\perp$ and apply induction.

### 5.2.2 Spectral Theorem for Bounded Self-Adjoint Operators

On infinite-dimensional Hilbert spaces, the spectral theorem requires the concept of spectral measures.

**Definition 5.2.3.** Let $H$ be a Hilbert space. A *projection-valued measure (PVM)* on $({\mathbb R}, \mathcal{B}({\mathbb R}))$ is a map $E: \mathcal{B}({\mathbb R}) \to \mathcal{B}(H)$ where each $E(B)$ is an orthogonal projection, $E({\mathbb R}) = I$, and $E(B \cap C) = E(B)E(C)$ and $E(B \cup C) = E(B) + E(C)$ for disjoint $B, C$.

**Theorem 5.2.4 (Spectral Theorem — Bounded Self-Adjoint Operators).** Let $A: H \to H$ be a bounded self-adjoint operator. There exists a unique PVM $E$ on $\sigma(A) \subseteq {\mathbb R}$ such that
$$A = \int_{\sigma(A)} \lambda\,dE(\lambda).$$

More generally, for any continuous $g: \sigma(A) \to {\mathbb C}$:
$$g(A) = \int_{\sigma(A)} g(\lambda)\,dE(\lambda).$$

**Application in Dynamics:** The Koopman operator $U_f: L^2(\mu) \to L^2(\mu)$ defined by $U_f(\varphi) = \varphi \circ f$ for a measure-preserving map $f$ is *unitary* (if $f$ is invertible). Its spectrum:
- $1 \in \sigma(U_f)$ always (since $U_f(1) = 1$)
- Eigenvalues of $U_f$ are the *dynamical eigenvalues* of $f$
- $f$ is ergodic iff $1$ is a simple eigenvalue of $U_f$
- $f$ is weakly mixing iff the only eigenvalue is $1$

---

## 5.3 Singular Value Decomposition

**Theorem 5.3.1 (SVD).** Every matrix $A \in M_{m \times n}({\mathbb R})$ can be written as $A = U\Sigma V^T$ where $U \in M_{m \times m}$ and $V \in M_{n \times n}$ are orthogonal, and $\Sigma \in M_{m \times n}$ is diagonal with nonneg entries $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_{\min(m,n)} \geq 0$ (the *singular values*).

*(proof)* $A^TA$ is positive semi-definite, so $A^TA = V\Lambda V^T$ (spectral theorem). Set $\sigma_i = \sqrt{\lambda_i}$ and $u_i = Av_i/\sigma_i$ for $\sigma_i > 0$. Extend to an ONB to get $U$.

**Low-Rank Approximation:**

**Theorem 5.3.2 (Eckart-Young-Mirsky).** The best rank-$k$ approximation to $A$ in the Frobenius norm (or operator norm) is $A_k = U\Sigma_k V^T$ where $\Sigma_k = \text{diag}(\sigma_1, \ldots, \sigma_k, 0, \ldots, 0)$.

**Application in Data-Driven Dynamics:** Given time-series data from a dynamical system, *Dynamic Mode Decomposition (DMD)* uses the SVD to find the best linear approximation to the dynamics. The DMD modes are the eigenvectors of the best-fit linear map, and they reveal the dominant spatial patterns and their temporal evolution.

---

## 5.4 Operators on Hilbert Spaces

### 5.4.1 Bounded Linear Operators

**Definition 5.4.1.** A linear map $T: H \to K$ between Hilbert spaces is *bounded* if $\|T\| = \sup_{\|x\|=1} \|Tx\| < \infty$. The space $\mathcal{B}(H, K)$ of bounded linear operators is a Banach space. $\mathcal{B}(H) = \mathcal{B}(H,H)$ is a *C*-algebra.

**Definition 5.4.2.** The *adjoint* $T^*: K \to H$ of a bounded operator $T: H \to K$ is the unique bounded operator satisfying $\langle Tx, y \rangle_K = \langle x, T^*y \rangle_H$ for all $x \in H$, $y \in K$.

- $T$ is *self-adjoint* if $T = T^*$
- $T$ is *unitary* if $T^*T = TT^* = I$ (equivalently, $T$ is a surjective isometry)
- $T$ is *normal* if $T^*T = TT^*$
- $T$ is a *projection* if $T^2 = T = T^*$

**Theorem 5.4.3 (Closed Graph Theorem).** If $T: H \to K$ is a closed linear operator (its graph $\{(x, Tx)\}$ is closed in $H \times K$), then $T$ is bounded.

### 5.4.2 The Spectrum of a Bounded Operator

**Definition 5.4.4.** For $T \in \mathcal{B}(H)$, the *resolvent set* is $\rho(T) = \{\lambda \in {\mathbb C} : T - \lambda I \text{ is invertible}\}$. The *spectrum* is $\sigma(T) = {\mathbb C} \setminus \rho(T)$, decomposed as:
- *Point spectrum* $\sigma_p(T)$: eigenvalues ($\ker(T-\lambda I) \neq 0$)
- *Continuous spectrum* $\sigma_c(T)$: $T - \lambda I$ injective with dense but not closed range
- *Residual spectrum* $\sigma_r(T)$: $T - \lambda I$ injective but range not dense

**Theorem 5.4.5.** For bounded $T$, $\sigma(T)$ is a nonempty compact subset of ${\mathbb C}$, contained in the disk $\{|\lambda| \leq \|T\|\}$.

**Application:** The *spectral radius* $r(T) = \sup\{|\lambda| : \lambda \in \sigma(T)\} = \lim_n \|T^n\|^{1/n}$. For the transfer operator of a dynamical system, the spectral gap (difference between $1 = r(T)$ and the next largest spectral radius) controls the exponential rate of mixing.

---

## 5.5 Compact Operators

**Definition 5.5.1.** $T: H \to K$ is *compact* if it maps bounded sets to relatively compact sets (sets with compact closure). Equivalently: every bounded sequence $(x_n)$ has a subsequence $(x_{n_k})$ with $Tx_{n_k}$ convergent.

**Examples 5.5.2.**
- Every finite-rank operator (range is finite-dimensional) is compact.
- The Hilbert-Schmidt operators: $T$ with $\sum_{i,j} |\langle Te_i, e_j \rangle|^2 < \infty$.
- Integral operators $T_k f(x) = \int k(x,y) f(y)\,dy$ with square-integrable kernel $k$.

**Theorem 5.5.3 (Spectral Theorem for Compact Self-Adjoint Operators).** Let $T: H \to H$ be compact and self-adjoint. Then:
1. $\sigma(T) \subseteq {\mathbb R}$ and $\sigma(T) \setminus \{0\}$ consists only of eigenvalues.
2. The eigenvalues form a (possibly finite or empty) sequence $\lambda_1, \lambda_2, \ldots \to 0$.
3. Each nonzero eigenspace is finite-dimensional.
4. The eigenvectors $\{e_i\}$ form an orthonormal basis for $H$ (if $T \neq 0$).

**Application — Perron-Frobenius for Operators:** The *transfer operator* (Ruelle-Perron-Frobenius operator) $\mathcal{L}: L^2(X, \mu) \to L^2(X, \mu)$ of an expanding map satisfies, under suitable conditions, the spectral theorem for compact operators. The dominant eigenvalue is $1$ (with corresponding eigenfunction the invariant density), and the spectral gap below $1$ controls mixing rates.

---

## 5.6 The Perron-Frobenius Theorem

**Theorem 5.6.1 (Perron-Frobenius).** Let $A \in M_n({\mathbb R})$ have all entries $> 0$ (strictly positive matrix). Then:
1. $A$ has a real positive eigenvalue $\lambda_{\text{PF}}$ (the *Perron eigenvalue*) with $\lambda_{\text{PF}} > |\lambda|$ for all other eigenvalues $\lambda$.
2. The eigenspace for $\lambda_{\text{PF}}$ is one-dimensional with eigenvector $v > 0$ (all components positive).
3. $A^n / \lambda_{\text{PF}}^n \to v w^T$ (in a suitable sense) where $w^T A = \lambda_{\text{PF}} w^T$.

For nonneg irreducible $A$: $\lambda_{\text{PF}} = \max\{|\lambda| : \lambda \in \sigma(A)\}$ (still simple).

**Application in Symbolic Dynamics:** For a subshift of finite type with transition matrix $A \in M_k(\{0,1\})$ (irreducible), the topological entropy is $h = \log \lambda_{\text{PF}}(A)$. The Parry measure (measure of maximal entropy) is given by the left and right Perron-Frobenius eigenvectors.

---

## 5.7 Tensor Products and Exterior Algebra

**Definition 5.7.1.** The *tensor product* $V \otimes W$ of vector spaces is characterized by a universal bilinear map $V \times W \to V \otimes W$, $(v,w) \mapsto v \otimes w$. Concretely, if $\{e_i\}$ is a basis for $V$ and $\{f_j\}$ for $W$, then $\{e_i \otimes f_j\}$ is a basis for $V \otimes W$.

**Definition 5.7.2.** The *exterior power* $\bigwedge^k V$ consists of alternating $k$-linear forms on $V^*$ (or equivalently antisymmetric tensors). If $\dim V = n$, then $\dim \bigwedge^k V = \binom{n}{k}$.

For $A \in M_n({\mathbb R})$, the action on $\bigwedge^k {\mathbb R}^n$ has eigenvalues $\lambda_{i_1} \cdots \lambda_{i_k}$ for all $k$-subsets $\{i_1, \ldots, i_k\}$ of the eigenvalues of $A$. In particular, the action on $\bigwedge^n {\mathbb R}^n \cong {\mathbb R}$ has eigenvalue $\det(A) = \lambda_1 \cdots \lambda_n$.

**Application in Dynamics:** The *Lyapunov exponents* (Chapter 11) measure the growth rates of volumes under the tangent map. The growth rate of $k$-volumes is governed by the action of $D\Phi_t$ on $\bigwedge^k T_pM$.

---

## Exercises

**Exercise 5.1.** Find the Jordan canonical form of $A = \begin{pmatrix} 3 & 1 & 0 \\ 0 & 3 & 1 \\ 0 & 0 & 3 \end{pmatrix}$ and compute $e^{tA}$.

**Exercise 5.2.** Let $A$ be real symmetric. Show all eigenvalues are real. Show eigenvectors for distinct eigenvalues are orthogonal.

**Exercise 5.3.** Compute the SVD of $A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \\ 1 & 0 \end{pmatrix}$. Find the rank-1 approximation.

**Exercise 5.4.** Let $H = L^2([0,1])$ and $T: H \to H$ the Volterra operator $Tf(x) = \int_0^x f(t)\,dt$. Show $T$ is compact. Is $T$ self-adjoint? What is $\sigma(T)$?

**Exercise 5.5.** Let $U: \ell^2 \to \ell^2$ be the bilateral shift: $U(e_n) = e_{n+1}$ for all $n \in {\mathbb Z}$. Show $U$ is unitary and compute its spectrum.

**Exercise 5.6.** (Perron-Frobenius) Let $A = \begin{pmatrix} 1 & 2 \\ 1 & 0 \end{pmatrix}$ (transition matrix for a 2-state system). Find the Perron eigenvalue and eigenvector. If $A$ is the transition matrix of a subshift of finite type, what is its topological entropy?

**Exercise 5.7.** (Tensor product and quantum information) In quantum mechanics, the state space of a system composed of subsystems $A$ and $B$ is $H_A \otimes H_B$. Show that not every state in $H_A \otimes H_B$ can be written as a pure product state $v_A \otimes v_B$. Such states are *entangled*. For $H_A = H_B = {\mathbb C}^2$, exhibit a maximally entangled state (the Bell state $(\ket{00} + \ket{11})/\sqrt{2}$) and compute the entanglement entropy.

**Exercise 5.8.** The *Koopman operator* for the doubling map $f: x \mapsto 2x \pmod{1}$ on $[0,1]$ acts on $L^2([0,1])$ by $U_f \varphi = \varphi \circ f$. Compute $U_f(e^{2\pi i k x})$ for each $k \in {\mathbb Z}$ and describe the spectrum of $U_f$.

---

## Chapter Notes

The Jordan canonical form is covered in Halmos' *Finite Dimensional Vector Spaces* and Hoffman-Kunze's *Linear Algebra*. For the spectral theorem on infinite-dimensional Hilbert spaces, see Rudin's *Functional Analysis* (Chapter 12) and Reed-Simon's *Methods of Mathematical Physics, Vol. 1* (Chapters 6-7). Horn-Johnson's *Matrix Analysis* is the comprehensive reference for matrix theory.

The Perron-Frobenius theorem (Section 5.6) is applied extensively in Chapter 12 (symbolic dynamics — entropy is $\log \lambda_{\text{PF}}$) and Chapter 7 (the Ruelle-Perron-Frobenius theorem for transfer operators). The spectral gap below the Perron eigenvalue controls exponential mixing in Chapters 7 and 9.

Tensor products (Section 5.7) connect this chapter to quantum information theory (Chapter 21), where the tensor product structure of the Hilbert space is the source of entanglement. The exterior algebra connects to the theory of Lyapunov exponents (Chapter 11) via the multiplicative ergodic theorem.
