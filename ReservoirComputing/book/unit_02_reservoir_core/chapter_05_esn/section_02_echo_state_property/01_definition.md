# Section 2.1: The Echo State Property — Definition and Meaning

## 2.1 Motivation: The Problem of Initial Conditions

Consider the ESN update $F : \mathbb{R}^N \to \mathbb{R}^N$ defined by:

$$F(x; u) = (1-\alpha)x + \alpha \tanh(W^{rec} x + W^{in} u + b) \tag{2.1}$$

Given a bi-infinite input sequence $\mathbf{u} = (\ldots, u_{-2}, u_{-1}, u_0, u_1, u_2, \ldots)$, we can drive the reservoir from two different initial states $x_0$ and $x_0'$ and observe two different trajectories:

$$x_t = F(x_{t-1}; u_t), \quad x_t' = F(x_{t-1}'; u_t)$$

For the ESN to be a well-defined computational device — one whose output depends only on the input history and not on an arbitrary initial condition — we need these trajectories to converge:

$$\|x_t - x_t'\| \to 0 \quad \text{as } t \to \infty \tag{2.2}$$

This is the essential requirement. When it holds, the reservoir state at time $t$ is determined entirely by the input sequence up to time $t$, regardless of where we started. The reservoir is then a nonlinear functional of the input history.

---

## 2.2 Formal Definition

We follow the formalization of [Jaeger2001] and the rigorous treatment in [Yildiz2012].

Let $\mathcal{U} \subseteq \mathbb{R}^K$ be the (compact) input space and let $\mathcal{U}^\mathbb{Z}$ denote the space of bi-infinite input sequences. For a reservoir map $F: \mathbb{R}^N \times \mathcal{U} \to \mathbb{R}^N$, define the **driven state trajectory** starting from initial state $x$ under input sequence $\mathbf{u}$:

$$X_t(\mathbf{u}, x) = F(X_{t-1}(\mathbf{u}, x); u_t), \quad X_0(\mathbf{u}, x) = x$$

**Definition 2.1 (Echo State Property).** The reservoir map $F$ has the **echo state property** with respect to a compact input space $\mathcal{U}$ and a compact state space $\mathcal{X} \subseteq \mathbb{R}^N$ if for every input sequence $\mathbf{u} \in \mathcal{U}^\mathbb{Z}$ and every pair of initial states $x, x' \in \mathcal{X}$:

