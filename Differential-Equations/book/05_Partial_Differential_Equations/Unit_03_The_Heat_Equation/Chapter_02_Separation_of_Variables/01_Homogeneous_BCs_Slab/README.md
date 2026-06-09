# Homogeneous Boundary Conditions: The Heat Equation on a Slab

The simplest and most fundamental application of separation of variables is the heat equation on a one-dimensional slab $[0,L]$ with homogeneous Dirichlet boundary conditions. This problem is solvable in complete generality for arbitrary initial data, and its solution exhibits all the key features of parabolic equations: infinite series representations, exponential decay, smoothing of initial data, and eventual approach to the trivial equilibrium.

## The Problem

We seek the temperature $u(x,t)$ satisfying:

$$u_t = \kappa\,u_{xx}, \qquad 0 < x < L,\; t > 0, \tag{1}$$
$$u(0,t) = 0, \qquad u(L,t) = 0, \qquad t > 0, \tag{2}$$
$$u(x,0) = f(x), \qquad 0 < x < L. \tag{3}$$

The boundary conditions (2) say the endpoints of the slab are maintained at zero temperature. The initial condition (3) specifies the initial temperature profile.

## Step 1: Separation of Variables

Seek solutions of the form $u(x,t) = X(x)T(t)$. Substituting into (1):

$$X(x)T'(t) = \kappa X''(x)T(t) \implies \frac{T'(t)}{\kappa T(t)} = \frac{X''(x)}{X(x)} = -\lambda,$$

where $-\lambda$ is the separation constant (the sign is chosen to anticipate that $\lambda > 0$ for non-trivial solutions decaying in time). This gives two ODEs:

$$T' + \kappa\lambda T = 0, \tag{4}$$
$$X'' + \lambda X = 0, \quad X(0) = X(L) = 0. \tag{5}$$

The boundary conditions (2) require $X(0)T(t) = 0$ and $X(L)T(t) = 0$ for all $t > 0$, so $X(0) = X(L) = 0$.

## Step 2: Solving the Eigenvalue Problem

The Sturm-Liouville problem (5) is: find all values of $\lambda$ for which there exists a nontrivial solution $X$.

- If $\lambda < 0$: the general solution of $X'' + \lambda X = 0$ is $X = Ae^{\sqrt{-\lambda}x} + Be^{-\sqrt{-\lambda}x}$ (or equivalently, $X = A\cosh(\mu x) + B\sinh(\mu x)$ with $\mu = \sqrt{-\lambda}$). Applying $X(0) = 0$ gives $A = 0$, and $X(L) = 0$ gives $B\sinh(\mu L) = 0$, so $B = 0$. Only the trivial solution.

- If $\lambda = 0$: $X'' = 0$ gives $X = Ax + B$. Boundary conditions give $B = 0$ and $AL = 0$, so $A = 0$. Only the trivial solution.

- If $\lambda > 0$: $X = A\cos(\sqrt{\lambda}x) + B\sin(\sqrt{\lambda}x)$. From $X(0) = 0$: $A = 0$. From $X(L) = 0$: $B\sin(\sqrt{\lambda}L) = 0$. For a nontrivial solution, $\sin(\sqrt{\lambda}L) = 0$, i.e., $\sqrt{\lambda}L = n\pi$ for $n = 1, 2, 3, \ldots$

The **eigenvalues** are:

$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \qquad n = 1, 2, 3, \ldots$$

The corresponding **eigenfunctions** are:

$$X_n(x) = \sin\!\left(\frac{n\pi x}{L}\right).$$

## Step 3: Solving for $T(t)$

With $\lambda = \lambda_n$, equation (4) gives:

$$T_n(t) = e^{-\kappa\lambda_n t} = e^{-\kappa(n\pi/L)^2 t}.$$

## Step 4: Superposition

Each product $u_n(x,t) = \sin(n\pi x/L)\,e^{-\kappa(n\pi/L)^2 t}$ satisfies (1) and (2). By superposition (linearity), so does any finite sum, and under appropriate convergence conditions, also the infinite series:

$$u(x,t) = \sum_{n=1}^\infty b_n\,\sin\!\left(\frac{n\pi x}{L}\right)e^{-\kappa(n\pi/L)^2 t}. \tag{6}$$

## Step 5: Applying the Initial Condition

At $t=0$, (6) gives $u(x,0) = \sum_{n=1}^\infty b_n\sin(n\pi x/L) = f(x)$. This requires $f$ to be represented as a Fourier sine series on $[0,L]$. By the orthogonality of the sine functions:

$$\int_0^L \sin\!\left(\frac{m\pi x}{L}\right)\sin\!\left(\frac{n\pi x}{L}\right)dx = \begin{cases} 0 & m \neq n \\ L/2 & m = n \end{cases},$$

we obtain the **Fourier coefficients**:

$$b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx. \tag{7}$$

The solution is completely determined: (6) with coefficients (7).

## Convergence and Regularity

For $f \in L^2(0,L)$, the series (6) converges in $L^2$ for each $t > 0$. For $t > 0$, the exponential factors $e^{-\kappa(n\pi/L)^2 t}$ decay faster than any polynomial in $n$, so the series converges absolutely and uniformly, and defines a $C^\infty$ function of $(x,t)$ for $t > 0$, even if $f$ was only in $L^2$. This is the smoothing property: the heat equation instantly smooths any $L^2$ initial data.

**Theorem.** If $f \in L^2(0,L)$ and $b_n$ are given by (7), then the function $u$ defined by (6) satisfies the heat equation (1), the boundary conditions (2), and $u(\cdot,t) \to f$ in $L^2(0,L)$ as $t\to 0^+$. The solution is unique among functions in $C([0,L]\times(0,\infty))$ satisfying the initial and boundary conditions.

## Example: Step Function Initial Data

Let $L = \pi$, $\kappa = 1$, and $f(x) = 1$ for $0 < x < \pi$.

Then $b_n = \frac{2}{\pi}\int_0^\pi \sin(nx)\,dx = \frac{2}{\pi}\cdot\frac{1-\cos(n\pi)}{n} = \frac{2}{\pi}\cdot\frac{1-(-1)^n}{n}$.

So $b_n = 0$ for $n$ even, and $b_n = \frac{4}{n\pi}$ for $n$ odd. The solution is:

$$u(x,t) = \frac{4}{\pi}\sum_{k=0}^\infty \frac{1}{2k+1}\sin((2k+1)x)\,e^{-(2k+1)^2 t}.$$

At $t=0$: the series is the Fourier sine series of the constant function $1$ on $(0,\pi)$ — a classical result. For $t > 0$: each mode decays exponentially, with higher modes (large $n$) decaying much faster. The solution smooths and decays to zero.

## Long-Time Behavior

For large $t$, the dominant term is the $n=1$ mode:

$$u(x,t) \approx b_1 \sin\!\left(\frac{\pi x}{L}\right)e^{-\kappa(\pi/L)^2 t}.$$

All higher modes become negligible compared to the fundamental mode. The decay rate $\kappa\lambda_1 = \kappa\pi^2/L^2$ is determined by the smallest eigenvalue $\lambda_1 = \pi^2/L^2$. A longer slab ($L$ larger) decays more slowly; higher diffusivity ($\kappa$ larger) accelerates decay. This behavior — convergence to the fundamental mode — is typical of parabolic equations: the long-time dynamics are governed by the spectral gap between the first and second eigenvalues.
