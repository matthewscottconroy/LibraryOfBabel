# Chapter 20: Exercises

---

## Section 20.1 — Four-Vectors and Relativistic Dynamics

**20.1.1.** *Relativistic kinematics and the mass-shell condition.*

(a) A particle of rest mass $m$ moves with velocity $v = 0.6c$ in the $x$-direction. Write down its 4-momentum $p^\mu = (E/c, p_x, 0, 0)$. Compute $E$, $p_x$, and verify the mass-shell condition $p_\mu p^\mu = -m^2c^2$.

(b) Two photons travel in opposite directions with energies $E_1 = 1$ MeV and $E_2 = 1$ MeV (in the lab frame). Compute the invariant mass $\sqrt{-p^2_{\rm total}/c^2}$ of the two-photon system. Can these photons annihilate to produce an electron-positron pair? (The electron mass is $m_e c^2 = 0.511$ MeV.)

(c) In a collider, two protons each have energy $E = 7$ TeV (the LHC design energy). Their momenta are equal and opposite. What is the invariant mass of the two-proton system? How does this compare to a fixed-target experiment where one proton is at rest and the other has the same total energy (14 TeV)?

(d) A photon of energy $E_\gamma$ scatters off a stationary electron. Show that the scattered photon energy $E_\gamma'$ as a function of scattering angle $\theta$ is:
$$E_\gamma' = \frac{E_\gamma}{1 + (E_\gamma/m_e c^2)(1-\cos\theta)}$$
This is the Compton formula. Derive it using 4-momentum conservation.

---

**20.1.2.** *Mass-energy equivalence.*

(a) A uranium-235 nucleus captures a neutron and fissions into barium-141, krypton-92, and 3 neutrons. The masses are:
- $^{235}$U: $235.043924$ u
- $^1$n: $1.008665$ u
- $^{141}$Ba: $140.914411$ u
- $^{92}$Kr: $91.926156$ u

Compute the mass defect $\Delta m$ in atomic mass units ($1$ u = $931.5$ MeV/$c^2$). What is the energy released in MeV per fission event? Compare this to the energy released by burning one carbon atom ($\sim 1$ eV per bond).

(b) The proton mass is $m_p c^2 = 938.3$ MeV. Quarks contribute only about $\sim 9$ MeV to this. Where does the rest of the proton mass come from? (Answer: the kinetic energy and potential energy of the strongly-interacting quarks and gluons — the binding energy of QCD is negative and large, but the kinetic energy contribution to the rest mass is huge.)

(c) The Sun radiates $L_\odot = 3.8\times 10^{26}$ W. How much mass does it lose per second? At this rate, how long would it take to lose 0.1% of its mass? Compare to the age of the solar system ($4.6$ Gyr).

(d) A photon is emitted vertically downward in a gravitational field. Its energy is $E = hf$. If $E = mc^2$, the photon has an "effective mass" $m = hf/c^2$. Using the gravitational redshift formula derived from equivalence principle arguments, show that a photon falling through height $\Delta h$ gains energy $\Delta E = (hf/c^2)g\Delta h$. This is consistent with $E = mc^2$.

---

**20.1.3.** *Relativistic rocket.*

A rocket has initial rest mass $M_0$ and ejects propellant at exhaust speed $u$ relative to the rocket.

(a) Show that the Tsiolkovsky rocket equation becomes, relativistically:
$$\frac{v}{c} = \frac{(M_0/M_f)^{2u/c} - 1}{(M_0/M_f)^{2u/c} + 1}$$
where $M_f$ is the final mass and $u$ is the exhaust speed. (For $u \ll c$ and small $\Delta v$, recover the non-relativistic result $\Delta v = u\ln(M_0/M_f)$.)

(b) For a photon rocket ($u = c$), what is the final mass ratio needed to reach $v = 0.9c$? What about $v = 0.99c$?

(c) To reach Alpha Centauri (4.3 light-years away) in 1 year of ship time requires $\gamma \approx$ large enough to give the appropriate time dilation. What $\gamma$ is needed? For a photon rocket, what is the required mass ratio?

(d) The "brachistochrone" trajectory — constant 1g acceleration to the midpoint, then deceleration — reaches any point in the galaxy in a few decades of ship time due to time dilation. Compute the ship time and Earth time for a 1g brachistochrone trip to (i) Alpha Centauri, (ii) the galactic center (26,000 ly), (iii) the Andromeda galaxy (2.5 million ly).

---

## Section 20.2 — The Stress-Energy Tensor

**20.2.1.** *Properties of $T^{\mu\nu}$.*

(a) For a perfect fluid at rest ($u^\mu = (c, 0, 0, 0)$), write out all components of $T^{\mu\nu} = (\epsilon + p)u^\mu u^\nu/c^2 + p\eta^{\mu\nu}$. Identify which component is energy density, which is momentum density, and which is stress (pressure).

(b) Show that conservation $\partial_\mu T^{\mu\nu} = 0$ gives two separate equations: the energy equation and the momentum equation. Show that in the non-relativistic limit ($p \ll \rho c^2$, $v \ll c$), these reduce to the continuity equation ($\partial\rho/\partial t + \nabla\cdot(\rho\mathbf{v}) = 0$) and the Euler equation ($\rho D\mathbf{v}/Dt = -\nabla p$).

(c) The electromagnetic stress-energy tensor is $T^{\mu\nu}_{\rm EM} = F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}\eta^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}$ (in Gaussian units; adjust by $1/\mu_0$ in SI). Show that $T^{\mu\nu}_{\rm EM}$ is traceless: $T^\mu_{\ \mu} = 0$. Physically, why is EM traceless? (Hint: massless particles, or: $p = \rho c^2/3$ for radiation.)

