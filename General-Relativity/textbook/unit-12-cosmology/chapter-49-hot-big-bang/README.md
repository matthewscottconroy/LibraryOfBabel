# Chapter 49: The Hot Big Bang

---

## Chapter Introduction

The Big Bang was not an explosion. It was an everywhere-at-once beginning: the sudden appearance of space, time, energy, and the laws of physics from a state of infinite density — or from some quantum predecessor that our current theories cannot describe. In the first fraction of a second, the universe was a hot, dense plasma of elementary particles and radiation in thermal equilibrium. As it expanded and cooled, a succession of phase transitions froze out different physics: quarks condensed into hadrons, electrons and positrons annihilated into photons, Big Bang nucleosynthesis produced the light elements, and finally the universe became transparent and the CMB was released.

The Hot Big Bang is the best-tested physical theory in cosmology. Its predictions — the CMB temperature, the light element abundances, the evolution of the expansion rate — have been confirmed with extraordinary precision. It is also incomplete: we do not understand what happened before $\sim 10^{-43}$ s (the Planck time), why the universe contains more matter than antimatter (baryogenesis), what dark matter is, or what seeded the primordial density fluctuations.

This chapter traces the thermal history of the universe from the Planck epoch to recombination, deriving the key milestones and physical processes.

---

## Thermal History: A Timeline

The thermal history of the universe is a story of cooling and phase transitions:

| Epoch | Time | Temperature | Energy | Events |
|-------|------|-------------|--------|--------|
| Planck | $< 10^{-43}$ s | $> 10^{32}$ K | $> 10^{19}$ GeV | Quantum gravity; unknown |
| GUT | $\sim 10^{-36}$ s | $\sim 10^{29}$ K | $\sim 10^{16}$ GeV | GUT phase transition? Baryogenesis? |
| Inflation | $\sim 10^{-36}$–$10^{-32}$ s | — | $\sim 10^{16}$ GeV | Exponential expansion; seeds perturbations |
| Electroweak | $10^{-12}$ s | $10^{15}$ K | $\sim 100$ GeV | Higgs mechanism; $W$, $Z$ acquire mass |
| QCD transition | $\sim 10^{-6}$ s | $\sim 2\times 10^{12}$ K | $\sim 200$ MeV | Quark confinement; hadrons form |
| Neutrino decoupling | $\sim 1$ s | $\sim 10^{10}$ K | $\sim 1$ MeV | Neutrino background frozen |
| $e^+e^-$ annihilation | $\sim 10$ s | $\sim 5\times 10^9$ K | $\sim 0.5$ MeV | Positrons annihilate; photons heated |
| BBN | $\sim 10$–$300$ s | $10^9$–$10^8$ K | $\sim 0.1$ MeV | D, $^3$He, $^4$He, $^7$Li synthesized |
| Matter-radiation equality | $\sim 47,000$ yr | $\sim 9000$ K | $\sim 0.8$ eV | Matter begins to dominate |
| Recombination | $\sim 380,000$ yr | $\sim 3000$ K | $\sim 0.3$ eV | Hydrogen forms; CMB decouples |
| Reionization | $\sim 100$–$900$ Myr | $\sim 10$–$100$ K | — | First stars reionize IGM |
| Today | 13.8 Gyr | 2.73 K | $2.3\times 10^{-4}$ eV | |

---

## Thermal Equilibrium in the Early Universe

The early universe was a thermal plasma in local thermal equilibrium (LTE): particle interaction rates $\Gamma \gg H$ (the Hubble rate), so every particle species interacted rapidly and maintained thermal distributions. When $\Gamma \lesssim H$, a species "freezes out" — decouples from the plasma and free-streams.

The number density of a relativistic species in thermal equilibrium at temperature $T$ is:
$$n = \frac{g}{2\pi^2}\int_0^\infty\frac{p^2 dp}{e^{E/T}\pm 1}$$

For bosons (−): $n = \zeta(3)gT^3/\pi^2$ (photons: $g = 2$, $n_\gamma = 2\zeta(3)T^3/\pi^2$)
For fermions (+): $n = (3/4)\zeta(3)gT^3/\pi^2$

