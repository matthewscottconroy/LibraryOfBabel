# 18.3.1 Optical Parametric Amplification and Down-Conversion

## Photons Made in Pairs

A second-order nonlinear crystal ($\chi^{(2)}$ medium — BBO, KTP, lithium niobate) mediates a three-wave interaction in which one **pump** photon is annihilated and two photons, historically called **signal** and **idler**, are created. Energy and momentum are conserved:

$$\omega_p = \omega_s + \omega_i, \qquad \mathbf{k}_p = \mathbf{k}_s + \mathbf{k}_i.$$

The first condition (energy) fixes the down-converted frequencies; the second (**phase matching**) is the demand that the three waves stay in step over the crystal length, achieved either by birefringence — angling the crystal so the polarization-split refractive indices compensate dispersion — or by **quasi-phase-matching**, periodically poling the crystal ($\chi^{(2)}$ sign-flipped every coherence length) as in PPKTP and PPLN. Without phase matching the pair-generation amplitude oscillates and averages to nothing; with it, the amplitude accumulates coherently along the crystal.

Treating the bright pump as an undepleted classical field of amplitude $\propto e^{-i\omega_p t}$, the interaction Hamiltonian reduces to

$$\hat{H}_{\text{int}} = i\hbar\big(\eta^*\,\hat{a}_s\hat{a}_i - \eta\,\hat{a}_s^\dagger\hat{a}_i^\dagger\big),$$

with $\eta \propto \chi^{(2)}E_p$. Integrating over the interaction time gives precisely the **two-mode squeezing operator** of Section 17.3.3,

$$\hat{S}_2(\xi) = \exp\!\big(\xi^*\,\hat{a}_s\hat{a}_i - \xi\,\hat{a}_s^\dagger\hat{a}_i^\dagger\big), \qquad \xi = re^{i\theta},$$

whose tell-tale structure is the term $\hat{a}_s^\dagger\hat{a}_i^\dagger$: it creates signal and idler photons *in pairs*. Parametric down-conversion is the hardware realization of two-mode squeezing.

## The State: Two-Mode Squeezed Vacuum

Applied to vacuum, $\hat{S}_2(\xi)$ builds the **two-mode squeezed vacuum** (Section 17.3.3):

$$|\psi\rangle = \hat{S}_2(\xi)|0,0\rangle = \frac{1}{\cosh r}\sum_{n=0}^{\infty} \tanh^n\!r\;|n,n\rangle,$$

(absorbing the phase into the mode definitions). Photon numbers in the two arms are **perfectly correlated** — the state is a superposition of $|0,0\rangle, |1,1\rangle, |2,2\rangle,\dots$, never $|n,m\rangle$ with $n\neq m$. The mean photon number per mode is $\langle\hat{n}\rangle = \sinh^2 r$, and each arm *alone* is a thermal state: maximally noisy marginals concealing perfect joint correlations, the continuous-variable face of entanglement. In the weak-pump regime ($r\ll 1$) the state is dominated by $|0,0\rangle$ with a small $|1,1\rangle$ admixture — a probabilistic source of **photon pairs**.

## Heralding and Entanglement

Two uses follow immediately. First, **heralding**: detect the idler and you know — up to loss — that a signal photon exists in its conjugate mode, converting the probabilistic pair source into a *heralded single-photon source*. This is the SPDC route to single photons that dominated quantum optics before quantum dots (Chapter 19), limited fundamentally by the multi-pair statistics computed below.

Second, **entanglement**. The polarization structure depends on the phase-matching type:

- **Type-I**: signal and idler share one polarization (both ordinary or both extraordinary), emitted into a cone. Two crossed type-I crystals pumped at $45^\circ$ give polarization entanglement.
- **Type-II**: signal and idler have *orthogonal* polarizations, emitted into two cones. Along the two directions where the cones intersect, the photon pair is polarization-entangled in a Bell state,

