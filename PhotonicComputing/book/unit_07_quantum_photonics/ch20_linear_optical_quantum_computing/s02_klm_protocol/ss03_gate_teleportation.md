# 20.2.3 Gate Teleportation and Success Boosting

## Move the Gamble Offline

The problem at the end of Section 20.2.2: a 1/16-probability gate that *destroys its inputs on failure* cannot be applied directly to precious data qubits. The solution is borrowed from Gottesman and Chuang (1999): **gate teleportation**.

Recall quantum teleportation: a Bell measurement on the data qubit and half of an entangled pair transfers the data state onto the other half, up to a known Pauli correction. Gottesman-Chuang's twist: if you want to apply a gate $V$ to the data, you may instead apply $V$ *to the entangled resource before teleporting*. For a CZ on two qubits, teleport both through Bell pairs whose far halves have *already* been acted on by a CZ:

$$|\text{resource}\rangle = (I \otimes CZ \otimes I)\,\big(|\Phi^+\rangle \otimes |\Phi^+\rangle\big).$$

Because CZ commutes (up to Paulis) with the teleportation corrections, the teleported qubits emerge with the CZ applied. The consequence for linear optics is transformative:

**The probabilistic CZ never touches data.** A factory attempts the 1/16 gate on ancilla photons, over and over, *offline*. Failures cost nothing but ancilla photons. Each success is stored (a delay line suffices, briefly) as a certified resource state. Data qubits interact only with certified resources, via teleportation.

The gamble has not disappeared — it has moved into the teleportation step, because Bell measurement itself cannot be done deterministically with linear optics.

## The Linear-Optics Bell Measurement Bottleneck

A complete Bell-state measurement distinguishes the four states $|\Phi^\pm\rangle, |\Psi^\pm\rangle$. With linear optics and photon counting (no ancillas), only **two of the four** can be identified — the standard circuit (50/50 splitter + polarization analysis) identifies $|\Psi^+\rangle$ and $|\Psi^-\rangle$ and confuses the two $|\Phi\rangle$ states — for a maximum success probability of

$$P_{BSM} = \tfrac{1}{2}$$

(Calsamiglia & Lütkenhaus, 2001, proved this bound). Teleportation through a Bell pair with a linear-optics BSM therefore succeeds with probability 1/2. Two lines of attack raise it:

**KLM's boosted teleportation.** Replace the Bell pair with the $2n$-mode entangled ancilla

$$|t_n\rangle = \frac{1}{\sqrt{n+1}}\sum_{j=0}^{n} |1\rangle^{\otimes j}|0\rangle^{\otimes (n-j)} \otimes |0\rangle^{\otimes j}|1\rangle^{\otimes (n-j)},$$

and measure the data mode together with the first $n$ ancilla modes in an $(n+1)$-mode Fourier-transform interferometer. If the detectors count $k$ photons total with $0 < k \leq n$, the data state reappears in ancilla mode $n+k$, up to a known phase correction (feed-forward). Failure occurs only for the outcomes $k = 0$ or $k = n+1$, which reveal (and hence destroy) the data amplitude — with probability

$$P_{fail} = \frac{1}{n+1}, \qquad P_{success} = \frac{n}{n+1}.$$

A CZ teleported through two such ancillas succeeds with probability $\left(\frac{n}{n+1}\right)^2$: 44% for $n=1$ (the basic Bell case), 82% for $n = 9$, 98% for $n = 99$. Arbitrarily close to deterministic — *using only linear optics, photon counting, and increasingly baroque ancilla states*. This is the theorem: linear optical quantum computing is scalable, with polynomial resource overhead.

**Boosted Bell measurement.** Alternatively, keep ordinary teleportation but improve the BSM with ancillas: an ancillary Bell pair raises the success probability to 3/4 (Grice, 2011), as do four unentangled single photons (Ewert & van Loock, 2014); iterating approaches 1. These "boosted fusion" measurements reappear as the workhorse of fusion-based architectures (Section 20.3.3).

Failure, in both schemes, is *heralded and benign in direction*: a failed teleportation measures the data qubit in the computational basis — an erasure at a known location, feeding forward into the loss-tolerant codes of Section 20.5.

## The Cost, Honestly Accounted

Everything above is polynomial — and brutally expensive. The ancilla $|t_n\rangle$ is itself an $n$-photon entangled state that must be built from probabilistic gates; error correction against teleportation failure (KLM layered a parity code atop $n = 1$ teleportation, then concatenated) multiplies photon counts again. Contemporary estimates put original-flavor KLM at $10^4$–$10^5$ physical operations and ancilla photons per high-fidelity two-qubit gate — numbers that made the protocol a proof of principle rather than a machine. The essential legacy is architectural:

1. **Offline resource-state factories + teleportation** — nondeterminism quarantined away from data;
2. **Feed-forward** — measurement outcomes steering later optics in real time, demonstrated at the requisite nanosecond scales by Prevedel et al. (2007) with switched fiber optics;
3. **Heralded failure as erasure** — the error model that photonic codes are designed around.

Cluster-state MBQC (next section) kept all three pillars and slashed the overhead by orders of magnitude — Nielsen's 2004 hybrid already cut resource counts ~50-fold — by realizing that if teleportation is how gates happen, the entire computation might as well *be* teleportation through one big entangled state.
