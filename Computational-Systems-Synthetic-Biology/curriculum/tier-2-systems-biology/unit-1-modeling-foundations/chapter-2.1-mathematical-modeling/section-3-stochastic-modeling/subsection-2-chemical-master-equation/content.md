# The Chemical Master Equation

## From Trajectories to Probability Distributions

With deterministic ODE models, you ask: "Given these initial conditions and parameters, what is the concentration of mRNA at time $t = 30$ minutes?" The answer is a single number — a smooth curve through time.

With stochastic models, you have to ask a different question: "Given these initial conditions and parameters, what is the *probability* that there are exactly $n$ mRNA molecules at time $t = 30$ minutes?" The answer is a distribution — a histogram over all possible integer values of $n$. The deterministic model tracks a single trajectory; the stochastic model tracks the evolution of a probability distribution over all possible trajectories.

The **Chemical Master Equation (CME)** is the governing equation for this probability distribution. It is exact — no approximations beyond the well-mixed assumption are made. Understanding the CME is essential for understanding what the Gillespie algorithm computes and why stochastic simulations are the way to solve it.

## State Space and Propensity Functions

Let the system state be a vector $\mathbf{n} = (n_1, n_2, \ldots, n_M)$ giving the integer copy numbers of each of $M$ molecular species. For a system with $R$ reaction channels, each reaction $j$ is characterized by:

**Stoichiometric change vector** $\boldsymbol{\nu}_j$: how the state changes when reaction $j$ fires. For example, the reaction mRNA $\rightarrow \emptyset$ decreases the mRNA count by 1, so $\nu_j = -1$ for mRNA.

**Propensity function** $a_j(\mathbf{n})$: the instantaneous probability rate that reaction $j$ fires given that the system is in state $\mathbf{n}$. More precisely, $a_j(\mathbf{n})\,dt$ is the probability that reaction $j$ fires in the next infinitesimal interval $[t, t + dt)$.

For elementary reactions, propensities follow from combinatorics — the probability that a particular set of molecules collides:

| Reaction type | Propensity |
|---|---|
| $\emptyset \xrightarrow{k} X$ (zeroth order) | $a = k$ |
| $X \xrightarrow{k} \ldots$ (first order) | $a = k \cdot n_X$ |
| $X + Y \xrightarrow{k} \ldots$ (bimolecular) | $a = k \cdot n_X \cdot n_Y / \Omega$ |
| $2X \xrightarrow{k} \ldots$ (dimerization) | $a = k \cdot n_X(n_X - 1) / (2\Omega)$ |

where $\Omega$ is the system volume (which enters because bimolecular rate constants have units of M$^{-1}$s$^{-1}$, and concentrations are $n/\Omega$).

The dimerization propensity $n_X(n_X - 1)/2$ is worth pausing on: it counts the number of distinct pairs of $X$ molecules. With $n_X$ molecules, there are $n_X(n_X-1)/2$ possible pairs — and each pair has an independent chance to react. This is where the molecular discreteness enters: with 2 molecules, there is 1 possible pair; with 3 molecules, there are 3 pairs; the rate scales as $n(n-1)/2$, not $n^2/2$ as the continuous approximation would suggest. At large $n$ the difference is negligible; at small $n$ it matters.

## The CME

Let $P(\mathbf{n}, t)$ denote the probability of being in state $\mathbf{n}$ at time $t$. The CME states how this probability changes over time:

$$\frac{\partial P(\mathbf{n}, t)}{\partial t} = \sum_{j=1}^{R} \left[ a_j(\mathbf{n} - \boldsymbol{\nu}_j)\, P(\mathbf{n} - \boldsymbol{\nu}_j, t) - a_j(\mathbf{n})\, P(\mathbf{n}, t) \right]$$

The first term in the sum represents **gain**: transitions into state $\mathbf{n}$ by firing reaction $j$ from state $\mathbf{n} - \boldsymbol{\nu}_j$. The second term represents **loss**: transitions out of state $\mathbf{n}$ by firing reaction $j$ from state $\mathbf{n}$ itself.

This equation encodes the following logic: state $\mathbf{n}$ can be reached from any state $\mathbf{n} - \boldsymbol{\nu}_j$ (by firing reaction $j$), and can be left to any state $\mathbf{n} + \boldsymbol{\nu}_j$ (by the same reaction firing again). It is a differential equation for every element of the probability distribution simultaneously — one equation per possible state $\mathbf{n}$.

