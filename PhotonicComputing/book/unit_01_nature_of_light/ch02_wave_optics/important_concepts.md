# Important Concepts — Chapter 2: Wave Optics

## Superposition and the Interference Intensity Formula

For two coherent waves of amplitudes $E_1$, $E_2$ and phase difference $\Delta\phi$:

$$I = I_1 + I_2 + 2\sqrt{I_1 I_2}\cos\Delta\phi$$

Fringe visibility: $V = 2\sqrt{I_1 I_2}/(I_1+I_2)$. Maximum visibility requires equal intensities. Interference requires same frequency, same polarization, stable phase (coherence time condition).

## Fabry-Pérot Resonator

Resonance condition: $L = m\lambda/(2n)$. Airy function gives sharp peaks with:
- Finesse: $F = \pi R^{1/2}/(1-R)$
- FSR: $\nu_\text{FSR} = c/(2n_g L)$
- FWHM: $\Delta\nu = \nu_\text{FSR}/F$
- Q factor: $Q = \nu/\Delta\nu = mF$

High finesse → narrow peaks, many round trips, slow photon decay. Tradeoff: high Q → limited modulation bandwidth. On-chip: ring resonators (silicon photonics).

## Mach-Zehnder Interferometer

Transfer function: $I_\text{out} = I_\text{in}(1 \pm \cos\Delta\phi)/2$. The MZI implements a unitary $2\times 2$ rotation in field space:

$$U_\text{MZI}(\Delta\phi) = e^{i\phi_0}\begin{pmatrix} i\cos(\Delta\phi/2) & \sin(\Delta\phi/2) \\ \sin(\Delta\phi/2) & i\cos(\Delta\phi/2) \end{pmatrix}$$

The $\pi/2$ phase in the off-diagonal elements of the beam splitter matrix is required by energy conservation and time-reversal symmetry.

**Reck/Clements theorem**: Any $N \times N$ unitary matrix can be decomposed into $N(N-1)/2$ MZIs. An MZI mesh therefore implements arbitrary unitary (and, via SVD, arbitrary linear) matrix multiplications on optical fields.

## Fraunhofer Diffraction = Spatial Fourier Transform

The far-field (Fraunhofer) diffraction pattern is the spatial Fourier transform of the aperture field:

$$E(\theta) \propto \hat{E}(\sin\theta/\lambda) = \int E(x') e^{-i2\pi x'\sin\theta/\lambda} dx'$$

Key results: single slit $\to$ sinc, circular aperture $\to$ Airy disk (first zero at $\sin\theta = 1.22\lambda/D$), Gaussian $\to$ Gaussian.

**A thin lens performs the Fourier transform** between its front and back focal planes. The 4f system implements convolution via Fourier-plane filtering. Diffractive neural networks exploit this directly.

## Polarization

Any polarization state = $\mathbf{J} = (E_x, E_y)^T$ (Jones vector). Optical elements = $2 \times 2$ complex matrices (Jones matrices). Key matrices: polarizers (rank-1 projectors), wave plates (diagonal unitary matrices), rotators.

Poincaré sphere: all polarization states map to points on (or inside) the unit sphere. Pure polarizations: on the surface ($\text{DOP}=1$). Unitary polarization transformations = rotations of the sphere.

**Birefringence**: different $n$ for different polarizations. Silicon waveguide: $\Delta n_\text{eff} \approx 0.6$ (TE vs TM) — requires single-polarization design or polarization diversity.

**LiNbO₃ Pockels effect**: $V_\pi L \approx 2$ V·cm (thin-film LiNbO₃). Enables 100+ GHz bandwidth modulators.

## Temporal and Spatial Coherence

**Temporal coherence**: $\gamma(\tau) = \Gamma(\tau)/\Gamma(0)$; coherence time $\tau_c \sim 1/\Delta\nu$; coherence length $L_c = c\tau_c$. The Wiener-Khinchin theorem: spectrum and autocorrelation are a Fourier transform pair.

**Spatial coherence**: $\mu(\mathbf{r}_1, \mathbf{r}_2) = J(\mathbf{r}_1, \mathbf{r}_2)/\sqrt{I_1 I_2}$; spatial coherence length $l_c = \lambda/\theta_s$ (van Cittert-Zernike theorem).

**For photonic computing**: coherent architectures require mutual coherence between all modes (single-source illumination). DFB lasers ($L_c > 300$ m) provide ample coherence for on-chip path differences ($\sim$ mm).

## Gaussian Beams

The lowest-order solution of the paraxial wave equation $\nabla_\perp^2 u + 2ik\partial u/\partial z = 0$:

| Parameter | Formula |
|-----------|---------|
| Rayleigh range | $z_R = \pi w_0^2/\lambda$ |
| Beam radius | $w(z) = w_0\sqrt{1 + (z/z_R)^2}$ |
| Wavefront curvature | $R(z) = z[1 + (z_R/z)^2]$ |
| Far-field divergence | $\theta = \lambda/(\pi w_0)$ |

Uncertainty principle: $w_0\theta = \lambda/\pi$ (constant for all Gaussian beams — diffraction-limited).

ABCD law: $q_\text{out} = (Aq_\text{in}+B)/(Cq_\text{in}+D)$ — same matrices as for geometric rays.

## Coupling Efficiency

Mode overlap: $\eta = |\iint E_\text{beam}E_\text{mode}^* dA|^2/(\|E_\text{beam}\|^2\|E_\text{mode}\|^2)$

Gaussian-to-Gaussian: $\eta = [2w_1 w_2/(w_1^2+w_2^2)]^2 \leq 1$.

Fiber (MFD 10 μm) to silicon waveguide (width 0.45 μm): direct coupling $\approx -22$ dB. Inverse taper or grating coupler: $-0.5$ to $-1.5$ dB. Coupling loss directly limits system power budget.

## Key Numbers

| Quantity | Value |
|----------|-------|
| Fringe visibility (equal intensities) | $V = 1$ |
| Si ring resonator Q (state of art) | $> 10^6$ |
| MZI beam splitter cross-coupling phase | $\pi/2$ (required by unitarity) |
| Coherence length of DFB laser ($\Delta\nu = 1$ MHz) | $\sim 300$ m |
| Coherence length of LED ($\Delta\nu = 5$ THz) | $\sim 60$ μm |
| Gaussian beam uncertainty product | $w_0\theta = \lambda/\pi$ |
| Fiber-to-silicon direct coupling loss | $\sim 22$ dB |
| State-of-art grating coupler efficiency | $> 85\%$ ($< 0.7$ dB) |
| LiNbO₃ Pockels coefficient $r_{33}$ | 30.8 pm/V |
| Si waveguide TE/TM birefringence | $\Delta n_\text{eff} \approx 0.6$ |
