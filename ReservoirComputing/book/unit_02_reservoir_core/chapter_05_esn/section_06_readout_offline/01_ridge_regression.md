# Section 6: Offline Readout Training — Ridge Regression

## 6.1 The Training Problem

After collecting reservoir states $x_1, x_2, \ldots, x_T \in \mathbb{R}^N$ (with washout already removed) and the corresponding target outputs $y_1^*, y_2^*, \ldots, y_T^* \in \mathbb{R}^L$, we want to find a linear readout $W^{out} \in \mathbb{R}^{L \times N}$ such that $y_t = W^{out} x_t \approx y_t^*$.

**Why linear?** The power of reservoir computing comes precisely from the fact that the reservoir performs the expensive nonlinear computation, mapping the input history to a high-dimensional, nonlinearly transformed state space. The readout need only find the right linear combination of these features. Linear readout = linear regression = convex optimization = unique global minimum = closed-form solution.

**Assembling the matrices.** Define the **state matrix** $X \in \mathbb{R}^{N \times T}$ and **target matrix** $Y^* \in \mathbb{R}^{L \times T}$:

$$X = \begin{pmatrix} | & | & & | \\ x_1 & x_2 & \cdots & x_T \\ | & | & & | \end{pmatrix}, \quad Y^* = \begin{pmatrix} | & | & & | \\ y_1^* & y_2^* & \cdots & y_T^* \\ | & | & & | \end{pmatrix} \tag{6.1}$$

The readout prediction is $\hat{Y} = W^{out} X$, and we want $\hat{Y} \approx Y^*$.

**Note on dimensions.** With $N$ reservoir neurons, $T$ time steps, and $L$ output channels:
- $X \in \mathbb{R}^{N \times T}$: state matrix (each column is a time step)
- $W^{out} \in \mathbb{R}^{L \times N}$: readout weights
- $Y^*, \hat{Y} \in \mathbb{R}^{L \times T}$: target and predicted outputs

If $T \gg N$ (the typical case), the system is overdetermined and we need least squares. If $T \ll N$, it is underdetermined and we need regularization.

---

## 6.2 Ordinary Least Squares

The unregularized loss is:

$$L_{OLS}(W^{out}) = \|Y^* - W^{out} X\|_F^2 \tag{6.2}$$

where $\|\cdot\|_F$ is the Frobenius norm: $\|A\|_F^2 = \text{tr}(A^\top A) = \sum_{ij} A_{ij}^2$.

**Taking the gradient.** We differentiate with respect to $W^{out}$. Using matrix calculus (treating $W^{out}$ as the variable):

$$\frac{\partial}{\partial W^{out}} \|Y^* - W^{out} X\|_F^2 = \frac{\partial}{\partial W^{out}} \text{tr}\left[(Y^* - W^{out} X)(Y^* - W^{out} X)^\top\right]$$

Expanding:

$$= \frac{\partial}{\partial W^{out}} \left[\text{tr}(Y^* Y^{*\top}) - 2\text{tr}(W^{out} X Y^{*\top}) + \text{tr}(W^{out} X X^\top W^{out\top})\right]$$

Using standard matrix derivative identities ($\frac{\partial}{\partial A}\text{tr}(AB) = B^\top$, $\frac{\partial}{\partial A}\text{tr}(ABA^\top) = 2AB$ for symmetric $B$):

$$\frac{\partial L_{OLS}}{\partial W^{out}} = -2 Y^* X^\top + 2 W^{out} X X^\top \tag{6.3}$$

Setting to zero:

$$W^{out} X X^\top = Y^* X^\top$$
$$W^{out} = Y^* X^\top (XX^\top)^{-1} \tag{6.4}$$

This is the OLS solution, valid when $XX^\top$ is invertible ($T \geq N$ and $X$ has full row rank).

**Problem:** When $T$ is only slightly larger than $N$, or when $W^{out}$ columns are nearly collinear, $XX^\top$ is nearly singular. The solution (6.4) is numerically unstable, and $W^{out}$ can have very large entries — the classic symptom of overfitting.

---

## 6.3 The Ridge Regression Loss

