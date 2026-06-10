# Chapter 45: Sources of Gravitational Waves

---

## Chapter Introduction

The quadrupole formula tells us that any system with a time-varying mass quadrupole moment radiates gravitational waves. But not all sources are equally loud. The gravitational wave luminosity scales as $P \sim G/c^5 \times (\text{quadrupole moment})^3$, and the factor $G/c^5 = 3.6\times 10^{-53}$ W$^{-1}$ is extraordinarily small. This is why gravitational waves from everyday objects are undetectable, while merging black holes can briefly outshine the entire electromagnetic universe.

The characteristic dimensionless strain from a source at distance $r$ is:
$$h \sim \frac{2G}{c^4 r}\ddot{Q} \sim \frac{4G\mu v^2}{c^4 r} \sim \frac{R_s}{r}\left(\frac{v}{c}\right)^2$$

where $R_s = 2GM/c^2$ is the Schwarzschild radius of the source. Gravitational wave emission is efficient only when two conditions hold: (1) masses are **compact** (comparable to their Schwarzschild radius), and (2) the dynamics are **relativistic** ($v \sim c$). This is why compact binary mergers — double neutron stars, neutron star–black hole binaries, and binary black holes — are the loudest sources.

The astrophysical landscape of gravitational wave sources is rich. In frequency, it spans twelve orders of magnitude: from nanohertz signals of supermassive black hole binaries (detected by pulsar timing arrays) through millihertz signals of stellar binaries and massive black hole mergers (the target of LISA) up to the tens-to-thousands of Hz band accessible to ground-based interferometers (LIGO/Virgo/KAGRA). Each frequency band probes a different population of sources and a different era in cosmic history.

This chapter applies the quadrupole formula to the major astrophysical sources, derives the inspiral waveform, explains the chirp mass, and discusses the gravitational wave background from unresolved populations.

---

## Chapter Sections

- [Section 45.1: Binary Inspiral and the Quadrupole Formula](section-45.1-quadrupole-formula/README.md)

---

## Important Concepts

**Chirp mass**: $\mathcal{M} = (m_1 m_2)^{3/5}/(m_1+m_2)^{1/5}$. This combination of masses controls the leading-order inspiral rate: $\dot{f}_{\rm GW} = \frac{96}{5}\pi^{8/3}\left(\frac{G\mathcal{M}}{c^3}\right)^{5/3}f_{\rm GW}^{11/3}$. The chirp mass is the best-measured parameter in a compact binary coalescence.

**GW frequency evolution**: For a circular inspiral, the GW frequency sweeps as $f_{\rm GW}(t) = \frac{1}{\pi}\left(\frac{5}{256}\frac{1}{t_c-t}\right)^{3/8}\left(\frac{G\mathcal{M}}{c^3}\right)^{-5/8}$, sweeping upward as the binary tightens and the merger time $t_c$ approaches.

**Characteristic strain**: For an inspiral lasting $N$ cycles near frequency $f$, the signal-to-noise accumulates as $h_c \sim h\sqrt{N}$. Sources spend more time at lower frequencies, so SNR per logarithmic frequency interval $\propto h_c^2/f$.

**Inspiral horizon distance**: The maximum distance at which a source can be detected with SNR $\sim 8$. For advanced LIGO: $\sim 450$ Mpc for binary neutron stars ($1.4+1.4 M_\odot$), $\sim 5$ Gpc for binary black holes ($30+30 M_\odot$).

**Neutron star equation of state**: Gravitational waves from binary neutron star mergers encode information about nuclear matter at supra-nuclear densities. The tidal deformability parameter $\Lambda \sim (c^2 R)^5/(G^5 M^5)$ affects the waveform at merger; GW170817 measured $\tilde{\Lambda} = 300^{+500}_{-190}$ and constrained $R_{1.4} = 11.9 \pm 1.4$ km.

