# Solutions: Ordinary Differential Equations

## Problem 1: Separable ODE with Initial Condition

**Problem.** Solve $\frac{dy}{dx} = \frac{x^2}{1+y^2}$, $y(0) = 1$.

**Solution.** Separate: $(1+y^2)\,dy = x^2\,dx$.

Integrate: $y + y^3/3 = x^3/3 + C$.

Apply $y(0) = 1$: $1 + 1/3 = 0 + C$, so $C = 4/3$.

Implicit solution: $y + y^3/3 = x^3/3 + 4/3$, or $3y + y^3 = x^3 + 4$.

The solution cannot be solved explicitly for $y$ in terms of elementary functions.

**Common mistake.** Forgetting the constant of integration or absorbing it incorrectly. The implicit form is the general solution; the IVP pins down $C$.

---

## Problem 2: Linear First-Order ODE

**Problem.** Solve $y' - 2y = e^{3x}$, $y(0) = 0$.

**Solution.** Integrating factor: $\mu = e^{-2x}$.

Multiply: $(e^{-2x}y)' = e^{-2x}e^{3x} = e^x$.

Integrate: $e^{-2x}y = e^x + C$.

$y = e^{3x} + Ce^{2x}$.

Apply $y(0) = 0$: $0 = 1 + C$, so $C = -1$.

Solution: $y = e^{3x} - e^{2x} = e^{2x}(e^x - 1)$.

**Verification:** $y' = 3e^{3x} - 2e^{2x}$. $y' - 2y = 3e^{3x} - 2e^{2x} - 2(e^{3x}-e^{2x}) = e^{3x}$. Correct.

---

## Problem 3: Second-Order ODE with Repeated Root

**Problem.** Solve $y'' - 4y' + 4y = 0$, $y(0) = 1$, $y'(0) = 3$.

**Solution.** Characteristic equation: $r^2 - 4r + 4 = (r-2)^2 = 0$. Repeated root $r = 2$.

General solution (repeated root requires $xe^{rx}$ for second solution): $y = (c_1 + c_2 x)e^{2x}$.

$y' = c_2 e^{2x} + 2(c_1+c_2x)e^{2x} = (2c_1 + c_2 + 2c_2 x)e^{2x}$.

Apply ICs: $y(0) = c_1 = 1$. $y'(0) = 2c_1 + c_2 = 3$, so $c_2 = 3 - 2 = 1$.

Solution: $y = (1+x)e^{2x}$.

**Why two solutions in general.** The solution space of a second-order linear ODE is 2-dimensional. For a repeated root $r$, the functions $e^{rx}$ and $xe^{rx}$ are linearly independent (their Wronskian at any point is $e^{2rx} \neq 0$) and span the solution space.

---

## Problem 4: Variation of Parameters

**Problem.** Solve $y'' - y = e^x/x$.

**Solution.** Homogeneous solutions: $y_1 = e^x$, $y_2 = e^{-x}$.

Wronskian: $W = e^x(-e^{-x}) - e^{-x}(e^x) = -1 - 1 = -2$.

Variation of parameters formulas:
$u_1' = -y_2 f / W = -e^{-x}(e^x/x)/(-2) = 1/(2x)$.
$u_2' = y_1 f / W = e^x(e^x/x)/(-2) = -e^{2x}/(2x)$.

$u_1 = \frac{1}{2}\ln|x|$ (integrate $1/(2x)$).
$u_2 = -\frac{1}{2}\int\frac{e^{2x}}{x}\,dx = -\frac{1}{2}\,\text{Ei}(2x)$

where $\text{Ei}(t) = \int_{-\infty}^t e^s/s\,ds$ is the exponential integral (a non-elementary function).

$y_p = u_1 y_1 + u_2 y_2 = \frac{e^x\ln|x|}{2} - \frac{e^{-x}\,\text{Ei}(2x)}{2}$.

General solution: $y = c_1 e^x + c_2 e^{-x} + \frac{e^x\ln|x|}{2} - \frac{e^{-x}\,\text{Ei}(2x)}{2}$.

**Remark.** The non-elementary $\text{Ei}$ function arising here illustrates that variation of parameters always gives a formula for $y_p$, but the integrals may not be expressible in terms of elementary functions. In such cases, the formula with an unevaluated integral is still the correct answer.

---

## Problem 5: Phase Portrait of a 2x2 Linear System

**Problem.** Classify the equilibrium at the origin for $\mathbf{x}' = \begin{pmatrix}-1&2\\-2&-1\end{pmatrix}\mathbf{x}$ and sketch the phase portrait.

**Solution.** Eigenvalues: $\det(A-\lambda I) = (-1-\lambda)^2 + 4 = \lambda^2 + 2\lambda + 5 = 0$.

$\lambda = (-2 \pm \sqrt{4-20})/2 = -1 \pm 2i$.

Complex eigenvalues with $\alpha = -1 < 0$: stable spiral. All trajectories spiral inward toward the origin.

