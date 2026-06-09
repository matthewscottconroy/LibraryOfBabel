# Markov Chains

The GENSCAN gene prediction algorithm, published in 1997, could look at a raw stretch of DNA sequence and identify the likely positions of genes with remarkable accuracy. At its core, it was doing something conceptually simple: modeling the sequence as a series of observations emitted by a hidden Markov model whose states corresponded to biological categories — exon, intron, promoter, splice site. The "hidden" states could not be observed directly; only the sequence itself could. But by training the HMM on known annotated sequences, the model learned the probabilistic signatures of each state, and the Viterbi decoding algorithm could then find the most likely assignment of states to each nucleotide position. The same mathematical framework is now used in ChromHMM to annotate chromatin states from histone mark data.

Markov chains are probabilistic models of systems that transition between discrete states over time, where the future depends only on the present state — not on the entire history. This **Markov property** makes these models mathematically tractable and biologically realistic: a cell's behavior at time $t+1$ is largely determined by its current molecular state, not by the full history of how it got there.

## Transition Matrices

A **discrete-time, finite-state Markov chain** is defined by:
- A state space $\mathcal{S} = \{s_1, s_2, \ldots, s_n\}$
- A **transition matrix** $P \in \mathbb{R}^{n \times n}$ where $P_{ij} = P(X_{t+1} = s_j | X_t = s_i)$

Each row of $P$ is a probability distribution (sums to 1). The distribution over states at time $t$ is the row vector $\boldsymbol{\pi}^{(t)}$, and:

$$\boldsymbol{\pi}^{(t+1)} = \boldsymbol{\pi}^{(t)} P, \quad \boldsymbol{\pi}^{(t)} = \boldsymbol{\pi}^{(0)} P^t$$

**Biological example — CpG island model:** DNA sequences can be modeled as Markov chains where the state is the current nucleotide. "CpG islands" (regions of high CpG dinucleotide frequency, associated with gene promoters) can be distinguished from bulk genome by training two Markov chains: one for CpG island sequences, one for non-island sequences. For a new sequence, the likelihood ratio of the two models determines classification. The Markov assumption — that the probability of the next nucleotide depends only on the current one — is a simplification, but a surprisingly effective one for this classification task.

## Stationary Distributions

A distribution $\boldsymbol{\pi}^*$ is **stationary** for chain $P$ if $\boldsymbol{\pi}^* = \boldsymbol{\pi}^* P$, i.e., $\boldsymbol{\pi}^*$ is a left eigenvector of $P$ with eigenvalue 1.

The stationary distribution satisfies the **global balance equations:**

$$\pi^*_j = \sum_i \pi^*_i P_{ij} \quad \text{for all } j$$

Interpretation: the probability flowing into state $j$ equals the probability flowing out at stationarity.

**Detailed balance (reversibility):** If $\pi^*_i P_{ij} = \pi^*_j P_{ji}$ for all $i, j$, the chain is **reversible** — it looks the same whether time runs forward or backward. This condition arises in thermodynamics (Kolmogorov criterion) and simplifies MCMC analysis. Biochemical systems at thermodynamic equilibrium satisfy detailed balance; driven systems out of equilibrium (like the cell cycle) do not, and this asymmetry is physically meaningful.

## Ergodicity and Mixing

An **ergodic** Markov chain is one that is:
- **Irreducible:** every state can reach every other state (the chain is connected)
- **Aperiodic:** the chain does not get stuck in cycles

For ergodic chains, the **Perron-Frobenius theorem** guarantees:
1. A unique stationary distribution $\boldsymbol{\pi}^*$ exists
2. $\boldsymbol{\pi}^{(t)} \to \boldsymbol{\pi}^*$ as $t \to \infty$, regardless of initial distribution

The **mixing time** $t_{\text{mix}}$ is the number of steps required to get within a small distance of stationarity. Fast mixing means the chain converges quickly — critical for MCMC efficiency and for assessing whether a gene regulatory network reaches its steady-state distribution quickly after a perturbation.

## Continuous-Time Markov Chains

Many biological processes occur in continuous time. A **continuous-time Markov chain (CTMC)** is defined by a **rate matrix** (generator) $Q$ where:
- $Q_{ij} \geq 0$ for $i \neq j$: rate of transitions from state $i$ to state $j$
- $Q_{ii} = -\sum_{j \neq i} Q_{ij}$: each row sums to zero

The time-evolution of probabilities satisfies the **master equation** (also called the chemical master equation in systems biology):

$$\frac{d\boldsymbol{\pi}(t)}{dt} = \boldsymbol{\pi}(t) Q$$

