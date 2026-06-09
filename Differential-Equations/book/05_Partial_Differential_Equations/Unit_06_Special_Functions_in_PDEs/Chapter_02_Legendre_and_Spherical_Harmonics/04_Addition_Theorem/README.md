# The Addition Theorem for Spherical Harmonics

The addition theorem is the fundamental identity connecting spherical harmonics to the geometry of angles on the sphere. It states that the Legendre polynomial $P_\ell(\cos\gamma)$, where $\gamma$ is the angle between two unit vectors $\hat{\mathbf{x}}$ and $\hat{\mathbf{y}}$, can be expressed as:

$$P_\ell(\cos\gamma) = \frac{4\pi}{2\ell+1}\sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})}. \tag{Addition Theorem}$$

This identity is the bridge between the Legendre polynomial expansion of the Coulomb potential and the full multipole expansion in spherical harmonics. It encodes the rotational invariance of $P_\ell(\cos\gamma)$ as a consequence of the transformation properties of $Y_\ell^m$ under $SO(3)$.

## Statement and Proof

**Theorem (Addition Theorem).** Let $\hat{\mathbf{x}}, \hat{\mathbf{y}} \in S^2$ be unit vectors with angle $\cos\gamma = \hat{\mathbf{x}}\cdot\hat{\mathbf{y}}$. For each $\ell \geq 0$:

$$\sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})} = \frac{2\ell+1}{4\pi}P_\ell(\cos\gamma).$$

**Proof.** Fix $\hat{\mathbf{y}}$ and define the function $F_\ell(\hat{\mathbf{x}}) = \sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})}$ on $S^2$.

**Step 1: $F_\ell$ is in the $\ell$-th eigenspace.** Since $\Delta_{S^2}Y_\ell^m = -\ell(\ell+1)Y_\ell^m$, the function $F_\ell$ (a finite sum of eigenfunctions with eigenvalue $-\ell(\ell+1)$) satisfies $\Delta_{S^2}F_\ell = -\ell(\ell+1)F_\ell$.

**Step 2: $F_\ell$ is rotationally symmetric about $\hat{\mathbf{y}}$.** For any rotation $R$ that fixes $\hat{\mathbf{y}}$ (i.e., $R\hat{\mathbf{y}} = \hat{\mathbf{y}}$):

