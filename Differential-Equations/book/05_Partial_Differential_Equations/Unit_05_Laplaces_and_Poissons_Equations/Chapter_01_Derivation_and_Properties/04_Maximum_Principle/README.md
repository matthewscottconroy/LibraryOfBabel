# The Maximum Principle for Harmonic Functions

The maximum principle for harmonic functions is the elliptic analogue of the parabolic maximum principle for the heat equation, but in the elliptic context it is even more powerful: the statement is simpler, the proof is more direct, and the consequences are more far-reaching. It is the most important single theorem in the theory of elliptic PDEs.

## The Weak Maximum Principle

**Theorem (Weak Maximum Principle).** Let $\Omega$ be a bounded open set and let $u \in C^2(\Omega)\cap C(\bar\Omega)$ be harmonic in $\Omega$. Then:

$$\max_{\bar\Omega} u = \max_{\partial\Omega} u \qquad \text{and} \qquad \min_{\bar\Omega} u = \min_{\partial\Omega} u.$$

The maximum (and minimum) of a harmonic function on a closed bounded domain is attained on the boundary.

**Proof (by contradiction).** Suppose the maximum is attained at an interior point $\mathbf{x}_0 \in \Omega$ with $u(\mathbf{x}_0) = M > \max_{\partial\Omega}u$.

Consider the auxiliary function $v(\mathbf{x}) = u(\mathbf{x}) + \varepsilon|\mathbf{x}|^2$ for small $\varepsilon > 0$. Then $\Delta v = \Delta u + 2n\varepsilon = 2n\varepsilon > 0$ in $\Omega$.

The function $v$ attains its maximum on $\bar\Omega$ somewhere (by compactness). Since $v(x_0) = M + \varepsilon|x_0|^2 > \max_{\partial\Omega}u + \varepsilon|x_0|^2 \geq \max_{\partial\Omega}v$, the maximum of $v$ is attained at some interior point $\mathbf{y}_0 \in \Omega$.

At an interior maximum: $\nabla v(\mathbf{y}_0) = 0$ and $D^2 v(\mathbf{y}_0)$ is negative semidefinite (all second partial derivatives are $\leq 0$), so $\Delta v(\mathbf{y}_0) = \text{tr}(D^2 v) \leq 0$.

But $\Delta v = 2n\varepsilon > 0$ everywhere — contradiction. Sending $\varepsilon \to 0$ completes the proof.

**Alternative proof via mean value property:** If $u$ attains its maximum $M$ at $\mathbf{x}_0 \in \Omega$, then for all small $r$:

$$M = u(\mathbf{x}_0) = \frac{1}{|\partial B_r|}\int_{\partial B_r}u\,dS \leq M.$$

The mean equals $M$ and the integrand is $\leq M$, so $u = M$ on $\partial B_r$ for all small $r$. By connectedness, $u \equiv M$ in $\Omega$ — so if $u$ attains its maximum in the interior, it must be constant. A nonconstant harmonic function cannot attain its maximum at an interior point.

## The Strong Maximum Principle

**Theorem (Strong Maximum Principle).** If $u \in C^2(\Omega)\cap C(\bar\Omega)$ is harmonic in the connected domain $\Omega$ and attains its maximum at an interior point, then $u$ is constant.

This strengthening says: a nonconstant harmonic function cannot attain its maximum anywhere in the interior of the domain. The maximum is always achieved strictly on the boundary (unless $u$ is constant).

The strong maximum principle is proved using the mean value property: if $u(\mathbf{x}_0) = \max_{\bar\Omega}u = M$ and $\mathbf{x}_0 \in \Omega$, then the mean value at $\mathbf{x}_0$ forces $u = M$ on $\partial B_r$ for all small $r$. Applying the argument inductively, $\{u = M\}$ is open. Since $\{u = M\}$ is also closed (by continuity) and $\Omega$ is connected, $\{u = M\} = \Omega$.

## Consequences of the Maximum Principle

**Uniqueness for the Dirichlet Problem.** Suppose $u_1$ and $u_2$ both solve $\Delta u = f$ in $\Omega$ with $u = g$ on $\partial\Omega$. Then $w = u_1 - u_2$ satisfies $\Delta w = 0$ in $\Omega$ and $w = 0$ on $\partial\Omega$. By the maximum principle: $\max_{\bar\Omega}w = \max_{\partial\Omega}w = 0$ and $\min_{\bar\Omega}w = \min_{\partial\Omega}w = 0$. So $w = 0$ everywhere, i.e., $u_1 = u_2$.

**Continuous dependence on boundary data.** If $g_1 \leq g_2$ on $\partial\Omega$, then $u_1 \leq u_2$ in $\Omega$ (comparison principle). More quantitatively: if $|g_1 - g_2| \leq \varepsilon$ on $\partial\Omega$, then $|u_1 - u_2| \leq \varepsilon$ in $\Omega$. This is continuous dependence in the $L^\infty$ norm — the strongest possible sense.

**A priori estimate:** $\|u\|_{L^\infty(\Omega)} \leq \|g\|_{L^\infty(\partial\Omega)}$. The solution is controlled by its boundary data.

**Non-existence of interior extrema.** A harmonic function that is not identically constant cannot be zero on the boundary and positive somewhere in the interior (and vice versa). This is used to prove the positivity of electrostatic potentials: if the boundary of a conductor is held at positive potential, the interior potential is positive.

## Maximum Principle for Subharmonic Functions

A function $u$ is **subharmonic** if $\Delta u \geq 0$ (the value at every point is at most the average over surrounding spheres). A superharmonic function satisfies $\Delta u \leq 0$.

**Theorem.** If $u$ is subharmonic in $\Omega$ and continuous on $\bar\Omega$, then $\max_{\bar\Omega}u = \max_{\partial\Omega}u$.

(The minimum of a subharmonic function need not be on the boundary — there is no minimum principle for subharmonic functions.)

This generalization covers Poisson's equation: if $\Delta u = -f \leq 0$ (i.e., $f \geq 0$), then $u$ is superharmonic and its maximum is attained on the boundary.

## The Hopf Lemma

The Hopf lemma is a quantitative version of the strong maximum principle at the boundary:

**Theorem (Hopf Lemma).** Let $u$ be harmonic in $\Omega$ and continuous on $\bar\Omega$. If $u$ attains its maximum at $\mathbf{x}_0 \in \partial\Omega$ and $u$ is not constant, then the outward normal derivative satisfies:

$$\frac{\partial u}{\partial\nu}(\mathbf{x}_0) > 0.$$

The Hopf lemma says that if the maximum is on the boundary, the function is actively "pushing outward" — the normal derivative points away from the domain. This result is crucial for Neumann problems (showing the solution is non-degenerate at the boundary) and for the theory of free boundary problems.
