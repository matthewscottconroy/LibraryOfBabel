# Section 8.3: Input Scaling

## 8.3.1 The Operating Point of a Neuron

Every tanh neuron in the reservoir processes its input through the function $f(a) = \tanh(a)$, where $a = \mathbf{w}^\top \mathbf{r}(t-1) + w^{in} u_t$ is the pre-activation. The behavior of this nonlinearity changes dramatically with the magnitude of $a$:

- For $|a| \ll 1$: $\tanh(a) \approx a$. The neuron is in the **linear regime** — it faithfully amplifies its input with unit gain.
- For $|a| \approx 1$: the neuron is in the **transition regime**, where $\tanh$ curves away from linearity.
- For $|a| \gg 1$: $\tanh(a) \approx \pm 1$. The neuron is **saturated** — it has lost sensitivity to its input and produces a nearly constant output.

The input scaling $\sigma_{in}$ controls the scale of the entries of $W^{in}$, and therefore the typical magnitude of the contribution $w^{in} u_t$ to the pre-activation. If $\sigma_{in}$ is small, the input nudges the reservoir gently and the neurons operate near the linear regime. If $\sigma_{in}$ is large, the input dominates and drives neurons into saturation. Between these extremes lies the transition zone, where the neurons' nonlinearity is most active.

This is not just a soft effect — it is quantitatively large. The effective Jacobian of the reservoir map (the matrix of partial derivatives $\partial r_i(t)/\partial r_j(t-1)$) changes its spectral properties dramatically with $\sigma_{in}$, as we will show.

## 8.3.2 The Effective Jacobian: Full Derivation

The reservoir update is

$$\mathbf{r}(t) = \tanh\bigl(W\mathbf{r}(t-1) + W^{in} u_t\bigr).$$

Let $W^{in} = \sigma_{in} \tilde{W}^{in}$ where $\tilde{W}^{in}$ has entries of order 1. The pre-activation vector at time $t$ is

$$\mathbf{a}(t) = W\mathbf{r}(t-1) + \sigma_{in}\tilde{W}^{in} u_t.$$

