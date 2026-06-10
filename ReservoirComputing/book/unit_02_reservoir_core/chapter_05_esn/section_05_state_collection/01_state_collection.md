# Section 5.5: Collecting Reservoir States for Training

## 5.5.1 From Dynamics to Data

Once the reservoir is constructed and fixed, training reduces to a linear regression problem. The inputs to that regression are the reservoir state vectors $\mathbf{x}(t)$, one for each time step. The process of driving the reservoir with the training input, recording these state vectors, and organizing them into a matrix is called *state collection*. It is mundane in implementation but conceptually important: the quality of the collected state matrix determines everything about what the readout can learn.

This section covers the full pipeline: the washout period, the state matrix, extended states, and practical variants for large reservoirs or long sequences [Jaeger2001, Lukosevicius2012].

## 5.5.2 The Washout Period

The reservoir is initialized at $\mathbf{x}(0) = \mathbf{0}$ (or some other arbitrary starting state). The first several steps of the dynamics are therefore contaminated by this initial condition rather than reflecting the input history alone. If we include these states in training, the readout will learn to compensate for the arbitrary initialization — fitting the training set poorly and generalizing even worse.

The *washout period* is the number of initial time steps $t_{wash}$ that are discarded. After $t_{wash}$ steps, the initial condition has decayed sufficiently that $\mathbf{x}(t_{wash})$ depends primarily on the recent input history and negligibly on $\mathbf{x}(0)$.

**Quantifying the washout.** For a linear reservoir $\mathbf{x}(t) = W^{rec} \mathbf{x}(t-1) + W^{in} \mathbf{u}(t)$, the contribution of the initial state after $t$ steps is $(W^{rec})^t \mathbf{x}(0)$, which has norm decaying as $\rho(W^{rec})^t \|\mathbf{x}(0)\|$. For the initial condition to contribute less than $\varepsilon$ relative to the driven state:

$$\rho(W^{rec})^{t_{wash}} < \varepsilon,$$

giving

$$t_{wash} > \frac{\log \varepsilon}{\log \rho(W^{rec})} = \frac{-\log(1/\varepsilon)}{\log(1/\rho)}.$$

For $\rho = 0.9$ and $\varepsilon = 10^{-3}$: $t_{wash} > 3\log(10) / \log(10/9) \approx 65$ steps. For $\rho = 0.99$: $t_{wash} > 690$ steps. The closer $\rho$ is to 1, the longer the memory and the longer the required washout.

**Rule of thumb.** A practical heuristic is $t_{wash} \approx 10 \tau_{\max}$, where $\tau_{\max} = -1 / \log \rho$ is the longest timescale in the reservoir (the time constant associated with the spectral radius). For $\rho = 0.9$, $\tau_{\max} \approx 9.5$, so $t_{wash} \approx 100$. For tasks with nonlinear dynamics, the effective memory may be shorter than $\tau_{\max}$, and a shorter washout can suffice [Lukosevicius2012].

## 5.5.3 The State Matrix

After discarding the washout period, collect the remaining states into the *state matrix*

