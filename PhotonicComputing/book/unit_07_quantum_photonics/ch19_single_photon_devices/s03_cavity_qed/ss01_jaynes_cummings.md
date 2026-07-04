# 19.3.1 The Jaynes-Cummings Model

## The Minimal Light-Matter System

Take the simplest matter system — a two-level atom with ground state $|g\rangle$, excited state $|e\rangle$, and transition frequency $\omega_a$ — and the simplest field — a single cavity mode of frequency $\omega_c$ with annihilation operator $\hat{a}$. Their interaction, in the dipole and rotating-wave approximations, is the **Jaynes-Cummings Hamiltonian** (Jaynes & Cummings, 1963):

$$\hat{H}_{JC} = \hbar\omega_c\,\hat{a}^\dagger\hat{a} + \frac{\hbar\omega_a}{2}\,\hat{\sigma}_z + \hbar g\left(\hat{a}^\dagger\hat{\sigma}_- + \hat{a}\,\hat{\sigma}_+\right)$$

where $\hat{\sigma}_+ = |e\rangle\langle g|$, $\hat{\sigma}_- = |g\rangle\langle e|$, and $\hat{\sigma}_z = |e\rangle\langle e| - |g\rangle\langle g|$. The interaction term does exactly one thing, coherently and reversibly: it converts atomic excitation into a photon ($\hat{a}^\dagger\hat{\sigma}_-$) and back ($\hat{a}\,\hat{\sigma}_+$).

The coupling rate $g$ is the dipole matrix element $d$ times the vacuum field amplitude of the mode:

$$g = \frac{d}{\hbar}\sqrt{\frac{\hbar\omega_c}{2\varepsilon_0 V}}$$

with $V$ the cavity mode volume. This formula carries the central engineering message of cavity QED: **$g \propto 1/\sqrt{V}$** — shrink the box, and the vacuum field per photon grows. A photonic crystal cavity with $V \approx (\lambda/n)^3 \sim 0.06\ \mu\text{m}^3$ concentrates one photon's field enough that a quantum dot reaches $g/2\pi \sim 10$–20 GHz; a centimeter-scale Fabry-Pérot with a trapped atom manages $g/2\pi \sim 10$–50 MHz.

The rotating-wave approximation (dropping $\hat{a}^\dagger\hat{\sigma}_+$ and $\hat{a}\hat{\sigma}_-$) is valid for $g \ll \omega_c$ — comfortably true in all optical cavity QED. (Its breakdown, at $g \gtrsim 0.1\,\omega_c$, defines the *ultrastrong coupling* regime, accessible in superconducting circuits but not optics.)

## Exact Solution: The Dressed-State Ladder

$\hat{H}_{JC}$ conserves the total excitation number $\hat{N} = \hat{a}^\dagger\hat{a} + |e\rangle\langle e|$, so the Hamiltonian block-diagonalizes into 2×2 sectors spanned by $\{|e, n\rangle, |g, n+1\rangle\}$. On resonance ($\omega_a = \omega_c \equiv \omega$), the interaction couples the two degenerate basis states of each sector with matrix element $\hbar g\sqrt{n+1}$:

$$H^{(n)} = \hbar\omega\left(n + \tfrac{1}{2}\right)\mathbb{1} + \hbar\begin{pmatrix} 0 & g\sqrt{n+1} \\ g\sqrt{n+1} & 0 \end{pmatrix}$$

(taking the sector's common energy as reference). The eigenstates — the **dressed states** — are the symmetric and antisymmetric superpositions

$$|n, \pm\rangle = \frac{1}{\sqrt{2}}\big(|e, n\rangle \pm |g, n+1\rangle\big), \qquad E_{n,\pm} = \hbar\omega\left(n + \tfrac{1}{2}\right)\ \pm\ \hbar g\sqrt{n+1}.$$

Three consequences, each experimentally iconic:

1. **Vacuum Rabi splitting.** The lowest doublet ($n = 0$) is split by $2\hbar g$ — an energy gap created by a *single shared excitation* interacting with the *vacuum*. A weak probe transmitted through the cavity shows two peaks separated by $2g$ instead of one: the definitive signature that a single emitter and a single mode have hybridized. First seen with single atoms (Thompson, Rempe & Kimble, 1992) and with single quantum dots in photonic crystal and micropillar cavities (Yoshie et al.; Reithmaier et al., both 2004).

2. **Vacuum Rabi oscillations.** Prepare $|e, 0\rangle$ (excited atom, empty cavity). It is not an eigenstate; it oscillates:

$$|\psi(t)\rangle = \cos(gt)\,|e,0\rangle - i\sin(gt)\,|g,1\rangle$$

The excitation swaps between atom and field at rate $2g$ — spontaneous emission made *reversible*. Stop the evolution at $gt = \pi/2$ (by tuning, or by letting the photon leak out) and you have converted a stationary qubit into a flying photon: this is the physical primitive behind cavity-based single-photon sources and spin-photon interfaces.

3. **An anharmonic ladder.** The splitting grows as $\sqrt{n+1}$: the transition energies from the ground state to $|0,\pm\rangle$, and from $|0,\pm\rangle$ to $|1,\pm\rangle$, are *different*. The spectrum is nonlinear at the level of individual quanta — the seed of photon blockade (next subsection), and the only known route to optical nonlinearities that act photon-by-photon.

## Collapse and Revival

Drive the cavity into a coherent state $|\alpha\rangle = e^{-|\alpha|^2/2}\sum_n \frac{\alpha^n}{\sqrt{n!}}|n\rangle$ with the atom in $|g\rangle$. Each Fock component $|n\rangle$ Rabi-oscillates at its own frequency $2g\sqrt{n}$; the atomic inversion is a sum over incommensurate oscillations:

$$\langle \hat{\sigma}_z(t)\rangle = \sum_n P_n \cos(2g\sqrt{n}\,t), \qquad P_n = e^{-|\alpha|^2}\frac{|\alpha|^{2n}}{n!}$$

The oscillations dephase ("collapse") on a timescale $t_c \sim 1/g$, then — because the frequencies are discrete, not continuous — rephase and **revive** at $t_r \approx 2\pi\sqrt{\bar{n}}/g$. The revival is a direct witness of field quantization: a classical field of fluctuating amplitude produces the collapse but can never produce the revival. Observed in Rydberg-atom microwave cavity QED (Haroche's and Walther's groups), collapse-and-revival remains one of the cleanest demonstrations that the electromagnetic field takes discrete values.

## What the Model Leaves Out

Everything in this subsection is unitary. Real cavities leak (photon decay rate $\kappa$), real emitters spontaneously emit into other modes ($\gamma$) and dephase ($\gamma^*$). The fate of the Jaynes-Cummings physics is decided by the competition between $g$ and these dissipation rates — the subject of the next two subsections, and the difference between a curiosity and a device.
