# Integrating Factors for Exactness

If $M\,dx + N\,dy = 0$ is not exact, it may be possible to find a function $\mu(x, y)$ such that $\mu M\,dx + \mu N\,dy = 0$ is exact. Such a function is an **integrating factor** for exactness. Finding integrating factors is an art, guided by a systematic condition and a collection of special cases.

## The Condition

Multiplying by $\mu$, the new equation has $\tilde{M} = \mu M$ and $\tilde{N} = \mu N$. Exactness requires $\partial(\mu M)/\partial y = \partial(\mu N)/\partial x$, i.e.,

$$\mu_y M + \mu M_y = \mu_x N + \mu N_x.$$

This is a partial differential equation for $\mu$, which in general is harder to solve than the original ODE. However, special assumptions about the form of $\mu$ lead to tractable ordinary differential equations.

## Integrating Factor Depending Only on $x$

Assume $\mu = \mu(x)$. Then $\mu_y = 0$, and the condition becomes $\mu M_y = \mu_x N + \mu N_x$, or

$$\mu_x = \mu\,\frac{M_y - N_x}{N} \implies \frac{d\mu}{\mu} = \frac{M_y - N_x}{N}\,dx.$$

This ODE for $\mu$ has a solution if and only if $(M_y - N_x)/N$ depends only on $x$. When it does, $\mu(x) = e^{\int (M_y - N_x)/N\,dx}$.

## Integrating Factor Depending Only on $y$

Assume $\mu = \mu(y)$. Then $\mu_x = 0$, and the condition becomes $\mu_y M = \mu(N_x - M_y)$, or

$$\frac{d\mu}{\mu} = \frac{N_x - M_y}{M}\,dy.$$

This has a solution if and only if $(N_x - M_y)/M$ depends only on $y$.

## Worked Example

Solve $(xy^2 + y)\,dx + (x^2 y)\,dy = 0$.

Check: $M = xy^2 + y$, $N = x^2 y$. $M_y = 2xy + 1$, $N_x = 2xy$. $M_y - N_x = 1 \neq 0$, so not exact.

Test for $\mu(x)$: $(M_y - N_x)/N = 1/(x^2 y)$, which depends on $y$ as well as $x$. So $\mu \neq \mu(x)$ alone.

Test for $\mu(y)$: $(N_x - M_y)/M = -1/(xy^2 + y) = -1/[y(xy + 1)]$. This depends on $x$ as well. Try $\mu = \mu(xy)$ or other special forms.

Try $\mu = 1/y^2$: $\tilde{M} = x + 1/y$, $\tilde{N} = x^2/y$. $\partial\tilde{M}/\partial y = -1/y^2$ and $\partial\tilde{N}/\partial x = 2x/y$. These are not equal, so $\mu = 1/y^2$ does not work.

Try $\mu = 1/y$: $\tilde{M} = xy + 1$, $\tilde{N} = x^2$. $\partial\tilde{M}/\partial y = x$ and $\partial\tilde{N}/\partial x = 2x$. Not equal.

Try $\mu = 1/(xy^2)$: $\tilde{M} = 1 + 1/(xy)$, $\tilde{N} = x/y$. $\partial\tilde{M}/\partial y = -1/(xy^2)$ and $\partial\tilde{N}/\partial x = 1/y$. Not equal.

In practice, finding integrating factors for arbitrary equations requires experimentation or a computer algebra system. For this example, the equation $xy^2\,dx + x^2y\,dy = d(x^2y^2/2) = 0$ and $y\,dx = d(xy) - x\,dy$... the search can be complex. The method is most tractable when the equation has special structure.

## Linear Equations as a Special Case

The integrating factor for the linear equation $y' + p(x)y = q(x)$, written as $(p(x)y - q(x))\,dx + dy = 0$ with $M = py - q$ and $N = 1$, satisfies $(M_y - N_x)/N = p(x)$, which depends only on $x$. The integrating factor is $\mu(x) = e^{\int p\,dx}$, exactly as derived in Chapter 3. So the integrating factor method for linear equations is the special case of the exactness integrating factor where $\mu$ depends only on $x$.

## Other Special Forms

Other productive assumptions include $\mu = x^m y^n$ (power-law integrating factors) and $\mu = f(xy)$ or $\mu = f(x^2 + y^2)$. For each assumption, the exactness condition reduces to a specific equation for the unknown function, which may or may not have a simple solution.

The general theory of integrating factors is extensive but not always practical: knowing that an integrating factor exists (which it always does, by theory) does not tell you how to find it. The value of the method lies in the tractable special cases, which cover a useful but limited range of equations.
