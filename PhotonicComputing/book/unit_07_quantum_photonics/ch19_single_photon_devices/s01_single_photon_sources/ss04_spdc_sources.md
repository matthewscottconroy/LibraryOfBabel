# 19.1.4 SPDC Sources: Heralding and Multiplexing

## From Squeezing to Single Photons

Chapter 18 introduced spontaneous parametric down-conversion (SPDC) as the physics of the optical parametric amplifier: a pump photon at $\omega_p$ splits in a $\chi^{(2)}$ crystal into signal and idler photons with $\omega_s + \omega_i = \omega_p$, subject to phase matching $\mathbf{k}_p = \mathbf{k}_s + \mathbf{k}_i$. The output of a single-mode-pair SPDC process is the two-mode squeezed vacuum,

$$|\psi\rangle = \sqrt{1-\lambda^2}\sum_{n=0}^{\infty} \lambda^n |n\rangle_s |n\rangle_i, \qquad \lambda = \tanh r,$$

with mean pair number per pulse $\mu = \sinh^2 r$. The photons come in *pairs*, and pair emission is *random* — the state contains vacuum (mostly), one pair (sometimes), and two or more pairs (rarely but fatally).

Two facts turn this into a single-photon source:

1. **Heralding.** Detecting the idler photon projects the signal mode onto (approximately) a one-photon state. SPDC is thus a *heralded* source: it does not fire on demand, but it announces when it has fired. Third-order nonlinear media work identically via spontaneous four-wave mixing (SFWM, $2\omega_p \to \omega_s + \omega_i$), which is how photon-pair sources are built in silicon and Si₃N₄ microrings — the source technology of choice for chip-scale quantum photonics, including PsiQuantum's architecture.
2. **Multiplexing.** Running many probabilistic sources in parallel (or one source across many time bins) and routing whichever one fired to a common output converts "sometimes" into "almost always."

## The Brightness-Purity Trade-off, Quantitatively

Herald with an ideal click (threshold) detector on the idler. The signal state conditioned on a click is $\rho_s \propto \sum_{n\ge1} p_n |n\rangle\langle n|$ where $p_n = \mu^n/(1+\mu)^{n+1}$ is the thermal pair-number distribution. Computing the heralded second-order correlation gives

$$g^{(2)}_h(0) = \frac{\langle n(n-1)\rangle}{\langle n \rangle^2} = \frac{2\mu}{1+\mu} \approx 2\mu \quad (\mu \ll 1).$$

The requirement $g^{(2)}_h(0) < 0.01$ therefore forces $\mu \lesssim 0.005$: the source may fire on at most half a percent of pulses. This is the fundamental brightness-purity trade-off of all parametric sources. (Multi-Schmidt-mode emission relaxes the unheralded statistics toward Poissonian but does not rescue the heralded purity; photon-number-resolving heralds, which can veto multi-pair events, do — one reason PNR detectors matter, Section 19.2.3.)

Two more figures of merit are specific to heralded sources:

- **Heralding efficiency (Klyshko efficiency)** $\eta_H$: the probability that a signal photon is actually present in the collection mode given an idler click, $\eta_H = C/(N_i)$ in terms of coincidence and idler singles rates. Bulk PPKTP sources engineered for the 2015 loophole-free Bell tests reached $\eta_H \sim 0.75$–0.9; waveguide and microring sources are catching up. Every percent of heralding inefficiency is photon loss injected directly into the quantum circuit.
- **Spectral purity** $P = \mathrm{Tr}(\rho_s^2)$ of the heralded photon: energy conservation and phase matching generally *entangle* signal and idler spectra (the joint spectral amplitude $f(\omega_s,\omega_i)$ is not factorable), so heralding leaves the signal in a mixed spectral state, destroying HOM interference between independent sources. The fix is **group-velocity matching**: in KTP pumped near 775 nm, the phase-matching function can be oriented so that $f(\omega_s,\omega_i) \approx g(\omega_s)h(\omega_i)$, giving heralded-state purities >0.99 (with apodized, "Gaussian-engineered" poling) *without lossy filtering*. This is why telecom-band PPKTP and PPLN sources dominate multiphoton interference experiments.

## Why SPDC Persists Despite the Trade-off

Room-temperature operation, any phase-matchable wavelength (including 1550 nm), pump-tunable brightness, near-perfect indistinguishability by engineering rather than by luck, and — via the same interaction at higher gain — squeezed light and entangled pairs (type-II SPDC yields polarization Bell states, Kwiat et al., 1995). SPDC/SFWM sources are also the only ones today that are manufacturable in a CMOS photonics foundry: a microring pumped by a telecom laser, with no epitaxy, no single-emitter placement, and no spectral inhomogeneity. The entire cost is the probabilistic firing — which multiplexing addresses.

## Multiplexed (Quasi-Deterministic) Single-Photon Sources

Run $N$ heralded sources in parallel, each firing with probability $p_1 = \mu/(1+\mu)^2 \cdot \eta_H$ per pulse (kept small for purity). The probability that *at least one* fires is

$$P_N = 1 - (1 - p_1)^N,$$

and an $\log_2 N$-deep switch network routes the heralded photon to the output. With $p_1 = 0.01$ and $N = 500$, $P_N \approx 0.993$: a 99% "on-demand" source built from 1%-probability parts, with $g^{(2)}(0)$ still set by the low $\mu$ of each individual source.

**Worked example — the switch-loss ceiling.** The output photon must traverse the multiplexing network. Suppose each 2×2 switch has transmission $\eta_{sw} = 0.98$ (0.09 dB — aggressive for today's fast switches) and the network depth is $\log_2 N = 9$ for $N = 500$. Delivered efficiency: $0.98^9 \approx 0.83$, before source heralding efficiency and detector loss. If instead $\eta_{sw} = 0.9$ (0.46 dB, typical of current fast LiNbO₃ or BTO switches), delivery falls to $0.9^9 \approx 0.39$ — the multiplexing has *destroyed* more brightness than it created relative to a modest QD source. The lesson: **multiplexed SPDC lives or dies on ultra-low-loss fast switches**, which is why photonic quantum computing companies treat the switch, not the source, as the critical component. Temporal multiplexing (one source, one switchable delay loop, many time bins) trades the switch tree for a low-loss loop and has demonstrated the same principle with fewer components (e.g., Kaneda & Kwiat's time-multiplexed source, with heralded output probability >60%).

## SPDC vs. Quantum Dots: The Architect's Summary

| | Heralded SPDC/SFWM | Quantum dot |
|---|---|---|
| Firing | Probabilistic, heralded ($\mu \lesssim 0.005$ for purity) | On-demand ($\pi$-pulse) |
| $g^{(2)}(0)$ | $\approx 2\mu$, tunable | $<10^{-3}$ demonstrated |
| Indistinguishability | >0.99 by JSA engineering; identical sources trivial to replicate | 0.98–0.995 same-dot; remote dots need tuning |
| Wavelength | Any, incl. 1550 nm natively | 900–950 nm mature; telecom emerging |
| Temperature | 300 K (source itself) | ~4 K |
| Foundry-compatible | Yes (Si, Si₃N₄, LNOI rings) | No (III-V epitaxy, per-dot tuning) |
| Scaling burden | Switch loss, PNR heralding | Spectral inhomogeneity, cryogenics |

Chapter 20's architectures inherit exactly these constraints: fusion-based schemes (PsiQuantum) assume multiplexed SFWM sources; boson-sampling demonstrations split between QD single photons (small $n$, high rate) and squeezed-light SPDC (Gaussian boson sampling, where the multi-pair "problem" becomes the computational resource).
