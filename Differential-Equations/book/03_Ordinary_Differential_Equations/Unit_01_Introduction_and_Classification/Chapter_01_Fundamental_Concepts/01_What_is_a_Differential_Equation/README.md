# What is a Differential Equation?

Consider a quantity $y$ that changes continuously with time $t$. If the rate of change of $y$ is proportional to $y$ itself, then the mathematical expression of this fact is $dy/dt = ky$ for some constant $k$. This single equation encodes an enormous amount of information: it says that the function $y(t)$ and its own derivative are related by a fixed ratio. The task of differential equations as a subject is to recover $y$ from this relational information, and to understand what kinds of behavior can arise from different relations of this sort.

## The Formal Definition

An **ordinary differential equation** (ODE) is an equation that involves an unknown function of one independent variable together with one or more of its derivatives. Writing $x$ for the independent variable and $y$ for the unknown function, the most general form is

$$F\!\left(x,\, y,\, y',\, y'',\, \ldots,\, y^{(n)}\right) = 0,$$

where $F$ is a given function of $n+2$ variables and $y^{(k)}$ denotes the $k$-th derivative of $y$ with respect to $x$. The integer $n \geq 1$ is the **order** of the equation, being the order of the highest derivative that appears.

The word "ordinary" distinguishes this from a **partial differential equation** (PDE), which involves an unknown function of several independent variables and partial derivatives. The equation $\partial u/\partial t = k\,\partial^2 u/\partial x^2$ governing heat conduction in a rod is a PDE; the equation $y'' + y = 0$ governing oscillations of a spring is an ODE. This entire module concerns ordinary differential equations.

## Why Differential Equations Arise

The reason differential equations appear so pervasively in science is that the laws of nature are typically expressed as relations between quantities and their rates of change, not as direct recipes for the quantities themselves. Newton's second law $F = ma$ says that force equals mass times acceleration, where acceleration is the second derivative of position. This is a differential equation for position as a function of time. The law of radioactive decay says that the rate at which atoms disintegrate is proportional to the number present: $dN/dt = -\lambda N$. Kirchhoff's voltage law applied to an RLC circuit yields a second-order ODE for the current or charge. In each case, what is known is a relation, and what is sought is the function satisfying it.

## Concrete Examples

**Example 1: Exponential growth.** The equation

$$\frac{dy}{dt} = ky,\quad k > 0,$$

models a population or bank balance growing at a constant percentage rate. It is a first-order ODE. One verifies directly that $y = Ce^{kt}$ satisfies it for any constant $C$: differentiating gives $dy/dt = Cke^{kt} = k \cdot Ce^{kt} = ky$. The family of functions $\{Ce^{kt} : C \in \mathbb{R}\}$ constitutes all solutions.

**Example 2: Simple harmonic oscillation.** The equation

$$\frac{d^2x}{dt^2} + \omega^2 x = 0,\quad \omega > 0,$$

describes a mass on a frictionless spring, where $x$ is displacement and $\omega = \sqrt{k/m}$. It is a second-order ODE. The functions $x = A\cos(\omega t) + B\sin(\omega t)$ satisfy it for any constants $A, B$, and there are no other solutions (a fact we will prove when we develop the theory of linear equations).

**Example 3: Nonlinear pendulum.** The exact equation for a pendulum of length $\ell$ swinging under gravity is

$$\frac{d^2\theta}{dt^2} + \frac{g}{\ell}\sin\theta = 0.$$

This second-order ODE is nonlinear because of the $\sin\theta$ term. No elementary closed-form solution exists; analytic solutions require elliptic integrals. For small $\theta$ one approximates $\sin\theta \approx \theta$ and recovers the simple harmonic oscillator above. This example already illustrates the central difficulty of the subject: nonlinearity destroys the solution methods that work so well in the linear case.

**Example 4: A first-order nonlinear equation.** Consider

$$\frac{dy}{dx} = y^2.$$

Separating variables gives $dy/y^2 = dx$, so $-1/y = x + C$, yielding $y = -1/(x+C)$. This family of solutions has a striking feature: for $C = 0$, the solution is $y = -1/x$, which is undefined at $x = 0$. Even though the equation is perfectly smooth, individual solutions can fail to exist for all $x$. This is a phenomenon specific to nonlinear equations, and it motivates the careful study of the interval of existence.

## What Solving Means

A **solution** of the ODE $F(x, y, y', \ldots, y^{(n)}) = 0$ on an interval $I$ is a function $\phi: I \to \mathbb{R}$ that is $n$ times differentiable on $I$ and satisfies $F(x, \phi(x), \phi'(x), \ldots, \phi^{(n)}(x)) = 0$ for all $x \in I$. The interval $I$ is part of the data: a function may satisfy the equation on one interval but not on another because its derivatives may not exist at certain points.

This definition is more subtle than it first appears. The equation $y' = |y|^{1/2}$ has the obvious solution $y = 0$, but it also has the solution

$$y = \begin{cases} 0 & x \leq 0 \\ x^2/4 & x > 0 \end{cases}$$

which satisfies $y' = |y|^{1/2}$ everywhere, including at $x = 0$ where the derivative from the left is $0$ and from the right is also $0$. The existence of this second solution shows that uniqueness can fail, a phenomenon we will examine in depth when we discuss Picard's theorem.

## The Solution Curve

Geometrically, a solution of a first-order ODE $y' = f(x,y)$ is a curve in the $xy$-plane. At each point $(x, y)$, the equation prescribes the slope that the solution curve must have there. The collection of all these prescribed slopes, visualized as short line segments, is called the **direction field** (or slope field) of the equation. A solution is a curve that is everywhere tangent to these segments. This geometric picture makes it possible to understand qualitative behavior, such as whether solutions grow, decay, or oscillate, without ever writing down a formula.

## Looking Ahead

The subject divides naturally according to the structure of the equation: its order, whether it is linear or nonlinear, and whether its coefficients are constant or variable. Each structural class admits its own theory and its own solution methods. The first task, taken up in the next section, is to introduce this classification precisely so that the appropriate tool can be selected for any given equation.
