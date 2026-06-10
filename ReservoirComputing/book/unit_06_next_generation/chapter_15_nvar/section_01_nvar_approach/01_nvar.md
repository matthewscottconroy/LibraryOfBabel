# The NVAR Approach: Nonlinear Vector Autoregression

## Core Idea: Replacing the Reservoir with Explicit Features

The echo state network uses a random recurrent network to map the input history $\mathbf{u}_{(-\infty, t]}$ into a high-dimensional feature vector $\mathbf{x}_t \in \mathbb{R}^N$. The reservoir provides these features implicitly: its dynamics, driven by the input, produce states that are nonlinear functionals of the input history. The specific features are determined by the random initialization and are opaque — they are not interpretable or analytically derivable.

Nonlinear Vector Autoregression (NVAR) takes a radically different approach: construct the feature vector explicitly from delayed copies and polynomial products of the input. No random matrices, no hidden dynamics, no black box. The feature vector is fully deterministic, interpretable, and computed by direct arithmetic on the observed input [Gauthier et al. 2021].

## Feature Vector Construction

Let $\mathbf{u}_t \in \mathbb{R}^d$ be the $d$-dimensional input at time $t$, and let $k \geq 1$ be the delay spacing. The NVAR feature vector is assembled from:

**Linear features (lagged inputs):**

$$\mathbf{o}_t^{\text{lin}} = [\mathbf{u}_t^\top, \mathbf{u}_{t-k}^\top, \mathbf{u}_{t-2k}^\top, \ldots, \mathbf{u}_{t-(s-1)k}^\top]^\top \in \mathbb{R}^{sd},$$

where $s$ is the number of delays.

**Quadratic features (products of pairs):**

$$\mathbf{o}_t^{\text{quad}} = [\mathbf{u}_t \otimes \mathbf{u}_t, \mathbf{u}_t \otimes \mathbf{u}_{t-k}, \ldots]^\top,$$

consisting of all pairwise products $(u_{t-ik}^{(a)})(u_{t-jk}^{(b)})$ for $0 \leq i \leq j \leq s-1$ and $1 \leq a \leq b \leq d$ (or all $a, b$ for the full outer product).

The complete feature vector is the concatenation:

$$\mathbf{o}_t = [\mathbf{o}_t^{\text{lin}}; \mathbf{o}_t^{\text{quad}}] \in \mathbb{R}^D,$$

where $D = sd + \binom{sd + 1}{2}$ for degree-2 polynomial expansion of the full lagged input vector [Gauthier et al. 2021].

## Linear Readout on Polynomial Features

The NVAR prediction is simply a linear combination of the feature vector:

$$\hat{\mathbf{y}}_t = \mathbf{W}^{\text{out}} \mathbf{o}_t,$$

where $\mathbf{W}^{\text{out}} \in \mathbb{R}^{d_{\text{out}} \times D}$ is trained by ridge regression:

$$\hat{\mathbf{W}}^{\text{out}} = \mathbf{Y}^{*\top} \mathbf{O}^\top (\mathbf{O} \mathbf{O}^\top + \lambda \mathbf{I})^{-1},$$

with $\mathbf{O} \in \mathbb{R}^{D \times T}$ the feature matrix. No reservoir update equation is needed — the feature computation is a deterministic function of the current and past inputs, and the model is a kernel machine with a polynomial kernel (see Section 15.5).

## Gauthier et al. 2021: NVAR vs. ESN on Lorenz Prediction

The key empirical paper is Gauthier et al. [2021], who tested NVAR on the Lorenz system prediction task — the canonical reservoir computing benchmark. Their NVAR setup used:

- $d = 3$ (three Lorenz variables: $x, y, z$)
- $s = 2$ delays with spacing $k = 1$ (using $\mathbf{u}_t$ and $\mathbf{u}_{t-1}$)
- Degree-2 polynomial expansion
- Total feature dimension: $D = 3 \times 2 + \binom{7}{2} = 6 + 21 = 27$

This tiny feature vector of dimension 27 achieved valid prediction times (time until normalized MSE exceeds a threshold) comparable to an ESN with $N = 500$ neurons — a 500-dimensional feature vector. The NVAR uses approximately $500/27 \approx 18\times$ fewer effective parameters [Gauthier et al. 2021].

## Implementation: Pure NumPy

A complete NVAR for Lorenz prediction requires no specialized libraries, no GPU, and no random matrix generation. The implementation is:

1. Collect observations $\{\mathbf{u}_t\}_{t=1}^T$ from the Lorenz system.
2. Construct the feature matrix $\mathbf{O}$ by computing delayed copies and their pairwise products.
3. Solve ridge regression: $\hat{\mathbf{W}} = (\mathbf{O}\mathbf{O}^\top + \lambda \mathbf{I})^{-1}\mathbf{O}\mathbf{Y}^{*\top}$.
4. Predict: $\hat{\mathbf{u}}_{t+1} = \hat{\mathbf{W}} \mathbf{o}_t$.

The entire procedure is deterministic, reproducible, and requires only basic linear algebra. This is in sharp contrast to ESNs, which depend on random initialization and whose performance varies across seeds [Gauthier et al. 2021].

## When NVAR is Appropriate

NVAR is most appropriate for low-dimensional dynamical systems where: (1) the input is observed in full (all state variables), (2) the relevant nonlinearities are polynomial, (3) interpretability of the feature construction is valuable, and (4) the computational budget favors exact computation over random approximation.

The approach faces difficulties with high-dimensional inputs, long-range temporal dependencies, and non-polynomial nonlinearities. These scenarios motivate the hybrid and extension approaches discussed in subsequent sections.

---

## References

- Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12(1), 5564.
