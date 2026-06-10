# Section 48.1: The $\Lambda$CDM Model and Its Observational Pillars

---

## What $\Lambda$CDM Is

The standard cosmological model, $\Lambda$CDM, is built on six assumptions:

1. General relativity governs the large-scale dynamics of spacetime
2. The universe is spatially homogeneous and isotropic on large scales (FLRW metric)
3. The universe contains cold dark matter (CDM) — non-relativistic, non-baryonic matter that interacts only gravitationally (and possibly weakly)
4. The universe contains a cosmological constant $\Lambda$ (dark energy with $w = -1$)
5. Primordial perturbations were seeded by a process that produced an approximately scale-invariant spectrum (inflation)
6. Baryons, photons, and neutrinos follow standard physics

From these assumptions and six free parameters ($H_0$, $\Omega_b h^2$, $\Omega_{\rm DM}h^2$, $A_s$, $n_s$, $\tau$), the model makes predictions for observables spanning twelve orders of magnitude in scale. The agreement with observations is essentially perfect.

This section surveys the four main observational pillars.

---

## Pillar I: The Cosmic Microwave Background

**The observation.** The CMB is thermal radiation with a perfect blackbody spectrum at $T = 2.72548 \pm 0.00057$ K, with temperature anisotropies $\Delta T/T \sim 10^{-5}$ that encode the state of the universe at $z \approx 1100$.

**The physics.** At $z \approx 1100$, the temperature fell below $\sim 3000$ K, neutral hydrogen formed (recombination), and the universe became transparent. Photons decoupled from the baryon-photon fluid and free-streamed to us as the CMB. The anisotropies we observe are the imprints of primordial density perturbations, processed by:
- Acoustic oscillations in the photon-baryon fluid (before recombination)
- Diffusion damping (Silk damping) on small scales
- Sachs-Wolfe effect (potential wells imprint on photon temperature)
- Integrated Sachs-Wolfe effect (photons gain/lose energy traversing evolving potential wells)
- Gravitational lensing by intervening structure

**The angular power spectrum.** The CMB temperature field is decomposed in spherical harmonics: $\Delta T(\hat{n})/T = \sum_{\ell m} a_{\ell m}Y_{\ell m}(\hat{n})$. The power spectrum $C_\ell = \langle|a_{\ell m}|^2\rangle$ has characteristic acoustic peaks at $\ell_n \approx n\pi d_A(z_*)/r_s$:
- First peak: $\ell_1 \approx 220$ — corresponds to the scale that had undergone exactly one half-oscillation at recombination
- Second peak: $\ell_2 \approx 540$ — one full oscillation; amplitude sensitive to baryon density (odd peaks are enhanced by baryons, even peaks suppressed)
- Third peak: $\ell_3 \approx 820$ — sensitive to dark matter density
- Higher peaks: damped by Silk diffusion at $\ell > 1000$

**Planck results.** The Planck satellite (ESA, 2009–2018) measured the CMB to the cosmic variance limit for $\ell \leq 2000$ — meaning the fundamental statistical limit set by having only one sky. Planck confirmed: (1) a spatially flat universe, (2) $\Omega_\Lambda = 0.685 \pm 0.007$, (3) a Harrison-Zel'dovich-like spectrum ($n_s = 0.965 \pm 0.004$, slightly red-tilted), (4) no evidence for primordial gravitational waves at current sensitivity.

**CMB polarization.** The CMB is also polarized at the $\sim 10\%$ level from Thomson scattering. E-mode polarization correlates with temperature anisotropies; B-mode polarization would signal primordial gravitational waves from inflation. The tensor-to-scalar ratio $r < 0.036$ (Bicep/Keck 2021) constrains inflationary models.

---

## Pillar II: Big Bang Nucleosynthesis

**The observation.** The primordial abundances of light elements (H, D, $^3$He, $^4$He, $^7$Li) can be measured in old, metal-poor astrophysical environments. Observed values:
- $^4$He mass fraction: $Y_p = 0.245 \pm 0.003$
- Deuterium abundance: D/H $= (2.527\pm 0.030)\times 10^{-5}$ (from QSO absorption systems)
- $^7$Li: $^7$Li/H $\approx 1.6\times 10^{-10}$ (observed in halo stars; a factor of $\sim 3$ below BBN prediction — the "lithium problem")

**The physics.** At $T \sim 10^{10}$ K ($t \sim 1$ s), the universe was a hot dense plasma of baryons, leptons, and photons in thermal equilibrium. As it cooled:
- $T \sim 10^{10}$ K: neutrinos decouple; $n/p$ ratio freezes out at $n/p \approx 1/7$
- $T \sim 10^9$ K ($t \sim 3$ min): nuclear reactions begin forming deuterium, helium
- $T \sim 3\times 10^8$ K: reactions freeze out; $^4$He mass fraction $\approx 25\%$, trace D, $^3$He, $^7$Li