$$|\psi\rangle = \frac{1}{\sqrt2}\big(|H\rangle_1|V\rangle_2 + e^{i\phi}|V\rangle_1|H\rangle_2\big),$$

with local wave plates and birefringent compensation selecting any of the four Bell states. This is the Kwiat-Mattle-Weinfurter-Zeilinger-Sergienko-Shih source of 1995 (Kwiat et al., 1995), the workhorse entangled-pair source for three decades of Bell tests, quantum teleportation, and entanglement-based QKD (Chapter 22). A modern PPKTP source delivers on the order of $10^6$ entangled pairs per second at milliwatt-scale pump power.

**Worked example.** *Pair statistics, multi-pair contamination, and heralded purity.*

The pair-number distribution is geometric (thermal),

$$P_n = \frac{\tanh^{2n}\!r}{\cosh^2\!r} = (1-\lambda)\,\lambda^n, \qquad \lambda \equiv \tanh^2\!r = \frac{\mu}{1+\mu},$$

where $\mu = \langle n\rangle = \sinh^2 r$ is the mean pair number per pump pulse. The probabilities of zero, one, and two pairs are $P_0 = 1-\lambda$, $P_1 = \lambda(1-\lambda)$, $P_2 = \lambda^2(1-\lambda)$, so the ratio of double-pair to single-pair emission is

$$\frac{P_2}{P_1} = \lambda = \frac{\mu}{1+\mu} \approx \mu \quad (\mu\ll1).$$

Take $\mu = 0.1$ pairs per pulse: then $\lambda = 0.091$, so **9% of the "single-pair" heralds are secretly double-pair events** — two signal photons masquerading as one. This contamination is exactly what a heralded HOM measurement (Section 18.2.2) exposes. Quantitatively, threshold-heralding the idler gives a heralded signal purity

$$g^{(2)}_h(0) = \frac{2\mu}{1+\mu} = \frac{2(0.1)}{1.1} = 0.18,$$

far too high for quantum computing. Demanding $g^{(2)}_h(0) < 0.01$ forces $\mu \lesssim 0.005$ — at which brightness a 1 GHz pump delivers only $\sim 5\times10^{6}$ heralds/s before the heralding efficiency ($\sim 0.8$) and downstream loss are applied. This is the fundamental **brightness-purity trade-off** of every parametric source: purity demands a dim pump, because the pair statistics are thermal and the two-pair rate grows as $\mu^2$. Photon-number-resolving heralds and group-velocity-matched (spectrally factorable) crystals push against the trade-off, but never abolish it (Chapter 19).

## Spectral Purity: The Second Hidden Cost

Multi-pair emission is not the only imperfection of an SPDC source; **spectral correlation** is the other. Energy conservation $\omega_s+\omega_i=\omega_p$ correlates the signal and idler frequencies, so the pair's joint spectral amplitude — the product of the pump envelope and the crystal's phase-matching function — is generally *entangled* in frequency. That is a resource for energy-time entanglement, but a liability for heralding: detecting the idler collapses the signal into a *mixed* spectral state, and a mixed photon has reduced HOM visibility with any other photon. A Schmidt decomposition of the joint spectral amplitude quantifies this through the spectral purity $P=\sum_k\lambda_k^2$ (unity for a factorable, unentangled spectrum). Achieving $P\to1$ without lossy spectral filtering requires **group-velocity matching** — engineering the crystal and pump bandwidth so the phase-matching function is oriented to factorize — the design target of sources feeding indistinguishable photons into the circuits of Chapters 19 and 20.

## Why It Matters

Spontaneous parametric down-conversion is the most-used non-classical light source in the world. It heralds single photons, generates the entangled pairs behind Bell tests and quantum communication (Chapter 22), and is the direct physical embodiment of the two-mode squeezing operator — pair creation as a laboratory fact. Its thermal pair statistics set the purity ceiling that motivated the deterministic quantum-dot sources of Chapter 19, and its degenerate limit, where signal and idler collapse into one mode, produces the single-mode squeezed light of the next subsection.
