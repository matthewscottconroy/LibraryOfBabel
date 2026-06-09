# Chapter 3: Linear First-Order Equations

A first-order linear ODE $y' + p(x)y = q(x)$ admits a complete and elegant solution theory. Unlike separable equations, which require the right-hand side to factor in a special way, the linear structure provides a universal method, the integrating factor, that works for every equation of this form. The result is a closed-form solution formula expressed as an integral, one of the most useful formulas in applied mathematics.

## Chapter Contents

The chapter develops the theory of linear first-order equations in four stages. The first section introduces the standard form and explains why linearity is the defining structural property. The second develops the integrating factor method in full generality, proving that it converts every linear first-order equation into a direct integration. The third section presents variation of parameters as an alternative and conceptually distinct approach, which extends naturally to higher-order equations. The fourth section applies the methods to physically meaningful problems: mixing tanks, population models with immigration, and electrical circuits.

## The Integrating Factor Idea

The fundamental insight is that the left side of $y' + p(x)y$, while not immediately integrable, can be made into an exact derivative by multiplying by an appropriate function $\mu(x)$. When $\mu y' + \mu p(x) y = (\mu y)'$, which requires $\mu' = \mu p(x)$ and therefore $\mu = e^{\int p(x)\,dx}$, the equation becomes $(\mu y)' = \mu q(x)$, which integrates directly to give $\mu y = \int \mu q(x)\,dx + C$.

This device reduces a seemingly complex problem to a straightforward integration. The integrating factor technique is one of the few genuinely universal exact methods in ODE theory.

## Connections to Later Material

The linear first-order equation is the simplest model for all the phenomena that appear in higher-order linear theory. The homogeneous solution $y_h = Ce^{-\int p\,dx}$ is the kernel of the linear operator $L[y] = y' + p(x)y$. The particular solution $y_p = e^{-\int p\,dx}\int e^{\int p\,dx}q(x)\,dx$ is produced by variation of parameters. Both the structure (general = homogeneous + particular) and the method (variation of parameters) reappear for all higher-order linear equations.
