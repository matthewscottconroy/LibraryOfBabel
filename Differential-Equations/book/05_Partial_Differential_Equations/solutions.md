# Solutions: Partial Differential Equations

## Problem 1: Method of Characteristics, Linear Case

**Problem.** Solve $u_x + 2u_y = 0$, $u(x,0) = \sin(x)$.

**Solution.** Characteristics: $dx/ds = 1$, $dy/ds = 2$. So $x = x_0 + s$, $y = 2s$. Thus $s = y/2$ and $x_0 = x - y/2$.

Along each characteristic, $du/ds = 0$, so $u$ is constant: $u = u(x_0, 0) = \sin(x_0) = \sin(x - y/2)$.

**Verification:** $u_x = \cos(x-y/2)$, $u_y = -\frac{1}{2}\cos(x-y/2)$. $u_x + 2u_y = \cos - \cos = 0$. Correct. $u(x,0) = \sin(x - 0) = \sin x$. Correct.

**Geometric interpretation.** The solution is constant along lines $x - y/2 = c$ (the characteristics). The initial data on the $x$-axis is propagated along these lines.

---

## Problem 2: Shock Formation

**Problem.** For Burgers' equation $u_t + uu_x = 0$ with $u(x,0) = f(x) = 1/(1+x^2)$, find when and where the first shock forms.

**Solution.** Characteristics: $x(t) = x_0 + f(x_0)t = x_0 + t/(1+x_0^2)$.

Two characteristics starting at $x_0$ and $x_0 + h$ cross when $x(t;x_0) = x(t;x_0+h)$ for small $h > 0$:

