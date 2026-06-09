# Conservative Fields

Not all vector fields are created equal. Among all the vector fields one could write down, there is a distinguished class — the **conservative fields** — that behave far more simply under integration. The work done by a conservative field as a particle moves from point $A$ to point $B$ is the same regardless of which path the particle takes. This path independence is not merely a computational convenience; it reflects a deep structural property of the field and has profound physical consequences. In mechanics, conservation of energy is precisely the statement that gravitational and electrostatic forces are conservative.

## Definition

A vector field $\mathbf{F}: D \to \mathbb{R}^n$ is called **conservative** on $D$ if there exists a scalar function $f: D \to \mathbb{R}$ such that

$$\mathbf{F} = \nabla f.$$

The function $f$ is called a **potential function** (or scalar potential) for $\mathbf{F}$. In physics, the convention is often to define the potential with a minus sign, writing $\mathbf{F} = -\nabla V$, so that potential energy $V$ decreases in the direction of force. We use the mathematicians' convention $\mathbf{F} = \nabla f$ unless stated otherwise.

## Why Conservative Fields Are Special

The name "conservative" comes from energy conservation. If $\mathbf{F}$ is a force field and $f$ is its potential, then the work done by $\mathbf{F}$ moving a particle from $\mathbf{r}(a)$ to $\mathbf{r}(b)$ along any path $\mathbf{r}(t)$ is

