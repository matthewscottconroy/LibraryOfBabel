# 22.3.1 — The Quantum Internet Roadmap

## Stages Defined by What the End Nodes Can Do

A quantum network is easy to hype and hard to specify: is a metropolitan QKD ring a "quantum internet"? Is Micius? Wehner, Elkouss, and Hanson (2018) [1] cut through the ambiguity with a functional taxonomy whose organizing variable is *the capability of the end nodes* — not the distance spanned, the number of users, or the raw entanglement rate, but what a client at the edge can actually *do* with the quantum states delivered to it. Each stage enables strictly more applications than the last, and each demands strictly more of the hardware. The ladder has six rungs.

**Stage 1 — Trusted-repeater (trusted-node) networks.** The end nodes only prepare and measure single qubits over each individual link; there is no end-to-end quantum connection. Long distances are bridged by *trusted* intermediate nodes that receive a key on one link, hold it in classical memory, and re-encrypt it onto the next — the key exists in the clear inside every relay. This is the only stage deployed at scale today: the 2,000 km Beijing–Shanghai backbone chains 32 trusted nodes; Micius, operated as a trusted relay, linked Beijing and Vienna across 7,600 km (2018) [3]; and the integrated space-to-ground network of 2021 fused both into a 4,600 km system serving over 150 users [2]. The capability is genuine QKD between any two users; the price is that every relay is a point of total trust.

**Stage 2 — Prepare-and-measure networks.** The end nodes still only prepare and measure (no quantum memory), but the network now supports *end-to-end* QKD in which no intermediate node need be trusted. The enabling primitive is an untrusted measurement node: MDI-QKD (Section 22.1.2), where Alice and Bob send states to a relay that performs a Bell measurement it cannot exploit. Any two users share provably secret bits even if the entire network core is adversarial. Field MDI-QKD over metropolitan fibre realizes this rung; it removes the trusted-node liability of Stage 1 without yet requiring anyone to *store* a qubit.

**Stage 3 — Entanglement-distribution networks.** The network delivers genuine end-to-end *entanglement*; end nodes receive and immediately measure entangled qubits (still no storage). Because the shared state can be Bell-tested, this rung unlocks *device-independent* protocols — DI-QKD, whose security is certified by a CHSH violation without trusting the devices at all (Section 22.1.1) [5]. Micius distributing entangled pairs to ground stations 1,120–1,203 km apart and running BBM92 with an untrusted source [4] demonstrates the entanglement-distribution capability at continental scale; laboratory DI-QKD was shown in 2022 with trapped ions, neutral atoms, and photons.

**Stage 4 — Quantum-memory networks.** End nodes now hold a quantum memory: they can *store* received entanglement and apply local gates. This is the qualitative jump from communication to networked *computation* — deterministic qubit teleportation between nodes, entanglement swapping on demand, simple distributed algorithms, and blind quantum computing all become possible. The Delft three-node NV-centre network (Pompili et al., 2021) [6] and its sequel — qubit teleportation between the two *non-adjacent* nodes across an intermediate swap (Hermans et al., 2022) [7] — are the first systems to exercise Stage-4 primitives end to end, albeit over metres rather than kilometres. The memory-enhanced SiV link that beat the direct-transmission bound (Section 22.2.2) is a Stage-4 building block.

**Stage 5 — Few-qubit fault-tolerant networks.** End nodes become small error-corrected quantum computers holding a handful of *logical* qubits. Operations cross the fault-tolerance threshold, so protocols run at arbitrary fidelity and depth: high-quality distributed gates, leader election, Byzantine agreement, and clock networks limited only by the logical error rate. No such network yet exists; it awaits logical qubits carrying a network interface.

**Stage 6 — Quantum-computing networks.** End nodes are full fault-tolerant quantum computers, and the network delivers entanglement fast and clean enough to knit them into a single distributed machine — the endpoint Kimble envisioned, with no essential distinction between local and remote qubits. Timelines are speculative (Stages 5–6 are widely placed decades out), but the ladder's point is that each rung is independently useful, so the internet is built incrementally rather than awaited whole.

The chapter outline's five-stage sketch (QKD → entanglement distribution → memory networks → logical qubits → distributed computing) is a coarse-graining of this scheme; the six-stage Wehner–Elkouss–Hanson version is the field standard because it separates the two distinct trust/hardware transitions the coarse version blurs — Stage 1→2 (removing trusted nodes) and Stage 2→3 (adding real entanglement).

