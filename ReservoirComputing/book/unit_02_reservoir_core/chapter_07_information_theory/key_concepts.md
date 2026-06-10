# Chapter 7: Key Concepts

**1. Information-Processing Capacity ($C_f$)**
The fraction of variance in a target function $f$ of the input history that can be explained by a linear readout from the reservoir state. Defined as $C_f = \operatorname{Var}[\hat{y}_f] / \operatorname{Var}[f]$, where $\hat{y}_f$ is the optimal linear reconstruction. Equivalently, this is the $R^2$ coefficient of the best linear regression of $f$ on the reservoir state $\mathbf{r}(t)$.

**2. Total Information-Processing Capacity**
The sum $C_{total} = \sum_j C_{b_j}$ over a complete orthonormal basis $\{b_j\}$ of target functions. By the Parseval identity and rank arguments, this sum is independent of the choice of basis and satisfies $C_{total} \leq N$. It measures the total number of distinct scalar functions of the input history that the reservoir can independently encode.

**3. The $C_{total} \leq N$ Bound**
The fundamental limit on reservoir information processing: a reservoir with $N$ neurons can encode at most $N$ orthogonal scalar functions of the input history. The proof uses the rank of the state covariance matrix and the Parseval identity. The bound is tight for linear reservoirs with orthogonal weight matrices and zero-mean input.

**4. Capacity Decomposition**
The total capacity decomposes additively over orthogonal target functions: $C_{f+g} = C_f + C_g$ when $\langle f, g \rangle = 0$. This allows the capacity budget to be parceled out into contributions from linear memory (Jaeger's MC), quadratic memory, cross-delay nonlinear terms, etc., giving a "fingerprint" of what the reservoir is computing.

**5. Memory Capacity ($MC_k$ and $MC$)**
The $k$-step memory capacity $MC_k$ is the maximum $R^2$ achievable by a linear readout of the current reservoir state for predicting the input $k$ timesteps ago. Total memory capacity $MC = \sum_k MC_k \leq N$. This is the Jaeger (2002) special case of the Dambre framework, restricted to linear target functions.

**6. Geometric Memory Decay**
For a linear reservoir with spectral radius $\rho$, the memory profile decays geometrically: $MC_k \propto \rho^{2k}$. This formula, derived from the impulse response expansion, quantifies the trade-off between spectral radius and memory timescale. Larger $\rho$ means slower decay and longer effective memory, at the cost of reduced stability margin.

**7. State Covariance Matrix and Rank**
The state covariance matrix $R_{\mathbf{rr}} = \mathbb{E}[\mathbf{r}(t)\mathbf{r}(t)^\top]$ plays a central role in all capacity calculations. Its rank determines how many independent directions of state space are active, and thus how much capacity is available. A full-rank $R_{\mathbf{rr}}$ is necessary (though not sufficient) for achieving $C_{total} = N$.

**8. The Nonlinearity-vs-Memory Trade-off**
Capacity spent on nonlinear transformations of the input (quadratic and higher-degree basis functions) is capacity not available for linear memory of the input history. This trade-off is fundamental and unavoidable for fixed $N$. Designing reservoirs for tasks requiring both long memory and strong nonlinearity therefore requires large $N$.

**9. Echo State Property and Capacity**
The echo state property (ESP) is the precondition for well-defined memory capacity: if the reservoir state is not uniquely determined by the input history (i.e., two initial conditions lead to different asymptotic trajectories), then $MC_k$ cannot be consistently defined. The ESP ensures that the reservoir state is a deterministic functional of the input history, making all capacity quantities well-defined.

**10. Transfer Entropy**
Transfer entropy $T_{X \to Y}$ measures the directed information flow from process $X$ to process $Y$, beyond what $Y$'s own past predicts. In the reservoir context, $T_{u \to r_i}$ measures how much the input history contributes to neuron $i$'s dynamics above its autoregressive component. Transfer entropy is a nonlinear generalization of Granger causality and provides a complementary view to the linear memory capacity framework, capturing information flow in the full nonlinear sense.
