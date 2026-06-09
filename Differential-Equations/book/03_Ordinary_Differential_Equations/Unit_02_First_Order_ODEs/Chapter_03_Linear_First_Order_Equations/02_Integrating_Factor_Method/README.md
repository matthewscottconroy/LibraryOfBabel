# The Integrating Factor Method

The integrating factor method is the universal solution algorithm for first-order linear equations. It converts $y' + p(x)y = q(x)$ into an equation whose left side is a perfect derivative, enabling direct integration. The result is an explicit formula for the general solution.

## Derivation

Multiply both sides of $y' + p(x)y = q(x)$ by a function $\mu(x)$ to be determined:

$$\mu(x)\,y' + \mu(x)\,p(x)\,y = \mu(x)\,q(x).$$

The left side would be the derivative of $\mu(x)y$ if and only if $(\mu y)' = \mu y' + \mu' y$ equals $\mu y' + \mu p y$, which requires $\mu' = \mu p(x)$. This is a separable equation for $\mu$:

$$\frac{d\mu}{\mu} = p(x)\,dx \implies \ln\mu = \int p(x)\,dx \implies \mu(x) = e^{\int p(x)\,dx}.$$

(The constant of integration is absorbed into the arbitrary constant of the final solution, so it is conventional to take the constant as zero.) With this choice of $\mu$:

$$\left(\mu(x)\,y\right)' = \mu(x)\,q(x).$$

Integrating both sides:

$$\mu(x)\,y = \int \mu(x)\,q(x)\,dx + C.$$

Dividing by $\mu(x)$:

$$y = \frac{1}{\mu(x)}\left[\int \mu(x)\,q(x)\,dx + C\right] = e^{-\int p\,dx}\left[\int e^{\int p\,dx}\,q(x)\,dx + C\right].$$

This is the general solution.

## Worked Example 1

Solve $y' + 2y = 4x$.

The integrating factor is $\mu = e^{\int 2\,dx} = e^{2x}$. Multiplying: $(e^{2x}y)' = 4xe^{2x}$. Integrating the right side by parts:

$$\int 4xe^{2x}\,dx = 4\cdot\frac{xe^{2x}}{2} - 4\int\frac{e^{2x}}{2}\,dx = 2xe^{2x} - e^{2x} + C_1.$$

So $e^{2x}y = (2x - 1)e^{2x} + C$, giving

$$y = 2x - 1 + Ce^{-2x}.$$

Verification: $y' = 2 - 2Ce^{-2x}$; $y' + 2y = 2 - 2Ce^{-2x} + 4x - 2 + 2Ce^{-2x} = 4x$. Correct.

## Worked Example 2

Solve $xy' - y = x^2\sin x$, $y(\pi) = 0$.

First write in standard form: $y' - (1/x)y = x\sin x$. The integrating factor is $\mu = e^{-\int (1/x)\,dx} = e^{-\ln x} = 1/x$ (assuming $x > 0$). Multiplying: $(y/x)' = \sin x$. Integrating: $y/x = -\cos x + C$, so $y = -x\cos x + Cx$. Imposing $y(\pi) = 0$: $0 = -\pi\cos\pi + C\pi = \pi + C\pi$, giving $C = -1$. The solution is

$$y = -x\cos x - x = -x(\cos x + 1).$$

## Worked Example 3: Variable Coefficients

Solve $y' + (\cos x)y = \cos x$.

Integrating factor: $\mu = e^{\sin x}$. Multiplying: $(e^{\sin x}y)' = e^{\sin x}\cos x$. The right side is $e^{\sin x}\cos x = \frac{d}{dx}e^{\sin x}$. Integrating: $e^{\sin x}y = e^{\sin x} + C$, so

$$y = 1 + Ce^{-\sin x}.$$

This is the general solution. As $x \to \pm\infty$, $\sin x$ oscillates, so $e^{-\sin x}$ oscillates between $e^{-1}$ and $e^1$, and the solution oscillates around 1.

## The Formula and Its Structure

The general solution $y = e^{-\int p\,dx}\left(C + \int e^{\int p\,dx}\,q(x)\,dx\right)$ has a clear structure. The term $Ce^{-\int p\,dx}$ is the homogeneous solution (decaying, growing, or oscillating depending on $p$). The integral term is the particular solution arising from the forcing $q(x)$. Setting $C = 0$ gives the particular solution; adding the arbitrary multiple of the homogeneous solution gives the general solution.

For an IVP with $y(x_0) = y_0$, the constant is:

$$C = \mu(x_0)y_0 - \int_{x_0}^{x_0}\mu(t)q(t)\,dt = \mu(x_0)y_0,$$

and the solution takes the clean form

$$y(x) = \frac{1}{\mu(x)}\left[\mu(x_0)y_0 + \int_{x_0}^x \mu(t)q(t)\,dt\right].$$

This shows explicitly how the initial condition $y_0$ and the accumulated forcing $\int_{x_0}^x \mu(t)q(t)\,dt$ combine to produce the solution.
