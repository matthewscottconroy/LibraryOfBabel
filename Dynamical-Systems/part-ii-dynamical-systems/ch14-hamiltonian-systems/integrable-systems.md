# 14.2 Integrable Systems

Most Hamiltonian systems are not solvable in any meaningful sense — you cannot write down a formula for the orbits. But there is a special class, the *completely integrable* systems, where the orbits are as regular as possible: quasi-periodic motion on invariant tori. These systems are the foundation for KAM theory, which asks what happens when you perturb them.

The condition for integrability is that the system has "as many conserved quantities as degrees of freedom." For an $n$-degree-of-freedom system (living on a $2n$-dimensional phase space), you need $n$ independent conserved quantities. But independence is not enough — they must be *in involution* (their Poisson brackets must all vanish), which is a strong compatibility condition.

**Definition 14.2.1.** A Hamiltonian system $(M^{2n}, \omega, H)$ is *completely integrable* (in the Liouville-Arnold sense) if there exist $n$ smooth functions $F_1 = H, F_2, \ldots, F_n: M \to \mathbb{R}$ that are:
1. *Independent*: $dF_1, \ldots, dF_n$ are linearly independent on a dense open set.
2. *In involution*: $\{F_i, F_j\} = 0$ for all $i, j$, where the Poisson bracket is $\{F, G\} = \omega(X_F, X_G)$.

**Theorem 14.2.2 (Liouville-Arnold Theorem).** For a completely integrable system with compact connected level sets $M_c = \{F_1 = c_1, \ldots, F_n = c_n\}$:
1. Each $M_c$ is diffeomorphic to an $n$-torus $\mathbb{T}^n$.
2. The Hamiltonian flow on $M_c$ is quasi-periodic: $(\theta_1, \ldots, \theta_n) \mapsto (\theta_1 + \omega_1 t, \ldots, \theta_n + \omega_n t)$ for some frequency vector $\omega = (\omega_1, \ldots, \omega_n)$.
3. There exist *action-angle coordinates* $(I_1, \ldots, I_n, \theta_1, \ldots, \theta_n)$ (a symplectomorphism to a neighborhood of $M_c$) in which $H = H(I_1, \ldots, I_n)$ depends only on the actions, and $\omega = \sum_i dI_i \wedge d\theta_i$.

What this is saying is: every completely integrable Hamiltonian system, in a neighborhood of a regular level set, looks like a system of uncoupled rotors. The actions $I_i$ are constants of motion; the angles $\theta_i$ rotate at constant frequencies $\omega_i = \partial H/\partial I_i$. The whole phase space (locally) is foliated by invariant tori, and the dynamics on each torus is a uniform rotation.

If the frequency vector $\omega = (\omega_1, \ldots, \omega_n)$ has all ratios $\omega_i/\omega_j$ rational, the orbits on the torus are periodic. If some ratio is irrational, the orbits are dense on the torus — quasi-periodic motion, never returning exactly to the start but coming arbitrarily close.

**Examples:**

- *1D Hamiltonian* $H = p^2/2 + V(q)$: always integrable, since $H$ itself is the one conserved quantity and the level sets are (generically) circles.
- *$n$ uncoupled harmonic oscillators*: $H = \sum_i (p_i^2 + q_i^2)/2$. The action-angle variables are $I_i = (p_i^2 + q_i^2)/2$ (the energy of the $i$-th oscillator) and $\theta_i = \arctan(p_i/q_i)$ (the phase).
- *The Kepler problem* (gravitational 2-body): integrable with 3 conserved quantities (energy, and two components of angular momentum). The elliptical orbits are the level sets of these conserved quantities.
- *Geodesic flow on an ellipsoid*: integrable (Jacobi, 1838) — one of the most beautiful classical results in Hamiltonian mechanics.

The Kepler problem is the most historically important: its integrability is why Newton could compute planetary orbits exactly. The three-body problem is not integrable, which is why Poincaré's 1890 proof of this caused a crisis. KAM theory is the resolution of that crisis — and we turn to it now.
