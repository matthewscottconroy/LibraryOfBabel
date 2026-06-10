# Section 8.2: Spectral Radius Tuning

## 8.2.1 The Master Knob

Among all reservoir hyperparameters, the spectral radius $\rho = \max_i |\lambda_i(W)|$ has the most direct and well-understood theoretical interpretation. It controls the timescale over which the reservoir "forgets" past inputs — it is, in the language of dynamical systems, the parameter that determines how quickly the reservoir's free response decays. This makes it the primary handle for matching the reservoir's temporal processing to the temporal structure of the task.

To see why, recall the dynamics of a linear reservoir:

$$\mathbf{r}(t) = W\mathbf{r}(t-1) + \mathbf{w}^{in} u_t.$$

Iterating the recursion with initial condition $\mathbf{r}(0) = \mathbf{0}$ gives:

$$\mathbf{r}(t) = \sum_{k=0}^{t-1} W^k \mathbf{w}^{in} u_{t-k} + W^t \mathbf{r}(0).$$

The term $W^t \mathbf{r}(0)$ decays to zero if all eigenvalues of $W$ have absolute value less than 1, i.e., if $\rho < 1$. The rate of this decay is governed by $\rho$: the dominant term decays as $\rho^t$.

The term $\sum_k W^k \mathbf{w}^{in} u_{t-k}$ shows that the current state is a weighted sum of all past inputs, with the weight on input $u_{t-k}$ proportional to $W^k \mathbf{w}^{in}$, which has magnitude $\sim \rho^k$. This is the *memory kernel* of the reservoir.

## 8.2.2 The Geometric Series Argument: $MC_k \propto \rho^{2k}$

We can make the connection to memory capacity precise. The derivation follows the calculation in Section 7.2.5, but we now track the dependence on $\rho$ carefully.

**Setup.** Assume the reservoir weight matrix $W$ is diagonalizable with eigendecomposition $W = V \Lambda V^{-1}$, where $\Lambda = \text{diag}(\lambda_1, \ldots, \lambda_N)$ and $|\lambda_1| = \rho \geq |\lambda_2| \geq \cdots \geq |\lambda_N|$. Let $\tilde{\mathbf{w}}^{in} = V^{-1}\mathbf{w}^{in}$ be the input weight vector in the eigenbasis.

**State in eigenbasis.** Define $\tilde{\mathbf{r}}(t) = V^{-1}\mathbf{r}(t)$. Then

$$\tilde{r}_i(t) = \lambda_i \tilde{r}_i(t-1) + \tilde{w}^{in}_i u_t,$$

i.e., each mode $i$ evolves independently as a scalar AR(1) process with pole $\lambda_i$.

The stationary solution is

$$\tilde{r}_i(t) = \sum_{k=0}^\infty \lambda_i^k \tilde{w}^{in}_i u_{t-k} = \tilde{w}^{in}_i \sum_{k=0}^\infty \lambda_i^k u_{t-k}.$$

**Cross-covariance in eigenbasis.** The cross-covariance between mode $i$ and input $u_{t-k}$ is

$$\tilde{c}_{ik} = \mathbb{E}[\tilde{r}_i(t) u_{t-k}] = \tilde{w}^{in}_i \lambda_i^k.$$

(Since $\mathbb{E}[u_{t-j} u_{t-k}] = \delta_{jk}$.)

**Memory capacity for mode $i$.** The contribution of mode $i$ to $MC_k$ is

$$MC_k^{(i)} = \frac{|\tilde{c}_{ik}|^2}{\tilde{\sigma}_i^2} = \frac{|\tilde{w}^{in}_i|^2 |\lambda_i|^{2k}}{|\tilde{w}^{in}_i|^2 / (1 - |\lambda_i|^2)} = (1 - |\lambda_i|^2) |\lambda_i|^{2k}.$$

Summing over modes (in the eigenbasis, modes are independent, so contributions add):

$$MC_k = \sum_{i=1}^N (1 - |\lambda_i|^2) |\lambda_i|^{2k} \cdot \frac{|\tilde{w}^{in}_i|^2}{\sum_j |\tilde{w}^{in}_j|^2 |\lambda_j|^{2k}/(1-|\lambda_j|^2)}.$$

