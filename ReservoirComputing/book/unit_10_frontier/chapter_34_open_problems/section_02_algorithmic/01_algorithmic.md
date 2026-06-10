# Open Algorithmic Problems in Reservoir Computing

## 34.2.1 The Algorithmic Frontier

The preceding theoretical chapters have established rigorous foundations for reservoir computing: universal approximation (Chapter 26), spectral properties of random matrices (Chapter 27), generalization bounds (Chapter 28), and ergodic characterizations of the ESP (Chapter 29). These results tell us what reservoirs *can* do in principle. The algorithmic questions are harder: how do we find reservoirs that actually achieve these theoretical guarantees, and what are the limits of current training algorithms?

This section catalogs the major open algorithmic problems. For each problem, we describe what is known, what is unknown, and why the problem is difficult. In the tradition of Hilbert's problems in mathematics and Cook's theorem in computational complexity, we aim to state these problems precisely enough that future researchers can recognize when progress has been made.

## 34.2.2 Problem 1: Optimal Reservoir Design

**Problem.** Given a task class $\mathcal{T}$ (e.g., NARMA-$k$, Lorenz prediction, speech recognition), what is the optimal reservoir weight matrix $W^{\text{rec}}$ that maximizes the information processing capacity (IPC) [Dambre et al. 2012] for tasks in $\mathcal{T}$?

**What is known.** [Dambre et al. 2012] proved that the total IPC (sum over all orthogonal basis functions) is bounded by $N$ (the reservoir size) for any reservoir satisfying the ESP. The bound is achieved by a linear reservoir (diagonal $W^{\text{rec}}$ with unit entries): $\mathrm{IPC}_{\mathrm{total}} = N$. Nonlinear reservoirs sacrifice linear memory capacity for nonlinear processing capacity.

The spectral radius $\rho(W^{\text{rec}})$ affects the memory-nonlinearity tradeoff: small $\rho$ gives more nonlinear capacity; large $\rho < 1$ gives more linear memory. Topology (random vs. ring vs. clustered) affects which specific IPC components are large.

**What is unknown.** For a specific nonlinear task $f(u_{t-k_1}, \ldots, u_{t-k_n})$ (e.g., compute the XOR of inputs $k_1$ and $k_2$ steps ago), what is the optimal $W^{\text{rec}}$? No closed-form answer exists. The optimization landscape $\mathrm{IPC}_f(W^{\text{rec}})$ is highly nonconvex and task-specific.

**Why it is hard.** The IPC is defined as a quadratic function of the reservoir states (via covariance matrices), but $W^{\text{rec}}$ enters through a complex nonlinear dynamical system. The resulting objective is neither convex nor has known efficient global optimization algorithms.

**Partial results.** Evolutionary algorithms [Section 9.4] can optimize $W^{\text{rec}}$ for specific tasks, but without theoretical guarantees. The **self-organized criticality** hypothesis [Langton 1990] suggests that reservoirs near the edge of chaos maximize IPC, but this is a heuristic rather than a theorem.

## 34.2.3 Problem 2: Convergence of FORCE Learning

**Problem.** Under what conditions does FORCE learning [Sussillo & Abbott 2009] converge, and at what rate?

**Background.** FORCE (First-Order Reduced and Controlled Error) learning is an online algorithm for training the *internal* weights of a reservoir to generate target patterns autonomously. The algorithm modifies $W^{\text{rec}}$ at each time step using a recursive least squares (RLS) update to minimize the output error.

**What is known.** [Sussillo & Abbott 2009] demonstrated empirically that FORCE converges for smooth periodic targets (sine waves, van der Pol oscillations) in reservoirs with $N = 500$–$5000$ neurons. [Rainer & Mayr 2019] proved local convergence under assumptions of small initial error and sufficiently large $N$.

**What is unknown.** (a) Global convergence: if FORCE starts far from the target, does it converge? (b) Convergence rate: how many time steps $T$ are needed to achieve error $\varepsilon$? (c) Which target patterns can FORCE learn? Empirically, FORCE fails on chaotic targets and high-frequency patterns; no theoretical characterization of the learnable class is known.

**Why it is hard.** FORCE modifies $W^{\text{rec}}$ while running the reservoir, creating a time-varying dynamical system. Standard convergence proofs for RLS assume a fixed linear system; FORCE's nonlinear, time-varying, closed-loop setting requires new mathematical tools.

## 34.2.4 Problem 3: ESP-Preserving Reservoir Adaptation

**Problem.** Can the reservoir weights $W^{\text{rec}}$ be updated online while guaranteeing that the ESP is preserved throughout?

