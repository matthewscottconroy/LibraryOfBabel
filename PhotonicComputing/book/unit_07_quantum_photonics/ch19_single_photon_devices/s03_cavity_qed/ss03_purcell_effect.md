# 19.3.3 The Purcell Effect: Engineering Spontaneous Emission

## Spontaneous Emission Is Not a Property of the Atom

Spontaneous emission feels like an intrinsic atomic property, but Fermi's golden rule says otherwise:

$$\Gamma = \frac{2\pi}{\hbar^2}\,|\langle f|\hat{H}_{int}|i\rangle|^2\,\rho(\omega)$$

The rate depends on $\rho(\omega)$ — the **density of electromagnetic states** at the transition frequency, evaluated where the emitter sits. Change the photonic environment, change the decay rate. Purcell stated the result in a famously terse 1946 abstract (Purcell, *Phys. Rev.* 69, 681): a resonant cavity enhances the emission rate of a coupled dipole by the factor now bearing his name.

## The Purcell Factor

A cavity of quality factor $Q$ and mode volume $V$ concentrates its density of states into a Lorentzian of width $\omega/Q$ around resonance. Working through the golden rule for an emitter on resonance, at the field antinode, with its dipole aligned to the mode, the emission rate into the cavity mode exceeds the free-space rate (in the same host material, index $n$) by

$$\boxed{F_P = \frac{3}{4\pi^2}\left(\frac{\lambda}{n}\right)^3 \frac{Q}{V}}$$

Everything is in the ratio $Q/V$: how long the cavity stores light, per how small a box. The prefactor $3/4\pi^2 \approx 0.076$ means a cavity needs $Q/V \gtrsim 13\,(\lambda/n)^{-3}$ just to double the emission rate — and photonic crystal nanocavities, with $V \approx 0.5\,(\lambda/n)^3$ and $Q$ up to $10^6$, offer *ideal* Purcell factors in the tens of thousands. (Real devices realize far less, as discussed below, but tens to hundreds are routinely engineered.)

In cavity-QED language, the Purcell-enhanced rate into the cavity is $\Gamma_c = 4g^2/\kappa$ (the "bad-cavity" limit $\kappa \gg g$): the emitter coherently feeds the mode at rate $g$, and the cavity drains it at $\kappa$ before it can return. The connection $F_P = \Gamma_c/\Gamma_0 = \frac{3}{4\pi^2}(\lambda/n)^3 Q/V$ follows by substituting $g(V)$ and $\kappa(Q)$.

Three fine-print conditions, each an engineering task:

1. **Spectral alignment:** the emitter must sit within the cavity linewidth; detuning $\delta$ reduces enhancement by $1/(1 + 4\delta^2/\kappa^2)$. For narrowband cavities this demands nm-scale (or better) tuning — gas condensation, temperature, strain.
2. **Spatial alignment:** enhancement scales as $|E(\mathbf{r}_e)|^2/|E_{max}|^2$; a quantum dot 100 nm off the antinode of a photonic crystal cavity mode loses most of its $F_P$. Hence deterministic-positioning technologies (in-situ optical lithography, AFM registration).
3. **Linewidth hierarchy:** the *emitter* must be spectrally narrower than the cavity ($\gamma, \gamma^* < \kappa$), or the roles reverse and the effective $Q$ is the emitter's. Broad room-temperature emitters cannot harvest a high-$Q$ cavity's full Purcell factor — one reason single-photon sources run cold.

## The β-Factor: Why Sources Want Purcell, Not Strong Coupling

For a source, the quantity that matters is the fraction of emitted photons that enter the *collectable* cavity mode:

$$\beta = \frac{F_P\,\Gamma_0}{F_P\,\Gamma_0 + \Gamma_{leak}} \approx \frac{F_P}{F_P + 1}$$

(with $\Gamma_{leak} \approx \Gamma_0$ the residual emission into all other modes). Modest Purcell factors already saturate this: $F_P = 10$ gives $\beta = 0.91$; $F_P = 50$ gives $\beta = 0.98$. Beyond brightness, the shortened lifetime $T_1' = T_1/F_P$ outpaces dephasing — recall $M = T_2/2T_1$ from Section 19.1.1 — so Purcell enhancement simultaneously raises indistinguishability and repetition rate. This triple dividend (brightness, coherence, speed) is why every leading quantum-dot source (Section 19.1.2) is a Purcell device operating in weak coupling, not a strong-coupling device.

## Worked Example: Micropillar vs. Photonic Crystal

**Micropillar** ($Q = 5{,}000$, $V = 15\,(\lambda/n)^3$, emitter well aligned):

$$F_P = 0.076 \times \frac{5000}{15} \approx 25$$

A $T_1 = 1$ ns quantum dot decays in ~40 ps; $\beta = 25/26 = 0.96$. The pillar's Gaussian-like output mode couples efficiently to fiber — the design used by the near-optimal sources of Section 19.1.2.

**Photonic crystal L3 nanocavity** ($Q = 30{,}000$, $V = 0.6\,(\lambda/n)^3$):

$$F_P = 0.076 \times \frac{30000}{0.6} \approx 3800 \quad \text{(ideal)}$$

In practice, spatial/spectral misalignment and the linewidth hierarchy cap measured lifetime reductions at ~10–75× — still transformative, and the near-unity $\beta$ ($>0.98$) of photonic-crystal *waveguides* (a continuum version of the same density-of-states engineering) drives the Danish planar-source approach.

The comparison illustrates a design maxim: **$V$ is worth more than $Q$ only until the emitter linewidth or fabrication tolerance is reached; after that, mode-matching to the collection channel decides the device.**

## Suppression and the Other Half of the Effect

The density of states can also be *reduced*: an emitter in a photonic bandgap, or in a cavity tuned off-resonance, radiates more slowly than in free space (inhibited spontaneous emission — demonstrated for Rydberg atoms by Hulet, Hilfer & Kleppner, 1985). Photonic crystal sources exploit both directions at once: the bandgap suppresses emission into unwanted modes ($\Gamma_{leak} \ll \Gamma_0$), while the defect mode enhances the wanted channel — pushing $\beta$ beyond what enhancement alone achieves.

## Purcell Physics Beyond Sources

The same $Q/V$ logic governs devices across this book: microring modulators (resonant enhancement of electro-optic interaction, Chapter 7), nanolasers approaching thresholdless operation ($\beta \to 1$), cavity-enhanced detectors (the SNSPD optical stack of Section 19.2.2 is an absorption-side Purcell argument), and the cavity-QED nodes of Chapter 22. Wherever a weak light-matter interaction must be made strong without more power, the answer is the same: shrink $V$, raise $Q$, and align the emitter to the mode.
