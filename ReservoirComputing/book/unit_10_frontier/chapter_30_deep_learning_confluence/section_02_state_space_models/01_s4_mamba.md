# Section 30.2: Structured State Space Models — S4, Mamba, and the Reservoir Connection

## 30.2.1 The State Space Model Framework

A continuous-time state space model (SSM) is defined by the equations:
$$\frac{dx}{dt} = Ax(t) + Bu(t), \quad y(t) = Cx(t) + Du(t),$$
where $x(t) \in \mathbb{R}^N$ is the state, $u(t) \in \mathbb{R}$ is the input, $y(t) \in \mathbb{R}$ is the output, and $A \in \mathbb{R}^{N \times N}$, $B \in \mathbb{R}^{N \times 1}$, $C \in \mathbb{R}^{1 \times N}$, $D \in \mathbb{R}$ are parameters.

In its continuous form, this is a linear reservoir: $A$ plays the role of the (scaled) reservoir weight matrix $W$, $B$ plays the role of the input weight $W_{\text{in}}$, $C$ plays the role of the readout weight $W_{\text{out}}$, and $D$ is a skip connection. The key differences from classical reservoir computing are:
1. **All parameters are learned**: In a reservoir computer, $A$, $B$ are fixed and only $C$ (and $D$) are trained. In S4, all parameters are learned.
2. **The model is linear**: There is no nonlinear activation in the state equation (nonlinearity is introduced between SSM layers, as in a deep network).
3. **The model is discretized for computation**: The continuous dynamics are discretized using the zero-order hold (ZOH) or bilinear (Tustin) method, yielding a discrete-time recurrence.

## 30.2.2 Discretization and the Discrete SSM

The continuous SSM is discretized with step size $\Delta$ using the zero-order hold method:
$$\bar{A} = e^{\Delta A}, \quad \bar{B} = (\Delta A)^{-1}(e^{\Delta A} - I)\Delta B = (e^{\Delta A} - I)A^{-1}B,$$
giving the discrete recurrence:
$$x_k = \bar{A} x_{k-1} + \bar{B} u_k, \quad y_k = C x_k + D u_k.$$

This is exactly a linear reservoir with weight matrix $\bar{A}$ and input weights $\bar{B}$. The discrete SSM maps an input sequence $(u_1, u_2, \ldots, u_L)$ to an output sequence $(y_1, y_2, \ldots, y_L)$ via a linear recurrence.

**Convolution view.** The output can also be written as a convolution:
$$y_k = \sum_{j=0}^{k} \bar{K}_j u_{k-j} + D u_k,$$
where $\bar{K}_j = C \bar{A}^j \bar{B}$ is the *impulse response* of the system. For an $L$-length input, the convolution can be computed in $O(L \log L)$ time using FFT, making the SSM much faster than a naive recurrence ($O(L^2)$ for attention, $O(L)$ for recurrence but with sequential computation).

## 30.2.3 The HiPPO Framework: Principled Reservoir Initialization

The critical insight of the S4 line of work is not the state space model formulation itself (that is classical) but the *initialization* of $A$. The HiPPO framework (High-Order Polynomial Projection Operators, [GuHasani2020]) provides a principled initialization for $A$ based on the goal of optimal online polynomial approximation of the input history.

**Definition 30.2.1 (HiPPO Operators).** For an input signal $u: [0, t] \to \mathbb{R}$ and a measure $\mu_t$ on $[0, t]$, the HiPPO state at time $t$ is the vector of coefficients $c(t) \in \mathbb{R}^N$ of the best polynomial approximation to the history $u_{[0,t]}$ in the $L^2(\mu_t)$ sense:
$$c(t) = \arg\min_{c \in \mathbb{R}^N} \int_0^t \left(u(s) - \sum_{n=0}^{N-1} c_n(t) p_n^{\mu_t}(s)\right)^2 d\mu_t(s),$$
where $\{p_n^{\mu_t}\}$ are the orthogonal polynomials with respect to $\mu_t$.

The remarkable fact is that for natural choices of $\mu_t$, the evolution of $c(t)$ satisfies a *linear ODE* $\dot{c} = Ac + Bu$ with specific matrices $A$ and $B$ that can be written in closed form.

**HiPPO-LegS (Legendre Scale Measure).** With $\mu_t = \frac{1}{t}\mathbf{1}_{[0,t]}$ (uniform measure on $[0,t]$), the HiPPO ODE has:
$$A_{nk} = \begin{cases} -(2n+1)^{1/2}(2k+1)^{1/2} & \text{if } n > k \\ -n & \text{if } n = k \\ 0 & \text{if } n < k \end{cases}, \quad B_n = (2n+1)^{1/2}.$$

The state $c(t)$ encodes the history of $u$ up to time $t$ via Legendre polynomial coefficients — a compression of the infinite-dimensional history into a finite-dimensional vector. The state at time $t$ is the best degree-$(N-1)$ polynomial approximation to the history $u_{[0,t]}$.

**Connection to reservoir computing.** The HiPPO-LegS matrix $A$ is a *structured reservoir* that is designed, not random. It encodes the ideal linear fading memory: all polynomial features of the input history up to degree $N-1$ are perfectly preserved. A random reservoir approximates this ideal (via random projections), but the HiPPO initialization achieves it exactly and in a way that is differentiable and learnable.