$$X = \begin{pmatrix} \mathbf{x}(t_{wash}+1) & \mathbf{x}(t_{wash}+2) & \cdots & \mathbf{x}(T) \end{pmatrix} \in \mathbb{R}^{N \times T'},$$

where $T' = T - t_{wash}$ is the number of training time steps after washout. Each column is a single state vector (a snapshot of all $N$ neuron activations at one time step), and each row is the time series of a single neuron.

The target matrix is similarly truncated:

$$Y = \begin{pmatrix} \mathbf{y}(t_{wash}+1) & \mathbf{y}(t_{wash}+2) & \cdots & \mathbf{y}(T) \end{pmatrix} \in \mathbb{R}^{M \times T'},$$

where $M$ is the output dimension.

The readout weight matrix $W^{out} \in \mathbb{R}^{M \times N}$ is then found by solving the least-squares problem

$$W^{out} = \arg\min_{W} \|WX - Y\|_F^2 + \lambda \|W\|_F^2,$$

which has the closed-form solution $W^{out} = Y X^\top (XX^\top + \lambda I)^{-1}$ (Section 5.6).

## 5.5.4 Extended State Vectors

A simple and effective modification is to concatenate the input $\mathbf{u}(t)$ to the state vector before forming the state matrix:

$$\tilde{\mathbf{x}}(t) = \begin{pmatrix} \mathbf{x}(t) \\ \mathbf{u}(t) \end{pmatrix} \in \mathbb{R}^{N+K}.$$

This extended state matrix $\tilde{X} \in \mathbb{R}^{(N+K) \times T'}$ allows the readout to use the current input directly, without requiring the reservoir to propagate it through the recurrent dynamics. The additional $K$ rows add negligible cost but can substantially improve performance when the target depends directly on the current input [Lukosevicius2012].

It is also common to include nonlinear state features. Adding squared activations $x_i^2$ or products $x_i x_j$ for selected pairs enlarges the feature space further, at the cost of increased state matrix size. This is a mild form of feature engineering within the reservoir computing framework.

## 5.5.5 State Compression for Large Reservoirs

For reservoirs with $N = 10^4$ or larger, the state matrix $X \in \mathbb{R}^{N \times T'}$ may be too large to store in memory or to invert in the readout step ($O(N^3)$ for the matrix inverse). Two compression strategies are available:

**PCA compression.** Compute the principal components of $X$ and project onto the top $K_{PCA} \ll N$ components:

$$\hat{X} = U_{K_{PCA}}^\top X \in \mathbb{R}^{K_{PCA} \times T'},$$

where $U_{K_{PCA}}$ contains the top $K_{PCA}$ left singular vectors of $X$. This discards directions of low variance in the state space, keeping only the most active subspace. The cost is $O(N T' K_{PCA})$ for the projection, plus the cost of computing the SVD.

**Random projection compression.** Draw a random projection matrix $P \in \mathbb{R}^{K_{rp} \times N}$ with i.i.d. Gaussian entries and set $\hat{X} = PX / \sqrt{K_{rp}}$. By the Johnson-Lindenstrauss lemma (Section 4.4), this preserves pairwise distances approximately when $K_{rp} = O(\log T')$. This is faster than PCA ($O(NK_{rp} T')$ instead of a full SVD) and equally effective in practice for moderate compression ratios.

## 5.5.6 Subsampling

If the input has slow dynamics relative to the simulation time step, states at consecutive times are highly correlated. Training on all $T'$ states is redundant and expensive. *Subsampling* at every $k$-th step:

$$X^{sub} = \begin{pmatrix} \mathbf{x}(t_{wash}+k) & \mathbf{x}(t_{wash}+2k) & \cdots \end{pmatrix},$$

reduces the training set size to $T'/k$ without substantial information loss when $k$ is smaller than the autocorrelation time of the reservoir states. The subsampling rate $k$ is a secondary hyperparameter; $k = 1$ (no subsampling) is the default.

## 5.5.7 Online vs. Offline Collection

**Offline collection** runs the full training sequence, stores all states, and solves the linear system in one batch. This is the standard approach for moderate-length sequences and requires $O(N T')$ memory.

**Online collection** updates a pair of running sums (the correlation matrices needed for ridge regression) at each time step:

$$A \leftarrow A + \mathbf{x}(t)\mathbf{x}(t)^\top, \quad B \leftarrow B + \mathbf{y}(t)\mathbf{x}(t)^\top,$$

where $A \in \mathbb{R}^{N \times N}$ and $B \in \mathbb{R}^{M \times N}$. At the end of the sequence, solve $W^{out}(A + \lambda I)^{-1} = B$ for $W^{out}$. This requires only $O(N^2)$ memory (for $A$ and $B$), regardless of sequence length $T'$. The accumulation step costs $O(N^2)$ per time step rather than storing the full $N \times T'$ matrix.

Online collection is essential for very long sequences where $T' \gg N$, or for streaming applications where the full sequence is not available in advance.

## 5.5.8 Summary

The state collection procedure is:

1. Initialize $\mathbf{x}(0) = \mathbf{0}$ and run the reservoir dynamics for $t_{wash}$ steps, discarding the states.
2. For $t = t_{wash}+1, \ldots, T$: update the reservoir state, and store (or accumulate) $\mathbf{x}(t)$ or the extended state $\tilde{\mathbf{x}}(t)$.
3. Form the state matrix $X$ (or accumulate $A = XX^\top$ and $B = YX^\top$ online).
4. Optionally compress via PCA or random projection.

The result is the input to the offline linear solver (Section 5.6). No gradient computation, no backpropagation — just matrix multiplication.

---

## References

- **[Jaeger2001]** H. Jaeger. "The echo state approach to analysing and training recurrent neural networks." *GMD Report 148*, German National Research Center for Information Technology, 2001.
- **[Lukosevicius2012]** M. Lukosevicius. "A practical guide to applying echo state networks." In *Neural Networks: Tricks of the Trade*, Springer, pp. 659-686, 2012.
