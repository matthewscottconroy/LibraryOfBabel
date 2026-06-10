# Section 46.1: Interferometric Detection and LIGO

---

## The Principle of Interferometric Detection

The effect of a gravitational wave on matter is to produce a tidal deformation — a stretching along one axis and compression along the perpendicular axis, oscillating in time. For a $+$-polarized wave propagating in the $z$-direction with strain amplitude $h$ and angular frequency $\omega_{\rm GW}$:
$$\delta L_x = +\frac{1}{2}h L_x\cos(\omega_{\rm GW}t), \quad \delta L_y = -\frac{1}{2}h L_y\cos(\omega_{\rm GW}t)$$

The **differential** arm length change is $\delta L \equiv \delta L_x - \delta L_y = hL$. This is precisely what a Michelson interferometer measures.

In a Michelson interferometer, a laser beam is split at a beamsplitter: half travels down one arm and reflects off a mirror ("test mass"), half travels down the perpendicular arm and reflects off another mirror. When recombined, the interference depends on the round-trip optical path difference $\Delta\phi = 4\pi\delta L/\lambda$. If $\delta L = hL/2$ (one arm stretches while the other compresses), then:
$$\Delta\phi = \frac{2\pi}{\lambda}hL$$

For LIGO ($L = 4$ km, $\lambda = 1064$ nm, $h \sim 10^{-21}$):
$$\Delta\phi = \frac{2\pi\times 10^{-21}\times 4000}{1.064\times 10^{-6}} \approx 2.4\times 10^{-11}\text{ rad}$$

This is an incredibly small phase — a fraction of a proton diameter in optical path length change. How can it possibly be measured?

---

## Building Up to LIGO Sensitivity

**Step 1: Fabry-Perot cavities.** Instead of a single mirror, LIGO uses Fabry-Perot resonant arm cavities with finesse $F \approx 280$. The light bounces back and forth $2F/\pi \approx 180$ times in each arm before leaving, multiplying the effective arm length by this factor. The effective optical path is $L_{\rm eff} = 4FL/\pi \approx 720$ km. This boosts the phase shift to:
$$\Delta\phi_{\rm eff} = \frac{2\pi}{\lambda}hL_{\rm eff} \approx 4\times 10^{-9}\text{ rad}$$

**Step 2: Power recycling.** With the arms resonant, the light leaving through the antisymmetric (dark) port drops dramatically — most light returns toward the laser. A "power recycling mirror" placed between the laser and beamsplitter forms another resonant cavity, building up the circulating power from $\sim 100$ W laser input to $\sim 100$ kW in each arm. Higher power means more photons, reducing shot noise.

**Step 3: Signal recycling.** A "signal recycling mirror" at the antisymmetric port resonantly enhances gravitational wave signals at a particular frequency, further increasing sensitivity in a chosen band.

**Step 4: Suspension and isolation.** The test masses (40 kg fused silica mirrors, 34 cm diameter, polished to $\lambda/10$ flatness) are suspended from multi-stage pendulums. The "quadruple pendulum" system at LIGO reduces seismic noise by a factor of $10^{10}$ at 10 Hz. The entire optical system sits on seismically isolated platforms.

**Step 5: Quantum noise reduction.** In O3 and beyond, LIGO injects "squeezed light" from a non-linear optical parametric amplifier into the dark port. Squeezing reduces shot noise at high frequencies below the shot noise limit (at the cost of increased radiation pressure noise at low frequencies, or vice versa — subject to uncertainty principle). This improved high-frequency sensitivity by $\sim 15$–$40\%$ in O3.

---

## Noise Sources and the Sensitivity Curve

The strain sensitivity of Advanced LIGO is characterized by the amplitude spectral density $S_h(f)^{1/2}$ in units of $1/\sqrt{\rm Hz}$. At design sensitivity, this curve has several distinct regions:

**Below 10 Hz**: Dominated by seismic noise — ground vibrations that couple mechanically into the mirrors despite the isolation. This sets the low-frequency cutoff of the detector. Future underground detectors (Einstein Telescope, Cosmic Explorer) will extend this to $\sim 3$ Hz.

**10–50 Hz**: Dominated by thermal noise from the suspension fibers and mirror substrate. The Brownian motion of atoms in the silica creates length fluctuations; the fluctuation-dissipation theorem relates this to the mechanical loss angle. Cryogenic detectors (KAGRA uses cryogenic sapphire mirrors; the Einstein Telescope will use silicon at 10K) dramatically reduce this noise.

**50–300 Hz**: The most sensitive band — "shot noise limited" — where Advanced LIGO reaches $\sim 8\times 10^{-24}$ Hz$^{-1/2}$ at 100 Hz. This is where binary black hole mergers and binary neutron star late inspiral are detected.

