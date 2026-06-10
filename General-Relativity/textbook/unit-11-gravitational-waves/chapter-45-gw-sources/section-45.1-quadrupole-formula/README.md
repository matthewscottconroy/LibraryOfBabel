# Section 45.1: Binary Inspiral and the Quadrupole Formula

---

## From Static Orbits to Inspiraling Spirals

The Hulse-Taylor binary pulsar does not orbit at fixed separation. Gravitational waves carry energy and angular momentum away from the system, causing the orbit to shrink — very slowly at first, then catastrophically fast at the end. The binary inspires inward along a sequence of quasi-circular orbits (for circular initial conditions), each orbit slightly tighter than the last, until the two compact objects reach separations comparable to their Schwarzschild radii and merge.

This evolution has three phases:

1. **Inspiral**: well-separated, quasi-circular (or quasi-elliptical) orbit; post-Newtonian (PN) expansion is valid; the waveform is a smooth chirp with increasing frequency and amplitude
2. **Merger**: separations $\sim GM_{\rm tot}/c^2$; highly relativistic, strong-field dynamics; only numerical relativity gives accurate waveforms
3. **Ringdown**: the merger remnant (a single black hole, or a hypermassive/supramassive neutron star) rings down through quasi-normal modes; the dominant mode for a Kerr black hole has frequency $f_{\rm QNM} \sim c^3/(2\pi G M_{\rm fin})(1 - 0.63(1-a/M)^{3/10})$ where $a/M$ is the final spin

For the LIGO band (10–1000 Hz), binary black holes spend from seconds (for $\sim 30+30 M_\odot$ at 10 Hz) to hundreds of seconds (for $\sim 1.4+1.4 M_\odot$ neutron stars) in band before merger.

---

## The Quadrupole Formula in Practice

Recall the mass quadrupole tensor:
$$Q^{ij} = \int \rho\left(x^i x^j - \frac{1}{3}\delta^{ij}r^2\right)d^3x$$

For two point masses in a circular orbit with angular frequency $\Omega$, orbital separation $a$, and reduced mass $\mu = m_1 m_2/(m_1+m_2)$, the quadrupole moment is:
$$Q^{ij} = \mu a^2\begin{pmatrix}\cos^2\Omega t & \cos\Omega t\sin\Omega t & 0\\ \cos\Omega t\sin\Omega t & \sin^2\Omega t & 0\\ 0 & 0 & 0\end{pmatrix} - \frac{1}{3}\mu a^2\delta^{ij}$$

The trace-subtracted form has components oscillating at twice the orbital frequency. Taking three time derivatives (as required by the quadrupole formula) and contracting:
$$\dddot{Q}_{ij}\dddot{Q}^{ij} = 32\mu^2 a^4\Omega^6$$

The total radiated power is:
$$P_{\rm GW} = \frac{G}{5c^5}\dddot{Q}_{ij}\dddot{Q}^{ij} = \frac{32G\mu^2 a^4\Omega^6}{5c^5}$$

Using Kepler's law $\Omega^2 = G(m_1+m_2)/a^3 = GM_{\rm tot}/a^3$:
$$\boxed{P_{\rm GW} = \frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}}$$

This is the **Peters-Mathews formula** (1963). Notice the extreme sensitivity to separation: $P \propto a^{-5}$. As the orbit tightens, the power increases rapidly, accelerating the inspiral.

---

## Orbital Decay: The Peters Equation

By equating the radiated power to minus the time derivative of the total orbital energy $E = -Gm_1 m_2/(2a)$:
$$\frac{dE}{dt} = \frac{Gm_1 m_2}{2a^2}\dot{a} = -P_{\rm GW} = -\frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}$$

Solving for $\dot{a}$:
$$\frac{da}{dt} = -\frac{64G^3 m_1 m_2(m_1+m_2)}{5c^5 a^3}$$

This is the **Peters equation**. Integrating from $a_0$ to $0$:
$$t_{\rm merge} = \frac{5c^5}{256G^3}\frac{a_0^4}{m_1 m_2(m_1+m_2)} = \frac{5}{256}\frac{c^5}{G^3}\frac{a_0^4}{\mu M_{\rm tot}^2}$$

For PSR B1913+16 (with $m_1 = 1.4408 M_\odot$, $m_2 = 1.3874 M_\odot$, $a_0 = 1.95 R_\odot$, eccentricity $e = 0.6171$): the circular formula gives $\sim 350$ Myr, but the eccentricity enhancement $f(e) = (1+73e^2/24+37e^4/96)/(1-e^2)^{7/2}$ increases the merger rate by a factor of $\sim 12$, giving $t_{\rm merge} \approx 300$ Myr from now.

