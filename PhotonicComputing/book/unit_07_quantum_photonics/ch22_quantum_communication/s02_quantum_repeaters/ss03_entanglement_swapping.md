# 22.2.3 — Entanglement Swapping, Teleportation, and Repeater Scaling

## The Payload Protocol: Quantum Teleportation

Suppose Alice and Bob already share the Bell pair $|\Phi^+\rangle_{AB} = (|00\rangle + |11\rangle)/\sqrt{2}$, and Alice holds an unknown qubit $|\psi\rangle_C = \alpha|0\rangle + \beta|1\rangle$ to deliver. Rewriting the three-qubit state in the Bell basis of Alice's two qubits $(C, A)$:

$$|\psi\rangle_C |\Phi^+\rangle_{AB} = \frac{1}{2}\Big[ |\Phi^+\rangle_{CA}\,|\psi\rangle_B + |\Phi^-\rangle_{CA}\,\hat{Z}|\psi\rangle_B + |\Psi^+\rangle_{CA}\,\hat{X}|\psi\rangle_B + |\Psi^-\rangle_{CA}\,\hat{X}\hat{Z}|\psi\rangle_B \Big]$$

Alice performs a **Bell-state measurement (BSM)** on $(C, A)$ and telephones Bob her two-bit outcome; Bob applies the corresponding Pauli correction ($\hat{I}, \hat{Z}, \hat{X}$, or $\hat{X}\hat{Z}$) and holds $|\psi\rangle$ *exactly* (Bennett et al., 1993 [1]). Note what happened: the unknown state crossed from Alice to Bob although no physical carrier of it did — the entanglement was the channel, and it is consumed in the act. No-cloning is respected (Alice's copy is destroyed by her measurement), and no signal travels faster than light (without the two classical bits, Bob's qubit is maximally mixed). Teleportation of photonic qubits was demonstrated by the Innsbruck group in 1997 [2], and its CV cousin — teleporting field quadratures using two-mode squeezing, Chapter 21 — unconditionally in 1998.

For repeaters the moral is: *once end-to-end entanglement exists, transmitting qubits is a solved problem.* The entire difficulty concentrates in creating that entanglement over distance.

## Entanglement Swapping: Teleporting Entanglement Itself

Now the repeater's core move. Let node $A$ share a Bell pair with middle node $M$ (qubits $A, M_1$), and $M$ share another with $B$ (qubits $M_2, B$) — each pair created over a *short*, survivable segment. The middle node performs a BSM on its two qubits $M_1, M_2$. Expanding $|\Phi^+\rangle_{AM_1}|\Phi^+\rangle_{M_2B}$ in the Bell basis of $(M_1, M_2)$:

$$|\Phi^+\rangle_{AM_1}|\Phi^+\rangle_{M_2B} = \frac{1}{2}\Big[ |\Phi^+\rangle_{M_1M_2}|\Phi^+\rangle_{AB} + |\Phi^-\rangle_{M_1M_2}|\Phi^-\rangle_{AB} + |\Psi^+\rangle_{M_1M_2}|\Psi^+\rangle_{AB} + |\Psi^-\rangle_{M_1M_2}|\Psi^-\rangle_{AB} \Big]$$

Whatever outcome $M$ announces, $A$ and $B$ now hold a known Bell state (converted to $|\Phi^+\rangle$ by local Paulis) — **entangled, though their photons never met** (Żukowski, Zeilinger, Horne & Ekert, 1993 [3]; first demonstrated by Pan et al., 1998 [4]). Swapping is precisely teleportation where the "payload" is itself half of another entangled pair. Chain it: pairs over $n$ adjacent segments, BSMs at the $n-1$ intermediate nodes, and the two end nodes share entanglement across the full span. With photonic BSMs, remember the Chapter 20 constraint: linear optics distinguishes only two of the four Bell states, capping each swap's success at 50% unless ancillas or matter qubits perform the measurement deterministically.