The energy density:
$$\rho = \frac{g_*\pi^2}{30}T^4$$

where $g_* = \sum_{\rm bosons} g_i + \frac{7}{8}\sum_{\rm fermions} g_i$ is the effective number of relativistic degrees of freedom. At the QCD transition: $g_* \sim 106.75$ (all Standard Model particles). Today: $g_* = 2$ (photons only, plus neutrinos with a separate temperature).

The entropy density $s = (2\pi^2/45)g_{*S}T^3$ where $g_{*S}$ is the entropy-weighted effective DOF. Entropy conservation (the universe expands adiabatically after baryogenesis): $s a^3 = \text{const}$, so $T \propto g_{*S}^{-1/3}/a$. When $g_{*S}$ decreases (species freeze out), $T$ gets a small boost.

---

## Big Bang Nucleosynthesis in Detail

BBN occurs at $T \sim 0.01$–$1$ MeV ($t \sim 10^{-2}$–$10^3$ s). The relevant nuclear reactions are:

**Neutron-proton interconversion**: At $T \gg 1$ MeV, weak reactions maintain the $n/p$ ratio at:
$$\frac{n}{p} = e^{-\Delta m/T} = e^{-1.293\text{ MeV}/T}$$
where $\Delta m = m_n - m_p = 1.293$ MeV. At $T \sim 1$ MeV, the reaction rate $\Gamma_{n\leftrightarrow p} \sim G_F^2 T^5 \sim H \sim T^2/M_{\rm Pl}$, so freeze-out occurs at $T_{\rm fo} \sim (G_F^2 M_{\rm Pl})^{-1/3} \approx 0.8$ MeV. At this temperature: $n/p \approx e^{-1.293/0.8} \approx 1/5.5$. After freeze-out, the ratio shifts to $\sim 1/7$ due to neutron $\beta$-decay during the BBN epoch.

**Deuterium bottleneck**: Even though temperatures for D formation are reached at $T \sim 2.2$ MeV (binding energy of D), photodissociation prevents D from accumulating until $T \sim 0.07$ MeV ($\eta^{-1}$ photons per baryon means there are enough high-energy photons to destroy D). Once D survives, nuclear reactions proceed rapidly.

**Helium-4**: Nearly all remaining neutrons end up in $^4$He (the most tightly bound light nucleus). With $n/p = 1/7$ at BBN:
$$Y_p = \frac{2(n/p)}{1 + (n/p)} \approx \frac{2/7}{1 + 1/7} = \frac{2}{8} = 0.25$$

The $^4$He mass fraction is $\approx 25\%$ — in excellent agreement with observation.

**Deuterium abundance**: D/H is highly sensitive to the baryon-to-photon ratio $\eta = n_b/n_\gamma \approx 6\times 10^{-10}$. Higher $\eta$ → more efficient burning of D → lower D/H. The observed D/H measurement is the best measurement of $\Omega_b h^2$.

**The CMB and BBN together**: The baryon density $\Omega_b h^2 = 0.0222$ from D/H at $z \sim 3$ (QSO absorption) agrees with $\Omega_b h^2 = 0.02237$ from CMB anisotropies at $z \approx 1100$ — these probe cosmological epochs separated by $300$ Myr, yet agree to 1%. This is one of the most striking confirmations of the standard model.

---

## Recombination and the Last Scattering Surface

At $T \sim 3000$ K ($z \sim 1100$, $t \sim 380,000$ yr), the universe cooled enough for neutral hydrogen to form:
$$\text{p} + e^- \to \text{H} + \gamma$$

The **Saha equation** gives the ionization fraction $x_e = n_e/(n_e + n_H)$:
$$\frac{x_e^2}{1-x_e} = \frac{1}{n_b}\left(\frac{m_e T}{2\pi}\right)^{3/2}e^{-B_1/T}$$

