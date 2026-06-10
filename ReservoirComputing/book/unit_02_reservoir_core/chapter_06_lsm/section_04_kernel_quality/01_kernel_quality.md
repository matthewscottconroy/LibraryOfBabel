# Section 6.4: Kernel Quality of the Liquid State Machine

## 6.4.1 The Liquid as a Kernel

The Liquid State Machine (LSM) operates in two stages: a fixed recurrent network (the "liquid") maps input streams to a high-dimensional state trajectory, and a memoryless readout maps instantaneous states to outputs. This two-stage architecture is formally equivalent to a kernel method. The kernel is defined implicitly by the liquid's dynamics: two input histories $\mathbf{u}_1(\cdot)$ and $\mathbf{u}_2(\cdot)$ are compared not directly but through the states they produce.

**Definition 6.4.1 (Liquid State Kernel).** Let $\mathbf{x}(t; \mathbf{u})$ denote the liquid state at time $t$ driven by input history $\mathbf{u}(\cdot)$. The *liquid kernel* is

$$k(\mathbf{u}_1, \mathbf{u}_2) = \langle \mathbf{x}(t; \mathbf{u}_1),\ \mathbf{x}(t; \mathbf{u}_2) \rangle = \mathbf{x}(t; \mathbf{u}_1)^\top \mathbf{x}(t; \mathbf{u}_2),$$

where $\langle \cdot, \cdot \rangle$ is the Euclidean inner product in $\mathbb{R}^N$.

Equivalently, the kernel gram matrix $K_{ij} = k(\mathbf{u}_i, \mathbf{u}_j) = \mathbf{x}(t; \mathbf{u}_i)^\top \mathbf{x}(t; \mathbf{u}_j)$ encodes pairwise similarities between input histories as seen by the liquid. Any linear readout $y(t) = \mathbf{w}^\top \mathbf{x}(t; \mathbf{u})$ computes a function that lies in the RKHS induced by this kernel.

This kernel perspective clarifies the power and limits of LSMs: the set of functions that a linear readout can compute is precisely the set of functions in the RKHS of $k(\cdot, \cdot)$. If the kernel is rich — if it can distinguish all relevant pairs of inputs — the readout can compute any target function in that class. If the kernel is degenerate — if different inputs produce similar states — the readout is blind to those distinctions.

## 6.4.2 The Separation Property

Maass, Natschläger, and Markram [Maass2002] formalized two conditions that determine whether an LSM can compute useful functions.

**Definition 6.4.2 (Separation Property).** A liquid $L$ has the *separation property* (SP) with respect to an input class $\mathcal{U}$ if for any two distinct input streams $\mathbf{u}_1, \mathbf{u}_2 \in \mathcal{U}$ with $\mathbf{u}_1 \neq \mathbf{u}_2$, the resulting liquid states differ:

$$\mathbf{x}(t; \mathbf{u}_1) \neq \mathbf{x}(t; \mathbf{u}_2).$$

The SP is a necessary condition for the readout to distinguish the two inputs. If two distinct input histories produce identical liquid states, no linear (or nonlinear) readout can tell them apart.

For a reservoir with the echo state property (Section 5.2), the SP holds automatically: distinct input histories produce distinct states whenever the inputs differ during the memory window. The SP can fail if two input streams converge to the same asymptotic attractor — a pathology that arises when the reservoir is too strongly driven or too strongly contracted.

## 6.4.3 The Approximation Property

**Definition 6.4.3 (Approximation Property).** A readout function class $\mathcal{F}$ has the *approximation property* (AP) with respect to a target class $\mathcal{G}$ if for every target function $g \in \mathcal{G}$ and every $\varepsilon > 0$, there exists $f \in \mathcal{F}$ such that $\|f - g\|_\infty < \varepsilon$ on the relevant input domain.

For LSMs, the readout class is the set of linear functions of the liquid state: $\mathcal{F} = \{f(\mathbf{u}) = \mathbf{w}^\top \mathbf{x}(t; \mathbf{u}) : \mathbf{w} \in \mathbb{R}^N\}$. The AP holds for a broad target class $\mathcal{G}$ whenever the liquid states span a rich enough subspace [MaassMarkramMatthew2002]. The Stone-Weierstrass theorem (Section 26.1) guarantees the AP for continuous target functionals when the liquid states form a separating, point-distinguishing algebra — which holds under the fading memory condition.

**The SP + AP theorem.** Maass et al. proved that an LSM with the SP and a readout with the AP is a universal approximator for continuous functionals on input streams: for any causal, time-invariant, fading-memory functional $F$ and any $\varepsilon > 0$, there exist a liquid with SP and a readout with AP such that the LSM approximates $F$ to within $\varepsilon$ [Maass2002].

## 6.4.4 Kernel Quality: Formal Definition

