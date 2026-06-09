# 8.2 Lyapunov's Direct Method

Here is the key idea. Imagine you have a function $V$ shaped like a bowl around the equilibrium — positive everywhere, zero only at the origin, and its level sets $\{V = c\}$ are closed curves surrounding the origin. Now suppose that along any trajectory, $V$ is non-increasing. Then the trajectory can't cross from $\{V < c\}$ to $\{V > c\}$: it stays inside the bowl. That's stability.

This geometric picture is the entire content of Lyapunov's direct method. The rest is making it precise.

**Definition 8.2.1.** A *Lyapunov function* for $\dot{x} = f(x)$ near the origin is a $C^1$ function $V: U \to [0, \infty)$ (on some open $U \ni 0$) satisfying:
1. $V(0) = 0$ and $V(x) > 0$ for $x \neq 0$ (positive definite)
2. $\dot{V}(x) = \nabla V(x) \cdot f(x) \leq 0$ along trajectories (non-increasing)

The key quantity is $\dot{V}(x) = \nabla V \cdot f(x)$: this is the time-derivative of $V$ along the trajectory through $x$, computed without solving the ODE. It's the directional derivative of $V$ in the direction of the vector field. If this is $\leq 0$ everywhere, then $V$ decreases (or stays constant) along every trajectory.

**Theorem 8.2.2 (Lyapunov Stability Theorem).**
- If (1) and (2) hold with $\dot{V} \leq 0$: the origin is Lyapunov stable.
- If (1) holds and $\dot{V}(x) < 0$ for $x \neq 0$ (negative definite): the origin is asymptotically stable.
- If (1) holds and $\dot{V}(x) \leq -\alpha V(x)$ for some $\alpha > 0$: the origin is exponentially stable.

*(proof of stability)* Given $\varepsilon$, let $c = \min_{\|x\|=\varepsilon} V(x) > 0$. The sublevel set $\{V \leq c/2\}$ is compact and contains a ball $\{x : \|x\| < \delta\}$. If $\|x(0)\| < \delta$, then $V(x(0)) < c/2 \leq c$, and since $\dot{V} \leq 0$, $V(x(t)) \leq c$ for all $t \geq 0$, so $x(t) \in \{V \leq c\} \subseteq \{\|x\| \leq \varepsilon\}$.

The proof is a model of clarity. The sublevel sets of $V$ are the "bowls," and the key insight is that $\dot{V} \leq 0$ means trajectories can only move to lower or equal sublevel sets — never out. Once inside $\{V \leq c\}$, the trajectory stays there. Choosing $c$ small enough to fit inside $\{\|x\| \leq \varepsilon\}$ gives stability.

---

## Examples

**Example 8.2.3 (Damped Harmonic Oscillator).** $\ddot{q} + c\dot{q} + kq = 0$ ($c, k > 0$). As a first-order system: $\dot{x}_1 = x_2$, $\dot{x}_2 = -kx_1 - cx_2$. Lyapunov function: $V = kx_1^2/2 + x_2^2/2$ (total energy).
$$\dot{V} = kx_1 x_2 + x_2(-kx_1 - cx_2) = -cx_2^2 \leq 0.$$
$\dot{V} = 0$ iff $x_2 = 0$, but then $\dot{x}_2 = -kx_1 \neq 0$ unless $x_1 = 0$ too. So the origin is asymptotically stable.

Let's see this in action. The Lyapunov function $V = kx_1^2/2 + x_2^2/2$ is the total mechanical energy: potential plus kinetic. For the *undamped* oscillator ($c = 0$), energy is conserved — $\dot{V} = 0$ everywhere — and the equilibrium is Lyapunov stable but not asymptotically stable. Adding damping ($c > 0$) makes $\dot{V} = -cx_2^2 \leq 0$, so energy decreases whenever $x_2 \neq 0$. The system dissipates energy and the trajectory is attracted to the origin.

Notice that $\dot{V} = 0$ when $x_2 = 0$ — on the horizontal axis. But the system can't stay on the horizontal axis (unless at the origin itself), because on the horizontal axis $\dot{x}_2 = -kx_1 \neq 0$ when $x_1 \neq 0$. So the trajectory immediately leaves the horizontal axis. This observation leads to LaSalle's principle, which we develop in the next section.

**Example 8.2.4 (Nonquadratic Lyapunov).** For $\dot{x} = -x^3$: $V(x) = x^2/2$, $\dot{V} = -x^4 \leq 0$. GAS. But note: $\dot{V} \leq -2V^2$, not $-\alpha V$, so stability is not exponential (solutions decay as $t^{-1/2}$, not $e^{-\lambda t}$).

This example shows the importance of the third bullet in Theorem 8.2.2. For $\dot{x} = -x^3$, you can find a Lyapunov function that works (and proves GAS), but the bound on $\dot{V}$ gives $-2V^2$, not $-\alpha V$. This means the Gronwall-type argument for exponential stability breaks down, and indeed the decay is polynomial, not exponential.

The next section handles a case that this theorem misses: when $\dot{V} \leq 0$ but not strictly negative.
