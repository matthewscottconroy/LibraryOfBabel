# Periodic Orbits

Not every orbit settles to a fixed point. Many dynamical systems exhibit persistent oscillation: the state cycles through a finite set of values repeatedly without converging or diverging. These periodic orbits are the next simplest type of long-term behavior after fixed points, and they form the scaffolding around which more complicated dynamics—including chaos—are organized.

## Definitions

Let $f: X \to X$. A point $x_0 \in X$ is **periodic of period $n$** if $f^n(x_0) = x_0$ and $n$ is the smallest positive integer with this property. The set $\{x_0, f(x_0), f^2(x_0), \ldots, f^{n-1}(x_0)\}$ is the **periodic orbit** or **$n$-cycle** through $x_0$. A period-1 point is simply a fixed point.

The period is sometimes called the **minimal period** to emphasize that we require $n$ to be the smallest such integer. Every point in a periodic orbit of period $n$ has the same minimal period.

## Reduction to Fixed Points

The central observation is that periodic points of period $n$ for $f$ are fixed points of the iterated map $f^n$. This reduces all questions about periodic orbits to the fixed point theory already developed, applied to $f^n$ instead of $f$.

In particular, stability of a periodic orbit is determined by $(f^n)'(x_0)$. By the chain rule applied repeatedly,

$$(f^n)'(x_0) = \prod_{k=0}^{n-1} f'(x_k)$$

where $x_k = f^k(x_0)$. This product is called the **multiplier** of the periodic orbit.

**Lemma.** The multiplier is the same for every point in the orbit.

**Proof.** If $x_0, x_1 = f(x_0), \ldots, x_{n-1} = f^{n-1}(x_0)$ are the points in the orbit, then

$$(f^n)'(x_1) = \prod_{k=1}^{n} f'(x_k) = f'(x_n) \cdot \prod_{k=1}^{n-1} f'(x_k) = f'(x_0) \cdot \prod_{k=1}^{n-1} f'(x_k),$$

using $x_n = x_0$. This is a cyclic permutation of the product $(f^n)'(x_0) = \prod_{k=0}^{n-1} f'(x_k)$, so the two are equal. $\square$

The stability criterion is then immediate from the fixed point theorem: a periodic orbit of period $n$ is asymptotically stable if $|\prod_{k=0}^{n-1} f'(x_k)| < 1$ and unstable if this product exceeds 1 in absolute value.

## Period-2 Orbits of the Logistic Map

For the logistic map $f_r(x) = rx(1-x)$, we seek period-2 orbits. These are fixed points of $f_r^2$ that are not fixed points of $f_r$ itself. The fixed points of $f_r^2$ satisfy

$$f_r^2(x) = x.$$

Since every fixed point of $f_r$ is also a fixed point of $f_r^2$, the period-2 points satisfy $(f_r^2(x) - x)/(f_r(x) - x) = 0$. One can show (by direct computation) that the period-2 points come in a conjugate pair

$$p, q = \frac{(r+1) \pm \sqrt{(r+1)(r-3)}}{2r},$$

which are real and distinct when $r > 3$. So period-2 orbits are born exactly at $r = 3$, where the fixed point $x^* = 1 - 1/r$ loses stability.

The multiplier of the period-2 orbit is

$$f_r'(p) \cdot f_r'(q) = (2-r+r/p-r \cdot 2p)(2-r+r/q - r \cdot 2q).$$

A cleaner computation shows this equals $-r^2 + 2r + 4$. This has absolute value less than 1 when $3 < r < 1 + \sqrt{6} \approx 3.449$. At $r = 1 + \sqrt{6}$, the period-2 orbit loses stability and a period-4 orbit is born—the next step of the period-doubling cascade.

## Graphical Analysis of Period-2 Orbits

A period-2 orbit appears in the cobweb diagram as a rectangle: starting from $x_0$, the cobweb path visits $x_1 = f(x_0)$, returns to $x_0 = f(x_1)$, and cycles. In the graph of $f^2$, the period-2 points appear as the two new fixed points that emerge at $r = 3$ as the original fixed point loses stability in a period-doubling bifurcation.

## Sharkovskii's Theorem

One of the most remarkable theorems in one-dimensional dynamics constrains which periods can coexist. Define the **Sharkovskii ordering** on positive integers:

$$3 \succ 5 \succ 7 \succ \cdots \succ 2 \cdot 3 \succ 2 \cdot 5 \succ 2 \cdot 7 \succ \cdots \succ 4 \cdot 3 \succ 4 \cdot 5 \succ \cdots \succ 2^3 \succ 2^2 \succ 2 \succ 1.$$

This is a total ordering of all positive integers, with 3 at the top and the powers of 2 at the bottom in decreasing order.

**Theorem (Sharkovskii, 1964).** Let $f: \mathbb{R} \to \mathbb{R}$ be continuous, and suppose $f$ has a periodic point of period $m$. Then $f$ has a periodic point of period $n$ for every $n$ with $m \succ n$ in the Sharkovskii ordering.

**Corollary.** If $f$ has a period-3 orbit, then $f$ has periodic orbits of every period.

This corollary, popularized by Li and Yorke in their 1975 paper "Period Three Implies Chaos," was one of the first precise mathematical statements connecting periodic behavior to complex dynamics. It demonstrates that the existence of a single period-3 point guarantees a richly structured orbit space.

## Periodic Windows

In the logistic map with $r \in (r_\infty, 4)$ (where $r_\infty \approx 3.5699$ is the accumulation point of period doublings), the dynamics are predominantly chaotic but interspersed with **periodic windows**: parameter intervals where a stable periodic orbit exists. The largest such window is the period-3 window near $r \approx 3.83$. Within each periodic window, one sees a complete period-doubling cascade as the parameter increases through the window, so the structure of the bifurcation diagram is self-similar.

## Higher-Dimensional Periodic Orbits

For maps $f: \mathbb{R}^n \to \mathbb{R}^n$, a periodic orbit of period $k$ is again a set of $k$ points cyclically permuted by $f$, and stability is determined by the eigenvalues of the Jacobian matrix $Df^k(x_0)$. By the chain rule for Jacobians, $Df^k(x_0) = Df(x_{k-1}) \cdots Df(x_1) Df(x_0)$. The orbit is asymptotically stable if all eigenvalues of this matrix product lie inside the unit disk, unstable if any eigenvalue lies outside. The study of the eigenvalues of this matrix—particularly as a parameter varies—is the starting point for the bifurcation theory of periodic orbits.
