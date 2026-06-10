# Chapter 46: Gravitational Wave Detection

---

## Chapter Introduction

Detecting gravitational waves requires measuring changes in length smaller than $10^{-18}$ meters — a thousand times smaller than a proton — in arms four kilometers long. This is not merely an engineering challenge: it required fundamentally new ideas in optics, control systems, quantum measurement, and data analysis. The path from Einstein's 1916 prediction to the 2015 detection spans nearly a century of theoretical, experimental, and computational development.

Joseph Weber built the first gravitational wave detectors in the 1960s: resonant bar detectors ("Weber bars") that would ring like a struck bell if a gravitational wave passed through. Weber famously claimed detections in 1969, but his results were never reproduced by other groups. The community eventually concluded his coincidence analysis was flawed, but his work galvanized the field. The lesson was that gravitational wave detection required noise sources to be understood at the fundamental quantum level.

The idea of laser interferometry for gravitational wave detection was developed independently by Rainer Weiss (MIT) in 1972, and by Ronald Drever and Kip Thorne (Caltech). The key insight was that a Michelson interferometer measures differential arm length changes — precisely the quantity $\delta L/L = h/2$ that a gravitational wave produces. In principle, with long enough arms and enough laser power, the shot noise could be beaten down below the gravitational wave strain.

In practice, this required solving dozens of difficult problems: seismic isolation spanning 10 orders of magnitude in noise reduction; optical cavities to multiply the effective photon travel distance; power recycling mirrors to increase the circulating laser power; thermal noise from mirror suspension fibers; quantum radiation pressure fluctuations; non-stationary detector noise "glitches"; and global networks for event localization and background rejection.

LIGO (the Laser Interferometer Gravitational-Wave Observatory) began operations in 2002. After years of operations at Initial LIGO sensitivity without detections, the detector was upgraded to Advanced LIGO, with projected sensitivity $\sim 10\times$ better. On September 14, 2015, during the first science run of Advanced LIGO, the detectors in Hanford, Washington and Livingston, Louisiana recorded GW150914 — the gravitational wave signal from the merger of two black holes 1.3 billion light-years away.

This chapter describes how gravitational wave detectors work, how signals are extracted from noise, what GW150914 and subsequent events revealed, and where gravitational wave astronomy is headed.

---

## Chapter Sections

- [Section 46.1: Interferometric Detection and LIGO](section-46.1-ligo/README.md)

---

## Important Concepts

**Michelson interferometer principle**: A laser beam is split into two perpendicular arms, reflects off mirrors at the end, and recombines at the beamsplitter. The interference pattern depends on the phase difference $\Delta\phi = 4\pi\delta L/\lambda$ where $\delta L$ is the differential arm length. A gravitational wave with strain $h$ and frequency $f$ produces $\delta L = hL/2$, so $\Delta\phi = 2\pi h L/\lambda$.

**Shot noise**: Photons arrive at the photodetector as a Poisson process. With $N$ photons per measurement time, the phase uncertainty is $\delta\phi_{\rm shot} = 1/\sqrt{N}$. Higher laser power reduces shot noise; advanced LIGO uses 100 W laser power and builds up $\sim 200$ kW in the arm cavities via resonant power buildup.

**Radiation pressure noise**: Fluctuations in photon number impart random kicks to the mirror via radiation pressure. This scales opposite to shot noise — increasing laser power makes it worse. The balance point is the Standard Quantum Limit (SQL), a fundamental bound from the Heisenberg uncertainty principle: $h_{\rm SQL} \sim \sqrt{\hbar/(m L^2 f^2)}$. Advanced LIGO operates near the SQL in its most sensitive band, and techniques (squeezed light injection) can surpass it.

**Seismic isolation**: Ground vibrations from wind, traffic, ocean waves, and earthquakes would overwhelm gravitational wave signals. LIGO uses multiple-stage pendulum suspensions (the "quadruple pendulum") to reduce seismic noise by $10^{10}$ at frequencies above $\sim 10$ Hz. The mirrors (40 kg fused silica "test masses") hang from the final stage, each costing $\sim \$1$M to produce with the required surface quality.

**Thermal noise**: Random thermal vibrations of mirror surfaces and suspension fibers — from the fluctuation-dissipation theorem $S_x(f) \propto k_T\text{Im}[\chi(f)]/f$ — set the noise floor in the most sensitive band (~100–300 Hz). This is reduced by using low-mechanical-loss materials (fused silica, silicon) and cryogenic cooling (proposed for future detectors).

