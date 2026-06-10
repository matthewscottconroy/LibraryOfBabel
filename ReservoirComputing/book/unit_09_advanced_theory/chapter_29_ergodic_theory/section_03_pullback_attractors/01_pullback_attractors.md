# Section 29.3: Pullback Attractors and the Echo State Property

## 29.3.1 Non-Autonomous Dynamical Systems

Classical dynamical systems theory studies autonomous systems: $\dot{x} = f(x)$ or $x(t+1) = g(x(t))$, where the right-hand side does not depend explicitly on time. The long-run behavior is captured by attractors — invariant sets that nearby orbits converge to.

Reservoir computing is fundamentally non-autonomous: the reservoir evolves as
$$x(t+1) = F(x(t), u(t))$$
where $F$ depends on the external input $u(t)$ at each time step. The input is not part of the state — it is an external "driver." The long-run behavior of such a system depends on the input sequence, and classical attractor theory does not directly apply.

**Definition 29.3.1 (Non-Autonomous Dynamical System / Cocycle).** A *non-autonomous discrete-time dynamical system* is a tuple $(\Omega, T, \mathcal{X}, \varphi)$ where:
- $(\Omega, \mathcal{F}, \mu, T)$ is a measure-preserving dynamical system (the *base system*, representing the input process),
- $\mathcal{X}$ is a complete metric space (the *fiber*, representing the reservoir state space),
- $\varphi: \mathbb{Z}_+ \times \Omega \times \mathcal{X} \to \mathcal{X}$ is a measurable map satisfying the *cocycle property*:
  $$\varphi(0, \omega, x) = x, \quad \varphi(m+n, \omega, x) = \varphi(m, T^n\omega, \varphi(n, \omega, x)).$$

The cocycle property says: evolving for $m+n$ steps starting from $(\omega, x)$ equals evolving $n$ steps from $(\omega, x)$ and then $m$ steps from the new position $(T^n\omega, \varphi(n,\omega,x))$.

For a reservoir with update rule $x(t+1) = F(x(t), u(t))$:
$$\varphi(n, \omega, x_0) = F(\cdots F(F(x_0, u_0(\omega)), u_1(T\omega)) \cdots, u_{n-1}(T^{n-1}\omega)),$$
where $u_k(T^k\omega) = u(k)$ is the input at time $k$ when the input sequence is $\omega$.

## 29.3.2 The Pullback Attractor

For non-autonomous systems, there are two natural notions of attractor:

1. **Forward attractor**: $A(t)$ such that $\text{dist}(\varphi(t, \omega, B), A(T^t\omega)) \to 0$ as $t \to \infty$ (the current state converges to the attractor at the current time).

2. **Pullback attractor**: $A(t)$ such that $\text{dist}(\varphi(t, T^{-t}\omega, B), A(\omega)) \to 0$ as $t \to \infty$ (starting further and further back in time, the state converges to the attractor now).

The pullback attractor is the appropriate concept for reservoir computing: we are interested in what the reservoir state *is now*, given that the input has been running since $-\infty$.

**Definition 29.3.2 (Pullback Attractor).** A family of compact sets $\{A(\omega)\}_{\omega \in \Omega}$ is a *pullback attractor* for the cocycle $\varphi$ if:
1. **Invariance**: $\varphi(1, \omega, A(\omega)) = A(T\omega)$ for all $\omega$.
2. **Pullback attracting**: For every bounded set $B \subseteq \mathcal{X}$:
   $$\lim_{t \to \infty} \text{dist}_{\mathcal{H}}(\varphi(t, T^{-t}\omega, B), A(\omega)) = 0,$$
   where $\text{dist}_{\mathcal{H}}$ denotes the Hausdorff distance.

Equivalently, using the notation of "running the system from time $-t$ to time $0$":
$$A(\omega) = \bigcap_{s \geq 0}\, \overline{\bigcup_{t \geq s} \varphi(t, T^{-t}\omega, B)}.$$

The limit (intersection of closed sets of decreasing diameter) exists when the system is dissipative (orbits are eventually confined to a bounded region).

## 29.3.3 The Main Theorem: ESP Equals Single-Valued Pullback Attractor

The following theorem is the central result of this chapter. It establishes that the echo state property is precisely equivalent to the pullback attractor being a single point (as a function of $\omega$, which encodes the entire past input).

