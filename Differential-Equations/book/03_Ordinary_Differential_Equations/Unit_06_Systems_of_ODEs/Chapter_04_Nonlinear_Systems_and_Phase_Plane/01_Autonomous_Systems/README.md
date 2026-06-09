# Autonomous Systems

An autonomous system of differential equations is one in which the independent variable $t$ does not appear explicitly in the defining equations. In two dimensions, the general form is:

$$x' = f(x, y), \qquad y' = g(x, y),$$

where $f$ and $g$ are functions of the state variables $(x,y)$ alone. The term "autonomous" means self-governing: the rate of change of the state depends only on the current state, not on the current time. This time-translation symmetry has profound geometric consequences and is what makes phase plane analysis effective.

## Time Invariance and Phase Curves

The defining geometric property of autonomous systems is that if $(x(t), y(t))$ is a solution, then so is $(x(t + c), y(t + c))$ for any constant $c$. Equivalently, shifting a solution in time produces another solution. This means that through any point $(x_0, y_0)$ in the phase plane, there passes exactly one orbit — the set of points traced by the solution — regardless of when the trajectory passes through that point. Two solutions through the same point may differ in their time parameterization, but they trace the same curve.

This stands in contrast to non-autonomous systems such as $x' = f(x,y,t)$, where the vector field changes as $t$ evolves and trajectories in the $(x,y)$-plane can cross one another (though not in the extended $(x,y,t)$-space). Autonomous systems define a genuine vector field in the $(x,y)$-plane, one that does not change over time.

**Uniqueness of orbits.** Suppose two trajectories $\mathbf{r}_1(t)$ and $\mathbf{r}_2(t)$ pass through the same point $(x_0, y_0)$: say $\mathbf{r}_1(t_1) = (x_0, y_0) = \mathbf{r}_2(t_2)$. Then $\mathbf{r}_1(t - t_1 + t_2) = \mathbf{r}_2(t)$ for all $t$, by the time-translation argument combined with uniqueness of solutions. So $\mathbf{r}_1$ and $\mathbf{r}_2$ trace the same geometric curve. It follows that orbits are either disjoint or identical — they never cross.

## Types of Orbits

Three distinct types of orbits arise in autonomous systems:

An **equilibrium point** (critical point, rest point) is a constant solution $(x(t), y(t)) = (x^*, y^*)$ for all $t$, defined by $f(x^*,y^*) = 0$ and $g(x^*,y^*) = 0$. It is an orbit consisting of a single point.

A **periodic orbit** (closed orbit, limit cycle) is a non-constant solution with $\mathbf{r}(t + T) = \mathbf{r}(t)$ for some minimal period $T > 0$. It is a closed curve in the phase plane traced repeatedly. Periodic orbits represent sustained oscillations.

An **open orbit** is a solution that is neither constant nor periodic. It is an injective curve in the phase plane: the trajectory passes through each of its points exactly once (as $t$ ranges over its maximal interval of existence).

The Poincaré-Bendixson theorem (discussed in a later chapter) characterizes which combinations of these orbit types can occur in bounded regions of the plane, placing strong constraints on the possible long-term behaviors.

## The Phase Plane and Vector Field

The **phase plane** is the $(x,y)$-plane viewed as the space of states of the system. At each point $(x,y)$, the vector $(f(x,y), g(x,y))$ is tangent to the orbit passing through that point. Drawing these vectors at a representative set of points produces the **direction field** (vector field) of the system. Orbits are the integral curves of this vector field — the curves everywhere tangent to the field.

Nullclines are particularly useful for sketching phase portraits. The **$x$-nullcline** is the curve where $f(x,y) = 0$, on which $x' = 0$ (the trajectory moves vertically). The **$y$-nullcline** is the curve where $g(x,y) = 0$, on which $y' = 0$ (the trajectory moves horizontally). Intersections of the two nullclines are equilibrium points. The nullclines divide the plane into regions where the signs of $x'$ and $y'$ are constant, allowing one to determine the direction of motion in each region without solving the equations.

## Reduction to a Single ODE

A useful technique for autonomous systems is to eliminate $t$ and consider $y$ as a function of $x$. By the chain rule, $dy/dx = (dy/dt)/(dx/dt) = g(x,y)/f(x,y)$, valid where $f(x,y) \neq 0$. This single ODE in $(x,y)$ describes the geometry of orbits, decoupled from the time parameterization. Solving this equation (sometimes possible for special forms) gives the orbital shapes directly.

**Example.** The system $x' = y$, $y' = -x$ gives $dy/dx = -x/y$, or $y\,dy = -x\,dx$. Integrating: $y^2/2 = -x^2/2 + C$, so $x^2 + y^2 = 2C$. The orbits are circles centered at the origin — the phase portrait consists of concentric circles, corresponding to uniform circular motion.

## The Energy Integral and Conservation Laws

For systems derived from Hamiltonian or Lagrangian mechanics, a conserved quantity $H(x,y)$ (the Hamiltonian, or energy) exists with the property that $H(x(t), y(t)) = \text{const}$ along any solution. This means orbits lie on level curves of $H$. The equation $H(x,y) = C$ describes a family of orbits parameterized by the constant $C$.

For a mechanical system with kinetic energy $\frac{1}{2}y^2$ and potential $V(x)$, setting $x' = y$ and $y' = -V'(x)$, the conserved energy is $H = \frac{1}{2}y^2 + V(x)$. The phase portrait is the family of level curves of $H$, which can be read off directly from the shape of $V$.

**Example.** The undamped pendulum $x'' + \sin x = 0$ becomes $x' = y$, $y' = -\sin x$ with energy $H = \frac{1}{2}y^2 - \cos x$. Level curves $\frac{1}{2}y^2 - \cos x = C$ give closed orbits (oscillations) for $C < 1$ and open orbits (full rotations) for $C > 1$. The separatrix $C = 1$ connects the saddle points at $x = \pm\pi$, $y = 0$ and separates oscillatory from rotational behavior.

## Limit Sets

As $t \to +\infty$, a trajectory may approach various limiting configurations, called the **$\omega$-limit set** of the initial point. For the $t \to -\infty$ direction, one speaks of the **$\alpha$-limit set**. Possible $\omega$-limit sets in the plane include:

An equilibrium point, in which case the trajectory is asymptotic to that rest point. A periodic orbit, in which case the trajectory spirals toward the closed curve. The orbit itself, if the trajectory is periodic. A more complex configuration of equilibria connected by heteroclinic or homoclinic orbits (such as the separatrix of the pendulum).

The remarkable fact — the content of the Poincaré-Bendixson theorem — is that in the plane, no other types of $\omega$-limit sets can occur for bounded orbits. This rules out chaos in two-dimensional autonomous systems: bounded orbits must eventually settle into equilibria or periodic behavior. Chaos requires at least three dimensions.

## Non-Autonomous Systems and the Comparison

A non-autonomous system $x' = f(x,y,t)$, $y' = g(x,y,t)$ can be made autonomous by introducing $t$ as a third variable with $t' = 1$, yielding a three-dimensional autonomous system. This perspective shows that non-autonomous two-dimensional systems are equivalent to autonomous three-dimensional ones, with all the additional complexity that entails. For the phase plane analysis of the two-dimensional state space alone to be effective, autonomy is essential.

This chapter focuses on autonomous systems throughout. The machinery of phase portraits, linearization, equilibrium classification, and Lyapunov theory is developed specifically for the autonomous case, where the geometry of the phase plane is the primary analytical tool.
