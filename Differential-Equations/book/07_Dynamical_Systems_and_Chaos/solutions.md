# Solutions: Dynamical Systems and Chaos

## Problem 1: Fixed Points and Stability of a Map

**Problem.** For the map $f(x) = 3x(1-x)$ (logistic map with $r=3$), find all fixed points, classify their stability, and determine whether a period-2 orbit exists.

**Solution.** **Fixed points:** $f(x^*) = x^*$: $3x(1-x) = x \Rightarrow 3x - 3x^2 = x \Rightarrow 2x - 3x^2 = 0 \Rightarrow x(2-3x) = 0$.

Fixed points: $x^* = 0$ and $x^* = 2/3$.

Stability: $f'(x) = 3(1-2x)$.
- At $x^* = 0$: $|f'(0)| = 3 > 1$. Unstable.
- At $x^* = 2/3$: $|f'(2/3)| = |3(1 - 4/3)| = |-1| = 1$. Marginally stable (neutral; need higher-order analysis).

At exactly $r = 3$, the fixed point $x^* = 2/3$ has $f'(x^*) = -1$, which is the bifurcation point where a period-2 orbit is being born. For $r$ slightly above 3, the fixed point loses stability and a period-2 orbit appears.

**Period-2 orbit:** Solve $f^2(x) = x$. Since $f^2(x) = x$ includes fixed points, factor them out. The period-2 points satisfy $f^2(x) = x$ but $f(x) \neq x$, i.e., they are roots of $f^2(x) - x = 0$ that are not roots of $f(x) - x = 0$.

At $r = 3$: $f^2(x) = 3\cdot 3x(1-x)(1-3x(1-x)) = x$ (a degree-4 polynomial). The two fixed points account for two roots. At $r=3$ exactly, the period-2 orbit is at $x = 2/3$ with multiplicity 2 (tangent bifurcation). For $r > 3$, two distinct period-2 points bifurcate off.

---

## Problem 2: Phase Portrait of a Nonlinear System

**Problem.** Analyze the phase portrait of $\dot{x} = y - x^2$, $\dot{y} = -y$.

**Solution.** **Equilibria:** $y - x^2 = 0$ and $y = 0$, giving $x^2 = 0$, so $(x,y) = (0,0)$ is the only equilibrium.

**Jacobian:** $J = \begin{pmatrix}-2x & 1 \\ 0 & -1\end{pmatrix}$.

At $(0,0)$: $J = \begin{pmatrix}0&1\\0&-1\end{pmatrix}$. Eigenvalues: $\lambda_1 = 0$, $\lambda_2 = -1$.

A zero eigenvalue means the equilibrium is non-hyperbolic; the Hartman-Grobman theorem does not apply directly.

**Stable manifold:** The $y$-equation decouples: $\dot{y} = -y$, so $y(t) = y_0 e^{-t}$. Substituting into $\dot{x} = y - x^2$: $\dot{x} = y_0e^{-t} - x^2$.

**Center manifold analysis:** Near $(0,0)$, the center manifold (tangent to the zero eigenspace, the $x$-axis) can be parametrized $y = h(x) = ax^2 + \cdots$. The invariance condition $\dot{y} = h'(x)\dot{x}$ gives $-h = h'(x)(h-x^2)$. At leading order: $-ax^2 = 2ax(ax^2-x^2) \approx -2ax^2$, so $-a = -2a$, giving $a = 0$. The center manifold is locally $y = 0$ (the $x$-axis), and on it $\dot{x} = -x^2$. This gives the flow $x(t) = x_0/(1+x_0 t)$: trajectories on the center manifold are attracted to $(0,0)$ from the left and escape to $+\infty$ from the right, but slowly (algebraically, not exponentially). The origin is a non-isolating equilibrium: it is stable from one direction and unstable from the other.

---

## Problem 3: Lyapunov Exponent for the Logistic Map

**Problem.** Compute the Lyapunov exponent for the logistic map $f_r(x) = rx(1-x)$ at $r = 4$ and interpret it.

**Solution.** $f_4'(x) = 4(1-2x)$.

$\lambda = \lim_{n\to\infty}\frac{1}{n}\sum_{k=0}^{n-1}\ln|f'(x_k)| = \lim\frac{1}{n}\sum\ln|4(1-2x_k)| = \ln 4 + \lim\frac{1}{n}\sum\ln|1-2x_k|$.

The logistic map at $r=4$ is conjugate to the tent map via $x = \sin^2(\pi\theta/2)$. Under this substitution, the tent map $T(\theta) = 2\theta \pmod 1$ has $|T'| = 2$, giving Lyapunov exponent $\ln 2$. The Lyapunov exponent is preserved under smooth conjugacy (with bounded derivatives), so $\lambda_{\text{logistic}} = \ln 2 \approx 0.693$.

**Alternative direct computation.** Using the invariant density: the logistic map at $r=4$ preserves the measure $\mu$ with density $\rho(x) = 1/(\pi\sqrt{x(1-x)})$ (arcsine distribution).

$\lambda = \int_0^1 \ln|f'(x)|\rho(x)\,dx = \int_0^1 \ln(4|1-2x|)\frac{dx}{\pi\sqrt{x(1-x)}} = \ln 4 + \int_0^1\frac{\ln|1-2x|}{\pi\sqrt{x(1-x)}}\,dx$.

The second integral evaluates to $-\ln 2$, giving $\lambda = \ln 4 - \ln 2 = \ln 2$.

