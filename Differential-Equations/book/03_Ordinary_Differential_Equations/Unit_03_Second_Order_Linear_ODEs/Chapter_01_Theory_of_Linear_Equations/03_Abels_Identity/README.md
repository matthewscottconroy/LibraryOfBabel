# Abel's Identity

Abel's identity gives a remarkable formula for the Wronskian of any two solutions of a second-order linear equation, expressing it in terms of the coefficient $p(x)$ without requiring explicit knowledge of the solutions themselves. It is one of the elegant universal results of ODE theory.

## The Theorem

**Theorem (Abel's Identity).** Let $y_1$ and $y_2$ be any two solutions of

$$y'' + p(x)\,y' + q(x)\,y = 0$$

on an interval $I$ where $p$ and $q$ are continuous. Then their Wronskian satisfies

$$W(x) = W(y_1, y_2)(x) = W(x_0)\,\exp\!\left(-\int_{x_0}^x p(t)\,dt\right)$$

for any $x_0 \in I$.

## Proof

Differentiate the Wronskian $W = y_1 y_2' - y_1' y_2$:

$$W' = y_1' y_2' + y_1 y_2'' - y_1'' y_2 - y_1' y_2' = y_1 y_2'' - y_1'' y_2.$$

Since $y_1$ and $y_2$ are solutions:

$$y_1'' = -p\,y_1' - q\,y_1, \qquad y_2'' = -p\,y_2' - q\,y_2.$$

Substitute:

$$W' = y_1(-p\,y_2' - q\,y_2) - (-p\,y_1' - q\,y_1)y_2 = -p(y_1 y_2' - y_1' y_2) = -p\,W.$$

So $W$ satisfies the first-order linear equation $W' = -p(x)W$, which separates to give

$$W(x) = W(x_0)\,e^{-\int_{x_0}^x p(t)\,dt}.$$

## Consequences

**Wronskian is never zero or always zero.** Since the exponential factor is always positive, $W(x)$ and $W(x_0)$ have the same sign for all $x$. Therefore either $W \equiv 0$ (when $W(x_0) = 0$) or $W$ is never zero.

**Formula for the Wronskian without solving.** Given the equation $y'' + p(x)y' + q(x)y = 0$, the Wronskian of any fundamental set evaluated at $x$ is

$$W(x) = Ce^{-\int p(x)\,dx}$$

for some constant $C \neq 0$ (determined by the initial Wronskian of the chosen fundamental set). This is determined entirely by $p(x)$, not by $q(x)$.

**Example.** For $y'' + 2y' + y = 0$ (with $p = 2$): the Wronskian of any fundamental set is $W(x) = W(x_0)e^{-2(x-x_0)}$. The specific fundamental set $\{e^{-x}, xe^{-x}\}$ has $W = e^{-x}\cdot(e^{-x} - xe^{-x}) - (-e^{-x})\cdot xe^{-x} = e^{-2x} - xe^{-2x} + xe^{-2x} = e^{-2x}$. Indeed, $e^{-2x} = e^{-2(x-0)}$ with $W(0) = 1$.

## Extension to Higher Order

For an $n$-th order equation $y^{(n)} + p_{n-1}y^{(n-1)} + \cdots + p_0 y = 0$, the generalized Abel identity gives

$$W(x) = W(x_0)\exp\!\left(-\int_{x_0}^x p_{n-1}(t)\,dt\right).$$

The Wronskian depends only on the coefficient of $y^{(n-1)}$, not on the lower-order coefficients.

## Applications

Abel's identity is used to:
1. Verify the Wronskian of a computed fundamental set by a shortcut.
2. Determine whether two given solutions are linearly independent without computing $W$ explicitly: check $W(x_0)$ at one convenient point.
3. In the variation of parameters formula: the Wronskian appears in the denominator, and Abel's identity gives its explicit value.
4. Prove the Liouville formula for the determinant of the fundamental matrix of a first-order linear system $\mathbf{x}' = A(t)\mathbf{x}$: $\det\Phi(t) = \det\Phi(t_0)\exp\!\left(\int_{t_0}^t \mathrm{tr}\,A(s)\,ds\right)$, which is the matrix analog of Abel's identity.

Abel's identity reveals a profound asymmetry in the roles of $p$ and $q$ in the equation: $p$ determines the growth or decay of the Wronskian (and hence the rate at which solutions become more or less "different" from each other), while $q$ determines the oscillatory or monotone character of individual solutions without affecting their linear independence.
