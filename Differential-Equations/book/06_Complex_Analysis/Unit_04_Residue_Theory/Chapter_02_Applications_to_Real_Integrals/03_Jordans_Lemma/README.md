# Jordan's Lemma

Jordan's lemma extends the semicircular contour technique to integrals of the form $\int_{-\infty}^\infty f(x) e^{i\xi x}\, dx$, which arise in Fourier analysis and the inversion of Laplace transforms. The issue is that $|e^{i\xi z}|$ does not simply tend to zero on a large semicircle: it equals $e^{-\xi y}$, which decays for $y > 0$ (when $\xi > 0$) but only if the contour stays in the upper half-plane. Jordan's lemma makes this decay quantitative and shows it can compensate for a slower decay of $f$ than the ML inequality would ordinarily require.

## Statement of Jordan's Lemma

**Lemma (Jordan).** Let $\xi > 0$, and let $\Gamma_R = \{Re^{i\theta} : \theta \in [0, \pi]\}$ be the upper semicircle of radius $R$. If $M_R = \max_{z \in \Gamma_R}|f(z)| \to 0$ as $R \to \infty$, then:
$$\lim_{R \to \infty}\int_{\Gamma_R} f(z) e^{i\xi z}\, dz = 0.$$

**Note.** The condition is only $M_R \to 0$, not $RM_R \to 0$ as in the plain ML inequality. This means Jordan's lemma applies to $f$ with $|f| = O(1/R)$ (like rational functions with $\deg q = \deg p + 1$), whereas the plain ML estimate requires $O(1/R^2)$.

## Proof of Jordan's Lemma

**Proof.** Parametrize $\Gamma_R$ by $z = Re^{i\theta} = R(\cos\theta + i\sin\theta)$, $\theta \in [0, \pi]$:
$$\left|\int_{\Gamma_R} f(z)e^{i\xi z}\, dz\right| \leq \int_0^\pi |f(Re^{i\theta})| e^{-\xi R\sin\theta} R\, d\theta \leq M_R R\int_0^\pi e^{-\xi R\sin\theta}\, d\theta.$$

The key estimate is: $\sin\theta \geq \frac{2\theta}{\pi}$ for $\theta \in [0, \pi/2]$ (the concavity of $\sin$ on $[0, \pi]$ gives $\sin\theta \geq 2\theta/\pi$ on the first half). Using the symmetry of $\sin$ around $\pi/2$:
$$\int_0^\pi e^{-\xi R\sin\theta}\, d\theta = 2\int_0^{\pi/2} e^{-\xi R\sin\theta}\, d\theta \leq 2\int_0^{\pi/2} e^{-\xi R \cdot 2\theta/\pi}\, d\theta = 2\cdot\frac{\pi}{2\xi R}(1 - e^{-\xi R}) \leq \frac{\pi}{\xi R}.$$

Therefore:
$$\left|\int_{\Gamma_R} f(z)e^{i\xi z}\, dz\right| \leq M_R R \cdot \frac{\pi}{\xi R} = \frac{\pi M_R}{\xi} \to 0. \quad \square$$

## Application to Fourier Integrals

**Worked example 1.** Evaluate $\displaystyle I = \int_{-\infty}^\infty \frac{e^{i\xi x}}{x^2 + a^2}\, dx$ for $\xi > 0$, $a > 0$.

Close in the upper half-plane. Pole in upper half-plane: $z = ia$.
$\mathrm{Res}\!\left(\frac{e^{i\xi z}}{z^2+a^2}; ia\right) = \frac{e^{i\xi \cdot ia}}{2ia} = \frac{e^{-\xi a}}{2ia}$.

$|f(z)| = 1/(|z|^2 - a^2) = O(1/R^2)$ on $|z| = R$, so Jordan's lemma applies (and actually so does ML). By Jordan's lemma (the exponential integral over $\Gamma_R$ vanishes):
$$I = 2\pi i \cdot \frac{e^{-\xi a}}{2ia} = \frac{\pi e^{-\xi a}}{a}.$$

Taking real and imaginary parts:
$$\int_{-\infty}^\infty \frac{\cos(\xi x)}{x^2 + a^2}\, dx = \frac{\pi e^{-\xi a}}{a}, \qquad \int_{-\infty}^\infty \frac{\sin(\xi x)}{x^2 + a^2}\, dx = 0$$
(the latter by oddness of the integrand, consistent with the imaginary part being $0$). $\square$

**Worked example 2.** Evaluate $\displaystyle I = \int_{-\infty}^\infty \frac{x\sin(\xi x)}{x^2 + a^2}\, dx$ for $\xi > 0$, $a > 0$.