**Stochastic gravitational wave background**: Superposition of unresolved binary mergers throughout the universe creates a stochastic background with characteristic strain $h_c^2(f) \propto f^{2/3}$ (for a population of inspiraling binaries). LIGO/Virgo/KAGRA began probing this background in O3.

---

## Important Figures

**Subrahmanyan Chandrasekhar (1910–1995)**: Showed that white dwarfs have a maximum mass ($1.4 M_\odot$, the Chandrasekhar limit). This set the stage for understanding neutron stars and black holes as the endpoints of stellar evolution and thus as gravitational wave sources.

**Jocelyn Bell (born 1943) and Antony Hewish (1924–2021)**: Discovered pulsars (1967). The subsequent realization that pulsars are rapidly rotating neutron stars made neutron stars a physical reality rather than a theoretical curiosity.

**Russell Hulse (born 1950) and Joseph Taylor (born 1941)**: Discovered the first binary pulsar, PSR B1913+16, in 1974. Over 20 years of timing, they measured the orbital period decrease and showed it agreed with GR's quadrupole formula to 0.1% — the first (indirect) observational proof that gravitational waves carry energy and orbital angular momentum. Nobel Prize in Physics 1993.

**Saul Teukolsky (born 1947)**: Derived the Teukolsky equation governing perturbations of Kerr black holes (1972), foundational for computing gravitational waveforms from compact object mergers.

**Frans Pretorius (born 1972)**: Made the first successful numerical relativity simulation of a binary black hole merger (2005), enabling accurate waveform templates for the merger and ringdown phases inaccessible to post-Newtonian methods.

---

## Further Reading

**Hulse, R.A. and Taylor, J.H. (1975). "Discovery of a Pulsar in a Binary System." *Astrophysical Journal Letters*, 195, L51.**
The original discovery paper for PSR B1913+16.

**Taylor, J.H. and Weisberg, J.M. (1982). "A New Test of General Relativity — Gravitational Radiation and the Binary Pulsar PSR 1913+16." *Astrophysical Journal*, 253, 908.**
The definitive demonstration of orbital energy loss to gravitational waves.

**Peters, P.C. (1964). "Gravitational Radiation and the Motion of Two Point Masses." *Physical Review*, 136, B1224.**
Derives the Peters formula for orbital inspiral under quadrupole radiation, still the standard at leading PN order.

**Peters, P.C. and Mathews, J. (1963). "Gravitational Radiation from Point Masses in a Keplerian Orbit." *Physical Review*, 131, 435.**
Computes the power spectrum of gravitational radiation from an eccentric binary.

**Pretorius, F. (2005). "Evolution of Binary Black-Hole Spacetimes." *Physical Review Letters*, 95, 121101.**
First successful numerical relativity simulation of binary black hole merger to gravitational wave emission.

**Abbott, B.P. et al. (LIGO/Virgo) (2017). "GW170817: Observation of Gravitational Waves from a Binary Neutron Star Inspiral." *Physical Review Letters*, 119, 161101.**
Detection of the first binary neutron star merger, with simultaneous electromagnetic counterpart.

**Cutler, C. and Flanagan, É.E. (1994). "Gravitational waves from merging compact binaries." *Physical Review D*, 49, 2658.**
Derives the Fisher matrix for parameter estimation from compact binary signals; foundational for data analysis.

**Maggiore, M. (2007). *Gravitational Waves: Theory and Experiments.* Oxford University Press.**
Comprehensive two-volume treatment (vol. 1: theory; vol. 2: astrophysical and cosmological sources). The standard reference.

---

## Exercises

**45.1.** *The Peters formula and inspiral time.*

The orbital separation of a circular binary decays as:
$$\frac{da}{dt} = -\frac{64G^3 m_1 m_2 (m_1+m_2)}{5c^5 a^3}$$

