# 3.2.3 — Population Inversion and Optical Gain

## Why Population Inversion Is Non-Trivial

The Einstein relation $B_{12} = B_{21}$ tells us something important: at equilibrium (Boltzmann distribution), the ground state always has higher population than the excited state ($N_1 > N_2$ at any finite temperature). This means a medium in thermal equilibrium always *absorbs* light at the transition frequency — never amplifies it.

To achieve gain, we must create a *non-equilibrium* population: more atoms in the excited state than in the ground state ($N_2 > N_1$, $\Delta N > 0$). This is *population inversion*, and it cannot be achieved by illuminating a two-level system with resonant light alone. With resonant light, the steady-state solution of the rate equations gives:

$$\frac{N_2}{N} = \frac{B_{21}\rho}{B_{21}\rho + B_{21}\rho + A_{21}} = \frac{B_{21}\rho}{2B_{21}\rho + A_{21}}$$

As the field intensity $\rho \to \infty$: $N_2/N \to 1/2$ (equal populations). The system is bleached — it reaches transparency, but never gain. This is *saturation* of a two-level absorber: at sufficiently high intensity, it becomes transparent. But it cannot be inverted.

## Three-Level and Four-Level Systems

To achieve population inversion, we need at least three energy levels. The classic scheme:

**Three-level laser** (e.g., ruby, Er$^{3+}$ at 1550 nm — though Er is more complex):

```
         |3⟩ (pump level)
          ↑ pump (ω_p)    ↓ fast non-radiative decay
         |2⟩ (upper laser level, long lifetime τ₂₁)
          ↓ laser emission (ω₀ = ω₂₁)
         |1⟩ (lower laser level = ground state)
```

Atoms are pumped from $|1\rangle$ to $|3\rangle$ at rate $R_p$. They rapidly relax non-radiatively to $|2\rangle$ (fast relaxation means $N_3 \approx 0$ always). $|2\rangle$ has a long lifetime $\tau_{21}$ — long enough for inversion to build up. Laser emission occurs on the $|2\rangle \to |1\rangle$ transition.

Problem with three levels: the lower laser level is the ground state $|1\rangle$, which is always significantly populated. Achieving inversion requires pumping more than half the total population into $|2\rangle$ — a high pump threshold.

**Four-level laser** (e.g., Nd:YAG, Er:fiber at 1550 nm in practice):

```
         |4⟩ (pump level)
          ↑ pump          ↓ fast non-radiative
         |3⟩ (upper laser level)
          ↓ laser emission
         |2⟩ (lower laser level, rapidly depleted)
          ↓ fast non-radiative
         |1⟩ (ground state)
```

Now the lower laser level $|2\rangle$ is not the ground state — it is rapidly depleted by non-radiative decay to $|1\rangle$. If $N_2 \approx 0$ always, any pumping at all creates $\Delta N = N_3 - N_2 > 0$ — population inversion with arbitrarily low pump power (zero threshold in the ideal case). Real four-level lasers have low but nonzero thresholds due to scattering losses and spontaneous emission.

**Erbium-doped fiber (1550 nm)**: Erbium ions in silica have a long-lived $^4I_{13/2}$ metastable level at $E_2 = 0.8$ eV above the $^4I_{15/2}$ ground state (transition at 1550 nm). Pumping with 980 nm or 1480 nm light inverts the population. The Er-doped fiber amplifier (EDFA) operates as a four-level system and provides gain across the C-band (1530–1565 nm) with typical gains of 20–40 dB and noise figures near the quantum limit ($\sim 3$ dB).

## The Gain Coefficient and Gain Saturation

For a homogeneously broadened gain medium (all atoms have the same transition frequency — appropriate for solid-state lasers and semiconductors), the gain coefficient as a function of intensity $I$ is:

$$g(I) = \frac{g_0}{1 + I/I_\text{sat}}$$

