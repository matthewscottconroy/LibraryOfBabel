# Chapter 35: Exercises

---

**35.1.** *Deriving the field equations from the action.*

The Einstein-Hilbert action is $S = \frac{1}{16\pi G}\int R\sqrt{-g}\,d^4x$ (in natural units $c = 1$).

(a) The variation of the Ricci scalar under $g^{\mu\nu}\to g^{\mu\nu} + \delta g^{\mu\nu}$ is:
$$\delta R = R_{\mu\nu}\delta g^{\mu\nu} + g^{\mu\nu}\delta R_{\mu\nu}$$
The Palatini identity states $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu v^\mu$ for some vector $v^\mu$. Show that the Palatini term is a boundary term and does not contribute to the bulk equations of motion (assuming $\delta g^{\mu\nu}$ vanishes on the boundary).

(b) The variation of $\sqrt{-g}$ is $\delta\sqrt{-g} = -\frac{1}{2}\sqrt{-g}\,g_{\mu\nu}\delta g^{\mu\nu}$. Derive this formula from $g = \det(g_{\mu\nu})$ using the identity $\delta\ln\det A = \text{tr}(A^{-1}\delta A)$.

(c) Combining (a) and (b), show that $\delta S/\delta g^{\mu\nu} = \frac{1}{16\pi G}(R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R)\sqrt{-g}$.

(d) The matter action contributes $\delta S_{\rm matter}/\delta g^{\mu\nu} = -\frac{1}{2}T_{\mu\nu}\sqrt{-g}$. Combining, derive the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$.

---

**35.2.** *Counting constraints and degrees of freedom.*

(a) The metric $g_{\mu\nu}$ has 10 independent components (symmetric $4\times 4$ matrix). Show that 4 of the 10 Einstein equations are the contracted Bianchi identity $\nabla_\mu G^{\mu\nu} = 0$, which holds for any metric (not just solutions). These 4 equations are therefore not independent constraints on the metric.

(b) Diffeomorphism invariance gives freedom to choose 4 coordinate functions $x^\mu$. This removes 4 more degrees of freedom from the metric. How many physical degrees of freedom does the gravitational field have?

(c) Electromagnetism has $A_\mu$ (4 components) minus 1 gauge degree of freedom (phase) = 3 components. The constraint $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$ removes one more, leaving 2 physical degrees of freedom (2 photon polarizations). Compare to the GR counting from (b). What are the 2 GR degrees of freedom?

(d) In the TT (transverse-traceless) gauge for gravitational waves, the metric perturbation $h_{\mu\nu}^{\rm TT}$ has exactly 2 independent components ($h_+$ and $h_\times$). Show that $h^{\rm TT}_{0\mu} = 0$ (no time components), $h^{ii}_{\rm TT} = 0$ (traceless), and $\partial^j h^{\rm TT}_{ij} = 0$ (transverse). How many of the original 10 components does this eliminate?

---

**35.3.** *The Newtonian limit.*

(a) For a static, non-relativistic ($v \ll c$) source with $T^{00} \approx \rho c^2$ and all other components negligible, write the $00$-component of the Einstein equations in Lorenz gauge and show it gives $\nabla^2 h_{00} = 8\pi G\rho/c^4$ (in SI units).

(b) Identifying $g_{00} = -(1 + 2\Phi/c^2)$ where $\Phi$ is the Newtonian gravitational potential, show that this reduces to Poisson's equation $\nabla^2\Phi = 4\pi G\rho$.

(c) Now consider the $ij$-components of the linearized Einstein equations in the same limit. Show that $\nabla^2 h_{ij} = 8\pi G T_{ij}/c^4$. For a non-relativistic source with negligible stress ($T_{ij} \approx 0$), show that $h_{ij} \approx 0$ except for $h_{ij} = h_{00}\delta_{ij}$ — the isotropic Newtonian metric.

(d) The spatial part of the metric is $g_{ij} = (1 - 2\Phi/c^2)\delta_{ij}$ in the PPN formalism. Why does this differ from $\delta_{ij}$ (flat)? What physical effect does the spatial metric curvature produce? (Hint: compute the deflection of light from the spatial part alone and compare to the total deflection $4GM/(bc^2)$.)

---

**35.4.** *Binary pulsar and the quadrupole formula.*

The Hulse-Taylor binary pulsar PSR B1913+16 has two neutron stars orbiting each other with:
- Orbital period $P_b = 7.75$ hours
- Orbital eccentricity $e = 0.617$
- Pulsar mass $m_1 = 1.441 M_\odot$
- Companion mass $m_2 = 1.387 M_\odot$

(a) Compute the orbital semi-major axis $a$ from Kepler's third law (using the total mass $M = m_1 + m_2$).

(b) The gravitational wave luminosity (Peters formula, from linearized GR):
$$P_{\rm GW} = \frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}f(e)$$
where $f(e) = (1-e^2)^{-7/2}(1 + \frac{73}{24}e^2 + \frac{37}{96}e^4)$. Compute $f(0.617)$ and $P_{\rm GW}$ for PSR B1913+16.

(c) The orbital energy is $E = -Gm_1 m_2/(2a)$. Show that $dE/dt = -P_{\rm GW}$ leads to an orbital decay rate $\dot{P}_b = -2.40\times 10^{-12}$ s/s (the observed value is $-2.423\pm 0.001\times 10^{-12}$ s/s — agreement to $0.1\%$).

(d) The Nobel Prize in Physics 1993 was awarded to Hulse and Taylor for this discovery. Why does the orbital decay provide indirect evidence for gravitational waves (rather than direct observation)? What was finally directly observed in 2015?

---

## Thought Experiments

**T35.1.** *Why can't gravity be a spin-1 gauge theory?*

Electromagnetism is the gauge theory of a massless spin-1 field (the photon). By analogy, could gravity be the gauge theory of a massless spin-1 field?

The answer is no: a spin-1 gauge theory has like charges repelling (like-sign charges repel in EM; like-mass objects would repel gravitationally, but we observe that gravity is universally attractive). More formally: a Lorentz-covariant, gauge-invariant theory of a massless spin-1 field coupled to its own source leads to inconsistency (the spin-1 field energy is negative for like-sign "charges"). Gravity must be described by a spin-2 field (the graviton), which gives attraction for all masses and couples to all forms of energy equally. Construct the argument for why spin-1 gravity leads to repulsion, and why spin-2 gravity naturally couples to $T^{\mu\nu}$ rather than a current $J^\mu$.

**T35.2.** *The cosmological constant as quantum vacuum energy.*

In quantum field theory, the vacuum state has nonzero energy density $\rho_{\rm vac}$. In GR, any energy density curves spacetime — so vacuum energy should contribute to $\Lambda$ as an effective $T^{\mu\nu}_{\rm vac} = -\rho_{\rm vac}c^2 g^{\mu\nu}$ (equation of state $w = -1$).

The problem: QFT estimates $\rho_{\rm vac} \sim (E_{\rm Planck})^4/(\hbar c)^3 \sim 10^{113}$ J/m$^3$. The observed value is $\rho_\Lambda \sim 6\times 10^{-10}$ J/m$^3$. The discrepancy is $\sim 10^{123}$.

Discuss: What are the proposed resolutions? (Supersymmetry, anthropic selection, sequestering, dynamical relaxation mechanisms.) Why is setting $\Lambda = 0$ by hand unstable (radiative corrections restore it)? Why is $\Lambda$ small but nonzero arguably harder to explain than $\Lambda = 0$?

