# First Order Approximation

The single most important technique in calculus — and in applied mathematics generally — is linearization: the approximation of a complicated function by a simpler linear one in a neighborhood of a given point. In one variable, this is the tangent line approximation $f(a+h) \approx f(a) + f'(a)h$. In several variables, the corresponding approximation is $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h}$. This section makes the approximation precise, quantifies the error, and develops the practical skill of using linear approximations in computation.

## The Linearization

If $f:\mathbb{R}^n\to\mathbb{R}$ is differentiable at $\mathbf{a}$, the **linearization** (or **first-order Taylor approximation**) of $f$ at $\mathbf{a}$ is the affine function

$$L(\mathbf{x}) = f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot(\mathbf{x}-\mathbf{a}).$$

This is the unique affine function whose value and gradient agree with those of $f$ at $\mathbf{a}$:
- $L(\mathbf{a}) = f(\mathbf{a})$.
- $\nabla L(\mathbf{a}) = \nabla f(\mathbf{a})$ (since $L(\mathbf{x}) = \text{const} + \nabla f(\mathbf{a})\cdot(\mathbf{x}-\mathbf{a})$, whose gradient is $\nabla f(\mathbf{a})$).

The approximation $f(\mathbf{x}) \approx L(\mathbf{x})$ is valid for $\mathbf{x}$ near $\mathbf{a}$, with error $o(\|\mathbf{x}-\mathbf{a}\|)$ — this is precisely the definition of differentiability.

For $f:\mathbb{R}^2\to\mathbb{R}$, the linearization gives the equation of the **tangent plane** to the graph $z = f(x,y)$ at the point $(a, b, f(a,b))$:

$$z = f(a,b) + f_x(a,b)(x-a) + f_y(a,b)(y-b).$$

## Taylor's Theorem with Remainder (First Order)

**Theorem.** If $f:\mathbb{R}^n\to\mathbb{R}$ is $C^2$ on an open set containing the segment $[\mathbf{a}, \mathbf{a}+\mathbf{h}]$, then

$$f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h} + R_1(\mathbf{h}),$$

where the **Lagrange remainder** satisfies $|R_1(\mathbf{h})| \leq C\|\mathbf{h}\|^2$ for some constant $C$ depending on the second derivatives of $f$.

**Proof sketch.** Define $g(t) = f(\mathbf{a}+t\mathbf{h})$ for $t\in[0,1]$. Then $g'(t) = \nabla f(\mathbf{a}+t\mathbf{h})\cdot\mathbf{h}$ (chain rule). Taylor's theorem in one variable: $g(1) = g(0) + g'(0) + \frac{1}{2}g''(\xi)$ for some $\xi\in(0,1)$. This gives $f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h} + R_1$ with $R_1 = \frac{1}{2}g''(\xi)$ bounded by a quadratic expression in $\|\mathbf{h}\|$.

## Worked Examples

**Example 1: Approximation.**

$f(x,y) = e^x\sin y$. At $(0, 0)$: $f(0,0) = 0$, $f_x = e^x\sin y\big|_{(0,0)} = 0$, $f_y = e^x\cos y\big|_{(0,0)} = 1$.

Linearization: $L(x,y) = 0 + 0\cdot x + 1\cdot y = y$.

So $e^x\sin y \approx y$ for small $x, y$.

**Example 2: Error estimation.**

Estimate $f(1.1, 2.05)$ for $f(x,y) = x^2 y$ using the linearization at $(1,2)$.

$f(1,2) = 2$, $f_x = 2xy = 4$, $f_y = x^2 = 1$.

$L(1.1, 2.05) = 2 + 4(0.1) + 1(0.05) = 2 + 0.4 + 0.05 = 2.45$.

Exact: $f(1.1, 2.05) = (1.21)(2.05) = 2.4805$.

Error: $|2.4805 - 2.45| = 0.0305$, while $\|\mathbf{h}\|^2 = (0.1)^2+(0.05)^2 = 0.0125$. The error is $O(\|\mathbf{h}\|^2)$ as expected.

**Example 3: Differentials.**

The **total differential** is the notation $df = f_x\,dx + f_y\,dy + f_z\,dz$, where $dx, dy, dz$ represent small increments in the variables. For $f(x,y,z) = xy^2+z^3$: $df = y^2\,dx + 2xy\,dy + 3z^2\,dz$. This is a shorthand for the linear approximation: the change in $f$ is approximately $df$ when the variables change by $dx, dy, dz$.

**Example 4: Error propagation.** The volume of a cylinder is $V = \pi r^2 h$. If $r$ and $h$ are measured with errors $\Delta r$ and $\Delta h$, the error in $V$ is approximately $\Delta V \approx \frac{\partial V}{\partial r}\Delta r + \frac{\partial V}{\partial h}\Delta h = 2\pi rh\,\Delta r + \pi r^2\,\Delta h$. This is the linearization applied to error analysis.

## Linearization of Vector-Valued Functions

For $\mathbf{f}:\mathbb{R}^n\to\mathbb{R}^m$, the first-order Taylor approximation is

$$\mathbf{f}(\mathbf{a}+\mathbf{h}) \approx \mathbf{f}(\mathbf{a}) + J_\mathbf{f}(\mathbf{a})\mathbf{h},$$

where $J_\mathbf{f}$ is the Jacobian matrix. This is the linear approximation of the map near $\mathbf{a}$.

## The Role of Linearization in Differential Equations

The qualitative behavior of a nonlinear system of ODEs $\dot{\mathbf{x}} = \mathbf{F}(\mathbf{x})$ near an equilibrium $\mathbf{x}^* = \mathbf{F}(\mathbf{x}^*) = \mathbf{0}$ is determined by the **linearized system** $\dot{\mathbf{y}} = J_\mathbf{F}(\mathbf{x}^*)\mathbf{y}$ (where $\mathbf{y} = \mathbf{x} - \mathbf{x}^*$). This is the first-order Taylor approximation of $\mathbf{F}$ at $\mathbf{x}^*$. The eigenvalues of the Jacobian $J_\mathbf{F}(\mathbf{x}^*)$ determine the stability of the equilibrium (Hartman-Grobman theorem). This is why linearization — and thus multivariable Taylor series — is fundamental to the analysis of differential equations.
