# Two-Point Boundary Value Problems

A two-point boundary value problem (BVP) consists of a second-order ordinary differential equation together with conditions imposed at two distinct points. The standard form is:

$$y'' + p(x)y' + q(x)y = f(x), \qquad a < x < b,$$

with boundary conditions, for example:

$$\alpha_0 y(a) + \alpha_1 y'(a) = \gamma_0, \qquad \beta_0 y(b) + \beta_1 y'(b) = \gamma_1.$$

When $\alpha_1 = \beta_1 = 0$: Dirichlet boundary conditions $y(a) = \gamma_0$, $y(b) = \gamma_1$. When $\alpha_0 = \beta_0 = 0$: Neumann boundary conditions $y'(a) = \gamma_0$, $y'(b) = \gamma_1$. The mixed case is a **Robin** boundary condition.

## Existence and Uniqueness

Unlike IVPs, BVPs do not automatically have unique solutions. The situation is governed by the **Fredholm alternative**, a linear algebra principle for differential operators.

**Theorem.** Consider the BVP $Ly = f$ (where $Ly = y'' + py' + qy$) with homogeneous boundary conditions ($\gamma_0 = \gamma_1 = 0$). Either:

(a) The homogeneous problem $Ly = 0$ with homogeneous BCs has only the trivial solution $y = 0$. In this case, the nonhomogeneous problem $Ly = f$ with any $f$ and any (possibly nonhomogeneous) BCs has a **unique solution**.

(b) The homogeneous problem $Ly = 0$ with homogeneous BCs has a nontrivial solution $\phi \neq 0$. In this case, the nonhomogeneous problem $Ly = f$ with homogeneous BCs has either no solution or infinitely many solutions (a one-parameter family $y_p + c\phi$), with a solution existing only if $f$ is "compatible" with $\phi$ (the solvability condition $\int_a^b f\phi\,dx = 0$ in the self-adjoint case).

**Example.** The BVP $y'' + y = 0$, $y(0) = 0$, $y(\pi) = 0$. The general solution of $y'' + y = 0$ is $y = c_1\cos x + c_2\sin x$. Applying $y(0) = 0$: $c_1 = 0$. So $y = c_2\sin x$. Applying $y(\pi) = 0$: $c_2\sin\pi = 0$: satisfied for any $c_2$. There are infinitely many solutions: $y = c\sin x$ for any constant $c$.

Contrast with $y'' + y = 0$, $y(0) = 0$, $y(\pi/2) = 0$. Then $y = c_2\sin x$, and $y(\pi/2) = c_2 = 0$, so only the trivial solution exists. If the right side is replaced by $f \neq 0$, the problem has a unique solution.

The bifurcation between cases (a) and (b) occurs exactly at the eigenvalues of the associated homogeneous problem — a foreshadowing of Sturm-Liouville theory.

## Structure of Solutions

For case (a) (unique solution), the solution can be written as $y = y_p + y_h$, where $y_h$ solves the homogeneous equation and is determined by the boundary conditions, and $y_p$ is any particular solution. For the Dirichlet problem, one can use the **complementary solutions**:

Let $y_1$ and $y_2$ be linearly independent solutions of the homogeneous equation $Ly = 0$. The general homogeneous solution is $c_1 y_1 + c_2 y_2$. Add a particular solution $y_p$ of $Ly = f$ (found by variation of parameters). Apply the two boundary conditions to determine $c_1$ and $c_2$: this gives a $2\times 2$ linear system $\begin{pmatrix}y_1(a)&y_2(a)\\y_1(b)&y_2(b)\end{pmatrix}\begin{pmatrix}c_1\\c_2\end{pmatrix} = \begin{pmatrix}\gamma_0 - y_p(a)\\\gamma_1 - y_p(b)\end{pmatrix}$ (for Dirichlet BCs).

The system has a unique solution iff the determinant $y_1(a)y_2(b) - y_2(a)y_1(b) \neq 0$ — exactly the condition that the homogeneous BVP has only the trivial solution.

## Worked Example: Dirichlet BVP

Solve $y'' - y = -x$, $y(0) = 0$, $y(1) = 1$.

The homogeneous equation $y'' - y = 0$ has solutions $y_1 = e^x$, $y_2 = e^{-x}$ (or equivalently $\cosh x$, $\sinh x$).

A particular solution of $y'' - y = -x$: try $y_p = ax + b$. Then $y_p'' = 0$, so $-ax - b = -x$: $a = 1$, $b = 0$. So $y_p = x$.

General solution: $y = c_1 e^x + c_2 e^{-x} + x$.

Apply $y(0) = 0$: $c_1 + c_2 = 0$, so $c_2 = -c_1$.

Apply $y(1) = 1$: $c_1 e + c_2 e^{-1} + 1 = 1$, so $c_1(e - e^{-1}) = 0$. Since $e - e^{-1} \neq 0$: $c_1 = 0$, $c_2 = 0$.

Solution: $y = x$. Verification: $y'' = 0$, $y'' - y = -x$. Correct.

## Physical Applications

Two-point BVPs arise in: the deflection of a beam under transverse loading (Euler-Bernoulli beam equation), where the beam is supported or clamped at both ends; the steady-state temperature in a rod with fixed endpoint temperatures and distributed heat sources; the electrostatic potential between two conducting plates; quantum mechanical bound states (Schrödinger equation on a finite interval); and the buckling load of a column (an eigenvalue BVP, with the critical load as the eigenvalue).

In each case, the boundary conditions encode the physical constraints — clamped vs. free end, grounded vs. insulated plate — and the differential equation encodes the physics of the interior. The interplay between the equation and the boundary conditions determines whether steady states exist and how many.

## Nonlinear BVPs

For nonlinear BVPs $y'' = f(x, y, y')$ with $y(a) = \alpha$, $y(b) = \beta$, the analysis is more complex. Solutions may not exist or may not be unique without special assumptions. The **Schauder fixed-point theorem** and the **method of upper and lower solutions** provide existence theorems under appropriate monotonicity or coercivity conditions. For numerical treatment, the shooting method (see next section) is standard.

A particularly useful sufficient condition for uniqueness is the **Picard condition**: if $\partial f/\partial y \geq 0$ on the domain, then the Dirichlet BVP has at most one solution. This is analogous to the monotonicity condition for the implicit function theorem.