**The observed orbital period change.** The orbital period decreases as angular momentum is carried away by gravitational waves. Computing $\dot{P}_b = -(96\pi/5)(2\pi/P_b)^{5/3}(G\mathcal{M}_{\rm NS}/c^3)^{5/3}$ with eccentricity corrections:
$$\dot{P}_b^{\rm GR} = -2.402531\times 10^{-12}$$

Measured: $\dot{P}_b^{\rm obs} = -2.4184\pm 0.0009\times 10^{-12}$ (after correcting for galactic acceleration). The ratio is $0.9997 \pm 0.0004$ — agreement to 0.03%, a profound confirmation of GR.

---

## The Chirp Mass

The frequency evolution of an inspiraling binary is:
$$\frac{df_{\rm GW}}{dt} = \frac{96}{5}\pi^{8/3}\left(\frac{G\mathcal{M}}{c^3}\right)^{5/3}f_{\rm GW}^{11/3}$$

where the **chirp mass** $\mathcal{M} = \mu^{3/5}M_{\rm tot}^{2/5} = (m_1 m_2)^{3/5}(m_1+m_2)^{-1/5}$ is the single combination of masses that governs the leading-order inspiral.

The origin of the name is audible: when a gravitational wave signal from a compact binary inspiral is shifted to the audio band, it sounds like a bird's chirp — a rising tone that sweeps higher and louder as the binary tightens, then cuts off at merger. In GW150914, this chirp lasts $\sim 0.2$ seconds and sweeps from 35 to 150 Hz. For GW170817 (binary neutron stars), it lasts over 100 seconds from when it enters the LIGO band.

Integrating the frequency evolution gives:
$$f_{\rm GW}(t) = \frac{1}{\pi}\left[\frac{5}{256}\frac{c^3}{G\mathcal{M}}\frac{1}{t_c - t}\right]^{3/8}$$

The chirp mass is directly measurable from the frequency evolution: $\mathcal{M} \propto (df/dt)^{3/5}/f^{11/5}$. For GW150914, $\mathcal{M} = 28.3^{+1.4}_{-1.5} M_\odot$ — the dominant uncertainty coming from the merger and spin effects, not the inspiral chirp.

---

## Gravitational Wave Amplitude

The gravitational wave strain at distance $r$ from a circular binary is:
$$h_+(t) = -\frac{4G\mathcal{M}}{c^2 r}\left(\frac{\pi G\mathcal{M}f_{\rm GW}}{c^3}\right)^{2/3}\cos\Phi(t)$$
$$h_\times(t) = -\frac{4G\mathcal{M}}{c^2 r}\left(\frac{\pi G\mathcal{M}f_{\rm GW}}{c^3}\right)^{2/3}\sin\Phi(t)\cos\iota$$

where $\iota$ is the inclination of the orbital plane to the line of sight and $\Phi(t) = 2\pi\int^t f_{\rm GW}(t')dt'$ is the accumulated phase. The amplitude grows as $f_{\rm GW}^{2/3}$ — the wave grows louder as the frequency increases. At the innermost stable circular orbit (where the inspiral approximation breaks down), the amplitude reaches its peak before the merger.

For GW150914 ($\mathcal{M} = 28.3 M_\odot$, $r = 410$ Mpc, $f_{\rm peak} \approx 150$ Hz):
$$h_{\rm peak} \approx \frac{4G\mathcal{M}}{c^2 r}\left(\frac{\pi G\mathcal{M}f}{c^3}\right)^{2/3} \approx 10^{-21}$$

This matches the observed strain.

---

## Beyond Circular Orbits: Eccentricity

The Peters-Mathews result extends to elliptical orbits with eccentricity $e$. The orbit-averaged power radiated is:
$$P_{\rm GW}(a,e) = \frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}\cdot f(e)$$

where the eccentricity enhancement factor is:
$$f(e) = \frac{1 + (73/24)e^2 + (37/96)e^4}{(1-e^2)^{7/2}}$$

For $e = 0$: $f(0) = 1$. For $e = 0.617$ (PSR B1913+16): $f(0.617) \approx 11.9$. For $e\to 1$: $f(e)\to\infty$, meaning nearly radial orbits are extremely efficient gravitational wave emitters.

Crucially, gravitational radiation also circularizes orbits: the eccentricity decreases as:
$$\frac{de}{dt} = -\frac{304G^3 m_1 m_2(m_1+m_2)}{15c^5 a^4}\cdot e\cdot\frac{1 + (121/304)e^2}{(1-e^2)^{5/2}}$$

So by the time a binary enters the LIGO band (at $\sim 10$ Hz), its eccentricity is typically negligible. This is why the circular waveform models are accurate for most detected events.

---

## Post-Newtonian Corrections