where $B_1 = 13.6$ eV is the hydrogen ionization energy. The transition from $x_e \approx 1$ to $x_e \approx 10^{-4}$ occurs over a redshift range $\Delta z \approx 200$ centered at $z_* \approx 1100$. The **last scattering surface** is not infinitely thin — it has a width $\Delta z \approx 100$, which damps temperature anisotropies on scales smaller than the diffusion length at recombination (Silk damping).

After recombination, photons free-stream and form the CMB. The universe enters the cosmic "dark ages" — neutral hydrogen with no luminous sources — until the first stars form at $z \sim 20$–$30$ and reionize the intergalactic medium.

---

## The Baryon Asymmetry: Why Is the Universe Made of Matter?

The universe contains matter but essentially no antimatter. But in thermal equilibrium at $T \gg 1$ GeV, matter and antimatter are produced in equal amounts. For every baryon, there are $\sim 10^{10}$ photons: $\eta = n_b/n_\gamma \approx 6\times 10^{-10}$.

This tiny asymmetry $\eta$ means that at the QCD transition ($T \sim 200$ MeV), for every $10^{10}$ quark-antiquark pairs, there was one extra quark. All the antiquarks annihilated with quarks, leaving one baryon in $10^{10}$ — the matter-dominated universe we live in.

What generated this asymmetry? The **Sakharov conditions** (1967) state that baryogenesis requires:
1. Baryon number violation (violating B conservation)
2. C and CP violation (distinguishing matter from antimatter)
3. Departure from thermal equilibrium (to prevent the equilibrium wiping out asymmetry)

The Standard Model satisfies all three conditions weakly (sphaleron processes violate B+L; the CKM matrix has CP violation; the electroweak phase transition could be out of equilibrium). But the SM CP violation is too small by $\sim 10$ orders of magnitude. New physics is required — perhaps leptogenesis (CP-violating decays of heavy neutrinos produce a lepton asymmetry, converted to baryons by sphalerons) or Affleck-Dine baryogenesis (involving scalar field dynamics in SUSY).

---

## Exercises

**49.1.** *Neutrino freeze-out and the CMB temperature.*

(a) Neutrinos decouple from the plasma at $T_\nu^{\rm dec} \approx 2$ MeV. After decoupling, the neutrino temperature scales as $T_\nu \propto 1/a$.

(b) After neutrino decoupling but before $e^+e^-$ annihilation, the photon temperature also scales as $T_\gamma \propto 1/a$. When the temperature drops to $T \sim m_e/3 \sim 0.17$ MeV, electrons and positrons annihilate into photons. Using entropy conservation $g_{*S}T_\gamma^3 a^3 = \text{const}$ before and after annihilation, show that the photon temperature is boosted by:
$$\frac{T_\gamma}{T_\nu} = \left(\frac{11}{4}\right)^{1/3} \approx 1.40$$

This ratio is preserved to today. The CMB temperature is $T_\gamma = 2.725$ K; what is the relic neutrino temperature $T_\nu$?

(c) The neutrino contribution to the radiation density is $\rho_\nu = (7/8)\times 3 \times 2\times (T_\nu/T_\gamma)^4\rho_\gamma$. Show that the effective number of relativistic species is $g_* = 2 + (7/8)\times 6\times(4/11)^{4/3} \approx 3.36$.

---

**49.2.** *BBN and the helium abundance.*

(a) The freeze-out temperature for n-p interconversion is $T_{\rm fo} \approx (G_F^2 g_*^{1/2}/M_{\rm Pl})^{-1/3}$. Substituting $G_F = 1.166\times 10^{-5}$ GeV$^{-2}$ and $g_* = 10.75$ at $T \sim 1$ MeV, compute $T_{\rm fo}$ in MeV.

(b) The neutron-to-proton ratio at $T = T_{\rm fo}$ is $n/p = e^{-1.293/T_{\rm fo}}$. Compute this ratio.

