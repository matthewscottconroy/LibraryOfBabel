# Chapter 01: Analytic Functions

The concept of an analytic function is the organizing principle of complex analysis. A function is analytic at a point if it is complex differentiable in an entire neighborhood of that point, and this single condition — which looks like a modest strengthening of complex differentiability at a point — turns out to imply infinite differentiability, representability by convergent power series, and a host of global constraints. This chapter establishes the foundations: the Cauchy-Riemann equations, the relationship between analyticity and harmonicity, and the construction of harmonic conjugates.

## Section 01: The Cauchy-Riemann Equations

The complex derivative of $f$ at $z_0$ is $f'(z_0) = \lim_{h \to 0} \frac{f(z_0 + h) - f(z_0)}{h}$, where $h \in \mathbb{C}$. If this limit exists, taking $h$ along the real axis and along the imaginary axis must yield the same value. Writing $f = u + iv$, this requirement forces:
$$u_x = v_y \qquad \text{and} \qquad u_y = -v_x$$
at $z_0 = (x_0, y_0)$. These are the Cauchy-Riemann equations. They are necessary for complex differentiability; with the additional hypothesis that the partial derivatives are continuous, they are also sufficient.

**Key theorem.** $f = u + iv$ has a complex derivative at $z_0$ if and only if the partial derivatives $u_x, u_y, v_x, v_y$ exist and are continuous in a neighborhood of $z_0$ and satisfy the Cauchy-Riemann equations there. When these conditions hold, $f'(z_0) = u_x(z_0) + iv_x(z_0)$.

## Section 02: Analyticity and Harmonic Functions

**Definition.** $f$ is analytic (or holomorphic) on an open set $D$ if $f$ is complex differentiable at every point of $D$.

If $f = u + iv$ is analytic on $D$, then $u$ and $v$ satisfy Laplace's equation:
$$\Delta u = u_{xx} + u_{yy} = 0, \qquad \Delta v = v_{xx} + v_{yy} = 0.$$
Functions satisfying Laplace's equation are called harmonic. The connection between analytic functions and harmonic functions is profound: it links complex analysis directly to the theory of elliptic PDEs and to physical applications including electrostatics, gravitational potential theory, and steady-state heat conduction.

The real and imaginary parts of an analytic function are always harmonic. Conversely, given a harmonic function $u$ on a simply connected domain, one can always find a harmonic conjugate $v$ such that $u + iv$ is analytic.

## Section 03: Harmonic Conjugates

**Definition.** Given a harmonic function $u$ on a domain $D$, a harmonic conjugate of $u$ is a harmonic function $v$ on $D$ such that $u + iv$ is analytic on $D$.

On a simply connected domain, harmonic conjugates always exist and are unique up to an additive constant. They are found by integrating the Cauchy-Riemann equations: $v_x = -u_y$ and $v_y = u_x$, so $v$ is determined by a line integral of the exact 1-form $-u_y\, dx + u_x\, dy$.

**Key theorem.** If $u$ is harmonic on a simply connected domain $D$, then $u$ has a harmonic conjugate on $D$.

This theorem fails on non-simply connected domains: for example, $u = \ln\sqrt{x^2 + y^2}$ is harmonic on $\mathbb{C} \setminus \{0\}$, but its harmonic conjugate would be $\arctan(y/x)$, which cannot be made continuous and single-valued on the punctured plane.

## Preview of Key Theorems

The theory developed in this chapter sets the stage for the integration theorems of Unit 03. Cauchy's theorem — that the integral of an analytic function around a closed curve in a simply connected domain is zero — relies on analyticity in an essential way. The mean value property of harmonic functions (which is a consequence of Cauchy's integral formula) and the maximum principle (harmonic functions on a domain attain their maximum on the boundary) are among the most important tools in PDE theory, and they flow directly from the complex analysis developed here.
