# 4.2.1 The Reservoir as a Random Feature Expansion in Time

## The Mystery of Random Weights

Why do random weights work?

This is the question that must be asked before anything else about reservoir computing can be properly understood. If you encountered the reservoir computing idea for the first time, without theoretical context, your reaction might be: this is a trick, not a principle. Surely the reservoir's performance must degrade badly when the random weights are unlucky. Surely a trained recurrent network must always outperform a random one. Surely this is just a computationally convenient approximation.

All of these intuitions are wrong — or at least, more limited than they first appear. The reservoir computing paradigm is not a trick. It rests on a theoretical foundation connecting high-dimensional random projections, kernel methods, and dynamical systems theory. This section develops that foundation.

## Random Feature Expansions: The Static Case

Before considering temporal inputs, let us recall the theory of random feature expansions for static inputs. This theory, due primarily to Rahimi and Recht [Rahimi2007, Rahimi2008], provides the conceptual model.

Consider a kernel function $k: \mathbb{R}^n \times \mathbb{R}^n \to \mathbb{R}$ — a positive definite similarity measure on the input space. The kernel trick [Schölkopf2002] allows us to work in a high-dimensional (possibly infinite-dimensional) feature space $\mathcal{H}$ implicitly, by computing inner products via $k(\mathbf{x}, \mathbf{z}) = \langle \phi(\mathbf{x}), \phi(\mathbf{z}) \rangle_{\mathcal{H}}$, where $\phi: \mathbb{R}^n \to \mathcal{H}$ is a feature map.

For shift-invariant kernels $k(\mathbf{x}, \mathbf{z}) = k(\mathbf{x} - \mathbf{z})$, Bochner's theorem guarantees that $k$ is the Fourier transform of a positive measure $p(\boldsymbol{\omega})$:

$$k(\mathbf{x} - \mathbf{z}) = \int_{\mathbb{R}^n} e^{i \boldsymbol{\omega}^T (\mathbf{x} - \mathbf{z})} \, dp(\boldsymbol{\omega})$$

The **random kitchen sink** [Rahimi2007] approximates this kernel by sampling $N$ random frequencies $\boldsymbol{\omega}_1, \ldots, \boldsymbol{\omega}_N \sim p(\boldsymbol{\omega})$ and computing the random feature map:

$$\phi(\mathbf{x}) = \frac{1}{\sqrt{N}} \left[\cos(\boldsymbol{\omega}_1^T \mathbf{x} + b_1), \ldots, \cos(\boldsymbol{\omega}_N^T \mathbf{x} + b_N)\right]^T$$

with $b_i \sim \text{Uniform}(0, 2\pi)$. By the law of large numbers:

$$\phi(\mathbf{x})^T \phi(\mathbf{z}) \xrightarrow{N \to \infty} k(\mathbf{x}, \mathbf{z})$$

uniformly over compact sets, with deviation $O(1/\sqrt{N})$ by McDiarmid's inequality.

**The key insight:** A random linear projection followed by a nonlinearity provides an explicit, finite-dimensional approximation to the kernel feature map. The approximation quality scales as $O(1/\sqrt{N})$: with $N$ random features, the error in approximating the kernel is bounded, and a linear classifier in the random feature space approximates the optimal kernel classifier in the original space.

This is the static version of the claim. The reservoir computing claim is the temporal generalization: random *recurrent* connections, applied to a sequence of inputs, provide a rich, high-dimensional feature expansion of the **input history** — not just the current input.

## From Static to Temporal: The Reservoir as a Volterra Series

To understand the temporal case, we need to understand what function class the reservoir approximates. The natural answer is the **Volterra series** — the temporal generalization of a polynomial in the input.

A **Volterra series** of order $p$ and memory $m$ represents a nonlinear temporal functional $H$ of an input sequence $\mathbf{u}_t, \mathbf{u}_{t-1}, \ldots$ as:

$$y_t = \sum_{d=0}^{p} \sum_{k_1, \ldots, k_d = 0}^{m} h_d(k_1, \ldots, k_d) \prod_{j=1}^{d} u_{t - k_j}$$

where $h_d$ is the $d$-th order Volterra kernel. This is a generalization of the convolution (which is the $d = 1$ case): it captures nonlinear interactions among past inputs at different lags.

A reservoir computer can approximate Volterra series of any finite order and memory:

