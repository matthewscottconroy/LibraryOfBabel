# Section 44.1: Gravitational Waves from Linearized GR

---

## Linearizing the Einstein Equations

Write the metric as a small perturbation of Minkowski spacetime:
$$g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}, \quad |h_{\mu\nu}| \ll 1$$

The Christoffel symbols, Riemann tensor, and Ricci tensor are all linear in $h_{\mu\nu}$ at lowest order:
$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}\eta^{\rho\sigma}(\partial_\mu h_{\nu\sigma} + \partial_\nu h_{\mu\sigma} - \partial_\sigma h_{\mu\nu}) + O(h^2)$$

$$R_{\mu\nu} = \frac{1}{2}\left(-\Box h_{\mu\nu} + \partial_\mu\partial^\alpha h_{\alpha\nu} + \partial_\nu\partial^\alpha h_{\alpha\mu} - \partial_\mu\partial_\nu h\right) + O(h^2)$$

where $\Box = \eta^{\mu\nu}\partial_\mu\partial_\nu = -\partial_t^2/c^2 + \nabla^2$ and $h = \eta^{\mu\nu}h_{\mu\nu}$ is the trace.

**Trace-reversed perturbation:** Define $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ (so $\bar{h} = -h$). In Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$, the linearized Einstein equations become:
$$\Box\bar{h}_{\mu\nu} = -\frac{16\pi G}{c^4}T_{\mu\nu}$$

This is the wave equation for the gravitational perturbation, sourced by the stress-energy tensor. The operator $\Box = -\partial_t^2/c^2 + \nabla^2$ gives propagation at speed $c$.

---

## Plane Wave Solutions

In vacuum ($T_{\mu\nu} = 0$), the equation $\Box\bar{h}_{\mu\nu} = 0$ has plane wave solutions:
$$\bar{h}_{\mu\nu} = \text{Re}\left(A_{\mu\nu}e^{ik_\rho x^\rho}\right)$$

where $k^\mu = (\omega/c, \mathbf{k})$ is the null wave 4-vector ($k_\mu k^\mu = 0$, i.e., $\omega = c|\mathbf{k}|$). The dispersion relation $\omega = c|\mathbf{k}|$ means gravitational waves travel at speed $c$.

**Counting polarizations.** The matrix $A_{\mu\nu}$ has 10 components (symmetric $4\times4$). Lorenz gauge $k^\mu A_{\mu\nu} = 0$ eliminates 4. Residual gauge freedom eliminates 4 more (choosing the TT gauge). Tracelessness $A^\mu_{\ \mu} = 0$ removes 1. Result: **2 independent polarization states** (the $+$ and $\times$ polarizations), just like electromagnetic waves.

---

## Transverse-Traceless (TT) Gauge

For a gravitational wave propagating in the $z$-direction ($k^\mu = (\omega/c)(1,0,0,1)$), the transverse-traceless (TT) gauge has:
$$h_{\mu\nu}^{\rm TT} = \begin{pmatrix}0 & 0 & 0 & 0\\ 0 & h_+ & h_\times & 0\\ 0 & h_\times & -h_+ & 0\\ 0 & 0 & 0 & 0\end{pmatrix}e^{i\omega(z/c-t)}$$

**Properties of TT gauge:**
- $h_{0\mu}^{\rm TT} = 0$: no time components
- $\partial^i h_{ij}^{\rm TT} = 0$: transverse (wavevector $k^i$ contracts to zero)
- $h^i_{i,\rm TT} = 0$: traceless

The two polarization states are:
- **Plus polarization** ($h_+$): stretches the $x$-direction and squeezes the $y$-direction (or vice versa)
- **Cross polarization** ($h_\times$): stretches the $x+y$ direction and squeezes $x-y$

The cross polarization is the plus polarization rotated by $45°$.

---

## Effect on Test Masses: The Geodesic Deviation Equation

For two test masses separated by the displacement vector $\xi^i$ (spatial separation, both at rest initially), in a gravitational wave background:
$$\ddot{\xi}^i = -R^i_{\ 0j0}\xi^j c^2 = \frac{c^2}{2}\ddot{h}_{ij}^{\rm TT}\xi^j$$

For the $+$ polarization wave propagating in $z$-direction:
$$\ddot{\xi}^x = \frac{c^2}{2}\ddot{h}_+\xi^x, \quad \ddot{\xi}^y = -\frac{c^2}{2}\ddot{h}_+\xi^y$$

The $+$ polarization stretches the $x$-separation while compressing the $y$-separation (and vice versa half a cycle later), with peak displacement:
$$\delta\xi^x = \frac{1}{2}h_+\xi^x$$

For LIGO arm length $L = 4$ km and strain $h_+ \sim 10^{-21}$:
$$\delta L = \frac{1}{2}h_+ L = \frac{1}{2}\times 10^{-21}\times 4\times 10^3 = 2\times 10^{-18}\text{ m}$$

This is about $1/500$ the diameter of a proton.

---

## Polarization States and the Graviton Spin