$$F_\ell(R\hat{\mathbf{x}}) = \sum_m Y_\ell^m(R\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})} = \sum_m \sum_{m'} D^{(\ell)}_{m'm}(R^{-1})Y_\ell^{m'}(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})}.$$

The sum $\sum_{m}D^{(\ell)}_{m'm}(R^{-1})\overline{Y_\ell^m(\hat{\mathbf{y}})} = \overline{\sum_m D^{(\ell)}_{mm'}(R)Y_\ell^m(\hat{\mathbf{y}})} = \overline{Y_\ell^{m'}(R^{-1}\hat{\mathbf{y}})} = \overline{Y_\ell^{m'}(\hat{\mathbf{y}})}$ (since $R\hat{\mathbf{y}} = \hat{\mathbf{y}}$ implies $R^{-1}\hat{\mathbf{y}} = \hat{\mathbf{y}}$). So $F_\ell(R\hat{\mathbf{x}}) = \sum_{m'}Y_\ell^{m'}(\hat{\mathbf{x}})\overline{Y_\ell^{m'}(\hat{\mathbf{y}})} = F_\ell(\hat{\mathbf{x}})$. Thus $F_\ell$ is invariant under all rotations fixing $\hat{\mathbf{y}}$, i.e., $F_\ell(\hat{\mathbf{x}}) = F_\ell(\cos\gamma)$ depends only on $\gamma = \angle(\hat{\mathbf{x}},\hat{\mathbf{y}})$.

**Step 3: Identify $F_\ell$.** In the coordinate system where $\hat{\mathbf{y}} = (0,0,1)$ (north pole), the rotational symmetry about the $z$-axis means $F_\ell$ has $m=0$ in its spherical harmonic expansion in $\hat{\mathbf{x}}$. But $F_\ell$ is also in the $\ell$-th eigenspace, so $F_\ell(\hat{\mathbf{x}}) = c Y_\ell^0(\theta) = c\sqrt{(2\ell+1)/(4\pi)}P_\ell(\cos\theta)$. The constant $c$ is determined by setting $\hat{\mathbf{x}} = \hat{\mathbf{y}}$:

$$F_\ell(\hat{\mathbf{y}}) = \sum_m |Y_\ell^m(\hat{\mathbf{y}})|^2 = \sum_m |Y_\ell^m(0,0)|^2.$$

At the north pole ($\theta=0$): $Y_\ell^m(0,\phi) \propto P_\ell^{|m|}(1) = 0$ for $m\neq 0$, and $Y_\ell^0(0,\phi) = \sqrt{(2\ell+1)/(4\pi)}P_\ell(1) = \sqrt{(2\ell+1)/(4\pi)}$. So $F_\ell(\hat{\mathbf{y}}) = (2\ell+1)/(4\pi)$.

Also, at $\gamma = 0$ (i.e., $\hat{\mathbf{x}} = \hat{\mathbf{y}}$): $P_\ell(\cos 0) = P_\ell(1) = 1$, and $c\sqrt{(2\ell+1)/(4\pi)} = (2\ell+1)/(4\pi)$, giving $c = \sqrt{(2\ell+1)/(4\pi)}$.

Therefore $F_\ell(\hat{\mathbf{x}}) = \frac{(2\ell+1)}{4\pi}P_\ell(\cos\gamma)$, completing the proof. $\square$

## Consequences and Applications

**1. Expansion of the Coulomb Potential.** The free-space Green's function $1/(4\pi|\mathbf{x}-\mathbf{y}|)$ expands as:

$$\frac{1}{|\mathbf{x}-\mathbf{y}|} = \sum_{\ell=0}^\infty\frac{r_<^\ell}{r_>^{\ell+1}}P_\ell(\cos\gamma),$$

where $r_< = \min(r,s)$, $r_> = \max(r,s)$, and $\gamma$ is the angle between $\mathbf{x}$ and $\mathbf{y}$. Applying the addition theorem to replace $P_\ell(\cos\gamma)$:

$$\frac{1}{|\mathbf{x}-\mathbf{y}|} = 4\pi\sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell\frac{1}{2\ell+1}\frac{r_<^\ell}{r_>^{\ell+1}}Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})}.$$

This is the **multipole expansion** of the Coulomb potential. Each term $r_<^\ell r_>^{-\ell-1}Y_\ell^m(\hat{\mathbf{x}})Y_\ell^{m*}(\hat{\mathbf{y}})$ is harmonic in $\mathbf{x}$ for $r \neq s$ (being a product of a harmonic homogeneous polynomial of degree $\ell$ and its reciprocal counterpart), confirming that the expansion converges to a harmonic function.

**2. Funk-Hecke Formula.** For any integrable function $K:[-1,1]\to\mathbb{C}$ (a "zonal kernel"), the spherical convolution with $K$ diagonalizes in the spherical harmonic basis:

$$\int_{S^2}K(\hat{\mathbf{x}}\cdot\hat{\mathbf{y}})\,Y_\ell^m(\hat{\mathbf{y}})\,dS(\hat{\mathbf{y}}) = \lambda_\ell\,Y_\ell^m(\hat{\mathbf{x}}),$$

where $\lambda_\ell = 2\pi\int_{-1}^1 K(t)P_\ell(t)\,dt$ is the $\ell$-th Legendre coefficient of $K$. This follows directly from the addition theorem: expand $K(\hat{\mathbf{x}}\cdot\hat{\mathbf{y}}) = \sum_k \hat{K}_k P_k(\hat{\mathbf{x}}\cdot\hat{\mathbf{y}})$, apply the addition theorem, and use orthonormality. The Funk-Hecke formula shows that rotationally symmetric operators on $S^2$ are diagonalized by spherical harmonics — they act as scalar multiplication by $\lambda_\ell$ on each eigenspace.

