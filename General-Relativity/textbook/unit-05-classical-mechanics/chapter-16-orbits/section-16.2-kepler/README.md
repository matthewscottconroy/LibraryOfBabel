# Section 16.2: Kepler's Laws and the Two-Body Problem

---

## Section Introduction

Kepler's three laws, deduced from Tycho Brahe's precise observational data between 1609 and 1619, stood for nearly eighty years as a purely empirical description of planetary motion. When Newton derived all three from his law of gravitation in the *Principia* (1687), it was one of the greatest theoretical achievements in history: abstract mathematics predicting, from first principles, the very regularities that had taken decades of patient observation to uncover.

This derivation is worth doing carefully. Kepler's first law (ellipses) requires solving Binet's equation. Kepler's second law (equal areas) follows from angular momentum conservation. Kepler's third law ($T^2 \propto a^3$) comes from computing the orbital period. Each is more than a formula — each reveals a deep feature of the $1/r$ potential.

The Laplace-Runge-Lenz vector — a conserved vector pointing from the focus to the perihelion — provides a deeper explanation for why orbits are closed: it is an extra conservation law that the $1/r$ potential possesses. In GR, this vector is not conserved (the orbit precesses), and computing how it drifts is essentially the problem of perihelion precession.

---

## 16.2.1 Kepler's First Law: Orbits Are Conic Sections

**Setting up Binet's equation**: For Newtonian gravity $F(r) = -GMm/r^2$, so $F(1/u) = -GMmu^2$. Binet's equation (Section 16.1.5) becomes:

$$\frac{d^2u}{d\phi^2} + u = \frac{GMm^2}{\ell^2} \equiv \frac{1}{p}$$

where $p = \ell^2/(GMm^2)$ is the **semi-latus rectum**.

**Solution**: The homogeneous equation $u'' + u = 0$ has solutions $\cos\phi$ and $\sin\phi$. A particular solution is $u = 1/p$ (constant). The general solution is:

$$u(\phi) = \frac{1}{p}(1 + e\cos(\phi - \phi_0))$$

Choosing $\phi_0 = 0$ (perihelion at $\phi = 0$):

$$r(\phi) = \frac{p}{1 + e\cos\phi}$$

This is the **polar equation of a conic section** in standard form. The parameter $e \geq 0$ is the **eccentricity**:
- $e = 0$: circle ($r = p =$ const)
- $0 < e < 1$: ellipse (with semi-major axis $a$ and $p = a(1-e^2)$)
- $e = 1$: parabola
- $e > 1$: hyperbola (unbound orbit)

The constant $e$ is determined by the initial conditions (energy and angular momentum):

$$e = \sqrt{1 + \frac{2E\ell^2}{G^2M^2m^3}}$$

For bound orbits ($E < 0$): $0 \leq e < 1$. For $E = -G^2M^2m^3/(2\ell^2)$: circular orbit ($e = 0$). □

**Cartesian form**: In Cartesian coordinates, $r = p/(1 + e\cos\phi)$ becomes $\sqrt{x^2+y^2} = p - ex$, squaring: $x^2(1-e^2) + 2epx + y^2 = p^2$. For an ellipse ($e < 1$):

$$\frac{(x + ae)^2}{a^2} + \frac{y^2}{b^2} = 1$$

where $a = p/(1-e^2)$ is the semi-major axis, $b = p/\sqrt{1-e^2} = a\sqrt{1-e^2}$ is the semi-minor axis, and the center is at $(-ae, 0)$. The Sun is at the focus $(0, 0)$, not the center.

---

## 16.2.2 Kepler's Second Law: Equal Areas in Equal Times

As shown in Section 16.1.4, for any central force:

$$\frac{dA}{dt} = \frac{\ell}{2m}$$

This is Kepler's second law. It is a pure consequence of angular momentum conservation and holds for all central forces, attractive or repulsive.

**Perihelion and aphelion**: At the turning points $r_{\min}$ (perihelion) and $r_{\max}$ (aphelion), $\dot{r} = 0$. Since $v_\phi = \ell/(mr)$, the orbital speed is $v_\phi = \ell/(mr_{\min})$ at perihelion and $v_\phi = \ell/(mr_{\max})$ at aphelion. The planet moves fastest at perihelion and slowest at aphelion.

For an ellipse: $r_{\min} = a(1-e)$ and $r_{\max} = a(1+e)$.

---

## 16.2.3 The Vis-Viva Equation

