# 37.5 Parabolic Implosion and Near-Parabolic Points

At a parabolic parameter — where a periodic point has a root of unity multiplier — the dynamics undergoes a dramatic change. As the parameter moves away from a parabolic value, the parabolic fixed point splits into nearby fixed points, and the Fatou components reorganize. The Julia set undergoes a "topological discontinuity": its Hausdorff limit as the parameter approaches the parabolic value is strictly larger than the Julia set at the parabolic point.

This is parabolic implosion.

**Definition 37.5.1.** A periodic point $p$ with multiplier $e^{2\pi ip/q}$ is *parabolic*. The dynamics near a parabolic point splits into *attracting* and *repelling* petals.

Near a parabolic point with multiplier $e^{2\pi i p/q}$, the dynamics looks like rotation by $p/q$ plus higher-order terms. There are $q$ attracting petals (Fatou components) and $q$ repelling petals. Points in attracting petals converge to the parabolic point forward in time; points in repelling petals converge backward in time.

**Theorem 37.5.2 (Shishikura, 1987).** The Hausdorff dimension of the Julia set of a quadratic polynomial with a parabolic point is strictly greater than 1 and less than 2.

The Julia set at a parabolic point is "thicker" than a smooth curve (dimension $> 1$) but not so thick as to be a positive-area set (dimension $< 2$). Shishikura's theorem gives a precise range.

**Theorem 37.5.3 (Parabolic Implosion — Douady, Lavaurs).** For the quadratic family $f_c$ near a parabolic parameter $c_0$ (multiplier $e^{2\pi ip/q}$): as $c \to c_0$ through parameter space, the "filled petals" of the parabolic Fatou components collapse, causing the Julia set to undergo a topological discontinuity — the *parabolic implosion*.

More precisely: the Hausdorff limit of $J(f_c)$ as $c \to c_0$ is strictly larger than $J(f_{c_0})$.

Parabolic implosion is what happens at the boundary of a hyperbolic component of the Mandelbrot set. As you approach the boundary from inside the hyperbolic component (where there's an attracting cycle), the attracting cycle collides with a repelling cycle on the boundary (creating a parabolic cycle), and the Julia set "implodes" — the Fatou components fill in, making the Julia set larger.

This phenomenon is important for understanding the fine structure of the Mandelbrot set boundary and for the MLC conjecture: near parabolic parameters, the local connectivity analysis requires understanding how the Julia set changes discontinuously.