This general formula is complex. For clarity, consider the **dominant mode approximation**: if one eigenvalue $\lambda_1$ dominates (i.e., $|\lambda_1| = \rho$ and $|\lambda_i| \ll \rho$ for $i > 1$), then for large $k$:

$$\boxed{MC_k \approx (1 - \rho^2) |\tilde{w}^{in}_1|^2 \cdot \rho^{2(k-1)} \propto \rho^{2k}.}$$

This is the key formula. The $k$-step memory capacity decays *geometrically* with rate $\rho^2$. Equivalently, the characteristic memory time is

$$\tau = \frac{-1}{\ln \rho^2} = \frac{-1}{2\ln\rho}.$$

As $\rho \to 1$, $\tau \to \infty$ and the reservoir develops arbitrarily long memory.

**Total memory capacity (dominant mode approximation):**

$$MC = \sum_{k=1}^\infty MC_k \approx (1-\rho^2)\rho^2 \sum_{k=0}^\infty \rho^{2k} \cdot |\tilde{w}^{in}_1|^2 = |\tilde{w}^{in}_1|^2 \rho^2.$$

For a reservoir where the input is spread equally across modes ($|\tilde{w}^{in}_i| = c$ for all $i$), the total is $MC = Nc^2 \cdot \rho^2 / (1-\rho^2) \cdot (1-\rho^2) = Nc^2\rho^2$, bounded by $N$ as expected. This scales as $\rho^2$ for small $\rho$, confirming the intuition that higher spectral radius means more total memory.

## 8.2.3 Effect on Stability: The Jacobian Argument

The spectral radius affects not only memory but also stability. For the nonlinear reservoir

$$\mathbf{r}(t) = \tanh(W\mathbf{r}(t-1) + \mathbf{w}^{in} u_t),$$

the *Jacobian* of the map $\mathbf{r}(t-1) \mapsto \mathbf{r}(t)$ at a point $\mathbf{r}^*$ is

