# Gauthier et al. 2021: Results in Detail

## The Lorenz Task

The Lorenz system is defined by the autonomous differential equations:

$$\dot{x} = \sigma(y - x), \quad \dot{y} = x(\rho - z) - y, \quad \dot{z} = xy - \beta z,$$

with standard parameters $\sigma = 10$, $\rho = 28$, $\beta = 8/3$. At these values the system exhibits a strange attractor with maximal Lyapunov exponent $\lambda_{\max} \approx 0.906$ [Lorenz 1963]. The Lyapunov time, defined as $T_\lambda = 1/\lambda_{\max} \approx 1.1$ time units, characterizes the timescale on which nearby trajectories diverge exponentially.

The prediction task: given a time series of observations $(x_t, y_t, z_t)$ sampled at interval $\Delta t$, predict the one-step-ahead values $(x_{t+1}, y_{t+1}, z_{t+1})$. The system is iterated autoregressively: predictions become inputs for subsequent predictions. Valid prediction time (VPT) is defined as the number of Lyapunov times the iterated forecast remains valid — formally, the first time $t$ such that

$$\text{MSE}(t) = \frac{1}{3}\left[(x_t^{\text{pred}} - x_t^{\text{true}})^2 + (y_t^{\text{pred}} - y_t^{\text{true}})^2 + (z_t^{\text{pred}} - z_t^{\text{true}})^2\right] > 0.05 \cdot \text{Var}(\mathbf{u}).$$

Longer VPT indicates better prediction [Gauthier et al. 2021].

## NVAR Setup and Feature Dimensions

Gauthier et al. [2021] used the following NVAR configuration:

| Parameter | Value |
|-----------|-------|
| Input dimension $d$ | 3 |
| Number of delays $s$ | 2 |
| Delay spacing $k$ | 1 time step |
| Polynomial degree | 2 |
| Feature dimension $D$ | 27 |
| Ridge parameter $\lambda$ | $10^{-7}$ |
| Training length $T_{\text{train}}$ | 5000 steps |

The feature vector dimension is computed as follows. The lagged input vector is $[x_t, y_t, z_t, x_{t-1}, y_{t-1}, z_{t-1}]^\top \in \mathbb{R}^6$. The degree-2 monomials of this 6-vector include: 6 linear terms + $\binom{6+1}{2} = 21$ quadratic terms (including squares), for a total of $D = 6 + 21 = 27$ features. The weight matrix $\mathbf{W}^{\text{out}} \in \mathbb{R}^{3 \times 27}$ has 81 free parameters.

## ESN Baseline

The ESN baseline used:

| Parameter | Value |
|-----------|-------|
| Reservoir size $N$ | 500 |
| Spectral radius $\rho$ | 0.9 |
| Input scaling $\sigma_{\text{in}}$ | 0.5 |
| Leak rate $\alpha$ | 1.0 (no leak) |
| Ridge parameter $\lambda$ | $10^{-7}$ |
| Training length $T_{\text{train}}$ | 5000 steps |

The ESN has $N = 500$ reservoir neurons, giving 500 features and $3 \times 500 = 1500$ readout parameters.

## Comparative Performance

Both NVAR and ESN achieved median VPT of approximately 5 Lyapunov times on the Lorenz prediction task. The distributions of VPT were similar, with NVAR having slightly lower variance (because it is deterministic) and ESN having occasional runs with higher VPT (due to favorable random initialization). The key result is:

$$\text{VPT}_{\text{NVAR}} \approx \text{VPT}_{\text{ESN}} \approx 5 T_\lambda,$$

with NVAR using approximately $1500 / 81 \approx 18\times$ fewer readout parameters [Gauthier et al. 2021].

## The Parameter Efficiency Argument

The NVAR's efficiency advantage is best understood through the lens of effective parameter count. The ESN has $3 \times 500 = 1500$ readout parameters, but the $N = 500$ reservoir neurons are random and not optimized — they provide random features, many of which may be redundant or irrelevant. The NVAR's 27 features are specifically constructed to capture the quadratic nonlinearities of the Lorenz system, making each feature count.

Gauthier et al. [2021] quantify this as: the NVAR captures the geometry of the Lorenz attractor through polynomial features that are naturally adapted to the system's quadratic nonlinearities (the $xy$ and $xz$ terms in the Lorenz equations). The ESN approximates these with random projections, which requires far more features to achieve the same coverage.

## What NVAR Reveals About ESN

The NVAR result has a diagnostic implication for ESN theory. If a 27-dimensional polynomial feature vector can match a 500-dimensional random reservoir, it suggests that the ESN is using far fewer effective features than its nominal dimension implies. The effective dimensionality of the ESN readout — measured by the rank of the state covariance matrix — may be much lower than $N$, particularly when the input lies on a low-dimensional attractor.

This observation connects to the rank analysis of ESN state matrices: for a Lorenz input on a strange attractor of fractal dimension $\approx 2.05$, the state covariance matrix of an ESN should have most of its variance concentrated in a few dimensions. NVAR explicitly captures these dimensions; ESN does so implicitly through the random projection [Gauthier et al. 2021].

## Honest Assessment of Generality

The authors are appropriately cautious about the scope of their finding. NVAR's advantage holds specifically for low-dimensional chaotic systems where (1) the attractor dimension is small, (2) the system's nonlinearities are polynomial or well-approximated by polynomials, and (3) all state variables are observed. For high-dimensional inputs, the polynomial feature explosion (Section 15.3) quickly makes NVAR impractical. For partially observed systems or non-polynomial dynamics, NVAR lacks the implicit memory and nonlinear coverage that the reservoir provides.

---

## References

- Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12(1), 5564.
- Lorenz, E. N. (1963). Deterministic nonperiodic flow. *Journal of the Atmospheric Sciences*, 20(2), 130–141.
