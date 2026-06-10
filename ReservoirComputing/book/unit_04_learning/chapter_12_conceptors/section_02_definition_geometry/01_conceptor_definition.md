# Section 12.2: Conceptor Definition and Geometry

## 12.2.1 The Motivation: Soft Projection

When a reservoir is driven by a periodic pattern $p$ (say, a sinusoid at frequency $f_0$), the reservoir states $\{\mathbf{r}(t)\}$ lie approximately in a low-dimensional subspace of $\mathbb{R}^N$. This subspace is determined by the pattern: different patterns drive the reservoir into different subspaces. If we could characterize these subspaces, we could:
1. Tell which pattern is currently driving the reservoir (pattern recognition).
2. Constrain the reservoir to stay in a particular subspace (pattern generation).
3. Combine subspaces (compositional processing).

The natural mathematical object for characterizing a subspace is a projection matrix. But hard projection (onto the exact subspace) is fragile: what if the driving pattern is noisy, or the pattern has not been seen for a while, or we want to interpolate between two patterns? We need a *soft* projection — one that captures the subspace's geometry but with a continuous degree of activation.

Conceptors are exactly this: regularized soft projections onto the pattern's state subspace.

## 12.2.2 Derivation from Regularized Least Squares

**Setting.** Suppose pattern $p$ drives the reservoir for $T_p$ timesteps after washout, producing states $\mathbf{r}_1^p, \ldots, \mathbf{r}_{T_p}^p \in \mathbb{R}^N$. Define the state covariance matrix (correlation matrix, for zero-mean states):

$$R^p = \frac{1}{T_p} \sum_{t=1}^{T_p} \mathbf{r}_t^p (\mathbf{r}_t^p)^\top \in \mathbb{R}^{N \times N}.$$

$R^p$ is symmetric positive semidefinite and encodes the geometry of the pattern's state cloud: its eigenvectors are the principal directions of the cloud, and its eigenvalues are the variances along each direction.

**The optimization problem.** We want a matrix $C \in \mathbb{R}^{N \times N}$ that acts as the "conceptual representation" of pattern $p$. Jaeger's key insight is to define the conceptor as the solution to the regularized least-squares problem:

$$C^p = \arg\min_{C \in \mathbb{R}^{N \times N}} \left\{\frac{1}{T_p}\sum_{t=1}^{T_p} \|\mathbf{r}_t^p - C\mathbf{r}_t^p\|^2 + \alpha^{-2}\|C\|_F^2\right\}.$$

This objective asks for the matrix $C$ that minimizes the squared error of "reconstructing" each state vector from itself via $C$, with a Frobenius norm regularization term $\alpha^{-2}\|C\|_F^2$.

**Solving the optimization.** Taking the derivative with respect to $C$ and setting it to zero:

$$\frac{\partial}{\partial C}\left\{\frac{1}{T_p}\sum_t \|\mathbf{r}_t - C\mathbf{r}_t\|^2 + \alpha^{-2}\text{tr}(C^\top C)\right\} = 0.$$

Expand the objective:

$$\frac{1}{T_p}\sum_t (\mathbf{r}_t - C\mathbf{r}_t)^\top(\mathbf{r}_t - C\mathbf{r}_t) + \alpha^{-2}\text{tr}(C^\top C)$$

$$= \text{tr}(R^p) - 2\text{tr}(CR^p) + \text{tr}(CR^p C^\top) + \alpha^{-2}\text{tr}(C^\top C).$$

Taking the matrix derivative with respect to $C$ (using $\partial\text{tr}(CR^pC^\top)/\partial C = 2CR^p$ and $\partial\text{tr}(C^\top C)/\partial C = 2C$):

$$-2R^p + 2CR^p + 2\alpha^{-2}C = 0.$$

$$C(R^p + \alpha^{-2}I) = R^p.$$

Since $R^p + \alpha^{-2}I$ is positive definite (for any $\alpha > 0$, the term $\alpha^{-2}I$ ensures invertibility even if $R^p$ is singular):

$$\boxed{C^p = R^p (R^p + \alpha^{-2} I)^{-1}.}$$

This is the **conceptor** for pattern $p$ with aperture $\alpha$.

**Alternate form.** Using commutativity of the product (for symmetric $R^p$):

$$C^p = (R^p + \alpha^{-2}I)^{-1} R^p = I - \alpha^{-2}(R^p + \alpha^{-2}I)^{-1}.$$

The second form is the "complement" form: $C^p = I - \alpha^{-2}(R^p + \alpha^{-2}I)^{-1}$, which makes the connection to the NOT operation transparent (see Section 12.3).

## 12.2.3 Geometric Interpretation via SVD

Let $R^p = U\Sigma U^\top$ be the eigendecomposition of the symmetric positive semidefinite matrix $R^p$, where $U$ is orthogonal and $\Sigma = \text{diag}(\sigma_1^2, \ldots, \sigma_N^2)$ with $\sigma_1^2 \geq \cdots \geq \sigma_N^2 \geq 0$.

The conceptor becomes:

$$C^p = U\Sigma U^\top(U\Sigma U^\top + \alpha^{-2}I)^{-1} = U\Sigma(\Sigma + \alpha^{-2}I)^{-1}U^\top.$$

Define $D = \Sigma(\Sigma + \alpha^{-2}I)^{-1} = \text{diag}\!\left(\frac{\sigma_1^2}{\sigma_1^2 + \alpha^{-2}}, \ldots, \frac{\sigma_N^2}{\sigma_N^2 + \alpha^{-2}}\right)$.

Then:

