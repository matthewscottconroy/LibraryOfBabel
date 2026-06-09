# Work Integrals

Suppose a particle moves through a force field $\mathbf{F}$ along a curved path $C$. At each instant, the component of force in the direction of motion does work on the particle; the component perpendicular to motion does none. To find the total work done, we must integrate this tangential component of $\mathbf{F}$ over the entire path. This leads directly to the vector line integral.

## Physical Motivation

In elementary physics, the work done by a constant force $\mathbf{F}$ moving a particle a displacement $\Delta\mathbf{r}$ is $W = \mathbf{F} \cdot \Delta\mathbf{r} = |\mathbf{F}||\Delta\mathbf{r}|\cos\theta$, where $\theta$ is the angle between them. When the force varies and the path is curved, we subdivide the path into small segments, apply the formula to each segment (where $\mathbf{F}$ and the direction are approximately constant), and sum. Taking the limit gives the vector line integral.

## Definition

Let $\mathbf{F}: D \to \mathbb{R}^n$ be a continuous vector field and $C$ a smooth oriented curve with parametrization $\mathbf{r}: [a,b] \to D$, $\mathbf{r}'(t) \neq \mathbf{0}$. The **work integral** (or **vector line integral**) of $\mathbf{F}$ along $C$ is

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt.$$

Writing $\mathbf{F} = (P, Q, R)$ and $d\mathbf{r} = (dx, dy, dz)$:

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_C P\,dx + Q\,dy + R\,dz.$$

## Relation to Scalar Line Integral

The unit tangent vector to $C$ at $\mathbf{r}(t)$ is $\hat{\mathbf{T}}(t) = \mathbf{r}'(t)/|\mathbf{r}'(t)|$. Therefore:

$$\int_C \mathbf{F}\cdot d\mathbf{r} = \int_a^b \mathbf{F}(\mathbf{r}(t))\cdot\mathbf{r}'(t)\,dt = \int_a^b \mathbf{F}(\mathbf{r}(t))\cdot\hat{\mathbf{T}}(t)\,|\mathbf{r}'(t)|\,dt = \int_C (\mathbf{F}\cdot\hat{\mathbf{T}})\,ds.$$

The vector line integral is the scalar line integral of the tangential component $\mathbf{F}\cdot\hat{\mathbf{T}}$ (the component of $\mathbf{F}$ in the direction of motion). This is the formal expression of the physical idea: only the component along the path contributes to work.

## Properties

**Linearity:** $\int_C (a\mathbf{F} + b\mathbf{G})\cdot d\mathbf{r} = a\int_C\mathbf{F}\cdot d\mathbf{r} + b\int_C\mathbf{G}\cdot d\mathbf{r}$.

**Additivity:** If $C = C_1 \cup C_2$ (joined end to end), then $\int_C\mathbf{F}\cdot d\mathbf{r} = \int_{C_1}\mathbf{F}\cdot d\mathbf{r} + \int_{C_2}\mathbf{F}\cdot d\mathbf{r}$.

**Reversal of orientation:** $\int_{-C}\mathbf{F}\cdot d\mathbf{r} = -\int_C\mathbf{F}\cdot d\mathbf{r}$.

**Independence of (orientation-preserving) reparametrization.** The value of $\int_C\mathbf{F}\cdot d\mathbf{r}$ depends only on the curve $C$ with its orientation, not on the particular parametrization.

## Worked Examples

**Example 1.** Let $\mathbf{F}(x,y) = -y\,\mathbf{i} + x\,\mathbf{j}$ and $C$ the unit circle traversed counterclockwise: $\mathbf{r}(t) = (\cos t, \sin t)$, $t \in [0, 2\pi]$.

$\mathbf{r}'(t) = (-\sin t, \cos t)$.

$\mathbf{F}(\mathbf{r}(t)) = (-\sin t, \cos t)$.

