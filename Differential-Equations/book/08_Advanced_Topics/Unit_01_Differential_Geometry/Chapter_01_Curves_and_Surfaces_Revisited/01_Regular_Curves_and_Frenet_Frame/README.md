# Regular Curves and the Frenet Frame

A curve in space can be twisted and bent in complex ways, yet its shape at each point is captured completely by just two scalar quantities: its curvature (how fast it bends) and its torsion (how fast it twists out of a plane). This section develops the machinery—the Frenet frame and its equations—that makes this precise, and proves the fundamental theorem of space curves: curvature and torsion together determine the curve up to rigid motion.

## Regular Parameterized Curves

A **parameterized curve** is a smooth map $\gamma: I \to \mathbb{R}^3$ for an interval $I \subset \mathbb{R}$. It is called **regular** if $\gamma'(t) \neq 0$ for all $t \in I$. Regularity ensures the curve has a well-defined tangent direction everywhere; without it, the curve might have corners or cusps where the notion of curvature breaks down.

Two parameterized curves $\gamma: I \to \mathbb{R}^3$ and $\tilde{\gamma}: J \to \mathbb{R}^3$ are **equivalent** if there exists a diffeomorphism $h: J \to I$ with $\tilde{\gamma} = \gamma \circ h$. If $h' > 0$ the reparameterization preserves orientation. The geometric shape of the curve is an equivalence class under orientation-preserving reparameterizations.

**Arc length parameterization.** Given a regular curve $\gamma$ and $a \in I$, define the arc length function

$$s(t) = \int_a^t |\gamma'(u)| \, du.$$

Since $|\gamma'| > 0$, $s$ is strictly increasing and defines a diffeomorphism from $I$ to an interval $J$. The curve reparameterized by arc length, $\tilde{\gamma}(s) = \gamma(t(s))$, satisfies $|\tilde{\gamma}'(s)| = 1$. Such a curve is called **unit-speed**. Unless stated otherwise, we assume unit-speed parameterization when computing intrinsic quantities.

## Curvature

For a unit-speed curve $\gamma: J \to \mathbb{R}^3$, differentiating $|\gamma'|^2 = 1$ gives $\gamma' \cdot \gamma'' = 0$: the second derivative is perpendicular to the tangent. The **curvature** is $\kappa(s) = |\gamma''(s)|$.

**Geometric interpretation.** Consider the unit tangent vector $T(s) = \gamma'(s)$ as a point on the unit sphere $S^2$. As $s$ varies, $T(s)$ traces a curve on $S^2$ called the **indicatrix** of $\gamma$. The curvature $\kappa(s) = |T'(s)|$ is the speed of this indicatrix. High curvature means the tangent direction is changing rapidly; $\kappa = 0$ means the curve is locally straight.

**Example.** For a circle of radius $r$, parameterized by arc length as $\gamma(s) = (r\cos(s/r), r\sin(s/r), 0)$: $\gamma'(s) = (-\sin(s/r), \cos(s/r), 0)$, $\gamma''(s) = (-\cos(s/r)/r, -\sin(s/r)/r, 0)$, so $\kappa = |\gamma''| = 1/r$. Larger circles are less curved.

## The Frenet-Serret Frame

For a unit-speed curve with $\kappa(s) > 0$ everywhere, define:

**Unit tangent:** $T(s) = \gamma'(s)$.

**Principal normal:** $N(s) = \frac{T'(s)}{|T'(s)|} = \frac{\gamma''(s)}{\kappa(s)}$.

Since $|T| = 1$, $T$ and $T' = \kappa N$ are orthogonal. The principal normal $N$ points toward the center of curvature.

**Binormal:** $B(s) = T(s) \times N(s)$.

By construction, $\{T(s), N(s), B(s)\}$ is a positively oriented orthonormal frame at each point, called the **Frenet frame** (or moving frame).

## The Frenet-Serret Equations

Differentiating the orthonormality relations $T \cdot T = N \cdot N = B \cdot B = 1$ and $T \cdot N = T \cdot B = N \cdot B = 0$, one finds:

$$T' = \kappa N.$$

For $N'$: write $N' = aT + bN + cB$. We have $b = N' \cdot N = 0$ (from $N \cdot N = 1$), $a = N' \cdot T = -N \cdot T' = -\kappa$ (from $N \cdot T = 0$, differentiated). Setting $\tau = -N' \cdot B = B' \cdot N$ (the **torsion**):

$$N' = -\kappa T + \tau B.$$