The total energy of an elliptic orbit can be expressed in terms of the semi-major axis alone:

$$E = -\frac{GMm}{2a}$$

*Derivation*: The total energy is $E = \frac{1}{2}m\dot{r}^2 + V_{\rm eff}(r)$. At the turning points $\dot{r} = 0$, so $E = V_{\rm eff}(r_{\min,\max})$. The two equations $E = -GMm/r_{\min,\max} + \ell^2/(2mr_{\min,\max}^2)$ have the product of roots $r_{\min}r_{\max} = \ell^2/(GMm^2) \cdot 1/(e-1)(e+1)/(1-e^2) = \ell^2p / (GMm^2(1-e^2))$... [working through the algebra gives $E = -GMm/(2a)$].

More elegantly: since $a = p/(1-e^2) = \ell^2/(GMm^2(1-e^2))$ and $E = -G^2M^2m^3(1-e^2)/(2\ell^2)$:

$$E = -\frac{G^2M^2m^3}{2\ell^2}(1-e^2) = -\frac{GMm}{2a}$$

The **vis-viva equation** gives the orbital speed at any point:

$$v^2 = GM\left(\frac{2}{r} - \frac{1}{a}\right)$$

*Proof*: $E = \frac{1}{2}mv^2 - GMm/r = -GMm/(2a)$, so $v^2 = GM(2/r - 1/a)$. □

**Escape velocity**: Setting $a \to \infty$ (parabolic orbit, $E = 0$): $v_{\rm esc} = \sqrt{2GM/r}$. From Earth's surface: $v_{\rm esc} = \sqrt{2 \times 6.67\times10^{-11} \times 5.97\times10^{24} / 6.37\times10^6} \approx 11.2$ km/s.

---

## 16.2.4 Kepler's Third Law

**Orbital period**: The area of an ellipse is $A = \pi a b = \pi a^2\sqrt{1-e^2}$. Since the areal velocity is $dA/dt = \ell/(2m) =$ const, the period is:

$$T = \frac{A}{dA/dt} = \frac{2m\pi a^2\sqrt{1-e^2}}{\ell}$$

Using $\ell = m\sqrt{GMp} = m\sqrt{GMa(1-e^2)}$:

$$T = \frac{2\pi a^2\sqrt{1-e^2}}{\sqrt{GMa(1-e^2)}} = \frac{2\pi a^{3/2}}{\sqrt{GM}}$$

Therefore:

$$T^2 = \frac{4\pi^2}{GM}a^3$$

This is **Kepler's third law**: the square of the period is proportional to the cube of the semi-major axis, with the constant $4\pi^2/(GM)$ depending only on the central mass.

**Historical significance**: Kepler stated this law in 1619 based on Brahe's data, but without knowing the constant or its physical meaning. Newton's derivation revealed that the constant encodes the mass of the Sun. This is now used in reverse: measuring orbital periods and semi-major axes gives the mass of any gravitating body (planets, stars, black holes, galaxy clusters).

**Measure a black hole's mass**: At the center of the Milky Way, stars orbit in periods of 10–100 years with semi-major axes of order 100–1000 AU. Applying $M = 4\pi^2 a^3/(GT^2)$ gives $M \approx 4 \times 10^6 M_\odot$ — the mass of Sagittarius A* [Ghez et al. (2008); Gillessen et al. (2009)].

---

## 16.2.5 The Laplace-Runge-Lenz Vector

Why are Keplerian orbits exactly closed? The answer is not just "angular momentum conservation" — that only explains why orbits stay in a plane. Closed orbits require an additional, "hidden" conservation law.

**Definition**: The **Laplace-Runge-Lenz vector** is:

$$\mathbf{A} = \mathbf{p} \times \mathbf{L} - GMm^2\hat{\mathbf{r}}$$

where $\mathbf{p} = m\dot{\mathbf{r}}$ is the momentum and $\mathbf{L} = m\mathbf{r} \times \dot{\mathbf{r}}$ is the angular momentum.

**Conservation**: $\dot{\mathbf{A}} = 0$ for the inverse-square law. *Proof*:

$$\dot{\mathbf{A}} = \dot{\mathbf{p}} \times \mathbf{L} + \mathbf{p} \times \dot{\mathbf{L}} - GMm^2\dot{\hat{\mathbf{r}}}$$

Since $\mathbf{L}$ is conserved ($\dot{\mathbf{L}} = 0$) and $\dot{\mathbf{p}} = -GMm\hat{\mathbf{r}}/r^2$:

$$\dot{\mathbf{A}} = -\frac{GMm}{r^2}\hat{\mathbf{r}} \times \mathbf{L} - GMm^2\frac{d}{dt}\left(\frac{\mathbf{r}}{r}\right)$$

A computation using $\mathbf{L} = m\mathbf{r} \times \dot{\mathbf{r}}$ shows $\hat{\mathbf{r}} \times \mathbf{L} = m(-r\dot{\mathbf{r}} + \dot{r}\mathbf{r})$ and $\frac{d}{dt}(\mathbf{r}/r) = \dot{\mathbf{r}}/r - \dot{r}\mathbf{r}/r^2$. Substituting and collecting terms shows $\dot{\mathbf{A}} = 0$. □

**Geometric meaning**: $\mathbf{A}$ points from the focus (origin) to the perihelion. Its magnitude is $|\mathbf{A}| = GMm^2 e$.

**Orbit equation from $\mathbf{A}$**: Take $\mathbf{A} \cdot \mathbf{r} = |\mathbf{A}|r\cos\phi = (\mathbf{p}\times\mathbf{L}) \cdot \mathbf{r} - GMm^2r = \mathbf{L}\cdot(\mathbf{r}\times\mathbf{p}) - GMm^2r = L^2/m - GMm^2r$. So:

$$r = \frac{\ell^2/(GMm^2)}{1 + e\cos\phi}$$

reproducing Kepler's first law without integrating Binet's equation.

**Why this matters for GR**: The LRL vector is conserved only for the exact $1/r$ potential. Any perturbation (from other planets, from the oblateness of the Sun, from GR) breaks this conservation. The precession of the perihelion is precisely the rate at which $\mathbf{A}$ rotates. Section 16.3 computes this rotation rate due to the GR correction.

**Algebraic structure**: The three components of $\mathbf{L}$ and the three components of $\mathbf{A}$ (restricted to $E < 0$) generate the Lie algebra $\mathfrak{so}(4)$ — the symmetry algebra of the 4-sphere $S^3$. This hidden $SO(4)$ symmetry is what enforces the exact closure of orbits. In quantum mechanics, it becomes the $SO(4)$ symmetry of the hydrogen atom, explaining the degeneracy of energy levels with different quantum numbers $\ell$ (a fact that is mysterious from the Schrödinger equation alone).

[Pauli, W. (1926). "Über das Wasserstoffspektrum vom Standpunkt der neuen Quantenmechanik." *Zeitschrift für Physik*, 36, 336–363. Pauli used the quantum analog of the LRL vector to derive the hydrogen spectrum algebraically — before Schrödinger's wave equation. One of the most elegant papers in quantum mechanics.]

---

## References

- Kepler, J. (1609). *Astronomia Nova.* Prague. [The first two laws: orbits are ellipses (I); equal areas in equal times (II). Based on Brahe's observations of Mars.]
- Kepler, J. (1619). *Harmonices Mundi.* Linz. [The third law: $T^2 \propto a^3$.]
- Newton, I. (1687). *Philosophiæ Naturalis Principia Mathematica.* Book I, Props. XI–XIII. London. [The derivation of elliptic orbits from the inverse-square law. Proves Kepler's three laws from Newton's laws and universal gravitation.]
- Laplace, P.S. (1799). *Mécanique Céleste*, Vol. 1. Paris. [The conserved vector later named for Runge and Lenz — Laplace discovered it first in 1799.]
- Pauli, W. (1926). "Über das Wasserstoffspektrum vom Standpunkt der neuen Quantenmechanik." *Zeitschrift für Physik*, 36, 336–363. [The quantum-mechanical LRL vector; algebraic derivation of the hydrogen spectrum.]
- Ghez, A.M. et al. (2008). "Measuring the mass of Sgr A* with stellar orbits around the Galactic center." *Astrophysical Journal*, 689, 1044–1062. [Orbital measurements of stars within 0.3 arcsec of the galactic center give $M_{\rm Sgr A^*} = (4.1 \pm 0.6) \times 10^6 M_\odot$. One of the most direct measurements of a supermassive black hole mass.]
- Goldstein, H. (1975). "More on the prehistory of the Laplace or Runge-Lenz vector." *American Journal of Physics*, 44, 1123–1124. [Historical note: the vector was conserved by Laplace (1799), Hamilton (1845), Runge (1919), and Lenz (1924) — in that order. The "LRL" attribution is standard but historically unjust to Laplace.]
