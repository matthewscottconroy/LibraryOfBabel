# 1.2.4 The Ampère-Maxwell Law

## The Equation

$$\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}} + \mu_0\varepsilon_0 \frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A}$$

The left side is the line integral of the magnetic field around a closed loop $C$. On the right, $I_{\text{enc}}$ is the total real current (charge per unit time) passing through any surface bounded by $C$, and the second term is $\mu_0 \varepsilon_0$ times the rate of change of electric flux through the same surface.

## Dissecting the Two Terms

**The first term: $\mu_0 I_{\text{enc}}$.**
This is Ampère's original law: steady electric currents create magnetic fields that circulate around the current direction according to the right-hand rule. A long straight wire carrying current $I$ creates a magnetic field at distance $r$ of magnitude $B = \mu_0 I / (2\pi r)$.

**The second term: $\mu_0\varepsilon_0 \frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A}$.**
This is Maxwell's displacement current. A changing electric field creates a circulating magnetic field, just as a real current does. Note the combination $\mu_0 \varepsilon_0$: we will see in Section 1.4 that $1/\sqrt{\mu_0 \varepsilon_0} = c$, the speed of light. This combination is not a coincidence — it is the same combination that appears in the wave equation.

## The Symmetry Between the Two Coupling Laws

The Ampère-Maxwell law, combined with Faraday's law, creates a perfect symmetry:

| Law | Source | Effect |
|-----|--------|--------|
| Faraday | $-\partial \mathbf{B}/\partial t$ | Creates circulating $\mathbf{E}$ |
| Ampère-Maxwell (displacement term) | $+\varepsilon_0 \partial \mathbf{E}/\partial t$ | Creates circulating $\mathbf{B}$ |

The signs differ because of the way the fields relate to each other in a propagating wave: if $\mathbf{E}$ points in the $x$-direction and $\mathbf{B}$ in the $y$-direction, the wave propagates in the $z$-direction — and the relative signs of the curl equations ensure this geometry is consistent.

## The Displacement Current in a Capacitor: Quantitative

Consider a parallel-plate capacitor with plate area $A$, plate separation $d$, and a charging current $I$.

The charge on the plates: $Q(t) = \int_0^t I(t') dt'$.
The electric field between plates: $E = Q/(\varepsilon_0 A) = \sigma/\varepsilon_0$ (Gauss's law).
The rate of change: $\partial E/\partial t = I/(\varepsilon_0 A)$.
The displacement current through the capacitor: $I_D = \varepsilon_0 A \cdot \partial E/\partial t = I$.

The displacement current between the plates exactly equals the conduction current in the wire. The magnetic field outside the capacitor (from the wire current) seamlessly continues into the field between the plates (from the displacement current). Ampère's law is satisfied for any choice of surface.

This quantitative equivalence is important: it shows that the displacement current is not a vague addition but a precisely defined quantity that integrates seamlessly with the existing structure of the theory.

## Units Check

The term $\mu_0 \varepsilon_0 \frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A}$ has units:

$$[\mu_0 \varepsilon_0] \cdot \left[\frac{d}{dt}\right] \cdot [\mathbf{E}] \cdot [A] = \frac{H}{m} \cdot \frac{F}{m} \cdot \frac{1}{s} \cdot \frac{V}{m} \cdot m^2$$

Since $H = kg\cdot m^2/(A^2\cdot s^2)$, $F = A^2\cdot s^4/(kg\cdot m^2)$, and $V = kg\cdot m^2/(A\cdot s^3)$:

$$\frac{H \cdot F}{m^2 \cdot s} \cdot \frac{V \cdot m^2}{m} = \frac{kg\cdot m^2}{A^2\cdot s^2} \cdot \frac{A^2\cdot s^4}{kg\cdot m^2} \cdot \frac{1}{s} \cdot \frac{kg\cdot m}{A\cdot s^3} = A = \text{amperes}$$

(We omit the detailed algebra but confirm the result: the displacement current term has units of amperes, the same as the current term. The equation is dimensionally consistent.)

## Maxwell's Equations: The Complete Set

We now have all four equations:

| Equation | Physical content |
|----------|-----------------|
| $\oint_S \mathbf{E} \cdot d\mathbf{A} = Q_{\text{enc}}/\varepsilon_0$ | Electric charges create diverging $\mathbf{E}$ |
| $\oint_S \mathbf{B} \cdot d\mathbf{A} = 0$ | No magnetic monopoles; $\mathbf{B}$ lines are closed |
| $\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -d\Phi_B/dt$ | Changing $\mathbf{B}$ creates circulating $\mathbf{E}$ |
| $\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I + \mu_0\varepsilon_0 d\Phi_E/dt$ | Currents and changing $\mathbf{E}$ create circulating $\mathbf{B}$ |

These four equations, together with the Lorentz force law $\mathbf{F} = q(\mathbf{E} + \mathbf{v} \times \mathbf{B})$, constitute a complete description of all classical electromagnetic phenomena.