**Matched filtering**: A known waveform template $\tilde{h}(f)$ is optimally extracted from noise by computing the inner product $\langle d, h\rangle = 4\text{Re}\int_0^\infty \frac{\tilde{d}(f)\tilde{h}^*(f)}{S_n(f)}df$ where $S_n(f)$ is the noise power spectral density. The matched filter SNR is $\rho^2 = 4\int_0^\infty |\tilde{h}(f)|^2/S_n(f)\,df$. Matched filtering is optimal (Wiener-optimal) for Gaussian noise.

**False alarm rate**: The rate at which random noise fluctuations mimic a signal. For a detection to be claimed, the SNR must exceed a threshold corresponding to a false alarm rate of $<1/\text{yr}$. GW150914 had false alarm rate $< 1$ per 203,000 years. The probability that it was noise was $< 2\times 10^{-7}$.

**Parameter estimation**: Bayesian inference with Markov Chain Monte Carlo (MCMC) recovers the posterior distribution over all waveform parameters ($\mathcal{M}$, mass ratio $q$, spins, sky position, distance, inclination, polarization, coalescence time/phase). GW150914's parameters were measured to be: $m_1 = 36^{+5}_{-4} M_\odot$, $m_2 = 29^{+4}_{-4} M_\odot$, $M_f = 62^{+4}_{-4} M_\odot$, $a_f = 0.67^{+0.05}_{-0.07}$, $r = 410^{+160}_{-180}$ Mpc, $E_{\rm rad} = 3.0^{+0.5}_{-0.4} M_\odot c^2$.

**The GWTC catalogs**: LIGO/Virgo/KAGRA have released gravitational wave transient catalogs (GWTC-1, GWTC-2, GWTC-3) covering O1 (2015), O2 (2016–17), and O3 (2019–20). As of GWTC-3: 90 confirmed compact binary coalescence events — including 80+ BBH, several BNS, and the first NSBH events.

---

## Important Figures

**Rainer Weiss (born 1932)**: Proposed the laser interferometric gravitational wave detector in his 1972 MIT internal report. Founded and led the LIGO project from the MIT side. Nobel Prize in Physics 2017.

**Kip Thorne (born 1940)**: Led the Caltech component of LIGO and made foundational contributions to the theoretical understanding of GW sources, waveforms, and detector noise. Nobel Prize in Physics 2017.

**Barry Barish (born 1936)**: Led LIGO as Director from 1994 to 2005 during the transition from a scientific project to a full-scale observatory. Created the LIGO Scientific Collaboration. Nobel Prize in Physics 2017.

**Ronald Drever (1931–2017)**: Experimental physicist at Glasgow and Caltech, co-founder of LIGO with Weiss. Developed key innovations in optical cavity design, power recycling, and length stabilization crucial for LIGO's operation.

**Peter Saulson (born 1954)**: Made foundational contributions to the analysis of thermal noise in LIGO interferometers and the theory of displacement noise in gravitational wave detectors.

**Curt Cutler (born 1960) and Éanna Flanagan (born 1964)**: Developed the theoretical framework for parameter estimation in compact binary coalescence (1994), including the Fisher matrix formalism and the matched-filtering signal model.

**Joseph Weber (1919–2000)**: Built the first gravitational wave detectors (resonant bars) and made the first (contested) claims of detection. Though his results were not reproduced, he inspired the field and demonstrated that gravitational wave detection was an experimental science, not merely a theorist's fantasy.

---

## Further Reading

**Abbott, B.P. et al. (LIGO Scientific Collaboration and Virgo Collaboration) (2016). "Observation of Gravitational Waves from a Binary Black Hole Merger." *Physical Review Letters*, 116, 061102.**
The GW150914 discovery paper. One of the most cited papers in physics.

**Weiss, R. (1972). "Electromagnetically Coupled Broadband Gravitational Antenna." *Quarterly Progress Report of the MIT Research Laboratory of Electronics*, 105, 54.**
The foundational MIT report proposing laser interferometric GW detection — not published in a journal, but one of the most important documents in experimental GR.

**Abramovici, A. et al. (1992). "LIGO: The Laser Interferometer Gravitational-Wave Observatory." *Science*, 256, 325.**
The original LIGO proposal paper.

**Abbott, B.P. et al. (2017). "GW170817: Observation of Gravitational Waves from a Binary Neutron Star Inspiral." *Physical Review Letters*, 119, 161101.**
The binary neutron star merger detection, with simultaneous electromagnetic counterpart.

**Abbott, B.P. et al. (2021). "GWTC-2: Compact Binary Coalescences Observed by LIGO and Virgo during the First Half of the Third Observing Run." *Physical Review X*, 11, 021053.**
Second gravitational wave transient catalog.

