# Section 7.1: Information-Processing Capacity

## 7.1.1 Setting the Stage

We want to measure how much information a reservoir carries about functions of its input history. The setup is precise. Let $\{u_t\}_{t=-\infty}^{\infty}$ be a stationary i.i.d. input sequence drawn from some distribution with finite moments. Let the reservoir have $N$ neurons with state $\mathbf{r}(t) \in \mathbb{R}^N$ evolving according to

$$\mathbf{r}(t) = \tanh\!\bigl(W\,\mathbf{r}(t-1) + W^{in} u_t\bigr),$$

where $W \in \mathbb{R}^{N \times N}$ and $W^{in} \in \mathbb{R}^N$. (The analysis extends immediately to more general update rules, as we will note, but this is our canonical form.)

A readout is a linear map $W^{out} \in \mathbb{R}^N$ that produces output

$$\hat{y}(t) = W^{out \top} \mathbf{r}(t).$$

Now suppose we have a *target function* $f: \mathbb{R}^\infty \to \mathbb{R}$, a scalar function of the semi-infinite input history $(\ldots, u_{t-2}, u_{t-1}, u_t)$. We want to know: how well can the linear readout reconstruct $f$ from the reservoir state? The answer depends on the reservoir, not on any particular choice of $W^{out}$; we optimize over $W^{out}$ and ask what the best possible linear reconstruction achieves.

## 7.1.2 The Capacity of a Reservoir for a Function

**Definition 7.1.1 (Function Capacity).** The *capacity* of a reservoir for target function $f$ is

$$C_f = \frac{\operatorname{Var}[\hat{y}_f(t)]}{\operatorname{Var}[f(t)]},$$

where $\hat{y}_f(t) = W^{out \top}_f \mathbf{r}(t)$ is the *optimal* linear reconstruction, obtained by minimizing the mean squared error:

$$W^{out}_f = \arg\min_{W^{out}} \mathbb{E}\bigl[(f(t) - W^{out \top}\mathbf{r}(t))^2\bigr].$$

The ratio $C_f \in [0, 1]$ measures the fraction of the variance of $f(t)$ that can be linearly explained by the reservoir state. If $C_f = 1$, the reservoir carries all information about $f$; if $C_f = 0$, it carries none.

**Remark.** The quantity $C_f$ is exactly the *coefficient of determination* $R^2$ of the optimal linear regression of $f$ on $\mathbf{r}$. This is not an accident — the capacity framework is built on the intimate connection between least-squares regression and orthogonal projection in function space.

### Computing $C_f$ Explicitly

The optimal linear readout is the solution to the normal equations. Let $R_{\mathbf{rr}} = \mathbb{E}[\mathbf{r}(t)\mathbf{r}(t)^\top]$ be the $N \times N$ state covariance matrix, and let $\mathbf{c}_f = \mathbb{E}[\mathbf{r}(t) f(t)]$ be the $N$-vector of cross-covariances between reservoir states and the target. Then

$$W^{out}_f = R_{\mathbf{rr}}^{-1} \mathbf{c}_f.$$

The variance of the optimal reconstruction is

$$\operatorname{Var}[\hat{y}_f(t)] = \mathbf{c}_f^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_f,$$

and therefore

$$C_f = \frac{\mathbf{c}_f^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_f}{\operatorname{Var}[f(t)]}.$$

This is a familiar expression: it is the squared correlation between $f(t)$ and its projection onto the column space of $R_{\mathbf{rr}}$, normalized by the target variance.

## 7.1.3 The Orthonormal Basis Expansion

The definition of $C_f$ for a single function is useful but incomplete. To measure *total* capacity, we need to sum over all possible target functions — but this sum is infinite-dimensional and needs a basis.

The key insight of [Dambre2012] is to choose the basis carefully. Let $\{b_j\}_{j=1}^\infty$ be an *orthonormal* family of functions in $L^2$ of the input history, orthonormal with respect to the inner product

$$\langle f, g \rangle = \mathbb{E}[f(t) g(t)].$$

Such a basis always exists: for i.i.d. inputs with known marginal distribution, it can be constructed explicitly (for uniform inputs on $[-1, 1]$, the Legendre polynomials form such a basis; for Gaussian inputs, the Hermite polynomials do). The orthonormality condition is

$$\langle b_j, b_k \rangle = \mathbb{E}[b_j(t) b_k(t)] = \delta_{jk}.$$

**Definition 7.1.2 (Total Information-Processing Capacity).** The *total information-processing capacity* of a reservoir is

