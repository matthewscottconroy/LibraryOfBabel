# Chapter 3: Laplace's Equation in Polar Coordinates

The disk $\{r < R\}$ is the most symmetric two-dimensional domain, and the natural coordinate system for it is polar coordinates $(r,\theta)$. Separation of variables in polar coordinates produces solutions of the form $r^n\cos(n\theta)$ and $r^n\sin(n\theta)$ — the harmonic polynomials — and $r^{-n}\cos(n\theta)$ and $r^{-n}\sin(n\theta)$ — decaying harmonics for exterior domains. The Dirichlet problem on the disk has an explicit closed-form solution: the Poisson integral formula, which expresses the value of a harmonic function in the interior of the disk as a weighted average of its boundary values.

## Laplacian in Polar Coordinates

In polar coordinates $x = r\cos\theta$, $y = r\sin\theta$:

$$\Delta u = u_{rr} + \frac{1}{r}u_r + \frac{1}{r^2}u_{\theta\theta} = \frac{1}{r}\frac{\partial}{\partial r}\!\left(r\frac{\partial u}{\partial r}\right) + \frac{1}{r^2}\frac{\partial^2 u}{\partial\theta^2}.$$

## Structure of This Chapter

**Section 1: Dirichlet Problem on the Disk** uses separation of variables $u = R(r)\Theta(\theta)$ to derive the general harmonic function on a disk:

$$u(r,\theta) = \frac{a_0}{2} + \sum_{n=1}^\infty r^n(a_n\cos n\theta + b_n\sin n\theta).$$

For a disk of radius $R$ with boundary data $u(R,\theta) = f(\theta)$, the coefficients are determined by the Fourier series of $f$. The resulting solution can be summed in closed form to yield the Poisson integral formula.

**Section 2: Poisson Integral Formula** gives the closed-form expression for the harmonic extension of boundary data $f(\theta)$ from $\partial B_R$ to $B_R$:

$$u(r,\theta) = \frac{R^2-r^2}{2\pi}\int_0^{2\pi}\frac{f(\phi)}{R^2 - 2Rr\cos(\theta-\phi) + r^2}\,d\phi.$$

The factor $(R^2-r^2)/(R^2 - 2Rr\cos(\theta-\phi)+r^2)$ is the **Poisson kernel** $P(r,\theta-\phi;R)$. It is positive and integrates to $1$ in $\phi$, making the formula a weighted average of $f$. As $r\to R$, the kernel concentrates near $\phi = \theta$ and the formula recovers $f(\theta)$.

**Section 3: Annular Domains** treats the Dirichlet problem on an annulus $a < r < b$ with prescribed data on both circles. The general harmonic function in an annulus includes both $r^n$ and $r^{-n}$ terms:

$$u(r,\theta) = \frac{a_0 + b_0\log r}{1} + \sum_{n=1}^\infty(a_n r^n + c_n r^{-n})(d_n\cos n\theta + e_n\sin n\theta).$$

The two sets of coefficients $a_n$, $c_n$ (for each $n$) are determined by data on the two boundary circles.

## Significance of the Poisson Formula

The Poisson integral formula is among the most important explicit formulas in analysis. It implies:
- **Existence:** the formula gives an explicit harmonic function with prescribed boundary data.
- **Smoothness:** the Poisson kernel is $C^\infty$ for $r < R$, so the solution is infinitely smooth in the interior.
- **Mean value property:** at $r=0$, $u(0) = \frac{1}{2\pi}\int_0^{2\pi}f(\phi)\,d\phi$ — the value at the center is the average of the boundary values.
- **Harnack estimates:** the explicit kernel bounds give quantitative Harnack-type inequalities.