For $B'$: since $|B| = 1$, $B' \perp B$. Also $B' \cdot T = -B \cdot T' = -B \cdot \kappa N = 0$, so $B' \parallel N$:

$$B' = -\tau N.$$

These three equations are the **Frenet-Serret equations**:

$$\begin{pmatrix} T' \\ N' \\ B' \end{pmatrix} = \begin{pmatrix} 0 & \kappa & 0 \\ -\kappa & 0 & \tau \\ 0 & -\tau & 0 \end{pmatrix} \begin{pmatrix} T \\ N \\ B \end{pmatrix}.$$

The matrix is skew-symmetric, which is characteristic of rotation: the Frenet frame rotates as one travels along the curve.

**Geometric meaning of torsion.** The binormal $B$ is normal to the osculating plane (the plane spanned by $T$ and $N$). The torsion $\tau$ measures how fast $B$ changes direction, i.e., how fast the osculating plane rotates. A planar curve has $\tau = 0$ everywhere; a helix has constant nonzero $\tau$.

## Example: The Circular Helix

The helix $\gamma(t) = (a\cos t, a\sin t, bt)$ (with $a > 0$, $b \geq 0$) has speed $|\gamma'| = \sqrt{a^2 + b^2}$. Reparameterizing by arc length with $c = \sqrt{a^2 + b^2}$:

$$T = \frac{1}{c}(-a\sin t, a\cos t, b), \quad \gamma'' = \frac{1}{c^2}(-a\cos t, -a\sin t, 0),$$

$$\kappa = |\gamma''| / (1/c) = \frac{a}{a^2+b^2}, \quad N = (-\cos t, -\sin t, 0), \quad B = T \times N = \frac{1}{c}(b\sin t, -b\cos t, a).$$

Then $B' = \frac{1}{c}(b\cos t, b\sin t, 0) \cdot (1/c) \cdot (-1) \cdot \frac{1}{c}$... more cleanly: $\tau = -B' \cdot N = b/(a^2+b^2)$.

Both $\kappa$ and $\tau$ are constant, characterizing the helix. The ratio $\tau/\kappa = b/a$ gives the "pitch" of the helix relative to its curvature.

## The Fundamental Theorem of Space Curves

**Theorem.** Let $\kappa: I \to (0, \infty)$ and $\tau: I \to \mathbb{R}$ be smooth functions. Then there exists a smooth regular curve $\gamma: I \to \mathbb{R}^3$, unique up to orientation-preserving rigid motion of $\mathbb{R}^3$, with curvature $\kappa$ and torsion $\tau$.

**Proof.** Existence: The Frenet system $dF/ds = AF$ (where $F = (T, N, B)^T$ and $A$ is the skew-symmetric matrix above) is a linear ODE with $A(s)$ smooth. For any initial frame $F(s_0) = (T_0, N_0, B_0)$ (orthonormal), there exists a unique smooth solution $F(s)$ remaining orthonormal (since $dF^TF/ds = F^T(A^T + A)F = 0$ for skew-symmetric $A$). Setting $\gamma(s) = \gamma(s_0) + \int_{s_0}^s T(u) \, du$ gives the desired curve.

Uniqueness up to rigid motion: If $\tilde{\gamma}$ is another such curve, apply a rigid motion to make $T(s_0) = \tilde{T}(s_0)$, $N(s_0) = \tilde{N}(s_0)$, $B(s_0) = \tilde{B}(s_0)$. Then the Frenet frames of both curves satisfy the same linear ODE with the same initial condition, so they coincide, hence $\gamma = \tilde{\gamma}$. $\square$

## Non-Unit-Speed Curves

For a regular but not unit-speed curve $\gamma(t)$, the Frenet invariants become:

$$\kappa = \frac{|\gamma' \times \gamma''|}{|\gamma'|^3}, \quad \tau = \frac{(\gamma' \times \gamma'') \cdot \gamma'''}{|\gamma' \times \gamma''|^2}.$$

These formulas, derived by the chain rule, enable computation of curvature and torsion directly from any parameterization without first converting to arc length.

**Example.** For the helix $\gamma(t) = (\cos t, \sin t, t)$: $\gamma' = (-\sin t, \cos t, 1)$, $\gamma'' = (-\cos t, -\sin t, 0)$, $\gamma' \times \gamma'' = (\sin t, -\cos t, 1)$, $|\gamma' \times \gamma''| = \sqrt{2}$, $|\gamma'|^3 = 2^{3/2}$. So $\kappa = \sqrt{2}/2^{3/2} = 1/2$. Similarly $\tau = 1/2$.