$$C^p = U D U^\top,$$

where the diagonal entries of $D$ are the **conceptor singular values**:

$$d_i = \frac{\sigma_i^2}{\sigma_i^2 + \alpha^{-2}} \in [0, 1).$$

**Geometric interpretation.**

- **Large $\sigma_i^2$ (important direction):** $d_i \approx 1$ — the conceptor fully passes this direction. The eigenvector $\mathbf{u}_i$ is a direction in which pattern $p$ has high state variance; the conceptor treats it as "pattern-relevant."

- **Small $\sigma_i^2$ (unimportant direction):** $d_i \approx 0$ — the conceptor suppresses this direction. The reservoir barely moved in this direction when driven by pattern $p$; the conceptor filters it out.

- **$\sigma_i^2 = \alpha^{-2}$ (threshold direction):** $d_i = 1/2$ — the direction is half-retained. This is the boundary of the conceptor's "soft subspace."

The aperture $\alpha$ sets the boundary between "relevant" and "irrelevant" directions: a direction with $\sigma_i > 1/\alpha$ is mostly retained; a direction with $\sigma_i < 1/\alpha$ is mostly suppressed.

## 12.2.4 The Aperture Parameter $\alpha$

The aperture $\alpha$ is the key hyperparameter of the conceptor. Its effects:

**Small $\alpha$ (large $\alpha^{-2}$):** The regularization is heavy. All singular values $d_i = \sigma_i^2/(\sigma_i^2 + \alpha^{-2}) \approx \alpha^2 \sigma_i^2 \to 0$. The conceptor approaches the zero matrix: $C^p \to 0$. The pattern is strongly filtered — the conceptor barely passes anything.

**Large $\alpha$ (small $\alpha^{-2}$):** The regularization is weak. For large directions, $d_i = \sigma_i^2/(\sigma_i^2 + \alpha^{-2}) \approx 1$. The conceptor approaches a hard projection onto the pattern's active subspace. For directions with zero eigenvalue, $d_i = 0$ exactly.

**Intermediate $\alpha$:** The conceptor is a soft projection, retaining directions proportionally to the pattern's state variance.

**Aperture adaptation.** The conceptor at aperture $\alpha$ can be computed from the conceptor at aperture $\beta$ by the **aperture adaptation** formula. If $C = UDU^\top$ with $d_i = \sigma_i^2/(\sigma_i^2 + \alpha^{-2})$, then solving for $\sigma_i^2 = d_i\alpha^{-2}/(1-d_i)$ and substituting:

$$C^{(\gamma)} = C\bigl(C + (\gamma\alpha)^{-2}(I-C)\bigr)^{-1}$$

(this is the conceptor at aperture $\gamma\alpha$, expressed in terms of $C$). This allows the aperture to be tuned after the conceptor has been computed, without rerunning the training data.

## 12.2.5 Properties of Conceptors

**Proposition 12.2.1.** For any pattern $p$ and aperture $\alpha > 0$:
1. $C^p$ is symmetric: $(C^p)^\top = C^p$.
2. $C^p$ is positive semidefinite: all eigenvalues in $[0,1)$.
3. $C^p \in [0, I]$ in the Loewner (PSD) order: $0 \preceq C^p \preceq I$.
4. $C^p = 0$ if and only if $R^p = 0$ (trivial pattern with zero-variance states).
5. $C^p$ is strictly between $0$ and $I$: no eigenvalue is exactly 0 or 1 (for nonzero $R^p$ and finite $\alpha$).

*Proof.* From $C^p = UDU^\top$ with $D = \text{diag}(d_i)$ and $d_i \in (0,1)$: symmetry follows from $U^\top = U^{-1}$ and the diagonal form; positive semidefiniteness follows from $d_i > 0$; the bound $C^p \preceq I$ follows from $d_i < 1$. The strict inequalities follow from $d_i = \sigma_i^2/(\sigma_i^2 + \alpha^{-2}) \in (0,1)$ for any $\sigma_i^2 \geq 0$ and $\alpha^{-2} > 0$. $\blacksquare$

## 12.2.6 Using Conceptors for Pattern Storage and Recall

**Storage.** For each pattern $p \in \{1, \ldots, P\}$, drive the reservoir with pattern $p$, collect states, compute $R^p$, and compute $C^p = R^p(R^p + \alpha^{-2}I)^{-1}$. Store $C^p$ (an $N \times N$ matrix).

**Recall.** To recall pattern $p$, run the reservoir with the conceptor applied at each step:

$$\mathbf{r}(t) = C^p \tanh\!\bigl(W\mathbf{r}(t-1) + \mathbf{w}^{fb} z(t-1)\bigr),$$

where $z(t) = \mathbf{w}^{out\top}\mathbf{r}(t)$ and $\mathbf{w}^{out}$ is the readout weight (trained separately for each pattern by ridge regression during the storage phase).

The matrix $C^p$ projects the new state onto the subspace of pattern $p$'s activity, steering the reservoir toward the states characteristic of that pattern.

**Multiple pattern storage.** With $P$ patterns, the reservoir stores $P$ conceptors $\{C^1, \ldots, C^P\}$. To switch between patterns, switch the conceptor applied to the reservoir dynamics. The capacity for pattern storage is limited by the dimensionality of the state space: if the patterns' active subspaces are sufficiently orthogonal, all $P$ patterns can be stored without interference. If patterns share subspace dimensions, interference occurs.

---

*The conceptor definition is elegant: a regularized projection encoding the geometry of a pattern's state activity. In the next section, we see that these matrices form a rich algebraic structure under Boolean operations.*
