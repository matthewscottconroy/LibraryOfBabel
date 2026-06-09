# Chapter 1: Geometric and Qualitative Methods

A first-order ODE $y' = f(x, y)$ assigns to each point $(x, y)$ in the plane a number $f(x, y)$, which prescribes the slope that any solution passing through that point must have. This assignment is a vector field, and solutions are curves that flow along it. The geometric perspective encoded in this picture allows one to understand the qualitative behavior of solutions, whether they grow, decay, oscillate, or approach equilibrium, without computing a single integral.

## Direction Fields

The direction field (or slope field) of $y' = f(x,y)$ is constructed by drawing a short line segment of slope $f(x,y)$ at each point $(x,y)$ in a grid covering the region of interest. Each segment indicates the direction a solution curve must travel when it passes through that point. Taken together, the segments reveal the flow: where solution curves converge, diverge, or run parallel.

From a direction field one can sketch approximate solution curves by starting at any point and following the direction of the nearby segments. This technique is particularly valuable for autonomous equations $y' = f(y)$, where the slope depends only on $y$ and the direction field is uniform in $x$.

## Phase Lines and Equilibria

For autonomous equations $y' = f(y)$, the equation depends on $y$ alone and is translation-invariant in $x$: if $\phi(x)$ is a solution, so is $\phi(x - c)$. This symmetry collapses the two-dimensional direction field to a one-dimensional picture called the **phase line**, which represents the dynamics entirely in terms of $y$.

The phase line is constructed by identifying the zeros of $f(y)$ (the equilibrium points), determining the sign of $f(y)$ between them, and drawing arrows indicating whether $y$ is increasing or decreasing. Equilibria are the constant solutions $y = y^*$ where $f(y^*) = 0$.

## Stability Analysis

An equilibrium $y^*$ is **stable** (attracting, or asymptotically stable) if solutions starting near $y^*$ tend toward $y^*$ as $x \to +\infty$. It is **unstable** if solutions starting near $y^*$ move away from it. The linearization test: if $f'(y^*) < 0$, then $y^*$ is asymptotically stable; if $f'(y^*) > 0$, it is unstable. When $f'(y^*) = 0$, higher-order analysis is needed.

## Key Theorems

The **isocline method** provides a systematic way to sketch direction fields: a curve $f(x,y) = c$ (an isocline) is the locus of points where all solution slopes equal $c$. By drawing several isoclines and marking their corresponding slopes, one builds the direction field efficiently.

The **monotonicity theorem** for autonomous equations states that every non-constant solution is strictly monotone (since if $y' = f(y) = 0$ at any interior point, the solution is identically constant by uniqueness). This rules out oscillatory behavior in one-dimensional autonomous systems and is the key reason the phase line completely captures the dynamics.