$$C_{total} = \sum_{j=1}^\infty C_{b_j},$$

where the sum is over any complete orthonormal basis $\{b_j\}$ of the target function space.

A crucial property: this sum is *independent of the choice of basis*. We will prove this below.

## 7.1.4 The Main Theorem: $C_{total} \leq N$

**Theorem 7.1.1 (Dambre et al. 2012).** For any reservoir with $N$ neurons satisfying the echo state property, and any i.i.d. input distribution, the total information-processing capacity satisfies

$$C_{total} \leq N.$$

*Proof.* The proof proceeds by expressing $C_{total}$ in terms of the state covariance matrix and invoking a rank argument.

**Step 1: Express each $C_{b_j}$ in terms of the state covariance.**

By the formula derived above,

$$C_{b_j} = \frac{\mathbf{c}_{b_j}^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_{b_j}}{\operatorname{Var}[b_j(t)]} = \mathbf{c}_{b_j}^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_{b_j},$$

where the last equality uses $\operatorname{Var}[b_j] = \mathbb{E}[b_j^2] = 1$ by the orthonormality of the basis (and assuming zero mean, which can always be arranged by centering).

**Step 2: Construct the infinite matrix of cross-covariance vectors.**

Define the matrix $M$ whose $j$-th column is $\mathbf{c}_{b_j} = \mathbb{E}[\mathbf{r}(t) b_j(t)] \in \mathbb{R}^N$. Each column is an $N$-vector, so $M \in \mathbb{R}^{N \times \infty}$ (formally an infinite matrix, but with only $N$ rows).

Then

$$C_{total} = \sum_{j=1}^\infty \mathbf{c}_{b_j}^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_{b_j} = \operatorname{tr}\!\bigl(M^\top R_{\mathbf{rr}}^{-1} M\bigr) = \operatorname{tr}\!\bigl(R_{\mathbf{rr}}^{-1} M M^\top\bigr).$$

The last equality uses the cyclic property of the trace: $\operatorname{tr}(AB) = \operatorname{tr}(BA)$ for matrices where both products are defined.

**Step 3: Bound $MM^\top$.**

The matrix $MM^\top \in \mathbb{R}^{N \times N}$ has entries

$$(MM^\top)_{ik} = \sum_{j=1}^\infty \mathbb{E}[r_i(t) b_j(t)] \cdot \mathbb{E}[r_k(t) b_j(t)].$$

By Parseval's theorem applied to the $L^2$ inner product (since $\{b_j\}$ is a complete orthonormal basis),

$$\sum_{j=1}^\infty \mathbb{E}[r_i(t) b_j(t)] \cdot \mathbb{E}[r_k(t) b_j(t)] = \mathbb{E}[r_i(t) r_k(t)] - \mathbb{E}[r_i(t)]\mathbb{E}[r_k(t)].$$

In matrix form: $MM^\top = R_{\mathbf{rr}} - \bar{\mathbf{r}}\bar{\mathbf{r}}^\top$, where $\bar{\mathbf{r}} = \mathbb{E}[\mathbf{r}(t)]$.

**Step 4: Complete the trace bound.**

Substituting into the expression for $C_{total}$:

$$C_{total} = \operatorname{tr}\!\bigl(R_{\mathbf{rr}}^{-1}(R_{\mathbf{rr}} - \bar{\mathbf{r}}\bar{\mathbf{r}}^\top)\bigr) = \operatorname{tr}(I_N) - \operatorname{tr}\!\bigl(R_{\mathbf{rr}}^{-1}\bar{\mathbf{r}}\bar{\mathbf{r}}^\top\bigr).$$

The first term is $N$. The second term is $\bar{\mathbf{r}}^\top R_{\mathbf{rr}}^{-1} \bar{\mathbf{r}} \geq 0$ (since $R_{\mathbf{rr}}$ is positive definite). Therefore

$$C_{total} = N - \bar{\mathbf{r}}^\top R_{\mathbf{rr}}^{-1} \bar{\mathbf{r}} \leq N.$$

The bound is tight if and only if $\bar{\mathbf{r}} = \mathbf{0}$, i.e., the reservoir state has zero mean. $\blacksquare$

**Remark 7.1.1.** This proof reveals something important: the capacity loss is exactly $\bar{\mathbf{r}}^\top R_{\mathbf{rr}}^{-1} \bar{\mathbf{r}}$, the Mahalanobis distance of the mean state from the origin. If the reservoir mean state is far from zero — as it can be when the input drives the reservoir into a strongly saturated regime — capacity is wasted. This is one argument for centering reservoir states before training the readout.

