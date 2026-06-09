# Variation of Parameters for First-Order Linear Equations

Variation of parameters is a method for finding particular solutions of nonhomogeneous linear equations by systematically "varying" the constant that appears in the homogeneous solution. For first-order equations it is equivalent to the integrating factor method, but the conceptual framework it introduces, treating the constant as an unknown function, generalizes cleanly to higher-order equations and systems where no equivalent of the integrating factor is available.

## The Method

Consider $y' + p(x)y = q(x)$. The homogeneous equation $y' + p(x)y = 0$ has general solution $y_h = Ce^{-\int p\,dx}$. Write $e^{-\int p\,dx} = \phi(x)$ for brevity, so $y_h = C\phi(x)$.

The variation of parameters idea: seek a particular solution in the form $y_p = v(x)\phi(x)$, where $v(x)$ is now an unknown function (the "varied parameter"). Substituting:

$$y_p' + p(x)y_p = v'\phi + v\phi' + p(x)v\phi = v'\phi + v(\phi' + p\phi) = v'\phi + v \cdot 0 = v'\phi,$$

since $\phi' + p\phi = 0$ (as $\phi$ solves the homogeneous equation). Setting this equal to $q(x)$:

$$v'\phi(x) = q(x) \implies v'(x) = \frac{q(x)}{\phi(x)} = q(x)e^{\int p\,dx}.$$

Integrating: $v(x) = \int q(x)e^{\int p\,dx}\,dx$. Therefore

$$y_p = v(x)\phi(x) = e^{-\int p\,dx}\int q(x)e^{\int p\,dx}\,dx,$$

which is the same formula produced by the integrating factor method.

## Why the Method Works

The key step is that $y_p = v\phi$ implies $y_p' + py_p = v'\phi$, with the $v\phi' + pv\phi$ terms canceling because $\phi$ satisfies the homogeneous equation. This cancellation is the essential trick: varying the parameter turns the problem of finding $y_p$ into solving $v'(x) = q(x)/\phi(x)$, a first-order equation for $v$ that involves only integration.

This structural argument works regardless of the order of the equation, as long as the solution $\phi$ of the homogeneous equation is known. For a second-order homogeneous equation with two independent solutions $\phi_1$ and $\phi_2$, the variation of parameters method seeks $y_p = v_1\phi_1 + v_2\phi_2$ with two conditions imposed on $v_1$ and $v_2$.

## Worked Example

Solve $y' - y = e^{2x}$.

Homogeneous solution: $\phi(x) = e^x$ (from $y' - y = 0$). Seek $y_p = v(x)e^x$. Then $y_p' - y_p = v'e^x + ve^x - ve^x = v'e^x = e^{2x}$, so $v' = e^x$, giving $v = e^x$. Thus $y_p = e^x \cdot e^x = e^{2x}$.

General solution: $y = Ce^x + e^{2x}$.

Verification: $y' = Ce^x + 2e^{2x}$; $y' - y = Ce^x + 2e^{2x} - Ce^x - e^{2x} = e^{2x}$. Correct.

## Comparison with the Integrating Factor

Both methods yield the same solution, and for first-order equations the integrating factor is computationally simpler because it requires only one integration. The advantage of variation of parameters is conceptual: it introduces the idea of "varying a constant," which is the right framework for thinking about particular solutions of nonhomogeneous equations in general. When the homogeneous solution is known, variation of parameters provides a constructive recipe for the particular solution that requires no guessing about its form, in contrast to the method of undetermined coefficients.