$x_0 + t f(x_0) = x_0 + h + t f(x_0+h)$, i.e., $tf(x_0) = h + tf(x_0+h)$. In the limit: $t f'(x_0) = -1$, i.e., $t = -1/f'(x_0)$ (valid when $f'(x_0) < 0$).

$f'(x) = -2x/(1+x^2)^2$. This is negative for $x > 0$, with minimum at $f''(x_0) = 0$:
$f''(x) = -2(1+x^2)^2 + 2x\cdot 2(1+x^2)(2x) = -2(1-3x^2)/(1+x^2)^3 = 0 \Rightarrow x_0 = 1/\sqrt{3}$.

$f'(1/\sqrt{3}) = -2/\sqrt{3}\cdot(4/3)^{-2} = -2/\sqrt{3}\cdot 9/16 = -9/(8\sqrt{3}) = -3\sqrt{3}/8$.

Shock forms at $t_c = 8/(3\sqrt{3}) \approx 1.54$ and $x_c = 1/\sqrt{3} + t_c\cdot f(1/\sqrt{3}) = 1/\sqrt{3} + (8/3\sqrt{3})(3/4) = 1/\sqrt{3} + 2/\sqrt{3} = 3/\sqrt{3} = \sqrt{3}$.

---

## Problem 3: Separation of Variables with Inhomogeneous IC

**Problem.** Solve $u_t = u_{xx}$ for $0 < x < \pi$, $t > 0$, with $u(0,t) = u(\pi,t) = 0$ and $u(x,0) = x(\pi-x)$.

**Solution.** Eigenfunction expansion: $u = \sum b_n\sin(nx)e^{-n^2 t}$.

Compute $b_n$ from initial condition:
$b_n = \frac{2}{\pi}\int_0^\pi x(\pi-x)\sin(nx)\,dx$.

Integrate by parts twice: $\int_0^\pi x(\pi-x)\sin(nx)\,dx$.

Let $I = \int_0^\pi(\pi x - x^2)\sin(nx)\,dx$. Integration by parts (twice):
$I = [-(\pi x-x^2)\cos(nx)/n]_0^\pi + \frac{1}{n}\int_0^\pi(\pi-2x)\cos(nx)\,dx$
$= 0 + \frac{1}{n}\left[(\pi-2x)\frac{\sin(nx)}{n}\right]_0^\pi + \frac{2}{n^2}\int_0^\pi\sin(nx)\,dx$
$= 0 + \frac{2}{n^2}\cdot[-\cos(nx)/n]_0^\pi = \frac{2}{n^3}(1-(-1)^n) = \frac{2}{n^3}\cdot\begin{cases}2 & n \text{ odd}\\0 & n \text{ even}\end{cases}$.

So $b_n = \frac{2}{\pi}\cdot\frac{4}{n^3}$ for $n$ odd, $b_n = 0$ for $n$ even.

$u(x,t) = \frac{8}{\pi}\sum_{k=0}^\infty\frac{\sin((2k+1)x)}{(2k+1)^3}e^{-(2k+1)^2 t}$.

---

## Problem 4: d'Alembert's Formula

**Problem.** Use d'Alembert's formula to solve $u_{tt} = 4u_{xx}$ on $\mathbb{R}$ with $u(x,0) = \cos x$ and $u_t(x,0) = \sin x$.

**Solution.** Here $c = 2$. d'Alembert's formula:
$$u(x,t) = \frac{\cos(x-2t)+\cos(x+2t)}{2} + \frac{1}{2\cdot 2}\int_{x-2t}^{x+2t}\sin s\,ds.$$

$\frac{\cos(x-2t)+\cos(x+2t)}{2} = \frac{2\cos x\cos 2t}{2} = \cos x\cos 2t$ (product-to-sum identity).

$\frac{1}{4}\int_{x-2t}^{x+2t}\sin s\,ds = \frac{1}{4}[-\cos s]_{x-2t}^{x+2t} = \frac{1}{4}(-\cos(x+2t)+\cos(x-2t))$.

$= \frac{1}{4}(2\sin x\sin 2t\cdot(-2))$... let me be careful:

$-\cos(x+2t)+\cos(x-2t) = -[\cos x\cos 2t - \sin x\sin 2t] + [\cos x\cos 2t + \sin x\sin 2t] = 2\sin x\sin 2t$.

So: $u(x,t) = \cos x\cos 2t + \frac{1}{4}\cdot 2\sin x\sin 2t = \cos x\cos 2t + \frac{\sin x\sin 2t}{2}$.

**Verification:** $u(x,0) = \cos x$. $u_t(x,0) = [-2\cos x\sin 2t + \cos x\cos 2t]_{t=0}$... compute $u_t = -2\cos x\sin 2t + \sin x\cos 2t$. At $t=0$: $u_t(x,0) = \sin x$. Correct.

---

## Problem 5: Maximum Principle and Uniqueness

**Problem.** Prove that the solution to $u_t - u_{xx} = 0$ on $[0,1]\times[0,T]$ with $u(0,t) = u(1,t) = 0$ and $u(x,0) = f(x) \geq 0$ satisfies $u(x,t) \geq 0$ for all $t$.

**Solution.** By the maximum principle for the heat equation: the maximum of $u$ on the closed rectangle $[0,1]\times[0,T]$ is attained on the parabolic boundary: $t = 0$, $x = 0$, or $x = 1$.

The minimum principle (apply the maximum principle to $-u$): the minimum is also attained on the parabolic boundary.

On the parabolic boundary: $u(0,t) = 0$, $u(1,t) = 0$, $u(x,0) = f(x) \geq 0$.

So the minimum of $u$ on the parabolic boundary is $0$ (attained on the sides), and hence the minimum on the entire rectangle is $\geq 0$.

Therefore $u(x,t) \geq 0$ for all $(x,t) \in [0,1]\times[0,T]$.

**Physical interpretation:** If the rod starts nonnegative and has zero temperature at its ends, heat cannot create negative temperatures anywhere.

---

## Problem 6: Poisson Integral and Harmonic Functions

**Problem.** Find a harmonic function $u$ in the upper half-plane $y > 0$ satisfying $u(x,0) = \begin{cases}1 & |x| \leq 1 \\ 0 & |x| > 1\end{cases}$.

**Solution.** Poisson integral formula for the upper half-plane: $u(x,y) = \frac{y}{\pi}\int_{-\infty}^\infty\frac{u(t,0)}{(x-t)^2+y^2}\,dt$.

$u(x,y) = \frac{y}{\pi}\int_{-1}^1\frac{dt}{(x-t)^2+y^2}$.

Let $s = (x-t)/y$, $dt = -y\,ds$. When $t = -1$: $s = (x+1)/y$; when $t = 1$: $s = (x-1)/y$.

$u = \frac{y}{\pi}\int_{(x-1)/y}^{(x+1)/y}\frac{y\,ds}{y^2(s^2+1)} = \frac{1}{\pi}\int_{(x-1)/y}^{(x+1)/y}\frac{ds}{s^2+1} = \frac{1}{\pi}\left[\arctan s\right]_{(x-1)/y}^{(x+1)/y}$.

$u(x,y) = \frac{1}{\pi}\left(\arctan\frac{x+1}{y} - \arctan\frac{x-1}{y}\right)$.

This equals $\frac{1}{\pi}\cdot$(the angle subtended by $[-1,1]$ as seen from $(x,y)$), an intuitive result: harmonic measure is the angular measure.