Write $x\sin(\xi x) = \mathrm{Im}(x e^{i\xi x})$. Consider:
$$J = \int_{-\infty}^\infty \frac{x e^{i\xi x}}{x^2 + a^2}\, dx.$$
Here $|f(z)| = |z|/(|z|^2 - a^2) = O(1/R)$ on $|z| = R$, so Jordan's lemma applies.

Pole in upper half-plane: $z = ia$. $\mathrm{Res} = \frac{ia \cdot e^{i\xi(ia)}}{2ia} = \frac{e^{-\xi a}}{2}$.

$J = 2\pi i \cdot \frac{e^{-\xi a}}{2} = \pi i e^{-\xi a}$.

$I = \mathrm{Im}(J) = \pi e^{-\xi a}$. $\square$

## Lower Half-Plane: $\xi < 0$

For $\xi < 0$, close in the lower half-plane (where $e^{i\xi z} = e^{-\xi y}$ and $y < 0$ gives $-\xi(-|y|) < 0$ since $-\xi > 0$... let me state this clearly):

For $\xi < 0$: $|e^{i\xi z}| = e^{-\xi y}$ with $\xi < 0$ so $-\xi > 0$. On the lower semicircle, $y < 0$, so $-\xi y > 0$ and $e^{-\xi y} \to \infty$ — the exponential grows. Instead, close in the lower half-plane: $y < 0$ and $-\xi y = |\xi||y| > 0$... 

Actually: for $\xi < 0$, use the lower semicircle $y \leq 0$ where $|e^{i\xi z}| = e^{-\xi\,\mathrm{Im}(z)}$. With $\mathrm{Im}(z) = R\sin\theta < 0$ for $\theta \in (-\pi, 0)$ and $\xi < 0$: $-\xi\,\mathrm{Im}(z) = |\xi| \cdot R|\sin\theta|$ — this grows! The correct statement is: for $\xi < 0$, close in the lower half-plane where $\mathrm{Im}(z) < 0$, and then $-\xi\,\mathrm{Im}(z) = |\xi||\mathrm{Im}(z)| < 0$ — wait.

For $\xi < 0$ and $y < 0$: $|e^{i\xi(x+iy)}| = e^{-\xi y}$. Since $\xi < 0$ and $y < 0$: $-\xi y = |\xi||y| > 0$... No: $-\xi y = -(< 0)(< 0) = -(> 0) < 0$. So $e^{-\xi y} = e^{(< 0)} < 1$: the exponential decays. Good. So for $\xi < 0$, close in the lower half-plane, and the residue theorem gives a sum over poles in the lower half-plane (with a sign reversal for the clockwise orientation).

## The Dirichlet Integral via Jordan's Lemma

**Worked example 3.** Evaluate $\displaystyle I = \int_0^\infty \frac{\sin x}{x}\, dx$.

Consider $f(z) = e^{iz}/z$. The real axis pole at $z = 0$ is avoided by an indentation: small upper semicircle $C_\varepsilon$ of radius $\varepsilon$ (indenting upward, so $z=0$ is outside the contour). The integrand has no poles in the upper half-plane (the singularity at $0$ is removable: $e^{iz}/z \to i$ as $z \to 0$... no, actually $e^{iz}/z$ has a simple pole at $0$ with residue $1$). 

The full contour: $[-R, -\varepsilon]$, upper small semicircle $C_\varepsilon$ from $-\varepsilon$ to $\varepsilon$ (clockwise, i.e., going above), $[\varepsilon, R]$, upper large semicircle $\Gamma_R$.

No poles inside: integral $= 0$. Large semicircle $\to 0$ by Jordan. Small semicircle contributes $-\pi i \cdot 1 = -\pi i$ (clockwise contribution is $-$ of $\pi i$ times the residue at $0$, which is $1$).

So: $\int_{-\infty}^\infty \frac{e^{ix}}{x}\, dx - \pi i = 0$... but this integral doesn't converge absolutely. The principal value: $\mathrm{PV}\int_{-\infty}^\infty \frac{e^{ix}}{x}\, dx = \pi i$.

Taking imaginary parts: $\mathrm{PV}\int_{-\infty}^\infty \frac{\sin x}{x}\, dx = \pi$. By evenness: $I = \pi/2$. $\square$

## Relationship to the Inversion Integral of the Laplace Transform

The Bromwich integral for the inverse Laplace transform,
$$f(t) = \frac{1}{2\pi i}\int_{c - i\infty}^{c + i\infty} F(s) e^{st}\, ds,$$
is evaluated by a semicircular contour in the left half-plane (for $t > 0$). The contributions from the large semicircle vanish by Jordan's lemma (with the rôle of $\xi$ played by $t$), and the inverse transform equals the sum of residues of $F(s)e^{st}$ over all poles of $F$.
