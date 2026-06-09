# Chapter 5: The Inverse Laplace Transform

After transforming an ODE and solving algebraically for $Y(s)$, one must invert the transform to recover $y(t)$. The inverse Laplace transform requires identifying $Y(s)$ in terms of known transforms via partial fractions, completing the square, or using table entries together with operational properties. For more general situations, the Bromwich integral provides a contour integral formula valid in principle for any function in the appropriate class.

## Partial Fractions: The Primary Method

For rational $Y(s) = P(s)/Q(s)$ (where $\deg P < \deg Q$), partial fraction decomposition breaks $Y$ into a sum of simple fractions, each recognizable from the basic table. The four types of partial fraction terms are:
- $A/(s-r)$ for a real simple root $r$ of $Q$, inverse $Ae^{rt}$.
- $A/(s-r)^k$ for a real root of multiplicity $k$, inverse $At^{k-1}e^{rt}/(k-1)!$.
- $(As+B)/((s-\alpha)^2+\beta^2)$ for complex conjugate roots $\alpha\pm\beta i$, inverse involving $e^{\alpha t}\cos\beta t$ and $e^{\alpha t}\sin\beta t$.
- Higher-multiplicity complex roots give additional polynomial factors of $t$.

## Table Methods and Operational Properties

The first shifting theorem, completing the square, and differentiation/integration of transforms extend the table inversions to cover the full exponential-polynomial-trigonometric class. The second shifting theorem handles the $e^{-as}$ factors arising from delayed or piecewise forcing.

## The Bromwich Integral

The rigorous inversion formula is the **Bromwich integral** (Mellin's inverse):

$$f(t) = \frac{1}{2\pi i}\int_{\gamma - i\infty}^{\gamma + i\infty}e^{st}F(s)\,ds,$$

where $\gamma > c$ is to the right of all singularities of $F(s)$. This contour integral is evaluated by residue methods (closing the contour to the left with a semicircle in the half-plane $\text{Re}(s) < \gamma$): the result is $f(t) = \sum_{\text{poles}}[\text{residues of }e^{st}F(s)]$. This recovers the partial-fraction inversion formula as a special case and provides the foundation for the theoretical study of the Laplace transform in complex analysis.
