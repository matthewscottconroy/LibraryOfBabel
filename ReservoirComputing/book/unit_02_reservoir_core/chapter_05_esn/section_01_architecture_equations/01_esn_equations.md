# Section 1: ESN Architecture and Equations

## 1.1 The Basic ESN

An echo state network consists of three layers:

1. An **input layer** that projects an external signal $u_t \in \mathbb{R}^K$ into the reservoir.
2. A **reservoir** of $N$ recurrently connected neurons whose state $x_t \in \mathbb{R}^N$ evolves over time.
3. A **readout layer** that maps the reservoir state $x_t$ (and sometimes the input $u_t$) to an output $y_t \in \mathbb{R}^L$.

Only the readout weights $W^{out} \in \mathbb{R}^{L \times N}$ are trained. The input weights $W^{in} \in \mathbb{R}^{N \times K}$ and recurrent weights $W^{rec} \in \mathbb{R}^{N \times N}$ are fixed at initialization.

In its simplest form, the ESN state update is:

$$x_{t+1} = \tanh\!\left(W^{rec} x_t + W^{in} u_{t+1} + b\right) \tag{1.1}$$

and the readout is:

$$y_{t+1} = W^{out} x_{t+1} \tag{1.2}$$

where $b \in \mathbb{R}^N$ is a bias vector (often set to zero in practice, but important for symmetry breaking). This is the "vanilla" ESN. In most practical applications, however, we use the **leaky integrator** form, which we now derive from first principles.

---

## 1.2 The Continuous-Time Rate Model

To understand where the ESN comes from, we must look at the continuous-time neural dynamics that the ESN approximates. Consider $N$ neurons, each characterized by its firing rate $r_i(t) \in [0, 1]$. The standard rate model in computational neuroscience describes how the "activation variable" $x_i(t)$ — a quantity proportional to the weighted input to neuron $i$ — evolves over time:

$$\tau \frac{dx_i}{dt} = -x_i(t) + \sum_{j=1}^{N} W^{rec}_{ij} f(x_j(t)) + \sum_{k=1}^{K} W^{in}_{ik} u_k(t) + b_i \tag{1.3}$$

Here:
- $\tau > 0$ is the **membrane time constant** of the neurons, with units of time.
- $x_i(t)$ is the activation (pre-nonlinearity state) of neuron $i$.
- $f : \mathbb{R} \to \mathbb{R}$ is the activation function, typically $\tanh$.
- $W^{rec}_{ij}$ is the synaptic weight from neuron $j$ to neuron $i$.
- $W^{in}_{ik}$ is the weight from input channel $k$ to neuron $i$.

In vector form:

$$\tau \dot{x} = -x + W^{rec} f(x) + W^{in} u + b \tag{1.4}$$

where $f$ is applied elementwise. This equation has a clear physical interpretation. The term $-x$ is a **leak**: in the absence of any input or recurrent drive, the activation decays exponentially to zero with time constant $\tau$. The term $W^{rec} f(x)$ is the **recurrent drive**: the weighted sum of all other neurons' firing rates. The term $W^{in} u$ is the **external input drive**.

The single parameter $\tau$ controls the timescale of the dynamics. When $\tau$ is large, neurons integrate over long windows and the network is "sluggish." When $\tau$ is small, neurons respond quickly and the network can track rapid changes.

---

## 1.3 Euler Discretization and the Leaky Integrator

We discretize equation (1.4) using the forward Euler method with step size $\Delta t$:

$$\frac{x_{t+1} - x_t}{\Delta t} \approx \frac{1}{\tau}\left(-x_t + W^{rec} f(x_t) + W^{in} u_{t+1} + b\right)$$

Rearranging:

$$x_{t+1} = x_t + \frac{\Delta t}{\tau}\left(-x_t + W^{rec} f(x_t) + W^{in} u_{t+1} + b\right)$$

$$x_{t+1} = \left(1 - \frac{\Delta t}{\tau}\right) x_t + \frac{\Delta t}{\tau}\left(W^{rec} f(x_t) + W^{in} u_{t+1} + b\right)$$

Now define the **leaking rate** $\alpha = \Delta t / \tau$. For this to be a valid discretization, we require $0 < \alpha \leq 1$. Substituting:

$$\boxed{x_{t+1} = (1 - \alpha) x_t + \alpha \tanh\!\left(W^{rec} x_t + W^{in} u_{t+1} + b\right)} \tag{1.5}$$

