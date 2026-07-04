# 19.3.2 Strong Coupling and Photon Blockade

## Three Rates Decide Everything

A real cavity-emitter system has three characteristic rates:

- $g$ — the coherent atom-photon coupling (Section 19.3.1),
- $\kappa = \omega_c/Q$ — the cavity photon decay rate (mirror transmission, scattering, absorption),
- $\gamma$ — the emitter's decay rate into non-cavity modes (plus pure dephasing $\gamma^*$ for solid-state emitters).

The physics is governed by the dimensionless comparisons among them. Two regimes matter:

**Strong coupling:** $g \gg \kappa, \gamma$. An excitation swaps between atom and field many times before it is lost. The dressed-state doublet of Section 19.3.1 is spectrally resolved: the vacuum Rabi splitting $2g$ exceeds the linewidths. The atom and cavity have ceased to be separate objects.

**Weak coupling:** $g \lesssim \kappa$. The cavity acts perturbatively — it cannot re-absorb the photon, but it *reshapes the vacuum* seen by the emitter, enhancing (or suppressing) spontaneous emission. This is the Purcell regime (Section 19.3.3).

A single figure of merit interpolates between them: the **cooperativity**

$$C = \frac{4g^2}{\kappa\gamma},$$

the ratio of coherent coupling to the geometric mean of the losses. $C > 1$ means the emitter talks to the cavity mode faster than to everything else combined — achievable in either regime, and often more important for devices than strong coupling itself.

## Representative Numbers

| Platform | $g/2\pi$ | $\kappa/2\pi$ | $\gamma/2\pi$ | Regime |
|---|---|---|---|---|
| Single atom, high-finesse Fabry-Pérot (Caltech/MPQ) | 10–50 MHz | 1–5 MHz | 3–6 MHz | Strong, $C \sim 10$–100 |
| Quantum dot in photonic crystal nanocavity | ~10–20 GHz | 10–30 GHz ($Q \sim 10^4$) | ~0.1–1 GHz | Border of strong |
| Quantum dot in micropillar ($Q \sim 10^3$–$10^4$) | ~5–15 GHz | 20–300 GHz | ~0.2 GHz | Weak/Purcell |
| SiV center in diamond photonic crystal cavity | ~5–10 GHz | ~20–50 GHz | ~0.1 GHz | $C > 100$ (bad-cavity) |

The table's lesson: solid-state platforms buy enormous $g$ with wavelength-scale mode volumes, but their moderate $Q$ makes $\kappa$ large. Atomic platforms have tiny $g$ but exquisite mirrors. Both routes have crossed into strong coupling: atoms in 1992 (Thompson, Rempe & Kimble — vacuum Rabi splitting with on average one atom), quantum dots in 2004 (Yoshie et al. in a photonic crystal; Reithmaier et al. in a micropillar — splittings of ~100 μeV).

## Photon Blockade: The Single-Photon Nonlinearity

Strong coupling converts the harmonic cavity into an *anharmonic* quantum system. The transition from the ground state to the first dressed doublet sits at $\omega \pm g$; climbing from the first doublet to the second requires $\omega \pm g(\sqrt{2}-1)$ — a *different* frequency, because the ladder spacing grows as $\sqrt{n}$.

Consequence: drive the system with a laser tuned to the $|0\rangle \to |1,-\rangle$ transition. The first photon is absorbed resonantly; a second photon at the same frequency finds no available transition — it is detuned by $g(2-\sqrt{2})$ and is rejected. The cavity admits photons **one at a time**: photon blockade. The transmitted light is antibunched, $g^{(2)}(0) < 1$, even though the input was a coherent laser beam. Birnbaum et al. (2005) demonstrated exactly this with one trapped cesium atom, measuring $g^{(2)}(0) \approx 0.13$.

Photon blockade is the conceptual answer to the question that drives Chapter 20: *can one photon change what happens to another photon?* In free space and in linear circuits, no. In a strongly coupled cavity, yes — the presence of one photon detunes the system for the next. Cavity QED thus offers a **deterministic** photon-photon nonlinearity:

- **Atom-photon gates:** Duan & Kimble (2004) proposed bouncing photonic qubits off a strongly coupled atom-cavity system, where the atomic state imposes a conditional $\pi$ phase on the reflected photon. Realized by the Rempe group as an atom-photon quantum gate and then a **photon-photon CZ gate** (Hacker et al., 2016), with the atom mediating the interaction between two photons that never coexist in the cavity.
- **Single-photon switches and transistors:** one gate photon, stored in the atom or in a Rydberg-blockaded ensemble, routes or blocks subsequent signal photons.

These deterministic gates are the road *not* taken by mainstream photonic quantum computing — the engineering overhead of one high-cooperativity atom (or artificial atom) per gate has so far lost to the linear-optics-plus-measurement approach of Chapter 20. But they set the standard the linear-optics world must beat, and hybrid architectures (cavity-QED-generated photonic cluster states; Section 20.3) increasingly merge the two.

## Dephasing: The Solid-State Tax

For quantum dots and color centers, pure dephasing $\gamma^*$ (phonons, charge noise, spectral diffusion) broadens the emitter line without emptying the excited state. Strong coupling then demands $g \gg \kappa, \gamma, \gamma^*$, and $\gamma^*$ is often the largest of the three at elevated temperatures — the reason quantum-dot cavity QED lives at 4 K and why the phonon sideband haunts even Purcell-enhanced sources. The practical resolution for *sources* is not to fight for strong coupling at all, but to work in the fast, high-$\beta$ weak-coupling regime — the Purcell effect, to which we now turn.
