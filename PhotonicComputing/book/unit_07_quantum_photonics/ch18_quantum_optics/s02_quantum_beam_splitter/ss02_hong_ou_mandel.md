# 18.2.2 The Hong-Ou-Mandel Effect

## Two Photons That Refuse to Separate

Send one photon into each input port of a 50/50 beam splitter — the state $|1,1\rangle$ — and ask how often the two detectors at the outputs click in coincidence. Classically, each photon chooses transmission or reflection independently, so a coincidence (one photon to each output) should occur half the time. Quantum-mechanically, if the two photons are truly *indistinguishable*, the coincidence rate is **exactly zero**. The photons always leave by the same port, bunched into $|2,0\rangle$ or $|0,2\rangle$. This is the **Hong-Ou-Mandel (HOM) effect**, the most important two-photon phenomenon in optics.

## The Cancellation, Explicitly

Take the input $|1,1\rangle = \hat{a}^\dagger\hat{b}^\dagger|0,0\rangle$ and apply the 50/50 creation-operator map from Section 18.2.1:

$$\hat{a}^\dagger \to \frac{\hat{c}^\dagger + i\,\hat{d}^\dagger}{\sqrt2}, \qquad \hat{b}^\dagger \to \frac{i\,\hat{c}^\dagger + \hat{d}^\dagger}{\sqrt2}.$$

The product of the two input creation operators becomes

$$\hat{a}^\dagger\hat{b}^\dagger \to \frac{1}{2}\big(\hat{c}^\dagger + i\hat{d}^\dagger\big)\big(i\hat{c}^\dagger + \hat{d}^\dagger\big)
= \frac{1}{2}\Big(i\,\hat{c}^{\dagger 2} + \hat{c}^\dagger\hat{d}^\dagger - \hat{d}^\dagger\hat{c}^\dagger + i\,\hat{d}^{\dagger 2}\Big).$$

Because $\hat{c}^\dagger$ and $\hat{d}^\dagger$ commute (different modes), the cross terms cancel: $\hat{c}^\dagger\hat{d}^\dagger - \hat{d}^\dagger\hat{c}^\dagger = 0$. The surviving terms are

$$\hat{a}^\dagger\hat{b}^\dagger \to \frac{i}{2}\big(\hat{c}^{\dagger 2} + \hat{d}^{\dagger 2}\big).$$

Acting on vacuum and using $\hat{c}^{\dagger 2}|0\rangle = \sqrt2\,|2\rangle$,

$$|1,1\rangle \;\longrightarrow\; \frac{i}{\sqrt2}\big(|2,0\rangle + |0,2\rangle\big).$$

The output is a superposition of two photons in port $c$ and two in port $d$ — and the coincidence term $|1,1\rangle$ has **vanished identically**. Its probability is zero.

The physics of the disappearance is two-photon interference. The coincidence outcome $|1,1\rangle$ can be reached two indistinguishable ways: *both photons transmit* (amplitude $t\cdot t = t^2 = 1/2$) or *both photons reflect* (amplitude $r\cdot r = r^2 = (i/\sqrt2)^2 = -1/2$). Because the two paths lead to the identical final state and the photons carry no label to tell them apart, the amplitudes add:

$$A_{\text{coinc}} = t^2 + r^2 = \tfrac{1}{2} - \tfrac{1}{2} = 0.$$

Destructive interference of the "both-transmitted" and "both-reflected" amplitudes annihilates the coincidence. It is not that the photons repel; it is that the *amplitude* for them to come out apart cancels. Nothing here is classical — a classical intensity picture has no two-photon amplitude to cancel — and nothing requires the photons to interact. They interact with the beam splitter, never with each other.

## The HOM Dip and Its Visibility

Perfect cancellation demands perfect indistinguishability: the two photons must be identical in every degree of freedom — frequency, arrival time, polarization, and transverse mode. Spoil any of these and the two paths acquire a distinguishing label, the interference weakens, and coincidences reappear. Scan a **relative delay** $\tau$ between the two input photons and the coincidence probability traces the **HOM dip**. For photons with Gaussian spectral amplitudes of rms bandwidth $\sigma$,

$$P_{\text{coinc}}(\tau) = \frac{1}{2}\Big(1 - e^{-\sigma^2\tau^2}\Big),$$

zero at $\tau=0$ and rising to the distinguishable-particle value $1/2$ once $|\tau|$ exceeds the photon coherence time $\tau_c \sim 1/\sigma$. The dip is a direct picture, in coincidence counts, of the wavepacket overlap.

The depth of the dip is the **visibility**

$$V = \frac{C_{\text{off}} - C_{\min}}{C_{\text{off}}} = 1 - 2P_{\min},$$

where $C_{\text{off}} \propto 1/2$ is the coincidence level at large delay and $C_{\min}$ the level at the bottom. For ideal photons $P_{\min}=0$ and $V=1$. In general the visibility equals the **indistinguishability**

$$V = M = |\langle\psi_1|\psi_2\rangle|^2,$$

the squared overlap of the two single-photon wavepackets. HOM is thus a *meter for indistinguishability*: a number between 0 (fully distinguishable) and 1 (perfectly identical) read straight off the dip. This is why the HOM visibility is the figure of merit for single-photon sources in Chapter 19 — it certifies that two photons emitted at different times, or by different devices, are genuinely the same photon.

**Worked example.** *Dip width, and the ceiling set by imperfect sources.*

*(a) Timing resolution.* Down-converted photons filtered to a bandwidth of $\Delta\nu \approx 1$ THz have coherence time $\tau_c \sim 1/(2\pi\Delta\nu) \approx 160$ fs. The HOM dip has this sub-picosecond width even though the detectors' timing jitter is tens of picoseconds — because the dip is resolved by *scanning the optical delay* $\tau$ (a translation stage moving a mirror by micrometers), not by timing the clicks. This is exactly how Hong, Ou, and Mandel measured subpicosecond time intervals between two photons in 1987 (Hong, Ou & Mandel, 1987): the position of the dip minimum located the point of equal path length to better than the photon coherence length, giving femtosecond effective resolution from picosecond detectors.

*(b) Multi-photon ceiling.* Real single-photon sources occasionally emit two photons, quantified by $g^{(2)}(0)$. These stray pairs produce coincidences that no interference cancels, filling in the dip and capping the visibility even for perfectly indistinguishable photons. To leading order the ceiling is

$$V_{\max} \approx 1 - 2\,g^{(2)}(0).$$

For two sources each with $g^{(2)}(0)=0.02$, the maximum achievable HOM visibility is about $V_{\max}\approx 0.96$ — so a measured dip of, say, $0.94$ implies near-unity intrinsic indistinguishability, with the shortfall dominated by multi-photon emission rather than by spectral mismatch. (The exact coefficient depends on the source model and on how coincidences are normalized; some conventions quote $V_{\max}\approx 1 - g^{(2)}(0)$, placing the ceiling near $0.98$. Either way, purity and indistinguishability are entangled specifications: you cannot certify one without controlling the other.)

## Why It Matters

The HOM effect is the single quantum resource that linear optics offers for free. Two non-interacting photons, given nothing but a beam splitter and the demand that they be identical, produce an output that is entangled in photon number and impossible to reproduce classically. Section 18.2.3 elevates this from a beautiful experiment to the *primitive* of an entire computing architecture: every entangling gate in Chapter 20 is HOM interference, harvested by measurement. And the same effect run in reverse — the requirement of indistinguishability — is what makes the source engineering of Chapter 19 so unforgiving.
