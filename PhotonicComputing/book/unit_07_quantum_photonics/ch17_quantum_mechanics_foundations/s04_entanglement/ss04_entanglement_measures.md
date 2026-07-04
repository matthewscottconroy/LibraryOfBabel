# 17.4.4 Entanglement Measures

## From Phenomenon to Resource

Once entanglement is being generated on demand, distributed through fibers, and consumed by teleportation and fusion gates, "is it entangled?" stops being the right question. The working questions become *how much* entanglement a state carries, how entanglement degrades under loss and noise, and how many high-quality Bell pairs can be distilled from many noisy ones. Entanglement theory answers with **measures**: functions of the state that cannot increase under local operations and classical communication (**LOCC**) — the operations available to separated parties — and that vanish exactly on separable states. The LOCC-monotonicity requirement is what makes a measure meaningful: whatever entanglement is, two labs connected only by a classical channel cannot create it.

## Pure States: Entanglement Entropy

For a bipartite pure state, one measure rules by theorem. Take the Schmidt decomposition (17.4.1), $|\psi\rangle = \sum_k \lambda_k |u_k\rangle|v_k\rangle$, and compute the **von Neumann entropy of either reduced state**:

$$E(|\psi\rangle) = S(\rho_A) = -\mathrm{Tr}\left(\rho_A \log_2 \rho_A\right) = -\sum_k \lambda_k^2 \log_2 \lambda_k^2 = S(\rho_B)$$

Product state: one Schmidt coefficient, $E = 0$. Bell state: coefficients $(1/\sqrt{2}, 1/\sqrt{2})$, $E = 1$ — one **ebit**, the natural unit. The operational meaning is exact and beautiful (Bennett et al., 1996): asymptotically, $n$ copies of $|\psi\rangle$ can be *distilled* by LOCC into $\approx nE$ Bell pairs, and $\approx nE$ Bell pairs suffice to *prepare* $n$ copies. Entanglement entropy is a conversion rate, like an exchange rate into a hard currency.

**Worked example.** For the partially entangled state $|\psi(\theta)\rangle = \cos\theta\,|00\rangle + \sin\theta\,|11\rangle$:

$$\rho_A = \begin{pmatrix}\cos^2\theta & 0 \\ 0 & \sin^2\theta\end{pmatrix}, \qquad E(\theta) = -\cos^2\theta \log_2 \cos^2\theta - \sin^2\theta\log_2\sin^2\theta$$

$E$ rises from 0 (at $\theta = 0$: product) to 1 (at $\theta = 45°$: Bell state) and back. At $\theta = 30°$: $\cos^2\theta = 0.75$, $E = -(0.75)(\log_2 0.75) - (0.25)(\log_2 0.25) = 0.311 + 0.5 = 0.811$ ebits. Such non-maximally entangled states are not merely defective Bell pairs — tuning $\theta$ is exactly what the Eberhard-inequality Bell tests of 2015 did to lower the detection-efficiency threshold, and what heralded photonic gates tune for optimal success probability.

## Mixed States: The Realistic Zoo

Real distributed states are mixed (loss, background counts, phase drift), and for mixed states no single measure suffices; several standard ones coexist, each operationally motivated:

**Entanglement of formation** $E_F(\rho)$: the minimum average pure-state entanglement over all ensemble decompositions of $\rho$ — the ebit cost to *build* the state. For two qubits it is exactly computable through Wootters' **concurrence** $C$ (1998):

$$E_F = h\!\left(\frac{1 + \sqrt{1 - C^2}}{2}\right), \qquad h(x) = -x\log_2 x - (1-x)\log_2(1-x)$$

with $C(\rho) = \max(0,\ \mu_1 - \mu_2 - \mu_3 - \mu_4)$, where $\mu_i$ are the decreasing square roots of the eigenvalues of $\rho(\hat{\sigma}_y\otimes\hat{\sigma}_y)\rho^*(\hat{\sigma}_y\otimes\hat{\sigma}_y)$. Concurrence runs from 0 (separable) to 1 (Bell state) and is the number photonic tomography papers most often quote.

**Negativity** $\mathcal{N}(\rho)$: based on the **Peres-Horodecki criterion** (1996) — transpose subsystem B's indices (partial transpose). For any separable state the result is a valid state; negative eigenvalues of $\rho^{T_B}$ therefore certify entanglement. Negativity sums them: $\mathcal{N} = (\|\rho^{T_B}\|_1 - 1)/2$. For two qubits (and qubit-qutrit), PPT is *necessary and sufficient*; in larger systems there exist entangled states with positive partial transpose — **bound entanglement**, undistillable yet not separable, a reminder that the resource theory has genuine fine structure.

**Distillable entanglement** $E_D(\rho)$: the rate of Bell pairs extractable by LOCC — the measure quantum repeaters (Chapter 22) actually care about, since noisy channel-distributed pairs must be purified before use. In general $E_D \leq E_F$: entanglement, like energy, is cheaper to waste than to recover.

**Worked example (Werner state).** Mix a singlet with white noise: $\rho_W = p\,|\Psi^-\rangle\langle\Psi^-| + (1-p)\,\mathbb{1}/4$ — the standard model of a Bell pair after depolarizing transmission. Diagnostics as $p$ decreases:

- Entangled (PPT violated) iff $p > 1/3$; concurrence $C = \max(0, (3p-1)/2)$.
- Violates CHSH only for $p > 1/\sqrt{2} \approx 0.707$.

So for $1/3 < p \leq 1/\sqrt{2}$ the state is *entangled but Bell-local* — entanglement and nonlocality are inequivalent resources, and a link can be quantum-correlated yet useless for device-independent protocols. Fidelity budgeting in entanglement distribution is precisely the management of $p$ against these thresholds.

## Design Consequences for Photonic Systems

Three habits of thought follow from the resource view, and they recur through Chapters 20–22:

1. **Entanglement is budgeted, not just detected.** A fusion-based quantum computer consumes ebits (in resource states) per logical operation; a repeater chain consumes ebits per swap. Architectures are costed in ebits the way classical designs are costed in gate-count.
2. **Loss converts pure entanglement into mixed entanglement**, and measures quantify the exchange rate. A polarization Bell pair sent through 20 km of fiber arrives (when it arrives) with reduced concurrence from multipair emission and background — the source brightness/fidelity tradeoff of Section 18.3.1.
3. **Monogamy**: a qubit maximally entangled with one partner cannot be entangled with any other (quantified for qubits by the Coffman-Kundu-Wootters inequality $C_{A|B}^2 + C_{A|C}^2 \leq C_{A|BC}^2$). Monogamy is why entanglement-based QKD is secure — correlations Eve shares are correlations Alice and Bob demonstrably lack — and why cluster states (Chapter 20) spread entanglement thinly but topologically, rather than concentrating it pairwise.

With states, dynamics, quantized fields, and entanglement in hand, the foundations are complete. Chapter 18 takes this machinery into the laboratory, where beam splitters, correlators, and nonlinear crystals turn it into measured curves.