with solution $\boldsymbol{\pi}(t) = \boldsymbol{\pi}(0) e^{Qt}$. The **chemical master equation (CME)** is the fundamental stochastic description of biochemical reaction networks, tracking the probability distribution over all possible molecular counts. For most systems, the CME is too high-dimensional to solve exactly, motivating the Gillespie algorithm for simulation. This is the rigorous foundation underlying everything you will later learn about stochastic gene expression: the noise that Elowitz and colleagues measured in their two-reporter experiment is a consequence of the inherent stochasticity described by the CME.

## Hidden Markov Models

A **hidden Markov model (HMM)** extends the Markov chain framework by separating hidden states from observed emissions. The system transitions through hidden states $z_1, z_2, \ldots$ according to a Markov chain, and at each time step emits an observable $x_t$ drawn from an emission distribution that depends only on the current hidden state $z_t$.

**Parameters:**
- Initial distribution $\boldsymbol{\pi}^{(0)}$
- Transition matrix $A$ (between hidden states)
- Emission distribution $B$ (observations given hidden state)

**Three fundamental algorithms:**
1. **Forward algorithm:** Compute $P(\mathbf{x}_{1:T}|\text{model})$ — the likelihood of the observation sequence. Used for model scoring and comparison.
2. **Viterbi algorithm:** Find the most likely hidden state sequence $\mathbf{z}^* = \arg\max P(\mathbf{z}|\mathbf{x})$ — the "decoded" state sequence. Dynamic programming, $O(T \cdot K^2)$.
3. **Baum-Welch algorithm:** Estimate model parameters from unlabeled data — an EM algorithm. Used to train HMMs from sequence data.

**Biological applications:**
- **Gene prediction (GENSCAN, Augustus):** Hidden states represent exons, introns, intergenic regions, splice sites. Observations are DNA sequence. Viterbi decoding predicts gene structure.
- **CpG island detection:** Two hidden states (island, non-island) with state-specific nucleotide emission probabilities.
- **Protein secondary structure:** Three hidden states (helix, sheet, coil) emitting amino acids with state-specific frequencies.
- **Chromatin state annotation (ChromHMM):** Hidden states correspond to chromatin states (active promoter, enhancer, heterochromatin, etc.) emitting combinations of histone marks.

## Why This Matters for Computational Biology

Markov chains are among the most productive mathematical ideas in all of computational biology. Every major sequence annotation tool — gene prediction, repeat finding, transcription factor binding site scanning — is built on HMMs. MCMC algorithms, which power Bayesian inference throughout the field, are Markov chains by construction. The stochastic chemical kinetics of gene expression is modeled by continuous-time Markov chains (the chemical master equation). Phylogenetic models of sequence evolution are continuous-time Markov chains on sequence space. Understanding the theory behind Markov chains lets you understand why these tools work, diagnose when they fail, and extend them to new problems.

```python
import numpy as np
from scipy.linalg import expm

# Simple 2-state HMM: CpG island (state 0) vs non-island (state 1)
# Transition matrix
A = np.array([[0.9, 0.1],    # P(stay in island | island), P(leave | island)
              [0.02, 0.98]])  # P(enter island | non-island), P(stay | non-island)

# Emission probabilities (for each nucleotide A,C,G,T)
B = np.array([[0.20, 0.30, 0.30, 0.20],  # island: high C and G
              [0.30, 0.20, 0.20, 0.30]])  # non-island: high A and T

# Stationary distribution (dominant left eigenvector)
eigvals, eigvecs = np.linalg.eig(A.T)
stat_idx = np.argmin(np.abs(eigvals - 1.0))
pi_star = np.real(eigvecs[:, stat_idx])
pi_star /= pi_star.sum()
print(f"Stationary distribution: island={pi_star[0]:.3f}, non-island={pi_star[1]:.3f}")
# ~17% of genome in CpG islands -- unrealistically high for illustration, but shows principle

# Viterbi decoding on a short sequence
# Nucleotide encoding: A=0, C=1, G=2, T=3
sequence = np.array([1, 2, 1, 2, 0, 3, 0, 3, 1, 2, 1, 2])  # CGCGATAT CGCG
T, K = len(sequence), 2

viterbi = np.zeros((T, K))
backptr = np.zeros((T, K), dtype=int)
viterbi[0] = np.log(pi_star) + np.log(B[:, sequence[0]])

for t in range(1, T):
    for k in range(K):
        scores = viterbi[t-1] + np.log(A[:, k]) + np.log(B[k, sequence[t]])
        backptr[t, k] = np.argmax(scores)
        viterbi[t, k] = np.max(scores)

# Traceback
path = np.zeros(T, dtype=int)
path[-1] = np.argmax(viterbi[-1])
for t in range(T-2, -1, -1):
    path[t] = backptr[t+1, path[t+1]]

labels = ['island', 'non-island']
print("\nViterbi decoded path:")
for t, (nt_idx, state) in enumerate(zip(sequence, path)):
    nt = 'ACGT'[nt_idx]
    print(f"  Position {t}: {nt} -> {labels[state]}")
```
