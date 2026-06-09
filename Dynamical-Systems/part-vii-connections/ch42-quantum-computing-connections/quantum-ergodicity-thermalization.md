# 42.4 Quantum Ergodicity and Thermalization

The quantum ergodic hypothesis — the Eigenstate Thermalization Hypothesis (ETH) — is one of the central open problems in quantum many-body physics. It asks: do individual energy eigenstates of a "chaotic" quantum system look thermal?

**Eigenstate Thermalization Hypothesis (ETH — Deutsch, 1991; Srednicki, 1994):**

**Conjecture 42.4.1 (ETH).** For a non-integrable quantum system with Hamiltonian $H$ (energy eigenstates $|E_n\rangle$): for any local observable $A$:
$$\langle E_n | A | E_m \rangle = f_A(\bar{E})\delta_{nm} + e^{-S(\bar{E})/2}R_{nm}g_A(\bar{E}, \omega),$$
where $\bar{E} = (E_n+E_m)/2$, $\omega = E_n - E_m$, $S(\bar{E})$ is the thermodynamic entropy at energy $\bar{E}$, and $R_{nm}$ is a random variable with $\langle R_{nm}^2\rangle = 1$.

**Interpretation:** ETH says that individual energy eigenstates "look thermal" for local observables — their expectation values in an eigenstate equal the thermal average. This is the quantum version of the ergodic hypothesis.

The ETH is a strong statement. Classical ergodic theory says: time averages equal space averages for most initial conditions. ETH says something much stronger: individual energy eigenstates (not time averages) already look like thermal states. Every eigenstate is simultaneously an "equilibrium state" for local measurements.

The off-diagonal elements $e^{-S(\bar{E})/2}R_{nm}g_A(\bar{E}, \omega)$ are exponentially small in the system size (since $S \sim N$, the system size). This means that matrix elements between different energy eigenstates are tiny — the energy eigenstates don't "mix" when you measure local observables.

**Theorem 42.4.2 (Berry-Tabor / Bohigas-Giannoni-Schmit Conjecture — Quantum Chaos).** For classically chaotic systems:
- Energy level spacings follow GUE (Gaussian Unitary Ensemble) statistics
- Eigenstates are approximately random unit vectors in the relevant Hilbert space subspace

For classically integrable systems:
- Energy level spacings are Poissonian (independent)
- Eigenstates are localized along classical tori

The BGS conjecture is the quantum chaos conjecture: a quantum system whose classical limit is chaotic has GUE level statistics. This has been verified numerically for many systems but is not proved in general. The connection to the classical KS entropy (from Chapter 7) runs through Berry's conjecture that the eigenstates of a classically chaotic system look like random waves — a quantitative version of the quantum ergodic hypothesis.