The separation property is qualitative — states either differ or they do not. For quantitative assessment, Maass et al. introduced the *kernel quality* measure, which captures how well the liquid separates a given set of input streams.

**Definition 6.4.4 (Kernel Quality).** Let $\{\mathbf{u}_1, \ldots, \mathbf{u}_M\}$ be a set of input streams and $\mathbf{x}_m = \mathbf{x}(t; \mathbf{u}_m)$ the corresponding liquid states. The *kernel quality* $Q_K$ is defined as the rank of the kernel gram matrix:

$$Q_K = \text{rank}(K), \quad K_{mn} = \mathbf{x}_m^\top \mathbf{x}_n.$$

Equivalently, $Q_K$ equals the number of linearly independent directions in the liquid state space spanned by the $M$ state vectors. When $Q_K = M$, all $M$ input streams produce linearly independent states — the best possible separation. When $Q_K < M$, some pairs of inputs produce states that are linearly dependent, and target functions sensitive to those distinctions cannot be learned.

In practice, due to noise and numerical precision, $Q_K$ is estimated as the effective rank: the number of eigenvalues of $K$ above a threshold $\varepsilon_0$:

$$Q_K^{(\varepsilon_0)} = |\{i : \lambda_i(K) > \varepsilon_0 \cdot \lambda_1(K)\}|,$$

where $\lambda_1(K)$ is the largest eigenvalue of $K$. A typical threshold is $\varepsilon_0 = 10^{-3}$.

## 6.4.5 Kernel Rank and Computable Functions

The kernel rank $Q_K$ directly determines the class of functions that the readout can compute:

**Proposition 6.4.1.** *The space of linear functionals of the liquid state has dimension at most $Q_K$. Therefore, the linear readout can compute at most $Q_K$ linearly independent target functions of the given input streams.*

This follows immediately from the definition: the row space of the state matrix $[\mathbf{x}_1, \ldots, \mathbf{x}_M]$ has dimension $Q_K$, and any linear readout is a linear combination of the rows of this matrix.

The practical implication is that $Q_K / M$ measures the fraction of information about the input class that the liquid preserves. A liquid with $Q_K / M \approx 1$ is effectively injective: it maps different inputs to different (and linearly independent) states. A liquid with $Q_K / M \approx 0$ compresses all inputs into a low-dimensional manifold, losing most of the input information.

**Tino and Hammer's analysis** [TinoHammer2003] extended this picture by relating $Q_K$ to the geometry of the liquid's state-space embedding. They showed that for random Gaussian liquids, $Q_K$ scales with the number of active degrees of freedom in the reservoir: $Q_K \approx \min(M, N_{eff})$, where $N_{eff}$ is the effective dimensionality of the state distribution (the participation ratio $N_{eff} = (\sum_i \lambda_i)^2 / \sum_i \lambda_i^2$ of the reservoir state covariance).

## 6.4.6 Geometric Interpretation

The kernel quality has a clean geometric interpretation. The $M$ liquid states $\mathbf{x}_1, \ldots, \mathbf{x}_M \in \mathbb{R}^N$ form a point cloud in the liquid's state space. The kernel gram matrix $K$ describes the geometry of this cloud (inner products between all pairs of points, after centering). The rank of $K$ is the dimension of the affine subspace spanned by the cloud.

A high-quality liquid (large $Q_K$) maps input streams to points that are spread across a high-dimensional subspace of $\mathbb{R}^N$: the point cloud has full effective rank. A low-quality liquid maps inputs to a nearly one-dimensional subspace (a line or plane in $\mathbb{R}^N$), collapsing almost all the input information.

The ideal liquid maximizes $Q_K = \min(M, N)$ for any input class of size $M$: it maps $M$ input streams to $M$ points in general position in $\mathbb{R}^N$. This is possible only when $M \leq N$ (the liquid has enough dimensions to accommodate all input streams) and when the dynamics are not too contractive (the echo state property holds but not too strongly, so distinct inputs remain distinct).

**Edge of chaos.** The edge of chaos (Section 6.5) is precisely the dynamical regime where $Q_K$ is maximized: the liquid is neither too contractive (which collapses inputs together) nor too chaotic (which amplifies noise, distorting the kernel). This provides a dynamical interpretation of why edge-of-chaos operation is optimal: it maximizes kernel quality, and hence the class of computable functions.

---

## References

- **[Maass2002]** W. Maass, T. Natschläger, and H. Markram. "Real-time computing without stable states: A new framework for neural computation based on perturbations." *Neural Computation*, 14(11):2531-2560, 2002.
- **[TinoHammer2003]** P. Tino and B. Hammer. "Architectural bias in recurrent neural networks: Fractal analysis." *Neural Computation*, 15(8):1931-1957, 2003.
