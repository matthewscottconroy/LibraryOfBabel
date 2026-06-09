# 29.1 The Boltzmann-Gibbs-Shannon Connection

It's worth being explicit about the claim at the heart of this chapter: Boltzmann entropy, Gibbs entropy, and Shannon entropy are not analogous quantities. They are the same quantity. The apparent multiplicity comes from three different physicists (Boltzmann, Gibbs, Shannon) approaching the same mathematical object from different starting points. Recognizing this identity is one of the most clarifying moments in the entire subject.

## 29.1.1 Statistical Mechanics Entropy

Boltzmann's entropy applies when all microstates are equally likely — the microcanonical ensemble. It's the simplest case.

**Definition 29.1.1 (Boltzmann Entropy).** For a macrostate with $\Omega$ microstates (each equally likely):
$$S_B = k_B \ln \Omega,$$
where $k_B = 1.38 \times 10^{-23}$ J/K is Boltzmann's constant.

The $\ln \Omega$ is just the entropy of the uniform distribution over $\Omega$ outcomes, times $k_B$ to convert from nats to physical units. If there are $\Omega = 2^n$ microstates, then $S_B = n k_B \ln 2$, which is $n$ bits times $k_B \ln 2$ joules/kelvin per bit. This is where the physical conversion factor comes from.

Gibbs generalized Boltzmann to non-uniform distributions.

**Definition 29.1.2 (Gibbs Entropy).** For a probability distribution $p = (p_1, \ldots, p_n)$ over microstates:
$$S_G = -k_B \sum_i p_i \ln p_i.$$

**Theorem 29.1.3 (Shannon = Gibbs).** $S_G = k_B \cdot H(p)/\log e$, where $H(p) = -\sum_i p_i \log_2 p_i$ is Shannon entropy in bits. The conversion: 1 bit $= k_B \ln 2$ joules/kelvin.

The proof is a unit conversion. Shannon entropy uses $\log_2$ (bits); Gibbs entropy uses $\ln$ (nats) times $k_B$ (joules/kelvin). Change the logarithm base and multiply by the appropriate constant, and they're identical.

**Maximum Entropy Principle:** Among all distributions over $n$ microstates, the uniform distribution maximizes $S_G$. The Gibbs distribution $p_i \propto e^{-E_i/k_BT}$ maximizes $S_G$ subject to fixed mean energy $\langle E \rangle$.

The maximum entropy principle is one of the central tools of statistical mechanics, and Jaynes's 1957 reformulation made it the foundation of the information-theoretic approach to the whole subject. Thermal equilibrium is not just "the state that happens" — it is the state that maximizes entropy (information-theoretic uncertainty) subject to the constraint that average energy is fixed. This is a variational principle, and it's the one that generates the Boltzmann distribution from first principles.

## 29.1.2 Free Energy and Information

The connection deepens when we look at free energy, which is the thermodynamically relevant quantity for systems in contact with a heat bath.

**Definition 29.1.4.** The *Helmholtz free energy* is $F = U - TS$ where $U = \langle E \rangle$ is the internal energy, $T$ is temperature, $S$ is Gibbs entropy. At equilibrium, $F$ is minimized.

**Theorem 29.1.5 (Free Energy = Negative Relative Entropy).** For a system with Gibbs equilibrium distribution $p^* \propto e^{-E_i/k_BT}$ and actual distribution $q$:
$$F(q) = F(p^*) + k_BT \cdot D_{KL}(q \| p^*).$$

So the *excess free energy* is $k_BT$ times the KL divergence from equilibrium. Minimizing free energy = minimizing KL divergence.

This is the key formula. The KL divergence $D_{KL}(q \| p^*)$ measures how far the current distribution is from equilibrium. Multiplying by $k_BT$ converts information distance to free energy. As the system evolves toward equilibrium, $D_{KL}$ decreases monotonically — and this is the physical content of the second law.

We'll see this KL-divergence formulation of thermodynamics appear again in the Wasserstein gradient flow framework of Chapter 30 (where the Fokker-Planck equation is the gradient flow of free energy in Wasserstein space) and in the entropy production formulas of Section 29.5.
