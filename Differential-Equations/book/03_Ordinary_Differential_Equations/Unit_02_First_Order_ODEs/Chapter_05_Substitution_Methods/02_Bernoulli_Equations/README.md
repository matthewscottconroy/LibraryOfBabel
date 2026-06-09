# Bernoulli Equations

The **Bernoulli equation** is the first-order ODE

$$y' + p(x)\,y = q(x)\,y^n,$$

where $n$ is a real number, $n \neq 0, 1$. For $n = 0$ the equation is linear (already handled), and for $n = 1$ it is separable (since $y' = (q-p)y$ is separable). For other values of $n$, the $y^n$ term makes the equation nonlinear. The Bernoulli equation was studied by Jacob Bernoulli in 1695 and linearized by Leibniz shortly after.

## The Linearizing Substitution

The key observation is that multiplying the equation by $(1-n)y^{-n}$ produces a combination that becomes linear under the substitution $w = y^{1-n}$.

Multiply through by $(1-n)y^{-n}$:

$$(1-n)y^{-n}y' + (1-n)p(x)y^{1-n} = (1-n)q(x).$$

Since $w = y^{1-n}$, we have $w' = (1-n)y^{-n}y'$. Therefore:

$$w' + (1-n)p(x)\,w = (1-n)q(x).$$

This is a linear first-order equation for $w$, solvable by the integrating factor method.

## Worked Example 1

Solve $y' - y = xy^2$ (Bernoulli with $n = 2$).

Here $p = -1$, $q = x$, $n = 2$. Set $w = y^{1-2} = y^{-1} = 1/y$. Then $w' = -y^{-2}y'$, so multiplying the equation by $-y^{-2}$: $-y^{-2}y' + y^{-1} = -x$, giving $w' + w = -x$. Solve by integrating factor $\mu = e^x$:

$(e^x w)' = -xe^x$. Integrating by parts: $\int -xe^x\,dx = -xe^x + e^x + C = (1-x)e^x + C$. So $e^x w = (1-x)e^x + C$, giving $w = (1-x) + Ce^{-x}$. Since $y = 1/w$:

$$y = \frac{1}{(1-x) + Ce^{-x}}.$$

## Worked Example 2: Logistic via Bernoulli

The logistic equation $P' = rP - \frac{r}{K}P^2$ can be written as $P' - rP = -\frac{r}{K}P^2$. This is Bernoulli with $n = 2$, $p = -r$, $q = -r/K$. Setting $w = P^{-1}$:

$$w' + rw = \frac{r}{K}.$$

Integrating factor $\mu = e^{rt}$: $(e^{rt}w)' = (r/K)e^{rt}$, so $e^{rt}w = (1/K)e^{rt} + C$, giving $w = 1/K + Ce^{-rt}$. Therefore

$$P = \frac{1}{w} = \frac{1}{1/K + Ce^{-rt}} = \frac{K}{1 + CKe^{-rt}}.$$

Imposing $P(0) = P_0$: $CK = K/P_0 - 1 = (K - P_0)/P_0$. This recovers the logistic formula derived by separation, confirming both approaches.

## General $n < 0$: Negative Exponents

For $n = -1$: $y' + p(x)y = q(x)y^{-1}$. Setting $w = y^2$ gives $w' + 2p(x)w = 2q(x)$, a linear equation. The equation for $n = -1$ arose in the theory of curves and in thermodynamics.

For general negative $n$: the equation $y' + py = qy^n$ with $n < 0$ has the $y^n$ term with $n < 0$, so $y^n$ diverges as $y \to 0$. The substitution $w = y^{1-n}$ still works, giving a linear equation for $w$.

## Physical Applications

The Bernoulli equation models numerous physical phenomena. The equation for a falling body with air resistance proportional to $v^2$: $m\dot{v} = mg - kv^2$ can be written as $\dot{v} - (g/v_t^2)v^2 = g$ (where $v_t$ is terminal velocity), which for some formulations becomes Bernoulli. More directly, the equation governing the growth of an organism with Gompertz kinetics or certain types of enzyme kinetics involves Bernoulli or related nonlinear structures.

The key lesson is that the substitution $w = y^{1-n}$, while mechanical in its application, represents a genuine algebraic insight: the nonlinearity of the Bernoulli equation is precisely the nonlinearity that can be removed by a power-law change of variables.