**Background.** In standard reservoir computing, $W^{\text{rec}}$ is fixed at initialization. But biological neural networks update their connections continuously (via plasticity), and engineering applications may require adaptation to changing tasks. Updating $W^{\text{rec}}$ online creates a moving reservoir, and the ESP may be violated if the update pushes $\rho(W^{\text{rec}})$ above 1 or changes the operator norm unfavorably.

**What is known.** [Steil 2004] showed that the **backpropagation decorrelation (BpD)** algorithm can update $W^{\text{rec}}$ while approximately preserving the ESP for small updates. **Intrinsic plasticity** [Section 9.5] can adapt neuron gain and bias without modifying $W^{\text{rec}}$, preserving the ESP by construction.

**What is unknown.** Are there ESP-preserving update rules that can change the effective spectral radius continuously while maintaining the ESP throughout? Can the update rule be made to increase IPC monotonically without violating ESP?

**Formal question.** Define the set of ESP-preserving perturbations of $W^{\text{rec}}$:

$$
\mathcal{E}_{W^{\text{rec}}} = \left\{\Delta W : W^{\text{rec}} + \Delta W \text{ satisfies the ESP}\right\}.
$$

Is $\mathcal{E}_{W^{\text{rec}}}$ convex? Connected? What is the largest $\|\Delta W\|_\mathrm{op}$ that guarantees ESP preservation?

## 34.2.5 Problem 4: Multi-Task Reservoir Learning

**Problem.** Can a single reservoir solve $K$ distinct tasks simultaneously without interference, and what is the optimal task allocation strategy?

**Background.** [Jaeger 2014] introduced **conceptors** — $N \times N$ projection matrices $C^{(k)}$ that "softly" project the reservoir state onto subspaces relevant to each task. Conceptors allow storing and retrieving $K$ patterns without interference, as long as the task-relevant subspaces are approximately orthogonal.

**What is known.** Conceptors can store up to $\sim N$ distinct patterns with zero interference when the pattern-induced reservoir state matrices are orthogonal [Jaeger 2014]. Conceptor composition allows combining patterns (AND, OR, NOT operations over subspaces).

**What is unknown.** (a) What is the maximum number $K^*$ of tasks that can be solved by an $N$-dimensional reservoir with at most $\varepsilon$ interference between tasks? (b) How does $K^*$ scale with $N$ for general nonlinear tasks? (c) Is there a computationally efficient algorithm to find the $K^*$ optimal task-adapted conceptors given only training data?

**Why it matters.** Multi-task learning with a single reservoir is directly relevant to embedded systems (one physical reservoir for multiple functions) and to understanding how the brain solves multiple tasks with the same neural substrate.

## 34.2.6 Problem 5: Sample Complexity of FORCE

**Problem.** Given a target periodic pattern with period $T_{\text{target}}$, how many time steps of FORCE training are needed to achieve output error $< \varepsilon$?

**What is known.** Empirically, FORCE requires $T \sim O(T_{\text{target}} / \varepsilon^2)$ time steps [Sussillo & Abbott 2009]. No theoretical justification for this scaling has been published.

**Why it matters.** For control applications with slowly varying targets ($T_{\text{target}} \gg 1$), FORCE learning may be too slow. Understanding the sample complexity would guide the design of faster learning algorithms.

## 34.2.7 Summary Table

| Problem | Known | Unknown | Difficulty |
|---|---|---|---|
| Optimal $W^{\text{rec}}$ | IPC bounds; edge-of-chaos heuristic | Optimal topology for task class | Nonconvex, task-specific |
| FORCE convergence | Empirical; local convergence | Global convergence; rate | Time-varying, closed-loop RLS |
| ESP-preserving adaptation | Small-update BpD; intrinsic plasticity | General ESP-preserving update rule | Constraint manifold geometry |
| Multi-task capacity | Conceptors; orthogonal subspaces | $K^*(N)$ scaling for nonlinear tasks | Combinatorial, nonlinear |
| FORCE sample complexity | Empirical $O(T_\text{target}/\varepsilon^2)$ | Theoretical justification | Nonlinear RLS analysis |

## References

- Dambre, J., Verstraeten, D., Schrauwen, B., and Massar, S. (2012). Information processing capacity of dynamical systems. *Scientific Reports*, 2(1), 514.
- Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv:1403.3369*.
- Langton, C. G. (1990). Computation at the edge of chaos: Phase transitions and emergent computation. *Physica D*, 42(1–3), 12–37.
- Rainer, F. and Mayr, C. (2019). Local online learning in recurrent networks with random feedback connections. In *Proceedings of NeurIPS Workshop on Biological and Artificial RL*.
- Steil, J. J. (2004). Backpropagation decorrelation: Online recurrent learning with $O(N)$ complexity. In *Proceedings of IJCNN 2004*, 2, 843–848.
- Sussillo, D. and Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
