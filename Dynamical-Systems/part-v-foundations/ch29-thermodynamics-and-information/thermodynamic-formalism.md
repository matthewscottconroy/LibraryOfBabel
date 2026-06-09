# 29.4 Thermodynamic Formalism

Here is where the physical story and the dynamical systems story fuse completely. The thermodynamic formalism, developed by David Ruelle and Rufus Bowen in the 1970s, is a precise mathematical translation of statistical mechanics into the language of hyperbolic dynamical systems. Every concept has its counterpart: microstates become periodic orbits, energy becomes an action functional, the partition function becomes a sum over periodic orbits, free energy becomes topological pressure, and phase transitions are non-differentiabilities of pressure.

This isn't an analogy. It's an identification. The formalism was built specifically to make the connection exact.

**Definition 29.4.1 (Transfer Matrix / Partition Function).** For an SFT with transition matrix $A$ and potential $\phi: X \to \mathbb{R}$, the *partition function* at inverse temperature $\beta$ is:
$$Z_n(\beta) = \sum_{x: f^n(x) = x} e^{\beta\sum_{k=0}^{n-1}\phi(f^k(x))}.$$

Compare to the statistical mechanics partition function $Z = \sum_i e^{-\beta E_i}$. In the dynamical setting, microstates are periodic orbits of period $n$, and energy is replaced by the negative cumulative potential $-\sum_{k=0}^{n-1}\phi(f^k(x))$. The inverse temperature $\beta$ is a free parameter.

**Theorem 29.4.2.** The *free energy* $F(\beta) = \lim_{n\to\infty}\frac{1}{n}\log Z_n(\beta) = P(f, \beta\phi)$ (the topological pressure).

The topological pressure, defined in Chapter 22 via open covers, is the free energy of the dynamical system. This is not just terminological: the variational formula for pressure, $P(f, \phi) = \sup_\mu \{h_\mu(f) + \int \phi\, d\mu\}$, is exactly the Legendre transform relationship between entropy and free energy in thermodynamics.

The full dictionary:

| Thermodynamics | Thermodynamic Formalism |
|---|---|
| States (microstates) | Periodic orbits |
| Energy $E_i$ | $-\sum_{k=0}^{n-1}\phi(f^k(x))$ (action) |
| Inverse temperature $\beta$ | Parameter $\beta$ |
| Partition function $Z$ | $\sum_{\text{Per}_n} e^{\beta S_n(x)}$ |
| Free energy $-\frac{1}{\beta}\log Z$ | $-P(f, \beta\phi)$ |
| Gibbs state | Equilibrium state (SRB measure) |
| Phase transition | Non-differentiability of $P(\beta\phi)$ |

The SRB (Sinai-Ruelle-Bowen) measure is the dynamical analogue of the Gibbs distribution: it is the unique measure that maximizes the variational expression for pressure (entropy plus average potential). For Axiom A attractors, the SRB measure describes the long-run statistics of Lebesgue-almost-every trajectory near the attractor.

**Theorem 29.4.3 (Phase Transitions in Dynamics).** The pressure function $\beta \mapsto P(f, \beta\phi)$ is convex and continuous. A *phase transition* occurs when $P$ is not differentiable at some $\beta^*$ — i.e., there are multiple tangent measures (equilibrium states) at $\beta^*$.

In statistical mechanics, phase transitions (water to ice, paramagnet to ferromagnet) are points of non-analyticity in the free energy. In dynamics, they are exactly the same: points where the pressure fails to be differentiable, and where multiple equilibrium measures coexist. The dynamical system is "indecisive" at a phase transition — there are multiple natural measures, and which one the system chooses depends on how you approach the critical parameter.

**Example 29.4.4 (Hofbauer Tower).** For piecewise monotone interval maps, the existence of phase transitions at $\beta = 1$ (the inverse temperature corresponding to the natural measure) is related to the number of SRB measures and the decay of correlations.

The Hofbauer tower is a technical tool — a symbolic extension of an interval map that makes the hyperbolic structure explicit — but the result it gives is dynamically meaningful: whether a system at "natural temperature" ($\beta = 1$) has a unique or multiple equilibrium states determines the system's long-run ergodic behavior. A phase transition at $\beta = 1$ means multiple SRB measures and, typically, irregular decay of correlations.