## Analytical Solutions

The CME can be solved analytically only for simple reaction networks. These exact solutions are enormously valuable as test cases and as sources of intuition:

**Constitutive expression (birth-death):** Gene produces mRNA at rate $\alpha$; mRNA degrades at rate $\delta$ per molecule.

$$\frac{dP(n)}{dt} = \alpha P(n-1) + \delta(n+1)P(n+1) - (\alpha + \delta n)P(n)$$

Steady-state solution: $P(n) = \frac{(\alpha/\delta)^n e^{-\alpha/\delta}}{n!}$, a **Poisson distribution** with mean and variance both equal to $\alpha/\delta$.

This is a beautiful result: purely constitutive expression (no regulation, no bursting) produces Poisson-distributed mRNA. The Fano factor equals 1. This is the baseline noise floor — and the fact that real genes show Fano factors of 2–50 tells you immediately that something more complex than simple constitutive expression is occurring.

**Two-state promoter (bursty expression):** Promoter switches between ON ($k_\text{on}$) and OFF ($k_\text{off}$) states; mRNA produced at rate $\alpha$ only when ON.

Steady-state solution: **negative binomial distribution** with:

$$\langle n \rangle = \frac{\alpha k_\text{on}}{\delta(k_\text{on} + k_\text{off})}, \quad \text{Fano} = 1 + \frac{\alpha k_\text{off}}{\delta(k_\text{on} + k_\text{off})}$$

The Fano factor exceeds 1 — this is **super-Poisson noise** due to transcriptional bursting. The excess noise (Fano $- 1$) measures how much extra variance the bursting adds beyond the constitutive Poisson baseline. It depends on the ratio of burst size ($\alpha/\delta$) to switching speed: slow switching with large bursts produces the most noise.

## The Fokker-Planck Approximation

For large molecule numbers, the discrete CME can be approximated by a continuous **Fokker-Planck equation** (FPE) for a probability density $p(\mathbf{x}, t)$ over continuous concentrations:

$$\frac{\partial p}{\partial t} = -\sum_i \frac{\partial}{\partial x_i}\left[f_i(\mathbf{x})\, p\right] + \frac{1}{2\Omega}\sum_{i,j}\frac{\partial^2}{\partial x_i \partial x_j}\left[D_{ij}(\mathbf{x})\, p\right]$$

where $\mathbf{f}(\mathbf{x})$ is the deterministic drift (the ODE right-hand side) and $D_{ij}$ is the diffusion matrix determined by the reaction stoichiometries and propensities. The factor $1/\Omega$ shows that fluctuations scale as the inverse square root of system volume — they vanish in the thermodynamic limit. This is the mathematical statement of why ODE models become valid when molecule numbers are large.

The FPE is equivalent to the Langevin equation (Chemical Langevin Equation) and provides a bridge between the discrete CME and continuous SDE descriptions.

## Numerical Solution Strategies

The CME is a high-dimensional linear ODE system. For even a few molecular species, the state space is enormous (all combinations of molecule numbers). Direct numerical integration of the CME is feasible only for:
- Very small state spaces (few species, low copy numbers)
- Finite state projection (truncating the state space at a maximum molecule number)

For most biologically realistic networks, the CME is solved by **simulation** (Section 2.1.3.3: Gillespie Algorithm) rather than by direct numerical integration. Each Gillespie trajectory is one sample from the CME distribution — exactly like drawing one sample from a probability distribution rather than computing the distribution analytically.

## Why This Matters

The CME is the theoretical foundation of stochastic chemical kinetics. Every stochastic simulation method — the Gillespie Algorithm, tau-leaping, the Chemical Langevin Equation — is either an exact sampler from the CME distribution (Gillespie) or an approximation to it. Understanding the CME clarifies exactly what biological quantity a stochastic simulation is computing: a sample trajectory from the probability distribution over molecular states.

This distinction between the distribution and any particular trajectory is fundamental to interpreting simulation results, computing means and variances, and understanding noise decomposition. When you run 1000 Gillespie simulations and compute the histogram of mRNA counts at steady state, you are numerically computing the stationary distribution of the CME. When the histogram is Poisson (Fano = 1), the system is producing mRNA constitutively. When it is super-Poisson (Fano > 1), something is creating correlations — bursting, switching, feedback. The CME framework gives you the language to say precisely what.
