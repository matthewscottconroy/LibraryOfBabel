# The Echo State Property as an Ergodic Property

## 29.2.1 Ergodic Theory: A Brief Overview

**Ergodic theory** studies the long-time behavior of measure-preserving dynamical systems. Its central result — Birkhoff's ergodic theorem [Birkhoff 1931] — establishes conditions under which time averages equal space averages. This section develops the connection between ergodic theory and the echo state property (ESP) of reservoir computing, showing that the ESP is equivalent to a specific ergodic property of the driven reservoir system.

**Definition 29.1 (Measure-Preserving Dynamical System).** A **measure-preserving dynamical system (MPDS)** is a tuple $(\Omega, \mathcal{B}, \mu, T)$ where $(\Omega, \mathcal{B}, \mu)$ is a probability space and $T: \Omega \to \Omega$ is a measurable, measure-preserving map ($\mu(T^{-1}A) = \mu(A)$ for all $A \in \mathcal{B}$).

**Theorem 29.1 (Birkhoff's Ergodic Theorem [Birkhoff 1931]).** Let $(\Omega, \mathcal{B}, \mu, T)$ be a MPDS and $f \in L^1(\mu)$. Then

$$
\lim_{n \to \infty} \frac{1}{n}\sum_{k=0}^{n-1} f(T^k \omega) = \mathbb{E}_\mu[f \mid \mathcal{I}](w) \quad \mu\text{-a.s.},
$$

where $\mathcal{I}$ is the $\sigma$-algebra of $T$-invariant sets. If $T$ is **ergodic** (the only invariant sets have measure 0 or 1), then $\mathbb{E}_\mu[f \mid \mathcal{I}] = \int f\,d\mu$, and time averages equal the space average.

## 29.2.2 The Driven Reservoir as a Random Dynamical System

A reservoir computing system driven by input $u: \mathbb{Z} \to \mathcal{U}$ evolves as

$$
\mathbf{x}(t+1) = F(\mathbf{x}(t), u(t+1)),
$$

where $F: \mathbb{R}^N \times \mathcal{U} \to \mathbb{R}^N$ is the reservoir map (e.g., $F(\mathbf{x}, u) = \tanh(W^{\text{rec}}\mathbf{x} + W^{\text{in}}u)$). This is a **random dynamical system** (RDS) [Arnold 1998]: the map $F$ varies with the "random" input $u(t)$.

**The skew-product representation.** The driven reservoir can be written as a deterministic system on the extended state space $(\mathbf{x}, u) \in \mathbb{R}^N \times \mathcal{U}^{\mathbb{Z}}$:

$$
\Phi: (\mathbf{x}, u) \mapsto (F(\mathbf{x}, u(0)),\, \sigma u),
$$

where $\sigma: \mathcal{U}^{\mathbb{Z}} \to \mathcal{U}^{\mathbb{Z}}$ is the left shift ($(\sigma u)(t) = u(t+1)$). This formulation is developed in detail in Section 29.4; here we use it to connect to ergodic theory.

If the input sequence $u$ is drawn from a stationary ergodic process with measure $\mathbb{P}$ on $\mathcal{U}^{\mathbb{Z}}$, then $\sigma$ is $\mathbb{P}$-preserving and ergodic. The full system $(\mathbb{R}^N \times \mathcal{U}^{\mathbb{Z}}, \Phi)$ is a random dynamical system over the ergodic base $(\mathcal{U}^{\mathbb{Z}}, \mathbb{P}, \sigma)$.

## 29.2.3 Stationary Measures and the Echo State Property

**Definition 29.2 (Stationary Measure for Driven Reservoir).** A probability measure $\mu$ on $\mathbb{R}^N$ is **stationary** for the driven reservoir if it is invariant under the expected action of $F$:

$$
\mu(A) = \mathbb{E}_u[F(\cdot, u)_* \mu](A) = \int_{\mathcal{U}} \mu(F(\cdot, u)^{-1}A)\,\mathbb{P}_u(du), \quad \forall A \in \mathcal{B}(\mathbb{R}^N).
$$

In words: if the reservoir state $\mathbf{x}$ is distributed according to $\mu$ and the input $u$ is drawn from its marginal distribution, then the next reservoir state $F(\mathbf{x}, u)$ is also distributed according to $\mu$.

**Theorem 29.2 (ESP $\Leftrightarrow$ Unique Stationary Measure).** Under mild regularity conditions on $F$ (Feller property), the following are equivalent:

1. The driven reservoir satisfies the ESP: for any two initial conditions $\mathbf{x}_0, \mathbf{x}_0'$ and any input sequence $u$, $\|F^t(\mathbf{x}_0, u) - F^t(\mathbf{x}_0', u)\|_2 \to 0$ as $t \to \infty$.

2. The driven reservoir has a unique stationary measure $\mu_u$ for each (stationary ergodic) input process $u$.

