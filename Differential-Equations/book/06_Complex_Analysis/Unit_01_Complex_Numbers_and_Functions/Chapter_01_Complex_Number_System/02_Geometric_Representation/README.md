# Geometric Representation of Complex Numbers

The most important insight in elementary complex analysis is that complex arithmetic is geometry. Identifying a complex number $z = x + iy$ with the point $(x, y)$ in the Euclidean plane transforms algebraic operations into geometric ones: addition becomes vector addition, multiplication becomes rotation and scaling. This section makes that correspondence precise and develops the geometric tools — modulus, argument, the unit circle, and the Riemann sphere — that appear throughout the subject.

## The Complex Plane

The complex plane, or Argand diagram, is the set $\mathbb{C}$ visualized as $\mathbb{R}^2$: the real part $\mathrm{Re}(z) = x$ is measured on the horizontal axis (the real axis) and the imaginary part $\mathrm{Im}(z) = y$ on the vertical axis (the imaginary axis). The origin corresponds to $0$, and the real numbers embed as the horizontal axis.

The distance between two complex numbers is
$$d(z, w) = |z - w| = \sqrt{(\mathrm{Re}(z-w))^2 + (\mathrm{Im}(z-w))^2},$$
which is the ordinary Euclidean distance between the corresponding points. The complex plane is therefore a metric space, and all the topological concepts of metric spaces — open sets, closed sets, compactness, connectedness — apply directly.

Key geometric sets:
- The circle of radius $r$ centered at $z_0$: $\{z : |z - z_0| = r\}$.
- The open disk: $D(z_0, r) = \{z : |z - z_0| < r\}$.
- The punctured disk: $D'(z_0, r) = \{z : 0 < |z - z_0| < r\}$.
- The upper half-plane: $\{z : \mathrm{Im}(z) > 0\}$.

## Modulus and Argument

The modulus $|z| = \sqrt{x^2 + y^2}$ is the distance from $z$ to the origin. The argument $\arg z$ is the angle $\theta$ that the ray from $0$ to $z$ makes with the positive real axis, measured counterclockwise. It satisfies $\cos\theta = x/|z|$ and $\sin\theta = y/|z|$, and it is determined modulo $2\pi$.

**Definition.** The principal argument $\mathrm{Arg}\, z \in (-\pi, \pi]$ is the unique argument of $z \neq 0$ lying in the interval $(-\pi, \pi]$.

The argument is not defined at $z = 0$, and the principal argument is discontinuous on the negative real axis (where it jumps from $\pi$ to values approaching $-\pi$). This discontinuity is the prototype for all branch cut phenomena in complex analysis.

## Multiplication as Rotation and Scaling

**Theorem.** If $z = |z|(\cos\alpha + i\sin\alpha)$ and $w = |w|(\cos\beta + i\sin\beta)$, then
$$zw = |z||w|\bigl(\cos(\alpha + \beta) + i\sin(\alpha + \beta)\bigr).$$

In words: multiplying by $w$ scales by $|w|$ and rotates by $\arg w$. This is the central geometric fact of complex multiplication.

**Proof.** Direct computation using the addition formulas for sine and cosine:
$$\cos\alpha\cos\beta - \sin\alpha\sin\beta = \cos(\alpha+\beta), \qquad \cos\alpha\sin\beta + \sin\alpha\cos\beta = \sin(\alpha+\beta). \quad \square$$

**Consequence.** Multiplication by $i = e^{i\pi/2}$ is a $90^\circ$ counterclockwise rotation. Multiplication by $-1 = e^{i\pi}$ is a $180^\circ$ rotation. Multiplication by $e^{i\theta}$ for general $\theta$ is a pure rotation by $\theta$.

**Worked example.** Find the image of the square with vertices $1, i, -1, -i$ under multiplication by $1 + i$.

We have $|1 + i| = \sqrt{2}$ and $\arg(1+i) = \pi/4$. So multiplication by $1 + i$ rotates by $45^\circ$ and scales by $\sqrt{2}$. The four vertices map as follows:
$$1 \mapsto 1+i, \quad i \mapsto i \cdot (1+i) = i - 1, \quad -1 \mapsto -1-i, \quad -i \mapsto -i+1.$$
The image is again a square, rotated $45^\circ$ and scaled by $\sqrt{2}$.

## The Complex Conjugate Geometrically

The conjugate $\bar{z} = x - iy$ is the reflection of $z$ in the real axis. Algebraically, conjugation is an automorphism of $\mathbb{C}$ that fixes $\mathbb{R}$:
$$\overline{z + w} = \bar{z} + \bar{w}, \qquad \overline{zw} = \bar{z}\bar{w}.$$
Geometrically, $z$ and $\bar{z}$ are symmetric about the real axis, and $z + \bar{z} = 2\mathrm{Re}(z)$, $z - \bar{z} = 2i\,\mathrm{Im}(z)$.

## The Riemann Sphere

The complex plane can be compactified by adding a single point at infinity, denoted $\infty$, to obtain the extended complex plane $\hat{\mathbb{C}} = \mathbb{C} \cup \{\infty\}$. This set can be put into bijection with the unit sphere $S^2 \subset \mathbb{R}^3$ via stereographic projection: project from the north pole $N = (0,0,1)$ through a point $(X, Y, 0)$ on the equatorial plane (identified with $\mathbb{C}$) to the sphere. Explicitly, the point $z = x + iy$ corresponds to
$$\left(\frac{2x}{|z|^2+1},\; \frac{2y}{|z|^2+1},\; \frac{|z|^2-1}{|z|^2+1}\right) \in S^2,$$
and the north pole $N$ corresponds to $\infty$.

The Riemann sphere is the natural domain for Mobius transformations (studied in Unit 04), which extend continuously to $\hat{\mathbb{C}}$ and act as bijections of the sphere. Stereographic projection has the beautiful property of mapping circles and lines in $\mathbb{C}$ to circles on $S^2$ (where lines in $\mathbb{C}$ correspond to circles through $N$).

## Geometric Interpretation of Key Inequalities

The triangle inequality $|z + w| \leq |z| + |w|$ asserts that the length of one side of a triangle is at most the sum of the other two sides — the standard Euclidean fact. Equality holds when $z$ and $w$ point in the same direction from the origin, i.e., when $w = tz$ for some $t \geq 0$.

The inequality $|\mathrm{Re}(z)| \leq |z|$ and $|\mathrm{Im}(z)| \leq |z|$ are geometric projections: the real and imaginary parts are at most as large as the modulus. These appear constantly in estimates for integrals.

## Applications in Physics

The geometric representation of $\mathbb{C}$ is not merely a mnemonic. In two-dimensional electrostatics and fluid mechanics, the complex plane is the physical plane, and complex-valued functions encode vector fields. The real and imaginary parts of an analytic function $f = u + iv$ (when $f$ satisfies the Cauchy-Riemann equations) are the velocity potential and stream function of an irrotational, incompressible fluid flow. The deep connection between complex analysis and two-dimensional physics is one of the great themes of Unit 04 on conformal mapping.
