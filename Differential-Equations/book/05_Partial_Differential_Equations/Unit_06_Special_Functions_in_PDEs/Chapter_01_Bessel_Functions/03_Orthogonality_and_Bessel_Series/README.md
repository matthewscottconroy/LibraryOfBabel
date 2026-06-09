# Orthogonality and Bessel Series

The orthogonality of Bessel functions on a finite interval is the key ingredient for solving PDEs on disks and cylinders via separation of variables. Just as a function on $[0,L]$ is expanded in a Fourier sine series with respect to the orthogonal system $\{\sin(n\pi x/L)\}$ for the Dirichlet problem on a rectangle, a function on $[0,R]$ is expanded in a **Bessel-Fourier series** with respect to the orthogonal system $\{J_\nu(j_{\nu,n}r/R)\}$ for the Dirichlet problem on a disk. This section derives the orthogonality relation, computes the normalization constants, establishes completeness, and works through complete PDE examples.

## Orthogonality on $[0,R]$

**Theorem.** For $m \neq n$ and any $\nu \geq 0$:

$$\int_0^R J_\nu\!\left(\frac{j_{\nu,m}}{R}r\right) J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right) r\,dr = 0.$$

**Proof.** Let $\alpha = j_{\nu,m}/R$ and $\beta = j_{\nu,n}/R$ with $\alpha \neq \beta$. The functions $f = J_\nu(\alpha r)$ and $g = J_\nu(\beta r)$ satisfy:

$$\frac{d}{dr}\!\left(r\frac{df}{dr}\right) + \left(\alpha^2 r - \frac{\nu^2}{r}\right)f = 0, \qquad \frac{d}{dr}\!\left(r\frac{dg}{dr}\right) + \left(\beta^2 r - \frac{\nu^2}{r}\right)g = 0.$$

Multiply the first by $g$ and the second by $f$, subtract, and integrate from $0$ to $R$:

$$(\alpha^2-\beta^2)\int_0^R fg\,r\,dr = \left[r(fg'-gf')\right]_0^R.$$

At $r=0$: $fg' - gf' \to 0$ (both $J_\nu(\alpha r)$ and $J_\nu(\beta r)$ vanish as $r\to 0$ for $\nu > 0$; for $\nu=0$ the derivatives cancel). At $r=R$: $f(R) = J_\nu(\alpha R) = J_\nu(j_{\nu,m}) = 0$ and $g(R) = J_\nu(j_{\nu,n}) = 0$, so the bracket vanishes. Since $\alpha \neq \beta$, the integral is zero. $\square$

## Normalization

**Theorem.** The $L^2$ norm (with weight $r$) is:

