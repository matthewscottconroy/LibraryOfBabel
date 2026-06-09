# 8.3 LaSalle's Invariance Principle

Lyapunov's theorem requires $\dot{V} < 0$ for asymptotic stability. But in practice, many natural Lyapunov functions — like total energy — satisfy only $\dot{V} \leq 0$, with equality on a nontrivial set. The damped oscillator is a perfect example: $\dot{V} = -cx_2^2$, which vanishes on the entire horizontal axis $\{x_2 = 0\}$.

In such cases, Lyapunov's theorem only gives stability, not asymptotic stability. But physical intuition says the system should converge to the origin anyway — it keeps losing energy and can't stay on the horizontal axis. LaSalle's invariance principle (1960) makes this precise.

Lyapunov's theorem requires $\dot{V} < 0$. What if $\dot{V} \leq 0$ but with equality on a nontrivial set?

**Theorem 8.3.1 (LaSalle's Invariance Principle).** Let $V$ be a Lyapunov function with $\dot{V} \leq 0$. Let $E = \{x : \dot{V}(x) = 0\}$ and $M$ = largest positively invariant set contained in $E$. Then every bounded trajectory converges to $M$ as $t \to \infty$.

*(proof)* The orbit lies in the compact sublevel set $\{V \leq V(x(0))\}$. By Birkhoff's theorem (topological version), $\omega(x(0)) \neq \emptyset$. Since $V$ decreases along trajectories, $V|_{\omega(x(0))} = c$ for some constant $c$, so $\dot{V}|_{\omega(x(0))} = 0$, i.e., $\omega(x(0)) \subseteq E$. Since $\omega(x(0))$ is positively invariant, $\omega(x(0)) \subseteq M$.

What this is really saying: since $V$ is non-increasing along trajectories, and the orbit is bounded (lies in a compact sublevel set), $V$ must converge to some limit along the orbit. The omega-limit set sits at that limiting value, so $\dot{V} = 0$ on the omega-limit set. The omega-limit set is therefore contained in $E = \{\dot{V} = 0\}$. But the omega-limit set is invariant, so it must sit inside the largest invariant subset of $E$, which is $M$. Every trajectory converges to $M$.

**Corollary 8.3.2.** If $M = \{0\}$ (the only invariant set in $E$ is the origin), then the origin is asymptotically stable.

**Example 8.3.3.** Return to the damped oscillator. $E = \{x_2 = 0\}$. On $E$: $\dot{x}_2 = -kx_1$, so the trajectory immediately leaves $E$ unless $x_1 = 0$ too. The largest invariant set in $E$ is $\{0\}$, so the origin is GAS.

This is the argument we were aiming for. On the set $\{x_2 = 0\}$, the dynamics immediately push $x_2$ away from zero (as long as $x_1 \neq 0$). So no trajectory can stay in $E$ except the origin itself. LaSalle tells us every trajectory converges to the largest invariant subset of $E$, which is $\{0\}$.

LaSalle's principle is particularly powerful in control and robotics, where energy-like Lyapunov functions arise naturally but rarely have strictly negative time derivatives. It's also the bridge between Lyapunov stability theory and the Poincaré-Bendixson theorem: in a bounded region of the plane, every omega-limit set is either an equilibrium or a periodic orbit, and LaSalle can often rule out periodic orbits (if $\dot{V} < 0$ on periodic orbits, they'd have to move to a lower sublevel set, contradicting periodicity).

In the next section, we ask the converse question: must every stable system have a Lyapunov function?