$$W = \int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt = \int_a^b \nabla f(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt = f(\mathbf{r}(b)) - f(\mathbf{r}(a)).$$

The last equality uses the chain rule: $\frac{d}{dt}f(\mathbf{r}(t)) = \nabla f(\mathbf{r}(t)) \cdot \mathbf{r}'(t)$. The work depends only on the values of $f$ at the endpoints, not on the path. This is the Fundamental Theorem for Line Integrals, which will be stated and proved fully in Unit 2, Chapter 3; the key insight here is that it is an immediate consequence of conservativity.

In particular, if the particle traverses a closed loop — returning to its starting point — then $W = f(\mathbf{r}(a)) - f(\mathbf{r}(a)) = 0$. A conservative field does no net work on any closed path.

## The Curl Test

How can we determine whether a given vector field is conservative without finding a potential function? In two dimensions, suppose $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ is conservative, so $P = \partial f/\partial x$ and $Q = \partial f/\partial y$. If $f$ is $C^2$ (twice continuously differentiable), then mixed partial derivatives commute:

$$\frac{\partial P}{\partial y} = \frac{\partial^2 f}{\partial y\,\partial x} = \frac{\partial^2 f}{\partial x\,\partial y} = \frac{\partial Q}{\partial x}.$$

The condition $\partial P/\partial y = \partial Q/\partial x$ is therefore a **necessary** condition for $\mathbf{F}$ to be conservative (on a $C^2$ potential). This is called the **exactness condition**, and a field satisfying it is called **irrotational** (because the $z$-component of its curl is zero).

In three dimensions, the analogous conditions are $\partial P/\partial y = \partial Q/\partial x$, $\partial Q/\partial z = \partial R/\partial y$, and $\partial P/\partial z = \partial R/\partial x$ — collectively, $\nabla \times \mathbf{F} = \mathbf{0}$.

**Theorem (Necessary Condition).** If $\mathbf{F}$ is conservative and $C^1$ on $D$, then $\nabla \times \mathbf{F} = \mathbf{0}$ on $D$.

The converse — that $\nabla \times \mathbf{F} = \mathbf{0}$ implies $\mathbf{F}$ is conservative — is true only under a topological restriction on the domain. It fails on domains with "holes."

## A Counterexample: The Vortex Field

The field

$$\mathbf{F}(x, y) = \frac{-y}{x^2 + y^2}\,\mathbf{i} + \frac{x}{x^2 + y^2}\,\mathbf{j}$$

is defined on $D = \mathbb{R}^2 \setminus \{(0,0)\}$. A direct computation shows

$$\frac{\partial}{\partial y}\left(\frac{-y}{x^2+y^2}\right) = \frac{y^2 - x^2}{(x^2+y^2)^2}, \quad \frac{\partial}{\partial x}\left(\frac{x}{x^2+y^2}\right) = \frac{y^2 - x^2}{(x^2+y^2)^2},$$

so $\partial P/\partial y = \partial Q/\partial x$ everywhere on $D$. The curl test passes. Yet $\mathbf{F}$ is not conservative on $D$: computing the line integral of $\mathbf{F}$ around the unit circle (parametrized by $\mathbf{r}(t) = (\cos t, \sin t)$, $t \in [0, 2\pi]$) gives

$$\oint \mathbf{F} \cdot d\mathbf{r} = \int_0^{2\pi} \left(\frac{-\sin t}{\cos^2 t + \sin^2 t}\cdot (-\sin t) + \frac{\cos t}{\cos^2 t + \sin^2 t}\cdot\cos t\right)dt = \int_0^{2\pi} 1\,dt = 2\pi \neq 0.$$

A nonzero closed-loop integral means the field cannot be conservative. The resolution of this apparent contradiction is topological: the domain $D = \mathbb{R}^2 \setminus \{(0,0)\}$ has a hole (the missing origin), and on such domains the curl test is necessary but not sufficient. On simply connected domains (domains without holes), the two conditions are equivalent.

## Simply Connected Domains and the Full Characterization

A domain $D \subseteq \mathbb{R}^2$ is **simply connected** if it is connected and every closed loop in $D$ can be continuously contracted to a point without leaving $D$. Intuitively, it has no holes. Disks, rectangles, and the entire plane are simply connected. Annuli and punctured planes are not.

**Theorem (Characterization of Conservative Fields).** Let $D \subseteq \mathbb{R}^2$ be an open simply connected domain, and let $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ be $C^1$ on $D$. The following are equivalent:
1. $\mathbf{F}$ is conservative on $D$ (i.e., $\mathbf{F} = \nabla f$ for some $f \in C^2(D)$).
2. $\partial P/\partial y = \partial Q/\partial x$ on $D$.
3. $\oint_C \mathbf{F} \cdot d\mathbf{r} = 0$ for every closed curve $C$ in $D$.
4. The line integral $\int_C \mathbf{F} \cdot d\mathbf{r}$ is path-independent in $D$.

The proof of the equivalence $(2) \Rightarrow (1)$ on simply connected domains uses Green's Theorem (from Unit 4) and constitutes one of the non-trivial results of the module. The implications $(1) \Rightarrow (4) \Rightarrow (3) \Rightarrow (2)$ are all accessible now.

## Physical Examples

**Gravity.** Near Earth's surface, the gravitational field is $\mathbf{F} = -g\,\mathbf{k}$ (pointing downward). A potential is $f(x,y,z) = -gz$, so $\mathbf{F} = \nabla(-gz) = -g\mathbf{k}$. Gravity is conservative, confirming that the work done in lifting an object depends only on vertical displacement, not on the path.

**Inverse-square field.** The gravitational field of a point mass, $\mathbf{F} = -GM\mathbf{r}/|\mathbf{r}|^3$, has potential $f = GM/|\mathbf{r}|$. Orbital mechanics is possible — and energy is conserved — because gravity is conservative.

**Non-conservative field.** Friction forces are never conservative. A box dragged across a rough table takes more energy if dragged in a longer path, confirming that friction cannot be the gradient of any potential.

## Summary

Conservative vector fields are those that arise as gradients of scalar potential functions. They do zero work around any closed loop, and the work done between any two points depends only on the endpoints. The curl test ($\nabla \times \mathbf{F} = \mathbf{0}$) provides a computable necessary condition, and on simply connected domains it is also sufficient. The vortex field demonstrates that the topology of the domain — specifically the presence of holes — can prevent a curl-free field from being conservative. This interplay between analysis and topology will deepen when we study the Fundamental Theorem for Line Integrals and simply connected regions in Unit 2.