(c) Between $T_{\rm fo}$ and BBN ($T \sim 0.07$ MeV), neutrons $\beta$-decay with mean life $\tau_n = 879$ s. At $T = 0.07$ MeV, the cosmic time is $t_{\rm BBN} \approx (3/(32\pi G\rho))^{1/2} \approx 200$ s. The surviving $n/p$ ratio is $(n/p)_{\rm fo}\times e^{-t_{\rm BBN}/\tau_n}$. Compute this and hence the $^4$He mass fraction $Y_p$.

(d) If instead there were 4 neutrino species ($N_\nu = 4$), $g_*$ increases and the universe expands faster, raising $T_{\rm fo}$. Estimate the change in $Y_p$. This is how BBN constrains $N_\nu$: measured $Y_p$ implies $N_\nu < 3.5$ (2$\sigma$).

---

**49.3.** *Recombination and the sound horizon.*

(a) The sound speed in the photon-baryon fluid is $c_s = c/\sqrt{3(1+R)}$ where $R = 3\rho_b/(4\rho_\gamma)$. At $z = 1100$, compute $R$ and $c_s$.

(b) The sound horizon at recombination is:
$$r_s = \int_0^{t_*}\frac{c_s\,dt}{a(t)} = \int_0^{a_*}\frac{c_s\,da}{a^2 H(a)}$$
Numerically compute $r_s$ in Mpc for the $\Lambda$CDM parameters. Verify you get $r_s \approx 147$ Mpc.

(c) The angular size of the sound horizon on the CMB sky is $\theta_* = r_s/d_A(z_*)$. Compute $\theta_*$ and the corresponding multipole $\ell_1 = \pi/\theta_*$. Verify $\ell_1 \approx 220$.

---

## Thought Experiments

**T49.1.** *The anthropic puzzle of baryogenesis.*

If $\eta = 0$ (perfect matter-antimatter symmetry), all baryons and antibaryons would annihilate at $T \sim m_p/20 \approx 50$ MeV, leaving a universe of photons, neutrinos, and dark matter — with no atoms, stars, or observers.

If $\eta$ were $\gg 10^{-10}$ (many more baryons than photons), the early universe would have very little entropy per baryon. Nucleosynthesis would produce a different element distribution; structure formation would proceed differently.

The observed value $\eta \approx 6\times 10^{-10}$ might be a selection effect (only certain values allow observers to exist) or might be explained by a specific baryogenesis mechanism. Can you construct any argument that constrains the range of $\eta$ consistent with the existence of observers? Is this an anthropic argument or a physical prediction?

**T49.2.** *Thermodynamics and the arrow of time.*

The second law of thermodynamics says entropy increases. But the FLRW universe began in a state of very low entropy (for its energy) — the Big Bang initial state. Why? The thermal equilibrium of the early universe is a state of high entropy for radiation but low entropy for gravity (smooth spacetime). As gravity clumps matter, gravitational entropy increases even as thermodynamic entropy of matter also increases.

Penrose has argued that the extremely uniform initial conditions (low gravitational entropy) require fine-tuning more extreme than $e^{10^{123}}$ to one. Does inflation explain this? Or does it shift the problem to the pre-inflationary state? What would a "typical" beginning of the universe look like?

---

## Laboratory Exercise: Simulating Nucleosynthesis

**L49.1.** *Numerical BBN with a simple network.*

Using Python, implement a simplified BBN network with the key reactions:
- $n + \nu_e \leftrightarrow p + e^-$
- $p + e^- \leftrightarrow n + \nu_e$ 
- $n \to p + e^- + \bar{\nu}_e$ (free decay)
- $p + n \to D + \gamma$
- $D + D \to ^3\text{He} + n$
- $D + D \to T + p$
- $T + p \to ^4\text{He} + \gamma$
- $^3\text{He} + n \to ^4\text{He} + \gamma$

Integrate the reaction network using `scipy.integrate.solve_ivp`, evolving the temperature from $T = 10$ MeV to $T = 0.01$ MeV. Plot the mass fractions $Y(^4\text{He})$, D/H, $^3\text{He}$/H as a function of temperature.

Compare to the standard BBN code results (PArthENoPE, AlterBBN) and to the observed values. Vary $\Omega_b h^2$ to see the sensitivity of D/H.
