# 4.1 Existence and Uniqueness

## 4.1.1 The Initial Value Problem

The fundamental question: given a vector field (a rule for motion at each point), does a trajectory through any given starting point exist, and is it unique? This sounds basic, but answering it carefully requires the full machinery of the Banach Fixed Point Theorem.

**Setup.** Let $U \subseteq \mathbb{R}^n$ be open and $f: U \to \mathbb{R}^n$ a smooth (or merely Lipschitz) vector field. The *initial value problem (IVP)* is:
$$\dot{x} = f(x), \quad x(0) = x_0 \in U.$$

A *solution* on an interval $I \ni 0$ is a differentiable map $\varphi: I \to U$ satisfying $\dot{\varphi}(t) = f(\varphi(t))$ for all $t \in I$ and $\varphi(0) = x_0$.

The key reformulation: integrate both sides from $0$ to $t$. Solutions are precisely fixed points of a certain operator:

**Integral Formulation.** The IVP is equivalent to the integral equation:
$$\varphi(t) = x_0 + \int_0^t f(\varphi(s))\,ds.$$

This is the Picard iteration operator $T[\varphi](t) = x_0 + \int_0^t f(\varphi(s))\,ds$. A solution is a fixed point of $T$. This reformulation is the key: it converts a differential equation into a fixed-point problem, and we know how to solve fixed-point problems — use the Banach Fixed Point Theorem.

## 4.1.2 Picard-Lindelöf Theorem

For the Banach Fixed Point Theorem to apply, we need $T$ to be a contraction on a complete metric space. The Lipschitz condition on $f$ is exactly what makes $T$ contractive:

**Definition 4.1.1.** $f: U \to \mathbb{R}^n$ is *locally Lipschitz* if for each compact $K \subseteq U$ there is $L = L(K) > 0$ with $\|f(x) - f(y)\| \leq L\|x - y\|$ for all $x, y \in K$.

*Note:* $C^1$ vector fields are locally Lipschitz, so Picard-Lindelöf applies to all $C^1$ vector fields.

**Theorem 4.1.2 (Picard-Lindelöf / Cauchy-Lipschitz).** Let $f: U \to \mathbb{R}^n$ be locally Lipschitz with Lipschitz constant $L$ on $\bar{B}(x_0, r)$. Set $M = \sup_{x \in \bar{B}(x_0,r)} \|f(x)\|$ and $T = \min(r/M, 1/(2L))$. Then there exists a unique solution $\varphi: [-T, T] \to \bar{B}(x_0, r)$ to the IVP.

*(proof)* The operator $T[\varphi](t) = x_0 + \int_0^t f(\varphi(s))\,ds$ maps the complete metric space $\mathcal{X} = \{\varphi \in C([-T,T], \bar{B}(x_0,r))\}$ to itself (for small enough $T$), and is a contraction:
$$\|T[\varphi] - T[\psi]\|_\infty \leq L \cdot T \cdot \|\varphi - \psi\|_\infty \leq \frac{1}{2}\|\varphi - \psi\|_\infty.$$
The Banach Fixed Point Theorem gives the unique fixed point.

The time interval $T = \min(r/M, 1/(2L))$ has a clear interpretation: $r/M$ is how long it takes to cross the ball at maximum speed, and $1/(2L)$ is the contraction time. The bound $T \leq 1/(2L)$ is what makes the operator a contraction with constant $1/2$.

Solutions don't just exist locally — they can be extended to maximal intervals:

**Theorem 4.1.3 (Maximal Solutions).** Under the hypotheses of Picard-Lindelöf, for each $x_0 \in U$ there exists a unique *maximal solution* $\varphi: (t^-, t^+) \to U$ (with $-\infty \leq t^- < 0 < t^+ \leq +\infty$) that cannot be extended to a larger interval. If $t^+ < \infty$, the solution *escapes to infinity*: $\|\varphi(t)\| \to \infty$ or $\varphi(t) \to \partial U$ as $t \nearrow t^+$.

The escape criterion is important: a solution can only fail to be global if it runs off to infinity or hits the boundary. If the vector field is defined on all of $\mathbb{R}^n$ and bounded on bounded sets, solutions are global.

**Corollary 4.1.4.** On compact manifolds, every smooth vector field generates a *complete flow* (defined for all $t \in \mathbb{R}$).

This is why compact phase spaces are so convenient: the flow always exists for all time.

## 4.1.3 Dependence on Initial Conditions and Parameters

A fundamental result: solutions depend smoothly on the initial condition. This is what makes the flow a diffeomorphism (not just a continuous map):

**Theorem 4.1.5 (Smooth Dependence).** Let $f: U \to \mathbb{R}^n$ be $C^k$ ($k \geq 1$). Then the solution $\varphi(t, x_0)$ is $C^k$ jointly in $(t, x_0)$. The derivative $D_{x_0}\varphi(t, x_0)$ satisfies the *variational equation* (matrix ODE):
$$\frac{d}{dt} D_{x_0}\varphi = Df(\varphi(t, x_0)) \cdot D_{x_0}\varphi, \quad D_{x_0}\varphi(0) = I.$$

This is the fundamental theorem connecting flows and linearizations. The derivative of the flow with respect to initial conditions satisfies its own linear ODE — the variational equation. This equation is the linearization of the original ODE along the trajectory, and its solutions are the "Jacobi fields" of the flow. Lyapunov exponents are defined as the long-time growth rates of solutions to this variational equation.
