# Solutions: Advanced Topics

## Problem 1: Differential Forms and Integration

**Problem.** Verify the Generalized Stokes' Theorem for $\omega = x\,dy\wedge dz$ and the solid ball $V: x^2+y^2+z^2 \leq 1$.

**Solution.** $d\omega = dx\wedge dy\wedge dz$ (since $d(x\,dy\wedge dz) = dx\wedge dy\wedge dz + x\,d(dy\wedge dz) = dx\wedge dy\wedge dz$).

$\int_V d\omega = \int_V dx\,dy\,dz = \text{Vol}(B^3) = \frac{4\pi}{3}$.

For the boundary term: $\partial V = S^2$, the unit sphere. On $S^2$ with the outward orientation, parametrize using spherical coordinates:
$x = \sin\phi\cos\theta$, $y = \sin\phi\sin\theta$, $z = \cos\phi$.

$dy\wedge dz = (\cos\phi\sin\theta\,d\phi + \sin\phi\cos\theta\,d\theta)\wedge(-\sin\phi\,d\phi) = \cos\phi\sin\theta(-\sin\phi)\,d\phi\wedge dz$... 

More directly: the outward unit normal is $(x,y,z)$, and $\omega = x\,dy\wedge dz$. The flux form $\omega$ on $S^2$ gives $\int_{S^2}\omega = \int_{S^2} x\,dy\wedge dz = \int_{S^2} x(\mathbf{n}_x)\,dS$ where $\mathbf{n}_x = x$ is the $x$-component of the outward normal.

$\int_{S^2} x\cdot x\,dS = \int_{S^2} x^2\,dS = \frac{1}{3}\int_{S^2}(x^2+y^2+z^2)\,dS = \frac{1}{3}\cdot\text{Area}(S^2) = \frac{1}{3}\cdot 4\pi = \frac{4\pi}{3}$.

Both sides equal $4\pi/3$. Verified.

---

## Problem 2: Distributional Derivatives

**Problem.** Compute the distributional second derivative of $f(x) = |x|$.

**Solution.** $|x| = \begin{cases}-x & x < 0 \\ x & x > 0\end{cases}$.

First distributional derivative: $|x|' = \text{sgn}(x) = \begin{cases}-1&x<0\\1&x>0\end{cases} = 2H(x) - 1$ (where $H$ is the Heaviside function).

Check: for test function $\varphi$: $|x|'[\varphi] = -|x|[\varphi'] = -\int_{-\infty}^\infty |x|\varphi'(x)\,dx$

$= -\int_{-\infty}^0(-x)\varphi'\,dx - \int_0^\infty x\varphi'\,dx = \int_{-\infty}^0 x\varphi'\,dx - \int_0^\infty x\varphi'\,dx$.

$= [x\varphi]_{-\infty}^0 - \int_{-\infty}^0\varphi\,dx - [x\varphi]_0^\infty + \int_0^\infty\varphi\,dx = -\int_{-\infty}^0\varphi\,dx + \int_0^\infty\varphi\,dx = \int_{-\infty}^\infty\text{sgn}(x)\varphi\,dx$. Confirmed.

Second derivative: $|x|'' = (\text{sgn}(x))' = (2H(x)-1)' = 2H'(x) = 2\delta(x)$.

So $|x|'' = 2\delta$ in the distributional sense: the second derivative of $|x|$ is a point mass at the origin with weight 2. This is consistent with the fact that $|x|$ has a "kink" (non-smooth point) at the origin — the first derivative jumps by $2$, and the second derivative registers this jump as a delta function.

---

## Problem 3: Weak Solutions

**Problem.** Show that $u(x) = |x|$ is a weak solution of $-u'' = 2\delta$ (in distributional sense) and of $-u'' = 0$ away from the origin.

**Solution.** We need to verify: $\int u(-\varphi'')\,dx = 2\varphi(0)$ for all $\varphi \in C_c^\infty(\mathbb{R})$.

$\int_{-\infty}^\infty |x|(-\varphi''(x))\,dx = -\int_{-\infty}^0 (-x)\varphi''\,dx - \int_0^\infty x\varphi''\,dx$.

Integrate by parts:
$\int_{-\infty}^0(-x)\varphi''\,dx = [-(-x)\varphi']_{-\infty}^0 - \int_{-\infty}^0(-1)(-\varphi')\,dx = 0 - \int_{-\infty}^0\varphi'\,dx = -\varphi(0) + \varphi(-\infty) = -\varphi(0)$ (since $\varphi$ has compact support).

Wait, sign: $\int_{-\infty}^0(-x)\varphi''\,dx$. Let $u_1 = -x$, $dv_1 = \varphi''\,dx$:
$= [-x\varphi']_{-\infty}^0 - \int_{-\infty}^0(-\varphi')\,dx = 0 + \int_{-\infty}^0\varphi'\,dx = \varphi(0) - \varphi(-\infty) = \varphi(0)$.

