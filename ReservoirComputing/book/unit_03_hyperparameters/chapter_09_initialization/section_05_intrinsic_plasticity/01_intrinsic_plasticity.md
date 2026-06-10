# Section 9.5: Intrinsic Plasticity

## 9.5.1 The Problem with Default Initialization

When you build a reservoir and run it on input data, the neurons spend a substantial fraction of their time saturated near $\pm 1$. This is wasteful: a saturated neuron has $\tanh'(a) \approx 0$, contributing almost nothing to the reservoir's gradient and carrying little information about the input. Its activation value is "stuck" and effectively removes one dimension from the reservoir's active representation.

The problem has a clear information-theoretic formulation: a neuron whose output is always near $+1$ has low entropy and therefore carries little information. Maximum information transmission through a neuron requires that its output distribution has maximum entropy, which for a bounded output in $(-1, 1)$ means an output distribution close to uniform.

This observation motivates *intrinsic plasticity* (IP): an unsupervised local learning rule that adapts each neuron's *gain* $a_i$ and *bias* $b_i$ to shift the operating point, moving the neuron out of saturation and toward a maximally informative regime. Triesch [Triesch2005] derived IP from the infomax principle, and it has since become a standard tool for improving the performance of random reservoirs without touching the recurrent weights.

## 9.5.2 The Neuron Model

Rather than the standard sigmoid, consider a neuron with parameterized gain and bias:

$$y_i = f(a_i x_i + b_i),$$

where $x_i$ is the total synaptic input (the weighted sum of recurrent activations plus external input), $a_i > 0$ is the gain, $b_i$ is the bias, and $f = \tanh$ (or sometimes the logistic sigmoid $f(x) = 1/(1+e^{-x})$). The parameters $(a_i, b_i)$ are what IP adapts.

For the reservoir neuron $i$, the total input at time $t$ is

$$x_i(t) = \sum_j W_{ij} r_j(t-1) + W^{in}_i u_t,$$

and the activation is

$$r_i(t) = \tanh\bigl(a_i \cdot x_i(t) + b_i\bigr).$$

Note that $a_i = 1$, $b_i = 0$ recovers the standard ESN. IP adapts $(a_i, b_i)$ online, in the direction that maximizes information transmission through neuron $i$.

## 9.5.3 The Infomax Principle

**Objective.** Maximize the mutual information between the neuron's input $x_i$ and output $y_i$:

$$\max_{a_i, b_i} I(x_i; y_i).$$

Since $y_i = f(a_i x_i + b_i)$ is a deterministic function of $x_i$, the mutual information equals the entropy of the output (no noise in this idealized model):

$$I(x_i; y_i) = H(y_i) - H(y_i | x_i) = H(y_i) - 0 = H(y_i).$$

So we want to **maximize the output entropy** $H(y_i)$.

**Constraint.** We know the form of the transfer function $f = \tanh$. The maximum entropy distribution of a random variable with support $(-1, 1)$ (for tanh) is the uniform distribution on $(-1, 1)$.

Actually, maximizing entropy subject to a fixed (nonlinear) transfer function is more subtle. The infomax principle for a neuron with transfer function $f$ and input distribution $p_x(x)$ gives:

**Proposition 9.5.1.** The output distribution of $y = f(a x + b)$ is