(a) By integrating this equation, show that the inspiral time from initial separation $a_0$ to merger is:
$$t_{\rm merge} = \frac{5c^5}{256 G^3}\frac{a_0^4}{m_1 m_2(m_1+m_2)}$$

(b) Compute $t_{\rm merge}$ for PSR B1913+16 ($m_1 = 1.44 M_\odot$, $m_2 = 1.39 M_\odot$, current semi-major axis $a_0 = 1.95 R_\odot$). The actual orbit is eccentric ($e = 0.617$), which enhances inspiral; the Peters enhancement factor for $e = 0.617$ is $f(e) \approx 11.9$. With this correction, what is $t_{\rm merge}$? (Answer: $\sim 300$ Myr.)

(c) The double pulsar PSR J0737-3039 has $m_1 = 1.338 M_\odot$, $m_2 = 1.249 M_\odot$, and $a_0 = 0.83 R_\odot$ (eccentricity $e = 0.088$). Compute $t_{\rm merge}$. How does it compare to PSR B1913+16?

---

**45.2.** *The chirp mass and frequency evolution.*

(a) Show that the GW frequency of an inspiraling circular binary evolves as:
$$\dot{f}_{\rm GW} = \frac{96\pi^{8/3}}{5}\left(\frac{G\mathcal{M}}{c^3}\right)^{5/3}f_{\rm GW}^{11/3}$$

where $\mathcal{M} = (m_1 m_2)^{3/5}(m_1+m_2)^{-1/5}$ is the chirp mass. (Hint: combine the Kepler law $f_{\rm GW} = 2f_{\rm orb} = \sqrt{G(m_1+m_2)/a^3}/\pi$ with the Peters formula for $\dot{a}$.)

(b) Integrate to find $f_{\rm GW}(t)$ as a function of time remaining to merger $\tau = t_c - t$.

(c) GW150914 showed $f_{\rm GW}$ sweeping from $\sim 35$ Hz to $\sim 150$ Hz over $\sim 0.15$ seconds. Estimate $\mathcal{M}$ from this chirp rate. The actual chirp mass was $\mathcal{M} = 28.3 M_\odot$. Does your estimate agree?

(d) GW170817 (binary neutron star) showed a chirp lasting $\sim 100$ seconds sweeping from $\sim 24$ Hz to $\sim 1000$ Hz. Estimate $\mathcal{M}$. (Answer: $\sim 1.2 M_\odot$, consistent with two $\sim 1.4 M_\odot$ neutron stars.)

---

**45.3.** *Gravitational wave luminosity and energy scales.*

(a) Using the quadrupole formula $P_{\rm GW} = \frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}$, compute the gravitational wave luminosity of PSR B1913+16 at its current separation. Express your answer in solar luminosities ($L_\odot = 3.83\times 10^{26}$ W).

(b) At the moment of last stable orbit ($a \approx 6GM_{\rm tot}/c^2$, assuming equal masses), estimate the peak luminosity of a binary black hole merger with $M_{\rm tot} = 60 M_\odot$ (like GW150914 pre-merger). Express as a fraction of the "Planck luminosity" $L_P = c^5/G \approx 3.6\times 10^{52}$ W. The answer should be $\sim 0.01 L_P$.

(c) GW150914 radiated approximately $3 M_\odot c^2$ of energy in $\sim 0.1$ seconds. Compute the average power in watts, and compare to the electromagnetic luminosity of the entire observable universe ($\sim 10^{49}$ W). Why does the quoted peak power ($\sim 200 M_\odot c^2$/s $= 3.6\times 10^{49}$ W) exceed the EM luminosity of all stars by a factor of $\sim 10$?

---

**45.4.** *Neutron star merger and tidal deformability.*

In a binary neutron star inspiral, the finite size of the neutron stars matters at late stages. The leading-order tidal effect enters the phase evolution at 5PN order through the tidal deformability $\tilde{\Lambda}$.

