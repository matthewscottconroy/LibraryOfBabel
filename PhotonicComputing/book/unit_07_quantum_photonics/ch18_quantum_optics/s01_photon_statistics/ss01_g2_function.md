# 18.1.1 The Second-Order Coherence Function

## First-Order Coherence Is Not Enough

Before the correlation that matters, meet the one that does not. The **first-order coherence function**

$$g^{(1)}(\tau) = \frac{\langle\hat{a}^\dagger(t)\,\hat{a}(t+\tau)\rangle}{\langle\hat{a}^\dagger(t)\,\hat{a}(t)\rangle}$$

measures field-amplitude correlation. Its modulus $|g^{(1)}(\tau)|$ is exactly the fringe visibility in a Michelson or Mach-Zehnder interferometer, and by the Wiener-Khinchin theorem it is the Fourier transform of the power spectrum: $g^{(1)}$ knows the linewidth and nothing else. Here is the crucial deficiency. A thermal lamp filtered to a $1$ GHz linewidth and a laser stabilized to the same linewidth have *identical* $g^{(1)}(\tau)$ and produce *identical* interference fringes. First-order coherence — Dirac's "photon interfering with itself" — cannot see the difference between chaotic and coherent light. The quantum statistics live one order higher.

## Definition and the Classical Bound

The **second-order (intensity) coherence function** correlates photodetection *events*:

$$g^{(2)}(\tau) = \frac{\langle\hat{a}^\dagger(t)\,\hat{a}^\dagger(t+\tau)\,\hat{a}(t+\tau)\,\hat{a}(t)\rangle}{\langle\hat{a}^\dagger\hat{a}\rangle^2}$$

The numerator is *normally ordered* — all creation operators to the left — because that is the order in which a photodetector, which absorbs photons, acts. Operationally, $g^{(2)}(\tau)$ is the probability of detecting a photon at time $t+\tau$ *given* a detection at $t$, normalized so that uncorrelated (Poissonian) arrivals give $g^{(2)}=1$.

For a classical field of fluctuating intensity $I(t)$, the same quantity reads $g^{(2)}(\tau) = \langle I(t)I(t+\tau)\rangle/\langle I\rangle^2$, and at zero delay

$$g^{(2)}(0) = \frac{\langle I^2\rangle}{\langle I\rangle^2} = 1 + \frac{\langle(\Delta I)^2\rangle}{\langle I\rangle^2} \ge 1.$$

Because a variance can never be negative, **every classical field obeys $g^{(2)}(0)\ge 1$** — this is the Cauchy-Schwarz inequality for the field. Classical light is at best random ($g^{(2)}(0)=1$) and generically bunched ($>1$); it is *never* antibunched. The quantum expression escapes the bound only because normal ordering removes the shot-noise "self-count": $\hat{a}^\dagger\hat{a}^\dagger\hat{a}\hat{a} = \hat{n}(\hat{n}-1)$ counts *pairs*, and a state with fewer than one photon on average per pair has nowhere to hide.

## The Canonical Values

Writing $g^{(2)}(0) = \langle\hat{n}(\hat{n}-1)\rangle/\langle\hat{n}\rangle^2$ for a single mode, three benchmark states organize everything that follows:

- **Coherent state** $|\alpha\rangle$ (ideal laser). Poissonian statistics give $\langle\hat{n}(\hat{n}-1)\rangle = \bar{n}^2$, so $g^{(2)}(0)=1$ at all delays — photons arrive like ideal raindrops, memoryless.
- **Thermal / chaotic light** (a lamp, a single spatial-temporal mode of blackbody). $g^{(2)}(0)=2$: photons are twice as likely to arrive together as at random. This is **bunching**, the Bose tendency of indistinguishable bosons to clump.
- **Fock state** $|n\rangle$. $g^{(2)}(0) = 1 - 1/n$, so $|1\rangle$ gives exactly $0$ (a single photon cannot be detected twice), $|2\rangle$ gives $1/2$, and large $n$ approaches the classical $1$ from below. **Antibunching** — $g^{(2)}(0)<1$ — is the exclusive fingerprint of non-classical light.

## The Mandel Q Parameter

The same information, phrased in the number distribution rather than the correlation, is the **Mandel parameter**

