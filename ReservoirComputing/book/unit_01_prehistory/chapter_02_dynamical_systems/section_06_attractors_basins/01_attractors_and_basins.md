# 2.6 Attractors and Basin Structure

## What Is an Attractor?

Informally, an attractor is where a dynamical system ends up. Drop a marble anywhere on a landscape with a bowl, and it rolls to the bottom. Start a limit cycle oscillator anywhere in the right neighborhood, and it settles into the rhythm. The attractor is the long-time destiny of the system — the set that captures trajectories and holds them.

Formally:

**Definition.** A closed, bounded set $A \subset \mathbb{R}^n$ is an **attractor** of the system $\dot{\mathbf{x}} = f(\mathbf{x})$ if:
1. **Invariance:** $A$ is forward-invariant: if $\mathbf{x}(0) \in A$, then $\mathbf{x}(t) \in A$ for all $t > 0$.
2. **Attraction:** There exists an open neighborhood $U \supset A$ such that all trajectories starting in $U$ converge to $A$: $\text{dist}(\mathbf{x}(t), A) \to 0$ as $t \to \infty$.
3. **Minimality:** $A$ contains no proper subset satisfying conditions 1 and 2.

The minimality condition rules out trivially large sets (all of phase space is trivially forward-invariant and attracts everything in itself). It forces $A$ to be the smallest set with the attraction property — the irreducible core of the long-time behavior.

## Types of Attractors

The hierarchy of attractors, from simplest to most complex:

**Fixed points** ($\mathbf{x}^*$ with $f(\mathbf{x}^*) = 0$): Zero-dimensional attractors. The system comes to rest.

**Limit cycles**: One-dimensional closed curves. The system oscillates periodically.

**Invariant tori**: Two (or more) dimensional surfaces on which the system undergoes quasi-periodic motion — two or more incommensurable frequencies. Tori arise in Hamiltonian systems and in coupled oscillators.

**Strange attractors**: Fractally structured sets with non-integer Hausdorff dimension. The system is attracted to $A$ but wanders chaotically on it, never repeating. The Lorenz attractor, the Hénon attractor, and the Rössler attractor are the classical examples.

**Definition (Strange Attractor):** An attractor $A$ is **strange** if it is neither a fixed point nor a periodic orbit, and trajectories on it exhibit sensitive dependence on initial conditions ($\lambda_{\text{max}} > 0$).

Strange attractors are geometrically complex because they are the closure of an unstable manifold that is repeatedly folded back on itself — a process called **stretching and folding**. The stretching creates sensitivity (nearby points diverge); the folding maintains boundedness (the system does not escape to infinity). The fractal structure is the geometric record of this endless folding.

## Basin of Attraction

The **basin of attraction** of an attractor $A$ is the set of all initial conditions whose forward trajectories converge to $A$:

$$\mathcal{B}(A) = \{\mathbf{x}_0 \in \mathbb{R}^n : \text{dist}(\phi^t(\mathbf{x}_0), A) \to 0 \text{ as } t \to \infty\}$$

where $\phi^t$ is the flow map (the solution operator of the ODE).

For simple attractors, basins are simple: the basin of a stable fixed point is an open set bounded by stable manifolds of saddle points. The basin of the bowl-attractor is all of phase space above a certain energy threshold.

For systems with multiple attractors, basins can become extraordinarily complex:

**Riddled basins:** In some systems, the basin of one attractor is so interspersed with the basin of another that every open neighborhood of a point in $\mathcal{B}(A_1)$ intersects $\mathcal{B}(A_2)$. In such systems, long-run behavior is sensitive not just to the initial conditions (as in chaos) but to infinitesimally fine precision in measuring them — a kind of "super-chaotic" sensitivity [Alexander1992].

**Fractal basin boundaries:** When two or more attractors coexist, their basin boundaries are often fractal sets — the **Wada basins** phenomenon, where a single fractal curve bounds three or more distinct basins simultaneously.

## Multistability and Coexisting Attractors

A system is **multistable** if it has more than one attractor. Multistability is ubiquitous in nonlinear systems and is enormously important for computation.

