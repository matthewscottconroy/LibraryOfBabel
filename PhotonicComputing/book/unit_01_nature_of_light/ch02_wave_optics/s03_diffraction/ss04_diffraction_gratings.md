# 2.3.4 — Diffraction Gratings

## Multiple-Slit Interference

A diffraction grating consists of $N$ equally spaced slits (or lines) with period $\Lambda$ (the *grating pitch*). The total field at angle $\theta$ is the sum of contributions from all $N$ slits:

$$E(\theta) = E_\text{single}(\theta) \sum_{j=0}^{N-1} e^{ij\delta}$$

where $\delta = (2\pi\Lambda\sin\theta)/\lambda$ is the phase difference between adjacent slits and $E_\text{single}(\theta)$ is the single-slit amplitude (the diffraction envelope from Section 2.3.2). The geometric sum:

$$\sum_{j=0}^{N-1} e^{ij\delta} = e^{i(N-1)\delta/2} \frac{\sin(N\delta/2)}{\sin(\delta/2)}$$

gives the *interference factor*. The intensity pattern is:

$$I(\theta) = I_\text{single}(\theta) \cdot \frac{\sin^2(N\delta/2)}{\sin^2(\delta/2)} \cdot \frac{1}{N^2}$$

(normalized so the peak equals $I_\text{single}$, not $N^2 I_\text{single}$, for fair comparison).

## The Grating Equation

The intensity has principal maxima (bright fringes) where $\delta/2 = m\pi$, i.e., $\Lambda\sin\theta = m\lambda$:

$$\Lambda\sin\theta = m\lambda, \quad m = 0, \pm 1, \pm 2, \ldots$$

This is the **grating equation**. Each integer $m$ is a *diffraction order*. The $m = 0$ order is straight through (independent of wavelength). Orders $m \neq 0$ are dispersed: different wavelengths appear at different angles $\theta_m = \arcsin(m\lambda/\Lambda)$.

**Angular dispersion**: $d\theta/d\lambda = m/(\Lambda\cos\theta)$. For a grating at $\theta = 0$ (normal incidence), $m = 1$, $\Lambda = 1$ μm: $d\theta/d\lambda \approx 1/\Lambda = 1$ rad/μm = $10^{-6}$ rad/nm $\approx 57°$/μm. This is why gratings are used as wavelength-selective components: they separate different wavelengths by small but measurable angles.

**Peak width (resolving power)**: The $m$-th order principal maximum has angular width $\Delta\theta = \lambda/(N\Lambda\cos\theta) = \lambda/(D\cos\theta)$, where $D = N\Lambda$ is the total grating width. The minimum resolvable wavelength difference (Rayleigh criterion):

$$\Delta\lambda_\text{min} = \frac{\lambda}{mN}$$

The resolving power $\mathcal{R} = \lambda/\Delta\lambda = mN$ — proportional to the number of lines and the diffraction order. A grating with $N = 10,000$ lines used in first order has $\mathcal{R} = 10,000$: it can resolve wavelength differences of $\lambda/10,000 = 0.155$ nm at 1550 nm.

## Gratings in Photonic Computing

### Wavelength Demultiplexing (WDM)

In wavelength-division multiplexing (WDM), multiple wavelength channels (each carrying an independent data stream) share a single fiber or waveguide. A demultiplexer separates these channels by wavelength. Diffraction gratings — in various integrated forms — are the primary technology:

**Arrayed waveguide grating (AWG)**: An integrated-photonic version of a diffraction grating. Light from an input waveguide fans out in a free-propagation region (a slab waveguide), then enters a phased array of curved waveguides. Each waveguide has a different length (linearly increasing by $\Delta L$), providing a linear phase ramp across the array. The second free-propagation region then focuses different wavelengths to different output waveguides. The AWG is used in WDM multiplexers/demultiplexers in optical communications and as the wavelength-selection component in some photonic neural network weight bank architectures.

**Echelle grating**: A high-order grating (large $m$) provides very high angular dispersion in a compact footprint. Etched echelle gratings in silicon-on-insulator can separate C-band channels with 0.4 nm spacing in a footprint of $\sim 1$ mm × $1$ mm [1].

