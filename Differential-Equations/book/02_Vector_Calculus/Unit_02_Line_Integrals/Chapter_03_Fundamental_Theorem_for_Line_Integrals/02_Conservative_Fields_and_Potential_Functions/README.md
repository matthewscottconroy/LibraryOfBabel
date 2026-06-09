# Conservative Fields and Potential Functions

The relationship between conservative fields and potential functions is one of the central themes of vector calculus. We have now accumulated enough machinery to treat it comprehensively. A conservative field $\mathbf{F} = \nabla f$ has a potential function $f$ that encodes all the information about line integrals of $\mathbf{F}$: the integral from $A$ to $B$ is simply $f(B) - f(A)$, regardless of path. In this section we unify the earlier discussion of potential functions with the theory of path independence, provide an integral formula for constructing potentials directly from the field, and work through the theory in both two and three dimensions.

## The Gradient Theorem as a Reconstruction Formula

The Fundamental Theorem for Line Integrals says $\int_C\nabla f\cdot d\mathbf{r} = f(B) - f(A)$. Reading this as a formula for $f$: if we fix a base point $\mathbf{p}_0$ and define

$$f(\mathbf{p}) = f(\mathbf{p}_0) + \int_{\mathbf{p}_0}^{\mathbf{p}} \mathbf{F}\cdot d\mathbf{r},$$

where the integral is along any convenient path from $\mathbf{p}_0$ to $\mathbf{p}$ (the path independence of $\mathbf{F}$ guarantees this is well-defined), then $f$ is a potential function for $\mathbf{F}$.

This provides an alternative method for finding potentials. Instead of the component-by-component integration of Section 4 of Chapter 1, we integrate $\mathbf{F}$ along a convenient path (say, a straight-line path or a step path aligned with coordinate axes).

**Example.** Let $\mathbf{F}(x,y) = (2xy)\,\mathbf{i} + (x^2 + 3y^2)\,\mathbf{j}$. Find $f$ using the line integral method with base point $(0,0)$.

Choose the path $C$ from $(0,0)$ to $(x_0, y_0)$: first horizontal from $(0,0)$ to $(x_0, 0)$, then vertical from $(x_0, 0)$ to $(x_0, y_0)$.

*Horizontal segment $C_1$*: $\mathbf{r}(t) = (t, 0)$, $t \in [0, x_0]$, $d\mathbf{r} = (dt, 0)$.

$\mathbf{F}(t, 0) = (0, t^2)$. $\mathbf{F}\cdot d\mathbf{r} = 0\,dt$. $\int_{C_1}\mathbf{F}\cdot d\mathbf{r} = 0$.

*Vertical segment $C_2$*: $\mathbf{r}(t) = (x_0, t)$, $t \in [0, y_0]$, $d\mathbf{r} = (0, dt)$.

$\mathbf{F}(x_0, t) = (2x_0 t, x_0^2 + 3t^2)$. $\mathbf{F}\cdot d\mathbf{r} = (x_0^2 + 3t^2)\,dt$.

$\int_{C_2}\mathbf{F}\cdot d\mathbf{r} = \int_0^{y_0}(x_0^2 + 3t^2)\,dt = x_0^2 y_0 + y_0^3$.

Therefore $f(x_0, y_0) = 0 + x_0^2 y_0 + y_0^3 = x_0^2 y_0 + y_0^3$. Verify: $\nabla f = (2xy, x^2 + 3y^2) = \mathbf{F}$. Correct.

## Criteria for Conservativity (Summary)

Let $\mathbf{F}$ be a $C^1$ vector field on a connected open domain $D$. The following are equivalent on simply connected $D$:

1. $\mathbf{F} = \nabla f$ for some $f: D \to \mathbb{R}$.
2. $\nabla\times\mathbf{F} = \mathbf{0}$ on $D$.
3. $\int_C\mathbf{F}\cdot d\mathbf{r}$ is path-independent on $D$.
4. $\oint_C\mathbf{F}\cdot d\mathbf{r} = 0$ for every closed curve in $D$.

On non-simply-connected domains, $(1) \Rightarrow (2), (3), (4)$, and $(3) \Leftrightarrow (4)$, but $(2)$ does not imply the rest.

## The Role of the Potential in Physics

In Newtonian mechanics, if $\mathbf{F}$ is a conservative force with potential $f$ (so $\mathbf{F} = \nabla f$), then the quantity $V = -f$ is the **potential energy**. The work-energy theorem states that the work done by $\mathbf{F}$ equals the change in kinetic energy:

$$W = \int_C\mathbf{F}\cdot d\mathbf{r} = f(B) - f(A) = -V(B) + V(A).$$

Rearranging: $(K + V)\big|_B = (K + V)\big|_A$, where $K$ is kinetic energy. The total mechanical energy $K + V$ is conserved along any trajectory — this is the law of conservation of energy, and it follows directly from the conservative nature of the force.

In electrostatics, the electric field $\mathbf{E}$ is conservative in static situations: $\mathbf{E} = -\nabla V$, where $V$ is the electric potential (voltage). Moving a charge $q$ from $A$ to $B$ requires work $q(V(B) - V(A)) = q\Delta V$, purely a function of endpoints.

## Uniqueness of the Potential

The potential function is unique up to an additive constant. If $f_1$ and $f_2$ both satisfy $\nabla f_i = \mathbf{F}$ on a connected domain, then $\nabla(f_1 - f_2) = \mathbf{0}$, so $f_1 - f_2$ is constant. The choice of additive constant (equivalently, the choice of base point $\mathbf{p}_0$ and the convention $f(\mathbf{p}_0) = 0$) is a gauge choice with no physical consequences for differences.

## Example: Three-Dimensional Conservative Field

Let $\mathbf{F}(x,y,z) = (2xz + y)\,\mathbf{i} + (x - 2y)\,\mathbf{j} + (x^2 + 3z^2)\,\mathbf{k}$.

**Step 1: Verify conservativity.**

$\partial P/\partial y = 1 = \partial Q/\partial x$. $\partial P/\partial z = 2x = \partial R/\partial x$. $\partial Q/\partial z = 0 = \partial R/\partial y$. All three symmetry conditions hold. $\mathbf{F}$ is conservative on $\mathbb{R}^3$.

**Step 2: Find potential by integration.**

$\partial f/\partial x = 2xz + y \Rightarrow f = x^2 z + xy + g(y,z)$.

$\partial f/\partial y = x + \partial g/\partial y = x - 2y \Rightarrow \partial g/\partial y = -2y \Rightarrow g = -y^2 + h(z)$.

$\partial f/\partial z = x^2 + h'(z) = x^2 + 3z^2 \Rightarrow h'(z) = 3z^2 \Rightarrow h = z^3 + C$.

**Result:** $f(x,y,z) = x^2 z + xy - y^2 + z^3 + C$.

**Step 3: Compute a line integral.** The work done by $\mathbf{F}$ moving from $(0,0,0)$ to $(1,2,1)$:

$\int_C\mathbf{F}\cdot d\mathbf{r} = f(1,2,1) - f(0,0,0) = (1 + 2 - 4 + 1) - 0 = 0$.

## Summary

A conservative field $\mathbf{F} = \nabla f$ has a potential function that makes all line integrals trivial to compute. The potential can be found by component integration or by integrating $\mathbf{F}$ along a convenient path from a base point. The conditions for conservativity — zero curl, path independence, vanishing circulation — are all equivalent on simply connected domains. The potential function is the multivariable analogue of the antiderivative: it makes the "Fundamental Theorem" work in higher dimensions.
