# The Dirichlet Problem on a Rectangle

The Dirichlet problem for Laplace's equation on a rectangle is the prototypical elliptic boundary value problem. It combines the general theory — uniqueness via the maximum principle, existence via the Perron method — with a completely explicit solution via separation of variables and Fourier series.

## The Problem

Find $u(x,y)$ harmonic in $\Omega = (0,a)\times(0,b)$ with prescribed boundary data:

$$u(0,y) = 0, \quad u(a,y) = 0, \quad u(x,0) = 0, \quad u(x,b) = f(x).$$

(By superposition, any boundary data can be decomposed into problems of this type.)

## Separation of Variables

Seek $u = X(x)Y(y)$ with $X'' + \mu X = 0$ and $Y'' - \mu Y = 0$. Boundary conditions on the vertical sides give $X(0) = X(a) = 0$, so $\mu = (n\pi/a)^2$ and $X_n = \sin(n\pi x/a)$ for $n = 1, 2, \ldots$

For each $n$, the $Y$-equation is $Y_n'' = (n\pi/a)^2 Y_n$, with $Y_n(0) = 0$. The solution is:

$$Y_n(y) = A_n\sinh\!\left(\frac{n\pi y}{a}\right).$$

(We use $\sinh$ rather than $e^{n\pi y/a}$ to satisfy $Y_n(0) = 0$ automatically.)

## Solution

By superposition:

$$u(x,y) = \sum_{n=1}^\infty A_n\sinh\!\left(\frac{n\pi y}{a}\right)\sin\!\left(\frac{n\pi x}{a}\right).$$

Applying $u(x,b) = f(x)$:

$$f(x) = \sum_{n=1}^\infty A_n\sinh\!\left(\frac{n\pi b}{a}\right)\sin\!\left(\frac{n\pi x}{a}\right).$$

So $A_n = \frac{b_n}{\sinh(n\pi b/a)}$ where $b_n = \frac{2}{a}\int_0^a f(x)\sin(n\pi x/a)\,dx$.

The **solution** is:

$$u(x,y) = \sum_{n=1}^\infty \frac{b_n}{\sinh(n\pi b/a)}\sinh\!\left(\frac{n\pi y}{a}\right)\sin\!\left(\frac{n\pi x}{a}\right). \tag{1}$$

## Example: $f(x) = u_0$ (constant)

The Fourier coefficients: $b_n = \frac{2u_0}{a}\int_0^a\sin(n\pi x/a)\,dx = \frac{2u_0}{\pi}\cdot\frac{1-(-1)^n}{n}$.

So $b_n = 0$ for even $n$ and $b_n = 4u_0/(n\pi)$ for odd $n$. The solution:

$$u(x,y) = \frac{4u_0}{\pi}\sum_{k=0}^\infty\frac{1}{2k+1}\cdot\frac{\sinh((2k+1)\pi y/a)}{\sinh((2k+1)\pi b/a)}\sin\!\left(\frac{(2k+1)\pi x}{a}\right).$$

For $a = b = \pi$ and $y$ close to $b = \pi$: the first term dominates (the series converges very rapidly due to $\sinh(n\pi b/a)$ growing exponentially with $n$).

## Four-Sided Problem

For the general problem with nonhomogeneous data on all four sides:

$$u(0,y) = g_1(y),\ u(a,y) = g_2(y),\ u(x,0) = h_1(x),\ u(x,b) = h_2(x),$$

write $u = u_1 + u_2 + u_3 + u_4$ where each $u_i$ handles one nonhomogeneous side with zeros on the others. Each $u_i$ is solved by formula (1) or its analogues (with roles of $x$ and $y$ swapped, and the sinh/sin functions rearranged accordingly).

This decomposition is valid because the heat equation is linear: the sum satisfies all four boundary conditions simultaneously.

## Convergence

For $f \in L^2(0,a)$, the series (1) converges in $L^2$ at $y=b$ by Parseval's theorem. For $0 < y < b$, the extra factor $\sinh(n\pi y/a)/\sinh(n\pi b/a) \sim e^{n\pi(y-b)/a} = e^{-n\pi(b-y)/a}$ decays exponentially in $n$ (for $y < b$), making the series converge absolutely and uniformly for $y$ bounded away from $b$.

This exponential convergence reflects the analyticity of the harmonic function in the interior: the series is essentially a Laurent/power series in $e^{n\pi z/a}$ (after changing to complex variable), and the exponential decay of coefficients ensures analyticity.

## Poisson's Equation on a Rectangle

The nonhomogeneous problem $\Delta u = f$ with homogeneous Dirichlet conditions is solved by expanding $u$ and $f$ in a double sine series:

$$u(x,y) = \sum_{m,n}a_{mn}\sin\!\left(\frac{m\pi x}{a}\right)\sin\!\left(\frac{n\pi y}{b}\right), \quad f(x,y) = \sum_{m,n}f_{mn}\sin\!\left(\frac{m\pi x}{a}\right)\sin\!\left(\frac{n\pi y}{b}\right).$$

Substituting: $-\left[(m\pi/a)^2 + (n\pi/b)^2\right]a_{mn} = f_{mn}$, so $a_{mn} = -f_{mn}/\lambda_{mn}$ where $\lambda_{mn} = (m\pi/a)^2 + (n\pi/b)^2 > 0$. This is always uniquely solvable (no resonance, since all eigenvalues are positive) — a key property of the elliptic problem.