Ridge regression (also called Tikhonov regularization or $L_2$ regularization) adds a penalty on the norm of $W^{out}$:

$$\boxed{L(W^{out}) = \|Y^* - W^{out} X\|_F^2 + \lambda \|W^{out}\|_F^2} \tag{6.5}$$

where $\lambda > 0$ is the **regularization parameter**.

**Dimensional analysis.** The first term has units of $[y^*]^2 \cdot T$ (it sums $T$ squared errors). The second term has units of $[W^{out}]^2 \cdot L \cdot N$. For the two terms to be comparable, $\lambda$ has units $[y^*]^2 \cdot T / ([W^{out}]^2 \cdot L \cdot N)$. In practice, $\lambda$ is a dimensionless scale chosen by cross-validation, but understanding the dimensional structure helps set the search range: a natural scale for $\lambda$ is $\sigma_y^2 / \sigma_W^2$ where $\sigma_y$ is the output standard deviation and $\sigma_W$ is the expected weight magnitude.

**Why this form?** The penalty $\lambda\|W^{out}\|_F^2$ discourages large weights, pulling them toward zero. This has three beneficial effects:
1. **Numerical stability:** The regularized normal equations are always solvable (see below).
2. **Generalization:** Smaller weights produce a readout that is less sensitive to noise and variations in the reservoir state.
3. **Bayesian interpretation:** Ridge regression corresponds to a Gaussian prior on $W^{out}$ (Section 6.5).

---

## 6.4 Derivation of the Ridge Regression Solution

We differentiate $L$ from (6.5) with respect to $W^{out}$:

$$\frac{\partial L}{\partial W^{out}} = \frac{\partial}{\partial W^{out}} \|Y^* - W^{out} X\|_F^2 + \lambda \frac{\partial}{\partial W^{out}} \|W^{out}\|_F^2 \tag{6.6}$$

From (6.3), the first term gives $-2Y^* X^\top + 2W^{out} XX^\top$.

For the second term: $\|W^{out}\|_F^2 = \text{tr}(W^{out} W^{out\top})$, so:

$$\frac{\partial}{\partial W^{out}} \text{tr}(W^{out} W^{out\top}) = 2W^{out} \tag{6.7}$$

Therefore:

$$\frac{\partial L}{\partial W^{out}} = -2Y^* X^\top + 2W^{out} XX^\top + 2\lambda W^{out} \tag{6.8}$$

Setting to zero:

$$W^{out}(XX^\top + \lambda I) = Y^* X^\top$$

$$\boxed{W^{out} = Y^* X^\top (XX^\top + \lambda I)^{-1}} \tag{6.9}$$

This is the **ridge regression solution** (also written as the **regularized pseudoinverse** formula).

**Key properties of $(XX^\top + \lambda I)^{-1}$:**
- For any $\lambda > 0$, the matrix $XX^\top + \lambda I$ is positive definite: for any $v \neq 0$, $v^\top(XX^\top + \lambda I)v = \|Xv\|^2 + \lambda\|v\|^2 > 0$. So the inverse always exists.
- As $\lambda \to 0$: recovers OLS (when $XX^\top$ is invertible).
- As $\lambda \to \infty$: $W^{out} \to 0$ (weights shrink to zero as regularization dominates).
- The singular values of $(XX^\top + \lambda I)^{-1}$ are $1/(\sigma_i^2 + \lambda)$ where $\sigma_i$ are singular values of $X$. Ridge regularization thus shrinks small singular values more strongly than large ones.

---

## 6.5 Bayesian Interpretation: MAP Estimation

The ridge regression solution has a clean probabilistic interpretation as **maximum a posteriori (MAP) estimation**.

**Likelihood.** Suppose the targets are generated as:
$$y_t^* = W^{out} x_t + \epsilon_t, \quad \epsilon_t \sim \mathcal{N}(0, \sigma^2 I_L)$$

The log-likelihood of the parameters given the data is:

$$\log p(Y^* \mid W^{out}, X) = -\frac{1}{2\sigma^2} \|Y^* - W^{out} X\|_F^2 + \text{const} \tag{6.10}$$

**Prior.** Suppose we place a Gaussian prior on the readout weights:
$$W^{out} \sim \mathcal{N}(0, \tau^2 I_{L \times N})$$