The $^4$He abundance depends on the expansion rate (faster expansion → higher $n/p$ freeze-out → more $^4$He) and on the number of neutrino species $N_\nu$. Deuterium is extremely sensitive to the baryon density: higher $\eta_b = n_b/n_\gamma$ burns more D into $^4$He.

**The constraint.** The D/H measurement gives $\Omega_b h^2 = 0.0222 \pm 0.0003$ — consistent with the CMB value of $\Omega_b h^2 = 0.02237 \pm 0.00015$. The agreement between BBN (probing $t \sim 1$–$300$ s) and CMB (probing $t \sim 380,000$ yr) is a powerful consistency check spanning six decades in cosmic time.

The **lithium problem**: BBN predicts $^7$Li/H $\approx 4.7\times 10^{-10}$, while stellar observations give $\approx 1.6\times 10^{-10}$ — a factor of $\sim 3$ discrepancy. This may be systematic error (stellar depletion of Li in halo stars) rather than new physics, but remains unresolved.

---

## Pillar III: Baryon Acoustic Oscillations

**The observation.** Galaxy surveys (SDSS, BOSS, eBOSS, DESI) measure the 3D distribution of galaxies. The two-point correlation function $\xi(r)$ shows a characteristic bump at $r \approx 150$ Mpc — the baryon acoustic oscillation peak. This scale corresponds to the comoving sound horizon at recombination, $r_s \approx 147$ Mpc.

**The physics.** Before recombination, baryons and photons were tightly coupled in a fluid that supported acoustic oscillations. The preferred scale — the "sound horizon" $r_s = \int_0^{t_*} c_s(t)/a\,dt$ where $c_s = c/\sqrt{3(1+3\rho_b/4\rho_\gamma)}$ is the sound speed — is a known physical scale. When recombination occurs, the oscillation freezes and imprints a preferred scale on the distribution of matter.

**As a standard ruler.** The BAO peak at $r_s \approx 147$ Mpc appears at angle $\theta_*$ in the CMB and at separation $r_s/d_A(z_*)$ in projection on the sky and $r_s H(z)/c$ in the radial direction of galaxy surveys. Measuring these angles and redshifts constrains $d_A(z)$ and $H(z)$ at multiple redshifts. Combined with CMB, BAO provides the most precise constraints on dark energy.

**DESI 2024.** The Dark Energy Spectroscopic Instrument measured BAO with 6 million galaxies across $0.1 < z < 4.2$. Results were consistent with $\Lambda$CDM but showed a $2.6\sigma$ preference for $w_0 > -1$, $w_a < 0$ (if combined with CMB and SNe Ia) — a possible hint of dynamic dark energy.

---

## Pillar IV: Supernovae and the Accelerating Expansion

**The observation.** Type Ia supernovae are produced when a white dwarf in a binary system accretes mass past the Chandrasekhar limit and detonates. They are "standardizable candles" — their peak luminosity correlates with their light curve width (the Phillips relation), allowing distance measurements to $\sim 5\%$ per supernova.

**The 1998 discovery.** Two teams (Perlmutter et al. — Supernova Cosmology Project; Riess et al. — High-Z Supernova Search Team) measured SNe Ia at $z \sim 0.5$–$1$ and found them $\sim 25$% dimmer (further) than expected in a matter-dominated universe with $\Omega_m = 1$, $\Omega_\Lambda = 0$. The data fit perfectly with $\Omega_m \approx 0.3$, $\Omega_\Lambda \approx 0.7$. Conclusion: the universe is not decelerating but **accelerating**. This requires a component with $w < -1/3$ — dark energy.

**Current constraints.** The Union2.1 compilation (580 SNe Ia), Pantheon+ (1701 SNe Ia through $z = 2.3$), and DES-SN5YR (5-year Dark Energy Survey) all confirm the accelerating expansion. Combined with CMB and BAO: $w = -1.03 \pm 0.04$ — consistent with a cosmological constant.

---

## The Matter Power Spectrum and Structure Formation

The CMB anisotropies seed structure. After recombination, matter perturbations grow by gravitational instability. The growth factor satisfies:
$$\ddot{\delta}_m + 2H\dot{\delta}_m = \frac{4\pi G\rho_m\delta_m}{a^3}$$

In matter domination: $\delta_m \propto a \propto t^{2/3}$ (the growing mode). During $\Lambda$ domination, growth is suppressed as the Hubble friction term dominates.

The matter power spectrum $P(k) = \langle|\delta_k|^2\rangle$ at late times retains the shape of the primordial spectrum $P_{\rm prim}(k) \propto k^{n_s}$ modified by the **transfer function** $T(k)$ that encodes physics at and after matter-radiation equality:
$$P(k,z) = P_{\rm prim}(k) T^2(k) D^2(z)$$

