# Green's Function for the Ball (Sphere)

The Green's function for the ball $B_R = \{|\mathbf{x}| < R\}$ in $\mathbb{R}^n$ ($n \geq 2$) is constructed using the **Kelvin transform** — inversion of the point $\mathbf{y}$ in the sphere $|\mathbf{x}|=R$. This yields a closed-form formula for the Poisson kernel, recovering the classical Poisson integral formula for the ball.

## Kelvin Transform and Image Point

For $\mathbf{y} \in B_R$ ($\mathbf{y} \neq \mathbf{0}$), the **Kelvin inverse** (image point) is:

$$\mathbf{y}^* = \frac{R^2\mathbf{y}}{|\mathbf{y}|^2}.$$

Note $\mathbf{y}^*$ lies outside $B_R$ (since $|\mathbf{y}^*| = R^2/|\mathbf{y}| > R$). The key geometric identity: for $\mathbf{x} \in \partial B_R$ (so $|\mathbf{x}|=R$),

$$|\mathbf{x}-\mathbf{y}^*| = \frac{R}{|\mathbf{y}|}|\mathbf{x}-\mathbf{y}|.$$

*Proof:* $|\mathbf{x}-\mathbf{y}^*|^2 = |\mathbf{x}|^2 - 2\mathbf{x}\cdot\mathbf{y}^* + |\mathbf{y}^*|^2 = R^2 - 2R^2(\mathbf{x}\cdot\mathbf{y})/|\mathbf{y}|^2 + R^4/|\mathbf{y}|^2 = (R^2/|\mathbf{y}|^2)(|\mathbf{y}|^2 - 2\mathbf{x}\cdot\mathbf{y} + |\mathbf{x}|^2) = (R/|\mathbf{y}|)^2|\mathbf{x}-\mathbf{y}|^2$.

## Green's Function in $\mathbb{R}^n$, $n \geq 3$

$$G(\mathbf{x};\mathbf{y}) = \Phi(\mathbf{x}-\mathbf{y}) - \left(\frac{|\mathbf{y}|}{R}\right)^{n-2}\Phi(\mathbf{x}-\mathbf{y}^*) = \frac{1}{n(n-2)\omega_n}\left[\frac{1}{|\mathbf{x}-\mathbf{y}|^{n-2}} - \frac{R^{n-2}}{|\mathbf{y}|^{n-2}|\mathbf{x}-\mathbf{y}^*|^{n-2}}\right].$$

**Verification on $\partial B_R$:** Using the identity $|\mathbf{x}-\mathbf{y}^*| = (R/|\mathbf{y}|)|\mathbf{x}-\mathbf{y}|$:

$$G(\mathbf{x};\mathbf{y})\Big|_{|\mathbf{x}|=R} = \frac{1}{n(n-2)\omega_n}\left[\frac{1}{|\mathbf{x}-\mathbf{y}|^{n-2}} - \frac{R^{n-2}}{|\mathbf{y}|^{n-2}}\cdot\frac{|\mathbf{y}|^{n-2}}{R^{n-2}|\mathbf{x}-\mathbf{y}|^{n-2}}\right] = 0.$$

## Green's Function in $\mathbb{R}^2$

$$G(\mathbf{x};\mathbf{y}) = -\frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}| + \frac{1}{2\pi}\log\!\left(\frac{|\mathbf{y}|}{R}\right) + \frac{1}{2\pi}\log|\mathbf{x}-\mathbf{y}^*|.$$

On $|\mathbf{x}|=R$: $\log|\mathbf{x}-\mathbf{y}^*| = \log(R|\mathbf{x}-\mathbf{y}|/|\mathbf{y}|) = \log R + \log|\mathbf{x}-\mathbf{y}| - \log|\mathbf{y}|$, so $G = 0$.

## Poisson Kernel for the Ball

The Poisson kernel $P(\mathbf{x};\mathbf{y}) = -\partial G/\partial\nu_\mathbf{y}|_{\mathbf{y}\in\partial B_R}$ (outward normal on $\partial B_R$ points radially outward):

$$P(\mathbf{x};\mathbf{y}) = \frac{R^2-|\mathbf{x}|^2}{n\omega_n R|\mathbf{x}-\mathbf{y}|^n}, \qquad \mathbf{x}\in B_R,\; \mathbf{y}\in\partial B_R.$$

**Poisson formula for the ball:**

$$u(\mathbf{x}) = \frac{R^2-|\mathbf{x}|^2}{n\omega_n R}\int_{\partial B_R}\frac{g(\mathbf{y})}{|\mathbf{x}-\mathbf{y}|^n}\,dS(\mathbf{y}).$$

In 2D ($n=2$, $\omega_2=\pi$): $u(r,\theta) = \frac{R^2-r^2}{2\pi}\int_0^{2\pi}\frac{g(\phi)}{R^2-2Rr\cos(\theta-\phi)+r^2}\,d\phi$ — the classical Poisson formula recovered.

## Consequences

**Mean value property:** At $\mathbf{x} = \mathbf{0}$, the Poisson kernel becomes $P(\mathbf{0};\mathbf{y}) = 1/(n\omega_n R^{n-1}) = 1/|\partial B_R|$ — constant! The formula gives $u(\mathbf{0}) = \frac{1}{|\partial B_R|}\int_{\partial B_R}g\,dS$ — the mean value property.

**Harnack's inequality:** From the explicit form of the Poisson kernel, for $|\mathbf{x}| \leq r < R$:

$$\frac{R-r}{R+r} \leq \frac{P(\mathbf{x};\mathbf{y})}{1/|\partial B_R|} \leq \frac{R+r}{R-r},$$

giving: $(R-r)/(R+r)\cdot g_{\min} \leq u(\mathbf{x}) \leq (R+r)/(R-r)\cdot g_{\max}$ — a quantitative Harnack inequality.
