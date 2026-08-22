# Section 29.4: Skew-Product Systems and the Reservoir as a Cocycle

## 29.4.1 The Skew-Product Structure

The non-autonomous dynamical system framework of Section 29.3 has a natural reformulation as an *autonomous* system on an extended state space. This is the *skew-product* construction.

**Definition 29.4.1 (Skew-Product System).** Given a cocycle $\varphi: \mathbb{Z}_+ \times \Omega \times \mathcal{X} \to \mathcal{X}$ over a measure-preserving system $(\Omega, \mathcal{F}, \mu, T)$, the *skew-product* is the autonomous dynamical system on $\Omega \times \mathcal{X}$ with map:
$$\Theta: \Omega \times \mathcal{X} \to \Omega \times \mathcal{X}, \quad \Theta(\omega, x) = (T\omega, F(x, u(\omega))).$$

The skew-product $\Theta$ is autonomous (no explicit time dependence) and captures both the input dynamics (via $T$) and the reservoir dynamics (via $F$). The measure $\mu \otimes \nu$ (for some measure $\nu$ on $\mathcal{X}$) is $\Theta$-invariant if and only if $\nu$ is the marginal of the stationary measure of the coupled system.

**Example 29.4.1 (Reservoir as Skew-Product).** Consider an ESN:
$$x(t+1) = \tanh(Wx(t) + W_{\text{in}}u(t)).$$
The input $u(t) = u_0(T^t\omega)$ is driven by the shift $T$ on the input space $\Omega$. The skew-product map is:
$$\Theta(\omega, x) = (T\omega,\ \tanh(Wx + W_{\text{in}} u_0(\omega))).$$
The echo state property says there exists a measurable function $x^*: \Omega \to \mathcal{X}$ such that the graph $\{(\omega, x^*(\omega)) : \omega \in \Omega\}$ is an invariant set of $\Theta$.

## 29.4.2 Invariant Measures and Stationary Distributions

**Definition 29.4.2 (Stationary Measure for Reservoir).** A probability measure $P$ on $\Omega \times \mathcal{X}$ is *stationary* for the skew-product $\Theta$ if $\Theta_* P = P$. The marginal of $P$ on $\Omega$ must be $\mu$ (the input measure). The conditional measure $P(\cdot | \omega)$ on $\mathcal{X}$ given $\omega$ is then the *reservoir state distribution* when the input sequence is $\omega$.

**Theorem 29.4.1 (Existence of Stationary Measure).** *Suppose $\mathcal{X}$ is compact and $F: \mathcal{X} \times U \to \mathcal{X}$ is continuous. Then there exists at least one stationary measure $P$ for $\Theta$ with marginal $\mu$ on $\Omega$.*

