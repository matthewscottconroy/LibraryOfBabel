# Rational Trigonometric Integrals

Integrals of the form $\int_0^{2\pi} R(\cos\theta, \sin\theta)\, d\theta$, where $R$ is a rational function of its two arguments, arise frequently in physics and engineering. Elementary techniques (Weierstrass substitution $t = \tan(\theta/2)$, for instance) become cumbersome for all but the simplest $R$. The substitution $z = e^{i\theta}$ converts such integrals into contour integrals over the unit circle, to which the residue theorem applies.

## The Unit Circle Substitution

Let $z = e^{i\theta}$, $\theta \in [0, 2\pi]$. Then:
$$\cos\theta = \frac{e^{i\theta} + e^{-i\theta}}{2} = \frac{z + z^{-1}}{2} = \frac{z^2 + 1}{2z},$$
$$\sin\theta = \frac{e^{i\theta} - e^{-i\theta}}{2i} = \frac{z - z^{-1}}{2i} = \frac{z^2 - 1}{2iz},$$
$$d\theta = \frac{dz}{iz}.$$

Substituting into the integral:
$$\int_0^{2\pi} R(\cos\theta, \sin\theta)\, d\theta = \oint_{|z|=1} R\!\left(\frac{z^2+1}{2z}, \frac{z^2-1}{2iz}\right)\frac{dz}{iz} = \oint_{|z|=1} f(z)\, dz,$$
where $f(z)$ is a rational function of $z$ (after simplification). Apply the residue theorem:
$$\int_0^{2\pi} R(\cos\theta, \sin\theta)\, d\theta = 2\pi i \sum_{|z_k|<1}\mathrm{Res}(f; z_k),$$
summing over all poles of $f$ strictly inside the unit circle.

## Worked Examples

**Example 1.** Evaluate $\displaystyle I = \int_0^{2\pi}\frac{d\theta}{2 + \cos\theta}$.

Substituting: $\cos\theta = (z^2+1)/(2z)$ and $d\theta = dz/(iz)$:
$$I = \oint_{|z|=1} \frac{1}{2 + (z^2+1)/(2z)}\cdot\frac{dz}{iz} = \oint_{|z|=1}\frac{1}{\frac{4z + z^2 + 1}{2z}}\cdot\frac{dz}{iz} = \oint_{|z|=1}\frac{2z}{z^2 + 4z + 1}\cdot\frac{dz}{iz} = \frac{2}{i}\oint_{|z|=1}\frac{dz}{z^2 + 4z + 1}.$$

Roots of $z^2 + 4z + 1 = 0$: $z = (-4 \pm \sqrt{12})/2 = -2 \pm \sqrt{3}$.

$z_1 = -2 + \sqrt{3} \approx -0.27$ (inside $|z|=1$).
$z_2 = -2 - \sqrt{3} \approx -3.73$ (outside $|z|=1$).

$\mathrm{Res}$ at $z_1$: $\frac{1}{2z_1 + 4} = \frac{1}{2(-2+\sqrt{3})+4} = \frac{1}{2\sqrt{3}}$.

$$I = \frac{2}{i}\cdot 2\pi i\cdot\frac{1}{2\sqrt{3}} = \frac{2\pi}{\sqrt{3}}. \quad \square$$

**Example 2.** Evaluate $\displaystyle I = \int_0^{2\pi}\frac{\cos(2\theta)}{5 - 4\cos\theta}\, d\theta$.

Note $\cos(2\theta) = \mathrm{Re}(e^{2i\theta}) = \mathrm{Re}(z^2)$ on the unit circle. The integral becomes $\mathrm{Re}\!\left(\oint_{|z|=1}\frac{z^2}{5-4\cdot\frac{z^2+1}{2z}}\cdot\frac{dz}{iz}\right)$.

Simplify denominator: $5 - 4\cdot\frac{z^2+1}{2z} = \frac{10z - 4z^2 - 4}{2z} = \frac{-4z^2 + 10z - 4}{2z} = \frac{-2(2z^2 - 5z + 2)}{2z} = \frac{-(2z-1)(z-2)}{z}$.

So $f(z) = \frac{z^2}{\frac{-(2z-1)(z-2)}{z}}\cdot\frac{1}{iz} = \frac{z^2 \cdot z}{-(2z-1)(z-2)} \cdot \frac{1}{iz} = \frac{z^2}{-i(2z-1)(z-2)}$.

Wait, let me redo: $d\theta = dz/(iz)$, so:
$I = \oint \frac{z^2}{\frac{-(2z-1)(z-2)}{z}} \cdot \frac{dz}{iz} = \oint \frac{z^3}{-(2z-1)(z-2)}\cdot\frac{dz}{iz} = \frac{1}{-i}\oint \frac{z^2}{(2z-1)(z-2)}\, dz$.

Poles: $z = 1/2$ (inside), $z = 2$ (outside).

$\mathrm{Res}$ at $z = 1/2$: $\frac{(1/2)^2}{2(1/2 - 2)} = \frac{1/4}{2(-3/2)} = \frac{1/4}{-3} = -\frac{1}{12}$.

$I = \frac{1}{-i}\cdot 2\pi i\cdot(-\frac{1}{12}) = \frac{2\pi}{12} = \frac{\pi}{6}$. $\square$

**Example 3.** Evaluate $\displaystyle I = \int_0^{2\pi}\frac{d\theta}{(a + b\cos\theta)^2}$, $a > b > 0$.

After the substitution, $f(z) = \frac{-4iz^2}{b^2(z^2 + 2(a/b)z + 1)^2}$.

Let $\alpha = a/b > 1$. The denominator factors as $b^2(z - z_1)^2(z - z_2)^2$ where $z_1 = -\alpha + \sqrt{\alpha^2-1}$, $z_2 = -\alpha - \sqrt{\alpha^2-1}$, and only $z_1$ is inside the unit circle (since $|z_1| = 1/(|\alpha + \sqrt{\alpha^2-1}|) < 1$).

The residue at $z_1$ (a double pole) is computed by differentiating $\frac{d}{dz}\left[\frac{-4iz^2}{b^2(z-z_2)^2}\right]$ at $z = z_1$. The final answer is:
$$I = \frac{2\pi a}{(a^2-b^2)^{3/2}}.$$

## Validity Conditions

The method requires that $R(\cos\theta, \sin\theta)$ is well-defined for all $\theta \in [0, 2\pi]$ and that the resulting rational function $f(z)$ has no poles on the unit circle $|z| = 1$. If $R$ has a singularity (e.g., if the denominator of $R$ vanishes at some $\theta_0$), the integral is either improper (and may or may not converge) or the principal value must be taken.

## Efficiency of the Method

The unit circle method reduces a trigonometric integral to an algebraic problem: factoring a polynomial and computing residues. For example, evaluating $\int_0^{2\pi} \frac{d\theta}{(a + b\cos\theta)^n}$ for general $n$ by real methods requires $n-1$ integration by parts plus a reduction formula, while the contour method treats all $n$ uniformly.
