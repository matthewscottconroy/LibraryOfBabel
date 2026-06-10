# 4.1.1 Why a Two-Level System Cannot Lase

## The Seductive Simplicity of Two Levels

At first glance, a two-level system seems perfectly suited to making a laser. You have an atom with a ground state $|1\rangle$ and an excited state $|2\rangle$, separated by energy $\hbar\omega_0$. If you could get more atoms into $|2\rangle$ than $|1\rangle$, stimulated emission would amplify any light at frequency $\omega_0$. The atoms in $|2\rangle$ outnumber those in $|1\rangle$; a passing photon at $\omega_0$ triggers stimulated emission; you have gain. What stops this from working?

The answer is deceptively simple: *you cannot pump a two-level system to inversion using the same radiation that will stimulate emission*.

## The Argument from Detailed Balance

Consider a two-level atom in a radiation field of energy density $u(\omega_0)$. The populations $N_1$ and $N_2$ evolve according to:

$$\frac{dN_2}{dt} = B_{12} u(\omega_0) N_1 - B_{21} u(\omega_0) N_2 - A_{21} N_2$$

$$\frac{dN_1}{dt} = -B_{12} u(\omega_0) N_1 + B_{21} u(\omega_0) N_2 + A_{21} N_2$$

At steady state, $dN_2/dt = 0$:

$$B_{12} u N_1 = B_{21} u N_2 + A_{21} N_2$$

Since $B_{12} = B_{21} \equiv B$ (Chapter 3, Einstein relations):

$$Bu N_1 = N_2(Bu + A_{21})$$

$$\frac{N_2}{N_1} = \frac{Bu}{Bu + A_{21}}$$

Now observe: this ratio is always strictly less than 1, regardless of how large $u$ is. As $u \to \infty$ (infinitely intense pump radiation):

$$\frac{N_2}{N_1} \to \frac{Bu}{Bu} = 1$$

**The maximum population ratio achievable in a two-level system under any optical pumping is $N_2/N_1 \to 1$: equal populations, never inversion.**

This is not an engineering limitation. It is a consequence of the symmetry $B_{12} = B_{21}$: the same transition that pumps atoms from $|1\rangle$ to $|2\rangle$ stimulates them back down from $|2\rangle$ to $|1\rangle$ with equal probability. At best, you can equalize the populations. Stimulated emission at $\omega_0$ equals stimulated absorption at $\omega_0$: the gain is zero. You cannot get laser action from a two-level system.

## The Physical Intuition

There is a cleaner way to see this. A laser requires that light at the lasing frequency stimulates more emission than absorption. For this, you need $N_2 > N_1$. But the pump radiation, which is at frequency $\omega_0$ (the same frequency), drives both absorption (which increases $N_2$) and stimulated emission (which decreases $N_2$) in equal measure. The pump is fighting itself. At steady state, the best it can do is equalize the populations.

The resolution is to pump on a *different* transition, at a *different* frequency, so that the pump photons and the signal (lasing) photons do not compete. This is the key insight behind three- and four-level lasers.

## Why This Matters for Photonic Computing

This fundamental thermodynamic constraint has a direct engineering consequence: no laser ever built uses a two-level system as its gain medium. Every laser — semiconductor diode lasers, erbium-doped fiber amplifiers, Nd:YAG, Ti:sapphire, CO₂ — uses a multi-level pumping scheme. The specific multi-level structure determines the pump wavelength, the efficiency, the threshold power, and the noise figure.

For photonic computing systems, this matters because:

1. **EDFA noise figure**: Erbium-doped fiber amplifiers operate as three-level systems at 1550 nm (the lower laser level is the ground state). This means that achieving full inversion requires pumping every erbium ion out of the ground state, which is energetically costly and sets a minimum noise figure of 3 dB.

2. **Semiconductor laser efficiency**: The quantum defect — the ratio of lasing photon energy to pump photon energy — sets the minimum heat load. For an InP diode laser (emitting at 1550 nm, pumped electrically at 0.9 V bias plus ohmic losses), this is on the order of 30–50%.

3. **Silicon's fundamental problem**: Silicon cannot be made to lase by simple two- or even three-level pumping because the band structure creates an *indirect* gap. The relevant excited state (electron in the conduction band) decays predominantly via phonon emission (non-radiative recombination) rather than photon emission. The radiative efficiency of bulk silicon is approximately $10^{-6}$ — one radiative recombination per million non-radiative events.
