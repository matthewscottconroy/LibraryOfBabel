# 39.6 The Second Law in Quantum Thermodynamics

Shannon's entropy connects to thermodynamics through the Boltzmann-Gibbs picture: entropy is related to the logarithm of the number of microstates, and work extraction is bounded by $kT \times$ entropy. But this is an average statement. One-shot thermodynamics asks: what work can you extract with certainty (or high probability) from a single quantum system?

The answer replaces Shannon/von Neumann entropy with min-entropy.

**Theorem 39.6.1 (Quantum Second Law — Dahlsten-Renner-Rieper-Vedral, 2011).** A work extraction protocol from a quantum system $\rho$ at temperature $T$ can extract at most:
$$W^* = kT \cdot H_{\min}(\rho)$$
work in a *single shot*, compared to the average $kT \cdot S(\rho)$ from asymptotic i.i.d. processing.

In the standard thermodynamics picture (many identical systems, many cycles), you extract work at rate $kT \cdot S(\rho)$ per system. But for a single quantum system in a single run, the constraint is tighter: you can only reliably extract $kT \cdot H_{\min}(\rho)$.

**Corollary 39.6.2.** In the single-shot regime, the work fluctuations are significant. The second law has the form:
$$\langle W \rangle \leq kT \cdot S(\rho) \quad (\text{average})$$
but the one-shot bound $W \leq kT \cdot H_{\min}(\rho)$ holds with high probability (not just on average).

The gap between $H_{\min}(\rho)$ and $S(\rho)$ is the cost of certainty. If you're content with an average extraction, you get $kT \cdot S(\rho)$. If you need the extraction to work for sure (or with probability $1 - \varepsilon$), you can only get $kT \cdot H_{\min}^\varepsilon(\rho)$.

**Connection to Dynamical Systems:** A dynamical system with entropy $h$ generates $h$ bits/time, which can be extracted as work at rate $k_BT \cdot h$. One-shot thermodynamics corrects for finite-time effects: the *actual* work extracted in a finite window $T$ is $\approx k_BT \cdot H_{\min}^\varepsilon(\text{orbit}_T)$.

This connection runs deep. The KS entropy of a classical dynamical system measures the information generation rate. One-shot information theory says that to extract this information as work in finite time, you pay a penalty proportional to the square root of the time window. This is the finite-time correction to the second law, made rigorous by smooth entropy.
