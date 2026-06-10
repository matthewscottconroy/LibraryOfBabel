# Section 36.2: Conservation Laws and the TOV Equation

---

## Energy-Momentum Conservation in Curved Spacetime

In special relativity, energy-momentum conservation is $\partial_\mu T^{\mu\nu} = 0$. This is a genuine conservation law — it implies a conserved 4-vector $P^\nu = \int T^{0\nu}d^3x$.

In general relativity, the curved-space generalization is $\nabla_\mu T^{\mu\nu} = 0$. But this is *not* a conservation law in the usual sense. The covariant derivative $\nabla_\mu T^{\mu\nu} = \partial_\mu T^{\mu\nu} + \Gamma^\mu_{\mu\lambda}T^{\lambda\nu} + \Gamma^\nu_{\mu\lambda}T^{\mu\lambda}$ includes Christoffel terms that represent the exchange of energy and momentum between matter and the gravitational field. In curved spacetime, the gravitational field can do work on matter and vice versa.

There is no locally conserved energy-momentum 4-vector in GR (except in special circumstances — asymptotically flat spacetimes, or spacetimes with Killing vectors). This is not a deficiency of GR; it is a correct physical statement. In a non-static cosmological spacetime (for example), photons redshift as the universe expands, losing energy. That energy is not "gained" by gravity in any well-defined local sense.

What does hold locally is $\nabla_\mu T^{\mu\nu} = 0$ — the covariant equation. This gives the equations of motion for matter: the relativistic Euler equation (for perfect fluids) or the geodesic equation (for dust).

**Global conservation** can be defined in spacetimes with symmetries. If $\xi^\mu$ is a Killing vector ($\nabla_{(\mu}\xi_{\nu)} = 0$), then the current $J^\nu = T^{\mu\nu}\xi_\mu$ satisfies $\nabla_\nu J^\nu = 0$. By the covariant divergence theorem, $\int_\Sigma J^\mu n_\mu\sqrt{\gamma}\,d^3x$ is conserved between spacelike hypersurfaces $\Sigma$. For Schwarzschild (with time-translation Killing vector $\partial_t$), this gives a conserved total energy. For FLRW (no timelike Killing vector), no conserved total energy exists.

---

## Hydrostatic Equilibrium in GR: The TOV Equation

The most important application of $\nabla_\mu T^{\mu\nu} = 0$ in stellar physics is the relativistic equation of hydrostatic equilibrium.

A static, spherically symmetric star has the metric (Schwarzschild-like interior):
$$ds^2 = -e^{2\alpha(r)}c^2dt^2 + e^{2\beta(r)}dr^2 + r^2d\Omega^2$$
where $\alpha(r)$ and $\beta(r)$ are to be determined by the Einstein equations.

The Einstein equations plus $\nabla_\mu T^{\mu\nu} = 0$ for a perfect fluid give the **Tolman-Oppenheimer-Volkoff (TOV) equation:**
$$\frac{dp}{dr} = -\frac{(\varepsilon + p)(m(r)c^2 + 4\pi r^3 p)}{r^2 c^2\left(1 - \frac{2Gm(r)}{rc^2}\right)} \cdot \frac{G}{c^2}$$

