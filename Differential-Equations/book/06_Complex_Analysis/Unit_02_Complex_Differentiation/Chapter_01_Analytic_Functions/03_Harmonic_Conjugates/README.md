# Harmonic Conjugates

Given a harmonic function $u$ on a domain $D$, a harmonic conjugate of $u$ is a harmonic function $v$ such that $f = u + iv$ is analytic on $D$. The existence of $v$ is guaranteed when $D$ is simply connected, and $v$ is unique up to an additive constant. The construction of harmonic conjugates is a practical skill: it allows one to recover an analytic function from its real part, and it is the key step in solving certain boundary value problems and in verifying that a given real-valued function is actually the real part of an analytic function.

## Definition and Existence

**Definition.** Let $u : D \to \mathbb{R}$ be harmonic on a domain $D$. A function $v : D \to \mathbb{R}$ is a harmonic conjugate of $u$ if $v$ is harmonic on $D$ and $f = u + iv$ is analytic on $D$.

The Cauchy-Riemann equations $u_x = v_y$ and $u_y = -v_x$ show that $v$ must satisfy a first-order system of PDEs with $u$ as the right-hand side. This system is compatible (i.e., has a solution) if and only if the integrability condition holds:
$$\frac{\partial}{\partial y}(u_x) = \frac{\partial}{\partial x}(-u_y), \qquad \text{i.e.,} \quad u_{xy} = u_{yx},$$
which is automatically true for smooth $u$. The deeper condition is global: the 1-form $-u_y\, dx + u_x\, dy$ must be exact on $D$, which is equivalent to $D$ being simply connected.

**Theorem.** If $u$ is harmonic on a simply connected domain $D$, then $u$ has a harmonic conjugate $v$ on $D$, and $v$ is unique up to an additive real constant.

**Proof.** Define $v$ by the line integral
$$v(x, y) = \int_{(x_0, y_0)}^{(x, y)} -u_y(s, t)\, ds + u_x(s, t)\, dt$$
along any path in $D$ from a fixed base point $(x_0, y_0)$ to $(x, y)$. The integrand is the 1-form $\omega = -u_y\,ds + u_x\,dt$. Its exterior derivative is $d\omega = (-u_{yy} - u_{xx})\,ds\wedge dt = -\Delta u\,ds\wedge dt = 0$ since $u$ is harmonic. By Poincare's lemma (valid on simply connected domains), $\omega$ is exact, so the line integral is path-independent and $v$ is well-defined. By the fundamental theorem of calculus for line integrals, $v_x = -u_y$ and $v_y = u_x$, which are the Cauchy-Riemann equations. $\square$

## The Construction Algorithm

Given a harmonic $u$, construct $v$ by the following steps:

1. From $v_y = u_x$, integrate with respect to $y$ to get $v = \int u_x\, dy + g(x)$ for some function $g(x)$ to be determined.
2. Differentiate with respect to $x$: $v_x = \frac{\partial}{\partial x}\int u_x\,dy + g'(x)$.
3. Apply the second Cauchy-Riemann equation $v_x = -u_y$ and solve for $g'(x)$, then integrate to find $g(x)$.

**Worked example.** Find the harmonic conjugate of $u(x,y) = x^2 - y^2$.

**Step 1.** $u_x = 2x$. Integrate with respect to $y$: $v = \int 2x\,dy + g(x) = 2xy + g(x)$.

**Step 2.** Differentiate with respect to $x$: $v_x = 2y + g'(x)$.

**Step 3.** Apply $v_x = -u_y = -(-2y) = 2y$. So $2y + g'(x) = 2y$, giving $g'(x) = 0$, hence $g(x) = C$ (constant).

**Result:** $v(x,y) = 2xy + C$. The analytic function is $f = (x^2 - y^2) + i(2xy) + iC = z^2 + iC$. $\square$

**Worked example.** Find the harmonic conjugate of $u(x,y) = e^x \cos y$.

**Step 1.** $u_x = e^x\cos y$. Integrate with respect to $y$: $v = \int e^x\cos y\,dy + g(x) = e^x\sin y + g(x)$.

**Step 2.** $v_x = e^x\sin y + g'(x)$.

**Step 3.** $v_x = -u_y = -(-e^x\sin y) = e^x\sin y$. So $g'(x) = 0$, $g(x) = C$.

**Result:** $v = e^x\sin y + C$, and $f = e^x\cos y + ie^x\sin y + iC = e^x e^{iy} + iC = e^z + iC$. $\square$

## Uniqueness and Non-Uniqueness

On a simply connected domain, the harmonic conjugate $v$ is unique up to an additive constant. If $v$ and $v^*$ are both harmonic conjugates of $u$, then $f = u + iv$ and $g = u + iv^*$ are both analytic, so $f - g = i(v - v^*)$ is analytic. Its real part is $0$, so by the Cauchy-Riemann equations its imaginary part $v - v^*$ is constant.

On a non-simply connected domain, harmonic conjugates may fail to exist. The canonical example is $u = \ln r = \frac{1}{2}\ln(x^2 + y^2)$ on the punctured plane $\mathbb{C} \setminus \{0\}$: the "conjugate" would be $\arg z$, but this cannot be made continuous on the full punctured plane.

## Computing Analytic Functions from Real Parts

Harmonic conjugates provide a systematic method for recovering an analytic function $f$ from its real part $u$. An alternative, often faster method is to use the formula for $f'$:

Since $f'(z) = u_x - iu_y$ (from the Cauchy-Riemann equations), we can compute $f'(z)$ and then integrate. Specifically, treat $f'(z)$ as a function of $z$ alone by substituting $x = (z + \bar{z})/2$ and $y = (z - \bar{z})/(2i)$, then set $\bar{z} = 0$ (a formal substitution known as Milne-Thomson's method).

**Worked example (Milne-Thomson).** Given $u = x^3 - 3xy^2$, find $f(z)$ with $f(0) = 0$.

$u_x = 3x^2 - 3y^2$, $u_y = -6xy$.
$f'(z) = u_x - iu_y = (3x^2 - 3y^2) + i(6xy) = 3(x + iy)^2 = 3z^2$.

Integrate: $f(z) = z^3 + C$. Applying $f(0) = 0$ gives $C = 0$, so $f(z) = z^3$. $\square$

## Connections to Conformal Mapping

Harmonic conjugates are central to the theory of conformal mapping. If $w = f(z) = u + iv$ maps a domain $D$ conformally onto a target domain $\Omega$, then $(u, v)$ are the real and imaginary parts of the conformal map. The level curves of $u$ and $v$ — which are harmonic conjugates — form orthogonal families that "mesh" with the geometry of both $D$ and $\Omega$. This orthogonal net is the image of the coordinate lines in the $w$-plane under $f^{-1}$, and it encodes the distortion of the map.

In potential theory applications, conformal maps are used to solve Laplace's equation on complicated domains: map the domain conformally to a simple one (like a disk or half-plane), solve the problem there, and pull back. The harmonic conjugate structure is preserved under conformal maps, making this an internally consistent method.