Direction of rotation: the off-diagonal entry $a_{12} = 2 > 0$ (the $y$ component increases when $x > 0$), so rotation is clockwise.

The general (complex) solution: $\mathbf{x}(t) = e^{-t}[c_1(\cos 2t\,\mathbf{v}_R - \sin 2t\,\mathbf{v}_I) + c_2(\sin 2t\,\mathbf{v}_R + \cos 2t\,\mathbf{v}_I)]$

where $\mathbf{v}_R + i\mathbf{v}_I$ is an eigenvector for $\lambda = -1+2i$.

---

## Problem 6: Lyapunov Stability

**Problem.** Prove that the origin is asymptotically stable for $x' = -x + y^2$, $y' = -y$, using a Lyapunov function.

**Solution.** Try $V(x,y) = x^2 + y^2$.

$\dot{V} = 2x\dot{x} + 2y\dot{y} = 2x(-x+y^2) + 2y(-y) = -2x^2 + 2xy^2 - 2y^2$.

$-2x^2 + 2xy^2 = -2(x-y^2/2)^2 + y^4/2$.

$\dot{V} = -2(x-y^2/2)^2 + y^4/2 - 2y^2$.

For small $(x,y)$: the $-2y^2$ term dominates $y^4/2$, and $-2(x-y^2/2)^2 \leq 0$. More precisely, for $|(x,y)| < 1$: $y^4/2 \leq y^2/2$, so $\dot{V} \leq -2(x-y^2/2)^2 - 3y^2/2$.

This is strictly negative except at $y = 0$ and $x - y^2/2 = 0$, i.e., at the origin. By Lyapunov's theorem (LaSalle invariance or strict Lyapunov with modified $V$), the origin is asymptotically stable.

---

## Problem 7: Series Solution near a Regular Point

**Problem.** Find the first four terms of the power series solution to $y'' + xy' + y = 0$ about $x = 0$ with $y(0) = 1$, $y'(0) = 0$.

**Solution.** Assume $y = \sum_{n=0}^\infty a_n x^n$. Then $y' = \sum na_n x^{n-1}$, $y'' = \sum n(n-1)a_nx^{n-2}$.

Substituting:
$\sum_{n=2}^\infty n(n-1)a_n x^{n-2} + \sum_{n=1}^\infty na_n x^n + \sum_{n=0}^\infty a_n x^n = 0$.

Shift index in first sum ($m = n-2$): $\sum_{m=0}^\infty (m+2)(m+1)a_{m+2}x^m$.

Collect at each power of $x$:

$x^0$: $2a_2 + a_0 = 0 \Rightarrow a_2 = -a_0/2$.
$x^1$: $6a_3 + a_1 + a_1 = 6a_3 + 2a_1 = 0 \Rightarrow a_3 = -a_1/3$.
$x^2$: $12a_4 + 2a_2 + a_2 = 12a_4 + 3a_2 = 0 \Rightarrow a_4 = -a_2/4 = a_0/8$.

With $a_0 = y(0) = 1$, $a_1 = y'(0) = 0$: $a_2 = -1/2$, $a_3 = 0$, $a_4 = 1/8$.

$y = 1 - \frac{x^2}{2} + \frac{x^4}{8} - \cdots$

(The even terms satisfy $a_{2k} = (-1)^k/(2^k k!)$, giving $y = e^{-x^2/2}$ — verifiable by direct substitution.)

---

## Problem 8: Laplace Transform for a Discontinuous Forcing

**Problem.** Solve $y'' + y = f(t)$, $y(0) = y'(0) = 0$, where $f(t) = 1$ for $0 \leq t < \pi$ and $f(t) = 0$ for $t \geq \pi$.

**Solution.** Write $f(t) = 1 - u_\pi(t)$ (1 minus the unit step at $t = \pi$). Laplace transform: $\mathcal{L}\{y'' + y\} = (s^2+1)Y(s) = \mathcal{L}\{f\} = \frac{1}{s} - \frac{e^{-\pi s}}{s}$.

$Y(s) = \frac{1}{s(s^2+1)} - \frac{e^{-\pi s}}{s(s^2+1)}$.

Partial fractions: $\frac{1}{s(s^2+1)} = \frac{1}{s} - \frac{s}{s^2+1}$.

Inverse: $\mathcal{L}^{-1}\{1/s - s/(s^2+1)\} = 1 - \cos t$.

By the second shifting theorem ($\mathcal{L}^{-1}\{e^{-as}F(s)\} = u_a(t)f(t-a)$):

$y(t) = (1 - \cos t) - u_\pi(t)(1 - \cos(t-\pi))$.

For $t \geq \pi$: $1 - \cos(t-\pi) = 1 + \cos t$, so $y = (1-\cos t) - (1+\cos t) = -2\cos t$.

Full solution: $y(t) = \begin{cases}1-\cos t & 0 \leq t < \pi \\ -2\cos t & t \geq \pi.\end{cases}$
