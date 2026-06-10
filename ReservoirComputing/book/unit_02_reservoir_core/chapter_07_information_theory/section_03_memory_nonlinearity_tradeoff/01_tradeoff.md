# Section 7.3: The Memory-Nonlinearity Tradeoff

## 7.3.1 The Central Question

A reservoir that perfectly remembers its entire input history would have infinite linear memory capacity. A reservoir that is maximally nonlinear would mix and transform inputs in complex ways, enabling rich nonlinear computation. But these two desiderata pull against each other: strong nonlinearity distorts the input representation, destroying linear memory, while strong linear memory requires the network to behave nearly linearly, limiting nonlinear computation. This tension is not a design failure — it is a fundamental mathematical constraint.

Dambre et al. [Dambre2012] made this precise: for any finite reservoir of $N$ neurons, the total information processing capacity is bounded above by $N$, and this capacity is divided between linear and nonlinear components according to the reservoir's hyperparameters. Understanding this tradeoff is essential for matching a reservoir to a given task's requirements.

## 7.3.2 The Capacity Decomposition

Let $\mathbf{u}(t)$ be a scalar i.i.d. input with zero mean and unit variance. The reservoir state $\mathbf{x}(t) \in \mathbb{R}^N$ is a functional of the input history $\mathbf{u}_{1:t}$.

**Definition 7.3.1 (Degree-$k$ Target Functions).** A *degree-$k$ polynomial* target is any function of the form

$$y_k(t) = \sum_{j_1, \ldots, j_k} c_{j_1, \ldots, j_k} \prod_{l=1}^{k} u(t - j_l),$$

where $j_1, \ldots, j_k \geq 0$ are time delays. Degree-1 targets are linear functionals of the input history (purely temporal filtering). Degree-$k$ targets for $k \geq 2$ involve cross-products of inputs at different times.

**Definition 7.3.2 (Degree-$k$ Capacity).** The *degree-$k$ capacity* $C_k$ of a reservoir is the sum over all orthonormal basis functions $P_{\mathbf{j}}^{(k)}(\mathbf{u}_{1:t})$ of degree $k$ of the squared correlation between the reservoir state and the basis function, maximized over linear readout:

$$C_k = \sum_{\mathbf{j}} \max_{\mathbf{w}} \frac{[\text{Cov}(\mathbf{w}^\top \mathbf{x}(t),\ P_{\mathbf{j}}^{(k)}(\mathbf{u}_{1:t}))]^2}{\text{Var}(\mathbf{w}^\top \mathbf{x}(t)) \cdot \text{Var}(P_{\mathbf{j}}^{(k)})}.$$

Here $\{P_{\mathbf{j}}^{(k)}\}$ are the degree-$k$ Legendre polynomial basis functions of the input history (the orthonormal basis for degree-$k$ targets under the i.i.d. input distribution), and the sum runs over all multi-index delay vectors $\mathbf{j} = (j_1, \ldots, j_k)$ with $j_1 \leq j_2 \leq \cdots \leq j_k$.

The *linear memory capacity* (MC, Section 7.2) is $C_1$. The total capacity is

$$C_{total} = \sum_{k=1}^{\infty} C_k.$$

**Theorem 7.3.1 (Dambre et al. 2012).** *For any reservoir of $N$ neurons operating in the echo state regime:*
$$C_{total} = \sum_{k=1}^{\infty} C_k \leq N.$$

*Moreover, if the reservoir states are linearly independent (which holds generically), the bound is tight: $C_{total} = N$.*