$$J = \text{diag}(\tanh'(a_i^*)) \cdot W,$$

where $a_i^* = (W\mathbf{r}^* + \mathbf{w}^{in} u)_i$ are the pre-activation values and $\tanh'(a) = 1 - \tanh^2(a) \in (0, 1]$.

The spectral radius of $J$ is

$$\rho(J) = \rho\bigl(\text{diag}(\tanh'(a_i^*)) \cdot W\bigr) \leq \|\text{diag}(\tanh'(a_i^*))\|_2 \cdot \|W\|_2 \leq \rho(W) = \rho.$$

(Using $\tanh'(a) \leq 1$ for all $a$, so $\|\text{diag}(\tanh')\|_2 \leq 1$.)

**Implication:** The echo state property (contractivity of the reservoir map) is guaranteed when $\rho(J) < 1$, which is guaranteed when $\rho < 1$. More precisely, the reservoir is a contraction mapping if $\rho < 1$, ensuring the unique attractor (echo state property).

**Near saturation:** When $|a_i^*|$ is large, $\tanh'(a_i^*) \approx 0$, and the effective spectral radius is much smaller than $\rho$. Saturated neurons contribute little to the reservoir dynamics — they are "silent." This is why a reservoir driven with very large inputs can effectively have much smaller memory capacity than $\rho$ would suggest.

**Near the linear regime:** When $|a_i^*|$ is small, $\tanh'(a_i^*) \approx 1$, and $\rho(J) \approx \rho(W)$. The tanh nonlinearity is transparent and the reservoir behaves approximately linearly, so the theoretical formula $MC_k \propto \rho^{2k}$ is approximately correct.

## 8.2.4 The Edge of Stability Argument

The phrase "edge of chaos" (or "edge of stability") refers to the regime $\rho \approx 1$, where the reservoir is near the boundary between stable (contractive) and unstable (expanding) dynamics. This regime has attracted significant attention — both theoretical and practical — for the following reasons.

**Argument for the edge:**

1. **Memory.** As $\rho \to 1^-$, total memory capacity $MC \to N$ (for a linear reservoir). The reservoir can potentially store the maximum amount of information about its input history.

2. **Criticality.** In statistical physics, systems near phase transitions exhibit scale-free correlations and heightened sensitivity to inputs. Analogously, a reservoir near $\rho = 1$ has state correlations that decay slowly (power-law, approximately) rather than exponentially. This can improve the representation of inputs with long-range temporal dependencies.

3. **Computational richness.** It has been argued (informally) that the rich dynamics near $\rho = 1$ provide a more expressive function basis for the readout to exploit.

**Argument against the edge:**

1. **Stability in practice.** For $\rho$ very close to 1 with tanh nonlinearity, the reservoir is sensitive to initial conditions and can exhibit long transients. The "washout period" needed before training states becomes very long.

2. **Task dependence.** For tasks with short-range temporal dependencies, $\rho \ll 1$ is optimal. Using $\rho \approx 1$ wastes capacity on long-range memory that is not needed.

3. **The tanh effect.** The nonlinearity means that $\rho(J) < \rho(W)$ whenever the reservoir is not in the linear regime. A reservoir with $\rho(W) = 0.99$ but heavy saturation may have effective dynamics with spectral radius $0.7$, behaving like a reservoir with $\rho = 0.7$.

**Practical recommendation** [Lukosevicius2012]: Start with $\rho$ slightly less than 1 (e.g., $\rho = 0.9$) and tune based on the task's temporal structure. For tasks with long-range dependencies, try $\rho \in [0.9, 0.99]$. For classification tasks with short-range dependencies, try $\rho \in [0.5, 0.9]$.

## 8.2.5 Task-Dependent Optimal Spectral Radius

To make the advice quantitative, consider a task where the target output is a function of inputs from the past $\tau$ timesteps. The task's temporal horizon is $\tau$. To remember inputs from time $t - \tau$, we need $MC_\tau$ to be appreciable, i.e., $\rho^{2\tau}$ not negligibly small.

**Rule of thumb:** Choose $\rho$ such that

$$\rho^{2\tau} \geq \epsilon,$$

i.e.,

$$\rho \geq \epsilon^{1/(2\tau)} = e^{\ln\epsilon / (2\tau)}.$$

For $\epsilon = 0.01$ and $\tau = 10$: $\rho \geq 0.01^{1/20} = 10^{-2/20} = 10^{-0.1} \approx 0.79$.

For $\epsilon = 0.01$ and $\tau = 100$: $\rho \geq 0.01^{1/200} = 10^{-0.01} \approx 0.977$.

These thresholds give the *minimum* $\rho$ needed to remember inputs from $\tau$ steps ago with at least 1% of the possible correlation. In practice, $\rho$ should be tuned above this threshold by cross-validation.

**Complementary constraint:** The reservoir must satisfy the ESP, which requires $\rho < 1$ (for the linear case; for the nonlinear case, $\rho < 1$ is sufficient but not necessary). The practical upper limit is set by the washout time: longer $\tau_{washout}$ is acceptable if the training sequence is long.

## 8.2.6 Spectral Radius and the State Covariance Matrix

One often-overlooked effect of spectral radius is its impact on the *conditioning* of the state covariance matrix $R_{\mathbf{rr}}$. For ridge regression, the effective system matrix is $R_{\mathbf{rr}} + \lambda I$, and the numerical conditioning of this matrix affects the quality of the readout.

For a linear reservoir, the $(i,i)$ entry of $R_{\mathbf{rr}}$ in the eigenbasis is $|\tilde{w}^{in}_i|^2 / (1 - |\lambda_i|^2)$. As $|\lambda_i| \to 1$, this diverges. This means that modes with eigenvalues near 1 have very large variance in the state, while modes with eigenvalues near 0 have small variance. The resulting state covariance matrix becomes increasingly ill-conditioned as $\rho \to 1$.

This ill-conditioning makes ridge regression more aggressive (higher effective regularization relative to the signal variance in low-eigenvalue modes) and can suppress the contribution of low-eigenvalue modes to the readout. This is one mechanism by which high $\rho$ does *not* automatically translate to better task performance: the readout may not be able to exploit the long-range memory of slow modes because they are over-regularized.

The solution is adaptive regularization: use separate regularization strengths for different eigenvalue bands of $R_{\mathbf{rr}}$, or pre-whiten the states before regression. Section 8.5 discusses this in more detail.

---

*The spectral radius sets the timescale of memory. The next section analyzes how input scaling controls the nonlinear character of that memory.*
