# 20.2.1 Measurement-Induced Nonlinearity: The NS Gate

## The Target

The workhorse of the KLM construction is deliberately minimal. The **nonlinear sign (NS) gate** acts on a *single mode* containing at most two photons:

$$\alpha_0|0\rangle + \alpha_1|1\rangle + \alpha_2|2\rangle \ \xrightarrow{\ NS\ }\ \alpha_0|0\rangle + \alpha_1|1\rangle - \alpha_2|2\rangle.$$

Only the two-photon amplitude changes sign. This is manifestly nonlinear: a phase shifter $e^{i\phi\hat{n}}$ gives phases $(1, e^{i\phi}, e^{2i\phi})$ — the two-photon phase is always *twice* the one-photon phase, and $(0, 0, \pi)$ violates that constraint. NS is precisely a Kerr interaction $e^{i\pi\hat{n}(\hat{n}-1)/2}$ truncated to $n \leq 2$: the operation linear optics cannot do.

## The Circuit

KLM's construction uses the signal mode plus two ancilla modes, prepared with **one ancilla photon and one vacuum**: input $|\psi\rangle_1 \otimes |1\rangle_2 \otimes |0\rangle_3$. The three modes pass through a fixed linear network $U$ (three beam splitters in the original layout), and the ancillas are measured. The gate **succeeds when the detectors report exactly the ancilla input back**: one photon in mode 2, zero in mode 3.

Why this works: conditioned on that herald, the signal amplitudes are multiplied by outcome amplitudes that depend on how many signal photons *could have* interfered with the ancilla photon inside the network. Bosonic exchange makes those amplitudes different for $n = 0, 1, 2$ — this is where the two-photon physics enters — and the network is chosen to make the differences exactly the NS pattern.

Write the heralded transformation as $\alpha_n \to c_n\,\alpha_n$. Computing the amplitudes for detecting $(1_2, 0_3)$:

$$c_0 = U_{22}, \qquad c_1 = U_{11}U_{22} + U_{12}U_{21}, \qquad c_2 = U_{11}^2 U_{22} + 2\,U_{11}U_{12}U_{21}.$$

($c_1$: either both signal and ancilla photons pass straight, or they exchange; $c_2$: the permanents of the relevant 2×2 and 3×3 submatrices — a first glimpse of the permanents that will dominate Section 20.4.) The NS gate demands

$$c_0 = c_1 = -c_2 = \lambda,$$

with $|\lambda|^2$ — the success probability, notably *independent of the input state* — as large as possible. Solving the three constraints over unitary $3\times3$ matrices gives

$$U_{11} = 1 - \sqrt{2}, \qquad U_{12} = U_{21} = 2^{-1/4}, \qquad U_{22} = \tfrac{1}{2},$$

(the remaining entries fixed by unitarity), yielding

$$\lambda = \tfrac{1}{2}, \qquad P_{success} = |\lambda|^2 = \boxed{\tfrac{1}{4}}.$$

One can verify directly: $c_0 = 1/2$; $c_1 = (1-\sqrt{2})/2 + 1/\sqrt{2} = 1/2$; $c_2 = (1-\sqrt{2})^2/2 + 2(1-\sqrt{2})/\sqrt{2} = (3-2\sqrt{2})/2 + \sqrt{2} - 2 = -1/2$. ✓

When the detectors report anything else, the signal state is corrupted and must be discarded — the gate has *failed*, heralded as such. This distinction — **heralded failure**, as opposed to silent error — is what makes nondeterministic gates usable at all: you always know whether the gate worked, before the computation proceeds.

## Three Structural Lessons

**1. Measurement is the nonlinearity.** No material nonlinearity appears anywhere: the ingredients are a single ancilla photon (a nonclassical resource), bosonic exchange statistics inside $U$ (which entangle signal and ancilla amplitudes), and projective detection (which selects among them). Formally, the heralded map $\hat{\Omega} = \langle 1_2 0_3|\,\hat{U}\,|1_2 0_3\rangle$ is a *non-unitary contraction* on the signal mode — exactly the class of operations unitary evolution forbids.

**2. The herald must count.** Success requires distinguishing "exactly one photon" in mode 2 from two (the case where the signal dumped a photon into the ancilla). The NS gate thus needs photon-number-resolving detection — the TES and segmented-SNSPD technology of Section 19.2.3, and historically a major motivation for it.

**3. The probability ceiling is real.** Is 1/4 an artifact of KLM's particular circuit? Essentially no: it was later proven that no heralded linear-optical NS gate with a single ancilla photon — indeed no scheme with arbitrary ancilla photons in the natural constructions studied — exceeds $P = 1/4$ (Eisert, 2005, for the exact heralded case). Similar ceilings (Knill's $2/27$ bound context for two-ancilla CZ constructions; the 50% linear-optics Bell-measurement limit of Calsamiglia & Lütkenhaus, 2001) recur across linear optics. Measurement-induced nonlinearity is *bounded* nonlinearity; no amount of circuit cleverness makes a single heralded gate deterministic. Making *computation* deterministic anyway requires the teleportation machinery of Section 20.2.3.

## Variants

Simplifications followed quickly: Ralph et al. (2002) reduced the NS gate to two beam splitters; Knill (2002) found a CZ using two ancilla photons with success $2/27$ without separate NS gates; and "KLM-style" gates were demonstrated within two years of the proposal in bulk optics. The specific circuits matter less than the template they established — *interfere, measure, herald, feed forward* — which every subsequent photonic architecture, including the fusion networks now being manufactured, instantiates.