**Remark 7.1.2.** The proof used the completeness of the basis $\{b_j\}$ through Parseval's theorem. This is what makes the total capacity *independent of the specific basis chosen*: any complete orthonormal basis gives the same value.

## 7.1.5 The Orthogonal Decomposition

The theorem tells us the total budget is $N$ (for zero-mean states). The decomposition tells us *where* that budget is spent. Different target functions consume different amounts of capacity, and these amounts are additive for orthogonal targets.

**Corollary 7.1.1 (Capacity Decomposition).** For any two target functions $f, g$ with $\langle f, g \rangle = 0$,

$$C_{f+g} = C_f + C_g.$$

This is not obvious from the definition, but it follows from the fact that $\mathbf{c}_{f+g} = \mathbf{c}_f + \mathbf{c}_g$ and the cross terms vanish by orthogonality: $\mathbb{E}[\hat{y}_f \hat{y}_g] = \mathbf{c}_f^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_g = 0$ when $f \perp g$ (under mild conditions on the reservoir's coupling to the input history).

In practice, this means you can decompose the total capacity into contributions from:

- **Linear memory components**: $C_{u_{t-k}}$ for each delay $k$ (Jaeger's memory capacity).
- **Quadratic components**: $C_{u_{t-k}^2}$, $C_{u_{t-k} u_{t-l}}$ for $k \neq l$.
- **Higher-order components**: products and Legendre/Hermite polynomials of delayed inputs.

This decomposition gives a complete fingerprint of what the reservoir is computing. You can see, at a glance, whether a reservoir is spending its capacity on linear memory (good for linear prediction tasks), on nonlinear transformations of the current input (good for classification tasks requiring nonlinearity), or on nonlinear combinations of past and present inputs (the hardest case, requiring both memory and nonlinearity simultaneously).

## 7.1.6 When Is the Bound Tight?

The bound $C_{total} \leq N$ is tight — achieved with equality — when $\bar{\mathbf{r}} = \mathbf{0}$. But this is a necessary, not sufficient, condition. For the entire budget $N$ to be usefully deployed, the reservoir states must span $\mathbb{R}^N$; equivalently, the state covariance matrix $R_{\mathbf{rr}}$ must be full rank.

A linear reservoir with orthogonal weight matrix ($W^\top W = \rho^2 I$) achieves this bound, as shown in [Dambre2012]. Intuitively, an orthogonal weight matrix preserves the norms of state vectors, preventing any direction in state space from being preferentially contracted. This distributes the $N$ capacity units as evenly as possible across the orthogonal basis functions.

Nonlinear reservoirs (with tanh activations) can approach but not exceed $N$, and typically do not achieve the bound exactly. The tanh nonlinearity breaks the orthogonality of the dynamics, creating correlations between state dimensions that reduce effective capacity.

## 7.1.7 Practical Measurement

In practice, the capacity is measured empirically as follows:

1. Drive the reservoir with a long i.i.d. input sequence $u_1, u_2, \ldots, u_T$.
2. Collect reservoir states $\mathbf{r}(1), \ldots, \mathbf{r}(T)$ (discarding a washout period).
3. Construct the state covariance matrix $\hat{R}_{\mathbf{rr}} = \frac{1}{T} \sum_t \mathbf{r}(t)\mathbf{r}(t)^\top$.
4. For each target function $b_j$ in a finite basis, compute the cross-covariance $\hat{\mathbf{c}}_{b_j} = \frac{1}{T}\sum_t \mathbf{r}(t) b_j(t)$.
5. Estimate $C_{b_j} = \hat{\mathbf{c}}_{b_j}^\top \hat{R}_{\mathbf{rr}}^{-1} \hat{\mathbf{c}}_{b_j}$.

In step 5, $\hat{R}_{\mathbf{rr}}$ is typically regularized (ridge regression) to avoid numerical issues with near-singular covariance matrices.

The total capacity estimate $\hat{C}_{total} = \sum_j \hat{C}_{b_j}$ provides the capacity profile. Plotting $C_{b_j}$ as a function of the degree and delay of the basis function $b_j$ gives a rich picture of what the reservoir is actually computing.

---

*This section has laid the mathematical foundation. In the next section, we specialize to linear target functions and derive Jaeger's memory capacity, which admits a particularly clean analytical treatment.*
