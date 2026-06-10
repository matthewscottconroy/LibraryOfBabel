# Chapter 44: Exercises

---

**44.1.** *Linearized Christoffel symbols and the wave equation.*

Starting from the metric $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ with $|h_{\mu\nu}|\ll 1$:

(a) Show that, to first order in $h$:
$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}\eta^{\rho\sigma}(\partial_\mu h_{\nu\sigma} + \partial_\nu h_{\mu\sigma} - \partial_\sigma h_{\mu\nu})$$

(b) Show that the linearized Ricci tensor is:
$$R_{\mu\nu}^{(1)} = \frac{1}{2}\left(-\Box h_{\mu\nu} + \partial_\mu\partial^\alpha h_{\alpha\nu} + \partial_\nu\partial^\alpha h_{\alpha\mu} - \partial_\mu\partial_\nu h\right)$$

where $h = \eta^{\mu\nu}h_{\mu\nu}$ is the trace and indices are raised with $\eta^{\mu\nu}$.

(c) Define the trace-reversed perturbation $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$. Show that $\bar{h} = \eta^{\mu\nu}\bar{h}_{\mu\nu} = -h$.

(d) Show that in Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$, the linearized Einstein equations $R_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}R = \frac{8\pi G}{c^4}T_{\mu\nu}$ reduce to:
$$\Box\bar{h}_{\mu\nu} = -\frac{16\pi G}{c^4}T_{\mu\nu}$$

This is the derivation that Einstein himself carried out in 1916, from which he first predicted gravitational waves.

---

**44.2.** *Counting degrees of freedom and residual gauge freedom.*

The TT gauge reduces the 10-component symmetric tensor $h_{\mu\nu}$ to exactly 2 physical degrees of freedom. This exercise traces the counting precisely.

(a) The Lorenz gauge condition $\partial^\mu\bar{h}_{\mu\nu} = 0$ imposes 4 constraints on the 10 independent components of $\bar{h}_{\mu\nu}$. Show that this leaves 6 apparent degrees of freedom.

(b) Even within Lorenz gauge, there is residual gauge freedom: we can make coordinate transformations $x^\mu \to x^\mu + \xi^\mu$ as long as $\Box\xi^\mu = 0$. Show that such a transformation changes $h_{\mu\nu} \to h_{\mu\nu} - \partial_\mu\xi_\nu - \partial_\nu\xi_\mu$ and preserves the Lorenz gauge.

(c) The solution $\Box\xi^\mu = 0$ has 4 arbitrary functions (initial data). How does using these 4 functions bring the DOF count from 6 to 2?

(d) For a plane wave propagating in the $z$-direction, write the most general $h_{\mu\nu}$ in Lorenz gauge and explicitly construct the residual gauge transformation that puts it into TT gauge. Show that the resulting TT tensor has exactly the form:
$$h_{\mu\nu}^{\rm TT} = \begin{pmatrix}0&0&0&0\\0&h_+&h_\times&0\\0&h_\times&-h_+&0\\0&0&0&0\end{pmatrix}$$

---

**44.3.** *The TT projection operator.*

For a gravitational wave propagating in the direction $\hat{n}$ (unit vector), the TT part of any symmetric tensor $S_{ij}$ is:
$$S_{ij}^{\rm TT} = \Lambda_{ij,kl}\hat{n}\, S^{kl}$$

where the TT projection operator is:
$$\Lambda_{ij,kl} = P_{ik}P_{jl} - \frac{1}{2}P_{ij}P_{kl}, \quad P_{ij} = \delta_{ij} - n_i n_j$$

(a) Verify that $\Lambda_{ij,kl}$ is symmetric under $i\leftrightarrow j$ and $k\leftrightarrow l$, symmetric under $(ij)\leftrightarrow (kl)$, traceless ($\Lambda^i_{\ i,kl} = 0$), and transverse ($n^i\Lambda_{ij,kl} = 0$).

(b) For propagation in the $z$-direction ($\hat{n} = \hat{z}$), explicitly compute $\Lambda_{ij,kl}$ and verify that it projects the 6-component symmetric spatial tensor down to the 2 components $h_+$ and $h_\times$.

(c) Compute $S_{ij}^{\rm TT}$ for the quadrupole tensor of a binary in a circular orbit in the $xy$-plane, for a wave propagating in the $z$-direction. Verify you recover the two polarization states.

(d) Compute $S_{ij}^{\rm TT}$ for propagation along the $x$-axis (perpendicular to the orbital plane). What polarization states appear? What does this mean geometrically?

---

**44.4.** *Spin-2 polarization and helicity.*

The two polarization tensors for a gravitational wave propagating in the $z$-direction can be combined into circular polarizations:
$$e_{\mu\nu}^R = \frac{1}{\sqrt{2}}\begin{pmatrix}0&0&0&0\\0&1&i&0\\0&i&-1&0\\0&0&0&0\end{pmatrix}, \quad e_{\mu\nu}^L = (e_{\mu\nu}^R)^*$$

