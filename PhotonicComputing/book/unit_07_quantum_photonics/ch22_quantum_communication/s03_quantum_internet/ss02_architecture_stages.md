# 22.3.2 — Network Architecture and the Protocol Stack

## Entanglement as the Network Resource

A classical network moves *packets*; a quantum network's fundamental deliverable is a *shared Bell pair* of specified fidelity between two named nodes. Everything an application wants — a QKD bit, a teleported qubit, a distributed gate — is synthesized from that one resource (Section 22.2.3) [1]. This inverts the usual picture: the network does not transport the user's qubit at all (no-cloning forbids copying it, loss forbids sending it far); instead it manufactures generic entanglement, and the user *teleports* the payload across it locally. Designing the quantum internet is therefore the problem of producing, storing, routing, and purifying entanglement reliably on top of hardware whose every operation is probabilistic and whose memories decohere while they wait.

## The Protocol Stack

Borrowing the layering discipline of classical networking — while respecting that the quantum service is entirely different — a consensus stack has four layers [2, 3]:

- **Physical layer.** A single link attempts to generate heralded entanglement between adjacent nodes' memories. The workhorse is *midpoint heralding*: each node emits a photon entangled with its stationary qubit, the two photons meet at a station between them, and a Bell measurement there announces success without revealing the state (the single-click and two-photon schemes of DLCZ and NV entanglement, Section 22.2.2). Output: raw, sometimes low-fidelity entangled pairs, produced when they happen to succeed.

- **Link layer.** Converts the physical layer's stochastic trickle into a *robust service*: "deliver a Bell pair across this one link, on request, meeting a fidelity target." Dahlberg et al. (2019) [3] designed and implemented the first such protocol — a queue of entanglement requests, scheduling of generation attempts, fidelity estimation, and a hardware-abstracting interface — on NV-centre nodes. This is the layer that makes entanglement a *reliable primitive* rather than a lucky event.

- **Network layer.** Produces entanglement between *non-adjacent* nodes by chaining links with entanglement swapping (Section 22.2.3), and chooses *which* chain — entanglement *routing*. A path of $n$ links plus $n-1$ swaps yields one end-to-end pair.

- **Transport layer.** Delivers qubits (or streams of pairs at guaranteed fidelity) end to end, invoking purification and managing the rate/fidelity budget the application requested.

The analogy to TCP/IP is deliberate but imperfect: there is no retransmission of an unknown qubit (no-cloning), "buffering" means a decohering quantum memory with a hard time budget, and the resource is *consumed* on use.

## Entanglement Routing

Routing entanglement is not shortest-path routing [7]. Three quantum facts reshape it. First, links are *probabilistic and rate-limited*, so a path's throughput is set by its worst link, not its hop count. Second, each swap can both *fail* — probabilistically, for linear-optics Bell measurements, at the 50% ceiling of Chapter 20 — and *degrade fidelity* ($F' \approx F^2$ per swap, Section 22.2.3), so more hops means lower fidelity and more purification. Third, memories decohere: a path is viable only if all its links can be loaded and swapped *within the memory coherence time*, coupling routing to a stopwatch. A router therefore optimizes end-to-end *rate × fidelity* under a time budget, choosing among multi-path distribution, purification depth, and swap ordering — a genuinely new networking problem with no classical analogue.

## End-Node Platforms

The stack runs on stationary qubits at the end nodes, and the platform choice (extending the memory scorecard of Section 22.2.2) fixes the achievable rate and fidelity:

- **NV / SiV centres in diamond.** A photonic interface plus long-lived nuclear-spin memory and local gates — the platform of the Delft network. SiV in a nanocavity gives strong photon coupling; NV gives minute-scale nuclear-spin storage. Their optical transitions (637 / 737 nm) need frequency conversion to telecom.
- **Trapped ions.** The best gate fidelities and coherence times, full on-node processing, and demonstrated ion–photon entanglement; slower, and requiring conversion from visible to telecom.
- **Neutral atoms (Rydberg tweezer arrays).** Rapidly scaling qubit counts with reconfigurable connectivity — attractive as many-qubit end nodes.
- **Atomic ensembles / rare-earth AFC crystals.** No on-node logic, but massive *multimode* capacity that multiplies the entanglement attempt rate — ideal for the high-rate link segments rather than the processing nodes.

Real architectures mix them: ensemble/AFC memories where rate and multiplexing dominate, single emitters where the node must *compute*.

## Demonstrators

The Delft three-node NV network (Pompili et al., 2021) [4] is the reference implementation: three nodes, physical- and link-layer entanglement generation, memory storage on ¹³C nuclear spins, and enough control to run network-layer primitives. Its successor teleported a qubit between the two non-neighbouring nodes across an intermediate swap (Hermans et al., 2022) [5] — the first end-to-end demonstration of the Stage-4 stack (Section 22.3.1). Around it a testbed ecosystem is forming: the Quantum Internet Alliance in Europe, metropolitan entanglement testbeds around Boston, Chicago, and Delft–The Hague, and China's fibre-plus-satellite infrastructure — each a proving ground for the layers above.

