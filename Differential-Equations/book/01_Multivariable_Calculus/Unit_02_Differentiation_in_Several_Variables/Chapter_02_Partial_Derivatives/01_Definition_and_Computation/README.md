# Definition and Computation of Partial Derivatives

The derivative of a function of one variable measures the rate of change of that function with respect to its single input. When a function has multiple inputs — temperature depending on position $(x, y, z)$ in a room, say, or a profit function depending on prices $p_1, p_2, \ldots, p_n$ of different goods — the question arises: with respect to which variable should we differentiate? The answer, at the most basic level, is that we differentiate with respect to one variable at a time, holding all others fixed. This produces the partial derivatives, which are the building blocks of the entire theory of multivariable differentiation.

## Formal Definition

Let $f: D \subseteq \mathbb{R}^n \to \mathbb{R}$ and let $\mathbf{a} = (a_1, \ldots, a_n)$ be a point in the interior of $D$. The **partial derivative of $f$ with respect to $x_i$ at $\mathbf{a}$** is

$$\frac{\partial f}{\partial x_i}(\mathbf{a}) = \lim_{h\to 0} \frac{f(a_1, \ldots, a_{i-1}, a_i+h, a_{i+1}, \ldots, a_n) - f(a_1, \ldots, a_n)}{h},$$

provided this limit exists. In words: replace $x_i$ by $a_i + h$, leave all other variables at their values in $\mathbf{a}$, form the difference quotient, and take the limit as $h\to 0$.

The notation is extensive and varies by context:

$$\frac{\partial f}{\partial x_i}, \quad f_{x_i}, \quad \partial_{x_i} f, \quad D_i f, \quad D_{x_i} f.$$

For $f(x, y)$, the two partial derivatives are $\partial f/\partial x$ and $\partial f/\partial y$, also written $f_x$ and $f_y$.

## Geometric Interpretation

For $f: \mathbb{R}^2 \to \mathbb{R}$, the graph $z = f(x,y)$ is a surface in $\mathbb{R}^3$. The partial derivative $\frac{\partial f}{\partial x}(a,b)$ is the slope of the tangent line to the curve $z = f(x, b)$ (the cross-section of the surface with the plane $y = b$) at $x = a$. Similarly, $\frac{\partial f}{\partial y}(a,b)$ is the slope of the cross-section $z = f(a, y)$ at $y = b$. The two partial derivatives capture the slopes in the $x$- and $y$-directions respectively; they do not, by themselves, capture the slope in an arbitrary direction (that requires the directional derivative of Chapter 4).

## Computation Rules

Because the partial derivative with respect to $x_i$ is just an ordinary derivative with all other variables held constant, all standard differentiation rules apply:

**Power rule:** $\frac{\partial}{\partial x}(x^n y^m) = nx^{n-1}y^m$ (treating $y^m$ as a constant).

**Product rule:** $\frac{\partial}{\partial x}(f\cdot g) = f_x\cdot g + f\cdot g_x$.

**Chain rule (in its partial form):** if $h(x,y) = g(f(x,y))$, then $h_x = g'(f(x,y))\cdot f_x$.

## Worked Examples

**Example 1.** $f(x,y) = x^3 y^2 - 2xy + 5y$.

$f_x = 3x^2 y^2 - 2y$. (Treat $y$ as constant; differentiate $x^3\to 3x^2$, $xy\to y$, $5y\to 0$.)

$f_y = 2x^3 y - 2x + 5$. (Treat $x$ as constant; differentiate $y^2\to 2y$, $y\to 1$, $5y\to 5$.)

**Example 2.** $f(x,y) = \sin(x^2+y)$.

$f_x = \cos(x^2+y)\cdot 2x$.

$f_y = \cos(x^2+y)\cdot 1 = \cos(x^2+y)$.

**Example 3.** $f(x,y,z) = e^{xyz}$.

$f_x = yze^{xyz}$. $f_y = xze^{xyz}$. $f_z = xye^{xyz}$.

**Example 4 (using the definition).** Show that $f(x,y) = |xy|$ has zero partial derivatives at the origin.

$f_x(0,0) = \lim_{h\to 0}\frac{|h\cdot 0| - 0}{h} = \lim_{h\to 0}\frac{0}{h} = 0$.

$f_y(0,0) = 0$ similarly.

Yet $f$ is not differentiable at the origin (shown in Chapter 3), since the function is not smooth there. This illustrates that partial derivatives can exist even where differentiability fails.

## Partial Derivatives as Functions

For each $i$, $\frac{\partial f}{\partial x_i}$ is itself a function on (a subset of) $\mathbb{R}^n$. If all $n$ partial derivatives exist and are continuous on an open set $U$, we say $f$ is **of class $C^1$** on $U$, written $f \in C^1(U)$. This condition (continuous partial derivatives) is stronger than merely having partial derivatives, and it implies differentiability — a fact proved in Chapter 3.

## Notation for Functions of Many Variables

For $f: \mathbb{R}^n \to \mathbb{R}$ with variables $x_1, \ldots, x_n$, the partial derivatives are $\partial f/\partial x_i$ for $i = 1, \ldots, n$. The **gradient** is the vector

$$\nabla f = \left(\frac{\partial f}{\partial x_1}, \frac{\partial f}{\partial x_2}, \ldots, \frac{\partial f}{\partial x_n}\right),$$

fully developed in Chapter 4. For now, note that the gradient packages all $n$ partial derivatives into a single vector.

## When Partial Derivatives Don't Exist

The partial derivative $\partial f/\partial x_i$ at $\mathbf{a}$ is a limit and may fail to exist if the one-variable function $g(t) = f(a_1, \ldots, a_{i-1}, t, a_{i+1}, \ldots, a_n)$ is not differentiable at $a_i$. This can happen for the same reasons ordinary derivatives fail: corners, cusps, or jump discontinuities in $g$.

**Example.** $f(x,y) = |x|$. Then $f_x(0,y) = \lim_{h\to 0}|h|/h$, which does not exist (left and right limits are $-1$ and $+1$). The partial derivative with respect to $x$ fails at $x = 0$ for any $y$.

## Common Pitfalls

When computing $\partial f/\partial x$, all variables other than $x$ are treated as constants. Students sometimes inadvertently differentiate through other variables using the chain rule when it is not needed. For instance, $\partial/\partial x$ of $y^3$ is $0$, not $3y^2$ — $y$ is being held constant, so $y^3$ is a constant with respect to $x$.

Another common mistake is writing $\partial^2 f/(\partial x)^2$ (which is meaningful, being $f_{xx}$) and confusing it with $(\partial f/\partial x)^2$ (which is the square of $f_x$, a completely different quantity). The notation $\partial^2 f/\partial x^2$ always means the second partial derivative, never the square of the first.
