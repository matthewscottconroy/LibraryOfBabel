# Appendix A: Linear Algebra Reference

This appendix collects the linear algebra results used throughout the book, with proofs where the derivation illuminates the result. Notation follows the conventions of Appendix G.

---

## A.1 Eigenvalues and Eigenvectors

### Definitions

Let $A \in \mathbb{R}^{n \times n}$. A scalar $\lambda \in \mathbb{C}$ is an **eigenvalue** of $A$ if there exists a nonzero vector $\mathbf{v} \in \mathbb{C}^n$ such that $A\mathbf{v} = \lambda \mathbf{v}$. The vector $\mathbf{v}$ is the corresponding **eigenvector**.

The eigenvalues are the roots of the **characteristic polynomial**:

$$p(\lambda) = \det(\lambda I - A) = 0$$

This is a degree-$n$ polynomial, so $A$ has exactly $n$ eigenvalues in $\mathbb{C}$ (counted with multiplicity, by the Fundamental Theorem of Algebra).

### Spectral Properties Relevant to RC

For a reservoir weight matrix $W \in \mathbb{R}^{N \times N}$, the **spectral radius** is:

$$\rho(W) = \max_i |\lambda_i(W)|$$

The spectral radius controls the long-term behavior of the autonomous dynamics $\mathbf{x}(t) = W\mathbf{x}(t-1)$:
- If $\rho(W) < 1$: $W^t \to 0$ as $t \to \infty$ (stable, contracting)
- If $\rho(W) = 1$: marginally stable (oscillatory or marginally bounded)
- If $\rho(W) > 1$: $\|W^t\| \to \infty$ (unstable, exploding)

The echo state condition requires $\rho(W) < 1$, though in practice $\rho(W)$ slightly below 1 gives the best computational properties.

**Gershgorin's Circle Theorem**: Each eigenvalue $\lambda_i$ of $A$ lies within at least one of the Gershgorin discs:

$$D_i = \left\{z \in \mathbb{C} : |z - a_{ii}| \leq \sum_{j \neq i} |a_{ij}|\right\}$$

This provides a cheap upper bound on the spectral radius: $\rho(A) \leq \max_i \left(|a_{ii}| + \sum_{j \neq i}|a_{ij}|\right)$.

---

## A.2 Singular Value Decomposition — Full Derivation

### Statement

Every matrix $A \in \mathbb{R}^{m \times n}$ (with $m \geq n$ without loss of generality) has a **singular value decomposition**:

$$A = U \Sigma V^\top$$

where:
- $U \in \mathbb{R}^{m \times m}$ is orthogonal ($U^\top U = I_m$)
- $V \in \mathbb{R}^{n \times n}$ is orthogonal ($V^\top V = I_n$)
- $\Sigma \in \mathbb{R}^{m \times n}$ is diagonal with non-negative entries $\sigma_1 \geq \sigma_2 \geq \ldots \geq \sigma_n \geq 0$

The $\sigma_i$ are called **singular values**; the columns of $U$ are **left singular vectors**; the columns of $V$ are **right singular vectors**.

### Construction via Eigendecomposition of $A^\top A$

**Step 1**: Form $A^\top A \in \mathbb{R}^{n \times n}$. This matrix is symmetric positive semidefinite (SPSD):
- Symmetric: $(A^\top A)^\top = A^\top (A^\top)^\top = A^\top A$ ✓
- PSD: $\mathbf{x}^\top (A^\top A) \mathbf{x} = \|A\mathbf{x}\|^2 \geq 0$ ✓

**Step 2**: Compute the eigendecomposition of $A^\top A$. Since it is SPSD, all eigenvalues are real and non-negative. Order them: $\lambda_1 \geq \lambda_2 \geq \ldots \geq \lambda_n \geq 0$ with orthonormal eigenvectors $\mathbf{v}_1, \ldots, \mathbf{v}_n$:

$$A^\top A \mathbf{v}_i = \lambda_i \mathbf{v}_i$$

Set $V = [\mathbf{v}_1, \ldots, \mathbf{v}_n]$ and $\sigma_i = \sqrt{\lambda_i}$.

**Step 3**: For each $i$ with $\sigma_i > 0$, define $\mathbf{u}_i = \frac{1}{\sigma_i} A\mathbf{v}_i$. We verify $\|\mathbf{u}_i\| = 1$:

$$\|\mathbf{u}_i\|^2 = \frac{1}{\sigma_i^2}\|A\mathbf{v}_i\|^2 = \frac{1}{\sigma_i^2}\mathbf{v}_i^\top A^\top A \mathbf{v}_i = \frac{1}{\sigma_i^2}\lambda_i = 1 \checkmark$$

We also verify $\mathbf{u}_i \perp \mathbf{u}_j$ for $i \neq j$:

$$\mathbf{u}_i^\top \mathbf{u}_j = \frac{1}{\sigma_i\sigma_j}\mathbf{v}_i^\top A^\top A \mathbf{v}_j = \frac{\lambda_j}{\sigma_i\sigma_j}\mathbf{v}_i^\top\mathbf{v}_j = 0$$

(since the $\mathbf{v}_j$ are orthogonal). So $\{\mathbf{u}_i\}$ for $\sigma_i > 0$ form an orthonormal set. Extend to an orthonormal basis $U = [\mathbf{u}_1, \ldots, \mathbf{u}_m]$ of $\mathbb{R}^m$ (adding arbitrary orthonormal vectors if $r < m$, where $r = \text{rank}(A)$).

**Step 4**: Verify $A = U\Sigma V^\top$. It suffices to check $AV = U\Sigma$:

$$(AV)_i = A\mathbf{v}_i = \sigma_i \mathbf{u}_i = (U\Sigma)_i \checkmark$$

(The last equality uses $U_i = \mathbf{u}_i$ and $(\Sigma)_{ii} = \sigma_i$.) $\square$

### Thin SVD

For $r = \text{rank}(A) \leq n \leq m$, the **thin (economy) SVD** is:

$$A = U_r \Sigma_r V_r^\top$$

where $U_r \in \mathbb{R}^{m \times r}$, $\Sigma_r = \text{diag}(\sigma_1, \ldots, \sigma_r) \in \mathbb{R}^{r \times r}$, and $V_r \in \mathbb{R}^{n \times r}$. This is the numerically efficient form.

### SVD Applications in RC

**Rank of state matrix**: The state matrix $X \in \mathbb{R}^{N \times T}$ has SVD $X = U\Sigma V^\top$. The number of non-negligible singular values (effective rank) determines the computational capacity of the reservoir.

**Regularization**: Ridge regression solution $W_{\text{out}} = (X^\top X + \lambda I)^{-1} X^\top Y$ can be computed via SVD as:

$$W_{\text{out}} = V \text{diag}\left(\frac{\sigma_i}{\sigma_i^2 + \lambda}\right) U^\top Y$$

This is numerically stable and reveals the effective degrees of freedom: singular modes with $\sigma_i \gg \sqrt{\lambda}$ are fully used; modes with $\sigma_i \ll \sqrt{\lambda}$ are suppressed by regularization.

**Condition number**: $\kappa(X) = \sigma_1 / \sigma_r$ (ratio of largest to smallest nonzero singular value). Large condition number indicates near-linear dependence in the reservoir state and potential numerical instability.

---

## A.3 Moore-Penrose Pseudoinverse — Full Derivation

### Definition

For $A \in \mathbb{R}^{m \times n}$ with SVD $A = U\Sigma V^\top$, the **Moore-Penrose pseudoinverse** is:

$$A^+ = V \Sigma^+ U^\top$$

where $\Sigma^+ \in \mathbb{R}^{n \times m}$ has $(i,i)$ entry $1/\sigma_i$ for $\sigma_i > 0$ and $0$ elsewhere.

### Characterization via Four Penrose Conditions

The pseudoinverse is the unique matrix satisfying all four Penrose conditions:

1. $AA^+A = A$
2. $A^+AA^+ = A^+$
3. $(AA^+)^\top = AA^+$ (Hermitian)
4. $(A^+A)^\top = A^+A$ (Hermitian)

**Proof that $A^+ = V\Sigma^+U^\top$ satisfies these conditions**:

For condition 1: $AA^+A = (U\Sigma V^\top)(V\Sigma^+U^\top)(U\Sigma V^\top)$. Using $V^\top V = I$ and $U^\top U = I$:

$$AA^+A = U(\Sigma\Sigma^+\Sigma)V^\top = U\Sigma V^\top = A$$

(since $\Sigma\Sigma^+\Sigma = \Sigma$ for any diagonal matrix with this structure). Similarly for conditions 2–4. $\square$

### Pseudoinverse and Least Squares

The pseudoinverse gives the minimum-norm least-squares solution to $A\mathbf{x} \approx \mathbf{b}$:

$$\mathbf{x}^* = A^+\mathbf{b}$$

This is the unique solution that:
1. Minimizes $\|A\mathbf{x} - \mathbf{b}\|_2$ (least squares)
2. Among all least-squares solutions, minimizes $\|\mathbf{x}\|_2$ (minimum norm)

**Proof**: Let $\mathbf{x}^* = A^+\mathbf{b} = V\Sigma^+U^\top\mathbf{b}$. Then:

$$A\mathbf{x}^* = U\Sigma V^\top V\Sigma^+U^\top\mathbf{b} = U\Sigma\Sigma^+U^\top\mathbf{b} = U\Pi U^\top\mathbf{b}$$