meaning each weight is independently drawn from $\mathcal{N}(0, \tau^2)$. The log-prior is:

$$\log p(W^{out}) = -\frac{1}{2\tau^2} \|W^{out}\|_F^2 + \text{const} \tag{6.11}$$

**MAP estimate.** By Bayes' theorem, the posterior is:
$$\log p(W^{out} \mid Y^*, X) = \log p(Y^* \mid W^{out}, X) + \log p(W^{out}) + \text{const}$$

$$= -\frac{1}{2\sigma^2} \|Y^* - W^{out} X\|_F^2 - \frac{1}{2\tau^2} \|W^{out}\|_F^2 + \text{const}$$

Maximizing over $W^{out}$ is equivalent to minimizing:

$$\|Y^* - W^{out} X\|_F^2 + \frac{\sigma^2}{\tau^2} \|W^{out}\|_F^2$$

Comparing with (6.5), we see that the ridge regularization parameter $\lambda = \sigma^2/\tau^2$ is the **ratio of noise variance to prior variance**. 

**Interpretation.** Ridge regression is MAP estimation with a Gaussian prior. A larger $\lambda$ (stronger regularization) corresponds to smaller prior variance $\tau^2$, meaning a stronger belief that the weights should be near zero. A smaller $\lambda$ (weaker regularization) corresponds to larger $\tau^2$, allowing weights to be larger.

The Bayesian interpretation gives a principled way to think about $\lambda$: it encodes our prior belief about the scale of the readout weights, relative to the noise level in the targets. If we have domain knowledge about these quantities (e.g., from the signal-to-noise ratio of the measured targets), we can set $\lambda$ accordingly.

---

## 6.6 Choosing $\lambda$: Cross-Validation

In practice, $\lambda$ is chosen by cross-validation. The procedure is:

**$k$-fold cross-validation:**
1. Split the training set into $k$ folds $\mathcal{F}_1, \ldots, \mathcal{F}_k$.
2. For each candidate $\lambda$:
   - For each fold $\mathcal{F}_i$: train on all folds except $\mathcal{F}_i$, test on $\mathcal{F}_i$.
   - Record the average test NRMSE over all folds.
3. Select the $\lambda$ with the lowest average test NRMSE.
4. Retrain on the full training set with the chosen $\lambda$.

**For temporal data, use temporal cross-validation:** Standard $k$-fold is inappropriate for time series because it destroys temporal ordering, allowing the model to "see the future." Instead, use one of:
- **Walk-forward validation:** Train on $[1, T_1]$, validate on $[T_1+1, T_2]$, then retrain on $[1, T_2]$, validate on $[T_2+1, T_3]$, etc.
- **Hold-out:** Train on the first $T_{train}$ steps, validate on $[T_{train}+1, T_{val}]$.

**Practical range for $\lambda$:** The typical search range is $\lambda \in [10^{-6}, 10^2]$, searched on a log scale. Often $\lambda \in \{10^{-4}, 10^{-3}, 10^{-2}, 10^{-1}, 1, 10\}$ is sufficient for a coarse search.

---

## 6.7 Generalized Cross-Validation (GCV)

For large datasets where $k$-fold cross-validation is expensive, **generalized cross-validation** (GCV) [Golub1979] provides an efficient approximation.

The GCV criterion is:

$$GCV(\lambda) = \frac{\|Y^* - \hat{Y}(\lambda)\|_F^2}{\left[\text{tr}(I - H(\lambda))\right]^2} \tag{6.12}$$

where $H(\lambda) = X^\top (XX^\top + \lambda I)^{-1} X$ is the **hat matrix** (the matrix that maps targets to fitted values: $\hat{Y} = W^{out}(\lambda) X = Y^* H(\lambda)$) and $\text{tr}(I - H(\lambda)) = T - \text{tr}(H(\lambda))$ is the "effective degrees of freedom."

**Intuition:** GCV is an approximation to leave-one-out cross-validation that can be computed without actually removing each sample. It penalizes fits that use many effective parameters (high $\text{tr}(H)$).