where $g_0 = \sigma\Delta N_0$ is the small-signal gain (with unsaturated inversion $\Delta N_0$) and $I_\text{sat} = \hbar\omega_0/(2\sigma\tau_1)$ is the *saturation intensity*. For $I \ll I_\text{sat}$: $g \approx g_0$ (linear gain). For $I \gg I_\text{sat}$: $g \to 0$ (gain compressed to zero).

**Physical meaning of $I_\text{sat}$**: the intensity at which stimulated emission rate equals the spontaneous emission rate. Above this intensity, stimulated emission dominates and the inversion is depleted faster than it can be replenished by pumping — gain saturates.

**For EDFAs**: $I_\text{sat} \approx 1$–10 mW (a relatively low saturation intensity). Operating above saturation compresses the gain and also equalizes it across the amplifier bandwidth — useful for WDM systems where all channels should be amplified equally.

## Laser Threshold Condition

For a laser oscillator (amplifier in a cavity), the threshold condition is: round-trip gain equals round-trip loss:

$$R_1 R_2 e^{2g_0 L} = 1$$

where $R_1$, $R_2$ are mirror reflectances, $L$ is the gain medium length, and additional losses (scattering, internal absorption) are included in the exponent. Solving:

$$g_\text{th} = \frac{1}{2L}\ln\frac{1}{R_1 R_2} + \alpha_\text{int}$$

For a DFB semiconductor laser ($L = 300$ μm, $R_1 R_2 \approx 0.3$, $\alpha_\text{int} \approx 10$ cm⁻¹): $g_\text{th} = (1/0.06)\ln(1/0.3)/2 + 10 \approx 12 + 10 = 22$ cm⁻¹. Threshold gain density $g_\text{th}/\Gamma \approx 100$–200 cm⁻¹ (where $\Gamma$ is the optical confinement factor).

## Semiconductor Gain

In semiconductor lasers (Chapter 4), the gain medium is not a collection of two-level atoms but a semiconductor with a band structure. Electrons in the conduction band and holes in the valence band recombine radiatively, emitting photons. Population inversion is achieved by electrical injection: the Fermi level is split (quasi-Fermi levels for electrons and holes) so that the condition $E_{F,e} - E_{F,h} > \hbar\omega$ is satisfied for the relevant photon energy.

The semiconductor gain has a characteristic shape: it rises steeply above the bandgap energy and peaks at a specific wavelength determined by the carrier density and temperature. The gain spectrum is broader and more complex than a simple two-level Lorentzian. The peak gain coefficient for InGaAsP (used in 1550 nm semiconductor lasers) can reach $g_0 \approx 100$–300 cm⁻¹ at threshold carrier densities $N_\text{th} \approx 10^{18}$ cm⁻³.

**For photonic computing**: On-chip light sources are an active area of development. Silicon cannot lase efficiently (indirect bandgap — electron-phonon coupling required for optical transitions, reducing efficiency). Solutions under development include [1]:
- Wafer-bonded InP or GaAs lasers on Si substrate
- Quantum dot lasers grown directly on Si (Ge/Si quantum dot lasers, InAs quantum dot lasers on Si)
- Rare-earth-doped waveguide lasers (Er:Al₂O₃ on Si)
- GeSn alloy lasers (direct-bandgap Ge-Sn alloys are explored for C-band emission)

The integration of on-chip light sources remains one of the key unsolved challenges in silicon photonics and is a prerequisite for fully integrated photonic computing systems.

## Summary

- Population inversion ($N_2 > N_1$) requires non-equilibrium pumping; not achievable by resonant illumination of a two-level system alone.
- Three-level: lower laser level = ground state; high pump threshold. Four-level: lower laser level rapidly depleted; low threshold.
- Gain coefficient $g = g_0/(1 + I/I_\text{sat})$; saturation compresses gain at high intensity.
- Laser threshold: round-trip gain = round-trip loss.
- Silicon cannot lase efficiently (indirect bandgap); on-chip laser integration is a key open challenge.

---

*References*

[1] Liang, D. & Bowers, J.E. (2010). Recent progress in lasers on silicon. *Nature Photonics*, 4(8), 511–517. [DOI: 10.1038/nphoton.2010.167] [Review of approaches to on-chip laser integration.]
