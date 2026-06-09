# Higher-Order Extensions

The characteristic equation method extends naturally from second-order to $n$-th order constant-coefficient linear equations. The $n$-th order equation

$$a_n y^{(n)} + a_{n-1}y^{(n-1)} + \cdots + a_1 y' + a_0 y = 0$$

has characteristic polynomial $p(r) = a_n r^n + a_{n-1}r^{n-1} + \cdots + a_1 r + a_0$, and the general solution is built from the roots of $p(r) = 0$ by the same rule applied to each root according to its multiplicity and type.

## The Rule for Each Root

For a real root $r_j$ of multiplicity $m_j$, the corresponding contribution to the general solution is

$$\left(c_1 + c_2 x + c_3 x^2 + \cdots + c_{m_j}x^{m_j-1}\right)e^{r_j x}.$$

This $m_j$-dimensional family accounts for the multiplicity.

For a pair of complex conjugate roots $\alpha \pm \beta i$, each with multiplicity $m$, the contribution is

$$e^{\alpha x}\left[(A_1 + A_2 x + \cdots + A_m x^{m-1})\cos\beta x + (B_1 + B_2 x + \cdots + B_m x^{m-1})\sin\beta x\right].$$

## Why Polynomial Factors for Repeated Roots

As with the second-order case, repeated roots arise when $p(r_j) = 0$ and also $p'(r_j) = 0$, $\ldots$, $p^{(m_j - 1)}(r_j) = 0$ (the root has multiplicity $m_j$). One can verify that for each $k = 0, 1, \ldots, m_j - 1$, the function $x^k e^{r_j x}$ is a solution: $L[x^k e^{r_j x}] = e^{r_j x}\sum_{\ell=0}^k \binom{k}{\ell}\frac{k!}{(k-\ell)!}x^{k-\ell}\frac{p^{(\ell)}(r_j)}{\ell!} = 0$ since $p^{(\ell)}(r_j) = 0$ for $\ell < m_j$.

## Total Dimension

The fundamental theorem of algebra guarantees that the characteristic polynomial of degree $n$ has exactly $n$ roots (counted with multiplicity) in $\mathbb{C}$. The corresponding solution family has exactly $n$ linearly independent real solutions (complex conjugate roots each contribute twice their multiplicity in real solutions), giving an $n$-dimensional solution space. This confirms that the general solution of an $n$-th order linear ODE has exactly $n$ arbitrary constants.

## Worked Example: Third Order

Solve $y''' - y'' - y' + y = 0$.

Factor the characteristic polynomial: $r^3 - r^2 - r + 1 = r^2(r-1) - (r-1) = (r^2 - 1)(r - 1) = (r+1)(r-1)^2$.

Roots: $r = -1$ (simple) and $r = 1$ (double). General solution:

$$y = c_1 e^{-x} + (c_2 + c_3 x)e^x.$$

## Worked Example: Fourth Order with Complex Roots

Solve $y^{(4)} + 2y'' + y = 0$.

Characteristic equation: $r^4 + 2r^2 + 1 = (r^2 + 1)^2 = 0$. Roots: $r = \pm i$, each with multiplicity 2.

General solution:

$$y = (c_1 + c_2 x)\cos x + (c_3 + c_4 x)\sin x.$$

This equation arises in beam theory (the Euler-Bernoulli beam equation for a uniform elastic beam), where solutions with both $\cos x$ and $x\cos x$ terms describe specific bending modes.

## Practical Considerations

For equations of order higher than 4, finding the roots of the characteristic polynomial generally requires numerical methods (such as Newton's method or eigenvalue computation). For equations of low order (2, 3, 4), rational root testing and factoring often succeed. The cubic and quartic formulae exist in principle but are rarely used in practice because of their algebraic complexity. The important theoretical point, that the solution space is finite-dimensional and spanned by exponential-polynomial solutions, remains valid for all orders.
