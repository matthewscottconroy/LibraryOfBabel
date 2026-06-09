# Chapter 5: Substitution Methods

When a first-order ODE does not fit the separable, linear, or exact frameworks, the right substitution can transform it into one of these solvable forms. Several substitution strategies are sufficiently important and widely applicable to deserve systematic treatment. This chapter develops four of them: the homogeneous substitution ($v = y/x$), the Bernoulli linearization, the Riccati reduction, and a general strategy for recognizing useful substitutions.

## The Role of Substitutions

A substitution $y = f(x, v)$ (or equivalently, a change of dependent variable from $y$ to $v$) transforms the equation for $y$ into an equation for $v$. The goal is to choose the substitution so that the equation for $v$ is of a type already known to be solvable. The skill lies in recognizing which substitution applies to a given equation.

## Chapter Contents

The first section treats **homogeneous equations** in the sense of $y' = f(y/x)$: equations where the right side depends only on the ratio $y/x$. The substitution $v = y/x$ (so $y = vx$ and $y' = v + xv'$) converts these into separable equations for $v$.

The second section treats **Bernoulli equations** $y' + p(x)y = q(x)y^n$ for $n \neq 0, 1$. The substitution $w = y^{1-n}$ linearizes the equation. The cases $n = 0$ (linear) and $n = 1$ (separable) are excluded since they are already solvable by earlier methods. The Bernoulli equation arises naturally in the logistic model and in many physical contexts.

The third section addresses the **Riccati equation** $y' = p(x) + q(x)y + r(x)y^2$. Unlike the other equations here, Riccati equations have no universal closed-form solution. However, if one particular solution $y_1$ is known, the substitution $y = y_1 + 1/v$ reduces the equation to a first-order linear equation for $v$.

The fourth section discusses the general strategy of recognizing patterns that suggest productive substitutions, including equations of the form $y' = f(ax + by + c)$ and the linearization via $z = ax + by + c$.

## Conceptual Unity

All these methods share a single conceptual structure: a change of variables transforms the equation into a simpler form. The ability to recognize which transformation to apply, and why it works, is the central skill of this chapter. Developing this skill prepares the student for the broader use of transformations in differential equations, including the Laplace transform, Fourier series, and the transformations used in the theory of special functions.
