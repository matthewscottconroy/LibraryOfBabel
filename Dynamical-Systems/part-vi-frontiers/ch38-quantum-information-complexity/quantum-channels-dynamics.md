# 38.6 Quantum Channels as Dynamical Systems

A quantum Markov semigroup is the quantum analogue of a classical Markov process. It describes the time evolution of an open quantum system — one that interacts with an environment, which causes decoherence. From the dynamical systems perspective, this is a dissipative dynamical system in the space of density matrices.

**Definition 38.6.1.** A *quantum Markov semigroup* is a family $(\mathcal{E}_t)_{t \geq 0}$ of CPTP maps satisfying $\mathcal{E}_0 = \text{id}$ and $\mathcal{E}_{t+s} = \mathcal{E}_t \circ \mathcal{E}_s$.

CPTP means "completely positive trace-preserving" — these are the physically allowed quantum operations. The semigroup property is the quantum Markov assumption: the future depends only on the present state, not on history.

**Theorem 38.6.2 (Lindblad, 1976).** Every quantum Markov semigroup has a generator of the form:
$$\frac{d}{dt}\rho = \mathcal{L}(\rho) = -i[H, \rho] + \sum_k \left(L_k\rho L_k^\dagger - \frac{1}{2}\{L_k^\dagger L_k, \rho\}\right),$$
where $H$ is the Hamiltonian and $L_k$ are *Lindblad operators* (jump operators describing decoherence).

The Lindblad equation is the master equation for open quantum systems. The Hamiltonian part $-i[H, \rho]$ gives unitary (reversible) evolution. The Lindblad operator terms $L_k\rho L_k^\dagger - \frac{1}{2}\{L_k^\dagger L_k, \rho\}$ describe irreversible decay — decoherence and dissipation.

**Theorem 38.6.3 (Quantum Ergodicity).** A quantum Markov semigroup $(\mathcal{E}_t)$ has a unique stationary state $\sigma$ (satisfying $\mathcal{E}_t(\sigma) = \sigma$ for all $t$) iff the only observables commuting with all $L_k$ and $H$ are multiples of the identity (quantum ergodicity condition).

The quantum ergodicity condition is the direct analogue of classical ergodicity: a classical system is ergodic iff the only invariant measurable sets are the full space and the empty set. For quantum systems, the role of "invariant set" is played by "observable commuting with all the dynamics," and ergodicity means there's no such observable except the trivial ones.

This connects beautifully to the classical ergodic theory of this book: ergodicity of the Lindblad dynamics on density matrices corresponds to mixing in the classical setting. The spectral gap of the Lindblad operator — the gap between 0 and the next eigenvalue — controls the rate of convergence to the stationary state, analogous to the mixing rate in classical ergodic theory.