where $D(z)$ is the growth factor.

On scales larger than the Hubble radius at matter-radiation equality ($k < k_{\rm eq} \approx 0.01$ Mpc$^{-1}$): $T(k) \approx 1$ — unaffected. On smaller scales ($k > k_{\rm eq}$): power is suppressed as $T(k) \propto k^{-2}$ (modes that entered the horizon during radiation domination had their growth suppressed). The peak of $P(k)$ at $k \approx k_{\rm eq}$ is a sensitive measure of $\Omega_m h$.

The matter power spectrum is now measured with $\sim 1\%$ precision by galaxy surveys (SDSS, DESI) for $0.01 < k < 0.3$ Mpc$^{-1}$ — the large-scale structure of the universe at exquisite precision.

---

## Dark Matter: Evidence and Candidates

The evidence for dark matter is multiply independent and observationally overwhelming:

1. **Galaxy rotation curves** (Rubin, Ford 1970s): flat $v(r)$ requires $M(r) \propto r$, i.e., a dark halo
2. **Cluster dynamics** (Zwicky 1933): velocity dispersions in clusters imply mass $\sim 100\times$ the visible mass
3. **Gravitational lensing**: strong lensing (Einstein rings, arcs) and weak lensing (shear correlations) both measure mass independently of luminosity
4. **CMB anisotropies**: the ratio of odd to even acoustic peaks constrains baryon vs. dark matter density
5. **The Bullet Cluster** (Clowe et al. 2006): post-collision cluster shows dark matter (via lensing) separated from gas (via X-rays), ruling out simple MOND
6. **Structure formation**: without CDM, baryons can't start collapsing until recombination — too late to form observed structure; CDM halos collapse earlier and attract baryons

Dark matter candidates:

**WIMPs (Weakly Interacting Massive Particles)**: Mass $\sim 10$ GeV–$10$ TeV; annihilation cross-section $\sim$ weak scale. The "WIMP miracle": a particle with weak-scale mass and coupling automatically freezes out with $\Omega_{\rm DM} \sim 0.25$. Many SUSY models predict WIMPs. But direct detection (LUX, XENON, PandaX) has found nothing down to spin-independent cross-sections $\sigma_{\rm SI} < 10^{-47}$ cm² for $m_\chi = 30$ GeV — dramatically below the naive WIMP expectation.

**Axions**: Originally proposed by Peccei and Quinn to solve the strong CP problem. Mass $m_a \sim 10^{-6}$–$10^{-3}$ eV. Form a Bose-Einstein condensate in the early universe. Detected via their coupling to photons in strong magnetic fields. The ADMX experiment searches for axion dark matter.

**Primordial black holes (PBHs)**: Black holes formed in the early universe. Severely constrained by microlensing (EROS, OGLE, MACHO), CMB spectral distortions, and GW observations. Can account for a small fraction ($<1\%$) of dark matter.

**Fuzzy dark matter**: Ultra-light axion-like particles with $m \sim 10^{-22}$ eV and de Broglie wavelength $\sim$ kpc. Suppresses small-scale structure (solving the "too big to fail" and "missing satellites" problems). But constraints from Lyman-$\alpha$ forest suggest $m > 2\times 10^{-21}$ eV.

---

## Dark Energy: Evidence and Interpretations

Dark energy — whatever is causing the accelerating expansion — has $w \approx -1$. The leading interpretations:

**Cosmological constant $\Lambda$**: Vacuum energy in GR. But the theoretical prediction ($\sim M_{\rm Pl}^4$) exceeds the observed value by $10^{120}$ — the cosmological constant problem. Some argue that quantum corrections cancel to extraordinary precision (fine-tuning), or that the landscape of string vacua anthropically selects small $\Lambda$ (Weinberg 1987).

**Quintessence**: A rolling scalar field $\phi$ with potential $V(\phi)$, giving $w = (K - V)/(K + V)$ where $K$ is kinetic energy density. For slow-roll $K \ll V$: $w \approx -1$. Different potentials predict different time evolution of $w(z)$. Not yet distinguishable from $\Lambda$ with current data.

**Modified gravity**: Perhaps GR fails on cosmological scales. $f(R)$ gravity, DGP braneworld models, scalar-tensor theories. All predict deviations in the growth of structure that are testable with weak lensing surveys.

**Phantom energy**: $w < -1$, which leads to a "Big Rip" where the Hubble parameter diverges in finite time, tearing apart all bound structures. Current constraint $w = -1.03 \pm 0.04$ allows this, though it implies violation of the null energy condition.

The fundamental mystery of dark energy — why is $\Lambda$ so small but nonzero? — remains entirely open. It may be the deepest unsolved problem in fundamental physics.