**Example:** The Duffing oscillator $\ddot{x} + \delta\dot{x} - x + x^3 = F\cos(\omega t)$ can have coexisting large-amplitude and small-amplitude periodic attractors, with a fractal boundary between their basins.

**Computational significance:** Multistability allows a system to serve as an associative memory: different attractors correspond to different "memories," and the task of recall is dynamical — given a noisy or partial cue, the system falls into the nearest attractor's basin.

This is the principle behind **Hopfield networks** [Hopfield1982] — symmetric recurrent networks whose energy function landscape has local minima at stored memory patterns. It is also the principle behind **conceptors** (Chapter 12), which allow a reservoir to navigate among multiple stored dynamical patterns.

## Why Basin Structure Matters for Reservoir Computing

For reservoir computing, basin structure is relevant in several ways:

**1. Uniqueness of echo response.** The echo state property (Section 5.2) requires that the reservoir have a unique response to each input sequence, regardless of initial conditions. In geometrical terms, the reservoir must have a single attractor (in the sense of the pullback attractor — see Chapter 29) for each input sequence. This is equivalent to: the reservoir must have a single basin of attraction, with all initial conditions eventually converging to the same trajectory.

**2. Generative mode and output feedback.** When a reservoir is run in generative mode (with output fed back as input, Chapter 10), the reservoir + readout system may have multiple attractors corresponding to different learned patterns. The conceptors framework (Chapter 12) is essentially a tool for engineering the basin structure of a generative reservoir.

**3. Robustness.** A reservoir whose attractor has a large basin is robust to perturbations — small noise in the initial conditions or inputs is corrected by the contracting dynamics. A reservoir near an instability may have a small, fragile basin, making it sensitive to perturbations.

**4. Physical reservoirs and noise.** Physical reservoirs (Unit VII) are subject to thermal noise and manufacturing variability. The basin structure determines how robustly the reservoir can maintain its computed state under these perturbations.

## The Reservoir as a Pulled Attractor

There is a beautiful way to think about the role of input in reservoir computing that combines everything in this section.

Without input, the reservoir has its own attractor $A_0$ — perhaps a stable fixed point, or a limit cycle, or a chaotic attractor, depending on $\rho$.

With input $u_t$, the reservoir's dynamics are time-varying: the attractor changes at every time step as $u_t$ changes. This is a non-autonomous system, and the relevant concept is not a classical attractor but a **pullback attractor** — the attractor of the system as it has been driven by all past inputs (Chapter 29).

The echo state property is the requirement that this pullback attractor is a single point (or single curve, for continuous-time) for each input sequence — a 0-dimensional pullback attractor. The reservoir "forgets" its initial conditions because the input drives it to a unique trajectory.

The richness of that trajectory — how much it varies, how many distinct patterns it explores — is determined by the reservoir's dynamics. A rich attractor $A_0$ (high-dimensional, chaotic) generally produces a richer pullback attractor, encoding more information about the input history. This is why operating near the edge of stability ($\rho \approx 1$) increases computational performance, up to the point where the echo state property breaks down.

---

## References

- [Alexander1992] Alexander, J.C., Yorke, J.A., You, Z., & Kan, I. (1992). Riddled basins. *International Journal of Bifurcation and Chaos*, 2(4), 795–813.
- [Hopfield1982] Hopfield, J.J. (1982). Neural networks and physical systems with emergent collective computational abilities. *Proceedings of the National Academy of Sciences*, 79(8), 2554–2558.
- [Strogatz2018] Strogatz, S.H. (2018). *Nonlinear Dynamics and Chaos*, 2nd ed. CRC Press. Chapter 6 covers phase plane analysis and basin structure.
- [Ott2002] Ott, E. (2002). *Chaos in Dynamical Systems*, 2nd ed. Cambridge University Press. Chapter 3 covers strange attractors and fractal basins rigorously.
- [Jaeger2001] Jaeger, H. (2001). The 'echo state' approach to analysing and training recurrent neural networks. GMD Technical Report 148. Section 2 introduces the echo state property in terms of initial condition forgetting.