### Grating Couplers

Grating couplers are the primary interface between photonic integrated circuits and optical fibers in laboratory settings. A periodic perturbation in the waveguide surface (a grating with period $\Lambda$) adds or subtracts the grating vector $G = 2\pi/\Lambda$ from the waveguide mode's wavevector, allowing phase-matched coupling to free-space radiation at a specific angle:

$$n_\text{eff} \frac{2\pi}{\lambda} - \frac{2\pi}{\Lambda} = n_c \frac{2\pi}{\lambda}\sin\theta_c$$

where $n_\text{eff}$ is the waveguide effective index, $n_c$ is the cladding index, and $\theta_c$ is the coupling angle (typically $\sim 10°$ off normal for silicon-on-insulator couplers at 1550 nm). The coupling efficiency of grating couplers is limited by upward/downward symmetry and by diffraction bandwidth; state-of-the-art values exceed 80% coupling efficiency [2].

The coupling bandwidth (range of wavelengths efficiently coupled) is related to the angular acceptance of the grating by the dispersion relation. This sets a fundamental tradeoff: broadband grating couplers have wider acceptance angles and lower peak efficiency; narrowband couplers can achieve near-unity efficiency over a narrow wavelength range.

### Spatial Light Modulators and Holography

A spatial light modulator (SLM) is a 2D array of independently controllable phase or amplitude pixels. When programmed with the right pattern, it acts as a designed diffraction grating — routing light to specific diffraction orders or generating arbitrary wavefronts. This is the principle of holographic displays and holographic optical tweezers.

In photonic computing, SLMs are used for:
- **Programmable free-space optical processors**: Setting the Fourier-plane mask in a 4f system.
- **Optical beam steering**: Directing a laser beam to arbitrary angles for LiDAR or free-space optical communication.
- **Reconfigurable optical interconnects**: Routing light between different processors.

The key limitation of current SLMs: pixel pitch $\sim 5$–10 μm, limiting the maximum diffraction angle to $\sim \lambda/(2\Lambda_\text{pixel}) \approx 5°$–10°. Higher-density programmable gratings would allow wider-angle steering but require semiconductor fabrication at sub-micron scales.

## The Grating as a Fourier Analyzer

We close with the observation that a diffraction grating is physically a Fourier analyzer: the intensity at each diffraction order is proportional to the squared magnitude of the corresponding Fourier component of the grating transmission function. An amplitude grating with transmission $t(x) = \sum_m T_m e^{im2\pi x/\Lambda}$ diffracts with amplitude $E_m \propto T_m$ — the $m$-th Fourier coefficient of the grating function.

This is a completely general statement: the Fraunhofer diffraction pattern of any periodic structure is the discrete Fourier transform of one period. Non-periodic structures have continuous Fourier transforms (their diffraction patterns are continuous, not discrete). Aperiodic structures designed to produce specific diffraction patterns (like a hologram) are literally programming a Fourier transform in physical space — writing a target spectrum as a physical structure.

## Summary

- Grating equation: $\Lambda\sin\theta = m\lambda$; resolving power $= mN$.
- Angular dispersion $d\theta/d\lambda = m/(\Lambda\cos\theta)$ separates wavelength channels.
- Integrated grating technologies: AWG, echelle gratings, grating couplers — all central to silicon photonic systems.
- A diffraction grating physically computes the Fourier transform of its transmission profile.

---

*References*

[1] Brouckaert, J., Bogaerts, W., Dumon, P., Van Thourhout, D., & Baets, R. (2007). Planar concave grating demultiplexer fabricated on a nanophotonic silicon-on-insulator platform. *Journal of Lightwave Technology*, 25(5), 1269–1275. [DOI: 10.1109/JLT.2007.893400]

[2] Marchetti, R. et al. (2019). Coupling strategies for silicon photonics integrated chips. *Photonics Research*, 7(2), 201–239. [DOI: 10.1364/PRJ.7.000201] [Comprehensive review of grating coupler designs and efficiencies.]