**Theorem (informal):** A reservoir with $N$ units and $\tanh$ nonlinearity can, for appropriate random $W^{rec}$ and $W^{in}$ and appropriate linear readout, approximate any finite-order Volterra series to arbitrary accuracy as $N \to \infty$.

The argument has two parts:
1. The reservoir state $\mathbf{x}_t$ is a nonlinear function of the input history, containing products and cross-products of past inputs at different lags.
2. A linear readout $\mathbf{w}^T \mathbf{x}_t$ is a linear combination of these terms.
3. By choosing $\mathbf{w}$ appropriately, any Volterra series can be approximated.

This is not just intuition. Maass et al. [Maass2002] proved that liquid state machines satisfy a universal approximation property over causal, time-invariant functionals with fading memory — the class of functions that any sensible temporal processor would want to approximate.

## Why Random Weights Provide Diversity

The theoretical guarantee requires that the reservoir states span a sufficiently rich subspace of the Volterra series expansion. Why do random weights provide this?

The answer is related to the **diversity** of time scales and interaction patterns. Consider two neurons in the reservoir, $i$ and $j$, with different incoming connections from the input and from other neurons. Neuron $i$ integrates a particular linear combination of past inputs, passed through the nonlinearity. Neuron $j$ integrates a different combination.

If $W^{rec}$ and $W^{in}$ are random, then (with high probability) each neuron is computing a different nonlinear functional of the input history. The ensemble of $N$ neurons therefore provides $N$ different nonlinear projections — a diverse set of "basis functions" for the space of temporal functions.

More formally: the state $\mathbf{x}_t$ is a function of the input history $\mathbf{u}_t, \mathbf{u}_{t-1}, \ldots$. We can write it as:

$$x_t^{(i)} = g_i(\mathbf{u}_t, \mathbf{u}_{t-1}, \ldots, \mathbf{u}_{t-m_i})$$

where $g_i$ is the nonlinear temporal functional computed by neuron $i$ and $m_i$ is its effective memory. For the readout to approximate an arbitrary target function $\hat{y}_t = H(\mathbf{u}_t, \mathbf{u}_{t-1}, \ldots)$, we need the functions $\{g_i\}$ to form an approximating set for $H$.

Random $W^{rec}$ generates a diverse set $\{g_i\}$ in the following sense: the probability that two randomly-chosen neurons compute the same functional is zero. More importantly, as $N \to \infty$, the functions $\{g_i\}$ become dense in the space of bounded, causal, time-invariant functionals with fading memory, in the sense that any target functional can be approximated to arbitrary accuracy by a linear combination.

## The Role of Nonlinearity

A crucial point: the nonlinearity $f$ is necessary for this approximation property. Consider the case $f = $ identity (a linear reservoir). Then the state is:

$$\mathbf{x}_t = \sum_{s=0}^{t} (W^{rec})^s W^{in} \mathbf{u}_{t-s} + (W^{rec})^t \mathbf{x}_0$$

(ignoring the initial condition after washout). This is a linear convolution of the input with the impulse response $(W^{rec})^s W^{in}$. A linear readout on this state computes:

$$y_t = \mathbf{w}^T \mathbf{x}_t = \sum_{s=0}^{t} \mathbf{w}^T (W^{rec})^s W^{in} \mathbf{u}_{t-s}$$

which is a **linear filter** — a Volterra series of order 1. A linear reservoir can approximate any linear causal filter with fading memory, but it cannot approximate nonlinear temporal functions (e.g., $y_t = u_t \cdot u_{t-3}$).

The nonlinearity enables higher-order terms. With $f = \tanh$, the state $x_t^{(i)}$ contains terms like $u_{t-k} \cdot u_{t-l}$ (second-order), $u_{t-k} \cdot u_{t-l} \cdot u_{t-m}$ (third-order), and so on — up to all orders, due to the Taylor expansion of $\tanh$. This is what allows the readout to approximate the full Volterra series.

## Connection to the Random Kitchen Sink

The parallel with Rahimi and Recht's random kitchen sink is now precise:

| Static (Rahimi & Recht) | Temporal (Reservoir Computing) |
|---|---|
| Input: $\mathbf{x} \in \mathbb{R}^n$ | Input: sequence $\mathbf{u}_1, \mathbf{u}_2, \ldots, \mathbf{u}_t$ |
| Feature map: $\phi(\mathbf{x}) \in \mathbb{R}^N$ | Feature map: $\mathbf{x}_t \in \mathbb{R}^N$ |
| Random projections $\boldsymbol{\omega}_i$ | Random recurrent weights $W^{rec}$, $W^{in}$ |
| Kernel: $k(\mathbf{x}, \mathbf{z}) = e^{-\|\mathbf{x} - \mathbf{z}\|^2}$ | Temporal kernel: $k(\mathbf{u}, \mathbf{v}) = $ inner product in Volterra space |
| Output: $y = \mathbf{w}^T \phi(\mathbf{x})$ | Output: $y_t = \mathbf{w}^T \mathbf{x}_t$ |
| Training: linear regression | Training: linear regression |

