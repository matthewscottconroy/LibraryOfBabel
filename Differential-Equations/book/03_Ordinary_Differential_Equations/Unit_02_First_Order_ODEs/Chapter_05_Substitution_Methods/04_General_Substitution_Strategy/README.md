# General Substitution Strategy

Beyond the specific substitutions for homogeneous, Bernoulli, and Riccati equations, there is a broader class of situations where a well-chosen change of variables transforms an unfamiliar ODE into a solvable form. Developing the ability to recognize productive substitutions is as much an art as a science, but several patterns recur frequently.

## Equations of the Form $y' = f(ax + by + c)$

If the right-hand side depends on $x$ and $y$ only through a specific linear combination $u = ax + by + c$, the substitution $u = ax + by + c$ (with $b \neq 0$) leads to a separable equation.

Computing $u' = a + by'$, so $y' = (u' - a)/b$. The equation $y' = f(ax + by + c)$ becomes $(u' - a)/b = f(u)$, or $u' = a + bf(u)$, which is separable (since it is autonomous in $u$).

**Example.** Solve $y' = (x + y + 1)^2$.

Let $u = x + y + 1$, so $u' = 1 + y'$. Then $u' - 1 = u^2$, giving $u' = 1 + u^2$. Separating: $du/(1 + u^2) = dx$, so $\arctan(u) = x + C$, giving $u = \tan(x + C)$. Therefore $x + y + 1 = \tan(x + C)$, so $y = \tan(x + C) - x - 1$.

## Equations Involving $y/x$ and $y'$ in Symmetric Form

Some equations can be manipulated into the form $g(y/x)\,dx + h(y/x)\,dy = 0$, suggesting the homogeneous substitution. Others are more naturally addressed by recognizing an exact structure or an integrating factor.

## The Substitution $y = xv(x)$ in General

For any equation that is symmetric under scaling $(x, y) \to (\lambda x, \lambda y)$, the substitution $y = xv$ converts it to an equation for $v(x)$ that may be separable or otherwise simpler. Homogeneous equations (in the degree-0 sense) are the primary example, but the idea extends to equations with fractional homogeneity or with a change in the scaling symmetry.

## Inverse of the Unknown

For equations where $x$ is a complicated function of $y$ but $y$ is a simple function of $x$ is hard to find, interchanging the roles of $x$ and $y$ can help. Writing $dx/dy = 1/(dy/dx) = 1/f(x,y)$, the equation $y' = f(x,y)$ becomes $x' = 1/f(x,y)$ where differentiation is with respect to $y$. For equations of the form $y' = g(y)/h(x,y)$, the substitution $x$ as a function of $y$ may lead to a linear equation in $x$.

**Example.** The equation $y' = y/(y^2 - x)$ is awkward for $y$ as a function of $x$ but becomes linear when $x$ is a function of $y$: $dx/dy = (y^2 - x)/y = y - x/y$, or $x' + x/y = y$ (a linear ODE for $x$ as a function of $y$). Integrating factor $\mu = e^{\int dy/y} = y$: $(yx)' = y^2$, so $xy = y^3/3 + C$, giving $x = y^2/3 + C/y$.

## Recognizing Exact Differentials

Sometimes, groups of terms in an equation combine to form a recognizable exact differential:

$$d(xy) = y\,dx + x\,dy, \qquad d(x^2 + y^2) = 2x\,dx + 2y\,dy, \qquad d(y/x) = (x\,dy - y\,dx)/x^2.$$

Recognizing these patterns can reveal an implicit structure that makes the equation immediately integrable. For instance, $(x^2y + y^3)\,dx + (x^3 + xy^2)\,dy = xy(x\,dy + y\,dx) + y^2\cdot ...$; grouping may reveal $d(x^2y^2/2)$ or similar.

## The Philosophy of Substitutions

The general principle is that a substitution is productive if it:
1. Eliminates a nonlinearity (Bernoulli, Riccati).
2. Exploits a symmetry of the equation (homogeneity, periodicity).
3. Reduces the number of independent variables (the $u = ax + by$ substitution).
4. Converts a non-standard equation into a standard one (exact, linear, separable).

There is no algorithm that works for all equations; some ODEs have no closed-form solution at all, and the task shifts to numerical or qualitative analysis. But the ability to survey an equation, identify its structural features, and select the right transformation is the core skill of this chapter and of classical ODE theory.

## A Systematic Checklist

When facing an unfamiliar first-order ODE:
1. Is it separable? (Does $f(x,y)$ factor as $g(x)h(y)$?)
2. Is it linear? ($y'$ and $y$ appear only to the first power with $x$-dependent coefficients.)
3. Is it exact? (Check $M_y = N_x$.)
4. Is it homogeneous of degree 0? (Does $f(\lambda x, \lambda y) = f(x,y)$?)
5. Is it Bernoulli? ($y' + py = qy^n$.)
6. Is it Riccati? ($y' = p + qy + ry^2$, possibly with a known particular solution.)
7. Does the right side depend on $x$ and $y$ through a single combination $u = ax + by$?
8. Can the roles of $x$ and $y$ be interchanged productively?
9. Can an integrating factor (depending on $x$ alone, $y$ alone, or $xy$) make it exact?

Working through this checklist systematically converts the search for a substitution from guesswork into a structured diagnostic process.