where $m(r) = \frac{4\pi}{c^2}\int_0^r\varepsilon(r')r'^2 dr'$ is the gravitational mass enclosed within radius $r$, and the mass continuity equation is:
$$\frac{dm}{dr} = 4\pi r^2\frac{\varepsilon}{c^2}$$

**Newtonian limit:** Set $p \ll \varepsilon$, $4\pi r^3 p \ll m(r)c^2$, $2Gm(r)/(rc^2) \ll 1$. The TOV equation reduces to:
$$\frac{dp}{dr} = -\frac{G m(r)\rho}{r^2}$$
which is the Newtonian hydrostatic equilibrium equation $dP/dr = -g\rho$ (with $g = Gm(r)/r^2$).

**GR corrections:** Three terms make the TOV equation more restrictive than the Newtonian equation:
1. $\varepsilon + p$ instead of $\varepsilon$: pressure adds to inertia. More pressure means stronger inertia, which means more support needed.
2. $m(r)c^2 + 4\pi r^3 p$ instead of $m(r)c^2$: pressure contributes to the gravitational source in the interior (pressure gravitates).
3. $(1 - 2Gm(r)/(rc^2))^{-1}$: metric factor that increases the effective gravitational attraction.

All three GR corrections increase the required pressure gradient to maintain equilibrium. For a neutron star at nuclear density ($\varepsilon \sim 10^{35}$ J/m$^3$), these corrections are of order 10–30%.

---

## The Oppenheimer-Volkoff Mass Limit

For a given equation of state $p = p(\varepsilon)$, integrating the TOV equation from the center ($p = p_c$, $m = 0$) outward to the surface ($p = 0$) gives the stellar radius $R$ and gravitational mass $M = m(R)$.

By varying the central pressure $p_c$, one obtains a **mass-radius relation** $M(R)$ for the given equation of state. The maximum of $M(p_c)$ along this relation is the **maximum mass** for that equation of state.

For **polytropic models** $p = K\varepsilon^\Gamma$:
- For $\Gamma < 4/3$: no stable solutions exist above a critical central pressure (the star collapses). This is why white dwarfs have a maximum mass (Chandrasekhar limit $\sim 1.44\,M_\odot$ for electron degeneracy, $\Gamma \to 4/3$ at high density).
- For $4/3 < \Gamma \leq 2$: stable neutron star configurations exist.

The **Buchdahl limit** (Buchdahl 1959) gives a universal bound: for any star in GR satisfying $d\varepsilon/dr \leq 0$ (density decreasing outward) and WEC, the compactness must satisfy:
$$\frac{2GM}{Rc^2} \leq \frac{8}{9}$$
This is the most compact a star can be without becoming a black hole. A Schwarzschild black hole has $2GM/(Rc^2) = 1$ (the Schwarzschild radius).

**Numerical results** depend on the nuclear equation of state (EOS), which is uncertain above nuclear saturation density $\rho_0 = 2.7\times 10^{17}$ kg/m$^3$. Observations:
- The maximum observed neutron star mass is $\sim 2.1\,M_\odot$ (PSR J0952-0607, 2022). This rules out "soft" equations of state that predict lower maximum masses.
- GW170817 (neutron star merger) constrains the radius of a 1.4 $M_\odot$ neutron star to $11$–$13$ km (from the tidal deformability measurement).

---

## The Jeans Instability in GR

The Jeans instability — the tendency of a self-gravitating gas cloud to collapse under its own gravity — has a GR analog that is relevant to cosmology.

In Newtonian gravity, a perturbation of size $\lambda > \lambda_J = c_s\sqrt{\pi/G\rho}$ (where $c_s$ is the sound speed) grows exponentially. For $\lambda < \lambda_J$, pressure suppresses collapse (sound waves propagate faster than the collapse time).

In GR, the Jeans instability for a perfect fluid in an expanding FRW background (the Sachs-Wolfe and Lifshitz analysis) gives the equations governing the growth of density perturbations — the origin of large-scale structure in the universe. This is covered in Unit XII (Cosmology).

---

## Conservation of Baryon Number

In addition to energy-momentum conservation, GR inherits from SR the conservation of baryon number (in the absence of baryogenesis processes). The baryon number current is $j^\mu = n_B u^\mu$ where $n_B$ is the baryon number density in the rest frame. The conservation equation is:
$$\nabla_\mu j^\mu = \frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}n_B u^\mu) = 0$$

This gives the **continuity equation** for baryon number. Combined with the TOV equation, it determines the structure of a neutron star completely (given an equation of state and the boundary conditions $p(0) = p_c$, $m(0) = 0$, and $p(R) = 0$).

In cosmology, the baryon-to-photon ratio $\eta \approx 6\times 10^{-10}$ is the fundamental observable that connects Big Bang nucleosynthesis predictions to cosmic microwave background observations.