*Proof sketch.* (1) $\Rightarrow$ (2): If the ESP holds, any two trajectories starting from different initial conditions converge to the same trajectory. Any measure supported on two different trajectories is therefore not stationary (since the dynamics collapse them). Hence there is at most one stationary measure; existence follows from Krylov-Bogoliubov. (2) $\Rightarrow$ (1): If there are two distinct stationary measures $\mu$ and $\mu'$, consider their supports: two trajectories starting in the supports of $\mu$ and $\mu'$ do not converge, violating ESP. $\square$

**Ergodic reformulation.** The ESP means that the driven reservoir has a unique stationary measure. In ergodic language: the skew-product system $\Phi$ has a unique ergodic stationary measure for each input process. The echo state is the barycenter of this measure.

## 29.2.4 The Measurable Echo Function

**Definition 29.3 (Echo Function).** The **echo function** at time $t$ is the map $E_t: u \mapsto \mathbf{x}^*(t, u)$ from input sequences to reservoir states, where $\mathbf{x}^*(t, u)$ is the unique limit state (independent of initial conditions).

If the ESP holds, $E_t$ is well-defined for $\mathbb{P}$-almost every input sequence. The question is: is $E_t$ **measurable**?

**Proposition 29.3.** The ESP implies that $E_t: \mathcal{U}^{\mathbb{Z}_-} \to \mathbb{R}^N$ is measurable with respect to the $\sigma$-algebra generated by the past input $\{u(s): s \leq t\}$.

*Proof.* The echo state $\mathbf{x}^*(t, u) = \lim_{T\to\infty} F^T(\mathbf{x}_0, u(t-T), \ldots, u(t))$ is a limit of measurable functions of the input (since each $F^T$ is measurable and the limit exists by ESP). $\square$

**Connection to ergodic decomposition.** For a stationary ergodic input process, $\mathbb{P}$-a.s. uniqueness of the echo function (Proposition 29.3) follows from the unique ergodicity (Theorem 29.2). When the ESP fails (multiple stationary measures), different initial conditions may produce different limit states, and the echo function is not well-defined — training is meaningless because the readout trains on a trajectory that depends on the (arbitrary) initialization.

## 29.2.5 Pullback Attractors

The connection to pullback attractors (developed in Section 29.3) provides a more geometric perspective on the ESP as an ergodic property.

**Definition 29.4 (Pullback Attractor).** A **pullback attractor** for the driven reservoir is a family of sets $\{A_t(u)\}_{t \in \mathbb{Z}}$ (depending on the input sequence $u$) such that:
1. **Invariance:** $F(\cdot, u(t+1))A_t(u) = A_{t+1}(u)$ for all $t$.
2. **Pullback attraction:** For any bounded set $D \subseteq \mathbb{R}^N$, $\mathrm{dist}(F^T(D, u_{t-T:t}),\, A_t(u)) \to 0$ as $T \to \infty$.

**Theorem 29.4 (ESP $\Leftrightarrow$ Singleton Pullback Attractor).** The driven reservoir satisfies the ESP if and only if the pullback attractor $A_t(u)$ is a singleton $\{\mathbf{x}^*(t,u)\}$ for $\mathbb{P}$-a.e. input sequence $u$ and all $t$.

This result [Crauel & Flandoli 1994] is the definitive geometric characterization of the ESP. The echo state $\mathbf{x}^*(t,u)$ is the unique point in the pullback attractor at time $t$.

## 29.2.6 Implications for Training

The ergodic perspective has direct implications for reservoir learning:

**1. Convergence of ridge regression.** If the ESP holds, the training state matrix $\mathbf{X} = [\mathbf{x}(1), \ldots, \mathbf{x}(T)]^T$ converges as $T \to \infty$ (by Birkhoff's theorem applied to the ergodic driven reservoir) to a fixed state covariance matrix $\Sigma$. Ridge regression then converges to a fixed readout $\hat{\mathbf{w}}$ regardless of initialization. Without ESP, $\Sigma$ depends on the initialization, and different training runs produce different readouts.

**2. Washout period justification.** The washout period (discarding the first $T_{\text{wash}}$ time steps before recording states) is necessary because the initial condition $\mathbf{x}(0)$ may be far from the pullback attractor. After $T_{\text{wash}} \gg 1/|\log\rho(W^{\text{rec}})|$ steps, the trajectory has converged to the pullback attractor, and the recorded states are independent of $\mathbf{x}(0)$.

**3. Ergodicity of the learned readout.** Once $\hat{\mathbf{w}}$ is trained on a (sufficiently long) ergodic training sequence, the test performance equals the time-average performance — by Birkhoff's theorem. This is the rigorous justification for the standard practice of using a single long training sequence rather than multiple short ones.

## References

- Arnold, L. (1998). *Random Dynamical Systems*. Springer.
- Birkhoff, G. D. (1931). Proof of the ergodic theorem. *Proceedings of the National Academy of Sciences*, 17(12), 656–660.
- Crauel, H. and Flandoli, F. (1994). Attractors for random dynamical systems. *Probability Theory and Related Fields*, 100(3), 365–393.
- Jaeger, H. (2001). *The "echo state" approach to analysing and training recurrent neural networks*. GMD Technical Report 148.
- Ruelle, D. (1989). *Chaotic Evolution and Strange Attractors*. Cambridge University Press.