**Proof.** Start with any measure $P_0 = \mu \otimes \delta_{x_0}$ (all reservoir states initialized at $x_0$). The Cesàro averages $P_n = \frac{1}{n}\sum_{k=0}^{n-1} \Theta^k_* P_0$ are probability measures on $\Omega \times \mathcal{X}$. By compactness of $\mathcal{P}(\Omega \times \mathcal{X})$ (the set of probability measures with the weak topology, using Prokhorov's theorem since $\mathcal{X}$ is compact), there is a subsequential limit $P_\infty$. By construction, $\Theta_* P_\infty = P_\infty$. $\blacksquare$

**Theorem 29.4.2 (Uniqueness under ESP).** *If the reservoir has the echo state property (Theorem 29.3.1), the stationary measure $P$ is unique and is supported on the graph of $x^*$:*
$$P = \int_\Omega \delta_{x^*(\omega)}\, d\mu(\omega).$$

**Proof.** If two measures $P$ and $Q$ are stationary, their supports must both be invariant. The ESP implies all orbits converge to the graph of $x^*$, so both $P$ and $Q$ are supported on $\{(\omega, x^*(\omega))\}$, giving $P = Q = \int \delta_{x^*(\omega)}\, d\mu(\omega)$. $\blacksquare$

The stationary measure with ESP has a beautifully simple structure: the reservoir state is a deterministic function of the current input sample $\omega$. The entire probability distribution of the state is determined by the input distribution.

## 29.4.3 Measurable Selection and the Echo Response

In the proof of Theorem 29.3.1, we used the existence of a measurable function $x^*: \Omega \to \mathcal{X}$. This is not automatic — for set-valued pullback attractors, a measurable selection theorem is needed.

**Theorem 29.4.3 (Measurable Selection Theorem, Castaing-Varadarajan).** *Let $\Omega$ be a measurable space and $\mathcal{X}$ a complete separable metric space. Let $A: \Omega \to 2^{\mathcal{X}}$ be a set-valued map such that:*
1. $A(\omega)$ is non-empty and closed for $\mu$-a.e. $\omega$.
2. The map $\omega \mapsto A(\omega)$ is measurable (the set $\{(\omega, x) : x \in A(\omega)\}$ is measurable in $\Omega \times \mathcal{X}$).

*Then there exists a measurable selection: a measurable function $s: \Omega \to \mathcal{X}$ with $s(\omega) \in A(\omega)$ for $\mu$-a.e. $\omega$.*

This theorem guarantees the existence of a measurable echo state response even when the pullback attractor $A(\omega)$ is not a singleton — which can happen when the ESP fails. In that case, $x^*$ is any measurable selection from the pullback attractor, and the output $y(t) = W_{\text{out}} x^*(T^t\omega)$ is a well-defined stationary process.

**Remark 29.4.1 (Non-unique Echo State Response).** When the pullback attractor $A(\omega)$ is not a singleton (ESP fails), there are multiple stationary solutions. The system's long-term behavior depends on initial conditions: orbits starting from different initial states converge to different elements of $A(\omega)$. In this case, the reservoir's computational properties depend on the initial state — a problematic situation for a computing device that should produce deterministic outputs from the same inputs.

## 29.4.4 Lyapunov Exponents and the ESP

The ESP can also be characterized in terms of *Lyapunov exponents*, which measure the average rate of exponential divergence or convergence of nearby orbits.

**Definition 29.4.3 (Lyapunov Exponent).** For the cocycle $\varphi$ with derivative $D_x\varphi(t, \omega, x)$ (Jacobian with respect to $x$), the *maximal Lyapunov exponent* is:
$$\lambda_{\max} = \lim_{t \to \infty} \frac{1}{t} \log \|D_x \varphi(t, \omega, x^*(\omega))\|_{\text{op}} \quad \mu\text{-a.s.}$$
(by Oseledets' multiplicative ergodic theorem [Oseledets1968], this limit exists a.s. and is deterministic for ergodic $\mu$).

**Theorem 29.4.4 (ESP and Lyapunov Exponent).** *A reservoir with a unique stationary solution $x^*$ has the ESP if and only if $\lambda_{\max} < 0$: all Lyapunov exponents are negative.*

**Proof sketch.** The Lyapunov exponent $\lambda_{\max} < 0$ means that the linearization of the reservoir map around $x^*$ is contracting on average. By the multiplicative ergodic theorem, this implies that nearby orbits converge to $x^*$ exponentially, at rate $e^{\lambda_{\max} t}$. This is precisely the ESP. $\blacksquare$

For a linear reservoir $x(t+1) = Wx(t) + W_{\text{in}}u(t)$, the Lyapunov exponents are $\log|\lambda_i(W)|$ for each eigenvalue $\lambda_i$. The condition $\lambda_{\max} < 0$ reduces to $\rho(W) < 1$ — recovering the standard ESN stability condition.

For a nonlinear reservoir with $\tanh$ activation, the Lyapunov exponents depend on the trajectory $x^*$ and the input $\omega$. The condition $\lambda_{\max} < 0$ is more subtle and can hold even when the spectral radius of $W$ exceeds 1, due to the saturation of the tanh function.

## 29.4.5 The Edge of Chaos

The Lyapunov exponent perspective gives a rigorous foundation for the "edge of chaos" hypothesis in reservoir computing [Bertschinger2004]:

**The edge of chaos is the regime where $\lambda_{\max} \approx 0$.**

- **$\lambda_{\max} < 0$ (ordered phase)**: The reservoir has the ESP. Small perturbations are forgotten. The reservoir has limited memory and expressiveness.
- **$\lambda_{\max} > 0$ (chaotic phase)**: The reservoir does not have the ESP. Small differences in initial conditions grow exponentially. The reservoir is sensitive but unreliable.
- **$\lambda_{\max} \approx 0$ (critical phase)**: The reservoir balances memory and expressiveness. Information persists for long times without diverging.

The edge of chaos regime maximizes the *information transmission* through the reservoir, as measured by the mutual information between the input and the reservoir state. This has been established empirically and has theoretical support from the theory of cellular automata [Langton1990] and random Boolean networks.

**Caveat.** The edge of chaos hypothesis has been influential but also contested. The empirical evidence is mixed: some studies find peak performance near $\lambda_{\max} = 0$, others find optimal performance away from the edge. The difficulty is that $\lambda_{\max}$ depends on both the reservoir parameters and the input, so the "edge" is not a fixed point in parameter space. We discuss this further in Chapter 34 (Open Problems).
