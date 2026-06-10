# Section 9.4: Evolutionary Approaches to Reservoir Design

## 9.4.1 When Gradient-Free Global Search Is Needed

Structured initialization (Section 9.1) and task-informed adaptation (Section 9.3) work well when the structure of the optimal reservoir is known in advance — when we can guess that a ring topology maximizes memory, or that IP will improve neuron utilization. But some tasks require reservoir configurations that defy intuition: unusual connectivity patterns, non-standard weight distributions, or reservoirs with carefully coordinated heterogeneity that no analytical argument anticipates.

Evolutionary and neuroevolution methods search the space of reservoir configurations by iterating a population of candidate reservoirs, evaluating each on the task, and breeding new candidates from successful parents. They require no gradient information and make no assumptions about the structure of the optimal solution. The price is computational: evolutionary approaches are expensive, requiring hundreds to thousands of full reservoir evaluations. But when the search space is rugged and structured methods fail, evolutionary approaches can find unusual high-performing configurations that no other method discovers.

## 9.4.2 Genetic Algorithms for Reservoir Weights

A *genetic algorithm* (GA) [Holland1992] maintains a population of $P$ reservoir configurations (chromosomes), evaluates each on the task (fitness), and produces the next generation by selection, crossover, and mutation:

1. **Initialization.** Create $P$ random reservoir weight matrices $(W^{rec}_1, \ldots, W^{rec}_P)$ drawn from the standard distribution (Section 5.4).
2. **Evaluation.** Train the readout for each reservoir and compute the validation NRMSE as the fitness.
3. **Selection.** Select $P/2$ parent pairs from the population, weighted by fitness (e.g., tournament selection or rank-proportional selection).
4. **Crossover.** For each parent pair $(W_a, W_b)$, produce two offspring by exchanging subsets of weights (e.g., swap entire rows — "neuron crossover" — or swap individual entries — "weight crossover").
5. **Mutation.** Randomly perturb a small fraction of weights in each offspring (e.g., add Gaussian noise $\Delta W_{ij} \sim \mathcal{N}(0, \sigma_{mut}^2)$).
6. **Replacement.** Replace the old population with the offspring, retaining the best individual (elitism).
7. **Repeat** steps 2-6 for $G$ generations.

**Fitness landscape.** The fitness landscape for GA on reservoir weights is nearly flat: any two reservoirs with the same spectral radius but different weight matrices perform similarly (by the random feature argument, Section 4.4). GA therefore struggles to make progress on the weight space directly — the landscape has too many equivalent configurations and too few strong local gradients.

GA is more effective when applied to the *topology* (which connections exist) rather than the weights, or to the *hyperparameters* rather than individual weights. In these lower-dimensional spaces, the fitness landscape has more structure.

## 9.4.3 Evolution Strategies and CMA-ES

*Evolution Strategies* (ES) [Rechenberg1973] optimize continuous parameters by maintaining a distribution over the parameter space and updating it based on fitness. The Covariance Matrix Adaptation ES (CMA-ES) [HansenOstermeier2001] — described as a hyperparameter optimizer in Section 8.8 — is equally applicable to reservoir weight optimization.

**Application to reservoir design.** When applying CMA-ES to reservoir weights, the search space is $N^2$-dimensional — far too large for standard CMA-ES (which is practical for $d \leq 10^3$). The solution is to parameterize the reservoir indirectly:

- **Hyperparameter space:** Apply CMA-ES to the vector $(\rho, \sigma_{in}, \alpha, \lambda, p, \sigma_b)$ (6-10 dimensions). This is the most common and effective use of CMA-ES for reservoirs.
- **Matrix scaling:** Apply CMA-ES to a diagonal scaling matrix $D$, so the reservoir is $W^{rec} = D \tilde{W} D^{-1}$ for a fixed random $\tilde{W}$. This adapts the timescale of each neuron without changing the connectivity.
- **Block structure:** Apply CMA-ES to block-structured weight matrices, where each block represents a population of neurons (e.g., excitatory/inhibitory populations), reducing the search dimension to $O(B^2)$ for $B$ blocks.

## 9.4.4 NEAT Applied to Reservoirs

NeuroEvolution of Augmenting Topologies (NEAT) [StanleyMiikkulainen2002] evolves both the topology (which neurons and connections exist) and the weights of a neural network simultaneously. Applied to reservoirs, NEAT can discover non-standard topologies — small-world subgraphs, hierarchical modules, or heterogeneous neuron types — that outperform both random and hand-designed topologies on specific tasks.