$\int_0^\infty x\varphi''\,dx = [x\varphi']_0^\infty - \int_0^\infty\varphi'\,dx = 0 - (\varphi(\infty)-\varphi(0)) = \varphi(0)$.

So: $-\int_{-\infty}^\infty|x|\varphi''\,dx = -(-1)^2[\varphi(0)+\varphi(0)]$... let me redo carefully.

$-\int|x|\varphi''dx = \int_{-\infty}^0 x\varphi''\,dx - \int_0^\infty x\varphi''\,dx$.

(The sign in front of first integral comes from $-(-x) = x$.)

$\int_{-\infty}^0 x\varphi''\,dx = [x\varphi']_{-\infty}^0 - \int_{-\infty}^0\varphi'\,dx = 0 - [\varphi(0)-\varphi(-\infty)] = -\varphi(0)$.

$\int_0^\infty x\varphi''\,dx = [x\varphi']_0^\infty - \int_0^\infty\varphi'\,dx = 0 - [\varphi(\infty)-\varphi(0)] = \varphi(0)$.

$-\int|x|\varphi'' = -\varphi(0) - \varphi(0) = -2\varphi(0)$.

So $\int|x|(-\varphi'') = 2\varphi(0) = 2\delta[\varphi]$. Hence $-u'' = 2\delta$. Verified.

---

## Problem 4: Sobolev Space Membership

**Problem.** Show that $u(x) = |x|^\alpha$ (for $x \in (-1,1)$) belongs to $H^1(-1,1) = W^{1,2}(-1,1)$ if and only if $\alpha > -1/2$.

**Solution.** $u \in H^1(-1,1)$ requires $u \in L^2$ and $u' \in L^2$.

$\int_{-1}^1|x|^{2\alpha}\,dx = 2\int_0^1 x^{2\alpha}\,dx = 2\cdot\frac{1}{2\alpha+1}$ (convergent iff $2\alpha + 1 > 0$, i.e., $\alpha > -1/2$).

$u' = \alpha|x|^{\alpha-1}\text{sgn}(x)$ (classical for $x \neq 0$, distributional in general).

$\int_{-1}^1|u'|^2\,dx = \alpha^2\int_{-1}^1|x|^{2\alpha-2}\,dx = 2\alpha^2\int_0^1 x^{2\alpha-2}\,dx = \frac{2\alpha^2}{2\alpha-1}$ (convergent iff $2\alpha - 1 > -1$, i.e., $\alpha > 0$).

Wait — for the $L^2$ condition on $u$: $\alpha > -1/2$. For $u' \in L^2$: $\alpha > 0$. But we should allow $\alpha$ near 0; at $\alpha = 1/2$, $u = |x|^{1/2}$, $u' = (1/2)|x|^{-1/2}\text{sgn}(x)$, $(u')^2 = (1/4)|x|^{-1}$, and $\int|x|^{-1}\,dx$ diverges. So $u = |x|^{1/2} \notin H^1$.

Correction: $u' \in L^2$ requires $\int|x|^{2(\alpha-1)}\,dx < \infty$, i.e., $2(\alpha-1) > -1$, i.e., $\alpha > 1/2$.

Combined: $u \in H^1$ iff $u \in L^2$ and $u' \in L^2$, which requires $\alpha > -1/2$ AND $\alpha > 1/2$. So $u \in H^1$ iff $\alpha > 1/2$.

**Note.** The function $u = |x|^{1/2}$ is in $L^2$ but not in $H^1$; it is the borderline case of Sobolev theory showing that $H^1$ imposes more than just $L^2$ regularity. The Sobolev embedding theorem for $H^1((-1,1)) \hookrightarrow C(-1,1)$ is consistent: functions in $H^1$ of an interval are continuous, and $|x|^{1/2}$ is indeed continuous (consistent with membership for large $\alpha$, non-membership for small $\alpha$).

---

## Problem 5: Lax-Milgram for an Elliptic Problem

**Problem.** Use the Lax-Milgram theorem to prove existence and uniqueness of a weak solution to $-\Delta u + u = f$ on $\Omega$ with $\partial u/\partial n = 0$ on $\partial\Omega$ (Neumann boundary condition).

**Solution.** Weak formulation: multiply by $v$ and integrate (no boundary term since $\partial u/\partial n = 0$):
$$\int_\Omega(\nabla u\cdot\nabla v + uv)\,dx = \int_\Omega fv\,dx \quad \forall v \in H^1(\Omega).$$

Define $B(u,v) = \int_\Omega(\nabla u\cdot\nabla v + uv)\,dx$ on $H^1(\Omega) = W^{1,2}(\Omega)$.

**Boundedness:** $|B(u,v)| \leq \int|\nabla u||\nabla v| + |u||v| \leq \|\nabla u\|_{L^2}\|\nabla v\|_{L^2} + \|u\|_{L^2}\|v\|_{L^2} \leq \|u\|_{H^1}\|v\|_{H^1}$.

**Coercivity:** $B(u,u) = \int(|\nabla u|^2 + u^2) = \|u\|_{H^1}^2$.

(Note: coercivity here holds with constant $\alpha = 1$ on all of $H^1(\Omega)$, without needing the Poincaré inequality, because the $u^2$ term is present.)

**Bounded linear functional:** $F(v) = \int_\Omega fv\,dx$ is bounded on $H^1$: $|F(v)| \leq \|f\|_{L^2}\|v\|_{L^2} \leq \|f\|_{L^2}\|v\|_{H^1}$.

By Lax-Milgram: there exists a unique $u \in H^1(\Omega)$ with $B(u,v) = F(v)$ for all $v \in H^1(\Omega)$.

**Remark.** For pure Neumann boundary conditions on $-\Delta u = f$ (without the $+u$ term), the bilinear form $\int\nabla u\cdot\nabla v$ is only coercive on the orthogonal complement of the constants (by the Poincaré-Wirtinger inequality), and a compatibility condition $\int f = 0$ is needed. The term $+u$ in the equation removes this degeneracy.