## Placing Systems on the Ladder

**Worked example — where each system sits, and the cost of trust.** *Where does each system sit?* (i) Micius relaying Beijing–Vienna keys is **Stage 1**: the satellite holds both keys in the clear and must be trusted. (ii) The *same* satellite distributing entangled pairs for BBM92 over 1,120 km is **Stage 3** in capability — real entanglement, untrusted source — but the ground nodes only measure, so it does not reach Stage 4. (iii) The Delft three-node network is **Stage 4**: nodes store qubits and teleport across a swap, even though it spans a lab bench — illustrating that stage is set by *functionality, not distance*. (iv) A metropolitan ring using trusted relays is **Stage 1**; swap those relays for MDI nodes and it becomes **Stage 2**.

Why the Stage 1→3 jump matters *quantitatively*: a trusted-node chain is secure only if *every* relay is secure. For $N$ independent relays each compromised with probability $p$, end-to-end security survives with probability $(1-p)^N$. The 32-node Beijing–Shanghai backbone at a modest $p = 0.01$ per node gives

$$(1-p)^N = (0.99)^{32} \approx 0.725,$$

a $\sim 27.5\%$ chance that *some* node leaks the key — and the leak is catastrophic, since the key sits in plaintext at that node. A Stage-3 entanglement-distribution network needs to trust *zero* intermediate nodes: monogamy of entanglement guarantees that a Bell pair certified between the end nodes is correlated with nothing in the network core, whatever the relays did. That single change in trust model — not any change in key rate — is the entire reason to climb from Stage 1 to Stage 3.

## Summary

- The roadmap ranks networks by *end-node capability*, not distance or rate; six stages, each enabling strictly more applications and demanding strictly more hardware [1].
- Stage 1 (trusted-repeater) is deployed now — Beijing–Shanghai's 32 nodes, Micius's 7,600 km relay, the 4,600 km integrated network — at the cost of trusting every relay.
- Stage 2 (prepare-and-measure, MDI relays) removes trusted nodes; Stage 3 (entanglement distribution) adds real end-to-end entanglement and device-independent security, reached in capability by Micius's 1,120 km BBM92.
- Stage 4 (quantum memory) turns communication into networked computation — teleportation between nodes, blind computing — first exercised by the Delft NV network; Stages 5–6 (few-qubit fault-tolerant, then full quantum-computing networks) remain future work.
- A 32-node trusted chain at $p = 1\%$ per node carries a $\sim 27\%$ end-to-end compromise probability; entanglement networks trust zero relays — the quantitative payoff of climbing the ladder.

---

*References*

[1] Wehner, S., Elkouss, D., & Hanson, R. (2018). Quantum internet: A vision for the road ahead. *Science*, 362(6412), eaam9288. [DOI: 10.1126/science.aam9288]

[2] Chen, Y.-A., et al. (2021). An integrated space-to-ground quantum communication network over 4,600 kilometres. *Nature*, 589, 214–219. [DOI: 10.1038/s41586-020-03093-8]

[3] Liao, S.-K., et al. (2018). Satellite-relayed intercontinental quantum network. *Physical Review Letters*, 120(3), 030501. [DOI: 10.1103/PhysRevLett.120.030501] [Beijing–Vienna.]

[4] Yin, J., et al. (2020). Entanglement-based secure quantum cryptography over 1,120 kilometres. *Nature*, 582, 501–505. [DOI: 10.1038/s41586-020-2401-y]

[5] Pirandola, S., Andersen, U.L., Banchi, L., et al. (2020). Advances in quantum cryptography. *Advances in Optics and Photonics*, 12(4), 1012–1236. [DOI: 10.1364/AOP.361502] [Reviews device-independent QKD and the protocol landscape.]

[6] Pompili, M., Hermans, S.L.N., Baier, S., et al. (2021). Realization of a multinode quantum network of remote solid-state qubits. *Science*, 372(6539), 259–264. [DOI: 10.1126/science.abg1919]

[7] Hermans, S.L.N., Pompili, M., Beukers, H.K.C., Baier, S., Borregaard, J., & Hanson, R. (2022). Qubit teleportation between non-neighbouring nodes in a quantum network. *Nature*, 605, 663–668. [DOI: 10.1038/s41586-022-04697-y]
