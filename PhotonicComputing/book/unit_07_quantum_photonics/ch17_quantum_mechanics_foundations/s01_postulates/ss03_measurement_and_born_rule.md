# 17.1.3 Measurement and the Born Rule

## The Third Postulate

**Postulate 3 (Born rule).** *A measurement of observable $\hat{A} = \sum_n a_n |a_n\rangle\langle a_n|$ on a system in state $|\psi\rangle$ yields outcome $a_n$ with probability*

$$P(a_n) = |\langle a_n|\psi\rangle|^2 = \langle\psi|\hat{P}_n|\psi\rangle, \qquad \hat{P}_n = |a_n\rangle\langle a_n|$$

*and immediately after a measurement with outcome $a_n$, the state is the (renormalized) projection*

$$|\psi\rangle \;\longrightarrow\; \frac{\hat{P}_n|\psi\rangle}{\sqrt{\langle\psi|\hat{P}_n|\psi\rangle}}$$

For mixed states, $P(a_n) = \mathrm{Tr}(\rho\hat{P}_n)$ and $\rho \to \hat{P}_n\rho\hat{P}_n / \mathrm{Tr}(\rho\hat{P}_n)$. This is **projective (von Neumann) measurement**. Probabilities are guaranteed to be non-negative and, by completeness $\sum_n \hat{P}_n = \mathbb{1}$, to sum to one.

Three features distinguish quantum measurement from classical readout, and all three are load-bearing in photonic quantum technology:

1. **Randomness is fundamental.** For a state that is not an eigenstate of $\hat{A}$, the outcome is irreducibly probabilistic — not a reflection of hidden ignorance (Section 17.4.3 makes this claim precise and experimentally tested). A single photon in $|D\rangle$ hitting an H/V polarizing beam splitter is the textbook quantum random number generator; commercial QRNG chips are exactly this.

2. **Measurement disturbs.** After the H/V measurement, the photon *is* $|H\rangle$ or $|V\rangle$; the prior superposition's phase information is gone. A second measurement in the same basis confirms the first with certainty (repeatability), but the statistics of any incompatible observable have been irreversibly changed.

3. **The basis is chosen by the apparatus.** "Measurement" is not one operation but a family: wave plates before a polarizing beam splitter select *which* observable is measured. The same photon yields different, incompatible statistics depending on that classical choice — the operational core of both BB84 and Bell tests.

**Worked example (sequential polarizers).** Send $|H\rangle$ through a diagonal analyzer, then a vertical one. The first measurement yields $|D\rangle$ with probability $|\langle D|H\rangle|^2 = 1/2$; a transmitted photon *is now* $|D\rangle$, so the second yields $|V\rangle$ with probability $|\langle V|D\rangle|^2 = 1/2$. Net transmission: $1/4$. Remove the middle analyzer and the transmission is $|\langle V|H\rangle|^2 = 0$. An intermediate measurement *opens* a channel that was closed — impossible for classical filters acting on ignorance, and a two-polarizer demonstration that measurement changes the state.

## Photon Detection Is Destructive Measurement