The proof uses the fact that the reservoir state $\mathbf{x}(t) \in \mathbb{R}^N$ spans at most an $N$-dimensional space. Any linear readout $\mathbf{w}^\top \mathbf{x}(t)$ can be decomposed in the orthonormal basis of polynomial target functions. The sum of squared correlations across all target functions equals 1 for each readout direction (by Parseval's theorem), and there are at most $N$ independent readout directions. The total capacity is thus at most $N$, with equality when the reservoir states span exactly $N$ independent dimensions in the polynomial function space.

## 7.3.3 How Spectral Radius Controls the Split

The decomposition $C_{total} = \sum_k C_k$ is conserved — increasing $N$ increases total capacity, but hyperparameters merely redistribute capacity between degrees.

**Effect of spectral radius $\rho$.** As $\rho$ increases toward 1:
- The reservoir retains information from the distant past for longer. Linear targets at large delays $k$ (e.g., $y(t) = u(t - 50)$) become recoverable, increasing $C_1 = MC$.
- However, long-range linear memory is achieved at the cost of suppressing nonlinear interactions: for the reservoir to remember $u(t-50)$ linearly, it must be operating approximately linearly (small effective gain), which reduces the amplitude of degree-2 and higher terms.

This can be seen from the Volterra series expansion of the reservoir (Section 26.2): the $k$-th order Volterra kernel of the reservoir decays as $\rho^{|j_1| + \cdots + |j_k|}$, so high-order kernels (large $k$) decay faster for fixed delay lengths. Increasing $\rho$ extends the reach of all kernels, but benefits the first-order (linear) kernel most.

Empirically, $C_1$ increases roughly linearly with $\rho$ for $\rho \in (0, 1)$, while $C_2$ peaks around $\rho \approx 0.5$ and declines for $\rho$ near 1 [Dambre2012].

**Effect of input scaling $\sigma_{in}$.** As $\sigma_{in}$ increases:
- Neurons are driven further into the nonlinear regime ($\tanh$ saturation), increasing the amplitude of nonlinear mixing.
- Nonlinear capacity $\sum_{k \geq 2} C_k$ increases.
- But large inputs also compress the representation (saturated neurons carry little information), eventually reducing total capacity.
- Linear memory $C_1$ decreases with $\sigma_{in}$, because large inputs override the recurrent memory.

The input scaling thus controls a different axis of the tradeoff: not the temporal depth of memory but the degree of nonlinear mixing at each time step.

## 7.3.4 The Pareto Frontier

Plotting $C_1$ against $C_2$ (or against $\sum_{k \geq 2} C_k$) as $\rho$ and $\sigma_{in}$ vary traces a *Pareto frontier* of achievable (memory, nonlinearity) pairs [Legenstein2007]. The frontier is bounded by the total capacity $N$: no reservoir can simultaneously have $C_1 = N$ and $C_2 > 0$ (maximum linear memory forces zero nonlinear capacity, achieved only by the linear SCR), and no reservoir can have $C_2 = N$ and $C_1 > 0$ (maximum nonlinear capacity forces zero linear memory, an extreme limit that is not practically achievable with a single reservoir).

The Pareto frontier is approximately the simplex $\{(C_1, C_2) : C_1 + C_2 \leq N, C_1 \geq 0, C_2 \geq 0\}$ for the two-dimensional projection. In full generality, the frontier in the infinite-dimensional capacity space is the simplex $\sum_k C_k = N$.

## 7.3.5 Task-Dependent Optimization

The practical implication is that hyperparameter optimization should match the reservoir's capacity distribution to the task's requirements:

**Purely linear tasks** (e.g., linear time-series prediction, delay lines): maximize $C_1$ by setting $\rho$ close to 1 and $\sigma_{in}$ small. Consider using an SCR (Section 9.2) to achieve $C_1 = N$.

**Nonlinear tasks with short memory** (e.g., XOR of recent inputs, NARMA-2): maximize $C_2$ for short delays by setting $\rho$ small (around 0.5) and $\sigma_{in}$ large. The reservoir does not need long memory, but needs strong nonlinear mixing.

**Nonlinear tasks with long memory** (e.g., NARMA-10, chaotic time-series prediction): balance $C_1$ and $C_2$ at intermediate $\rho \approx 0.8$-$0.95$ and moderate $\sigma_{in}$. Legenstein and Maass [Legenstein2007] showed that the optimal point for such tasks lies near the edge of chaos.

---

## References

- **[Dambre2012]** J. Dambre, D. Verstraeten, B. Schrauwen, and S. Massar. "Information processing capacity of dynamical systems." *Scientific Reports*, 2:514, 2012.
- **[Legenstein2007]** R. Legenstein and W. Maass. "Edge of chaos and prediction of computational performance for neural circuit models." *Neural Networks*, 20(3):323-334, 2007.
