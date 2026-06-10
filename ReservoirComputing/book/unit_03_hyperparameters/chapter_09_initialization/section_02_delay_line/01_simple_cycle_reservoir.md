# Section 9.2: The Simple Cycle Reservoir

## 9.2.1 The Architecture

The Simple Cycle Reservoir (SCR), introduced by Rodan and Tino [RodanTino2011], is the most structurally constrained reservoir imaginable. It consists of $N$ neurons connected in a ring:

$$r_i(t) = \tanh\!\bigl(\rho \cdot r_{i-1}(t-1) + w^{in}_i u_t\bigr), \quad i = 1, \ldots, N,$$

with the cyclic identification $r_0 \equiv r_N$. The weight matrix $W$ has exactly $N$ nonzero entries, all equal to $\rho$, arranged as:

$$W = \rho \begin{pmatrix} 0 & 0 & \cdots & 0 & 1 \\ 1 & 0 & \cdots & 0 & 0 \\ 0 & 1 & \cdots & 0 & 0 \\ \vdots & & \ddots & & \vdots \\ 0 & 0 & \cdots & 1 & 0 \end{pmatrix},$$

i.e., $W_{i,i-1} = \rho$ for $i = 2, \ldots, N$ and $W_{1,N} = \rho$. This is the permutation matrix for a cyclic shift, scaled by $\rho$.

The spectral radius of $W$ is $\rho$: all $N$ eigenvalues of the cyclic permutation matrix are the $N$-th roots of unity $\omega^k = e^{2\pi i k/N}$ for $k = 0, 1, \ldots, N-1$, each scaled by $\rho$, giving $|\lambda_k| = \rho$ for all $k$. This is a very uniform spectrum — all modes have exactly the same magnitude.

For the input weights, Rodan and Tino found that a particularly effective choice is:

$$w^{in}_i = \begin{cases} +\sigma_{in} & \text{if } i = 1 \\ 0 & \text{otherwise} \end{cases},$$

i.e., the input feeds into only one neuron. Other choices (e.g., alternating $\pm\sigma_{in}$, or all positive) are also studied in [RodanTino2011], with modestly different performance.

## 9.2.2 Memory Capacity Analysis

The surprising result about the SCR is that, for the linear version (no tanh), the exact memory capacity can be computed analytically and equals exactly $N$ for a suitable choice of $\rho < 1$.

**Linear SCR.** The linear SCR with single input is

$$r_i(t) = \rho \cdot r_{i-1}(t-1) + \sigma_{in} \cdot \delta_{i,1} \cdot u_t.$$

The state at time $t$ is, by iterating the recursion, a linear functional of the input history. Let us compute it.

Neuron 1 receives the input:

$$r_1(t) = \rho r_N(t-1) + \sigma_{in} u_t.$$

Neurons $2, \ldots, N$ pass their predecessor's state:

$$r_i(t) = \rho r_{i-1}(t-1).$$

Iterating: $r_2(t) = \rho r_1(t-1)$, $r_3(t) = \rho r_2(t-1) = \rho^2 r_1(t-2)$, and so on. In general:

$$r_i(t) = \rho^{i-1} r_1(t-i+1).$$

So the state of neuron $i$ at time $t$ is determined by the state of neuron 1 at time $t-i+1$. This is already revealing: the reservoir acts as a shift register, with neuron $i$ holding the state of neuron 1 from $i-1$ steps ago.

Now, neuron 1's state satisfies:

$$r_1(t) = \rho r_N(t-1) + \sigma_{in} u_t = \rho \cdot \rho^{N-1} r_1(t-N) + \sigma_{in} u_t = \rho^N r_1(t-N) + \sigma_{in} u_t.$$

This is a scalar AR($N$) recursion with a single lag $N$:

$$r_1(t) = \rho^N r_1(t-N) + \sigma_{in} u_t.$$

The steady-state solution (for $\rho < 1$) is

$$r_1(t) = \sigma_{in} \sum_{k=0}^\infty \rho^{kN} u_{t-kN}.$$

Substituting back:

$$r_i(t) = \rho^{i-1} r_1(t-i+1) = \sigma_{in} \rho^{i-1} \sum_{k=0}^\infty \rho^{kN} u_{t-(i-1)-kN} = \sigma_{in} \sum_{k=0}^\infty \rho^{kN+i-1} u_{t-kN-(i-1)}.$$

Let $j = kN + (i-1)$ range over the arithmetic progression $\{i-1, N+i-1, 2N+i-1, \ldots\}$:

$$r_i(t) = \sigma_{in} \sum_{j \in S_i} \rho^j u_{t-j}, \quad S_i = \{i-1, N+i-1, 2N+i-1, \ldots\}.$$

## 9.2.3 The Cross-Covariance Structure

The cross-covariance between $r_i(t)$ and $u_{t-k}$ (for i.i.d. unit-variance input) is

$$c_{ik} = \mathbb{E}[r_i(t) u_{t-k}] = \begin{cases} \sigma_{in} \rho^k & \text{if } k \in S_i, \text{ i.e., } k \equiv i-1 \pmod{N} \\ 0 & \text{otherwise.} \end{cases}$$

This is the key structural result. **Neuron $i$ remembers only the inputs at delays $k \in \{i-1, N+i-1, 2N+i-1, \ldots\}$.** Each neuron is responsible for a specific arithmetic progression of delays, and these progressions are *disjoint* across neurons (since each delay $k$ is congruent to exactly one residue class modulo $N$). The neurons partition the set of all integer delays into $N$ groups.