Imperfection compounds. For Werner-type pairs of fidelity $F$, one swap yields roughly $F' \approx F^2$ (plus small terms): two 95% segments swap to $\sim 90.5\%$, and a 20-segment chain of 99% pairs would end near $82\%$ — below the QKD threshold. Enter **entanglement purification** (Bennett et al., 1996 [5]; Deutsch et al., 1996 [6]): consume two noisy copies of a pair to produce, probabilistically, one pair of higher fidelity, using only local CNOTs, measurements, and two-way classical communication (keep the pair when the check measurements agree). Iterated ("entanglement pumping"), purification converts polynomially many mediocre pairs into one excellent pair, provided local gate errors are small and — the recurring toll — the memories hold everything while classical confirmations fly back and forth. The original BDCZ repeater architecture [7] is exactly this sandwich: swap to extend, purify to repair, nested level by level.

## The Rate Arithmetic: Why Repeaters Win

**Direct transmission** over $L = 1{,}000$ km: $\eta = 10^{-\alpha L/10} = 10^{-20}$ at $\alpha = 0.2$ dB/km. A perfect 10 GHz single-photon source delivers $10^{10} \times 10^{-20} = 10^{-10}$ successes per second — one photon every ~317 years. The PLOB bound certifies that no repeaterless protocol does fundamentally better than $O(\eta)$.

**Segmented distribution** over $n = 20$ segments of $L_0 = 50$ km: each attempt succeeds with $p_0 \sim \eta_0 = 10\%$ (times source/detector/memory factors), and each attempt costs the heralding time $\sim L_0/c_{\text{fiber}} = 250\ \mu$s. A segment therefore succeeds in $\sim 2.5$ ms on average — *and, crucially, segments retry independently, in parallel*, with successes parked in memory. End-to-end entanglement then costs a polynomial overhead: waiting for all segments (a coupon-collector logarithmic factor, or geometric compounding if swaps are probabilistic), plus swap and purification rounds. Realistic architecture studies land at end-to-end rates of order Hz–kHz for 1,000 km — versus $10^{-10}$ Hz direct: **twelve or more orders of magnitude**, purchased with memories and local logic rather than with brighter lasers. The general structure of the tradeoff is captured by capacity theory: cutting a lossy channel into $n+1$ segments raises the end-to-end ceiling from $-\log_2(1-\eta)$ to $-\log_2(1-\eta^{1/(n+1)})$ [8] — each added node takes a root of the loss, exactly the $\sqrt{\eta}$ effect that twin-field QKD (one untrusted, memoryless node) already exhibits.

The fine print defines the research agenda. Memories must survive the *end-to-end* classical coordination time ($\sim 10$ ms per 1,000 km round trip, times retries) — hence the ms–s storage targets of 22.2.2. Multimode memories multiply $p_0$'s effective attempt rate. Deterministic BSMs (matter qubits) avoid the 50% linear-optics tax. And a radical alternative — the **all-photonic repeater** [9] — replaces memories entirely with large photonic graph states whose built-in redundancy lets a node "measure through" loss, trading matter memories for the massive multiplexed photon sources of Chapter 20's fusion architectures.

## State of the Art

Every ingredient is demonstrated; no full chain yet is. Milestones: entanglement swapping with photons (1998 [4]); heralded entanglement between matter nodes — NV centers 1.3 km apart — enabling the 2015 loophole-free Bell test [10]; atomic-ensemble memories entangled through tens of kilometers of field fiber with telecom conversion (2020 [11]); telecom-heralded entanglement of two solid-state AFC memories (2021, 22.2.2); a memory node *beating* the direct-transmission bound (SiV, 2020, 22.2.2); and teleportation between *non-adjacent* nodes of a three-node NV network — i.e., across a genuine swap — in 2022 [12]. The gap to a deployed repeater is quantitative, not conceptual: efficiencies, rates, and storage times each need roughly an order of magnitude, simultaneously.

## Summary

