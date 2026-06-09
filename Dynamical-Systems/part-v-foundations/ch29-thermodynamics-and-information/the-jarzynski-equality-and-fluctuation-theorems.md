# 29.3 The Jarzynski Equality and Fluctuation Theorems

Classical thermodynamics talks about equilibrium states and average quantities. The second law says, in equilibrium processes, that average work exceeds the free energy change: $\langle W \rangle \geq \Delta F$. But what about non-equilibrium processes — situations where you drive the system rapidly, far from equilibrium, and individual trajectories fluctuate enormously? Can we say anything precise?

In 1997, Christopher Jarzynski discovered that yes — there is an exact equality, not just an inequality, that holds for any protocol, near or far from equilibrium. It's one of the most surprising results in modern statistical physics, and it has a beautiful information-theoretic interpretation.

**Theorem 29.3.1 (Jarzynski Equality, 1997).** For a system driven from equilibrium state $A$ to equilibrium state $B$ by an arbitrary protocol:
$$\langle e^{-W/k_BT} \rangle = e^{-\Delta F/k_BT},$$
where $W$ is the work done on the system (random variable) and $\Delta F = F_B - F_A$ is the free energy difference.

The brackets $\langle \cdot \rangle$ denote an average over many independent realizations of the protocol. Even though individual trajectories can have wildly different work values — some positive, some negative — the *exponential average* exactly equals the exponential of the free energy difference.

**Corollary 29.3.2 (Second Law as Jensen's Inequality).** By Jensen's inequality $\langle e^{-W/k_BT}\rangle \geq e^{-\langle W\rangle/k_BT}$, so:
$$\langle W \rangle \geq \Delta F,$$
which is the second law (average work $\geq$ free energy change).

The second law drops out of the Jarzynski equality via Jensen's inequality. This is a remarkable derivation: the second law isn't an additional assumption — it's a consequence of the exact equality, via the convexity of the exponential function. The Jarzynski equality is strictly stronger than the second law.

The Crooks fluctuation theorem goes further, relating the probability distributions of work in the forward and reverse processes.

**Theorem 29.3.3 (Crooks Fluctuation Theorem, 1999).** For forward and reverse protocols connecting $A \leftrightarrow B$:
$$\frac{P_F(W)}{P_R(-W)} = e^{(W - \Delta F)/k_BT}.$$

The ratio of probabilities of observing work $W$ forward and $-W$ in reverse gives the entropy production. If you observe a trajectory where work $W$ was done, the ratio says how much more likely that trajectory is in the forward direction than in the reverse. Large positive work (far from equilibrium) corresponds to large forward-to-reverse ratios — the trajectory is highly "directional."

**Information-Theoretic Formulation:** The Jarzynski equality is the statement that the KL divergence between the forward and reverse path distributions equals the average entropy production:
$$D_{KL}(P_F \| P_R) = \langle \Sigma \rangle / k_B,$$
where $\Sigma$ is the total entropy production (system + environment).

This is the deepest formulation. The KL divergence between the forward path distribution and the time-reversed distribution is the average entropy production — the average irreversibility of the process. KL divergence measures how "distinguishable" the forward process is from the reverse, and the second law says this is always nonnegative: the forward process is always at least as likely (in the ensemble sense) as the reverse.

These fluctuation theorems are not just theoretical — they have been used to measure free energy differences of biomolecules from pulling experiments. By pulling a single RNA molecule and measuring the work distribution $P_F(W)$, you can compute $\Delta F = -k_BT \ln \langle e^{-W/k_BT} \rangle$ without ever reaching equilibrium. Jarzynski's equality turned into an experimental technique.

In the next section, we see how this entire framework — partition functions, free energy, equilibrium states, phase transitions — has an exact parallel in the hyperbolic dynamical systems studied by Ruelle and Bowen.