(a) The tidal deformability is $\Lambda = (2k_2/3)(c^2 R/GM)^5$ where $k_2$ is the Love number. For a typical neutron star ($M = 1.4 M_\odot$, $R = 12$ km, $k_2 \approx 0.09$), compute $\Lambda$.

(b) The tidal phase correction to the gravitational wave phase is approximately $\delta\Psi \approx -\frac{117\tilde{\Lambda}}{8}u^{10}$ where $u = (\pi\mathcal{M}f/c^3)^{1/3}$. Estimate the total phase correction from tidal effects for GW170817 ($\mathcal{M} = 1.188 M_\odot$) between 24 Hz and 1000 Hz. Is this detectable with LIGO (which can measure $\delta\Psi \gtrsim 0.01$ radians)?

(c) GW170817 measured $\tilde{\Lambda} < 800$ (90% credible). Using your formula from (a), what upper bound does this place on the neutron star radius $R$?

---

**Thought Experiment T45.1.** *What if the binary pulsar had been discovered in 1950?*

The Hulse-Taylor discovery came in 1974 and the orbital period decrease was confirmed by 1982. But the quadrupole formula itself dates to 1918 (Einstein) and the Peters formula to 1964. If PSR B1913+16 had been discovered in 1950, when the physics was already in place, would direct detection of gravitational waves have been prioritized earlier? What scientific infrastructure would have been needed in 1950 to actually detect GWs directly? What does this suggest about the relationship between theoretical predictions and experimental campaigns?

**Thought Experiment T45.2.** *The multi-messenger era.*

GW170817 was detected simultaneously in gravitational waves (LIGO/Virgo), gamma rays (Fermi/Integral), and then across the EM spectrum from radio to X-ray. This "multi-messenger" observation constrained: (1) the speed of gravitational waves ($|v_{\rm GW} - c|/c < 10^{-15}$), (2) the neutron star equation of state via tidal deformability, (3) the origin of short gamma-ray bursts, (4) the production site of heavy elements ($r$-process nucleosynthesis), and (5) an independent measurement of the Hubble constant ($H_0 = 70^{+12}_{-8}$ km/s/Mpc). How many of these were predicted before the detection? What does this illustrate about the scientific value of new observational windows?

---

## Laboratory Exercise: Simulating Binary Inspiral

**L45.1.** *Numerical inspiral and waveform generation in Python.*

**Task 1 (Orbital decay):** Integrate the Peters equation $da/dt = -C/a^3$ (where $C = 64G^3 m_1 m_2(m_1+m_2)/(5c^5)$) for a circular binary using `scipy.integrate.solve_ivp`. Verify that the inspiral time matches the analytic formula from Exercise 45.1.

**Task 2 (Waveform generation):** Construct the leading-order "restricted post-Newtonian" waveform:
$$h_+(t) = -\frac{4G\mathcal{M}}{c^2 r}\left(\pi G\mathcal{M}f/c^3\right)^{2/3}\cos\Phi(t)$$
where $\Phi(t) = 2\pi\int f_{\rm GW}(t)\,dt$. Plot the waveform for $\mathcal{M} = 28.3 M_\odot$ (GW150914-like) from $f_{\rm GW} = 20$ Hz to merger. Compare to the publicly available LIGO data.

**Task 3 (Chirp mass measurement):** Add Gaussian noise to the waveform. Implement a simple matched-filter search using `numpy.correlate`. By maximizing the SNR over a grid of $\mathcal{M}$ values, recover the chirp mass. What is the statistical uncertainty in $\mathcal{M}$ as a function of SNR?

**Task 4 (Stochastic background):** Simulate the gravitational wave background from a population of binary black hole mergers. Assume a merger rate $\mathcal{R} = 30$ Gpc$^{-3}$yr$^{-1}$ and a uniform distribution of chirp masses between 5 and 50 $M_\odot$. Compute the characteristic strain $h_c^2(f)$ and compare to the LIGO power-law sensitivity curve.