$$\int_C \mathbf{F}\cdot d\mathbf{r} = \int_0^{2\pi} (-\sin t, \cos t)\cdot(-\sin t, \cos t)\,dt = \int_0^{2\pi}(\sin^2 t + \cos^2 t)\,dt = \int_0^{2\pi} 1\,dt = 2\pi.$$

The positive value $2\pi$ confirms that $\mathbf{F}$ circulates in the same direction as the orientation of $C$.

**Example 2.** Let $\mathbf{F}(x,y,z) = (x, y, z)$ and $C$ the line segment from $(1,0,0)$ to $(0,1,1)$.

Parametrize: $\mathbf{r}(t) = (1-t, t, t)$, $t \in [0,1]$, so $\mathbf{r}'(t) = (-1,1,1)$.

$\mathbf{F}(\mathbf{r}(t)) = (1-t, t, t)$.

$$\int_C \mathbf{F}\cdot d\mathbf{r} = \int_0^1 (1-t, t, t)\cdot(-1, 1, 1)\,dt = \int_0^1 (-(1-t) + t + t)\,dt = \int_0^1 (3t - 1)\,dt = \left[\frac{3t^2}{2} - t\right]_0^1 = \frac{1}{2}.$$

**Example 3: Path dependence for a non-conservative field.** Let $\mathbf{F}(x,y) = y\,\mathbf{i}$ (not conservative, since $\partial P/\partial y = 1 \neq 0 = \partial Q/\partial x$). Compute $\int_C\mathbf{F}\cdot d\mathbf{r}$ from $(0,0)$ to $(1,1)$ along two paths.

*Path 1: straight line.* $\mathbf{r}(t) = (t, t)$, $t \in [0,1]$. $\mathbf{r}' = (1,1)$, $\mathbf{F} = (t, 0)$.

$$\int_{C_1} = \int_0^1 (t,0)\cdot(1,1)\,dt = \int_0^1 t\,dt = \frac{1}{2}.$$

*Path 2: parabola $y = x^2$.* $\mathbf{r}(t) = (t, t^2)$, $t \in [0,1]$. $\mathbf{r}' = (1, 2t)$, $\mathbf{F} = (t^2, 0)$.

$$\int_{C_2} = \int_0^1 (t^2, 0)\cdot(1, 2t)\,dt = \int_0^1 t^2\,dt = \frac{1}{3}.$$

The two values ($1/2 \neq 1/3$) confirm that $\mathbf{F}$ is not conservative and that the integral is path-dependent.

## Work against Gravity

Near Earth's surface, gravity exerts the force $\mathbf{F} = -mg\,\mathbf{k}$ on an object of mass $m$. The work done by gravity as the object moves along a path $C$ from height $z_a$ to height $z_b$ is

$$W = \int_C \mathbf{F}\cdot d\mathbf{r} = \int_a^b (-mg)\,z'(t)\,dt = -mg(z_b - z_a) = mg(z_a - z_b).$$

This depends only on the change in height, not on the path — confirming that gravity is conservative and that the work done by gravity in lowering an object from $z_a$ to $z_b$ is $mg(z_a - z_b)$, positive if $z_a > z_b$ (the object descends).

## Alternative Notation

The vector line integral is also written using differential forms:

$$\int_C \mathbf{F}\cdot d\mathbf{r} = \int_C P\,dx + Q\,dy + R\,dz.$$

Here $dx = x'(t)\,dt$, etc., so this is identical to the parametric formula. The differential form notation is concise and will be systematically developed in Unit 4, Chapter 4.

## Summary

The work integral $\int_C\mathbf{F}\cdot d\mathbf{r}$ measures the cumulative dot product of a vector field with the direction of motion along a curve. It equals the scalar line integral of the tangential component of $\mathbf{F}$, computed via the parametric formula $\int_a^b \mathbf{F}(\mathbf{r}(t))\cdot\mathbf{r}'(t)\,dt$. For conservative fields, the result is path-independent and can be evaluated using the potential function. For non-conservative fields, the integral genuinely depends on the path, and direct computation is required.