In both cases: random nonlinear projections provide an explicit, finite-dimensional approximation to a kernel classifier; a linear model in the projected space approximates the optimal kernel model; the approximation quality scales as $O(1/\sqrt{N})$.

The temporal version is richer because the "input" is a sequence and the "features" encode not just the current value but the entire history. This is what the reservoir's recurrent structure provides: a principled way to generate diverse, nonlinear projections of the input history.

## A Mathematical Guarantee

We can state the approximation property more precisely. Let $\mathcal{F}$ be the class of causal, time-invariant functionals with fading memory (Definition: $H$ has fading memory if there exists a function $\delta(\cdot)$ with $\delta(k) \to 0$ as $k \to \infty$ such that, for any two input sequences $\mathbf{u}$ and $\mathbf{v}$ with $\|\mathbf{u}_{t-k} - \mathbf{v}_{t-k}\| \leq \epsilon$ for $k \leq m$ and $\|\mathbf{u}_{t-k} - \mathbf{v}_{t-k}\| \leq 1$ for $k > m$, the output satisfies $|H[\mathbf{u}]_t - H[\mathbf{v}]_t| \leq \delta(m) + \epsilon$.)

**Theorem [Maass2002, Boyd1985]:** For any $H \in \mathcal{F}$ and any $\epsilon > 0$, there exists a reservoir with $N$ units (for $N$ sufficiently large), random weights drawn from a suitable distribution, and a linear readout $\mathbf{w} \in \mathbb{R}^N$ such that:

$$\sup_{t, \mathbf{u}} \left| \mathbf{w}^T \mathbf{x}_t - H[\mathbf{u}]_t \right| \leq \epsilon$$

with high probability over the random weight draw.

This is the **universal approximation theorem for reservoir computers**. It says that any function we would want to compute over temporal inputs — any continuous, causal, time-invariant functional with fading memory — can be approximated to any desired accuracy by a reservoir computer with enough units and a random weight draw.

This is not just encouraging; it is *why random weights work*. The random weights generate a diverse enough set of nonlinear temporal projections that, collectively, they can span the function space we care about. The linear readout selects the right linear combination from this spanning set.

## Practical Implications

The theoretical guarantee has several practical implications:

**More units is better (up to a point).** The approximation error scales as $O(1/\sqrt{N})$. Doubling the reservoir size reduces the error by a factor of $\sqrt{2} \approx 1.41$. But the linear regression problem grows in dimension with $N$, requiring more training data to avoid overfitting: roughly $T_{train} > N$ is needed for well-conditioned regression.

**The spectral radius matters.** The approximation guarantee requires that the reservoir have the fading memory property — that past inputs' influence decays over time. This is controlled by the spectral radius $\rho(W^{rec})$. For $\rho < 1$, fading memory is guaranteed (Chapter 5). The choice of $\rho$ trades off memory depth (how far back the reservoir "looks") against the stability of the fading.

**Random is generic, not specific.** The approximation is guaranteed for "generic" random weight draws — a set of measure 1. A specific unlucky draw could fail. In practice, this means it is worth trying a few random seeds if performance is poor; but typical draws work well.

**The readout must be trained.** The approximation property says that the right readout exists; it does not specify what it is. The readout must be trained from data to find the right linear combination. But the training problem — linear regression — is convex and well-understood.

---

## References

- [Rahimi2007] Rahimi, A., & Recht, B. (2007). Random features for large-scale kernel machines. *NeurIPS*, 20.
- [Rahimi2008] Rahimi, A., & Recht, B. (2008). Weighted sums of random kitchen sinks: Replacing minimization with randomization in learning. *NeurIPS*, 21.
- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states. *Neural Computation*, 14(11), 2531–2560.
- [Boyd1985] Boyd, S., & Chua, L. O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Trans. Circuits Syst.*, 32(11), 1150–1161.
- [Schölkopf2002] Schölkopf, B., & Smola, A. J. (2002). *Learning with Kernels*. MIT Press.
