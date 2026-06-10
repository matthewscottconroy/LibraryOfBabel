# Section 8.5: Reservoir Size

## 8.5.1 More Neurons, More Basis Functions

The reservoir's readout is a linear function of the $N$-dimensional state vector $\mathbf{x}(t)$. The $N$ neurons provide $N$ basis functions for approximating any target function of the input history. Increasing $N$ adds more basis functions and, therefore, more expressive power. This is the same argument that drives the utility of wide neural networks: more neurons span a richer function space, allowing the readout to approximate a wider class of targets.

More precisely, the readout computes functions in the RKHS of the liquid kernel $k(\mathbf{u}_1, \mathbf{u}_2) = \mathbf{x}(t;\mathbf{u}_1)^\top \mathbf{x}(t;\mathbf{u}_2)$ (Section 6.4). The dimension of this RKHS is bounded by $N$: no more than $N$ linearly independent functions can be expressed by a linear readout of an $N$-dimensional state. Adding neurons enlarges the RKHS and thus the set of expressible functions.

## 8.5.2 The IPC Bound and Memory Scaling

The information processing capacity (IPC) theorem (Section 7.3) states that for any reservoir of $N$ neurons, the total capacity satisfies $C_{total} \leq N$. This bound is tight in the linear case. Therefore:

- **Linear memory capacity** $MC = C_1$ is at most $N$.
- For a well-designed linear reservoir (e.g., SCR, Section 9.2), $MC = N$ exactly.
- For a random nonlinear reservoir near the edge of chaos, $MC \approx \alpha N$ for some $\alpha \in (0, 1)$ depending on $\rho$ and $\sigma_{in}$.

Doubling $N$ approximately doubles the total capacity, which for memory-dominated tasks translates directly into the ability to recall inputs from twice as far in the past. For nonlinear tasks, doubling $N$ provides twice as many nonlinear interaction terms, allowing the readout to learn more complex functions.

This scaling is the theoretical justification for using large reservoirs on demanding tasks.

## 8.5.3 Diminishing Returns in Practice

Despite the linear theoretical scaling, practical performance often shows strong diminishing returns. The pattern is consistent across task types [Jaeger2002]:

- Small reservoirs ($N < 50$): Performance improves rapidly with $N$. Each additional neuron provides a genuinely new basis function that the readout can exploit.
- Medium reservoirs ($N = 50$-$500$): Performance continues to improve but more slowly. Many new neurons provide similar basis functions to existing ones, adding redundancy rather than new capacity.
- Large reservoirs ($N > 1000$): Performance often plateaus. The task is solved well by $N = 100$-$500$ neurons, and adding more provides diminishing marginal benefit while the computational cost grows substantially.

The reason for diminishing returns is that most practical tasks have low *effective complexity*: the target function can be approximated well by a relatively small number of basis functions. Once $N$ exceeds the effective complexity of the task, additional neurons are redundant.

A rough practical rule [Lukosevicius2012]: start with $N = 100$ neurons. If the validation error has not plateaued (i.e., reducing $N$ to 50 significantly increases the error), double to $N = 200$ and repeat.

## 8.5.4 Computational Cost

The cost of reservoir computing scales with $N$ in two places:

**State update:** Each time step requires computing $W^{rec}\mathbf{x}(t-1)$, which costs $O(pN^2)$ for a reservoir with connectivity fraction $p$. For dense reservoirs ($p = 1$), this is $O(N^2)$. For sparse reservoirs ($p \sim 10/N$, i.e., 10 connections per neuron), this is $O(N)$.

**Readout training:** The ridge regression solution $W^{out} = YX^\top(XX^\top + \lambda I)^{-1}$ requires inverting the $N \times N$ matrix $XX^\top$, at cost $O(N^3)$ (or $O(N^2 T)$ for the $T \times T$ dual formulation when $T < N$). This is the dominant cost for large reservoirs.

The total training cost is therefore

$$\text{Cost} = O(pN^2 T + N^3),$$

where $T$ is the training sequence length. For $T \gg N$ (the common case), the $N^3$ term dominates. Doubling $N$ increases the training cost by a factor of 8.

**Implication.** Going from $N = 100$ to $N = 1000$ increases cost by a factor of $1000$ (for the $N^3$ term), while performance improvement is often less than 10%. The cost-performance frontier strongly favors moderate $N$ for most tasks.

## 8.5.5 Sparse Reservoirs and Efficient Computation

Sparse connectivity ($p \ll 1$) reduces the per-step cost from $O(N^2)$ to $O(pN^2) = O(N)$ for fixed $p$, but does not change the $O(N^3)$ readout cost. The readout cost can be reduced by:

**Dual formulation.** When $T < N$, use the dual ridge regression solution $W^{out} = YK^{-1}X$, where $K = X^\top X \in \mathbb{R}^{T \times T}$ and the inversion costs $O(T^3)$ instead of $O(N^3)$.

**Incremental/online readout.** Update the correlation matrices $A = XX^\top$ and $B = YX^\top$ online (Section 5.5), then solve the $N \times N$ linear system at the end. Cost: $O(N^2 T)$ for accumulation, $O(N^3)$ for the final solve.

**Krylov/conjugate gradient.** For very large $N$ with sparse $XX^\top$, use iterative linear solvers (conjugate gradient) to avoid the full $O(N^3)$ factorization. Convergence in $O(N)$ iterations with $O(N^2)$ cost per iteration gives $O(N^3)$ worst case but much less in practice for well-conditioned systems.

## 8.5.6 When to Use Large Reservoirs

Large reservoirs ($N > 1000$) are justified in specific scenarios:

**High-dimensional inputs.** When the input dimension $K$ is large (e.g., image or video streams, multivariate sensor arrays), the reservoir needs $N \gg K$ neurons to form useful nonlinear interactions between input channels.

**Complex temporal dependencies.** Tasks requiring memory at many different timescales simultaneously — such as predicting chaotic systems with a rich multiscale attractor — benefit from large $N$ to support the range of memory timescales.

**Large training data.** The total capacity $C_{total} \leq N$ bounds performance; if the training sequence is very long ($T \gg N^2$), the readout can exploit all $N$ basis functions reliably, and the statistical benefit of large $N$ is fully realized.

For most standard benchmarks (NARMA, Mackey-Glass, speech tasks), $N = 100$-$500$ achieves near-optimal performance. The practitioner should resist the temptation to use $N = 10{,}000$ before first establishing that $N = 500$ is insufficient.

---

## References

- **[Dambre2012]** J. Dambre, D. Verstraeten, B. Schrauwen, and S. Massar. "Information processing capacity of dynamical systems." *Scientific Reports*, 2:514, 2012.
- **[Jaeger2002]** H. Jaeger. "Short term memory in echo state networks." *GMD Report 152*, German National Research Center for Information Technology, 2002.
- **[Lukosevicius2012]** M. Lukosevicius. "A practical guide to applying echo state networks." In *Neural Networks: Tricks of the Trade*, Springer, pp. 659-686, 2012.
