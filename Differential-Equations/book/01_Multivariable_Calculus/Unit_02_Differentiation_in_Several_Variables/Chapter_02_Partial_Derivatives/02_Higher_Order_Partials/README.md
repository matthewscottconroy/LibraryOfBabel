# Higher Order Partial Derivatives

Once the first partial derivatives of a function are computed, those derivatives are themselves functions of $n$ variables and can in turn be differentiated. The resulting objects are the higher-order partial derivatives. Second-order partial derivatives carry information about the concavity and curvature of the function's graph; they appear in the second-order Taylor approximation, the Hessian matrix, the second derivative test, and, most importantly for this course, in partial differential equations, which are equations relating a function to its partial derivatives of various orders.

## Second-Order Partial Derivatives

For a function $f: \mathbb{R}^n \to \mathbb{R}$, the **second-order partial derivatives** are the partial derivatives of the first partial derivatives. For a function of two variables $f(x, y)$, there are four:

$$f_{xx} = \frac{\partial^2 f}{\partial x^2} = \frac{\partial}{\partial x}\left(\frac{\partial f}{\partial x}\right), \qquad f_{xy} = \frac{\partial^2 f}{\partial y \partial x} = \frac{\partial}{\partial y}\left(\frac{\partial f}{\partial x}\right),$$

$$f_{yx} = \frac{\partial^2 f}{\partial x \partial y} = \frac{\partial}{\partial x}\left(\frac{\partial f}{\partial y}\right), \qquad f_{yy} = \frac{\partial^2 f}{\partial y^2} = \frac{\partial}{\partial y}\left(\frac{\partial f}{\partial y}\right).$$

The **pure second derivatives** $f_{xx}$ and $f_{yy}$ differentiate twice with respect to the same variable. The **mixed second derivatives** $f_{xy}$ and $f_{yx}$ differentiate with respect to two different variables, in two different orders.

**Notation warning:** $f_{xy}$ is ambiguous across different texts. In the Leibniz notation $\frac{\partial^2 f}{\partial y \partial x}$, the convention is to differentiate first with respect to $x$ (right to left), then with respect to $y$. In subscript notation $f_{xy}$, the convention is usually to differentiate first with respect to $x$, then $y$ (left to right). Always verify the convention in use.

## Worked Examples

**Example 1.** $f(x,y) = x^3y^2 + \sin(xy)$.

$f_x = 3x^2y^2 + y\cos(xy)$.
$f_y = 2x^3y + x\cos(xy)$.

$f_{xx} = \frac{\partial}{\partial x}(3x^2y^2 + y\cos(xy)) = 6xy^2 - y^2\sin(xy)$.

$f_{yy} = \frac{\partial}{\partial y}(2x^3y + x\cos(xy)) = 2x^3 - x^2\sin(xy)$.

$f_{xy} = \frac{\partial}{\partial y}(3x^2y^2 + y\cos(xy)) = 6x^2y + \cos(xy) - xy\sin(xy)$.

$f_{yx} = \frac{\partial}{\partial x}(2x^3y + x\cos(xy)) = 6x^2y + \cos(xy) - xy\sin(xy)$.

In this case, $f_{xy} = f_{yx}$ — as Clairaut's theorem (the next section) guarantees, since all partials are continuous.

**Example 2.** $f(x,y,z) = e^{x+2y-z}$.

$f_x = e^{x+2y-z}$, $f_y = 2e^{x+2y-z}$, $f_z = -e^{x+2y-z}$.

$f_{xx} = e^{x+2y-z}$, $f_{yy} = 4e^{x+2y-z}$, $f_{zz} = e^{x+2y-z}$.

$f_{xy} = 2e^{x+2y-z}$, $f_{xz} = -e^{x+2y-z}$, $f_{yz} = -2e^{x+2y-z}$.

## Higher-Order Derivatives

Third- and higher-order derivatives are defined by continued differentiation. For $f(x,y)$, there are $2^k$ mixed partial derivatives of order $k$ (some of which coincide by Clairaut's theorem when $f$ is sufficiently smooth). For a function of $n$ variables, the number of distinct $k$-th order partial derivatives is $\binom{n+k-1}{k}$ (after applying symmetry).

The third derivative $f_{xxy}$ means: differentiate first twice with respect to $x$, then once with respect to $y$. For smooth functions, any permutation of the order of differentiation gives the same result.

## Multi-Index Notation

For functions of $n$ variables, it is cumbersome to write individual subscripts for high-order derivatives. A **multi-index** is $\alpha = (\alpha_1, \ldots, \alpha_n) \in \mathbb{Z}_{\geq 0}^n$. Define $|\alpha| = \alpha_1 + \cdots + \alpha_n$ (the order) and

$$\partial^\alpha f = \frac{\partial^{|\alpha|} f}{\partial x_1^{\alpha_1} \cdots \partial x_n^{\alpha_n}}.$$

This notation allows Taylor series and differential equations to be written compactly. For instance, Laplace's equation $\Delta f = 0$ says $\partial^{(2,0)}f + \partial^{(0,2)}f = 0$ in two variables.

## The Laplacian

A particularly important second-order differential operator is the **Laplacian**:

$$\Delta f = \nabla^2 f = \frac{\partial^2 f}{\partial x_1^2} + \frac{\partial^2 f}{\partial x_2^2} + \cdots + \frac{\partial^2 f}{\partial x_n^2} = \sum_{i=1}^n f_{x_i x_i}.$$

The Laplacian appears in Laplace's equation ($\Delta f = 0$, for harmonic functions), the heat equation ($\partial f/\partial t = k\Delta f$), and the wave equation ($\partial^2 f/\partial t^2 = c^2\Delta f$). A function satisfying $\Delta f = 0$ is called **harmonic** and has remarkable properties: it achieves its maximum and minimum values on the boundary of any compact region (maximum principle), and it is infinitely differentiable.

**Example.** Show $f(x,y) = \ln(x^2+y^2)$ is harmonic for $(x,y)\neq(0,0)$.

$f_x = \frac{2x}{x^2+y^2}$, $f_{xx} = \frac{2(x^2+y^2) - 2x\cdot 2x}{(x^2+y^2)^2} = \frac{2y^2-2x^2}{(x^2+y^2)^2}$.

$f_y = \frac{2y}{x^2+y^2}$, $f_{yy} = \frac{2x^2-2y^2}{(x^2+y^2)^2}$.

$\Delta f = f_{xx}+f_{yy} = \frac{2y^2-2x^2+2x^2-2y^2}{(x^2+y^2)^2} = 0$.

## Connection to Differential Equations

The fundamental partial differential equations of physics — Laplace's, heat, and wave equations — are second-order. Understanding what second-order partial derivatives are, how to compute them, and how they are related (Clairaut's theorem ensuring the Hessian is symmetric) is the prerequisite for everything in the PDE unit of this course. When one says "solve the heat equation $u_t = \Delta u$," one is asking for a function whose first time-derivative equals the sum of its second spatial partial derivatives — a statement meaningful only because of the theory developed here.