The leading-order quadrupole formula is the 0PN approximation. Higher-order corrections are organized by powers of $(v/c)^2$:

| PN Order | $(v/c)^{2n}$ | Physical Content |
|----------|-------------|-----------------|
| 0PN | $1$ | Quadrupole radiation, Newtonian orbit |
| 1PN | $(v/c)^2$ | Orbital energy corrections, tail effects |
| 1.5PN | $(v/c)^3$ | Gravitational wave tails (backscattering from spacetime curvature) |
| 2PN | $(v/c)^4$ | Spin-orbit coupling |
| 2.5PN | $(v/c)^5$ | 1PN radiation reaction (leading dissipative term) |
| 3PN | $(v/c)^6$ | Spin-spin coupling, higher-order orbital corrections |
| 3.5PN | $(v/c)^7$ | Higher-order radiation reaction |
| 5PN | $(v/c)^{10}$ | Tidal deformability (for neutron stars) |

For GW150914, the waveform matched the data with PN expansion through 3.5PN for the inspiral phase, connected to numerical relativity for the merger. The agreement was at the $\sim 1\sigma$ level in all measured parameters.

---

## Gravitational Wave Waveforms: The Complete Picture

The full GW signal from a compact binary coalescence has three distinct morphologies:

**Inspiral**: a smooth chirp with amplitude growing as $h \propto f^{2/3}$ and phase described by PN expansion. Lasts from seconds (heavy black holes) to thousands of seconds (light neutron stars) in the LIGO band.

**Merger**: a burst of strong-field radiation lasting $\sim G(m_1+m_2)/c^3 \sim 10$ ms for a $60 M_\odot$ total-mass system. Requires numerical relativity. The amplitude peaks here.

**Ringdown**: the remnant black hole (for a BBH merger) vibrates at its quasi-normal mode frequencies. The dominant mode for a Kerr black hole has:
$$f_{\rm QNM} \approx \frac{c^3}{2\pi G M_f}\left(1 - 0.63(1 - a_f/M_f)^{0.3}\right)$$
$$\tau_{\rm QNM} \approx \frac{GM_f}{c^3 \times 2\pi}(1 - a_f/M_f)^{-0.45}\cdot 4$$

where $M_f$ and $a_f$ are the final mass and spin. The ringdown is a damped sinusoid, exponentially decaying in $\sim$ milliseconds.

The complete matched-filter template must cover all three phases. The LIGO Science Collaboration uses template banks covering $m_1, m_2 \in [1, 200] M_\odot$ and spin magnitudes up to 0.99, requiring $\sim 10^6$ templates for a complete search.

---

## Astrophysical Sources Across the Frequency Band

**Ground-based detectors (LIGO, Virgo, KAGRA): 10–1000 Hz**
- Binary neutron stars ($1.4 + 1.4 M_\odot$): chirp frequency sweeps across the full band; tidal effects imprint neutron star structure; most rate-constrained source
- Binary black holes ($5$–$100 M_\odot$ each): dominant source class by signal strength; GW150914 through GW230529 in the catalogs (O1–O3)
- Neutron star–black hole binaries: first detected in GWTC-3 (GW200105, GW200115)
- Core-collapse supernovae: stochastic burst signals from convection/standing accretion shock instability; much weaker; not yet detected

**Space-based detectors (LISA, targeting $\sim 2030$s): $0.1$ mHz–$0.1$ Hz**
- Galactic double white dwarfs: $\sim 10^4$ resolvable sources, $\sim 10^7$ unresolved; form a "galactic confusion noise" at low frequencies
- Massive black hole binaries ($10^5$–$10^7 M_\odot$): cosmological signals from galaxy mergers, visible to $z \sim 20$
- Extreme mass ratio inspirals (EMRIs): stellar-mass compact objects spiraling into $10^5$–$10^7 M_\odot$ black holes; tens of thousands of waveform cycles in band

**Pulsar timing arrays (NANOGrav, PPTA, EPTA, IPTA): nHz**
- Supermassive black hole binaries ($10^8$–$10^{10} M_\odot$): predicted stochastic background from the population; NANOGrav (2023) announced evidence for a stochastic gravitational wave background consistent with such a population
- Primordial gravitational waves from inflation: not yet detected; predicted to be $\sim 10^{16}$ times too faint for current PTAs

**The nHz background (NANOGrav 2023)**: The North American Nanohertz Observatory for Gravitational Waves announced in June 2023 strong evidence for a stochastic gravitational wave background consistent with an isotropic background of $h_c \propto f^{-2/3}$, the spectral shape expected from a population of inspiraling supermassive black hole binaries. This is the lowest-frequency gravitational wave detection to date and opens a new observational window on the most massive black holes in the universe.