**Theorem 29.3.1 (Echo State Property as Pullback Attractor).** *Let $\varphi$ be the cocycle of a reservoir with update rule $x(t+1) = F(x(t), u(t))$, where $F: \mathcal{X} \times U \to \mathcal{X}$ is a continuous map and $\mathcal{X} \subseteq \mathbb{R}^N$ is a compact invariant set. The following are equivalent:*

*(i) The reservoir has the echo state property: for $\mu$-a.e. input sequence $\omega$, there exists a unique state $x^*(\omega) \in \mathcal{X}$ such that*
$$\lim_{t \to \infty} \|\varphi(t, T^{-t}\omega, x_0) - x^*(\omega)\| = 0 \quad \text{for all } x_0 \in \mathcal{X}.$$

*(ii) The pullback attractor $\{A(\omega)\}_{\omega \in \Omega}$ is $\mu$-a.s. a singleton: $A(\omega) = \{x^*(\omega)\}$ for $\mu$-a.e. $\omega$.*

*(iii) There exists a measurable function $x^*: \Omega \to \mathcal{X}$ (the echo state response) such that*
$$x^*(T\omega) = F(x^*(\omega), u(\omega)) \quad \mu\text{-a.s.}$$
*(the echo state response is a stationary solution of the driven system).*

**Proof.** We prove (i) $\Leftrightarrow$ (ii) and (ii) $\Leftrightarrow$ (iii).

**(i) $\Rightarrow$ (ii)**: Suppose the ESP holds. For $\mu$-a.e. $\omega$, all orbits starting from $\mathcal{X}$ at time $-t$ converge to $x^*(\omega)$ as $t \to \infty$. Therefore:
$$A(\omega) = \bigcap_{s \geq 0}\overline{\bigcup_{t \geq s} \varphi(t, T^{-t}\omega, \mathcal{X})} = \{x^*(\omega)\},$$
since the sets $\bigcup_{t \geq s}\varphi(t, T^{-t}\omega, \mathcal{X})$ shrink to the single point $x^*(\omega)$.

**(ii) $\Rightarrow$ (i)**: If $A(\omega) = \{x^*(\omega)\}$ is a singleton for $\mu$-a.e. $\omega$, then by the pullback attracting property (Definition 29.3.2), $\text{dist}(\varphi(t, T^{-t}\omega, \mathcal{X}), \{x^*(\omega)\}) \to 0$, which means all initial conditions converge to $x^*(\omega)$. This is precisely the ESP.

**(ii) $\Rightarrow$ (iii)**: Invariance of the pullback attractor gives $\varphi(1, \omega, A(\omega)) = A(T\omega)$. If $A(\omega) = \{x^*(\omega)\}$ and $A(T\omega) = \{x^*(T\omega)\}$, then:
$$\{x^*(T\omega)\} = \{F(x^*(\omega), u(\omega))\},$$
i.e., $x^*(T\omega) = F(x^*(\omega), u(\omega))$ a.s.

**(iii) $\Rightarrow$ (ii)**: Let $x^*$ be the measurable function satisfying $x^*(T\omega) = F(x^*(\omega), u(\omega))$ a.s. We claim $A(\omega) = \{x^*(\omega)\}$ a.s. First, $x^*(\omega)$ is in $A(\omega)$ by definition (it is a limit point of all orbits starting from $x^*(\omega)$, which is trivially $x^*(\omega)$). If $y \in A(\omega)$ is any other pullback limit, then there exist initial conditions $x_0^{(t)} \in \mathcal{X}$ with $\varphi(t, T^{-t}\omega, x_0^{(t)}) \to y$. But $\varphi(t, T^{-t}\omega, x^*(T^{-t}\omega)) = x^*(\omega)$ (by the cocycle property and the fact that $x^*$ is a stationary solution). If the system is *contracting* in a suitable sense (which follows from the ESP), then $|y - x^*(\omega)| = |\lim_t \varphi(t, T^{-t}\omega, x_0^{(t)}) - \varphi(t, T^{-t}\omega, x^*(T^{-t}\omega))| \leq \lim_t C e^{-\gamma t}\|x_0^{(t)} - x^*(T^{-t}\omega)\| = 0$. So $y = x^*(\omega)$ a.s. $\blacksquare$