## 9.2.4 Exact Memory Capacity Formula

Given the disjoint cross-covariance structure, the memory capacities are additive across neurons:

$$MC = \sum_{i=1}^N MC^{(i)},$$

where $MC^{(i)}$ is the memory capacity contribution from neuron $i$.

For neuron $i$, the non-zero cross-covariances are at delays $k \equiv i-1 \pmod{N}$. The state variance is

$$\sigma_{r_i}^2 = \text{Var}[r_i(t)] = \sigma_{in}^2 \sum_{m=0}^\infty \rho^{2(mN+i-1)} = \sigma_{in}^2 \rho^{2(i-1)} \sum_{m=0}^\infty \rho^{2mN} = \frac{\sigma_{in}^2 \rho^{2(i-1)}}{1 - \rho^{2N}}.$$

The cross-covariance at delay $k = mN + (i-1)$ is $c_{i,k} = \sigma_{in} \rho^{mN + i - 1}$.

The $k$-step memory capacity from neuron $i$ (for $k = mN + i - 1$) is

$$MC_k^{(i)} = \frac{c_{i,k}^2}{\sigma_{r_i}^2} = \frac{\sigma_{in}^2 \rho^{2(mN+i-1)}}{\sigma_{in}^2 \rho^{2(i-1)} / (1-\rho^{2N})} = (1 - \rho^{2N}) \rho^{2mN}.$$

Summing the geometric series over $m \geq 0$:

$$MC^{(i)} = \sum_{m=0}^\infty (1-\rho^{2N})\rho^{2mN} = (1-\rho^{2N}) \cdot \frac{1}{1-\rho^{2N}} = 1.$$

**Each neuron has memory capacity exactly 1!** Summing over all $N$ neurons:

$$\boxed{MC_{SCR} = N.}$$

The SCR achieves the theoretical maximum memory capacity for any $\rho < 1$.

**Interpretation:** The SCR is a delay line that partitions the full integer delay axis into $N$ interleaved arithmetic progressions, one per neuron. Each neuron achieves its maximum possible memory capacity (1) for its specific set of delays. The total is exactly $N$ — no capacity is wasted.

## 9.2.5 Why Uniform Eigenspectrum Matters

The reason the SCR achieves $MC = N$ is precisely that all its eigenvalues have the same magnitude $\rho$. Recall from Section 7.2.4 that $MC = N$ for a linear reservoir requires $R_{\mathbf{rr}} = QQ^\top$ (equality in the capacity bound proof), which requires that all capacity is linear. The SCR is linear (by construction in the analysis above), so this holds.

But more: the SCR achieves $MC = N$ because the neuron responses $\{r_1, \ldots, r_N\}$ are *orthogonal* in their memory content. Specifically, the cross-covariance matrix between neurons is diagonal: $\mathbb{E}[r_i(t) r_j(t)] = 0$ for $i \neq j$ (when the input is i.i.d.), because neurons $i$ and $j$ remember disjoint sets of delays. This is the orthogonality condition from Theorem 7.1.1.

For a random reservoir with non-uniform eigenspectrum $\{|\lambda_i|\}$, some eigenvalues are larger and some smaller than the mean. The modes with small eigenvalues contribute little to memory capacity, wasting the corresponding neurons' degrees of freedom. The SCR eliminates this waste by construction.

## 9.2.6 Performance on Benchmark Tasks

Despite its extreme simplicity, the SCR performs surprisingly competitively with random ESNs on many benchmark tasks. [RodanTino2011] showed that:

- **On memory-dominated tasks** (e.g., predicting delayed versions of the input): SCR outperforms ESN significantly, because $MC_{SCR} = N > MC_{ESN}$ for typical hyperparameter settings.

- **On nonlinear tasks** (e.g., NARMA-10): SCR and ESN perform comparably, with ESN slightly better in some settings.

- **On tasks requiring long-range nonlinear dependencies** (e.g., tasks from [Jaeger2002memory]): ESN has an advantage because its random connectivity creates richer nonlinear interactions, while the SCR's strict locality limits nonlinear mixing.

This trade-off is exactly what the capacity framework predicts. The SCR allocates all capacity to linear memory, achieving $MC = N$ but zero nonlinear capacity. A random ESN splits capacity between linear memory and nonlinear components. The optimal split depends on the task.

## 9.2.7 Extensions and Variations

**Jumpy Delay Lines.** Rodan and Tino also proposed "jumpy" variants of the SCR where connections skip multiple neurons ($W_{i, i-j} = \rho$ for some $j > 1$). This creates memories at a different set of delays. By combining multiple jumpy delay lines, one can create a reservoir with a customized memory profile.

**Bidirectional Connections.** Adding $W_{i-1,i} = \rho'$ (connections in both directions around the ring) introduces eigenvalues of magnitude different from $\rho$, adding a second memory timescale.

**Partially Random.** A hybrid approach: start with the SCR and add random connections with small weight (perturbation of the ring). This retains most of the SCR's memory capacity while introducing some nonlinear mixing. The analysis in [RodanTino2011] shows that small random perturbations do not significantly degrade memory capacity for reasonable perturbation magnitudes.

---

*The SCR shows that extreme structural simplicity can be computationally powerful, if the structure is designed with the task's requirements in mind. The next section introduces a different approach to structure: adapting the nonlinearity itself to maximize information transmission.*