**Saulson, P.R. (2017). *Fundamentals of Interferometric Gravitational Wave Detectors.* 2nd edition. World Scientific.**
The most thorough textbook treatment of interferometric detector physics.

**Maggiore, M. (2007). *Gravitational Waves, Vol. 1: Theory and Experiments.* Oxford University Press.**
Chapters 9–10 on interferometric detectors and noise sources.

---

## Exercises

**46.1.** *Interferometer response function.*

A Michelson interferometer has arms of length $L$ along the $x$ and $y$ directions. A gravitational wave with $+$ polarization propagates in the $z$-direction.

(a) Show that the differential arm length change is $\delta L_x - \delta L_y = h_+ L$.

(b) The interferometer response depends on the sky position $(\theta,\phi)$ and polarization angle $\psi$ of the source through antenna pattern functions $F_+(\theta,\phi,\psi)$ and $F_\times(\theta,\phi,\psi)$. The measured strain is $h(t) = F_+ h_+(t) + F_\times h_\times(t)$. For an optimally oriented source ($\theta = 0$, face-on), $F_+ = 1$ and $F_\times = 0$. For an edge-on source ($\theta = \pi/2$, $\phi = 0$), what are $F_+$ and $F_\times$?

(c) The sky-averaged squared antenna response is $\langle F_+^2 + F_\times^2\rangle = 1/5$. If LIGO can detect face-on sources at distance $r_{\rm max}$, what is the average detection distance over all sky positions and inclinations?

---

**46.2.** *Noise budget and sensitivity.*

The sensitivity of a gravitational wave detector is characterized by the noise amplitude spectral density $S_n(f)^{1/2}$ (in units of 1/√Hz, or equivalently strain/√Hz).

(a) The shot noise limited strain sensitivity is:
$$h_{\rm shot}(f) = \frac{1}{L}\sqrt{\frac{h\nu\lambda}{P_{\rm circ}}} = \sqrt{\frac{2h\nu}{P_{\rm circ}}}\frac{1}{4\pi FL/\lambda}$$
where $F$ is the finesse of the arm cavity, $P_{\rm circ}$ is the circulating power, $\nu$ is the laser frequency, $\lambda$ is the wavelength. For LIGO ($L = 4$ km, $F = 280$, $P_{\rm circ} = 100$ kW, $\lambda = 1064$ nm), compute $h_{\rm shot}$ at $f = 100$ Hz.

(b) The radiation pressure noise is:
$$h_{\rm rad}(f) = \frac{1}{\pi^2 f^2 m L}\sqrt{\frac{\hbar\omega P_{\rm circ}}{c}}$$
For LIGO ($m = 40$ kg mirror), compute $h_{\rm rad}$ at $f = 100$ Hz. At what frequency do shot noise and radiation pressure noise become equal (the Standard Quantum Limit)?

(c) The thermal noise from the mirror suspension is approximately $h_{\rm therm}(f) \approx (f_0/f)^2\times 10^{-23}$/√Hz where $f_0 = 1$ Hz (very roughly). At what frequency does thermal noise dominate over shot noise?

---

**46.3.** *Matched filtering and detection.*

In matched filtering, the SNR for detecting a signal $h(t)$ in noise with power spectral density $S_n(f)$ is:
$$\rho^2 = 4\int_0^\infty \frac{|\tilde{h}(f)|^2}{S_n(f)}df$$

(a) For a binary black hole with $\mathcal{M} = 28.3 M_\odot$ at $r = 410$ Mpc, the peak gravitational wave strain is $h_{\rm peak} \approx 10^{-21}$ and the signal lasts $\sim 0.2$ s in band with a roughly flat spectrum $|\tilde{h}(f)|^2 \approx h_{\rm peak}^2 / (2\Delta f)$ over bandwidth $\Delta f = 115$ Hz. Using Advanced LIGO's design sensitivity $S_n^{1/2} \approx 10^{-23}$ Hz$^{-1/2}$ at $f = 100$ Hz, estimate the SNR $\rho$ for GW150914.

(b) The threshold SNR for detection (false alarm rate $< 1$/yr across both LIGO detectors with $\sim 10^8$ templates) is approximately $\rho_{\rm thresh} \approx 8$ per detector. What is the maximum distance at which a GW150914-like binary could be detected with SNR $= 8$?

(c) Suppose instead the binary has $\mathcal{M} = 1.2 M_\odot$ (like GW170817) and lasts 100 seconds in band. How does the accumulated SNR compare to GW150914-like at the same strain amplitude?

---

**46.4.** *Testing general relativity with gravitational waves.*

