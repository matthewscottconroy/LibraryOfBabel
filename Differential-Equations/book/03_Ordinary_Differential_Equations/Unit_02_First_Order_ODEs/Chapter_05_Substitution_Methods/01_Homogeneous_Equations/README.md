# Homogeneous Equations

A first-order ODE is called **homogeneous** (in the sense of homogeneity of degree zero) if it can be written as

$$\frac{dy}{dx} = f\!\left(\frac{y}{x}\right)$$

for some function $f$. The right side depends on $y$ and $x$ only through their ratio $v = y/x$, making the equation scale-invariant: replacing $(x, y)$ by $(\lambda x, \lambda y)$ leaves the right side unchanged.

## Recognition

An equation $dy/dx = F(x, y)$ is homogeneous in this sense if $F(\lambda x, \lambda y) = F(x, y)$ for all $\lambda > 0$, i.e., $F$ is a homogeneous function of degree 0 in $(x, y)$.

Equivalently, write the ODE as $M(x,y)\,dx + N(x,y)\,dy = 0$. If $M$ and $N$ are both homogeneous of the same degree, the equation is homogeneous (degree 0 after dividing through).

**Examples.** $dy/dx = (x^2 + xy)/y^2$: numerator has degree 2 and denominator has degree 2, so the ratio is degree 0. This is homogeneous. Writing $v = y/x$: $y = vx$, $y^2 = v^2x^2$, $x^2 + xy = x^2 + x\cdot vx = x^2(1+v)$, so $dy/dx = (1+v)/v^2$.

## The Substitution

Set $v = y/x$, so $y = vx$ and $y' = v + xv'$. The equation $y' = f(y/x) = f(v)$ becomes

$$v + xv' = f(v) \implies xv' = f(v) - v \implies \frac{dv}{f(v) - v} = \frac{dx}{x}.$$

This is separable in $v$ and $x$. Integrating both sides gives an implicit relation between $v$ and $x$, and substituting back $v = y/x$ gives the general solution implicitly in $x$ and $y$.

## Worked Example 1

Solve $x\,dy - y\,dx = \sqrt{x^2 + y^2}\,dx$, i.e., $dy/dx = (y + \sqrt{x^2 + y^2})/x = y/x + \sqrt{1 + (y/x)^2}$.

Set $v = y/x$: $v + xv' = v + \sqrt{1 + v^2}$, so $xv' = \sqrt{1 + v^2}$, giving $dv/\sqrt{1+v^2} = dx/x$. Integrating: $\ln(v + \sqrt{1+v^2}) = \ln|x| + C_1$, so $v + \sqrt{1+v^2} = Ax$ (where $A = e^{C_1}$). Substituting $v = y/x$:

$$\frac{y}{x} + \sqrt{1 + \frac{y^2}{x^2}} = Ax \implies y + \sqrt{x^2 + y^2} = Ax^2.$$

## Worked Example 2

Solve $y' = (y^2 - xy)/(x^2)$ with $y(1) = 1$.

Write $y' = (y/x)^2 - (y/x)$. Setting $v = y/x$: $v + xv' = v^2 - v$, so $xv' = v^2 - 2v = v(v-2)$. Separating: $dv/[v(v-2)] = dx/x$. Partial fractions: $1/[v(v-2)] = \frac{1}{2}\left[\frac{-1}{v} + \frac{1}{v-2}\right]$. Integrating:

$$\frac{1}{2}\ln\left|\frac{v-2}{v}\right| = \ln|x| + C_1 \implies \left|\frac{v-2}{v}\right| = Ax^2.$$

Imposing $y(1) = 1$, so $v(1) = y(1)/1 = 1$: $|(1-2)/1| = A\cdot 1$, giving $A = 1$. So $(v-2)/v = \pm x^2$. Since $v(1) = 1$: $(1-2)/1 = -1$ and $\pm x^2|_{x=1} = \pm 1$. Taking the minus sign: $(v-2)/v = -x^2$. Solving: $v - 2 = -vx^2$, so $v(1 + x^2) = 2$, giving $v = 2/(1+x^2)$. Therefore $y = vx = 2x/(1+x^2)$.

Verification: $y' = (2(1+x^2) - 2x\cdot 2x)/(1+x^2)^2 = (2 - 2x^2)/(1+x^2)^2$. And $(y^2 - xy)/x^2 = (4x^2/(1+x^2)^2 - 2x^2/(1+x^2))/x^2 = 4/(1+x^2)^2 - 2/(1+x^2) = (4 - 2(1+x^2))/(1+x^2)^2 = (2-2x^2)/(1+x^2)^2$. Correct.

## Non-Homogeneous of the Form $y' = f\big(\frac{ax+by+c}{dx+ey+f}\big)$

If $c \neq 0$ or $f \neq 0$, the equation is not translation-invariant. When the lines $ax + by + c = 0$ and $dx + ey + f = 0$ intersect at $(x_0, y_0)$, the substitution $X = x - x_0$, $Y = y - y_0$ shifts the origin to the intersection, giving $dY/dX = f((aX+bY)/(dX+eY))$, which is homogeneous. When the lines are parallel ($ae = bd$), a different substitution $u = ax + by$ is appropriate.