$$p_y(y) = \frac{p_x\!\left(\frac{f^{-1}(y) - b}{a}\right)}{a \cdot |f'(f^{-1}(y))|}.$$

The entropy $H(y)$ is maximized when $p_y(y) = q(y)$ for some target distribution $q$. For the logistic sigmoid $f(x) = 1/(1+e^{-x})$, the natural target is the exponential distribution; for tanh, it is a related distribution. Triesch's original derivation used the logistic sigmoid.

## 9.5.4 Derivation for the Logistic Sigmoid

Let $f(x) = \sigma(x) = 1/(1+e^{-x}) \in (0,1)$. The inverse is $f^{-1}(y) = \ln(y/(1-y))$ and $f'(x) = \sigma(x)(1-\sigma(x)) = y(1-y)$.

For the output to have an exponential distribution $q(y) = \mu e^{-\mu y}$ on $y > 0$ (adjusting for the bounded support, this is an approximation for inputs near 0), we require:

$$p_y(y) = q(y) = \mu e^{-\mu y}.$$

This is the target. The IP update is derived by gradient ascent on the log-likelihood of the output under the target distribution:

$$\mathcal{L}(a_i, b_i) = \mathbb{E}_{x_i}[\log q(f(a_i x_i + b_i))].$$

For the exponential target $q(y) = \mu e^{-\mu y}$:

$$\log q(y) = \log \mu - \mu y = \text{const} - \mu y.$$

Since $y = f(a_i x_i + b_i)$, maximizing $\mathcal{L}$ with respect to $a_i$ and $b_i$ amounts to minimizing $\mathbb{E}[y] = \mathbb{E}[f(a_i x_i + b_i)]$ — the expected output activation — which pushes the neuron toward lower mean activation. But this alone would simply push $b_i \to -\infty$. The infomax objective has an additional entropy term.

The correct derivation uses the fact that maximizing $H(y) = -\mathbb{E}[\log p_y(y)]$ is equivalent to maximizing the log-likelihood $\mathbb{E}[\log q(y)]$ when $q$ is the maximum-entropy target.

**Gradient computation.** Take the gradient of $\mathcal{L}$ with respect to $b_i$:

$$\frac{\partial \mathcal{L}}{\partial b_i} = \mathbb{E}\left[\frac{d\log q(y)}{dy} \cdot \frac{dy}{db_i}\right] = \mathbb{E}\left[\frac{q'(y)}{q(y)} \cdot f'(a_i x_i + b_i)\right].$$

For $q(y) = \mu e^{-\mu y}$: $q'(y)/q(y) = -\mu$.

For $f(x) = \sigma(x)$: $f'(x) = y(1-y)$.

So:

$$\frac{\partial \mathcal{L}}{\partial b_i} = -\mu \mathbb{E}[y(1-y)].$$

This is negative — gradient descent would decrease $b_i$ without bound. We need to add the entropy term explicitly. The full infomax gradient on the log-determinant is:

$$\frac{\partial}{\partial b_i} H(y) = \frac{\partial}{\partial b_i} \mathbb{E}[\log |f'(a_i x_i + b_i)|] + \frac{\partial}{\partial b_i} H(x_i).$$

The second term is zero (the input entropy doesn't depend on $b_i$). So:

$$\frac{\partial H(y)}{\partial b_i} = \mathbb{E}\left[\frac{\partial}{\partial b_i} \log(y(1-y))\right] = \mathbb{E}\left[\frac{(1-2y) \cdot y(1-y)}{y(1-y)}\right] = \mathbb{E}[1-2y].$$

**Combining infomax with the target distribution.** Triesch's derivation combines the entropy maximization with the constraint that the output matches the target exponential distribution $q(y) = \mu e^{-\mu y}$. The resulting update rules are (in stochastic gradient form, for one sample at time $t$):

$$\Delta b_i = \eta \left(1 - (\mu + 1) y_i + \mu y_i^2\right) = \eta(1 - (2 + \mu/y_i) y_i + \mu y_i),$$

which simplifies to [Triesch2005]:

$$\boxed{\Delta b_i = \eta\bigl(1 - (2 + \mu)y_i + \mu y_i^2\bigr),}$$

where $\eta$ is the learning rate and $\mu$ is the mean of the target exponential distribution.

For the gain:

$$\Delta a_i = \eta\left(\frac{1}{a_i} + x_i - (2 + \mu) x_i y_i + \mu x_i y_i^2\right).$$

Wait — let us be more careful. The full derivation proceeds as follows.

## 9.5.5 Complete Derivation of IP Update Rules

Starting from the KL divergence objective (equivalent to maximizing the expected log-likelihood of the output under target $q$):

$$\mathcal{L}(a,b) = \mathbb{E}[\log q(y)] + H(y) = \mathbb{E}[\log q(y)] + \mathbb{E}[\log|f'(ax+b)/a|] + \text{const}.$$

But $H(y) = H(x) + \mathbb{E}[\log|df^{-1}/dy|] = H(x) - \mathbb{E}[\log|a f'(ax+b)|]$ (change of variables). So maximizing $H(y)$ subject to the target $q$ is equivalent to minimizing the KL divergence $D_{KL}(p_y \| q)$.

For the KL divergence:

$$D_{KL}(p_y \| q) = -H(y) - \mathbb{E}[\log q(y)].$$

Taking the stochastic gradient:

$$-\frac{\partial D_{KL}}{\partial b} = \frac{\partial H(y)}{\partial b} + \frac{\partial}{\partial b}\mathbb{E}[\log q(y)].$$

For the exponential target $q(y) = \mu e^{-\mu y}$ and logistic sigmoid:

$$\frac{\partial}{\partial b}\mathbb{E}[\log q(y)] = \frac{\partial}{\partial b}\mathbb{E}[\log \mu - \mu y] = -\mu \mathbb{E}\left[\frac{\partial y}{\partial b}\right] = -\mu \mathbb{E}[y(1-y)].$$

$$\frac{\partial H(y)}{\partial b} = \mathbb{E}[1 - 2y] \quad \text{(from the entropy gradient computed above)}.$$

Combining:

$$\Delta b = \eta(1 - 2y - \mu y(1-y)) = \eta(1 - 2y - \mu y + \mu y^2).$$

This is Triesch's formula with slight rearrangement. For the gain $a$:

$$\Delta a = \eta\left(\frac{1}{a} + x(1-2y) - \mu xy(1-y)\right) = \eta\left(\frac{1}{a} + x\bigl(1 - 2y - \mu y + \mu y^2\bigr)\right).$$

Noting that the quantity in the parenthesis for $\Delta b$ is $\Delta b / \eta$:

$$\boxed{\Delta a = \eta\!\left(\frac{1}{a} + x \cdot \frac{\Delta b}{\eta}\right), \qquad \Delta b = \eta(1 - (2+\mu)y + \mu y^2).}$$

Here $y = \sigma(ax + b)$ is the current output and $x$ is the current input.

## 9.5.6 Target Distribution and the Role of $\mu$

The parameter $\mu$ sets the mean of the exponential target distribution: $\mathbb{E}_{q}[y] = 1/\mu$ (for an exponential on $(0,\infty)$; for the bounded output of the logistic sigmoid, the exponential is approximated). The effect of $\mu$:

- **Large $\mu$ (small mean):** The target is a distribution concentrated near $y = 0$. IP will push neurons toward producing small outputs on average, avoiding saturation near 1 but also avoiding the linear regime.
- **Small $\mu$ (large mean):** The target is a more uniform distribution over the sigmoid's range.

A typical choice is $\mu = 0.1$ to $0.5$, aiming to spread the output distribution across the sigmoid's active range while avoiding both saturation at 0 and saturation at 1 (for the logistic sigmoid output in $(0,1)$).

**Why exponential?** The exponential distribution is the maximum-entropy distribution on the positive reals with a given mean. For a neuron whose output is approximately bounded to $[0, 1]$ (logistic sigmoid) or $[-1, 1]$ (tanh, for which a modified formula applies), the exponential-like distribution maximizes output entropy subject to a mean constraint, which is exactly the infomax objective.

## 9.5.7 Algorithm: IP in the Reservoir

IP is applied as an online unsupervised pre-training step before the reservoir is used for a task:

```python
def intrinsic_plasticity_step(a, b, x, eta=0.001, mu=0.1):
    """
    Apply one IP update step.
    a, b: current gain and bias (scalars for one neuron)
    x: current input (scalar)
    eta: learning rate
    mu: target exponential distribution mean (mu = 1/mean)
    Returns: updated (a, b)
    """
    import numpy as np
    y = 1.0 / (1.0 + np.exp(-(a * x + b)))  # logistic sigmoid
    
    delta_b = eta * (1 - (2 + mu) * y + mu * y**2)
    delta_a = eta * (1.0 / a + x * (1 - (2 + mu) * y + mu * y**2))
    
    return a + delta_a, b + delta_b

def run_ip_pretraining(W, W_in, u_seq, eta=0.0005, mu=0.1, washout=500):
    """
    Pre-train reservoir using IP, returning adapted gains and biases.
    """
    N = W.shape[0]
    a = np.ones(N)
    b = np.zeros(N)
    r = np.zeros(N)
    
    for t, ut in enumerate(u_seq):
        # Compute total input for each neuron
        x = W @ r + W_in * ut  # element-wise for single scalar input
        
        # IP update for each neuron
        for i in range(N):
            a[i], b[i] = intrinsic_plasticity_step(a[i], b[i], x[i], eta, mu)
        
        # Update state using adapted parameters
        r = 1.0 / (1.0 + np.exp(-(a * x + b)))  # logistic sigmoid with IP
        
    return a, b
```

After IP pre-training, the gains $\{a_i\}$ and biases $\{b_i\}$ are fixed, and the reservoir is run as usual for the downstream task.

## 9.5.8 Interaction with the Echo State Property

A key practical question: does IP adaptation preserve the echo state property? The ESP depends on the effective spectral radius of the Jacobian $J = \text{diag}(\sigma'(a_i x_i + b_i) \cdot a_i) \cdot W$. After IP adaptation, the gain $a_i$ may have increased or decreased from its initial value of 1. If gains grow large, the Jacobian spectral radius may exceed 1, violating the ESP.

**Safeguard:** Clamp the gains $a_i \in [a_{min}, a_{max}]$ during IP adaptation to prevent divergence. Typical bounds: $a_i \in [0.1, 5.0]$. This ensures the Jacobian spectral radius remains controlled.

**Theoretical analysis:** After IP convergence, the output distribution of each neuron is approximately exponential with mean $1/\mu$. The mean gain $\mathbb{E}[\sigma'(a_i x_i + b_i) \cdot a_i]$ can be computed from the target distribution: for the exponential distribution,

$$\mathbb{E}[\sigma(y)(1-\sigma(y)) \cdot a_i] \approx \bar{g}_{IP} \cdot a_i,$$

where $\bar{g}_{IP}$ is the mean derivative at the IP-equilibrium operating point. The ESP requires $\bar{g}_{IP} \cdot a_i \cdot \rho_W < 1$ for all neurons, which is satisfied if the IP equilibrium operating point is not too linear (not too large $a_i$).

## 9.5.9 Effect on Reservoir Performance

Empirically [Triesch2005, Schrauwen2008], IP pre-training improves ESN performance on a variety of tasks, with the most significant improvements observed when:

1. The initial reservoir has many saturated neurons (large $\sigma_{in}$, high $\rho$).
2. The task requires the reservoir to process fine distinctions between similar inputs.
3. The input distribution is approximately Gaussian (the exponential target is a good match for Gaussian inputs mapped through the sigmoid).

The improvement is typically 10–30% reduction in NRMSE for nonlinear tasks. For tasks dominated by linear processing, IP can sometimes *hurt* performance slightly, because the exponential target distribution encourages asymmetric neuron activations that can introduce a bias in the linear readout.

---

*Intrinsic plasticity adapts the nonlinearity to the input distribution, without changing the recurrent structure. This is a form of unsupervised learning that is consistent with the fixed-reservoir philosophy. The next major category of learning — FORCE learning (Chapter 11) — abandons this constraint and adapts the recurrent weights themselves.*