**Above 300 Hz**: Dominated by shot noise, scaling as $\sqrt{f}$. Quantum squeezing improves this region.

**Discrete spectral lines**: Powerline frequency ($60$ Hz and harmonics in the US), mirror suspension violin modes ($\sim 500$ Hz, $\sim 1000$ Hz), calibration lines — these must be identified and accounted for.

The sensitivity curve determines the detection range ("horizon") for a given source:
$$r_{\rm horizon} = \frac{2G\mathcal{M}}{c^2}\frac{c}{G\mathcal{M}/c^3}\int_0^{f_{\rm ISCO}}\frac{|\tilde{h}(f)|^2}{S_h(f)}df \times \frac{1}{\rho_{\rm min}^2}$$

More practically, for binary black holes of $\mathcal{M}\sim 30 M_\odot$, Advanced LIGO at design sensitivity can detect mergers to $\sim 5$ Gpc — covering most of the observable universe.

---

## Data Analysis: From Raw Strain to Events

The LIGO detectors produce a continuous data stream sampled at 16384 Hz (sometimes 4096 Hz). The goal is to find rare gravitational wave events in this noise-dominated data.

**Matched filter search.** The data $d(t) = h(t) + n(t)$ is the signal plus noise. The matched filter SNR time series is:
$$\rho(t_c) = \frac{\langle d, h\rangle}{\sqrt{\langle h, h\rangle}}$$

where the inner product is $\langle a, b\rangle = 4\text{Re}\int_0^\infty \frac{\tilde{a}(f)\tilde{b}^*(f)}{S_n(f)}df$. This is optimal in Gaussian noise (the Neyman-Pearson theorem guarantees this maximizes detection probability at fixed false alarm rate). For non-Gaussian noise, chi-squared consistency tests must also pass.

**Template banks.** No single template covers all possible compact binary parameters. A bank of $\sim 10^5$–$10^6$ templates covers the parameter space $(m_1, m_2, \chi_1, \chi_2)$ with maximum mismatch $\leq 3\%$ (ensuring $<3\%$ SNR loss). Each template is $\mathcal{O}(10^6)$ samples long for binary neutron stars; the entire search requires $\mathcal{O}(10^{12})$ floating-point operations per second.

**Coincidence requirement.** GW events must appear in at least two detectors within the light-travel time ($\sim 10$ ms for Hanford-Livingston at 3000 km). This requirement enormously reduces false alarms from terrestrial noise artifacts ("glitches").

**Background estimation.** The false alarm rate is estimated by time-sliding the data from the two detectors relative to each other. Any genuine GW signal cancels out after the time slide; only noise coincidences remain. With hundreds of seconds of time slides, false alarm rates can be measured down to $\sim 10^{-5}$ yr$^{-1}$.

**GW150914: The detection.** The matched-filter SNR time series peaked at $\rho = 13.0$ in Hanford and $\rho = 11.3$ in Livingston, with a network SNR of $\rho_{\rm net} = 24.4$. The time between the peaks (6.9 ms) was consistent with the $\sim 10$ ms light travel time between the detectors, with H1 arriving first. The false alarm probability was $< 2\times 10^{-7}$ — an unambiguous detection.

The signal matched a binary black hole coalescence template with $m_1 = 36 M_\odot$, $m_2 = 29 M_\odot$. The 200 ms of signal swept from 35 to 150 Hz, with the chirp, merger, and ringdown all clearly visible in the data.

---

## GW150914 in Detail

September 14, 2015. 09:50:45 UTC. LIGO had been running in engineering mode for just days before its first science run. The signal lasted 0.2 seconds and was audible when shifted to audio frequencies — a low-pitched "thump" rising to a click.

**What was detected**: Two black holes, $36 M_\odot$ and $29 M_\odot$, spiraling together and merging into a single $62 M_\odot$ black hole. The remaining $3 M_\odot c^2 = 5.3\times 10^{47}$ J was radiated as gravitational waves in a fraction of a second. At peak, the power was $\sim 3.6\times 10^{49}$ W — more than the electromagnetic luminosity of all stars in the observable universe combined.

**What it proved**:
1. Binary black holes exist and merge within the age of the universe
2. Gravitational waves exist and travel at (essentially) the speed of light
3. General relativity is correct in the strong-field, highly-dynamical regime
4. Black holes at $\sim 30 M_\odot$ — intermediate between stellar and galactic — exist in nature
5. Laser interferometry can achieve the sensitivity to measure $10^{-18}$-meter displacements

**What was tested by GW150914**:
- Post-Newtonian expansion through merger: consistent with GR
- Ringdown frequency ($f_{\rm QNM} \approx 150$ Hz, $\tau \approx 4$ ms): consistent with Kerr black hole
- Absence of graviton mass dispersion: $m_g < 1.2\times 10^{-22}$ eV/c²
- No birefringence (GR predicts none): confirmed to one part in $10^{14}$ in propagation speed