**NEAT for reservoir topology.** The NEAT algorithm begins with a minimal reservoir (few neurons, sparse connectivity) and adds neurons and connections through mutation, guided by the improvement in task performance. Crossover operates on topologies of different sizes by aligning neurons using a historical marker (innovation number). The population is divided into species (topologically similar reservoirs) to protect innovations from being immediately outcompeted by the larger existing population.

**Key finding.** NEAT applied to reservoir design tends to discover *modular* reservoirs: groups of tightly coupled neurons performing specialized computation (e.g., a ring subnetwork for memory, a densely connected subnetwork for nonlinear mixing) connected by sparse long-range connections. These structures echo the modular organization of biological cortex and are not discoverable by random initialization.

The cost is high: NEAT requires $O(P \cdot G)$ full reservoir evaluations, where $P = 100$-$500$ (population size) and $G = 50$-$200$ (generations). For a task requiring 10 seconds per evaluation, this is 14-280 hours of computation.

## 9.4.5 Indirect Encoding

A key insight from neuroevolution research is that *indirect encodings* — evolving a recipe for generating the reservoir, rather than evolving the reservoir weights directly — are more efficient for large reservoirs [Stanley2007].

**Hypercube-based encoding (HyperNEAT).** Rather than evolving $W_{ij}$ directly, HyperNEAT evolves a *compositional pattern-producing network* (CPPN) that maps the spatial coordinates $(i/N, j/N)$ of a connection to its weight:

$$W_{ij} = \text{CPPN}(i/N, j/N; \boldsymbol{\phi}),$$

where $\boldsymbol{\phi}$ are the evolved CPPN parameters. A CPPN with a few dozen parameters can generate a reservoir with $N^2$ weights, all consistent with a geometrically regular pattern. This forces the evolved reservoir to have spatial regularity — a strong inductive bias that is computationally favorable.

HyperNEAT-evolved reservoirs [Verstraeten2010] have been shown to outperform random reservoirs on tasks with spatial or temporal structure, because the evolved CPPN naturally produces weight patterns that are adapted to the regularity of the task's input.

## 9.4.6 The Fitness Landscape: Structure in Hyperparameter Space

The fitness landscape of reservoir computing has a characteristic structure:
- **Smooth in hyperparameter space:** The validation loss as a function of $(\rho, \sigma_{in}, \alpha, \lambda)$ is typically smooth with broad optima. Small perturbations of hyperparameters produce small changes in performance.
- **Rugged in weight space:** The validation loss as a function of individual weights $W_{ij}$ is nearly flat with respect to any single weight (the random feature argument), but has exponentially many local optima corresponding to different connectivity patterns.

This structure implies that evolutionary methods should operate in hyperparameter space for efficiency, and in weight/topology space only when structured novelty (modular topology, heterogeneous neurons) is the target.

**Practical recommendation.** For most tasks, use CMA-ES in hyperparameter space (Section 8.8) rather than GA or NEAT in weight space. Reserve NEAT and HyperNEAT for cases where you have strong reason to believe that structured topology matters — e.g., physical reservoir computing (Chapter 16-19) where the physical substrate constrains the topology, or neuromorphic hardware implementations where the connection structure is fixed and only neuron placement can be optimized.

---

## References

- **[HansenOstermeier2001]** N. Hansen and A. Ostermeier. "Completely derandomized self-adaptation in evolution strategies." *Evolutionary Computation*, 9(2):159-195, 2001.
- **[Holland1992]** J. H. Holland. *Adaptation in Natural and Artificial Systems*. MIT Press, 1992.
- **[Rechenberg1973]** I. Rechenberg. *Evolutionsstrategie: Optimierung technischer Systeme nach Prinzipien der biologischen Evolution*. Frommann-Holzboog, 1973.
- **[StanleyMiikkulainen2002]** K. O. Stanley and R. Miikkulainen. "Evolving neural networks through augmenting topologies." *Evolutionary Computation*, 10(2):99-127, 2002.
- **[Stanley2007]** K. O. Stanley, D. B. D'Ambrosio, and J. Gauci. "A hypercube-based encoding for evolving large-scale neural networks." *Artificial Life*, 15(2):185-212, 2009.
- **[Verstraeten2010]** D. Verstraeten, B. Schrauwen, and D. Stroobandt. "Reservoir-based techniques for speech recognition." *Proceedings of the World Congress on Computational Intelligence*, 2006.
