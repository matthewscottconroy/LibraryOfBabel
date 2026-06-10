# Chapter 36: Exercises

---

**36.1.** *Stress-energy components for a perfect fluid.*

For a perfect fluid with $\varepsilon = \rho_0 c^2$ (dust), $p = 0$, and 4-velocity $u^\mu = \gamma(c, v, 0, 0)$ in a flat background:

(a) Write out all 10 independent components of $T^{\mu\nu} = \rho_0 u^\mu u^\nu$.

(b) Verify that $\partial_\mu T^{\mu 0} = 0$ and $\partial_\mu T^{\mu 1} = 0$ give the non-relativistic continuity equation $\partial\rho/\partial t + \nabla\cdot(\rho\mathbf{v}) = 0$ and the Euler equation $\rho(\partial\mathbf{v}/\partial t + \mathbf{v}\cdot\nabla\mathbf{v}) = 0$, in the limit $v \ll c$.

(c) For a non-relativistic gas in thermal equilibrium (Maxwell-Boltzmann distribution), the stress tensor is $T^{ij} = p\delta^{ij}$ where $p = n k_B T$ (ideal gas law). Show that the perfect fluid formula $T^{ij} = (\varepsilon + p)u^i u^j + p\delta^{ij}$ reduces to this in the non-relativistic limit.

(d) For radiation ($p = \varepsilon/3$), compute $T^\mu_{\ \mu}$ (trace). Show it vanishes. Why must it, for photons?

---

**36.2.** *The TOV equation.*

A constant-density star ($\varepsilon = \text{const}$) provides an exact solution of the TOV equation.

(a) Integrate the mass equation $dm/dr = 4\pi r^2\varepsilon/c^2$ to get $m(r) = 4\pi\varepsilon r^3/(3c^2)$.

(b) Substitute into the TOV equation and integrate to find the pressure profile $p(r)$. The boundary condition is $p(R) = 0$ at the star's surface. Show the solution is:
$$\frac{p(r)}{\varepsilon} = \frac{\sqrt{1-2Gm(r)/(rc^2)} - \sqrt{1-r_s^3/R^3\cdot(2GM)/(Rc^2)}}{\sqrt{1 - r_s^3/R^3\cdot(2GM)/(Rc^2)} - 3\sqrt{1-2Gm(r)/(rc^2)}}$$
(This is the Schwarzschild interior solution.)

(c) Show that the central pressure diverges when $\sqrt{1 - 2GM/(Rc^2)} = 1/3$, i.e., when $2GM/(Rc^2) = 8/9$. This is the Buchdahl limit: a constant-density star cannot be more compact than $2GM/(Rc^2) = 8/9 \approx 0.889$ without the central pressure going infinite (becoming unphysical).

(d) For a neutron star with $M = 1.4 M_\odot$ and $R = 11.9$ km, compute $2GM/(Rc^2)$. How close is this to the Buchdahl limit? To the Schwarzschild radius ($2GM/(Rc^2) = 1$)?

---

**36.3.** *Energy conditions and their physical implications.*

For each of the following matter distributions, determine which energy conditions (WEC, NEC, SEC, DEC) are satisfied:

(a) A gas of photons at temperature $T > 0$: $\varepsilon = a_{\rm rad}T^4$, $p = \varepsilon/3$.

(b) A cosmological constant $\Lambda > 0$: $\varepsilon_\Lambda = \Lambda c^4/(8\pi G)$, $p_\Lambda = -\varepsilon_\Lambda$.

(c) A scalar field $\phi$ in slow-roll inflation: $\varepsilon = \frac{1}{2}\dot\phi^2 + V(\phi)$, $p = \frac{1}{2}\dot\phi^2 - V(\phi)$. For slow roll, $\dot\phi^2 \ll V(\phi)$, so $p \approx -V \approx -\varepsilon$. Which energy conditions are satisfied or violated?

(d) The quantum vacuum near a black hole horizon (Hawking radiation setup): the expected value of $T^{\mu\nu}$ near the horizon violates the WEC. Why does this not violate energy conservation?

---

**36.4.** *Stellar structure and maximum mass.*

(a) For an incompressible star (constant density $\rho$), use the TOV equation to find the maximum mass that can exist without the central pressure diverging. Express in solar masses assuming nuclear saturation density $\rho = \rho_0 = 2.7\times 10^{17}$ kg/m$^3$.

(b) The Chandrasekhar mass for a white dwarf supported by electron degeneracy pressure is $M_{\rm Ch} = 1.44 M_\odot$. A neutron star supported by neutron degeneracy pressure has (roughly) the same formula but with the proton mass $m_p$ replacing the electron mass $m_e$. Estimate the analog of the Chandrasekhar mass for neutrons and compare to the actual TOV maximum mass.

(c) The observation of a $2.1 M_\odot$ neutron star (PSR J0952-0607) rules out "soft" equations of state. A soft EOS has low pressure at high density (matter is compressible). Explain why a soft EOS predicts a lower maximum mass, and what this implies for the nuclear force at supranuclear densities.

---

## Thought Experiments

**T36.1.** *Does pressure repel?*

The Newtonian intuition is that pressure supports a star against collapse — higher pressure, less collapse. In GR, pressure also contributes to the gravitational source. Consider a thought experiment:

Imagine inflating a spherical ball with gas. In Newtonian gravity: more pressure = more support, no change in gravitational field from outside. In GR: more pressure inside increases $T^{ii}$ and hence $T = T^\mu_{\ \mu} = -\varepsilon + 3p$, which increases the scalar curvature $R = -8\pi G T$ and thus the total gravitational effect. Does this mean that pressurizing the ball *increases* its gravitational pull? What effect would this have on a neutron star being compressed?

**T36.2.** *The Tolman-Ehrenfest theorem and temperature gradients.*

In a static gravitational field, a body in thermal equilibrium has a temperature gradient: $T\sqrt{-g_{00}} = \text{const}$, meaning $T(r) = T_\infty/\sqrt{-g_{00}(r)} = T_\infty/\sqrt{1-2GM/(rc^2)}$. The temperature is higher at lower gravitational potential.

This seems counterintuitive — isn't thermal equilibrium supposed to mean uniform temperature? The resolution: in GR, a photon climbing out of a gravitational well is redshifted. For the photon gas at two heights to be in thermal equilibrium, the photon spectrum at the top (after redshifting) must match the Planck distribution at the top. This requires the temperature at the top to be lower by exactly the gravitational redshift factor. Show this explicitly.

