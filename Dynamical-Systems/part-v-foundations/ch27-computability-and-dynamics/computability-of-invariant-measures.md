# 27.2 Computability of Invariant Measures

Having established that dynamical systems and computations are the same kind of object, we can ask a finer question: what can we actually compute about the long-run statistical behavior of a system? The natural invariant measures — the measures that tell us where a typical orbit spends its time — are the central objects of ergodic theory. But are they computable?

The answer is nuanced in a way that turns out to be mathematically important. Some invariant measures are computable; others are not, even for seemingly simple systems. The distinction traces directly back to the ML-randomness of initial conditions.

**Definition 27.2.1.** A Borel probability measure $\mu$ on a compact metric space $X$ is *computable* if $\mu(U)$ is a computable real number for every computable open set $U$.

This is the "right" definition: we ask whether the measure of any effectively described region of space can be approximated to arbitrary precision by an algorithm. It's a strong demand — we need uniformity over all computable open sets — but it turns out to be satisfied by many natural examples.

**Theorem 27.2.2 (Computability of Ergodic Averages).** For a computable dynamical system $(X, f)$ and a computable integrable function $\varphi$:
- If $\mu$ is a computable $f$-invariant measure and $x$ is $\mu$-generic (ML-random), then $\frac{1}{n}\sum_{k<n}\varphi(f^k(x))$ converges computably to $\int\varphi\,d\mu$.
- The rate of convergence is computable from the ML-randomness of $x$.

In plain terms: Birkhoff averages along generic orbits are computable, and the speed of convergence can itself be computed — as long as the starting point is random enough. The Kolmogorov complexity of $x$ controls how fast the averages settle down.

What happens when the measure is computable but the initial condition is not random? Things can go wrong in a precise way.

**Theorem 27.2.3 (Galatolo-Hoyrup-Rojas).** For a computable expanding map $f$ on the circle:
- The absolutely continuous invariant measure (ACIM) is computable.
- For Lebesgue-a.e. $x$, ergodic averages converge computably.
- There exist $x$ where the ergodic averages are not computable at all (e.g., $x$ is computable but not ML-random).

This is a striking result. The ACIM is computable. The ergodic theorem tells us averages converge for a.e. point. But if you hand the algorithm a computable, non-random initial condition — a point whose orbit somehow "misses" the typical behavior — the ergodic averages may converge without being computable as real numbers.

The moral: computability of a measure does not imply computability of ergodic averages at every point. You genuinely need randomness in the starting position. The connection between ML-randomness and ergodic theory is one of the deeper threads of modern algorithmic information theory, and this chapter is where it shows up in the dynamics literature.

In the next section, we take a different angle: self-reference, fixed points, and what the recursion theorem says about self-replicating dynamical systems.