$$\int_0^R \left[J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right)\right]^2 r\,dr = \frac{R^2}{2}\left[J_\nu'(j_{\nu,n})\right]^2 = \frac{R^2}{2}\left[J_{\nu+1}(j_{\nu,n})\right]^2.$$

**Proof.** Start from the formula with $\alpha = \beta$: setting $\alpha = \beta$ and passing to the limit (or using the differentiation identity) yields the self-inner-product. The key identity uses $\int_0^1 t[J_\nu(\lambda t)]^2\,dt = \frac{1}{2}\{[J_\nu'(\lambda)]^2 + (1-\nu^2/\lambda^2)[J_\nu(\lambda)]^2\}$. Since $J_\nu(\lambda) = 0$ (at a zero $\lambda = j_{\nu,n}$), the second term drops and only $[J_\nu'(\lambda)]^2/2$ remains. Scaling by $R^2$ gives the result. The equality $J_\nu'(j_{\nu,n}) = -J_{\nu+1}(j_{\nu,n})$ (from the recursion $(d/dx)[x^{-\nu}J_\nu] = -x^{-\nu}J_{\nu+1}$ at $x=j_{\nu,n}$ where $J_\nu = 0$) gives the second form. $\square$

**Table of normalization constants** $\|J_0(j_{0,n}r/R)\|^2 = \frac{R^2}{2}[J_1(j_{0,n})]^2$:

| $n$ | $j_{0,n}$ | $J_1(j_{0,n})$ | $\|J_0\|^2/R^2$ |
|---|---|---|---|
| 1 | 2.4048 | 0.5191 | 0.1348 |
| 2 | 5.5201 | 0.3403 | 0.0579 |
| 3 | 8.6537 | 0.2715 | 0.0369 |

## Bessel-Fourier Series

**Theorem (completeness).** For each $\nu \geq 0$, the functions $\{J_\nu(j_{\nu,n}r/R)\}_{n=1}^\infty$ form a complete orthogonal system in $L^2([0,R]; r\,dr)$. Every $f \in L^2([0,R]; r\,dr)$ has the expansion:

$$f(r) = \sum_{n=1}^\infty c_n J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right),$$

with convergence in $L^2([0,R]; r\,dr)$, where:

$$c_n = \frac{2}{R^2[J_{\nu+1}(j_{\nu,n})]^2}\int_0^R f(r)\, J_\nu\!\left(\frac{j_{\nu,n}}{R}r\right)r\,dr.$$

**Remark on pointwise convergence.** At a point $r_0$ where $f$ satisfies a Dini condition (e.g., $f$ is differentiable at $r_0$), the series converges to $f(r_0)$. At a jump discontinuity, the series converges to the average of the left and right limits — analogous to the behavior of Fourier series.

## Heat Equation on a Disk

**Problem.** Find $u(r,\theta,t)$ satisfying $u_t = \kappa\Delta u$ on the disk $r < R$, with $u(R,\theta,t) = 0$ and $u(r,\theta,0) = f(r,\theta)$.

**Solution.** Expand $f$ in a double series (Fourier in $\theta$, Bessel in $r$):

$$f(r,\theta) = \sum_{m=-\infty}^\infty\sum_{n=1}^\infty A_{mn} J_{|m|}\!\left(\frac{j_{|m|,n}}{R}r\right)e^{im\theta},$$

where:

$$A_{mn} = \frac{1}{2\pi} \cdot \frac{2}{R^2[J_{|m|+1}(j_{|m|,n})]^2}\int_0^{2\pi}\int_0^R f(r,\theta)\, J_{|m|}\!\left(\frac{j_{|m|,n}}{R}r\right)e^{-im\theta}\, r\,dr\,d\theta.$$

Each eigenmode $J_{|m|}(j_{|m|,n}r/R)e^{im\theta}$ decays with rate $\kappa(j_{|m|,n}/R)^2$, giving:

$$u(r,\theta,t) = \sum_{m=-\infty}^\infty\sum_{n=1}^\infty A_{mn}\, e^{-\kappa(j_{|m|,n}/R)^2 t}\, J_{|m|}\!\left(\frac{j_{|m|,n}}{R}r\right)e^{im\theta}.$$

**Long-time behavior.** The dominant term (smallest decay rate) has $m=0$, $n=1$:

$$u(r,\theta,t) \approx A_{01}\, e^{-\kappa(j_{0,1}/R)^2 t}\, J_0\!\left(\frac{j_{0,1}}{R}r\right) \quad \text{as } t \to \infty.$$

The decay rate $\kappa(j_{0,1}/R)^2 \approx 5.783\kappa/R^2$ is the first eigenvalue of $-\Delta$ on the disk with Dirichlet boundary conditions. Larger disks cool more slowly (rate $\propto R^{-2}$).

**Worked example.** Take $f(r,\theta) = T_0$ (uniform initial temperature). By azimuthal symmetry, only $m=0$ terms contribute. The Bessel-Fourier coefficient is:

$$A_{0n} = \frac{2}{R^2[J_1(j_{0,n})]^2}\int_0^R T_0\, J_0\!\left(\frac{j_{0,n}}{R}r\right)r\,dr = \frac{2T_0}{R^2[J_1(j_{0,n})]^2}\cdot\frac{R^2}{j_{0,n}}J_1(j_{0,n}) = \frac{2T_0}{j_{0,n}J_1(j_{0,n})}.$$

(Using $\int_0^R J_0(\alpha r)\,r\,dr = (R/\alpha)J_1(\alpha R)$ with $\alpha R = j_{0,n}$.) Thus:

$$u(r,t) = 2T_0\sum_{n=1}^\infty \frac{J_0(j_{0,n}r/R)}{j_{0,n}J_1(j_{0,n})}\,e^{-\kappa(j_{0,n}/R)^2 t}.$$

**Verification:** At $t=0$: $u(r,0) = 2T_0\sum_n J_0(j_{0,n}r/R)/(j_{0,n}J_1(j_{0,n})) = T_0$ — this is the Bessel-Fourier expansion of the constant function $1$ on $[0,R]$, which is a known identity.

## Wave Equation on a Circular Membrane

**Problem.** Find $u(r,\theta,t)$ satisfying $u_{tt} = c^2\Delta u$ on $r < R$ with $u(R,\theta,t) = 0$, $u(r,\theta,0) = f(r,\theta)$, $u_t(r,\theta,0) = g(r,\theta)$.

**Solution.** The modes are now oscillatory in time:

$$u(r,\theta,t) = \sum_{m,n}\left[A_{mn}\cos(\omega_{mn}t) + B_{mn}\sin(\omega_{mn}t)\right]J_{|m|}\!\left(\frac{j_{|m|,n}}{R}r\right)e^{im\theta},$$

with $\omega_{mn} = cj_{|m|,n}/R$. The coefficients $A_{mn}$ and $B_{mn}$ are determined by the initial conditions exactly as for the heat equation but with $e^{-\lambda t}$ replaced by $\cos(\omega t)$ and $\sin(\omega t)$.

**Inharmonicity.** The frequencies $\omega_{mn} = cj_{|m|,n}/R$ are not integer multiples of $\omega_{01} = cj_{0,1}/R$. The ratio $\omega_{11}/\omega_{01} = j_{1,1}/j_{0,1} \approx 3.832/2.405 \approx 1.59$, which is not a rational number. This inharmonicity (irrational frequency ratios) is why drums and bells sound "unpitched" — unlike strings, where all frequencies are exact integer multiples of the fundamental.

## Neumann Boundary Condition: Bessel Functions of $J_\nu'$

For the Neumann condition $\partial u/\partial r|_{r=R} = 0$, the boundary condition on the radial factor is $J_\nu'(\mu R) = 0$, so $\mu R = j_{\nu,n}'$ (zeros of $J_\nu'$). The orthogonality relation and Bessel-Fourier series hold with $j_{\nu,n}$ replaced by $j_{\nu,n}'$, and the normalization constant becomes:

$$\int_0^R \left[J_\nu\!\left(\frac{j_{\nu,n}'}{R}r\right)\right]^2 r\,dr = \frac{R^2}{2}\left(1-\frac{\nu^2}{(j_{\nu,n}')^2}\right)\left[J_\nu(j_{\nu,n}')\right]^2.$$

The $n=0$, $\nu=0$ case (constant eigenfunction $J_0(0) = 1$) must be handled separately: the zero eigenvalue $\lambda=0$ has eigenfunction $u = \text{const}$, and the expansion has an additional term $c_0 \cdot 1$ (the spatial average).

## Parseval's Identity and Energy

For the heat equation, Parseval's identity for Bessel-Fourier series gives the energy:

$$\int_0^R [u(r,t)]^2\, r\, dr = \sum_n \frac{R^2}{2}[J_1(j_{0,n})]^2 [A_{0n}]^2 e^{-2\kappa(j_{0,n}/R)^2 t}.$$

This decreases monotonically, confirming energy dissipation. The rate of decrease is dominated by the first term, giving the long-time exponential decay at rate $2\kappa(j_{0,1}/R)^2$.

For the wave equation, the energy $\int_0^R[(u_t)^2 + c^2|\nabla u|^2]r\,dr$ is conserved, and Parseval gives it as a sum over modes — each mode contributing a constant (time-independent) energy, consistent with the oscillatory nature of wave solutions.
