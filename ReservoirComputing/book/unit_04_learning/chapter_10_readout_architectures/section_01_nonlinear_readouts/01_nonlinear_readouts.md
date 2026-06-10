# Beyond Linear Readouts: Nonlinear Output Architectures

## When Does Linearity Fail?

The standard reservoir computing pipeline rests on a deliberate architectural decision: the reservoir performs all nonlinear, high-dimensional computation, while the readout remains a simple affine map. This separation is intellectually clean and computationally attractive — it preserves convexity of the training problem and admits an analytic closed-form solution via ridge regression. For the vast majority of tasks in the literature, this suffices.

However, linearity of the readout is an assumption, not a theorem. When the reservoir state $\mathbf{x}_t \in \mathbb{R}^N$ does not contain all information needed to predict the output in a linearly accessible form, the linear readout constitutes a representational bottleneck. There are two structurally distinct reasons this can occur.

First, the reservoir may be too small or its dynamics too constrained to fully separate the relevant input histories in state space. If two distinct input trajectories $\mathbf{u}_{1,(-\infty,t]}$ and $\mathbf{u}_{2,(-\infty,t]}$ produce reservoir states $\mathbf{x}_1$ and $\mathbf{x}_2$ that are linearly inseparable with respect to the target, no linear readout can correct this. Second, the target function may require interactions between distinct reservoir dimensions that are not exposed linearly. In this case the reservoir states are individually informative, but their combinations are not linearly exploitable. Both pathologies motivate going beyond linear readouts [Schmidhuber et al. 2007].

## Quadratic Readout

The simplest nonlinear extension is the quadratic readout:

$$y = \mathbf{x}^\top \mathbf{A} \mathbf{x} + \mathbf{b}^\top \mathbf{x} + c,$$

where $\mathbf{A} \in \mathbb{R}^{N \times N}$, $\mathbf{b} \in \mathbb{R}^N$, and $c \in \mathbb{R}$. The term $\mathbf{x}^\top \mathbf{A} \mathbf{x}$ captures pairwise interactions among reservoir neurons. Interpretively, each entry $A_{ij}$ encodes the contribution of the product $x_i x_j$ to the output — a second-order temporal feature, since each $x_i$ is itself a filtered version of the entire input history.

The quadratic readout is equivalent to lifting the reservoir state to an augmented feature vector:

$$\tilde{\mathbf{x}} = \left[ x_1, x_2, \ldots, x_N, x_1^2, x_1 x_2, \ldots, x_N^2 \right]^\top \in \mathbb{R}^{N + N(N+1)/2},$$

and applying a linear readout to $\tilde{\mathbf{x}}$. This makes the training problem convex again — ridge regression on $\tilde{\mathbf{x}}$ is equivalent to solving the quadratic readout. The cost is the dimension explosion: for $N = 500$ neurons, the augmented feature vector has $500 + 125{,}250 = 125{,}750$ dimensions, requiring a much larger training set to avoid overfitting [Schrauwen et al. 2010].

## Kernel Readout

The kernel trick avoids the explicit feature expansion by replacing the inner product in the readout computation with a kernel evaluation:

$$y = \sum_{i=1}^{T_{\text{train}}} \alpha_i k(\mathbf{x}_i, \mathbf{x}),$$

where $k : \mathbb{R}^N \times \mathbb{R}^N \to \mathbb{R}$ is a positive-definite kernel and $\boldsymbol{\alpha} \in \mathbb{R}^{T_{\text{train}}}$ are dual weights learned by kernel ridge regression:

$$\boldsymbol{\alpha} = (\mathbf{K} + \lambda \mathbf{I})^{-1} \mathbf{y}^*,$$

with $K_{ij} = k(\mathbf{x}_i, \mathbf{x}_j)$ the Gram matrix. The radial basis function kernel $k(\mathbf{x}, \mathbf{x}') = \exp(-\|\mathbf{x} - \mathbf{x}'\|^2 / 2\sigma^2)$ gives an infinite-dimensional feature expansion, allowing the effective readout to capture arbitrary smooth functions of the reservoir state. The computational cost is $O(T_{\text{train}}^2 N)$ for Gram matrix construction and $O(T_{\text{train}}^3)$ for the inversion — expensive for large datasets, which is the primary practical limitation [Schmidhuber et al. 2007].

## Two-Layer Readout

A small multi-layer perceptron (MLP) on top of the reservoir states provides a more flexible nonlinear readout while keeping the number of trainable parameters manageable:

$$\mathbf{h} = \sigma(\mathbf{W}_1 \mathbf{x} + \mathbf{b}_1), \qquad y = \mathbf{w}_2^\top \mathbf{h} + b_2,$$

where $\mathbf{W}_1 \in \mathbb{R}^{M \times N}$ is the hidden weight matrix, $M \ll N$ is the hidden dimension, and $\sigma$ is a pointwise activation. With $M = 50$ and $N = 500$, this adds only $50 \times 500 = 25{,}000$ parameters. Training requires backpropagation through the MLP — but crucially, not through the reservoir — so the reservoir states simply serve as a fixed feature matrix. This preserves the spirit of reservoir computing while recovering some expressiveness [Schrauwen et al. 2010].

## When Nonlinear Readouts Help and When They Hurt

Nonlinear readouts are most beneficial when (1) the target function involves higher-order interactions among temporal features that the reservoir does not expose linearly, (2) the reservoir is small relative to the task complexity, or (3) the task requires decision boundaries that are genuinely curved in reservoir state space (e.g., XOR-like separation).

They are harmful in three common scenarios. First, they introduce additional hyperparameters (kernel bandwidth, MLP architecture) whose tuning cost may outweigh the gain. Second, they destroy the convexity guarantee: the MLP readout training problem is non-convex, and the quadratic feature expansion may be severely ill-conditioned with insufficient data. Third — and most importantly from a theoretical standpoint — they represent a misallocation of complexity. The canonical reservoir computing philosophy holds that the reservoir should perform the nonlinear lifting of input history into a high-dimensional linear feature space; a linear readout then extracts the relevant combination [Schmidhuber et al. 2007]. If the readout must be nonlinear, the reservoir has failed to do its job.

## The Core Tension

There is a fundamental design tension that every practitioner must confront. A sufficiently large, well-tuned reservoir with spectral radius near unity and appropriate input scaling should, in principle, linearly separate any computable function of the input history (within the bounds of the Boyd–Chua theorem). If a linear readout fails on a given task, the correct response is usually to enlarge or improve the reservoir — not to complicate the readout.

Nonlinear readouts are therefore best understood as a diagnostic tool and an occasional engineering necessity, not as a general solution. When the training budget is small or the task is genuinely low-dimensional, a kernel or quadratic readout may be justified. In large-scale applications, the computational and overfitting costs typically tip the balance back toward a well-engineered linear readout on a sufficiently rich reservoir.

---

## References

- Schmidhuber, J., Wierstra, D., Gagliolo, M., & Gomez, F. (2007). Training recurrent networks by evolvo and gradient-based algorithms. *IEEE Transactions on Neural Networks*, 18(3), 633–640.
- Schrauwen, B., Verstraeten, D., & Van Campenhout, J. (2010). An overview of reservoir computing: theory, applications and implementations. *Proceedings of the European Symposium on Artificial Neural Networks*, 471–482.