## 30.2.4 S4: Structured State Space Sequence Model

S4 [GuGoel2022] combines the HiPPO initialization with efficient computation via the diagonal-plus-low-rank (DPLR) structure of $A$.

**S4 architecture.** The S4 layer has parameters $(A, B, C, \Delta)$ where:
- $A$ is initialized as HiPPO-LegS and parameterized as $A = \Lambda - PQ^*$ (diagonal minus low-rank, $\Lambda$ diagonal and complex-valued).
- $B, C \in \mathbb{R}^{N}$ and $\Delta \in \mathbb{R}$ (a global step size) are learned.
- The DPLR structure enables the convolution kernel $\bar{K}$ to be computed via a Cauchy kernel evaluation, which can be done in $O(N + L)$ time.

**Why the DPLR structure?** A full $N \times N$ complex matrix $A$ has $O(N^2)$ parameters, making learning computationally expensive. The DPLR structure $A = \Lambda - PQ^*$ has only $O(N)$ parameters. More importantly, the Cauchy matrix kernel $\mathcal{K}_{nm} = \frac{1}{\lambda_n + \bar{\lambda}_m}$ (appearing in the computation of $\bar{K}$) can be evaluated efficiently using the DPLR structure, enabling $O(N \log^2 N)$ computation of the full kernel.

**Training.** In S4, all of $A, B, C, \Delta$ are learned by gradient descent, with $A$ initialized to the HiPPO-LegS value. The combination of a principled initialization (encoding the ideal fading memory structure) with a differentiable parameterization (enabling end-to-end gradient training) is the key contribution.

## 30.2.5 S4 as a Trained Reservoir

The connection to reservoir computing is precise:

1. **S4 with frozen $A, B$** (only $C, D$ learned) = a linear reservoir computer with HiPPO initialization, trained by linear readout optimization.

2. **S4 with all parameters learned** = a trainable reservoir, where the reservoir dynamics are improved by gradient descent.

3. **The HiPPO initialization** provides a "warm start" for the dynamics: the reservoir already implements good fading memory before any gradient steps.

**Theorem 30.2.1 (S4 as Reservoir, informal).** *A single S4 layer with frozen $A, B$ and trained $C, D$ is a linear reservoir computer with state dimension $N$ and HiPPO-LegS reservoir matrix. Its computational power is characterized by the Boyd-Chua theorem (Chapter 26): it can approximate any linear fading-memory functional.*

For nonlinear functionals, S4 layers are stacked with pointwise nonlinearities (typically GELU) between them, analogously to how deep reservoirs can approximate nonlinear functionals.

## 30.2.6 Mamba and Selective State Spaces

Mamba [GuDao2023] extends S4 by making the matrices $B$, $C$, and $\Delta$ *input-dependent*:
$$x_k = \bar{A}(\Delta) x_{k-1} + \bar{B}(u_k) u_k, \quad y_k = C(u_k)^\top x_k.$$

Here $B$, $C$, and $\Delta$ are computed from $u_k$ by learned linear projections. This breaks the time-invariance of the SSM — the dynamics now depend on the current input.

**Reservoir interpretation.** In reservoir computing terms, Mamba is a *liquid reservoir* [HasaniLechner2021]: the effective reservoir weight matrix $\bar{A}(\Delta)$ (and input/output connections) change depending on the current input. The input can "select" which parts of the history to retain (via $\Delta$, which controls the effective memory time constant) and which outputs to read (via $C(u_k)$).

**Why input-dependence matters.** The key limitation of S4 (and of linear reservoirs generally) is that the same features are computed regardless of the input content. Mamba's selective state spaces allow the model to *focus* on relevant parts of the input history. For language modeling, this means the model can retain information about relevant context (e.g., the subject of a long sentence) while discarding irrelevant context (e.g., filler words).

**Formal connection to ESNs.** Mamba's selective SSM can be written as:
$$x_k = \text{diag}(e^{\Delta_k \cdot a}) x_{k-1} + b_k u_k,$$
where $\Delta_k, b_k$ depend on $u_k$ via linear projections. This is a nonlinear (input-dependent) reservoir with a diagonal state matrix — a highly structured but very fast architecture.

## 30.2.7 Implications for Reservoir Design

The S4/Mamba line of work has several important lessons for reservoir computing:

1. **Initialization matters.** The HiPPO initialization dramatically outperforms random initialization for long-range dependency tasks. This suggests that the "random is fine" philosophy of classical reservoir computing may not extend to deep sequential models.

2. **Structured > Random for learning.** When reservoir weights are to be trained, structure (DPLR, diagonal) is essential for computational efficiency and may also improve optimization landscapes.

3. **The distinction between what is fixed and what is learned matters.** Keeping $A$ fixed and learning only $C$ (classical RC) vs. learning all parameters (S4) vs. making parameters input-dependent (Mamba) represents a hierarchy of expressiveness at a cost of increasing computational complexity and training difficulty.

4. **Long-range dependencies require careful initialization.** For inputs with long-range dependencies, the spectral radius of $A$ must be close to 1 (slow forgetting). Naive random initialization typically places the spectral radius well below 1, causing catastrophic forgetting of long-range context. HiPPO avoids this.