- Teleportation consumes a shared Bell pair + 2 classical bits to move an unknown qubit; the payload never crosses the channel.
- Entanglement swapping is teleportation of entanglement: a BSM at a middle node entangles end nodes that never interacted; linear-optics BSMs succeed at most 50%.
- Fidelity multiplies down chains ($F' \approx F^2$ per swap); purification (BBPSSW/DEJMPS) probabilistically distills better pairs from noisy ones at the cost of copies, local gates, and two-way classical communication.
- Rates: direct $O(\eta)$ (one photon per centuries at 1,000 km) versus polynomial scaling for segmented, memory-assisted distribution; $n$ nodes lift the capacity ceiling to $-\log_2(1-\eta^{1/(n+1)})$.
- All components (swapping, purification, memory nodes, telecom conversion, multi-node teleportation) are individually demonstrated; integration at rate is the open frontier.

---

*References*

[1] Bennett, C.H., Brassard, G., Crépeau, C., Jozsa, R., Peres, A., & Wootters, W.K. (1993). Teleporting an unknown quantum state via dual classical and Einstein-Podolsky-Rosen channels. *Physical Review Letters*, 70(13), 1895–1899. [DOI: 10.1103/PhysRevLett.70.1895]

[2] Bouwmeester, D., Pan, J.-W., Mattle, K., Eibl, M., Weinfurter, H., & Zeilinger, A. (1997). Experimental quantum teleportation. *Nature*, 390, 575–579. [DOI: 10.1038/37539]

[3] Żukowski, M., Zeilinger, A., Horne, M.A., & Ekert, A.K. (1993). "Event-ready-detectors" Bell experiment via entanglement swapping. *Physical Review Letters*, 71(26), 4287–4290. [DOI: 10.1103/PhysRevLett.71.4287]

[4] Pan, J.-W., Bouwmeester, D., Weinfurter, H., & Zeilinger, A. (1998). Experimental entanglement swapping: Entangling photons that never interacted. *Physical Review Letters*, 80(18), 3891–3894. [DOI: 10.1103/PhysRevLett.80.3891]

[5] Bennett, C.H., Brassard, G., Popescu, S., Schumacher, B., Smolin, J.A., & Wootters, W.K. (1996). Purification of noisy entanglement and faithful teleportation via noisy channels. *Physical Review Letters*, 76(5), 722–725. [DOI: 10.1103/PhysRevLett.76.722] [BBPSSW.]

[6] Deutsch, D., Ekert, A., Jozsa, R., Macchiavello, C., Popescu, S., & Sanpera, A. (1996). Quantum privacy amplification and the security of quantum cryptography over noisy channels. *Physical Review Letters*, 77(13), 2818–2821. [DOI: 10.1103/PhysRevLett.77.2818] [DEJMPS.]

[7] Briegel, H.-J., Dür, W., Cirac, J.I., & Zoller, P. (1998). Quantum repeaters: The role of imperfect local operations in quantum communication. *Physical Review Letters*, 81(26), 5932–5935. [DOI: 10.1103/PhysRevLett.81.5932]

[8] Pirandola, S. (2019). End-to-end capacities of a quantum communication network. *Communications Physics*, 2, 51. [DOI: 10.1038/s42005-019-0147-3]

[9] Azuma, K., Tamaki, K., & Lo, H.-K. (2015). All-photonic quantum repeaters. *Nature Communications*, 6, 6787. [DOI: 10.1038/ncomms7787]

[10] Hensen, B., et al. (2015). Loophole-free Bell inequality violation using electron spins separated by 1.3 kilometres. *Nature*, 526, 682–686. [DOI: 10.1038/nature15759]

[11] Yu, Y., et al. (2020). Entanglement of two quantum memories via fibres over dozens of kilometres. *Nature*, 578, 240–245. [DOI: 10.1038/s41586-020-1976-7]

[12] Hermans, S.L.N., Pompili, M., Beukers, H.K.C., Baier, S., Borregaard, J., & Hanson, R. (2022). Qubit teleportation between non-neighbouring nodes in a quantum network. *Nature*, 605, 663–668. [DOI: 10.1038/s41586-022-04697-y]