The projective postulate assumes the system survives to be re-measured. Most photonic measurements do not oblige: a single-photon avalanche diode or SNSPD (Chapter 19) *absorbs* the photon. The measurement is a projection onto photon-number states followed by loss of the system. This is why photonic architectures lean so heavily on **heralding** — measure one photon of a pair to certify its partner (Section 18.3) — and on **teleportation-style** tricks that consume ancilla photons instead of the data photon (Chapter 20). Non-destructive photon detection (quantum non-demolition measurement) exists but requires strong light-matter coupling (Chapter 19's cavity QED).

## Generalized Measurements: POVMs

Projective measurements are not the most general operation permitted by quantum mechanics. If the system is entangled with an ancilla which is then measured projectively, the effective statistics on the system are described by a **positive operator-valued measure (POVM)**: a set of positive operators $\{\hat{E}_m\}$ with $\sum_m \hat{E}_m = \mathbb{1}$ and $P(m) = \mathrm{Tr}(\rho\hat{E}_m)$, where the $\hat{E}_m$ need be neither projectors nor mutually orthogonal. Two photonic workhorses are naturally POVMs: a realistic click detector with efficiency $\eta$ implements $\hat{E}_{\text{click}} = \sum_{n\geq 1}[1-(1-\eta)^n]\,|n\rangle\langle n|$, $\hat{E}_{\text{no click}} = \mathbb{1} - \hat{E}_{\text{click}}$ (no photon-number resolution, just "click/no click"); and unambiguous discrimination of non-orthogonal states — allowed to sometimes answer "don't know" but never to answer wrongly — requires a three-outcome POVM on a qubit, which no projective measurement provides.

## You Cannot Read Out an Unknown State

The Born rule gives one number (one outcome) per photon, and collapse destroys the rest. Determining $\alpha$ and $\beta$ of an unknown qubit therefore requires an *ensemble* of identically prepared copies (tomography, 17.1.2). Given only a **single copy**, the state cannot be identified — and non-orthogonal states cannot be perfectly distinguished, since $|\langle\phi|\psi\rangle| > 0$ means every measurement has overlapping outcome distributions. The optimal error probability for distinguishing two equally likely pure states is the Helstrom bound:

$$P_{\text{err}} = \frac{1}{2}\left(1 - \sqrt{1 - |\langle\phi|\psi\rangle|^2}\right)$$

For BB84's basis states, $|\langle D|H\rangle|^2 = 1/2$ gives $P_{\text{err}} \approx 14.6\%$ — an eavesdropper *cannot* reliably read the key, and her attempts imprint disturbance.

## The No-Cloning Theorem

Could an eavesdropper — or an engineer wanting an optical amplifier — evade this by first *copying* the photon and measuring the copies? No. **An unknown quantum state cannot be copied** (Wootters & Zurek, 1982; Dieks, 1982).

*Proof.* Suppose a unitary $\hat{U}$ cloned two distinct non-orthogonal states onto a blank ancilla $|b\rangle$:

$$\hat{U}|\psi\rangle|b\rangle = |\psi\rangle|\psi\rangle, \qquad \hat{U}|\phi\rangle|b\rangle = |\phi\rangle|\phi\rangle$$

Unitaries preserve inner products, so $\langle\phi|\psi\rangle\langle b|b\rangle = \langle\phi|\psi\rangle^2$, i.e., $\langle\phi|\psi\rangle = \langle\phi|\psi\rangle^2$. This forces $\langle\phi|\psi\rangle = 0$ or $1$: only mutually orthogonal state families can be cloned. A device that copies $|H\rangle$ and $|V\rangle$ (a classical bit copier) is fine; one that also copies $|D\rangle$ is impossible. Linearity alone already forbids it: cloning the basis states forces $\hat{U}(\alpha|H\rangle + \beta|V\rangle)|b\rangle = \alpha|HH\rangle + \beta|VV\rangle$, which is an entangled state, not the product $(\alpha|H\rangle+\beta|V\rangle)^{\otimes 2}$ that cloning demands. $\blacksquare$

The engineering consequences are immense and double-edged. *Against us*: quantum signals cannot be regenerated — no EDFA-style amplifier can restore a qubit stream, which is why loss is the tyrant of Chapters 20 and 22 and why quantum repeaters must be built from entanglement swapping and memory rather than amplification. (Phase-insensitive amplifiers evade nothing: they necessarily add noise, at minimum the equivalent of one vacuum unit, precisely enforcing no-cloning.) *For us*: an eavesdropper cannot copy the key photons either — no-cloning is the bedrock of QKD security. The theorem also explains why measurement-based schemes consume states: information gain and state preservation are rivals, with cloning as the forbidden limit of having both.
