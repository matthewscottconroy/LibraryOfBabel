# Section 20.2: The KLM Protocol

Knill, Laflamme, and Milburn's 2001 construction proceeds in three movements, and this section follows their score. First, the **nonlinear sign (NS) gate**: a heralded linear-optical circuit that flips the phase of the two-photon amplitude in a single mode — impossible unitarily, possible with probability 1/4 conditioned on an ancilla measurement. Second, the **nondeterministic CZ**: two NS gates wrapped around a Hong-Ou-Mandel interference turn the sign flip into a two-qubit entangling gate succeeding with probability 1/16. Third, **gate teleportation**: the trick that rescues scalability, moving the gamble offline — prepare an entangled resource state by trial and error, then consume it to apply the gate near-deterministically, with success probability $n/(n+1)$ per qubit for an $n$-photon ancilla state.

The KLM protocol is today more blueprint than building plan — cluster-state methods (Section 20.3) superseded its direct implementation — but every concept it introduced (heralding, ancilla factories, feed-forward, teleported gates, loss-tolerant encoding) remains the working vocabulary of photonic quantum computing.

- **20.2.1** — Measurement-Induced Nonlinearity: The NS Gate
- **20.2.2** — The Nondeterministic CZ Gate
- **20.2.3** — Gate Teleportation and Success Boosting