where $\Pi = \Sigma\Sigma^+$ is the projection onto the column space of $A$. So $A\mathbf{x}^*$ is the projection of $\mathbf{b}$ onto the column space of $A$, which achieves the minimum residual. The minimum-norm property follows from $\mathbf{x}^* \in \text{row space}(A)$ (since $\mathbf{x}^* = V\Sigma^+U^\top\mathbf{b}$ lies in the span of $V$'s columns). $\square$

**RC Application**: In reservoir computing, the optimal readout weights are:

$$W_{\text{out}}^* = Y X^+$$

(without regularization), or equivalently $W_{\text{out}}^* = YX^\top(XX^\top)^+$ (in the overcomplete case). Ridge regression replaces $X^+$ with the regularized pseudoinverse $X^+_\lambda = X^\top(XX^\top + \lambda I)^{-1}$.

---

## A.4 Sherman-Morrison-Woodbury Formula

### Statement

Let $A \in \mathbb{R}^{n \times n}$ be invertible, $U \in \mathbb{R}^{n \times k}$, $C \in \mathbb{R}^{k \times k}$ invertible, $V \in \mathbb{R}^{k \times n}$. Then:

$$(A + UCV)^{-1} = A^{-1} - A^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}$$

The **Sherman-Morrison** formula is the special case $k=1$ (rank-1 update): for column vectors $\mathbf{u}, \mathbf{v} \in \mathbb{R}^n$:

$$(A + \mathbf{u}\mathbf{v}^\top)^{-1} = A^{-1} - \frac{A^{-1}\mathbf{u}\mathbf{v}^\top A^{-1}}{1 + \mathbf{v}^\top A^{-1}\mathbf{u}}$$

### Proof of Woodbury Formula

We verify directly: denote $M = A + UCV$ and $R = A^{-1} - A^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}$. We must show $MR = I$.

$$MR = (A + UCV)\left[A^{-1} - A^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}\right]$$

Expanding:

$$= I - U(C^{-1} + VA^{-1}U)^{-1}VA^{-1} + UCVA^{-1} - UCVA^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}$$

$$= I + \left[UCVA^{-1} - U(C^{-1} + VA^{-1}U)^{-1}VA^{-1} - UCVA^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}\right]$$

Let $Q = (C^{-1} + VA^{-1}U)^{-1}$. Factor out $U(\cdots)VA^{-1}$:

$$= I + \left[UC - Q - UCVA^{-1}UQ\right]VA^{-1}$$

$$= I + U\left[C - C^{-1}Q^{-1}Q - CVA^{-1}UQ\right]VA^{-1}$$

Wait — more cleanly: we need $UC - Q - UCVA^{-1}UQ = UC(I - VA^{-1}UQ) - Q = UC \cdot C^{-1}(C^{-1}+VA^{-1}U)Q - Q$ since $(I - VA^{-1}UQ) = C^{-1}(C^{-1}+VA^{-1}U)^{-1}\cdot(C^{-1}+VA^{-1}U)(I-VA^{-1}UQ)$... The clean proof uses a different approach:

**Factored proof**: Observe that $A + UCV = A(I + A^{-1}UCV)$. By the push-through identity for rank-$k$ updates, $(I + PQ)^{-1} = I - P(I + QP)^{-1}Q$ whenever $(I+PQ)$ is invertible (with $P = A^{-1}U$, $Q = CV$):

$$(I + A^{-1}UCV)^{-1} = I - A^{-1}U(I + CVA^{-1}U)^{-1}CV$$

Premultiplying by $A^{-1}$ gives the Woodbury formula with $C$ absorbed. $\square$

### Applications in RC

**Online Ridge Regression (Recursive Least Squares)**: The ridge regression solution can be updated when a new observation $(\mathbf{x}(t), y(t))$ arrives. Let $A_t = X_t^\top X_t + \lambda I$ where $X_t$ is the state matrix up to time $t$. Then:

$$A_{t+1} = A_t + \mathbf{x}(t+1)\mathbf{x}(t+1)^\top$$

By Sherman-Morrison (rank-1 Woodbury):

$$A_{t+1}^{-1} = A_t^{-1} - \frac{A_t^{-1}\mathbf{x}(t+1)\mathbf{x}(t+1)^\top A_t^{-1}}{1 + \mathbf{x}(t+1)^\top A_t^{-1}\mathbf{x}(t+1)}$$

This gives a $O(N^2)$ per-step update instead of the $O(N^3)$ full recomputation — essential for online reservoir learning (Chapter 9).

---

## A.5 Matrix Norms

### Induced (Operator) Norms

For a matrix $A \in \mathbb{R}^{m \times n}$ and vector norms $\|\cdot\|_\alpha$ on $\mathbb{R}^n$ and $\|\cdot\|_\beta$ on $\mathbb{R}^m$, the induced matrix norm is:

$$\|A\|_{\alpha \to \beta} = \sup_{\mathbf{x} \neq 0} \frac{\|A\mathbf{x}\|_\beta}{\|\mathbf{x}\|_\alpha} = \max_{\|\mathbf{x}\|_\alpha = 1} \|A\mathbf{x}\|_\beta$$

Key special cases:

**Spectral norm** ($\ell^2 \to \ell^2$): $\|A\|_2 = \sigma_1(A)$, the largest singular value.

**$\infty$-norm** ($\ell^\infty \to \ell^\infty$): $\|A\|_\infty = \max_i \sum_j |a_{ij}|$, the maximum row sum.

**1-norm** ($\ell^1 \to \ell^1$): $\|A\|_1 = \max_j \sum_i |a_{ij}|$, the maximum column sum.

### Frobenius Norm (Entry-Wise)

$$\|A\|_F = \sqrt{\sum_{i,j} a_{ij}^2} = \sqrt{\text{tr}(A^\top A)} = \sqrt{\sum_i \sigma_i^2}$$

The Frobenius norm is not an induced norm but is sub-multiplicative and unitarily invariant (invariant under $A \to UAV^\top$ for orthogonal $U, V$). It is widely used in RC for regularization: $\|W_{\text{out}}\|_F^2 = \text{tr}(W_{\text{out}}^\top W_{\text{out}})$ is the ridge regression penalty.

### Comparison Table

| Norm | Formula | Computation | Properties |
|---|---|---|---|
| Spectral $\|A\|_2$ | $\sigma_1(A)$ | Requires SVD | Induced, unitarily invariant |
| Frobenius $\|A\|_F$ | $\sqrt{\sum \sigma_i^2}$ | Cheap (sum of squares) | Not induced, unitarily invariant |
| $\infty$-norm $\|A\|_\infty$ | $\max_i \sum_j |a_{ij}|$ | $O(mn)$ | Induced |
| Nuclear $\|A\|_*$ | $\sum_i \sigma_i$ | Requires SVD | Convex relaxation of rank |

**Norm inequalities**:
$$\|A\|_2 \leq \|A\|_F \leq \sqrt{r}\|A\|_2$$

where $r = \text{rank}(A)$. These bound how far the spectral and Frobenius norms can differ.

---

## A.6 Kronecker Products

### Definition

For $A \in \mathbb{R}^{m \times n}$ and $B \in \mathbb{R}^{p \times q}$, the **Kronecker product** $A \otimes B \in \mathbb{R}^{mp \times nq}$ is:

$$A \otimes B = \begin{pmatrix} a_{11}B & a_{12}B & \cdots & a_{1n}B \\ a_{21}B & a_{22}B & \cdots & a_{2n}B \\ \vdots & & \ddots & \vdots \\ a_{m1}B & a_{m2}B & \cdots & a_{mn}B \end{pmatrix}$$

### Key Properties

1. **Mixed product rule**: $(A \otimes B)(C \otimes D) = (AC) \otimes (BD)$ (when dimensions are compatible).
2. **Transpose**: $(A \otimes B)^\top = A^\top \otimes B^\top$.
3. **Inverse**: $(A \otimes B)^{-1} = A^{-1} \otimes B^{-1}$ (when invertible).
4. **Eigenvalues**: If $A\mathbf{u} = \lambda\mathbf{u}$ and $B\mathbf{v} = \mu\mathbf{v}$, then $(A \otimes B)(\mathbf{u} \otimes \mathbf{v}) = (\lambda\mu)(\mathbf{u} \otimes \mathbf{v})$.
5. **Trace**: $\text{tr}(A \otimes B) = \text{tr}(A)\text{tr}(B)$.

### vec Operator

The **vec** operator stacks the columns of a matrix into a vector: for $A \in \mathbb{R}^{m \times n}$, $\text{vec}(A) \in \mathbb{R}^{mn}$. Key identity:

$$\text{vec}(AXB) = (B^\top \otimes A)\text{vec}(X)$$

This converts the matrix equation $AXB = C$ into the linear system $(B^\top \otimes A)\text{vec}(X) = \text{vec}(C)$.

### RC Application: Batch Readout Training

The ridge regression objective in batch form:

$$\min_{W_{\text{out}}} \|W_{\text{out}}X - Y\|_F^2 + \lambda\|W_{\text{out}}\|_F^2$$

has closed-form solution $W_{\text{out}} = YX^\top(XX^\top + \lambda I)^{-1}$. The Kronecker product appears when computing the solution for multiple output targets simultaneously via the vec operator, and in the analysis of the output weight covariance matrix in Bayesian interpretations of ridge regression (Appendix B).
