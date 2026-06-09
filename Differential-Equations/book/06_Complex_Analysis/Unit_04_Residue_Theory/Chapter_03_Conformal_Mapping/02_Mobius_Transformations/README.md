# Mobius Transformations

A Mobius transformation (also called a fractional linear transformation or bilinear map) is a function of the form
$$f(z) = \frac{az + b}{cz + d}, \qquad a, b, c, d \in \mathbb{C}, \quad ad - bc \neq 0.$$
Mobius transformations are the automorphisms of the Riemann sphere $\hat{\mathbb{C}} = \mathbb{C} \cup \{\infty\}$: they are the conformal bijections from $\hat{\mathbb{C}}$ to itself. Their algebraic and geometric properties are extraordinarily rich, and they appear throughout complex analysis, hyperbolic geometry, and mathematical physics.

## Basic Properties

**Analyticity and derivative.** For $cz + d \neq 0$, $f$ is analytic with:
$$f'(z) = \frac{a(cz+d) - c(az+b)}{(cz+d)^2} = \frac{ad - bc}{(cz+d)^2}.$$
Since $ad - bc \neq 0$, $f'(z) \neq 0$ wherever $f$ is defined, so $f$ is conformal at every finite point where it is defined.

**Extension to $\hat{\mathbb{C}}$.** Define $f(-d/c) = \infty$ and $f(\infty) = a/c$ (if $c \neq 0$), or $f(\infty) = \infty$ (if $c = 0$). With these definitions, $f : \hat{\mathbb{C}} \to \hat{\mathbb{C}}$ is a bijection.

**Inverse.** The inverse of $f(z) = (az+b)/(cz+d)$ is $f^{-1}(w) = (dw - b)/(-cw + a)$, corresponding to the matrix $\begin{pmatrix} d & -b \\ -c & a \end{pmatrix}$.

**Group structure.** Mobius transformations form a group under composition, isomorphic to $\mathrm{PGL}(2, \mathbb{C}) = \mathrm{GL}(2, \mathbb{C})/\mathbb{C}^*$. The composition of $f$ and $g$ corresponds to matrix multiplication of their coefficient matrices.

## Three-Point Determination

**Theorem.** A Mobius transformation is uniquely determined by specifying the images of three distinct points.

**Proof.** The cross-ratio $(z, z_1, z_2, z_3) = \frac{(z-z_1)(z_2-z_3)}{(z-z_3)(z_2-z_1)}$ is invariant under Mobius transformations: $(f(z), f(z_1), f(z_2), f(z_3)) = (z, z_1, z_2, z_3)$. Given the images of three points $z_1 \mapsto w_1$, $z_2 \mapsto w_2$, $z_3 \mapsto w_3$, the unique Mobius transformation is the one satisfying $(f(z), w_1, w_2, w_3) = (z, z_1, z_2, z_3)$. $\square$

**Standard maps.** The Mobius transformation sending $z_1, z_2, z_3$ to $0, 1, \infty$ respectively is:
$$f(z) = \frac{(z - z_1)(z_2 - z_3)}{(z - z_3)(z_2 - z_1)}.$$

## Circle-Preserving Property

**Theorem.** Mobius transformations map circles and lines to circles and lines (where a line is considered a circle through $\infty$).

**Proof.** A circle or line in $\mathbb{C}$ can be written as $\alpha|z|^2 + \beta z + \bar{\beta}\bar{z} + \gamma = 0$ with $\alpha, \gamma \in \mathbb{R}$ and $\beta \in \mathbb{C}$. Under $w = f(z) = (az+b)/(cz+d)$, substituting $z = (dw-b)/(-cw+a)$ yields an equation of the same form in $w$. $\square$

More geometrically: every Mobius transformation is a composition of translations ($z \mapsto z + b$), scalings ($z \mapsto az$), and the inversion ($z \mapsto 1/z$). Each of these individually maps circles/lines to circles/lines, and their composition does too.

## Worked Examples

**Example 1.** Find the Mobius transformation mapping $0 \mapsto 1$, $i \mapsto 0$, $\infty \mapsto -1$.

