# The Exactness Condition

Consider the first-order ODE written in the **differential form**

$$M(x, y)\,dx + N(x, y)\,dy = 0.$$

This is equivalent to $dy/dx = -M(x,y)/N(x,y)$ wherever $N \neq 0$, but the symmetric form in $dx$ and $dy$ is more natural for the exactness analysis. The equation is **exact** if the left side is the total differential of some function $F(x, y)$: that is, if there exists $F$ such that

$$dF = \frac{\partial F}{\partial x}\,dx + \frac{\partial F}{\partial y}\,dy = M\,dx + N\,dy.$$

This requires $\partial F/\partial x = M$ and $\partial F/\partial y = N$.

## The Exactness Criterion

**Theorem.** Suppose $M$ and $N$ have continuous first partial derivatives on a simply connected open region $D$. Then $M\,dx + N\,dy = 0$ is exact on $D$ if and only if

$$\frac{\partial M}{\partial y} = \frac{\partial N}{\partial x}$$

throughout $D$.

**Proof of necessity.** If $F$ exists with $F_x = M$ and $F_y = N$, then, assuming the mixed partials are continuous:

$$\frac{\partial M}{\partial y} = \frac{\partial^2 F}{\partial y\,\partial x} = \frac{\partial^2 F}{\partial x\,\partial y} = \frac{\partial N}{\partial x}.$$

**Proof of sufficiency** (sketch). This is Poincare's lemma for 1-forms. Define $F(x, y) = \int_{x_0}^x M(t, y_0)\,dt + \int_{y_0}^y N(x, s)\,ds$ where $(x_0, y_0)$ is a basepoint. Then $F_y(x,y) = N(x, y)$ directly. Computing $F_x$:

$$F_x = M(x, y_0) + \int_{y_0}^y \frac{\partial N}{\partial x}(x, s)\,ds = M(x, y_0) + \int_{y_0}^y \frac{\partial M}{\partial y}(x, s)\,ds = M(x, y_0) + M(x, y) - M(x, y_0) = M(x, y).$$

The step $\partial N/\partial x = \partial M/\partial y$ uses the hypothesis.

## Worked Examples

**Example 1.** Test for exactness: $(2xy + 3)\,dx + (x^2 + 4y)\,dy = 0$.

$M = 2xy + 3$, $N = x^2 + 4y$. $\partial M/\partial y = 2x$ and $\partial N/\partial x = 2x$. Since $M_y = N_x$, the equation is exact.

**Example 2.** Test for exactness: $(x^2 + y)\,dx + (y^2 + x)\,dy = 0$.

$M = x^2 + y$, $N = y^2 + x$. $M_y = 1$ and $N_x = 1$. Exact.

**Example 3.** Test for exactness: $(xy^2 + y)\,dx + (x^2y)\,dy = 0$.

$M = xy^2 + y$, $N = x^2y$. $M_y = 2xy + 1$ and $N_x = 2xy$. Since $M_y \neq N_x$, the equation is not exact.

## The Importance of Simple Connectivity

The condition $M_y = N_x$ is necessary and sufficient for exactness on a **simply connected** domain (one with no holes). On a non-simply-connected domain (such as the punctured plane $\mathbb{R}^2 \setminus \{(0,0)\}$), a closed form may not be exact. The classical example is

$$\frac{-y\,dx + x\,dy}{x^2 + y^2},$$

for which $M_y = N_x$ everywhere except the origin, but the form is not the differential of any smooth single-valued function on $\mathbb{R}^2 \setminus \{(0,0)\}$ (its integral around the unit circle is $2\pi \neq 0$). Exactness theory must therefore be applied with attention to the domain.

## Connection to Conservative Vector Fields

The exactness condition is identical to the condition for the vector field $\mathbf{F} = (M, N)$ to be conservative (curl-free in 2D). The potential function $F$ is the work function: the solution curves $F(x,y) = C$ are the equipotentials of the field $\mathbf{F}$. This geometric picture connects the algebra of exactness to the physics of conservative forces and the topology of the domain.
