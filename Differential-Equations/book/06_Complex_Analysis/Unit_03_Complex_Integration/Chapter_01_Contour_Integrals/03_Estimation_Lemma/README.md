# The Estimation Lemma (ML Inequality)

The estimation lemma, often called the ML inequality, is a simple but indispensable tool: it bounds the modulus of a contour integral by the product of the maximum modulus of the integrand and the length of the contour. While it rarely gives sharp estimates, it is powerful enough to establish the vanishing of auxiliary integrals in virtually every contour integration argument. This section states and proves the lemma, discusses when the bound is tight, and illustrates its use in several canonical situations.

## Statement and Proof

**Theorem (Estimation Lemma / ML Inequality).** Let $C$ be a contour of length $L$, and let $f$ be continuous on $C$ with $|f(z)| \leq M$ for all $z$ on $C$. Then
$$\left|\int_C f(z)\, dz\right| \leq ML.$$

**Proof.** Let $I = \int_C f(z)\, dz$. If $I = 0$ the inequality holds trivially. Otherwise, write $I = |I| e^{i\theta}$ so that $|I| = e^{-i\theta} I$. Then:
$$|I| = \mathrm{Re}(e^{-i\theta} I) = \mathrm{Re}\int_C e^{-i\theta} f(z)\, dz = \int_a^b \mathrm{Re}(e^{-i\theta} f(z(t))) |z'(t)| \cdot \frac{|z'(t)|}{|z'(t)|}... $$

More cleanly: using the parametrization $z = z(t)$, $t \in [a, b]$:
$$|I| = \mathrm{Re}(e^{-i\theta}I) = \int_a^b \mathrm{Re}(e^{-i\theta}f(z(t)))z'(t)\, dt \leq \int_a^b |e^{-i\theta}f(z(t))z'(t)|\, dt = \int_a^b |f(z(t))| |z'(t)|\, dt \leq M \int_a^b |z'(t)|\, dt = ML. \quad \square$$

## When Is the Bound Tight?

The ML bound is an equality $\left|\int_C f\, dz\right| = ML$ only if:
- $|f(z)| = M$ at every point of $C$ (the integrand has constant modulus on $C$), and
- $\arg(f(z) z'(t))$ is constant along $C$ (the integrand times the velocity always points in the same direction in $\mathbb{C}$).

These conditions are rarely met simultaneously in practice, so the ML bound often overestimates, sometimes dramatically. The key application is not sharpness but sufficiency: proving that an integral goes to zero.

## Canonical Application 1: Large Semicircle Goes to Zero

**Lemma.** If $f(z) \to 0$ uniformly as $|z| \to \infty$ (i.e., $M_R = \max_{|z|=R} |f(z)| \to 0$ as $R \to \infty$), then the integral of $f$ over the semicircle $C_R = \{Re^{i\theta} : \theta \in [0, \pi]\}$ goes to zero:
$$\left|\int_{C_R} f(z)\, dz\right| \leq M_R \cdot \pi R \to 0 \quad \text{if } RM_R \to 0.$$

**Worked example.** Let $f(z) = \frac{1}{z^2 + 1}$. On $|z| = R$ with $R > 1$:
$$|f(z)| = \frac{1}{|z^2 + 1|} \leq \frac{1}{|z|^2 - 1} = \frac{1}{R^2 - 1}.$$
The length of $C_R$ is $\pi R$, so:
$$\left|\int_{C_R} \frac{dz}{z^2 + 1}\right| \leq \frac{\pi R}{R^2 - 1} \to 0 \quad \text{as } R \to \infty.$$
This is the essential estimate used to evaluate $\int_{-\infty}^{\infty} \frac{dx}{x^2+1} = \pi$ by residues. $\square$

## Canonical Application 2: Small Circle Goes to Zero

**Lemma.** If $f$ has a simple pole at $z_0$ with $|(z - z_0)f(z)| \leq M_\varepsilon$ near $z_0$ (where $M_\varepsilon \to |\mathrm{Res}(f; z_0)|$ as $\varepsilon \to 0$), then the integral over the small circle $C_\varepsilon = \{z_0 + \varepsilon e^{it} : t \in [0, \theta]\}$ satisfies $\int_{C_\varepsilon} f(z)\, dz \to i\theta \cdot \mathrm{Res}(f; z_0)$ as $\varepsilon \to 0$.

More precisely, for a simple pole write $f(z) = \frac{a}{z - z_0} + g(z)$ where $g$ is bounded near $z_0$. Then $\int_{C_\varepsilon} g(z)\, dz \to 0$ by ML (since $|g| \leq M$ and $L = \theta\varepsilon \to 0$), while $\int_{C_\varepsilon} \frac{a}{z - z_0}\, dz = ai\theta$ (computed directly).

## Canonical Application 3: Bounding Series Remainders

The ML inequality is also used to show that power series converge uniformly on compact subsets of their disk of convergence, and that the remainder of a Taylor approximation goes to zero.

**Worked example.** Bound $\left|\int_C \frac{dz}{z(z-1)}\right|$ where $C$ is the line segment from $2$ to $2 + 2i$.

On $C$, parametrize $z = 2 + it$, $t \in [0, 2]$. Then $|z| = \sqrt{4 + t^2} \geq 2$ and $|z - 1| = |1 + it| = \sqrt{1 + t^2} \geq 1$. So:
$$|f(z)| = \frac{1}{|z||z-1|} \leq \frac{1}{2 \cdot 1} = \frac{1}{2}.$$
The length of $C$ is $2$. By ML:
$$\left|\int_C \frac{dz}{z(z-1)}\right| \leq \frac{1}{2} \cdot 2 = 1.$$

## The Integral Version of the Triangle Inequality

The ML inequality is the integral analogue of the modulus inequality $|z + w| \leq |z| + |w|$. More precisely, it says:
$$\left|\int_C f(z)\, dz\right| \leq \int_C |f(z)|\, |dz|.$$
The right-hand side is a real integral and can be estimated by standard real-variable techniques. This reduction from a complex bound to a real bound is the key step in virtually all contour integral estimates.

## Dependence on Geometry: The Importance of $L$

The lemma shows that the estimate depends on both the size of $f$ and the length of the path. In residue calculations, one often encounters a tension: making the contour large increases the length $L$ but also usually decreases $M$ (because $f(z) \to 0$ as $|z| \to \infty$). The ML inequality is useful precisely when $M$ decreases faster than $L$ increases. For rational functions $f(z) = P(z)/Q(z)$ with $\deg Q \geq \deg P + 2$, we have $|f(z)| = O(1/R^2)$ on $|z| = R$, so $ML = O(1/R) \to 0$. This is the key estimate in evaluating improper integrals by the semicircular contour method.