The two polarization states of gravitational waves are at $45°$ to each other (unlike EM, where the two polarizations are at $90°$). Under a rotation by angle $\psi$ about the propagation direction:
$$h_+\to h_+\cos 2\psi - h_\times\sin 2\psi$$
$$h_\times\to h_+\sin 2\psi + h_\times\cos 2\psi$$

The polarization patterns rotate by **twice** the angle of rotation — this is the signature of a spin-2 field. A spin-$s$ field's polarization patterns transform under rotation by $s$ times the rotation angle. For spin-1 (photon), the patterns rotate by $1\times$ the angle; for spin-2 (graviton), by $2\times$.

This is directly observable: the $+$ and $\times$ patterns are $45°$ apart (not $90°$ as for EM). The graviton is a massless spin-2 boson.

---

## The Quadrupole Formula

The retarded solution to $\Box\bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}/c^4$ gives, at large distance $r$ from the source (the "far-field" or "radiation zone"):

$$\bar{h}_{ij}^{\rm TT}(t,\mathbf{x}) = \frac{2G}{c^4 r}\Lambda_{ij,kl}\,\ddot{Q}^{kl}(t_{\rm ret})$$

where:
- $t_{\rm ret} = t - r/c$ is the retarded time
- $Q^{ij} = \int T^{00}(t,\mathbf{x}')\left(x'^i x'^j - \frac{1}{3}\delta^{ij}r'^2\right)\frac{d^3x'}{c^2}$ is the mass quadrupole moment
- $\Lambda_{ij,kl}$ is the TT projection operator

The gravitational wave strain (at distance $r$) is:
$$h \sim \frac{2G}{c^4 r}\ddot{Q}$$

The power radiated in gravitational waves (the quadrupole formula):
$$P = \frac{G}{5c^5}\dddot{Q}_{ij}\dddot{Q}^{ij} = -\frac{dE_{\rm orbital}}{dt}$$

This is the leading-order (quadrupole) formula. There is no monopole or dipole radiation from gravity:
- **No monopole radiation**: $\dddot{M} = \dddot{\int\rho\,d^3x} = 0$ (mass conservation)
- **No dipole radiation**: $\dddot{P}^i = \dddot{\int\rho v^i\,d^3x} = 0$ (momentum conservation)
- First non-vanishing contribution: mass **quadrupole** $Q^{ij}$

This is fundamentally different from EM: the dominant EM radiation is dipole (from accelerating charges), while the dominant gravitational radiation is quadrupole (from accelerating mass distributions).

---

## Example: Binary System

Two masses $m_1$ and $m_2$ in circular orbit with orbital frequency $\Omega$ and separation $a$ have quadrupole moment:
$$Q^{ij} = \mu a^2\begin{pmatrix}\cos^2\Omega t & \cos\Omega t\sin\Omega t & 0\\ \cos\Omega t\sin\Omega t & \sin^2\Omega t & 0\\ 0 & 0 & 0\end{pmatrix}$$

where $\mu = m_1 m_2/(m_1+m_2)$ is the reduced mass.

$\dddot{Q}_{ij}$ has components oscillating at $2\Omega$ (not $\Omega$). The gravitational wave frequency is **twice** the orbital frequency: $f_{\rm GW} = 2f_{\rm orbital}$.

The radiated power is:
$$P_{\rm GW} = \frac{32G^4}{5c^5}\frac{m_1^2 m_2^2(m_1+m_2)}{a^5}$$

For the Hulse-Taylor binary pulsar (PSR B1913+16), this formula predicts an orbital decay rate $\dot{P}_b = -2.40\times 10^{-12}$ s/s. The observed value (over 40 years of timing) is $-2.423\times 10^{-12}$ s/s — agreement to **0.1%** and the first indirect evidence for gravitational waves.

---

## Energy Carried by Gravitational Waves

The stress-energy of gravitational waves is carried by an effective stress-energy tensor at second order in $h$:
$$T_{\mu\nu}^{\rm GW} = \frac{c^4}{32\pi G}\langle\partial_\mu h_{\alpha\beta}\partial_\nu h^{\alpha\beta}\rangle$$

where $\langle\cdots\rangle$ denotes averaging over several wavelengths. For a plane wave with amplitude $h$:
$$T_{00}^{\rm GW} = \frac{c^2\omega^2}{32\pi G}\langle h_+^2 + h_\times^2\rangle$$

The energy flux (energy per unit area per unit time):
$$S = cT_{00}^{\rm GW} = \frac{c^3\omega^2}{32\pi G}h^2$$

For GW150914 at Earth ($r = 410$ Mpc): with $h \sim 10^{-21}$ and $f \sim 150$ Hz:
$$S \approx \frac{(3\times10^8)(2\pi\times150)^2}{32\pi\times 6.67\times10^{-11}}(10^{-21})^2 \approx 2\times 10^{-4}\text{ W/m}^2$$

Far less than sunlight ($\sim 1360$ W/m$^2$) but radiated by the most energetic event (briefly) in the observable universe.

