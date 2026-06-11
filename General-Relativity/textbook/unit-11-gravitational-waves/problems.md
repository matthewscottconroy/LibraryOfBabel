# Unit XI Problems: Gravitational Waves

*Linearized gravity, gravitational wave generation, the quadrupole formula, and detection.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Gravitational Waves in Linearized GR

**Problem 1.1** ★
Plane gravitational wave solutions: in the TT (transverse-traceless) gauge, the metric perturbation for a wave propagating in the $z$-direction is:

$$h_{\mu\nu}^\text{TT} = \begin{pmatrix}0 & 0 & 0 & 0\\ 0 & h_+ & h_\times & 0\\ 0 & h_\times & -h_+ & 0\\ 0 & 0 & 0 & 0\end{pmatrix} \cos(\omega t - kz)$$

(a) Show this satisfies the linearized vacuum Einstein equations $\Box h_{\mu\nu} = 0$ and the TT gauge conditions $h_{\mu\nu}n^\nu = 0$ (transverse: $k = \omega/c$) and $h^\mu_{\ \mu} = 0$ (traceless).

(b) How many independent polarization states does a gravitational wave have? Compare to electromagnetism. Why is the spin of the graviton 2 (while the photon is spin 1)?

(c) The effect of $h_+$-polarization on a ring of test particles in the $x$-$y$ plane: compute the proper distance $\delta L$ between particles at $(L_0, 0)$ and $(-L_0, 0)$ as a function of time. The relative change is $\delta L/L_0 = \frac{1}{2}h_+\cos(\omega t)$. This is the **strain** $h$.

(d) For a gravitational wave source at distance $D = 400$ Mpc (the distance to GW150914), with strain $h = 10^{-21}$ and detector arm length $L = 4$ km: compute the actual arm length change $\delta L$ in meters. Compare to the size of a proton ($10^{-15}$ m).

**Problem 1.2** ★★
The TT gauge: the transverse-traceless projection of a symmetric perturbation $h_{\mu\nu}$ in the $z$-direction:

$$h_{ij}^\text{TT} = P_{ia}P_{jb}h_{ab} - \frac{1}{2}P_{ij}P_{ab}h_{ab}$$

where $P_{ij} = \delta_{ij} - n_in_j$ is the projection onto the transverse plane (with $n^i = \hat{z}$).

(a) For $h_{ij}$ with only $h_{xx} = -h_{yy} = A$, $h_{xy} = B$, all other components zero: compute $h_{ij}^\text{TT}$.

(b) For a symmetric perturbation with $h_{xz} = C$: show that $h_{xz}^\text{TT} = 0$ (longitudinal components are projected out).

(c) The TT gauge is a specific choice within the Lorenz gauge. What additional gauge transformations preserve the TT conditions?

---

## Part 2: Gravitational Wave Generation

**Problem 2.1** ★★
The quadrupole formula: the gravitational wave strain produced by a non-relativistic source with reduced quadrupole moment:

$$I_{ij} = \int \rho(x)\left(x_ix_j - \frac{1}{3}\delta_{ij}r^2\right)d^3x$$

is (in the far field):

$$h_{ij}^\text{TT}(t,\mathbf{x}) = \frac{2G}{c^4 r}\ddot{I}_{ij}^\text{TT}(t_\text{ret})$$

where $t_\text{ret} = t - r/c$.

(a) For two equal masses $M$ in circular orbit of radius $a$ at angular frequency $\Omega$: compute the quadrupole moment $I_{ij}(t)$.

(b) Compute $\ddot{I}_{ij}$ and extract the $h_+$ and $h_\times$ amplitudes for an observer on the $z$-axis (perpendicular to the orbital plane).

(c) For a binary at distance $D$: $h_+ = (4G\mu\Omega^2 a^2/c^4 D)(1+\cos^2\iota)/2$ where $\mu = M/2$ is the reduced mass and $\iota$ is the inclination angle. For $\iota = 0$ (face-on): compute $h_+$ for the first detected binary merger GW150914 ($M_1 = 36M_\odot$, $M_2 = 29M_\odot$, $D = 440$ Mpc, at the point of merger where $r_\text{orbit} \approx r_s^\text{total}$).

**Problem 2.2** ★★
The radiated power (Peters formula): the total gravitational wave power from a binary system:

$$P = -\frac{dE}{dt} = \frac{32G^4}{5c^5}\frac{(M_1M_2)^2(M_1+M_2)}{r^5}$$

(a) For a circular orbit of the Hulse-Taylor binary pulsar: $M_1 = M_2 = 1.4 M_\odot$, $r = 10^9$ m. Compute $P$ in watts.