(a) Show that under a rotation by angle $\psi$ about the $z$-axis, $e_{\mu\nu}^R \to e^{2i\psi}e_{\mu\nu}^R$ and $e_{\mu\nu}^L \to e^{-2i\psi}e_{\mu\nu}^L$. The factor of $2$ in the exponent is the hallmark of spin-2.

(b) For comparison: a circularly polarized electromagnetic wave propagating in the $z$-direction has polarization vectors $\mathbf{e}^{\pm} = (\hat{x} \pm i\hat{y})/\sqrt{2}$. Show that under a rotation by $\psi$, $\mathbf{e}^{\pm}\to e^{\pm i\psi}\mathbf{e}^{\pm}$. The factor of $1$ reflects spin-1.

(c) A spin-$s$ field returns to itself after a rotation by $2\pi/s$ about the propagation direction. Verify this for $s = 1$ (photon: $\psi = 2\pi$ needed) and $s = 2$ (graviton: $\psi = \pi$ needed). How is this reflected in the angular separation between the two polarization states?

(d) A massless spin-0 field (if it existed as a long-range force mediator) would have no polarization states (helicities $0$ and $0$). What constraint would this place on the radiation from a binary? Compare to the graviton case.

---

**44.5.** *Energy flux from gravitational waves.*

The Isaacson effective stress-energy tensor for gravitational waves is:
$$T_{\mu\nu}^{\rm GW} = \frac{c^4}{32\pi G}\langle\partial_\mu h_{\alpha\beta}^{\rm TT}\partial_\nu h^{\alpha\beta}_{\rm TT}\rangle$$

where $\langle\cdots\rangle$ denotes averaging over several wavelengths (the Isaacson averaging, valid when $\lambda_{\rm GW}\ll$ curvature scale).

(a) For a monochromatic plane wave $h_{ij}^{\rm TT} = A_{ij}\cos(kz - \omega t)$ with amplitude $A$ (meaning $A_{+} = A_+$, $A_\times = A_\times$), compute $T_{00}^{\rm GW}$ and $T_{0z}^{\rm GW}$.

(b) Show that the energy flux (energy per unit area per unit time) is $S = cT_{00}^{\rm GW}$, and compute $S$ in terms of $\omega$, $A_+$, $A_\times$, $G$, and $c$.

(c) For GW150914 at Earth: $h \sim 10^{-21}$ (peak), $f = 150$ Hz, distance $r = 410$ Mpc. Compute the energy flux $S$ at Earth. Compare to the solar irradiance $1361$ W/m² and to the flux from the full moon.

(d) GW150914 radiated $3 M_\odot c^2 = 5.4\times 10^{47}$ J in $\sim 0.1$ s at distance 410 Mpc. Using isotropy (rough approximation), compute the total energy. Does this agree with the flux calculation? (Note: the actual emission is anisotropic, but this gives order-of-magnitude consistency.)

---

## Thought Experiments

**T44.1.** *Are gravitational waves "real"?*

For decades after Einstein's 1916 prediction, there was genuine confusion about whether gravitational waves carried real energy or were merely coordinate artifacts. Einstein himself wavered on this point in the 1930s, briefly co-authoring a paper claiming GWs did not exist (withdrawn after a referee caught an error).

The resolution came in the 1950s–60s through the "sticky bead" argument (Feynman at the 1957 Chapel Hill conference): imagine a bead threaded on a rough rod. A gravitational wave passing through will cause the bead to slide on the rod, generating heat. Since heat is frame-independent, the gravitational wave must have carried real energy.

Consider: What made this argument convincing when purely mathematical arguments about the linearized theory did not settle the question? What is the difference between a coordinate artifact and a physical effect? Can you think of an analogous historical confusion in electromagnetism?

**T44.2.** *What does LIGO actually detect?*

A common misconception is that LIGO's mirrors move when a gravitational wave passes through. In a free-fall (locally inertial) frame attached to one mirror, the other mirror appears to accelerate. But in the TT gauge, the mirror coordinates don't change — instead, the proper length between them oscillates.

Which description is "right"? The proper length in TT gauge changes: $\delta L = \frac{1}{2}h_+L$. The acceleration of one mirror relative to the other in the local Lorentz frame of the first is $\ddot{\xi} = \frac{c^2}{2}\ddot{h}_+L$. Both descriptions give the same observable: the phase difference at the beamsplitter. 

What does this tell you about the gauge-dependence of descriptions versus the gauge-independence of observables? Can you construct an argument that the phase difference at the beamsplitter is the fundamental gauge-independent observable?

**T44.3.** *Gravitational wave memory.*

A gravitational wave pulse (a transient, not a monochromatic wave) leaves a permanent displacement between free test masses after it passes — the "gravitational wave memory effect" (Zel'dovich, Braginsky, 1974). Unlike the oscillatory part of the wave, the memory is a DC displacement.

The memory comes from the nonlinear terms in the Einstein equations (the "Christodoulou memory") and from changes in the system's energy-momentum configuration. It is not present in linear theory but appears at second order.

Why is the memory a physical (gauge-invariant) effect? If memory exists, what would it do to LIGO's mirrors after a very strong gravitational wave passes? Could LIGO detect the memory effect from GW150914? (The answer is yes, in principle, but it would require $\sim 100\times$ current sensitivity for a single event.)
