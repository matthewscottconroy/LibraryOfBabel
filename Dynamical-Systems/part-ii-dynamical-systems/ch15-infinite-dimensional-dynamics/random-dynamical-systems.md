# 15.5 Random Dynamical Systems

An SDE generates a random process — a random path $X_t^\omega$ depending on the noise realization $\omega$. But each individual realization gives a *deterministic* trajectory: for fixed $\omega$, the map $x \mapsto X_t^\omega(x)$ (the solution starting at $x$ with noise $\omega$) is a well-defined deterministic function. The theory of *random dynamical systems* (RDS) makes this perspective precise and provides the framework for studying attractors, Lyapunov exponents, and bifurcations in the random setting.

The key concept is the *cocycle*: a family of maps indexed by time and noise realization, satisfying the appropriate composition rule.

**Definition 15.5.1 (Cocycle).** A *random dynamical system* over a probability-preserving transformation $\theta: (\Omega, \mathcal{F}, P) \to (\Omega, \mathcal{F}, P)$ (the "driving noise") is a measurable map:
$$\Phi: \mathbb{R}_+ \times \Omega \times X \to X$$
satisfying the *cocycle property* $P$-a.s.:
$$\Phi(t+s, \omega) = \Phi(t, \theta^s\omega) \circ \Phi(s, \omega).$$

Here $\theta^s\omega$ is the "shifted noise" — the noise realization starting from time $s$ instead of time 0. The cocycle property says: the map from time 0 to time $t+s$ (with noise $\omega$) equals the composition of the map from 0 to $s$ (with noise $\omega$) and the map from $s$ to $t+s$ (with the shifted noise $\theta^s\omega$).

**Example 15.5.2.** The solution $X_t^\omega(x)$ of an SDE defines a random dynamical system with $\Phi(t, \omega)(x) = X_t^\omega(x)$ — the solution starting at $x$ under the noise realization $\omega$. The driving transformation $\theta$ is the time-shift on the noise path: $(\theta^s\omega)(t) = \omega(t+s) - \omega(s)$ (shift and re-center).

The cocycle formalism recovers the semigroup property in a random setting: if you fix $\omega$ and vary $t$, you get a family of maps; but the composition rule involves the shifted noise, not the same noise.

## 15.5.1 Random Attractors

For a deterministic dissipative system, the global attractor is a single compact invariant set. For a random dynamical system, the attractor must be random — a family of compact sets $\mathcal{A}(\omega)$ depending on the noise realization, evolving with the noise over time.

**Definition 15.5.3.** A *random attractor* for an RDS $\Phi$ is a family of compact sets $\mathcal{A}(\omega)$ (depending measurably on $\omega \in \Omega$) that is:
- *Invariant*: $\Phi(t, \omega)\mathcal{A}(\omega) = \mathcal{A}(\theta^t\omega)$ for all $t \geq 0$ (the attractor at time 0 maps to the attractor at time $t$, with shifted noise)
- *Pullback attracting*: for any bounded $B \subseteq X$:
$$\text{dist}\left(\Phi(t, \theta^{-t}\omega)B,\ \mathcal{A}(\omega)\right) \to 0 \quad \text{as } t \to \infty.$$

The *pullback* convention is crucial and deserves explanation. Instead of starting at time 0 and watching the distribution converge to $\mathcal{A}$ in the future, we start at time $-t$ (in the far past) and watch what the system looks like at time 0. As $t \to \infty$, the system has had longer and longer to "forget" its initial condition. The pullback attractor $\mathcal{A}(\omega)$ is what you see at time 0 if you started the system infinitely far in the past with the noise realization $\omega$.

This might seem backwards, but it is the right notion for random (and more generally, non-autonomous) systems. The "forward" notion — convergence of the distribution at time $t$ as $t \to \infty$ — depends on the initial time, and for random systems the relevant structure is the pullback limit.

**Theorem 15.5.4 (Existence of Random Attractors).** Under conditions analogous to the deterministic case — asymptotic compactness and dissipativity of the pullback dynamics — a random dynamical system has a random attractor. The random attractor is the $P$-a.s. limit of the pullback of a bounded absorbing set:
$$\mathcal{A}(\omega) = \bigcap_{T \geq 0} \overline{\bigcup_{t \geq T} \Phi(t, \theta^{-t}\omega) B_0}.$$

For an SDE with dissipative drift and additive noise, the random attractor often collapses to a single random point — a *random fixed point* $x^*(\omega)$ that is a stationary solution of the SDE (a solution that is adapted to the noise and whose distribution is the stationary distribution). This random fixed point moves with the noise, but its distribution is the stationary distribution of the Fokker-Planck equation.