The image of $\infty$ is $a/c = -1$, so $a = -c$. The image of $0$ is $b/d = 1$, so $b = d$. The image of $i$ is $(ai + b)/(ci + d) = 0$, so $ai + b = 0$, giving $b = -ai$.

From $b = d$: $d = -ai$. From $a = -c$: $c = -a$.

$f(z) = \frac{az + (-ai)}{(-a)z + (-ai)} = \frac{z - i}{-z - i} = \frac{i - z}{i + z}$.

Check: $f(0) = i/i = 1$. $f(i) = (i-i)/(i+i) = 0$. $f(\infty) = -1/1 = -1$. $\square$

**Example 2.** The map $f(z) = \frac{z - i}{z + i}$ maps the upper half-plane $H = \{z : \mathrm{Im}(z) > 0\}$ to the unit disk $\mathbb{D} = \{w : |w| < 1\}$.

On the real axis $z = x$: $|f(x)| = |x - i|/|x + i| = 1$. So $f$ maps $\mathbb{R}$ to $|w| = 1$.

For $z = iy$, $y > 0$: $f(iy) = (iy - i)/(iy + i) = i(y-1)/(i(y+1)) = (y-1)/(y+1) \in (-1, 1)$. So $f$ maps the imaginary axis to $(-1, 1)$, inside $\mathbb{D}$.

By continuity and the open mapping theorem, $f$ maps $H$ to $\mathbb{D}$. The inverse is $z = i(1+w)/(1-w)$. $\square$

## Classification of Mobius Transformations

Fixed points of $f(z) = (az+b)/(cz+d)$ satisfy $(az+b)/(cz+d) = z$, i.e., $cz^2 + (d-a)z - b = 0$. Generically this has two solutions. The nature of the transformation is classified by its trace $\tau = a + d$ (i.e., the trace of the matrix $\begin{pmatrix}a&b\\c&d\end{pmatrix}$, normalized so that $ad-bc=1$):

- **Elliptic:** $\tau^2 \in [0, 4)$ (real, with $|\tau| < 2$). Two distinct fixed points; the transformation is a rotation around one fixed point (in the appropriate metric). No invariant real circles.
- **Hyperbolic:** $\tau^2 > 4$ (real and $> 4$). Two distinct fixed points on $\hat{\mathbb{R}}$; the transformation is a dilation along the axis through the fixed points.
- **Parabolic:** $\tau^2 = 4$ (i.e., $\tau = \pm 2$). One fixed point (a double root); the transformation is a translation in the appropriate coordinate.
- **Loxodromic:** $\tau^2 \in \mathbb{C} \setminus [0, 4]$. Two fixed points; the transformation is a combination of rotation and dilation.

**Worked example.** Classify $f(z) = (2z + 1)/(z + 2)$.

Matrix: $\begin{pmatrix}2&1\\1&2\end{pmatrix}$, $\det = 3$. Normalize: divide by $\sqrt{3}$: matrix $\frac{1}{\sqrt{3}}\begin{pmatrix}2&1\\1&2\end{pmatrix}$, trace $= 4/\sqrt{3} \approx 2.31 > 2$. So $\tau^2 = 16/3 > 4$: hyperbolic. Fixed points: $z^2 + (2-2)z - 1 = 0$... from $z = (2z+1)/(z+2)$: $z^2 + 2z = 2z + 1$, so $z^2 = 1$, giving $z = \pm 1$. Both real. $\square$

## The Schwarz Lemma

**Theorem (Schwarz Lemma).** Let $f : \mathbb{D} \to \mathbb{D}$ be analytic with $f(0) = 0$. Then $|f(z)| \leq |z|$ for all $z \in \mathbb{D}$ and $|f'(0)| \leq 1$. If $|f(z_0)| = |z_0|$ for some $z_0 \neq 0$, or if $|f'(0)| = 1$, then $f(z) = e^{i\theta}z$ for some real $\theta$ (a rotation).

The automorphisms of the unit disk (conformal bijections $\mathbb{D} \to \mathbb{D}$) are exactly the Mobius transformations:
$$f(z) = e^{i\theta}\frac{z - a}{1 - \bar{a}z}, \qquad |a| < 1, \quad \theta \in \mathbb{R}.$$
These are the Blaschke factors (up to rotation).