(b) The orbital period derivative: from $E = -GM_1M_2/(2r)$ and Kepler's third law $T^2 = 4\pi^2r^3/(G(M_1+M_2))$: show that $\dot{T} = -\frac{192\pi}{5}\frac{G^{5/3}}{c^5}(M_\text{chirp}\pi/T)^{5/3}$ where $M_\text{chirp} = (M_1M_2)^{3/5}/(M_1+M_2)^{1/5}$.

(c) For the Hulse-Taylor pulsar ($T = 7.75$ hours): compute the predicted $\dot{T}$ and compare to the measured value $\dot{T}_\text{obs} = -2.4025\times10^{-12}$ (dimensionless). This Nobel-Prize-winning measurement (1993) was the first indirect evidence for gravitational waves.

**Problem 2.3** ★★★
The chirp signal: as a binary inspirals, the orbital frequency increases (the orbit shrinks), leading to a characteristic "chirp" signal.

(a) From the quadrupole formula, the frequency evolution: $\dot{f} = \frac{96\pi^{8/3}}{5c^5}G^{5/3}M_\text{chirp}^{5/3}f^{11/3}$.

(b) Integrate to find $f(t)$ as a function of the time-to-merger $\tau = t_\text{merge} - t$: $f(\tau) \propto \tau^{-3/8}$.

(c) For GW150914 ($M_\text{chirp} = 28.3 M_\odot$): how many orbits does the binary complete in the LIGO band ($10$–$1000$ Hz)? How long does the signal last in band?

(d) The chirp mass $M_\text{chirp}$ can be measured from $\dot{f}(f)$: $M_\text{chirp} = \frac{c^3}{G}\left[\frac{5}{96\pi^{8/3}}\frac{\dot{f}}{f^{11/3}}\right]^{3/5}$. Why is $M_\text{chirp}$ the best-determined parameter in a gravitational wave detection?

---

## Part 3: Gravitational Wave Detection

**Problem 3.1** ★★
LIGO interferometry:

(a) A Michelson interferometer with arm length $L$ detects a gravitational wave of strain $h$. The phase difference between the two arms: $\delta\phi = 2kh L$ (where $k = 2\pi/\lambda$). For $L = 4$ km, $h = 10^{-21}$, $\lambda = 1064$ nm: compute $\delta\phi$.

(b) The shot noise in the phase measurement: $\delta\phi_\text{shot} = 1/\sqrt{N_\text{photons}}$ per measurement. For input power $P_0 = 200$ W (with power recycling) and measurement bandwidth $B = 1000$ Hz:
$$\delta\phi_\text{shot} = \sqrt{\frac{h\nu}{P_0 T_\text{int}}} = \sqrt{\frac{h\nu B}{P_0}}$$
Compute $\delta\phi_\text{shot}$ at $\lambda = 1064$ nm.

(c) Is the shot noise below the signal phase shift? What is the minimum detectable strain at frequency 100 Hz?

(d) Quantum squeezing: LIGO uses squeezed light to reduce shot noise by a factor $\sim3$dB. Explain qualitatively how squeezing the light source helps, and what the SQL (standard quantum limit) is.

**Problem 3.2** ★★★
Matched filtering: the GW signal $h(t)$ is buried in detector noise $n(t)$. The matched filter output:

$$z = \int_{-\infty}^\infty \frac{\tilde{h}^*(f)\tilde{s}(f)}{S_n(f)}df$$

where $\tilde{s}(f)$ is the Fourier transform of the data, $\tilde{h}(f)$ the template, and $S_n(f)$ the one-sided power spectral density of the noise.

(a) The optimal SNR: $\rho^2 = 4\int_0^\infty|\tilde{h}(f)|^2/S_n(f)\,df$. For LIGO with $S_n(f) = S_0 = 10^{-47}$ Hz⁻¹ (constant, simplification) between $f_\text{low} = 10$ Hz and $f_\text{high} = 1000$ Hz, and $h_0 = 10^{-21}$ amplitude: estimate $\rho$.

(b) A gravitational wave detection requires $\rho > 8$ (SNR threshold). Is this signal detectable?

(c) The parameter estimation uses a Bayesian posterior $p(\theta|\text{data}) \propto e^{-\chi^2(\theta)/2}p(\theta)$ where $\chi^2 = \langle s - h(\theta), s - h(\theta)\rangle$ is the mismatch inner product. For the chirp mass: why is its posterior narrow (well-measured) while the mass ratio posterior is broad?