This is the **leaky integrator ESN**. The readout is unchanged:

$$y_{t+1} = W^{out} x_{t+1} \tag{1.6}$$

Note that the pre-nonlinearity term $W^{rec} x_t + W^{in} u_{t+1} + b$ uses $x_t$ (the previous state), not $f(x_t)$. This is because in the Euler discretization, the right-hand side is evaluated at time $t$. Some formulations use $f(x_t)$ here instead; the difference is a matter of convention about what $x$ represents (pre- or post-nonlinearity activation), and both are valid as long as one is consistent.

**Remark on notation.** In the literature you will also encounter formulations where the state $z_t = f(x_t)$ is tracked instead of the pre-nonlinearity activation $x_t$. In that case the update takes the equivalent form:

$$z_{t+1} = f\!\left((1-\alpha) f^{-1}(z_t) + \alpha\left(W^{rec} z_t + W^{in} u_{t+1} + b\right)\right)$$

which is less clean. We will consistently use $x_t$ as the post-nonlinearity state, yielding:

$$x_{t+1} = (1 - \alpha) x_t + \alpha f\!\left(W^{rec} x_t + W^{in} u_{t+1} + b\right) \tag{1.5'}$$

where we understand $f = \tanh$ applied componentwise.

---

## 1.4 What $\alpha$ Controls

The leaking rate $\alpha \in (0, 1]$ plays a central role in ESN dynamics.

**Memory timescale.** Consider the response of a single neuron to a delta-function input at time $t = 0$, with no recurrent connections. The update becomes $x_{t+1} = (1 - \alpha) x_t$, whose solution is $x_t = (1-\alpha)^t x_0$. This decays to $e^{-1}$ of its initial value after $t^* = 1/|\ln(1-\alpha)| \approx 1/\alpha$ time steps for small $\alpha$. Thus the effective **memory time constant** in discrete steps is approximately $1/\alpha$.

A small $\alpha$ means long memory but slow response. A large $\alpha$ (approaching 1) means fast response but short memory. In the limit $\alpha = 1$, equation (1.5) reduces to the vanilla ESN (1.1), and there is no explicit leak.

**Smoothing.** Equation (1.5) is a first-order infinite impulse response (IIR) filter applied to the "instantaneous" reservoir state $\tanh(W^{rec} x_t + W^{in} u_{t+1} + b)$. The parameter $1-\alpha$ is the pole of this filter. For $\alpha < 1$, the reservoir state $x_t$ is a weighted average of all past reservoir activations, with exponentially decaying weights. This smoothing can be beneficial for tasks requiring long-range temporal integration.

**Matching to input timescales.** A crucial practical insight is that $\alpha$ should be matched to the characteristic timescale of the input signal. If the input changes on a timescale of $T$ samples, then setting $\alpha \approx 1/T$ will ensure the reservoir state tracks variations at that scale. This is one of the most important hyperparameters to tune.

**Multiple timescales.** Different neurons can have different leaking rates, creating a reservoir with heterogeneous timescales. Nodes with small $\alpha$ act as slow integrators; nodes with large $\alpha$ respond to rapid fluctuations. This heterogeneity can dramatically improve performance on tasks with multi-scale temporal structure [Jaeger2007].

---

## 1.5 Special Cases

**$\alpha = 1$: Vanilla ESN.** The update reduces to:
$$x_{t+1} = \tanh\!\left(W^{rec} x_t + W^{in} u_{t+1} + b\right)$$
No explicit memory beyond what is encoded in the network's own state. This is the original ESN formulation [Jaeger2001]. Stable if $\rho(W^{rec}) < 1$ (see Section 3).

**$\alpha \to 0$: Continuous limit.** As $\alpha \to 0$ with $\alpha t$ held fixed, the discrete dynamics recover the continuous-time ODE (1.4). This limit is useful for analysis but impractical for computation.

**$\alpha = 0$: No dynamics.** The reservoir state is frozen at its initial value and carries no information about the input. This is degenerate and useless. We always assume $\alpha > 0$.

---

## 1.6 Alternative Activation Functions

The $\tanh$ nonlinearity is standard in ESNs for several reasons: it is bounded (preventing unbounded growth), smooth (enabling gradient-based analysis), odd-symmetric (simplifying theoretical analysis), and maps $\mathbb{R} \to (-1, 1)$ (matching natural reservoir state ranges). However, other choices are possible, each with tradeoffs.

### 1.6.1 Sigmoid (Logistic)

$$f(x) = \frac{1}{1 + e^{-x}} = \sigma(x) \tag{1.7}$$

**Range:** $(0, 1)$, not centered at zero. **Derivative:** $\sigma'(x) = \sigma(x)(1-\sigma(x)) \leq 1/4$.

The sigmoid is asymmetric: it is bounded below by 0. This means that even with zero input, neurons tend to fire at rate $1/2$, introducing a bias. The maximum derivative is $1/4$, which means the sigmoid is "less expanding" than $\tanh$ (whose maximum derivative is $1$), making the ESP easier to satisfy for a given $\|W^{rec}\|$, but potentially at the cost of reduced computational capacity.

### 1.6.2 Rectified Linear Unit (ReLU)

$$f(x) = \max(0, x) \tag{1.8}$$

**Range:** $[0, \infty)$. **Derivative:** $f'(x) = \mathbf{1}[x > 0]$.

The ReLU is unbounded above, which creates stability concerns for recurrent networks. For the vanilla ESN update $x_{t+1} = \text{ReLU}(W^{rec} x_t + W^{in} u_t)$, the echo state property requires $\rho(W^{rec}) < 1$ in a stronger sense than for $\tanh$ — specifically, the map must be a contraction in the positive orthant. More precisely, for inputs bounded in $[-M, M]^K$, the reservoir state is bounded if $\|W^{rec}\| < 1$. The ReLU is popular in feedforward networks but is rarely used in classical ESNs; it appears more naturally in deep reservoir architectures.

### 1.6.3 Hard Tanh and Clipping

$$f(x) = \text{clip}(x, -1, 1) = \begin{cases} -1 & x < -1 \\ x & -1 \leq x \leq 1 \\ 1 & x > 1 \end{cases} \tag{1.9}$$

This is a piecewise-linear approximation to $\tanh$. Its maximum derivative is $1$, matching $\tanh$ near zero. It is computationally cheaper and makes certain theoretical arguments cleaner (the activation function is explicitly Lipschitz-1). The saturating behavior (outputting $\pm 1$ for large inputs) is identical to $\tanh$ in effect, so the practical difference is small.

### 1.6.4 The Role of Boundedness

A key point: **bounded activation functions are necessary for the standard theory of ESNs.** The echo state property proof (Section 2) relies on the activation function being globally Lipschitz — that is, satisfying $|f(a) - f(b)| \leq L|a-b|$ for some constant $L$. For $\tanh$, we have $L = 1$ (since $|\tanh'(x)| \leq 1$ for all $x$). For the sigmoid $\sigma$, we have $L = 1/4$. For ReLU, the Lipschitz constant is $1$ but the function is unbounded, requiring separate treatment.

When $f$ is bounded and Lipschitz, the reservoir state space is effectively compact and the contraction mapping theorem applies in a clean form. When $f$ is unbounded, more care is needed.

---

## 1.7 The Full System: Summary

Let us collect all the equations for a complete ESN:

**Reservoir update (leaky integrator):**
$$x_{t+1} = (1-\alpha)x_t + \alpha \tanh\!\left(W^{rec} x_t + W^{in} u_{t+1} + b\right) \tag{1.10a}$$

**Readout:**
$$y_{t+1} = W^{out} x_{t+1} \tag{1.10b}$$

**Parameters:**
- $W^{rec} \in \mathbb{R}^{N \times N}$: fixed recurrent weights, initialized randomly
- $W^{in} \in \mathbb{R}^{N \times K}$: fixed input weights, initialized randomly
- $b \in \mathbb{R}^N$: fixed bias, often zero
- $\alpha \in (0, 1]$: leaking rate, a hyperparameter
- $W^{out} \in \mathbb{R}^{L \times N}$: **trained** readout weights

**Initialization:** $x_0 = 0$ (or random), followed by a washout period to eliminate dependence on $x_0$.

This five-equation system — one for the dynamics, one for the output, plus parameter specifications — is the complete specification of an ESN. Its simplicity is deceptive. The dynamics of $x_t$ under equation (1.10a) can be extraordinarily rich, depending on $W^{rec}$, $W^{in}$, and $\alpha$. Understanding this richness is the work of the remainder of this chapter.
