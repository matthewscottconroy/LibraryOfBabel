# Existence and Uniqueness: Picard's Theorem

The most fundamental question in the theory of differential equations is whether a given initial value problem has a solution, and if so, whether that solution is unique. These questions have definitive answers for a wide class of problems, provided by the Picard-Lindelof theorem, which is the cornerstone of the basic existence and uniqueness theory.

## The Lipschitz Condition

The key hypothesis is a regularity condition on the right-hand side of $y' = f(x, y)$ with respect to the $y$-variable. A function $f(x, y)$ satisfies a **Lipschitz condition** in $y$ on a region $R$ if there exists a constant $L > 0$ (the Lipschitz constant) such that

$$|f(x, y_1) - f(x, y_2)| \leq L |y_1 - y_2|$$

for all $(x, y_1)$ and $(x, y_2)$ in $R$. Informally, the Lipschitz condition says that $f$ cannot change too rapidly in the $y$-direction; it is a quantitative continuity condition stronger than mere continuity but weaker than differentiability.

A sufficient condition for the Lipschitz property is that $\partial f/\partial y$ is continuous and bounded on $R$: if $|\partial f/\partial y| \leq L$ throughout $R$, then by the mean value theorem,

$$|f(x, y_1) - f(x, y_2)| = \left|\frac{\partial f}{\partial y}(x, \xi)\right||y_1 - y_2| \leq L|y_1 - y_2|.$$

## The Picard-Lindelof Theorem

**Theorem (Picard-Lindelof).** Let $R = \{(x, y) : |x - x_0| \leq a,\; |y - y_0| \leq b\}$ be a closed rectangle, and suppose $f: R \to \mathbb{R}$ is continuous on $R$ and satisfies a Lipschitz condition in $y$ on $R$ with constant $L$. Let $M = \max_{(x,y) \in R} |f(x,y)|$ and $h = \min(a, b/M)$. Then the initial value problem

$$y' = f(x, y),\qquad y(x_0) = y_0$$

has a unique solution $y = \phi(x)$ on the interval $|x - x_0| \leq h$.

The number $h$ is chosen to ensure that the solution curve stays within the rectangle $R$: the solution must travel no farther than $b$ in the $y$-direction while $x$ moves by at most $a$, and since $|y'| \leq M$, the $y$-displacement in a step of size $h$ is at most $Mh \leq b$.

## Proof via Picard Iteration

The proof is constructive. Define the sequence of **Picard iterates** $\{\phi_n\}$ by

$$\phi_0(x) = y_0,$$
$$\phi_{n+1}(x) = y_0 + \int_{x_0}^{x} f\!\left(t, \phi_n(t)\right)\,dt, \quad n = 0, 1, 2, \ldots$$

**Step 1: Each iterate is well-defined and lies in $R$.** One shows by induction that $|\phi_n(x) - y_0| \leq b$ for $|x - x_0| \leq h$, so the integrand $f(t, \phi_n(t))$ is always evaluated inside $R$.

**Step 2: The iterates converge uniformly.** Define $\epsilon_n(x) = \phi_{n+1}(x) - \phi_n(x)$. Then

$$|\epsilon_1(x)| = \left|\int_{x_0}^x [f(t, \phi_0(t)) - f(t, y_0)]\,dt\right| \leq 0,$$

since $\phi_0 = y_0$. More carefully, $|\phi_1(x) - \phi_0(x)| \leq M|x - x_0|$. By the Lipschitz condition and induction,

$$|\phi_{n+1}(x) - \phi_n(x)| \leq \frac{ML^n |x-x_0|^{n+1}}{(n+1)!} \leq \frac{ML^n h^{n+1}}{(n+1)!}.$$

Since $\sum_{n=0}^\infty \frac{ML^n h^{n+1}}{(n+1)!} = \frac{M}{L}(e^{Lh} - 1) < \infty$, the series $\sum |\phi_{n+1} - \phi_n|$ converges uniformly, so the partial sums $\phi_n = \phi_0 + \sum_{k=0}^{n-1}(\phi_{k+1} - \phi_k)$ converge uniformly to a limit function $\phi$.

**Step 3: The limit is a solution.** Because $f$ is continuous and the convergence is uniform, one may pass to the limit inside the integral:

$$\phi(x) = \lim_{n\to\infty}\phi_{n+1}(x) = y_0 + \int_{x_0}^x f(t, \phi(t))\,dt.$$

Differentiating both sides (valid since the right side is differentiable by the fundamental theorem of calculus), $\phi'(x) = f(x, \phi(x))$ and $\phi(x_0) = y_0$.

**Step 4: Uniqueness.** Suppose $\psi$ is another solution on $|x - x_0| \leq h$. Let $E(x) = |\phi(x) - \psi(x)|$. Then

$$E(x) = \left|\int_{x_0}^x [f(t,\phi(t)) - f(t,\psi(t))]\,dt\right| \leq L\int_{x_0}^x E(t)\,dt.$$

By Gronwall's inequality, $E(x) \leq 0 \cdot e^{L|x-x_0|} = 0$, so $\phi = \psi$.

## What the Hypotheses Buy

The theorem requires both continuity of $f$ and the Lipschitz condition. Continuity alone (Peano's theorem) guarantees existence but not uniqueness. The Lipschitz condition is what rules out bifurcating solution curves.

**Failure of uniqueness without Lipschitz.** Consider $y' = |y|^{1/2}$, $y(0) = 0$. The function $f(x,y) = |y|^{1/2}$ is continuous but $\partial f/\partial y = 1/(2|y|^{1/2})$ is unbounded near $y = 0$, so no Lipschitz condition holds near this point. Indeed, both $y = 0$ and

$$y = \begin{cases} 0 & x \leq c \\ \tfrac{1}{4}(x-c)^2 & x > c \end{cases}$$

solve the IVP for any $c \geq 0$, giving infinitely many solutions.

## Higher-Order and Systems

The theorem extends to higher-order equations and to systems. An $n$-th order equation $y^{(n)} = f(x, y, y', \ldots, y^{(n-1)})$ can be written as a first-order system by introducing $u_1 = y$, $u_2 = y'$, ..., $u_n = y^{(n-1)}$. The Picard-Lindelof theorem for systems then applies directly, requiring a Lipschitz condition on $f$ in all variables $u_1, \ldots, u_n$.

For the second-order linear equation $y'' + p(x)y' + q(x)y = g(x)$ with continuous coefficients $p, q, g$ on an interval $I$, the Lipschitz condition is automatically satisfied everywhere on $I$ (since $\partial f/\partial y$ and $\partial f/\partial y'$ are bounded on compact subsets). Consequently, the IVP has a unique solution on the entire interval $I$. This much stronger conclusion, available for linear equations, is discussed in the section on the interval of existence.

## Picard Iteration as a Numerical Method

The Picard iteration is not merely a theoretical device; it can be used to compute solutions as power series. Starting with $\phi_0 = y_0$:

**Example.** Solve $y' = y$, $y(0) = 1$ by iteration.

$$\phi_1(x) = 1 + \int_0^x 1\,dt = 1 + x.$$
$$\phi_2(x) = 1 + \int_0^x (1+t)\,dt = 1 + x + \frac{x^2}{2}.$$
$$\phi_3(x) = 1 + \int_0^x \left(1 + t + \frac{t^2}{2}\right)dt = 1 + x + \frac{x^2}{2} + \frac{x^3}{6}.$$

The pattern gives $\phi_n(x) = \sum_{k=0}^n x^k/k!$, which converges to $e^x$. This confirms that $y = e^x$ is the unique solution, now derived constructively rather than by guessing.
