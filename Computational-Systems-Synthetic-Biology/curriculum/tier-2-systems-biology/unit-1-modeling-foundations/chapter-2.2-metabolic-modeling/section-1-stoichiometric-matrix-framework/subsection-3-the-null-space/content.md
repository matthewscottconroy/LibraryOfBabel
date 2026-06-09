# The Null Space of the Stoichiometric Matrix

## Linear Algebra of Metabolic Networks

If the steady-state constraint $\mathbf{S} \cdot \mathbf{v} = \mathbf{0}$ defines which flux distributions are biologically feasible, then the question becomes geometric: what does the set of all feasible solutions look like? This is precisely the question that null space analysis answers, and the answer is far richer than you might expect.

The **null space** (or **kernel**) of the stoichiometric matrix $\mathbf{S}$ is the set of all flux vectors $\mathbf{v}$ satisfying $\mathbf{S} \cdot \mathbf{v} = \mathbf{0}$:

$$\mathcal{N}(\mathbf{S}) = \{\mathbf{v} \in \mathbb{R}^n : \mathbf{S} \cdot \mathbf{v} = \mathbf{0}\}$$

This is the space of all possible steady-state flux distributions — every feasible metabolic state (without considering flux bounds) lies in the null space of $\mathbf{S}$. Understanding the structure of this null space is fundamental to understanding what flux distributions are even possible for a given network.

## Dimensionality and Degrees of Freedom

The dimension of the null space equals the number of **degrees of metabolic freedom**: the number of independent flux values that can be freely chosen once the steady-state constraint is imposed.

By the rank-nullity theorem:

$$\dim(\mathcal{N}(\mathbf{S})) = n - \text{rank}(\mathbf{S})$$

where $n$ is the number of reactions and $\text{rank}(\mathbf{S})$ is the number of linearly independent rows of $\mathbf{S}$ (which equals the number of linearly independent metabolite balance equations).

**Interpretation**: if $n = 100$ reactions and $\text{rank}(\mathbf{S}) = 80$, then $\dim(\mathcal{N}) = 20$. The network has 20 degrees of freedom: 20 fluxes can be independently specified (within bounds), and the remaining 80 are determined by the steady-state equations.

```python
import numpy as np
from scipy.linalg import null_space

# Example stoichiometric matrix (from Section 2.2.1.1)
S = np.array([
    [-1,  0,  0, -1,  0],
    [+1, -1,  0,  0,  0],
    [ 0, -1, +1,  0,  0],
    [ 0, +1, -1,  0,  0],
    [ 0,  0, +1,  0, -1],
], dtype=float)

rank = np.linalg.matrix_rank(S)
n_reactions = S.shape[1]
dim_null = n_reactions - rank

print(f"Reactions: {n_reactions}")
print(f"Rank of S: {rank}")
print(f"Null space dimension: {dim_null}")

# Compute null space basis vectors
N = null_space(S)
print(f"Null space basis shape: {N.shape}")
print("Basis vectors:")
print(np.round(N, 3))
```

## The Null Space Basis: Pathway Interpretation

Each basis vector of the null space represents an independent mode of steady-state operation — a **flux pattern** that can be added to any feasible flux distribution and yield another feasible flux distribution. In metabolic terms, these basis vectors correspond to elementary metabolic pathways. The basis of the null space is not just an algebraic object; it is a decomposition of the cell's metabolic repertoire into its fundamental building blocks.

**Elementary Flux Modes (EFMs)**: the elementary rays of the null space cone (intersected with flux bounds) — the irreducible, minimal steady-state pathways. Every feasible flux distribution can be decomposed as a non-negative linear combination of EFMs. This is the theoretical basis for pathway analysis.

**Extreme Pathways (ExPa)**: a subset of EFMs that spans the null space cone and satisfies thermodynamic and reversibility constraints. Proposed by Palsson and colleagues as a basis for pathway analysis in genome-scale models.

## Worked Example: Glycolysis vs. Pentose Phosphate Pathway

Consider a simplified model where glucose can be metabolized through glycolysis (v1) or the pentose phosphate pathway (v2), both producing NADPH and pyruvate (with different stoichiometries). At steady state, the balance on each internal intermediate determines how much flux can go through each pathway simultaneously — but the ratio is unconstrained by stoichiometry alone (it is determined by enzyme kinetics and regulation).

The null space basis contains two vectors:
- One representing "all flux through glycolysis"
- One representing "all flux through the PPP"

Any mixture is feasible at steady state. The actual ratio in a given condition depends on factors (NADPH demand, G6P concentration, enzyme levels) not captured by stoichiometry. This is an important limitation to appreciate: the null space defines what is possible, but not what the cell actually chooses.

## The Left Null Space: Conservation Moieties

The **left null space** $\mathcal{N}(\mathbf{S}^\top)$ consists of vectors $\mathbf{l}$ such that $\mathbf{l}^\top \mathbf{S} = \mathbf{0}$. These vectors represent **conservation laws** in the metabolic network — quantities that are constant over time regardless of what fluxes are running.

A conservation moiety is a combination of metabolites whose total amount is conserved by all reactions in the network. Classic examples:
- **ATP + ADP + AMP = constant** (total adenosine nucleotide pool)
- **NAD + NADH = constant** (total nicotinamide pool)
- **Sum of hexose phosphates** may be conserved in some sub-networks

Conservation moieties arise because some atoms (the energy carriers, redox carriers) are not net produced or consumed — they are recycled. The left null space reveals these conserved quantities, which reduce the effective dimensionality of the ODE system and are important for model simplification.

## Implications for Constraint-Based Analysis

The null space structure determines what FBA can and cannot compute:

- **Fully determined reactions**: reactions that take the same value in all null space basis vectors are **coupled** — they always operate together. Their ratio is fixed by stoichiometry alone.
- **Free reactions**: reactions that appear in only one basis vector can vary independently within the null space.
- **Blocked reactions**: reactions that do not appear in any basis vector carry zero flux at steady state for stoichiometric reasons (regardless of kinetics or bounds).

Identifying blocked reactions before FBA analysis saves computation and reveals gaps in the network reconstruction (reactions that are stoichiometrically disconnected from the rest of the network).

## Why This Matters

The null space is the foundational geometric object of constraint-based metabolic modeling. FBA is simply the optimization of a linear objective over the feasible flux cone — which is the null space of $\mathbf{S}$ intersected with the flux bounds. Flux variability analysis explores the extent of the feasible cone in each direction. Understanding the null space structure — its dimension, its basis vectors, and which reactions are coupled — provides metabolic insights that go beyond any single FBA solution and reveals the fundamental architecture of metabolic networks.
