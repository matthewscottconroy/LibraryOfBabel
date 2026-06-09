# 21.3 Quantum Channel Capacity

A quantum channel can be used to transmit either classical information (bits) or quantum information (qubits). These are genuinely different resources, and the capacity of a quantum channel is not one number but two: the classical capacity $C(\mathcal{E})$ and the quantum capacity $Q(\mathcal{E})$.

## 21.3.1 Classical Capacity

**Definition 21.3.1 (Classical Capacity).** The *classical capacity* $C(\mathcal{E})$ of a quantum channel $\mathcal{E}$ is the maximum rate (bits per channel use) of reliable classical communication.

The first step is a bound on how much classical information can be extracted from a quantum channel:

**Theorem 21.3.2 (Holevo Bound — Holevo 1973).** For any encoding $\{p_i, \rho_i\}$ (message $i$ is encoded as state $\rho_i$ with probability $p_i$), the *Holevo $\chi$-information* bounds the accessible information by any measurement:
$$I(X; Y) \leq \chi = S\left(\sum_i p_i \rho_i\right) - \sum_i p_i S(\rho_i).$$

The Holevo information $\chi$ is the von Neumann entropy of the average state minus the average von Neumann entropy of the individual states. It measures the "quantum advantage" of the encoding: how much more information can be stored in the quantum states than in classical mixtures.

The Holevo bound is fundamental: no measurement on the output of the channel can extract more classical information than $\chi$. This bounds the rate of classical communication over the channel.

Is $\chi$ achievable? Yes — this is the HSW theorem:

**Theorem 21.3.3 (HSW Theorem — Hausladen-Schumacher-Westmoreland, Holevo 1997).** The classical capacity of a quantum channel is:
$$C(\mathcal{E}) = \lim_{n\to\infty} \frac{1}{n} \chi(\mathcal{E}^{\otimes n}) = \lim_{n\to\infty} \frac{1}{n} \max_{\{p_i, \rho_i^{(n)}\}} \left[S\left(\mathcal{E}^{\otimes n}\left(\sum_i p_i\rho_i^{(n)}\right)\right) - \sum_i p_i S(\mathcal{E}^{\otimes n}(\rho_i^{(n)}))\right].$$

The limit (regularization over $n$) is needed because $\chi$ may not be additive over tensor products. This is a crucial difference from the classical case: for classical channels, the capacity is always $\max_{p(x)} I(X;Y)$, a single-letter formula. For quantum channels, the capacity may require entangled inputs across many uses to achieve.

**The additivity question** was a major open problem for two decades. Hasings resolved it in 2009:

The question of whether $\chi$ is additive was a major open problem, resolved negatively by Hastings (2009): there exist quantum channels for which entangled inputs provide more classical capacity than product inputs.

In other words: sending entangled states across multiple uses of the channel can achieve higher classical capacity than sending product states. This has no classical analogue — for classical channels, using independent inputs is always optimal for classical capacity. The Hastings counterexample is probabilistic (it shows that random channels have this property) and there are no explicit examples of channels with large superadditive capacity gap.

## 21.3.2 Quantum Capacity

**Definition 21.3.4 (Quantum Capacity).** The *quantum capacity* $Q(\mathcal{E})$ is the maximum rate (qubits per channel use) of reliable quantum communication — transmission of quantum states preserving quantum coherence.

**Theorem 21.3.5 (LSD Theorem — Lloyd, Shor, Devetak).** The quantum capacity is:
$$Q(\mathcal{E}) = \lim_{n\to\infty} \frac{1}{n} \max_{\rho^{(n)}} I_c(\rho^{(n)}; \mathcal{E}^{\otimes n}),$$
where the *coherent information* is $I_c(\rho; \mathcal{E}) = S(\mathcal{E}(\rho)) - S(\mathcal{E}^c(\rho))$.

The coherent information involves $\mathcal{E}^c$, the *complementary channel* — the channel to the environment. The environment "steals" information from the transmitted quantum state, and coherent information measures how much quantum information reaches the receiver versus the environment.

**Remark 21.3.6.** The quantum capacity has no classical analogue: $Q > 0$ means the channel can transmit quantum coherence. Channels with $Q = 0$ completely destroy quantum information — no quantum error correction can help. Anti-degradable channels (where the eavesdropper/environment has a better view than the receiver) always have $Q = 0$. The depolarizing channel has $Q > 0$ for small enough error rates.

The LSD theorem shares the same "regularization" structure as the classical capacity: a limit over $n$ uses is needed. Whether coherent information is additive (or superadditive) for general channels is not known.

Computing quantum capacity is hard in general. For specific channels — the erasure channel, the bosonic Gaussian channel, channels with symmetry — closed-form results are available and are among the most practically relevant results in quantum information theory.