---

## Multi-Messenger Astronomy: GW170817

On August 17, 2017, LIGO/Virgo detected GW170817 — a binary neutron star inspiral — at a distance of 40 Mpc. This was the first gravitational wave event with an electromagnetic counterpart.

**Timeline**:
- $t = 0$: LIGO detects the merger of two neutron stars, 40 Mpc away
- $t = 1.74$ s: Fermi and Integral detect GRB 170817A, a short hard gamma-ray burst — the first direct connection between binary neutron star mergers and short GRBs
- $t = 11$ hours: Optical transient AT2017gfo detected by the Swope Telescope — a "kilonova" (macronova), powered by r-process nucleosynthesis of heavy elements in the merger ejecta
- $t$ to weeks: Detected across the EM spectrum (UV, optical, IR, X-ray, radio) over the following weeks as the kilonova faded and jet afterglow appeared

**Scientific results from GW170817**:
1. **Speed of gravity**: $|v_{\rm GW} - c|/c < 10^{-15}$ — gravitational waves travel at $c$ to 15 significant figures, ruling out entire classes of modified gravity theories
2. **Neutron star equation of state**: tidal deformability $\tilde{\Lambda} = 300^{+420}_{-230}$, constraining $R_{1.4} = 11.9 \pm 1.4$ km
3. **Origin of short GRBs**: confirmed for the first time
4. **Nucleosynthesis**: kilonova emission confirmed that neutron star mergers are major sites of $r$-process (heavy element) synthesis — gold, platinum, uranium
5. **Hubble constant**: $H_0 = 70^{+12}_{-8}$ km/s/Mpc from the gravitational wave distance and optical redshift — a new, standard-siren measurement independent of the cosmic distance ladder

---

## The GWTC Catalogs and Population Science

As of GWTC-3 (O3 results), LIGO/Virgo/KAGRA have reported 90 confirmed gravitational wave events:

**Binary black holes (BBH)**: $\sim 80$ events. Range of masses from $\sim 5 M_\odot$ to $\sim 100 M_\odot$ per component. Several "intermediate mass ratio" events. One event (GW190521) produced a remnant of $\sim 142 M_\odot$ — in the "pair instability" mass gap, suggesting hierarchical formation in dense environments.

**Binary neutron stars (BNS)**: 2 confident events (GW170817, GW190425). The rate constrains neutron star merger rates in the local universe.

**Neutron star–black hole (NSBH)**: 2 events (GW200105, GW200115). The mass asymmetries are consistent with a $\sim 8 M_\odot$ black hole and $\sim 1.9 M_\odot$ neutron star.

**Astrophysical insights from population analysis**:
- The BBH mass distribution shows peaks at $\sim 10 M_\odot$ and $\sim 35 M_\odot$, with a dip near $\sim 10$–$12 M_\odot$ (the "lower mass gap")
- Most BBH mergers show low effective aligned spins $\chi_{\rm eff} \approx 0$, suggesting formation in dynamical environments (globular clusters) or large spin misalignment from isolated binary evolution
- The merger rate is $\mathcal{R}_{\rm BBH} \approx 20$–$100$ Gpc$^{-3}$yr$^{-1}$, $\mathcal{R}_{\rm BNS} \approx 100$–$1700$ Gpc$^{-3}$yr$^{-1}$

---

## Future Detectors

The gravitational wave observational program is expanding across multiple frequency bands:

**A+ / A# upgrades (2020s)**: Incremental improvements to Advanced LIGO with squeezed light, better mirrors, and reduced thermal noise. Factor of $\sim 2$ improvement in range.

**Einstein Telescope (ET, Europe, 2030s)**: A triangular underground detector with 10 km arms, cryogenic silicon mirrors, and improved isolation. Expected to detect $\sim 10^5$ BBH events per year, performing cosmology across the universe.

**Cosmic Explorer (CE, US, 2030s)**: An L-shaped detector with 20–40 km arms. Together with ET, would enable detection of BBH mergers throughout the observable universe and BNS to $z \sim 2$.

**LISA (ESA, $\sim 2035$)**: Space-based laser interferometer with $2.5\times 10^9$ km arms in a triangular configuration, trailing Earth by $20°$. Targets millihertz GWs from massive black hole binaries, galactic binaries, and EMRIs. Will detect hundreds of thousands of sources.

**Pulsar Timing Arrays (PTAs)**: NANOGrav, PPTA, EPTA — currently providing evidence for a nHz stochastic background. The International Pulsar Timing Array (IPTA) combines data for maximum sensitivity. SKA (Square Kilometre Array) will time thousands of pulsars.

The next decade of gravitational wave astronomy will transform our understanding of black hole demographics, neutron star physics, cosmology, and the large-scale structure of spacetime across the cosmos.