$$\lim_{t \to \infty} \|X_t(\mathbf{u}, x) - X_t(\mathbf{u}, x')\| = 0 \tag{2.3}$$

In words: the reservoir **forgets its initial conditions**. No matter where you start it, the reservoir driven by the same input sequence eventually settles onto the same trajectory.

---

## 2.3 The Echo Function

When the ESP holds, something remarkable follows. For any input sequence $\mathbf{u} \in \mathcal{U}^\mathbb{Z}$, the limit:

$$\bar{x}_t(\mathbf{u}) = \lim_{s \to \infty} X_t(\mathbf{u}_{(-\infty, t]}, x) \tag{2.4}$$

exists and is independent of $x$. Here $\mathbf{u}_{(-\infty, t]}$ denotes the semi-infinite sequence $(\ldots, u_{t-2}, u_{t-1}, u_t)$.

This limit defines the **echo state** or **echo function** $E_t : \mathcal{U}^{\mathbb{Z}_{-}} \to \mathcal{X}$:

$$E_t(\mathbf{u}) = \bar{x}_t(\mathbf{u}) \tag{2.5}$$

The echo state at time $t$ is a **nonlinear functional of the semi-infinite input history** $(\ldots, u_{t-2}, u_{t-1}, u_t)$. When we train a linear readout $y_t = W^{out} x_t$, we are training a linear functional of this nonlinear feature map. This is the core computational architecture of the ESN.

**Remark.** The echo function is well-defined and unique once the ESP holds. Different input histories give different echo states (the separation property, to be discussed in Section 2 of the LSM chapter). The echo function is causal — it depends only on past and present inputs, not future inputs.

---

## 2.4 State Forgetting: What it Means Mathematically

The condition (2.3) is a statement about **exponential forgetting of initial conditions**. Let us unpack what "forgetting" means precisely.

Define the **state difference** at time $t$:

$$d_t = X_t(\mathbf{u}, x) - X_t(\mathbf{u}, x')$$

The ESP requires $d_t \to 0$ as $t \to \infty$, for all $\mathbf{u}$, $x$, $x'$.

Stronger versions of the ESP require **exponential** forgetting:

$$\|d_t\| \leq C \cdot r^t \cdot \|x - x'\| \tag{2.6}$$

for some constants $C > 0$ and $0 < r < 1$, independent of the input sequence $\mathbf{u}$. This is called **uniform** exponential state forgetting, and it is equivalent to the reservoir being a **uniform contraction** over all inputs (see Section 2.2 on sufficient conditions).

The rate $r$ determines how quickly the initial conditions are forgotten. After $t \approx -\ln(\epsilon)/(-\ln r)$ steps, the contribution of the initial state is below $\epsilon \|x - x'\|$. This is the **effective washout length**: the number of steps we need to discard at the beginning of each simulation to eliminate initialization effects.

---

## 2.5 The Contractivity Connection

The key mathematical tool for proving the ESP is the **contraction mapping theorem** (also known as the Banach fixed-point theorem).

**Definition 2.2 (Contraction).** A map $F : \mathcal{X} \to \mathcal{X}$ is a **$\gamma$-contraction** if there exists $0 \leq \gamma < 1$ such that for all $x, x' \in \mathcal{X}$:

$$\|F(x) - F(x')\| \leq \gamma \|x - x'\| \tag{2.7}$$

**Theorem 2.1 (Banach).** If $F : \mathcal{X} \to \mathcal{X}$ is a $\gamma$-contraction on a complete metric space $(\mathcal{X}, \|\cdot\|)$, then:
1. $F$ has a unique fixed point $x^* \in \mathcal{X}$.
2. For any $x_0 \in \mathcal{X}$, the iterates $x_n = F^n(x_0)$ converge to $x^*$ exponentially: $\|x_n - x^*\| \leq \gamma^n \|x_0 - x^*\|$.

**Connection to ESP.** If the driven map $F(\cdot; u_t)$ is a $\gamma$-contraction for every input $u_t \in \mathcal{U}$, uniformly in $u_t$, then:

$$\|X_t(\mathbf{u}, x) - X_t(\mathbf{u}, x')\| \leq \gamma^t \|x - x'\| \tag{2.8}$$

This follows by a telescoping argument: each step applies a contraction with ratio $\gamma$, so after $t$ steps the distance has shrunk by $\gamma^t$. Since $\gamma < 1$, this goes to zero exponentially fast, proving the ESP.

---

## 2.6 Proof Sketch: Contraction Implies ESP

Let us work through the argument carefully for the leaky integrator ESN.

**Given:** The map $F(x; u) = (1-\alpha)x + \alpha \tanh(W^{rec} x + W^{in} u + b)$.

**Claim:** If $\gamma \equiv (1-\alpha) + \alpha \|W^{rec}\|_2 \cdot 1 < 1$, then $F$ is a $\gamma$-contraction.

Here $\|W^{rec}\|_2 = \sigma_{max}(W^{rec})$ is the spectral norm (largest singular value).

**Proof.** Let $x, x' \in \mathbb{R}^N$. Then:

$$\|F(x; u) - F(x'; u)\| = \|(1-\alpha)(x - x') + \alpha(\tanh(W^{rec} x + c) - \tanh(W^{rec} x' + c))\|$$

where $c = W^{in} u + b$ is common to both terms and cancels. By the triangle inequality:

$$\leq (1-\alpha)\|x - x'\| + \alpha\|\tanh(W^{rec} x + c) - \tanh(W^{rec} x' + c)\|$$

Now use the global Lipschitz property of $\tanh$: since $|\tanh'(z)| \leq 1$ for all $z \in \mathbb{R}$, the mean value theorem gives:

$$\|\tanh(a) - \tanh(b)\| \leq \|a - b\|$$

(elementwise Lipschitz with constant 1 implies the same for the vector $\ell^2$ norm). Therefore:

$$\|\tanh(W^{rec} x + c) - \tanh(W^{rec} x' + c)\| \leq \|W^{rec}(x - x')\| \leq \|W^{rec}\|_2 \cdot \|x - x'\|$$

Substituting:

$$\|F(x;u) - F(x';u)\| \leq \left[(1-\alpha) + \alpha \|W^{rec}\|_2\right] \|x - x'\| \tag{2.9}$$

If the bracketed quantity $\gamma = (1-\alpha) + \alpha\|W^{rec}\|_2 < 1$, then $F(\cdot; u)$ is a $\gamma$-contraction for every $u$. Since $\gamma$ does not depend on $u$, this is a **uniform contraction** over all inputs.

By induction over $t$ steps:

$$\|X_t(\mathbf{u}, x) - X_t(\mathbf{u}, x')\| \leq \gamma^t \|x - x'\|$$

Since $\gamma < 1$, the right side goes to zero exponentially, proving the ESP. $\square$

**When does $\gamma < 1$?** We have $\gamma < 1$ iff $(1-\alpha) + \alpha \|W^{rec}\|_2 < 1$, which simplifies to:

$$\|W^{rec}\|_2 < 1 \tag{2.10}$$

This is the spectral norm condition for the ESP. Note that it is *sufficient but not necessary* — see Section 2.2 and Section 3 for sharper conditions.

---

## 2.7 The Fading Memory Interpretation

The echo state property is intimately connected to the concept of **fading memory**, formalized by Boyd and Chua [BoydChua1985] in the context of analog signal processing.

**Definition 2.3 (Fading Memory Property).** A causal time-invariant operator $H : \mathcal{U}^\mathbb{Z} \to \mathcal{Y}^\mathbb{Z}$ has the **fading memory property** if for any $\epsilon > 0$, there exists a weighting sequence $w_k > 0$ with $w_k \to \infty$ such that for any two input sequences $\mathbf{u}, \mathbf{u}'$ satisfying:

$$\sup_{k \geq 0} w_k |u_{t-k} - u'_{t-k}| < \delta$$

we have $|H(\mathbf{u})_t - H(\mathbf{u}')_t| < \epsilon$.

In plain language: the operator's output depends on its input history, but inputs further in the past have less influence — the influence **fades** exponentially with the lag.

**ESN and Fading Memory.** When the ESP holds, the echo function $E_t(\mathbf{u}) = \bar{x}_t$ is a fading memory functional. This can be shown as follows: suppose $\mathbf{u}$ and $\mathbf{u}'$ agree from time $t - T$ onward (they only differ for $t < -T$). Then:

$$\|E_t(\mathbf{u}) - E_t(\mathbf{u}')\| \leq \gamma^T \cdot \text{diam}(\mathcal{X})$$

The difference in the echo states is bounded by $\gamma^T$ times the diameter of the state space, which goes to zero exponentially in $T$. Thus events more than $T$ time steps in the past contribute less than $\gamma^T \cdot \text{diam}(\mathcal{X})$ to the current state. This is precisely the fading memory property with exponential weights $w_k = \gamma^{-k}$.

**Significance.** The fading memory property is what allows the ESN to generalize. It means the reservoir does not hold onto the distant past indefinitely — if it did, the network would need infinite precision to distinguish arbitrarily old events, and training would be ill-conditioned. Instead, the reservoir provides a graded, decaying representation of history, which the readout can use to solve temporal tasks.

---

## 2.8 The ESP as a Uniformity Condition

A subtle but important point: the ESP as defined in (2.3) requires convergence for **all** input sequences $\mathbf{u} \in \mathcal{U}^\mathbb{Z}$. This is a uniformity condition. There exist reservoirs where the ESP holds for some inputs but not others — for example, a reservoir with $\rho(W^{rec}) > 1$ might satisfy the ESP for small-amplitude inputs (where the reservoir operates in the linear regime near zero) but violate it for large-amplitude inputs.

The standard definition (2.3) over a compact $\mathcal{U}$ handles this correctly: the ESP must hold for all bounded inputs simultaneously. This is why the compactness of $\mathcal{U}$ appears in the definition — it ensures a uniform bound on input magnitude, which is necessary for the contraction argument to apply uniformly.

In practice, input signals are always bounded, and the ESP is satisfied for all practical purposes when the standard design rules are followed. The theoretical subtleties matter most when one is trying to push the reservoir close to instability to maximize its computational capacity.
