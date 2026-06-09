# Chapter 2: Differential Operators

Differentiation in single-variable calculus produces a single number — the derivative — from a function of one variable. In the setting of vector fields and multivariable functions, differentiation can produce qualitatively different types of outputs depending on what is being differentiated and in what sense. The three fundamental differential operators of vector calculus — the gradient, divergence, and curl — each capture a distinct aspect of how a field varies, and together they provide a complete picture of local behavior.

## The Del Operator

All three fundamental operators can be expressed using a single formal vector operator, called **del** or **nabla**:

$$\nabla = \frac{\partial}{\partial x}\,\mathbf{i} + \frac{\partial}{\partial y}\,\mathbf{j} + \frac{\partial}{\partial z}\,\mathbf{k}.$$

This symbol should be thought of as a vector whose components are partial differentiation operators. It does not represent a fixed vector but rather an instruction to differentiate. Depending on how $\nabla$ is applied — to a scalar field or to a vector field, and whether via multiplication or dot/cross product — different operators result:

- $\nabla f$ (del applied to a scalar, giving a vector): the **gradient**.
- $\nabla \cdot \mathbf{F}$ (del dot a vector field, giving a scalar): the **divergence**.
- $\nabla \times \mathbf{F}$ (del cross a vector field, giving a vector): the **curl**.
- $\nabla \cdot \nabla f = \nabla^2 f$ (divergence of the gradient, giving a scalar): the **Laplacian**.

This unification under a single symbol is more than notational elegance — it leads to a rich algebra of identities.

## Chapter Overview

**Section 1: Gradient Revisited** takes a deeper look at the gradient beyond its role as "the vector of partial derivatives." The gradient has a precise directional meaning: $\nabla f(\mathbf{p})$ points in the direction along which $f$ increases most rapidly from $\mathbf{p}$, and its magnitude gives the rate of that increase. The directional derivative in any direction $\hat{\mathbf{u}}$ is $D_{\hat{\mathbf{u}}}f = \nabla f \cdot \hat{\mathbf{u}}$, which is maximized when $\hat{\mathbf{u}}$ aligns with $\nabla f$.

**Section 2: Divergence** introduces the operator $\nabla \cdot \mathbf{F} = \partial P/\partial x + \partial Q/\partial y + \partial R/\partial z$. Geometrically, the divergence at a point measures the net rate of outflow of the field from an infinitesimal volume around that point. A field with positive divergence at a point behaves like a source there; negative divergence signals a sink. Incompressible fluid flow satisfies $\nabla \cdot \mathbf{v} = 0$ (solenoidal fields).

**Section 3: Curl** introduces $\nabla \times \mathbf{F}$, a vector field whose direction is the axis of rotation of $\mathbf{F}$ and whose magnitude measures the intensity of that rotation. The $z$-component of $\nabla \times \mathbf{F}$ in two dimensions reduces to $\partial Q/\partial x - \partial P/\partial y$, exactly the quantity that appears in the curl test for conservativity. Irrotational fields ($\nabla \times \mathbf{F} = \mathbf{0}$) are candidates for conservative fields.

**Section 4: The Laplacian** studies $\nabla^2 f = \nabla \cdot (\nabla f)$, the divergence of the gradient. In coordinates, $\nabla^2 f = \partial^2 f/\partial x^2 + \partial^2 f/\partial y^2 + \partial^2 f/\partial z^2$. Functions satisfying $\nabla^2 f = 0$ (harmonic functions) appear throughout physics: the electrostatic potential in a charge-free region, the steady-state temperature in a heat-conducting solid, and the velocity potential in irrotational fluid flow all satisfy Laplace's equation.

**Section 5: Vector Identities and del Algebra** develops the algebraic identities satisfied by $\nabla$: product rules, the vanishing of $\nabla \times (\nabla f)$, the vanishing of $\nabla \cdot (\nabla \times \mathbf{F})$, and several others. These identities are the grammar of vector calculus, and fluency with them is required for the integral theorems of Units 2 through 4.

## Why These Operators Matter

The divergence and curl are not abstract conveniences but the language in which fundamental physical laws are written. Maxwell's equations — the complete theory of electromagnetism — state precisely that: $\nabla \cdot \mathbf{E} = \rho/\varepsilon_0$ (Gauss's law), $\nabla \cdot \mathbf{B} = 0$ (no magnetic monopoles), $\nabla \times \mathbf{E} = -\partial\mathbf{B}/\partial t$ (Faraday's law), and $\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0\varepsilon_0 \partial\mathbf{E}/\partial t$ (Ampere-Maxwell law). The Navier-Stokes equations of fluid dynamics, the heat equation, and the wave equation are similarly written in terms of these operators. Understanding what gradient, divergence, curl, and Laplacian mean geometrically is prerequisite to reading the laws of nature.
