# 20.5.1 Photon Loss as Erasure

## Loss Is a Beam Splitter to the Environment

Every loss mechanism — waveguide absorption, scattering, imperfect coupling, sub-unity detection — is, quantum-mechanically, the same operation: the mode leaks into an inaccessible environmental mode. Loss with transmission $\eta$ acts on the annihilation operator as a beam splitter,

$$\hat a \;\to\; \sqrt{\eta}\,\hat a + \sqrt{1-\eta}\,\hat b_{\text{env}},$$

with $\hat b_{\text{env}}$ traced out. A single photon then survives with probability $\eta$ and, with probability $1-\eta$, is gone — the mode is left in vacuum. This is not a phase error and not a bit flip; it removes the excitation that *was* the qubit. The dominant photonic error is therefore categorically different from the depolarizing noise of matter qubits, and demands a different response.

## Dual-Rail Turns Loss into a Heralded Erasure

The dual-rail encoding of Section 20.1.1 converts this liability into an asset. The logical states $|0\rangle_L = |1,0\rangle$ and $|1\rangle_L = |0,1\rangle$ both contain **exactly one photon** distributed over the two rails. A superposition $\alpha|0\rangle_L + \beta|1\rangle_L$ still has total photon number one. If a photon is lost, both rails read vacuum, $|0,0\rangle$ — a state *orthogonal* to the entire logical subspace. A quantum non-demolition measurement of total photon number (or, in practice, the pattern of detector clicks) therefore reveals that a qubit was lost **without disturbing the survivors**, and reveals exactly *which* qubit and *when*. Loss becomes an **erasure**: an error whose location is known even though — before correction — its effect on the encoded information is not.

Locating an error is precisely the information a code most wants. A Pauli error must be *found* (by syndrome extraction) before it can be fixed; an erasure arrives pre-located, so all the code must do is *reconstruct* the missing qubit from its neighbours. The quantitative payoff is stark. A quantum code of distance $d$ corrects up to $d-1$ erasures but only $\lfloor (d-1)/2 \rfloor$ Pauli errors — a factor-of-two advantage in reach, because half a code's distance is spent *finding* unlocated errors. The smallest quantum code correcting a single erasure is the four-qubit $[[4,1,2]]$ code (Grassl, Beth & Pellizzari, 1997), which cannot correct a single *unlocated* Pauli error at all. The advantage compounds at the level of thresholds: the surface code tolerates an erasure rate approaching $\sim 50\%$ (it becomes a bond-percolation problem on the code lattice), against roughly $\sim 11\%$ for independent Pauli noise. Photonics is the rare platform whose *dominant* error is the *easy* one to correct — provided the hardware can keep the total loss beneath that threshold.

**Detecting the erasure.** The heralding is not automatic: it requires learning that a photon is *absent* without measuring *which rail* it would have occupied, since the latter would collapse the superposition. In the one-way model this comes for free — the qubit is measured anyway, and "no click in either rail" simultaneously delivers the logical outcome and flags the loss. Elsewhere it needs a quantum non-demolition parity check or a teleportation-based total-number measurement. Loss occurring *after* the final gate is caught by the readout detectors; loss *during* the computation must be caught by the code's own structure — exactly what the foliated codes of Section 20.5.2 provide, since every stabilizer measurement doubles as a loss detector.

## The Loss Budget

"Beneath the threshold" is a demanding specification, because losses multiply along a path. A photon traversing $N$ lossy elements, each of transmission $\eta_c$, survives with probability $\eta_c^N$. The budget accumulates from Chapter 19's hardware: waveguide propagation ($\sim 0.1$–$1$ dB/cm in silicon, far less in silicon nitride), fiber-chip coupling ($\sim 0.5$–$3$ dB per interface), each MZI or switch ($\sim 0.1$–$0.5$ dB), and detector inefficiency (SNSPDs now exceed $98\%$, i.e. $< 0.1$ dB). Because the erasure threshold caps the *total* loss per logical operation, and each operation touches many components, the per-component loss must sit in the $10^{-3}$–$10^{-2}$ range — the reason the field measures success in tenths of a decibel.

## Worked Example: Survival $\eta^N$ and the Component Budget

**How fast survival falls.** With per-component transmission $\eta_c$ and $N$ components in series,

$$P_{\text{survive}} = \eta_c^{\,N}.$$

For $\eta_c = 0.99$ (a $0.044$ dB component) and $N = 100$, $P_{\text{survive}} = 0.99^{100} = 0.366$ — nearly two-thirds of photons lost across a hundred elements. Improve each component to $\eta_c = 0.999$ and the same depth gives $0.999^{100} = 0.905$. The exponent turns modest per-element losses into decisive path losses.

**Inverting for the budget.** Suppose a fusion network tolerates a total photon-loss rate up to the (optimistic) erasure figure of $10\%$, so we require $P_{\text{survive}} = \eta_c^N > 0.9$ across a path of $N = 50$ components. Then

$$\eta_c > 0.9^{\,1/50} = 0.99790, \qquad \text{loss per component} < 0.21\% \equiv 0.0091\ \text{dB}.$$

Every waveguide bend, coupler, and switch on the path must lose under a hundredth of a decibel — a specification that dictates ultra-low-loss silicon-nitride routing, SNSPDs above $98\%$, and, above all, architectures (Section 20.5.2–20.5.3) that keep the component count $N$ per operation as small as possible. This is the fault-tolerance implication in one line: because erasures are cheap to correct but expensive to avoid, photonic architectures win by minimizing the number of things a photon must pass through before it is measured. Were these same losses *unlocated* — Pauli-like rather than heralded — the tolerable rate would collapse from $\sim 50\%$ to $\sim 11\%$, a four-to-five-fold tightening of every loss budget above; the located character of photon loss is worth a decisive factor in each of these numbers.
