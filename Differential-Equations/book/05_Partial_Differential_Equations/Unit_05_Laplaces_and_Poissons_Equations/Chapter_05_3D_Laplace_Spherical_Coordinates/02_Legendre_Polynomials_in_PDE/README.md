# Legendre Polynomials in PDE Applications

Legendre polynomials $P_\ell(t)$ and associated Legendre functions $P_\ell^m(t)$ arise from separation of variables for Laplace's equation in spherical coordinates when the problem has azimuthal symmetry (independent of $\phi$). They form a complete orthogonal system on $[-1,1]$ and are the radial basis functions for potential theory on the sphere.

## Legendre's Equation

The polar equation for the case $m=0$ (azimuthal symmetry, $\Phi = \text{const}$) is, with $t = \cos\theta$:

$$\frac{d}{dt}\!\left[(1-t^2)\frac{dP}{dt}\right] + \ell(\ell+1)P = 0, \qquad -1 \leq t \leq 1.$$

This is **Legendre's equation** of degree $\ell$. For $\ell = 0, 1, 2, \ldots$, the equation has polynomial solutions — the **Legendre polynomials** $P_\ell(t)$.

**Rodrigues' formula:** $P_\ell(t) = \frac{1}{2^\ell\ell!}\frac{d^\ell}{dt^\ell}(t^2-1)^\ell.$

First few:
$$P_0=1,\; P_1=t,\; P_2=\tfrac{1}{2}(3t^2-1),\; P_3=\tfrac{1}{2}(5t^3-3t),\; P_4=\tfrac{1}{8}(35t^4-30t^2+3).$$

## Key Properties

**Orthogonality:** $\int_{-1}^1 P_\ell(t)P_k(t)\,dt = \frac{2}{2\ell+1}\delta_{\ell k}$.

**Recursion:** $(n+1)P_{n+1}(t) = (2n+1)tP_n(t) - nP_{n-1}(t)$.

**Special values:** $P_\ell(1) = 1$, $P_\ell(-1) = (-1)^\ell$.

**Generating function:** $\sum_{\ell=0}^\infty P_\ell(t)s^\ell = (1-2ts+s^2)^{-1/2}$ for $|s|<1$.

## Applications to Potential Theory

**Potential of a point charge.** The Green's function for Laplace's equation in $\mathbb{R}^3$ is $G(\mathbf{x};\mathbf{y}) = 1/(4\pi|\mathbf{x}-\mathbf{y}|)$. Using the generating function with $t = \cos\gamma$ (angle between $\mathbf{x}$ and $\mathbf{y}$) and $s = r_</r_>$ (ratio of smaller to larger radius):

$$\frac{1}{|\mathbf{x}-\mathbf{y}|} = \sum_{\ell=0}^\infty\frac{r_<^\ell}{r_>^{\ell+1}}P_\ell(\cos\gamma),$$

where $r_< = \min(|\mathbf{x}|,|\mathbf{y}|)$ and $r_> = \max(|\mathbf{x}|,|\mathbf{y}|)$. This is the **Legendre polynomial expansion of the Coulomb potential** — the fundamental formula of electrostatics and gravitation.

**Multipole expansion.** If a charge distribution $\rho(\mathbf{y})$ is supported inside a ball of radius $R$, and we want the potential outside ($|\mathbf{x}| > R$):

$$u(\mathbf{x}) = \frac{1}{4\pi}\int\frac{\rho(\mathbf{y})}{|\mathbf{x}-\mathbf{y}|}\,d\mathbf{y} = \sum_{\ell=0}^\infty\frac{1}{|\mathbf{x}|^{\ell+1}}\sum_{m=-\ell}^\ell q_{\ell m}Y_\ell^m(\hat{\mathbf{x}}),$$

where $q_{\ell m} = \frac{4\pi}{2\ell+1}\int|\mathbf{y}|^\ell\rho(\mathbf{y})Y_\ell^{m*}(\hat{\mathbf{y}})\,d\mathbf{y}$ are the multipole moments.

## Associated Legendre Functions

For $m \neq 0$ (non-azimuthal problems), the polar equation is the **associated Legendre equation**:

$$\frac{d}{dt}\!\left[(1-t^2)\frac{dP}{dt}\right] + \left[\ell(\ell+1) - \frac{m^2}{1-t^2}\right]P = 0.$$

The bounded solutions are the **associated Legendre functions**:

$$P_\ell^m(t) = (-1)^m(1-t^2)^{m/2}\frac{d^m}{dt^m}P_\ell(t), \qquad m = 0, 1, \ldots, \ell.$$

For $m < 0$: $P_\ell^{-m}(t) = (-1)^m\frac{(\ell-m)!}{(\ell+m)!}P_\ell^m(t)$.

**Orthogonality:** $\int_{-1}^1 P_\ell^m(t)P_k^m(t)\,dt = \frac{2}{2\ell+1}\frac{(\ell+m)!}{(\ell-m)!}\delta_{\ell k}$.

## Dirichlet Problem on a Ball

For the ball $B_R$ with boundary data $g(\theta,\phi)$ expanded in spherical harmonics $g = \sum_{\ell,m}g_{\ell m}Y_\ell^m$, the solution is:

$$u(r,\theta,\phi) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell g_{\ell m}\left(\frac{r}{R}\right)^\ell Y_\ell^m(\theta,\phi), \qquad r < R.$$

For the exterior ($r > R$):

$$u(r,\theta,\phi) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell g_{\ell m}\left(\frac{R}{r}\right)^{\ell+1}Y_\ell^m(\theta,\phi), \qquad r > R.$$

The $r^\ell$ growth (interior) and $r^{-\ell-1}$ decay (exterior) follow from the radial equation. The transition at $r=R$ matches $u$ and $\partial u/\partial r$ continuously (for the interior problem with $u=g$ on $r=R$).
