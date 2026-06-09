# Path Independence

The work done by gravity on a falling stone depends only on how far the stone falls — not on whether it falls straight down, swings on a pendulum, or slides down a curved ramp. This path independence of work is not a special property of gravity, but of any conservative force. Making this notion precise requires careful definition, and understanding it fully requires proving that path independence, conservativity, and vanishing circulation are exactly equivalent.

## Definition

Let $\mathbf{F}: D \to \mathbb{R}^n$ be a continuous vector field on a connected open domain $D$. We say $\mathbf{F}$ is **path-independent** (or that the line integral $\int_C\mathbf{F}\cdot d\mathbf{r}$ is path-independent) on $D$ if: for every pair of points $A, B \in D$ and every two piecewise smooth curves $C_1, C_2$ in $D$ from $A$ to $B$,

$$\int_{C_1}\mathbf{F}\cdot d\mathbf{r} = \int_{C_2}\mathbf{F}\cdot d\mathbf{r}.$$

This means the value of the line integral depends only on the start and end points, not on the path connecting them.

## Equivalence Theorem

On a connected open domain $D$, the following are equivalent:

1. $\mathbf{F}$ is conservative ($\mathbf{F} = \nabla f$ for some $f \in C^1(D)$).
2. $\int_C\mathbf{F}\cdot d\mathbf{r}$ is path-independent on $D$.
3. $\oint_C\mathbf{F}\cdot d\mathbf{r} = 0$ for every closed piecewise smooth curve $C$ in $D$.

**Proof of $(1) \Rightarrow (2)$:** If $\mathbf{F} = \nabla f$ and $\mathbf{r}:[a,b]\to D$ parametrizes any curve from $A$ to $B$, then by the chain rule $\frac{d}{dt}f(\mathbf{r}(t)) = \nabla f(\mathbf{r}(t))\cdot\mathbf{r}'(t) = \mathbf{F}(\mathbf{r}(t))\cdot\mathbf{r}'(t)$. Integrating: $\int_a^b\mathbf{F}(\mathbf{r}(t))\cdot\mathbf{r}'(t)\,dt = f(\mathbf{r}(b)) - f(\mathbf{r}(a)) = f(B)-f(A)$.

**Proof of $(2) \Rightarrow (3)$:** For any closed curve $C$ from $A$ back to $A$, path independence gives $\oint_C\mathbf{F}\cdot d\mathbf{r} = f(A) - f(A) = 0$ (using the argument from (1), or: any closed curve can be viewed as two different paths from $A$ to $A$, so their integrals must be equal, but they differ only in sign by reversal, hence each is zero).

More carefully: if $\oint_C\mathbf{F}\cdot d\mathbf{r}$ depends only on endpoints (both = $A$) then it must be 0.

**Proof of $(3) \Rightarrow (2)$:** If $\int_{C_1}\mathbf{F}\cdot d\mathbf{r} \neq \int_{C_2}\mathbf{F}\cdot d\mathbf{r}$ for two paths from $A$ to $B$, form the closed curve $C = C_1 \cup (-C_2)$ (travel $C_1$ forward then $C_2$ backward). Then $\oint_C\mathbf{F}\cdot d\mathbf{r} = \int_{C_1}\mathbf{F}\cdot d\mathbf{r} - \int_{C_2}\mathbf{F}\cdot d\mathbf{r} \neq 0$, contradicting (3).

**Proof of $(2) \Rightarrow (1)$:** Fix a base point $\mathbf{p}_0 \in D$. For any $\mathbf{p} \in D$, define $f(\mathbf{p}) = \int_C\mathbf{F}\cdot d\mathbf{r}$ along any path $C$ in $D$ from $\mathbf{p}_0$ to $\mathbf{p}$ (this is well-defined by path independence). Then $f$ is $C^1$ and $\nabla f = \mathbf{F}$. (Proving this requires checking that the partial derivative $\partial f/\partial x_i = F_i$, which follows by integrating along a short segment parallel to $\mathbf{e}_i$.)

## Checking Path Independence in Practice

To determine whether $\mathbf{F}$ is path-independent on a given domain:

1. **Compute $\nabla\times\mathbf{F}$.** If it is nonzero, $\mathbf{F}$ is definitely not conservative and integrals are path-dependent.
2. **If $\nabla\times\mathbf{F} = \mathbf{0}$, check the domain.** If $D$ is simply connected, $\mathbf{F}$ is conservative and path-independent.
3. **If $D$ is not simply connected**, the curl test is not sufficient. Compute circulation around loops that encircle the "holes" of $D$. If all such circulations are zero, $\mathbf{F}$ may still be conservative; if any is nonzero, it is not.

## Worked Examples

**Example 1.** Is $\mathbf{F}(x,y) = (2xy)\,\mathbf{i} + (x^2 + 3y^2)\,\mathbf{j}$ path-independent on $\mathbb{R}^2$?

$\partial P/\partial y = 2x$, $\partial Q/\partial x = 2x$. Equal, and $\mathbb{R}^2$ is simply connected, so yes. Potential: $f = x^2 y + y^3$.

**Example 2.** Is $\mathbf{F}(x,y) = y\,\mathbf{i}$ path-independent?

$\partial P/\partial y = 1$, $\partial Q/\partial x = 0$. Not equal; $\mathbf{F}$ is not conservative. Path-dependent.

**Example 3.** Compute $\int_C \mathbf{F}\cdot d\mathbf{r}$ for $\mathbf{F} = (2xy)\,\mathbf{i} + (x^2+3y^2)\,\mathbf{j}$ from $(0,0)$ to $(1,2)$ along the curve $y = 2x^2$.

Since $\mathbf{F}$ is conservative with potential $f = x^2 y + y^3$:

$$\int_C\mathbf{F}\cdot d\mathbf{r} = f(1,2) - f(0,0) = (1\cdot 2 + 8) - 0 = 10.$$

No integration along the curve is needed.

## Using Path Independence to Simplify Computation

Even when we know a field is conservative and want the integral, we can choose the most convenient path. For $\int_C\mathbf{F}\cdot d\mathbf{r}$ with $\mathbf{F}$ path-independent from $A=(1,0)$ to $B=(0,1)$, any path works. The two-segment path $A\to(0,0)\to B$ (horizontal then vertical) may be the easiest.

This flexibility is one of the main computational advantages of working with conservative fields.

## Summary

Path independence means the line integral $\int_C\mathbf{F}\cdot d\mathbf{r}$ depends only on the endpoints, not the path. It is equivalent to conservativity of $\mathbf{F}$ and to vanishing circulation on every closed loop. These equivalences hold on any connected open domain; on simply connected domains, they are also equivalent to the vanishing of the curl. Path independence transforms line integral computation from a parametric calculation into an endpoint evaluation — the multivariable analogue of the one-variable Fundamental Theorem of Calculus.
