# 27.1.1 The Ising Problem and Why Physical Solvers Are Tempting

## The Problem

The Ising problem asks for the spin configuration $\{\sigma_i\}$, $\sigma_i \in \{-1, +1\}$, that minimizes the energy

$$H = -\sum_{i<j} J_{ij}\,\sigma_i \sigma_j - \sum_i h_i\,\sigma_i,$$

where $J_{ij}$ is the coupling matrix (ferromagnetic where positive, antiferromagnetic where negative) and $h_i$ are local fields. For arbitrary $J_{ij}$ — the *spin glass* case — finding the ground state is NP-hard; even for couplings restricted to a planar lattice with a field, the problem remains hard [Barahona, *J. Phys. A*, 1982]. The equivalent formulation over binary variables $x_i \in \{0,1\}$ is called QUBO (quadratic unconstrained binary optimization); the two are related by $\sigma_i = 2x_i - 1$.

The Ising form matters because it is a *lingua franca*: an enormous catalog of practically important problems — MaxCut, graph coloring, traveling salesman, number partitioning, portfolio selection, spin-glass models of protein folding — reduce to it with polynomial overhead, using penalty terms to encode constraints [Lucas, *Frontiers in Physics*, 2014]. Build one good Ising minimizer and you have, in principle, a heuristic engine for all of them.

Two caveats must be installed immediately, because they govern everything in Sections 27.1.2–27.1.3:

1. **NP-hardness is a worst-case statement.** No physical machine is expected to solve NP-hard problems in polynomial time in the worst case; the realistic goal is a *heuristic* that finds good (often optimal) solutions faster, or at lower energy, than classical heuristics on instance classes people actually care about.
2. **Reductions have costs.** Mapping a constrained problem into Ising form introduces penalty weights that must be balanced, and often inflates precision requirements on $J_{ij}$ — a serious issue for analog machines whose couplings have perhaps 4–8 bits of effective precision (Chapter 25).

## Why a Physical Solver?

The temptation is structural. The Ising energy is exactly the kind of function physics minimizes on its own: put $N$ bistable elements in a network with pairwise interactions, add noise and dissipation, and the system performs something like parallel, analog gradient descent with built-in fluctuation-driven escape from local minima — all $N$ spins updating simultaneously, in physical time, with no instruction stream. Compare the digital alternative: simulated annealing evaluates spin flips serially (or in limited parallel batches), paying memory access and instruction overhead per update.

The generic recipe for any *physical Ising machine* is:

- **a spin**: a bistable or phase-bistable physical element (an optical parametric oscillator's 0/π phase, a laser's polarization or mode, an electronic oscillator's phase, a pixel's transmission);
- **a coupling fabric**: some way to make element $i$'s state influence element $j$'s gain or field at strength $J_{ij}$;
- **an annealing schedule**: a control parameter (pump power, gain) swept so the system passes from a soft, exploratory regime into a frozen, decisive one;
- **readout**: measure all phases/states; report the sign pattern as $\{\sigma_i\}$.

Photonics offers particularly attractive raw materials: OPO phase bistability provides clean binary spins (27.1.2); optical interference implements the weighted sums $\sum_j J_{ij}\sigma_j$ passively; and round-trip times of nanoseconds to microseconds give update rates far beyond thermal annealers.

## The Competitive Landscape

Photonic Ising machines compete not only with CPUs. The relevant field circa 2025 includes quantum annealers (D-Wave's superconducting flux qubits), digital "annealer" ASICs/FPGAs, GPU-implemented heuristics — notably Toshiba's simulated bifurcation machine, a classical algorithm inspired by nonlinear Hamiltonian dynamics [Goto et al., *Science Advances*, 2019] — and spatial-photonic and opto-electronic machines (27.1.2). The comparative review by Mohseni, McMahon, and Byrnes [*Nature Reviews Physics*, 2022] is the standard map of this territory, and its sobering theme previews 27.1.3: well-implemented classical baselines are very hard to beat, and many published speedups dissolve when the baseline is competent.

Keep that in mind as we now build the most celebrated of these machines.