**Worked example — an entanglement-rate budget along a routed path.** Distribute entanglement over $L = 300$ km, comparing *direct* transmission with a routed 3-link path (nodes A–R₁–R₂–B, each link 100 km, midpoint-heralded). Assume an attempt clock $f_c = 1$ MHz, deterministic matter-qubit swaps ($p_{\text{swap}} \approx 1$), and memories that outlast the $\sim 3$ ms end-to-end coordination time.

Each 100 km link heralds at its midpoint, so each photon travels 50 km: survival $\eta = 10^{-0.2 \times 50 / 10} = 10^{-1} = 0.1$. For a two-photon heralding scheme with a linear-optics Bell station (success factor $\tfrac{1}{2}$), the per-attempt link success is

$$p_{\text{link}} = \tfrac{1}{2}\,\eta^2 = \tfrac{1}{2}(0.1)^2 = 5\times 10^{-3},$$

so one link becomes "ready" at $f_c\,p_{\text{link}} = 5$ kHz (mean load time 200 µs). All three links must be loaded *simultaneously* before swapping. For $n$ parallel links, each ready per cycle with probability $p$, the expected number of cycles until all are ready is $Z_n / p$ in the small-$p$ limit, with $Z_n = \sum_{k=1}^{n}(-1)^{k+1}\binom{n}{k}/k$; here $Z_3 = 3 - \tfrac{3}{2} + \tfrac{1}{3} = \tfrac{11}{6}$ [6]. Hence

$$E[T] = \frac{Z_3}{f_c\,p_{\text{link}}} = \frac{11/6}{10^{6}\times 5\times 10^{-3}} \approx 3.7\times 10^{-4}\ \text{s},$$

and with deterministic swaps the end-to-end pair arrives at $R \approx 2.7$ kHz. Compare direct transmission over the full 300 km: $\eta_{\text{dir}} = 10^{-0.2\times 300/10} = 10^{-6}$, giving $f_c\,\eta_{\text{dir}} = 1$ pair/s (and PLOB-limited). The routed path wins by more than $10^3$. Two caveats sharpen the routing lesson: if the swaps were probabilistic linear-optics measurements ($p_{\text{swap}} = \tfrac12$ each), the two swaps cost a further factor $\tfrac14$, dropping $R$ to $\sim 0.7$ kHz; and each swap multiplies infidelity ($F' \approx F^2$), so a *fourth* hop — though it would raise per-link $\eta$ — might *lower* delivered fidelity below the QKD threshold. Routing selects the path that maximizes rate × fidelity within the memory clock, not the one with the fewest kilometres or hops.

## Summary

- The quantum internet's resource is the shared Bell pair; the network manufactures generic entanglement and the user teleports payloads across it — it never transports the unknown qubit (no-cloning, loss).
- A four-layer stack — physical (heralded link attempts), link (a robust "entangle this link on request" service, first built by Dahlberg et al. on NV hardware), network (swapping + entanglement routing), transport (end-to-end delivery + purification) — organizes the design, analogous to but distinct from TCP/IP.
- Entanglement routing optimizes rate × fidelity under a memory-time budget; probabilistic links, failing and fidelity-degrading swaps, and decoherence make it a new problem.
- End-node platforms trade processing for rate: NV/SiV and trapped ions compute, ensembles/AFC multiplex; real networks mix them.
- The Delft three-node NV network and its non-neighbour teleportation are the first integrated Stage-4 demonstrators; testbeds (QIA, metro networks, China's satellite-fibre system) are scaling the stack.

---

*References*

[1] Kimble, H.J. (2008). The quantum internet. *Nature*, 453, 1023–1030. [DOI: 10.1038/nature07127]

[2] Wehner, S., Elkouss, D., & Hanson, R. (2018). Quantum internet: A vision for the road ahead. *Science*, 362(6412), eaam9288. [DOI: 10.1126/science.aam9288]

[3] Dahlberg, A., Skrzypczyk, M., Coopmans, T., et al. (2019). A link layer protocol for quantum networks. *Proceedings of the ACM SIGCOMM 2019 Conference*, 159–173. [DOI: 10.1145/3341302.3342070]

[4] Pompili, M., Hermans, S.L.N., Baier, S., et al. (2021). Realization of a multinode quantum network of remote solid-state qubits. *Science*, 372(6539), 259–264. [DOI: 10.1126/science.abg1919]

[5] Hermans, S.L.N., Pompili, M., Beukers, H.K.C., Baier, S., Borregaard, J., & Hanson, R. (2022). Qubit teleportation between non-neighbouring nodes in a quantum network. *Nature*, 605, 663–668. [DOI: 10.1038/s41586-022-04697-y]

[6] Sangouard, N., Simon, C., de Riedmatten, H., & Gisin, N. (2011). Quantum repeaters based on atomic ensembles and linear optics. *Reviews of Modern Physics*, 83(1), 33–80. [DOI: 10.1103/RevModPhys.83.33]

[7] Van Meter, R. (2014). *Quantum Networking*. Wiley-ISTE. [Book-length treatment of quantum network architecture and routing.]
