# Chapter 20: Linear Optical Quantum Computing

> *"For years the answer seemed obviously no: photons do not interact, so linear optics cannot make one photon's state depend on another's. Knill, Laflamme, and Milburn showed that the obvious answer was wrong — measurement itself supplies the missing nonlinearity."*

---

## The Bombshell of 2001

Can you build a universal quantum computer from beam splitters, phase shifters, single-photon sources, and photodetectors — with no photon-photon interaction anywhere in the machine? Linear optical elements transform mode operators linearly; each photon sails through the circuit indifferent to the others. Two-qubit gates require exactly the opposite: the state of one photon must conditionally change the state of another. The conclusion seemed airtight, and for the 1990s "linear optical quantum computing" was a contradiction in terms.

In 2001, Knill, Laflamme, and Milburn (KLM) demolished the impossibility argument. Their observation: photodetection is *not* a linear operation. Interfere the computational photons with ancilla photons, measure the ancillas, and keep the outcome only when the detectors report a particular pattern — the surviving state has undergone a transformation no linear circuit could apply. The resulting gates are probabilistic, but KLM showed the success probability can be boosted arbitrarily close to one by gate teleportation with larger ancilla states, all still within linear optics. Scalable quantum computing with light was possible in principle. The paper (*Nature* 409, 46) founded the field this chapter surveys.

What followed is a twenty-year arc of making "possible in principle" affordable in practice: the resource cost of original KLM was astronomical, and it fell by orders of magnitude through cluster states (measurement-based quantum computing), fusion gates, and finally fusion-based quantum computing (FBQC) — the architecture PsiQuantum is building in a CMOS photonics foundry. Along the way, the field produced quantum computing's first credible claims of computational advantage: boson sampling machines (Jiuzhang, Borealis) sampling from distributions whose classical computation is tied to the #P-hard matrix permanent.

## The Cast of Ideas

Five ideas organize the chapter, each building on the last:

1. **The dual-rail qubit** — one photon in two modes; every single-qubit gate is an interferometer (Section 20.1).
2. **Measurement-induced nonlinearity** — the KLM nonlinear sign gate, the nondeterministic CZ, and teleportation-based boosting (Section 20.2).
3. **Measurement-based computing** — pre-build entanglement into a cluster state, then compute by single-qubit measurements alone; fusion gates make cluster growth a percolation problem (Section 20.3).
4. **Boson sampling** — abandon universality, keep the permanent: a purpose-built linear-optical machine whose output distribution is classically intractable under plausible complexity assumptions (Section 20.4).
5. **Loss as erasure** — photon loss, the dominant photonic error, announces its own location; error correction exploits this to reach thresholds unthinkable for unlocated errors (Section 20.5).

Everything rests on the hardware of Chapter 19: sources set the input state quality, detector efficiency enters every heralding step, and the $\eta^n$ scaling law decides what is experimentally reachable.

---

## Chapter Structure

**Section 20.1 — The Qubit in Photonics**: Dual-rail encoding; single-qubit gates from beam splitters and phase shifters; why CNOT is the hard part.

**Section 20.2 — The KLM Protocol**: Measurement-induced nonlinearity and the NS gate; the nondeterministic CZ; gate teleportation and success boosting.

**Section 20.3 — Measurement-Based Quantum Computing**: Cluster states and their stabilizers; the one-way quantum computer; fusion-based quantum computing.

**Section 20.4 — Boson Sampling**: The Aaronson-Arkhipov hardness argument; Gaussian boson sampling; Jiuzhang, Borealis, and the classical-simulation arms race.

**Section 20.5 — Quantum Error Correction for Photonic Systems**: Photon loss as erasure; topological codes on photonic graph states; PsiQuantum's fault-tolerant roadmap.