**Interpretation.** $\lambda = \ln 2 \approx 0.693$ means nearby orbits diverge by a factor of $e^{\ln 2} = 2$ per iterate. To gain one extra bit of information about the orbit, the initial condition must be known to twice the precision.

---

## Problem 4: Hopf Bifurcation

**Problem.** For the system $\dot{x} = \mu x - y - x(x^2+y^2)$, $\dot{y} = x + \mu y - y(x^2+y^2)$, analyze the bifurcation at $\mu = 0$.

**Solution.** Convert to polar coordinates: $r^2 = x^2 + y^2$, $\theta = \arctan(y/x)$.

$r\dot{r} = x\dot{x} + y\dot{y} = x(\mu x - y - xr^2) + y(x + \mu y - yr^2) = \mu(x^2+y^2) - r^2(x^2+y^2) = \mu r^2 - r^4$.

$\dot{r} = \mu r - r^3 = r(\mu - r^2)$.

$r^2\dot\theta = x\dot{y} - y\dot{x} = x(x+\mu y - yr^2) - y(\mu x - y - xr^2) = x^2 + y^2 = r^2$.

So $\dot\theta = 1$.

**Analysis of $\dot{r} = r(\mu - r^2)$:**
- For $\mu \leq 0$: $r^* = 0$ is the only nonneg equilibrium; $\dot{r} < 0$ for $r > 0$; all solutions spiral into the origin. The origin is stable.
- At $\mu = 0$: the origin loses stability; the eigenvalues of the linearization at the origin are $\pm i$ (purely imaginary).
- For $\mu > 0$: $r^* = \sqrt{\mu}$ is a stable equilibrium of the radial equation; there is a stable limit cycle of radius $\sqrt{\mu}$ and period $2\pi$. The origin becomes unstable (repelling spiral).

This is a **supercritical Hopf bifurcation** at $\mu = 0$: a stable limit cycle is born.

---

## Problem 5: Poincaré-Bendixson and Limit Cycles

**Problem.** For the van der Pol equation $\dot{x} = y$, $\dot{y} = -x + \mu(1-x^2)y$ with $\mu > 0$, show that there exists a stable limit cycle.

**Solution.** **Step 1: No equilibria except the origin.** At an equilibrium: $y = 0$ and $-x + 0 = 0$, so $x = 0$. Only equilibrium: $(0,0)$.

**Step 2: Origin is unstable.** Jacobian: $J = \begin{pmatrix}0&1\\-1&\mu\end{pmatrix}$. Eigenvalues: $\lambda = (\mu \pm \sqrt{\mu^2-4})/2$. Both have positive real part $\mu/2 > 0$. Unstable focus/node.

**Step 3: Boundedness.** Let $V = x^2 + y^2$. $\dot{V} = 2xy + 2y(-x + \mu(1-x^2)y) = 2\mu y^2(1-x^2)$. For $|x| > 1$: $\dot{V} < 0$, so the level set $V = R^2$ is outward-crossing for large $R$. One can construct a trapping annulus $r_1 \leq \sqrt{V} \leq r_2$ with $r_1$ small and $r_2$ large.

**Step 4: Apply Poincaré-Bendixson.** A trajectory starting in the trapping annulus is bounded and cannot escape. Its $\omega$-limit set is nonempty, compact, and invariant. The only equilibrium $(0,0)$ is in the interior of the annulus (outside it). By Poincaré-Bendixson, the $\omega$-limit set is a periodic orbit (limit cycle).

**Common mistake.** Forgetting to exclude equilibria from the trapping region. If the only equilibrium is in the interior of the annulus (not in the annular region itself), then the $\omega$-limit set of a trajectory in the annulus cannot be the equilibrium, so it must be a limit cycle.

---

## Problem 6: Chaos in the Lorenz System

**Problem.** For the Lorenz system with $\sigma = 10$, $\beta = 8/3$, $\rho = 28$: verify that the system is dissipative, find all equilibria, and compute the Jacobian at the non-trivial equilibria.

**Solution.** **Dissipation:** $\nabla\cdot\mathbf{F} = \partial\dot{x}/\partial x + \partial\dot{y}/\partial y + \partial\dot{z}/\partial z = -\sigma - 1 - \beta = -10 - 1 - 8/3 = -41/3$.

Volume shrinks at rate $e^{-(41/3)t}$: strongly dissipative, with all volumes contracting to zero.

**Equilibria:** $\sigma(y-x) = 0 \Rightarrow y = x$. $x(\rho-z)-y = 0 \Rightarrow x(\rho - z - 1) = 0$. $xy - \beta z = 0 \Rightarrow x^2 = \beta z$.

From $x = 0$: $y = 0$, $z = 0$. Origin $(0,0,0)$.

From $x \neq 0$: $z = \rho - 1 = 27$, $x^2 = \beta\cdot 27 = 72$, $x = \pm 6\sqrt{2}$, $y = x$.

$C_\pm = (\pm 6\sqrt{2}, \pm 6\sqrt{2}, 27)$.

**Jacobian at $C_+$:**

$J = \begin{pmatrix}-\sigma & \sigma & 0 \\ \rho - z & -1 & -x \\ y & x & -\beta\end{pmatrix} = \begin{pmatrix}-10&10&0\\1&-1&-6\sqrt{2}\\6\sqrt{2}&6\sqrt{2}&-8/3\end{pmatrix}$.

The characteristic polynomial of this $J$ has one real negative eigenvalue and two complex conjugate eigenvalues with positive real part (for $\rho = 28$), indicating an unstable focus. Trajectories spiral away from $C_+$ and are eventually captured by the strange attractor.