GW observations provide new tests of GR in the strong-field, highly dynamic regime.

(a) **Dispersion**: If the graviton has a mass $m_g$, GWs travel with velocity $v_{\rm GW} = c\sqrt{1 - (m_g c^2/\hbar\omega)^2}$. Higher-frequency components would arrive earlier. For GW150914 ($\Delta t_{\rm obs} < 0.09$ s over $\Delta f \approx 100$ Hz at $r = 410$ Mpc), place an upper bound on the graviton mass $m_g$. Express in eV/c².

(b) **Speed of gravity**: GW170817 (the binary neutron star merger) was detected 1.74 seconds before the gamma-ray burst GRB 170817A. The distance was $\sim 40$ Mpc. Assuming the gamma-ray burst was emitted at most $10$ seconds after the GW merger, place a constraint on $|v_{\rm GW} - c|/c$.

(c) **Ringdown spectroscopy**: If the final black hole is Kerr (as GR predicts), its quasi-normal modes are determined entirely by $M_f$ and $a_f$. Detecting the dominant and first subdominant ringdown mode independently determines $M_f$ and $a_f$ twice — a consistency check on the Kerr hypothesis. What measurements are required to perform this test, and why is it challenging with current detector sensitivity?

---

**Thought Experiment T46.1.** *The information in a gravitational wave.*

GW150914 was detected as a chirp lasting 0.2 seconds with a specific morphology. In those 0.2 seconds, LIGO extracted: the chirp mass ($\pm 2\%$), the mass ratio ($\pm 10\%$), the final mass, the final spin, the distance ($\pm 40\%$), the sky position ($\pm 600$ deg²), the inclination, the luminosity distance, and evidence that the merger remnant was consistent with a Kerr black hole (no post-merger signals inconsistent with Kerr). 

Consider: what is the information content of a gravitational wave signal? How does it compare to an electromagnetic observation of a similar duration? What does this tell you about the fundamental differences between GW and EM astronomy as information channels?

**Thought Experiment T46.2.** *If LIGO had detected in 2002.*

Initial LIGO (2002–2010) had sensitivity $\sim 10\times$ worse than Advanced LIGO, meaning a detection volume $\sim 1000\times$ smaller. GW150914 was at 410 Mpc; Initial LIGO's horizon for such events was $\sim 50$ Mpc. The estimated binary black hole merger rate implies $\sim 0.001$ events/yr at that volume.

If LIGO had detected a binary black hole merger in 2002, it would have required a source $\sim 5\times$ closer than GW150914. At that point (before numerical relativity waveforms for spinning precessing binaries were complete), would the signal have been interpretable? What would have been missing from the scientific extraction, and what would have been impossible to claim?

---

## Laboratory Exercise: Signal Processing for Gravitational Wave Detection

**L46.1.** *Analyzing public LIGO data for GW150914.*

The LIGO Open Science Center (LOSC) provides public access to all LIGO data. In this lab you will replicate the GW150914 analysis.

**Setup**: Install `gwpy` and `PyCBC` (both available via pip). Download the 32-second segment of GW150914 data using:
```python
from gwpy.timeseries import TimeSeries
H1 = TimeSeries.fetch_open_data('H1', 1126259446, 1126259478)
L1 = TimeSeries.fetch_open_data('L1', 1126259446, 1126259478)
```

**Task 1 (Whitening):** The raw detector data is dominated by low-frequency noise (seismic) and high-frequency noise (shot noise). Whiten the data by dividing the Fourier transform by the square root of the noise power spectral density. Plot the whitened strain time series. Can you see the GW150914 signal by eye?

**Task 2 (Bandpassing):** Apply a bandpass filter from 35–350 Hz to the whitened data. Plot the result. Overlay the LIGO/Virgo published waveform template for GW150914.

**Task 3 (Q-transform):** Compute the Q-transform (a time-frequency representation) of the whitened data. Plot the spectrogram. You should see a characteristic upward-sweeping chirp pattern. Identify the moment of merger.

**Task 4 (Matched filtering):** Use the PyCBC package to perform matched filtering with a template waveform:
```python
from pycbc.waveform import get_td_waveform
hp, hc = get_td_waveform(approximant='SEOBNRv4', 
                          mass1=36, mass2=29, spin1z=0, spin2z=0,
                          delta_t=1/4096., f_lower=20)
```
Compute the matched filter SNR time series and identify the peak at the merger time.

**Task 5 (Comparison):** Compute the measured SNR and compare to the published value ($\rho = 24$ combined for the two detectors). What fraction of the SNR comes from the inspiral vs. merger+ringdown portions of the signal?