**3. Gegenbauer Expansion.** The addition theorem generalizes to $\mathbb{R}^n$: the Gegenbauer (ultraspherical) polynomials $C_\ell^\alpha(t)$ satisfy an analogous addition theorem in terms of spherical harmonics on $S^{n-1}$. For $n=2$ (circle), the addition theorem reduces to $\cos(m\gamma) = \cos(m\theta)\cos(m\phi) + \sin(m\theta)\sin(m\phi)$ — the standard angle addition formula.

**4. Green's Function for the Ball via Addition Theorem.** The Green's function for the ball $B_R$ can be derived using the multipole expansion. The solution of $-\Delta u = \delta(\mathbf{x}-\mathbf{y})$ in $B_R$ with $u=0$ on $\partial B_R$ is:

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{4\pi}\left[\frac{1}{|\mathbf{x}-\mathbf{y}|} - \frac{R}{|\mathbf{y}|}\frac{1}{|\mathbf{x}-\mathbf{y}^*|}\right],$$

where $\mathbf{y}^* = R^2\mathbf{y}/|\mathbf{y}|^2$ is the Kelvin inverse. This can also be written using the addition theorem:

$$G(\mathbf{x};\mathbf{y}) = \frac{1}{R}\sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell\frac{4\pi}{2\ell+1}\left[\frac{r^\ell s^\ell}{R^{2\ell}} - \frac{r^\ell}{s^{\ell+1}}\right]Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})} \quad \text{(for } r < s < R\text{)},$$

providing the spectral (eigenfunction expansion) form of the Green's function, complementary to the image-charge formula.

## The Convolution Theorem on $S^2$

The addition theorem implies a convolution theorem. The **convolution** of $f$ and $g$ on $S^2$ (under the action of $SO(3)$) is:

$$(f * g)(\hat{\mathbf{x}}) = \int_{SO(3)} f(R\hat{\mathbf{x}}_0)\,g(R^{-1}\hat{\mathbf{x}})\,dR,$$

where $\hat{\mathbf{x}}_0$ is a fixed base point. If $f(\hat{\mathbf{x}}) = K(\hat{\mathbf{x}}\cdot\hat{\mathbf{x}}_0)$ is a zonal function (depends only on the angle from $\hat{\mathbf{x}}_0$), then by the Funk-Hecke formula:

$$\widehat{(f*g)}_\ell^m = \hat{K}_\ell \cdot \hat{g}_\ell^m,$$

where $\hat{K}_\ell = 2\pi\int_{-1}^1 K(t)P_\ell(t)\,dt$ is the $\ell$-th Legendre coefficient of $K$. This is the exact analog of the ordinary convolution theorem (Fourier transform of a convolution is a product of Fourier transforms), with spherical harmonics playing the role of the Fourier modes.

**Application: smoothing on the sphere.** A Gaussian kernel on $S^2$ is $K_\sigma(\hat{\mathbf{x}}\cdot\hat{\mathbf{y}}) = C_\sigma e^{-(1-\hat{\mathbf{x}}\cdot\hat{\mathbf{y}})/\sigma^2}$. Its Legendre coefficients $\hat{K}_\ell \sim e^{-\ell(\ell+1)\sigma^2/2}$ (approximately, for small $\sigma$) decay rapidly for $\ell \gg 1/\sigma$. Convolution with this kernel is the heat flow on $S^2$ for time $\propto\sigma^2$, and in the spherical harmonic domain it simply multiplies each $\hat{f}_\ell^m$ by $\hat{K}_\ell$ — a perfect illustration of the heat equation on $S^2$ diagonalized in the $Y_\ell^m$ basis.

## Summation Formula and the Poisson Kernel on $S^2$

The addition theorem and geometric series give the Poisson kernel for the ball. For $r < R$:

$$\sum_{\ell=0}^\infty \frac{2\ell+1}{4\pi}\left(\frac{r}{R}\right)^\ell P_\ell(\cos\gamma) = \frac{R(R^2-r^2)}{4\pi(R^2-2Rr\cos\gamma+r^2)^{3/2}},$$

which is exactly the Poisson kernel for the 3D ball. The derivation uses $\sum_\ell(2\ell+1)s^\ell P_\ell(t) = (1-s^2)(1-2st+s^2)^{-3/2}$, obtained by differentiating the generating function with respect to $s$ and multiplying by $(1-s^2)/(1-2st+s^2)$.
