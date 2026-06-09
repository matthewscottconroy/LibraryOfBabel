# Chapter 1: Iteration and Maps

Every discrete dynamical system begins with a function applied repeatedly to itself. This chapter develops the language and foundational theory needed to analyze such iteration: fixed points, periodic orbits, stability via linearization, and graphical methods for visualizing dynamics. The logistic map serves as the primary example throughout, illustrating each concept in a setting where the geometry and algebra are explicit enough to be fully tractable.

## The Orbit of a Point

Let $f: X \to X$ be a function on a set $X$. Define the iterates $f^0 = \text{id}$, $f^1 = f$, $f^{n+1} = f \circ f^n$. The **orbit** of a point $x_0 \in X$ is the sequence

$$\mathcal{O}(x_0) = \{x_0, f(x_0), f^2(x_0), f^3(x_0), \ldots\}.$$

The orbit records the entire history of the system started at $x_0$. Most of the questions in dynamical systems reduce to understanding the qualitative structure of orbits: do they converge, oscillate, escape to infinity, or wander in an apparently random fashion?

## Fixed Points and Their Stability

A point $x^* \in X$ is a **fixed point** of $f$ if $f(x^*) = x^*$. Fixed points are the simplest possible orbits: the system, once started at $x^*$, remains there forever.

When $X \subset \mathbb{R}$ and $f$ is differentiable, the stability of $x^*$ is governed by the derivative. The key theorem is as follows.

**Theorem (Stability of Fixed Points).** Let $f: \mathbb{R} \to \mathbb{R}$ be $C^1$ and let $x^*$ be a fixed point of $f$.
- If $|f'(x^*)| < 1$, then $x^*$ is **asymptotically stable**: there exists $\delta > 0$ such that $|x_0 - x^*| < \delta$ implies $f^n(x_0) \to x^*$.
- If $|f'(x^*)| > 1$, then $x^*$ is **unstable**: orbits starting near $x^*$ move away from it.
- If $|f'(x^*)| = 1$, the test is inconclusive; higher-order analysis is required.

The proof is an application of the contraction mapping theorem. If $|f'(x^*)| = \lambda < 1$, continuity of $f'$ gives $|f'(x)| \leq (\lambda + \varepsilon)$ for $x$ in some neighborhood of $x^*$, and the mean value theorem then shows that $f$ is a contraction on that neighborhood.

## Periodic Orbits

A point $x_0$ is **periodic of period $n$** if $f^n(x_0) = x_0$ and $n$ is the smallest such positive integer. The orbit $\{x_0, f(x_0), \ldots, f^{n-1}(x_0)\}$ is called a **period-$n$ orbit** or an **$n$-cycle**.

Periodic orbits of period $n$ for $f$ are precisely the fixed points of $f^n$. This observation reduces stability analysis of periodic orbits to the fixed point theory already developed, applied to the iterate $f^n$. By the chain rule,

$$(f^n)'(x_0) = f'(x_{n-1}) \cdot f'(x_{n-2}) \cdots f'(x_1) \cdot f'(x_0),$$

where $x_k = f^k(x_0)$. Notice that this product is the same regardless of which point in the orbit one starts from: the **multiplier** $\lambda = (f^n)'(x_0)$ is an invariant of the orbit itself, not of the particular base point.

**Corollary.** A periodic orbit of period $n$ is asymptotically stable if and only if $|\prod_{k=0}^{n-1} f'(x_k)| < 1$.

## Graphical Analysis: Cobweb Diagrams

For functions $f: [a,b] \to [a,b]$, the cobweb (or staircase) diagram provides a powerful geometric method for understanding orbits. One draws the graph $y = f(x)$ and the diagonal $y = x$ in the same coordinate system. Starting from $(x_0, x_0)$ on the diagonal, one:

1. Draws a vertical line to $(x_0, f(x_0))$ on the graph of $f$.
2. Draws a horizontal line to $(f(x_0), f(x_0))$ on the diagonal.
3. Repeats.

The resulting staircase pattern traces the orbit. Fixed points appear as intersections of the graph with the diagonal. Whether the cobweb spirals toward or away from a fixed point visually encodes the stability condition $|f'(x^*)| < 1$ or $> 1$.

## The Logistic Map: First Look

The logistic map is defined by

$$f_r(x) = rx(1-x), \quad x \in [0,1], \quad r \in [0,4].$$

For $r \leq 1$, every orbit converges to $0$. For $1 < r \leq 3$, there is a unique non-zero fixed point $x^* = 1 - 1/r$ that attracts all orbits starting in $(0,1)$. One computes $f_r'(x^*) = 2 - r$, so the fixed point is asymptotically stable when $|2 - r| < 1$, i.e., $1 < r < 3$. At $r = 3$, the derivative equals $-1$ and stability is lost—the beginning of the period-doubling cascade treated in Chapter 2.

## Key Theorems to Be Developed

The remaining sections of this chapter establish the following:

- **Section 1** (Fixed Points and Stability): A complete treatment of the stability theorem, with higher-order analysis for the borderline case and examples from the logistic map.
- **Section 2** (Periodic Orbits): Period-2 orbits for the logistic map, the general multiplier formula, and an introduction to Sharkovskii's theorem, which constrains which period combinations can coexist.
- **Section 3** (Logistic Map): Global analysis of the logistic map for all $r \in [0,4]$, including the invariant interval, eventual boundedness of orbits, and the transition to chaos at $r = 4$.