**Remark 29.3.1 (Role of Compactness).** The assumption that $\mathcal{X}$ is compact is used to ensure the pullback attractor exists (as an intersection of non-empty closed sets, by Cantor's intersection theorem). For unbounded reservoir state spaces, one needs to verify that orbits are ultimately bounded — this follows from a global stability condition on the reservoir map $F$.

## 29.3.4 Conditions for the Echo State Property

Theorem 29.3.1 characterizes the ESP structurally. For practical verification, we need conditions on $F$ that guarantee the ESP.

**Proposition 29.3.1 (Contraction Sufficient Condition).** *If $F(\cdot, u)$ is a uniform contraction: there exists $0 < \gamma < 1$ such that*
$$\|F(x, u) - F(y, u)\| \leq \gamma \|x - y\| \quad \text{for all } x, y \in \mathcal{X}, u \in U,$$
*then the reservoir has the ESP.*

**Proof.** For any two initial conditions $x_0, y_0 \in \mathcal{X}$:
$$\|\varphi(t, \omega, x_0) - \varphi(t, \omega, y_0)\| \leq \gamma^t \|x_0 - y_0\|.$$
Therefore $\varphi(t, T^{-t}\omega, x_0) - \varphi(t, T^{-t}\omega, y_0) \to 0$ for all $x_0, y_0$, proving the pullback attractor is a singleton. $\blacksquare$

For a linear reservoir $x(t+1) = Wx(t) + Wu \cdot u(t)$, uniform contraction holds iff $\|W\|_{\text{op}} < 1$. This is a stronger condition than $\rho(W) < 1$ (spectral radius), but the two conditions agree in the limit $N \to \infty$ for symmetric $W$ (since $\|W\|_{\text{op}} = \rho(W)$ when $W$ is symmetric).

**Proposition 29.3.2 (ESP for RNNs with Bounded Inputs).** *For an ESN with tanh activation:*
$$x(t+1) = \tanh(Wx(t) + W_{\text{in}}u(t) + b),$$
*a sufficient condition for the ESP is $\|W\|_{\text{op}} < 1$ (where $\|\cdot\|_{\text{op}}$ is the operator norm, i.e., the largest singular value).*

**Proof.** We have $\|\tanh(a) - \tanh(b)\| \leq \|a - b\|$ (since $|\tanh'(t)| = \text{sech}^2(t) \leq 1$). Therefore:
$$\|x(t+1) - y(t+1)\| = \|\tanh(Wx(t) + \ldots) - \tanh(Wy(t) + \ldots)\| \leq \|W(x(t) - y(t))\| \leq \|W\|_{\text{op}} \|x(t) - y(t)\|.$$
So $F(\cdot, u)$ is a contraction with constant $\|W\|_{\text{op}} < 1$. By Proposition 29.3.1, the ESP holds. $\blacksquare$

**Remark 29.3.2 (ESP vs Spectral Radius).** The standard reservoir computing literature uses $\rho(W) < 1$ as the ESP condition. This is necessary (not sufficient) for the ESP in general — there exist matrices with $\rho(W) < 1$ but $\|W\|_{\text{op}} \geq 1$ that do not satisfy the ESP. In practice, reservoirs with spectral radius slightly below 1 often work well, and the gap between $\rho(W)$ and $\|W\|_{\text{op}}$ is not critical for empirical performance.

## 29.3.5 Stability of the Echo State Property

**Theorem 29.3.2 (Structural Stability of ESP).** *If the reservoir has the ESP (uniform pullback contraction), then the echo state map $\omega \mapsto x^*(\omega)$ is Hölder continuous: there exist $C, \alpha > 0$ such that for inputs $\omega, \omega'$ agreeing on $\{-K+1, \ldots, 0\}$ (same last $K$ inputs) but potentially differing at $\{-K, -K-1, \ldots\}$:*
$$\|x^*(\omega) - x^*(\omega')\| \leq C \gamma^K,$$
*where $\gamma$ is the contraction rate.*

**Proof.** Since $x^*(\omega)$ depends on the entire input history via $x^*(\omega) = \lim_{t \to \infty} \varphi(t, T^{-t}\omega, x_0)$, two input sequences that agree on the last $K$ steps differ only in their contributions from before time $-K$. The contraction property ensures that the contribution from before time $-K$ is down-weighted by $\gamma^K$. $\blacksquare$

This is exactly the fading memory property: the echo state map $x^*(\omega)$ depends on recent inputs strongly and on old inputs weakly. The ESP and fading memory are thus two faces of the same coin, connected through the pullback attractor framework.