(d) The total stress-energy includes both matter and radiation. In the early universe (radiation domination), $T^\mu_{\ \mu} = 0$ implies $R = g^{\mu\nu}R_{\mu\nu} = 0$ (the Ricci scalar vanishes). What does this imply for the cosmological Friedmann equations?

---

**20.2.2.** *Energy conditions.*

The "energy conditions" are physically-motivated inequalities on $T^{\mu\nu}$:
- **Weak Energy Condition (WEC):** $T_{\mu\nu}t^\mu t^\nu \geq 0$ for all timelike $t^\mu$. Measured energy density is non-negative.
- **Strong Energy Condition (SEC):** $(T_{\mu\nu} - \frac{1}{2}g_{\mu\nu}T)t^\mu t^\nu \geq 0$. Equivalent to $R_{\mu\nu}t^\mu t^\nu \geq 0$ (by Einstein equations). Gravity is attractive.
- **Dominant Energy Condition (DEC):** WEC plus energy flux is causal.

(a) For a perfect fluid, the WEC requires $\epsilon \geq 0$ and $\epsilon + p \geq 0$. The SEC requires $\epsilon + 3p \geq 0$ and $\epsilon + p \geq 0$. Show these follow from the definitions.

(b) For a cosmological constant $\Lambda > 0$, the effective energy density is $\epsilon_\Lambda = \Lambda c^2/(8\pi G) > 0$ (satisfies WEC) and pressure is $p_\Lambda = -\epsilon_\Lambda < 0$. Does $\Lambda > 0$ satisfy the SEC? What does this imply for gravity?

(c) Hawking radiation has negative energy density in the near-horizon region (related to the Unruh effect). This means the WEC is violated quantum-mechanically near black hole horizons. What physical consequence does this have?

(d) Exotic matter that violates the WEC ($\epsilon < 0$) could, in principle, stabilize a traversable wormhole (Morris-Thorne 1988). The amount of exotic matter required is proportional to the wormhole throat size. Estimate the required exotic matter energy for a 1-meter-throat wormhole.

---

## Thought Experiments

**T20.1.** *$E = mc^2$: why does mass have energy?*

Before relativity, mass and energy were entirely separate concepts. A brick sitting on a shelf has potential energy (due to its height) and thermal energy (due to molecular motion), but its *mass* seemed to be something different — an intrinsic property, not a form of energy.

Special relativity says that the rest energy $E_0 = mc^2$ is the energy of a particle at rest, distinct from kinetic or potential energy. This is not a definition but a physical consequence of the principle of relativity: if the energy of a moving object is $E = \gamma mc^2$, then as $v\to 0$, $E\to mc^2$. The rest energy is real — nuclear fission releases it, pair production creates it from photons, and the Sun converts it to light.

Think carefully about what it means to say that mass "is" energy. When we burn gasoline, where does the mass go? When we cool an object, does its mass decrease? (Answer: yes, by $\Delta m = \Delta E/c^2$ — but this is too small to measure for chemistry.) When a proton and antiproton annihilate into two photons, where did the mass "go"? Is the photon massless? Construct a consistent picture of what mass means in the light of $E = mc^2$.

---

**T20.2.** *The stress-energy tensor as the source of gravity.*

In Newtonian gravity, the source is mass density $\rho$. In GR, the source is $T^{\mu\nu}$ — the full stress-energy tensor. This means not just mass-energy, but also pressure and momentum flux, generate gravity.

This has a surprising consequence: pressure gravitates. A very massive star in hydrostatic equilibrium is supported by pressure, but that very pressure (via the Tolman-Oppenheimer-Volkoff equation) adds to the gravitational field, requiring even more pressure to support the star — leading to a maximum mass for neutron stars ($\sim 2-3 M_\odot$ depending on the equation of state). Show that in the weak-field limit, the source of the gravitational potential is $\rho + 3p/c^2$ (from the trace of the Einstein equations) rather than just $\rho$. For a normal star with $p \ll \rho c^2$, this makes no difference. For relativistic matter (neutron stars, early universe), it matters enormously.

---

## Laboratory Exercise: Relativistic Energy in Particle Detectors

**L20.1.** *Analyzing simulated particle collisions from CERN Open Data.*

CERN makes real LHC collision data available to the public through the CERN Open Data portal (opendata.cern.ch). Using this data, students can reconstruct particles from their decay products.

**Setup:** The CMS detector at LHC records the 4-momenta of detected particles. For each detected particle: $p^\mu = (E/c, p_x, p_y, p_z)$.

**Task 1 (Z boson):** Find electron-positron pairs $(e^+, e^-)$ in the CMS data. For each pair, compute the invariant mass $M = \sqrt{-(p_{e^+} + p_{e^-})^2}/c$. The Z boson has mass $91.2$ GeV/$c^2$. Plot the invariant mass distribution and identify the Z peak.

**Task 2 (Higgs boson):** Find four-lepton events ($e^+e^-e^+e^-$ or $\mu^+\mu^-\mu^+\mu^-$). Compute the four-lepton invariant mass. The Higgs boson (if the four leptons came from $H\to ZZ^*\to 4\ell$) has mass $125.1$ GeV/$c^2$.

**Task 3:** For each detected muon in the dimuon dataset, plot the distribution of $|\mathbf{p}|c/m_\mu c^2 = p c / (0.106$ GeV). This gives the $\gamma\beta$ distribution for cosmic ray muons entering the detector. Estimate the typical $\gamma$ factor.

**Note:** The CERN CMS Open Data portal provides Jupyter notebooks and data in ROOT format. Python-based analysis using the `coffea` or `uproot` libraries is recommended.