Using the SVD of $X = U \Sigma V^\top$ (where $U \in \mathbb{R}^{N \times N}$, $V \in \mathbb{R}^{T \times N}$, $\Sigma = \text{diag}(\sigma_1, \ldots, \sigma_N)$):

$$\text{tr}(H(\lambda)) = \sum_{i=1}^{N} \frac{\sigma_i^2}{\sigma_i^2 + \lambda} \tag{6.13}$$

This is a smooth, increasing function of the $\sigma_i^2$ and a decreasing function of $\lambda$. For large $\lambda$, $\text{tr}(H) \to 0$ (the fit uses no effective parameters); for $\lambda \to 0$, $\text{tr}(H) \to N$ (the fit uses all $N$ parameters).

**Algorithm:**
1. Compute the SVD of $X$.
2. Evaluate $GCV(\lambda)$ at candidate $\lambda$ values using (6.12).
3. Select the $\lambda$ minimizing $GCV$.

The SVD costs $O(N^2 T)$ and needs to be computed once; evaluating $GCV$ for each $\lambda$ is then $O(N)$, making GCV very efficient.

---

## 6.8 Alternative Formulation: Normal Equations

The solution (6.9) can equivalently be written using the **dual** formulation. If $T < N$ (more states than time steps — rare but possible for small datasets), it is more efficient to write:

$$W^{out} = Y^* (X^\top X + \lambda I)^{-1} X \quad \text{(only when $L < T$)} $$

Wait — let us be careful with dimensions. The standard formulation (6.9) involves inverting an $N \times N$ matrix at cost $O(N^3)$. There is an alternative via the kernel ridge regression formulation:

$$W^{out} = Y^* X^\top (X X^\top + \lambda I)^{-1}$$

...wait, this is exactly (6.9) already. Let us write it in the transposed convention some texts use. Define $\tilde{X} \in \mathbb{R}^{T \times N}$ with rows $x_t^\top$ and $\tilde{Y}^* \in \mathbb{R}^{T \times L}$ with rows $y_t^{*\top}$. Then the solution is:

$$W^{out\top} = (\tilde{X}^\top \tilde{X} + \lambda I_N)^{-1} \tilde{X}^\top \tilde{Y}^* \tag{6.14}$$

This is the familiar form of ridge regression. Alternatively, using the Woodbury identity:

$$(\tilde{X}^\top \tilde{X} + \lambda I_N)^{-1} \tilde{X}^\top = \tilde{X}^\top (\tilde{X}\tilde{X}^\top + \lambda I_T)^{-1}$$

This means:
- If $T \geq N$: invert an $N \times N$ matrix — cost $O(N^3 + N^2 T)$.
- If $T < N$: invert a $T \times T$ matrix — cost $O(T^3 + NT^2)$.

Since typically $T \gg N$ in ESN training (we collect many samples from a single reservoir), the $N \times N$ formulation (6.9) is almost always preferred.

---

## 6.9 Numerical Stability: Why Ridge is Better Than OLS

The ordinary least squares solution requires inverting $XX^\top$. When $X$ has nearly linearly dependent rows (which happens when reservoir neurons are highly correlated), $XX^\top$ is nearly singular — its smallest eigenvalue is near zero.

With ridge regularization, we invert $XX^\top + \lambda I$, whose smallest eigenvalue is at least $\lambda$. The **condition number** of the regularized matrix is:

$$\kappa(XX^\top + \lambda I) = \frac{\sigma_{max}^2 + \lambda}{\sigma_{min}^2 + \lambda} \leq \frac{\sigma_{max}^2 + \lambda}{\lambda} = 1 + \frac{\sigma_{max}^2}{\lambda} \tag{6.15}$$

For $\lambda \approx \sigma_{max}^2$, the condition number is at most $2$ — perfectly conditioned. As $\lambda \to 0$, the condition number approaches $\kappa(XX^\top) = (\sigma_{max}/\sigma_{min})^2$, which can be enormous.

The practical recipe: if you observe numerical issues with the ESN training (e.g., oscillating readout predictions, unexpected NaN values), try increasing $\lambda$. A well-conditioned system with a slight bias is almost always better than an ill-conditioned system with theoretically zero bias.