$$Q = \frac{\langle(\Delta\hat{n})^2\rangle}{\langle\hat{n}\rangle} - 1, \qquad g^{(2)}(0) = 1 + \frac{Q}{\langle\hat{n}\rangle}.$$

$Q=0$ is Poissonian (coherent); $Q>0$ is super-Poissonian (bunched, broader than Poisson); $Q<0$ is **sub-Poissonian**, the number-squeezed regime with no classical analogue. $Q$ has a hard floor at $-1$, reached only by a photon-number eigenstate ($\Delta\hat{n}=0$). Sub-Poissonian statistics ($Q<0$) and antibunching ($g^{(2)}(0)<1$) coincide for a single stationary mode, and both are strict non-classicality witnesses — but they are logically distinct in general, the former a statement about photon number, the latter about arrival timing.

**Worked example.** *Compute $g^{(2)}(0)$ for a Fock state and for thermal light from the number statistics.*

For the Fock state $|n\rangle$, the number operator is diagonal: $\hat{n}|n\rangle = n|n\rangle$ with zero variance. Then

$$\langle\hat{n}(\hat{n}-1)\rangle = n(n-1), \qquad g^{(2)}(0) = \frac{n(n-1)}{n^2} = 1 - \frac{1}{n}.$$

For $n=1$ this is $0$: a one-photon state has $\langle\hat{n}(\hat{n}-1)\rangle=0$ because there is no pair to detect. Its Mandel parameter is $Q = (0)/1 - 1 = -1$, the extreme sub-Poissonian value.

For a single-mode thermal state, the photon-number distribution is geometric, $P(n) = \bar{n}^n/(1+\bar{n})^{n+1}$. Its factorial moment is
$$\langle\hat{n}(\hat{n}-1)\rangle = \sum_{n} n(n-1)\,P(n) = 2\bar{n}^2,$$
a standard result for the Bose-Einstein distribution (equivalently, its variance is $\langle(\Delta\hat{n})^2\rangle = \bar{n} + \bar{n}^2$). Therefore
$$g^{(2)}(0) = \frac{2\bar{n}^2}{\bar{n}^2} = 2, \qquad Q = \frac{\bar{n}+\bar{n}^2}{\bar{n}} - 1 = \bar{n}.$$
Thermal light bunches at exactly $2$ *independent of brightness*, and its excess (super-Poissonian) number noise $Q=\bar{n}$ grows with the mean — the "excess noise" that plagues incoherent sources. Note the trap: a *multimode* thermal source (a lamp collected over many modes) averages toward $g^{(2)}(0)\to 1$, which is why the bunching signal requires spatial and temporal filtering to a single mode — the technical heart of the next subsection.

## Delay Dependence and the Siegert Relation

The zero-delay value is the headline, but the full curve $g^{(2)}(\tau)$ carries the timescales. For chaotic (thermal) light the two orders of coherence are locked together by the **Siegert relation**,

$$g^{(2)}(\tau) = 1 + \big|g^{(1)}(\tau)\big|^2,$$

so the bunching peak at $\tau=0$ decays to the uncorrelated value $1$ over the field's coherence time $\tau_c$ — the same $\tau_c$ that sets the first-order fringe contrast and the inverse linewidth. Bunching is thus a *finite-time* phenomenon: photons from a chaotic source clump within a coherence time and are uncorrelated beyond it. A detector slower than $\tau_c$ averages the peak away, which is exactly why the HBT measurement (Section 18.1.2) demands temporal resolution comparable to the coherence time, and why the historic experiments filtered their thermal light to a narrow bandwidth to stretch $\tau_c$ into the measurable range. Coherent light, by contrast, has $g^{(2)}(\tau)=1$ at all delays — no timescale, no memory — and antibunched light *rises* toward $1$ from below, the inverted signature of the next two subsections.

## Why It Matters

$g^{(2)}(0)$ is the master diagnostic of quantum light. It is the number a laboratory reports to claim a single-photon source ($g^{(2)}(0)<0.01$, Chapter 19); it is the quantity the Hanbury Brown-Twiss apparatus of Section 18.1.2 was built to measure; and its violation of the classical bound in Section 18.1.3 is the cleanest statement in this book of what "non-classical" means. Every downstream resource — heralded photons, entangled pairs, squeezed vacuum — is ultimately characterized by a correlation function of this family.
