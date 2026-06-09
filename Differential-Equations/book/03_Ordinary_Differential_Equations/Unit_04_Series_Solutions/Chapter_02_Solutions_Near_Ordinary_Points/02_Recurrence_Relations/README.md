# Recurrence Relations

The power series method converts the ODE into an infinite system of algebraic equations relating consecutive coefficients $a_n$. This system is called the **recurrence relation** (or recurrence formula), and solving it gives the coefficients of the series solution term by term.

## Types of Recurrence Relations

A **two-term recurrence** expresses $a_{n+2}$ directly in terms of $a_n$ (or $a_{n+k}$ in terms of $a_n$):

$$a_{n+2} = f(n)a_n.$$

This is the case when the ODE has only even- or only odd-power terms, decoupling the even and odd series. Examples: $y'' + y = 0$ gives $a_{n+2} = -a_n/((n+2)(n+1))$; $y'' - y = 0$ gives $a_{n+2} = a_n/((n+2)(n+1))$.

A **three-term recurrence** expresses $a_{n+2}$ in terms of both $a_{n+1}$ and $a_n$:

$$a_{n+2} = \alpha(n)a_{n+1} + \beta(n)a_n.$$

This arises when the coefficient functions $p$ or $q$ are non-constant polynomials.

## Worked Example: Legendre's Equation

The equation $(1-x^2)y'' - 2xy' + n(n+1)y = 0$ (Legendre's equation with parameter $n$) in standard form:

$$y'' - \frac{2x}{1-x^2}y' + \frac{n(n+1)}{1-x^2}y = 0.$$

It is easier to work in the un-divided form. Substituting $y = \sum_{k=0}^\infty a_k x^k$, one obtains the recurrence

$$a_{k+2} = \frac{k(k+1) - n(n+1)}{(k+2)(k+1)}a_k = \frac{(k-n)(k+n+1)}{(k+2)(k+1)}a_k.$$

When $n$ is a non-negative integer, the numerator $(k-n)(k+n+1)$ vanishes at $k = n$, causing the recurrence to terminate: one series terminates at the $x^n$ term, giving a polynomial solution (the Legendre polynomial $P_n(x)$). The other series does not terminate and defines the second solution $Q_n(x)$.

## Properties of Recurrence Relations

A recurrence relation determines all coefficients with index $n \geq 2$ from $a_0$ and $a_1$ (for second-order equations). The solution thus depends on exactly two free parameters, confirming that the solution space is two-dimensional.

In practice, one computes as many terms as needed to achieve desired accuracy or to identify a pattern. For known special functions (Bessel, Legendre, Hermite, etc.), the recurrence relation is the defining characteristic, used to derive orthogonality relations, generating functions, and other properties.

## Convergence via the Recurrence

The ratio $|a_{n+2}/a_n|$ from the recurrence determines the radius of convergence via the root or ratio test. For Legendre's equation, $|a_{k+2}/a_k| \to 1$ as $k \to \infty$, giving radius of convergence 1 — matching the distance to the nearest singularities at $x = \pm 1$.

For Airy's equation, $|a_{n+3}/a_n| = 1/((n+3)(n+2)) \to 0$, giving infinite radius of convergence — matching the fact that the equation has no finite singularities.