Define $D(t) = \text{diag}(\tanh'(a_i(t))) = \text{diag}(1 - r_i(t)^2)$. The Jacobian of the map $\mathbf{r}(t-1) \mapsto \mathbf{r}(t)$ is

$$J(t) = D(t) W.$$

The spectral radius of $J(t)$ is

$$\rho(J(t)) = \rho\bigl(D(t) W\bigr).$$

For a fixed recurrent matrix $W$ with spectral radius $\rho_W$, the effective spectral radius $\rho(J)$ is compressed by the diagonal factors $d_i = 1 - r_i^2$:

**Linear regime** ($|a_i| \approx 0$): $d_i = 1 - \tanh^2(0) = 1$, so $D \approx I$ and $\rho(J) \approx \rho_W$.

**Saturated regime** ($|a_i| \gg 1$): $d_i = 1 - \tanh^2(a_i) \approx 4e^{-2|a_i|} \approx 0$, so $D \approx 0$ and $\rho(J) \approx 0$.

**Intermediate regime**: $d_i$ takes intermediate values, and $\rho(J) \in (0, \rho_W)$.

To see the effect of $\sigma_{in}$ on the operating point, we need the stationary distribution of $|a_i|$.

## 8.3.3 Stationary Analysis: How $\sigma_{in}$ Sets the Operating Point

In the stationary regime (after the reservoir has been driven for a long time), the pre-activation $a_i$ of neuron $i$ has some distribution. Let us analyze this distribution as a function of $\sigma_{in}$.

**Two contributions to $a_i$:**

$$a_i(t) = \underbrace{\sum_j W_{ij} r_j(t-1)}_{\text{recurrent contribution}} + \underbrace{\sigma_{in} \tilde{w}^{in}_i u_t}_{\text{input contribution}}.$$

The recurrent contribution has variance $\sum_j W_{ij}^2 \operatorname{Var}[r_j]$, which depends on the reservoir's stationary variance $\sigma_r^2 = \operatorname{Var}[r_i]$ (approximately the same for all $i$ in a homogeneous reservoir).

The input contribution has variance $\sigma_{in}^2 (\tilde{w}^{in}_i)^2 \sigma_u^2$ where $\sigma_u^2 = \operatorname{Var}[u_t]$.

In the linear approximation ($r_j \approx a_j$ for small activations), the stationary variance satisfies

$$\sigma_r^2 \approx \sum_j W_{ij}^2 \sigma_r^2 + \sigma_{in}^2 (\tilde{w}^{in}_i)^2 \sigma_u^2,$$

giving

$$\sigma_r^2 \approx \frac{\sigma_{in}^2 \overline{(w^{in})^2} \sigma_u^2}{1 - \sum_j W_{ij}^2}.$$

The denominator $1 - \sum_j W_{ij}^2$ involves the row-sum of squared weights, which scales as $\rho_W^2 / N$ for sparse random matrices. For large $N$, this becomes $1 - \rho_W^2/N \approx 1$, giving

$$\sigma_r^2 \approx \sigma_{in}^2 \overline{(w^{in})^2} \sigma_u^2 / (1 - \rho_W^2 \cdot p),$$

where $p$ is the connection probability and we have used the mean squared row sum.

The key point: $\sigma_r \propto \sigma_{in}$ in the linear regime. As $\sigma_{in}$ grows, the activations grow proportionally and neurons begin to saturate.

## 8.3.4 The Mean Gain as a Function of $\sigma_{in}$

Define the **mean gain** of the reservoir:

$$\bar{g}(\sigma_{in}) = \mathbb{E}_{a \sim p_a(\cdot; \sigma_{in})}[\tanh'(a)] = \mathbb{E}[1 - r^2],$$

where $p_a$ is the stationary distribution of the pre-activation. This is the average diagonal element of $D(t)$.

The effective spectral radius is approximately

$$\rho_{eff} \approx \bar{g}(\sigma_{in}) \cdot \rho_W.$$

For small $\sigma_{in}$: $\bar{g} \approx 1$, so $\rho_{eff} \approx \rho_W$.

For large $\sigma_{in}$: $\bar{g} \to 0$ (all neurons saturate), so $\rho_{eff} \to 0$.

**Implication:** Increasing $\sigma_{in}$ has the *same qualitative effect as decreasing $\rho_W$* — it compresses the effective dynamics, reduces memory capacity, and moves the reservoir toward a more "static" regime dominated by the current input. But it does so in a fundamentally nonlinear way: the compression is largest for neurons that are most strongly driven, and the residual nonlinearity of the gain function creates quadratic and higher-order interactions.

Let us be more precise. For a symmetric zero-mean distribution $p_a$ with variance $\sigma_a^2$, the mean gain is

$$\bar{g} = \mathbb{E}[\tanh'(a)] = \int_{-\infty}^\infty (1 - \tanh^2(a)) p_a(a) \, da.$$

For a Gaussian $p_a = \mathcal{N}(0, \sigma_a^2)$:

$$\bar{g}(\sigma_a) = 1 - \mathbb{E}[\tanh^2(a)] = 1 - \int_{-\infty}^\infty \tanh^2(a) \frac{e^{-a^2/(2\sigma_a^2)}}{\sqrt{2\pi}\sigma_a} da.$$

This integral does not have a closed form, but it is easily evaluated numerically. Some key values:

| $\sigma_a$ | $\bar{g}(\sigma_a)$ | $\mathbb{E}[\tanh^2(a)]$ |
|------------|---------------------|--------------------------|
| 0.1        | 0.990               | 0.010                    |
| 0.5        | 0.910               | 0.090                    |
| 1.0        | 0.699               | 0.301                    |
| 2.0        | 0.345               | 0.655                    |
| 5.0        | 0.055               | 0.945                    |

The transition from near-linear ($\bar{g} \approx 1$) to saturated ($\bar{g} \approx 0$) occurs around $\sigma_a \approx 1$. Since $\sigma_a \propto \sigma_{in}$, the transition in terms of $\sigma_{in}$ occurs at $\sigma_{in} \approx 1/\|\tilde{W}^{in}\|$ (the inverse of the typical input weight magnitude).

## 8.3.5 Nonlinearity Quantification: The Taylor Expansion

To understand what kind of nonlinearity $\sigma_{in}$ introduces, expand $\tanh$ in a Taylor series around $a = 0$:

$$\tanh(a) = a - \frac{a^3}{3} + \frac{2a^5}{15} - \cdots$$

For a pre-activation $a = \mathbf{w}^\top \mathbf{r}(t-1) + \sigma_{in} w^{in} u_t$, the output of neuron $i$ contains the terms:

$$r_i(t) = \underbrace{a_i}_{\text{linear}} - \underbrace{\frac{a_i^3}{3}}_{\text{cubic}} + O(a_i^5).$$

The cubic term introduces third-order interactions. Since $a_i$ contains $\sigma_{in} w^{in}_i u_t$, the cubic term includes contributions of the form $(\sigma_{in})^3 (w^{in}_i)^3 u_t^3$ (third power of current input), $(\sigma_{in})^2 (w^{in}_i)^2 u_t^2 \cdot (\text{linear in } \mathbf{r})$ (mixed terms), and $\sigma_{in} w^{in}_i u_t \cdot (\text{quadratic in } \mathbf{r})$.

This shows concretely how $\sigma_{in}$ scales the nonlinear contributions:

- **Small $\sigma_{in}$**: cubic term scales as $(\sigma_{in})^3 \ll \sigma_{in}$. Nonlinear contributions are negligible. The reservoir is approximately linear.
- **$\sigma_{in} \approx 1$**: cubic term is comparable to the linear term. Significant nonlinear mixing of the input with the recurrent state.
- **Large $\sigma_{in}$**: the Taylor series is a poor approximation (the input drives neurons far from 0), but the key point is that the output is nearly binary ($\pm 1$), destroying information about the fine structure of the input.

## 8.3.6 Task-Dependent Optimal Input Scaling

The optimal $\sigma_{in}$ depends on whether the task requires:

**Linearity:** If the target function $y(t) = f(u_t)$ is linear in $u_t$ (e.g., simple filtering), then small $\sigma_{in}$ is optimal — it keeps the reservoir in the linear regime where it acts as a linear filter bank with high memory capacity.

**Mild nonlinearity:** If the task requires quadratic terms (e.g., $y(t) = u_t^2$), moderate $\sigma_{in}$ is better — it activates the nonlinearity without saturating the neurons.

**Strong nonlinearity (classification):** For tasks where the input needs to be mapped to a discrete category, larger $\sigma_{in}$ may be appropriate — the saturated regime produces more binary-like representations that can be more easily separated by a linear classifier.

**Empirical guidance** [Lukosevicius2012]:
- Start with $\sigma_{in}$ such that the typical pre-activation variance is $\sigma_a^2 \approx 1$ (in the transition zone).
- For regression tasks: bias toward smaller $\sigma_{in}$ (linear regime).
- For classification tasks: bias toward larger $\sigma_{in}$ (nonlinear regime).
- Always tune $\sigma_{in}$ jointly with $\rho$: the two parameters interact strongly.

## 8.3.7 The $(\rho, \sigma_{in})$ Interaction

To see the interaction concretely: suppose the task has a long temporal horizon (needing $\rho$ close to 1) but also requires nonlinear processing of the current input (needing $\sigma_{in}$ large enough to activate the tanh).

If we set $\rho = 0.99$ (for long memory) and $\sigma_{in} = 2.0$ (for nonlinearity), the effective spectral radius drops to approximately $\rho_{eff} = \bar{g}(2.0) \cdot 0.99 \approx 0.345 \cdot 0.99 \approx 0.34$, which corresponds to very short memory ($MC_k \propto 0.34^{2k} = 0.116^k$). The attempt to achieve long memory has been undone by the input saturation.

This is the fundamental interaction: large $\sigma_{in}$ destroys the memory benefit of large $\rho$. In the $(\rho, \sigma_{in})$ plane, there is a "usable" region where the reservoir has both adequate memory and adequate nonlinearity, bounded by:

- **Below**: $\rho$ too small → no long-range memory.
- **Left**: $\sigma_{in}$ too small → no nonlinear processing.
- **Right**: $\sigma_{in}$ too large → saturation destroys memory.
- **Above**: $\rho$ too large → instability or very long washout periods.

Grid search over this 2D space (as in Lab 8.1) will reveal the task-specific structure of this usable region.

---

*With $\rho$ setting the timescale of memory and $\sigma_{in}$ setting the nonlinear character of processing, the third major hyperparameter — the leak rate $\alpha$ — introduces a structural timescale through the reservoir architecture itself.*
