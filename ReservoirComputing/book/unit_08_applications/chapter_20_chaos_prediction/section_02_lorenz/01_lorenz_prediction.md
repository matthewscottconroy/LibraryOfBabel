# Lorenz System Prediction with Reservoir Computing

## The Lorenz System

The Lorenz system is a three-dimensional autonomous ordinary differential equation:

$$\dot{x} = \sigma(y - x), \quad \dot{y} = x(\rho - z) - y, \quad \dot{z} = xy - \beta z,$$

introduced by Edward Lorenz in 1963 as a simplified model of atmospheric convection [Lorenz 1963]. At the canonical parameter values $\sigma = 10$, $\rho = 28$, $\beta = 8/3$, the system has a strange attractor — a bounded, aperiodic trajectory that never repeats. The attractor has fractal dimension approximately 2.05 and maximal Lyapunov exponent $\lambda_{\max} \approx 0.906$ [Sprott 2003].

The Lyapunov exponent $\lambda_{\max}$ quantifies the exponential divergence of nearby trajectories: two initial conditions separated by $\epsilon$ will diverge to order-one distance in time $T_\lambda \approx 1/\lambda_{\max} \approx 1.1$ time units. This is the Lyapunov time — the fundamental timescale of predictability for the Lorenz system.

## The Reservoir Prediction Setup

The prediction task is to observe a finite segment of the Lorenz trajectory and produce forecasts up to several Lyapunov times ahead. Specifically:

**Input:** $\mathbf{u}_t = (x_t, y_t, z_t)$ sampled at interval $\Delta t$ (typically $\Delta t = 0.02$, giving 50 samples per Lorenz time unit).

**Target:** $\mathbf{y}_t^* = (x_{t+\Delta t}, y_{t+\Delta t}, z_{t+\Delta t})$ (one-step-ahead prediction).

**Evaluation:** Iterative one-step prediction (autoregressive forecasting): use the reservoir's prediction as the next input. Valid prediction time (VPT) is measured in Lyapunov times [Jaeger & Haas 2004].

The reservoir state update during evaluation is:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \hat{\mathbf{u}}_t),$$

where $\hat{\mathbf{u}}_t = \hat{\mathbf{W}}^{\text{out}} \mathbf{x}_{t-1}$ is the predicted state (output feedback), making the evaluation mode a fully autonomous dynamical system.

## The VPT Metric

The valid prediction time is defined as the first time step $T^*$ such that the normalized mean squared error exceeds a threshold:

$$\text{VPT} = T^* \cdot \Delta t / T_\lambda, \quad T^* = \min\left\{ t : \frac{\|\hat{\mathbf{u}}_t - \mathbf{u}_t^{\text{true}}\|^2}{\text{Var}(\mathbf{u})} > \delta \right\},$$

where $\delta = 0.05$ is the standard threshold [Jaeger & Haas 2004]. VPT is dimensionless (measured in Lyapunov times), enabling comparison across systems and prediction tasks.

For reference: a persistence forecast ($\hat{\mathbf{u}}_t = \mathbf{u}_{t-1}$) achieves VPT $\approx 0.1$ Lyapunov times; climatological mean achieves VPT = 0. A VPT of 3–5 Lyapunov times is considered good for reservoir computing.

## Jaeger & Haas 2004: The Foundational Result

The first demonstration of reservoir computing for Lorenz prediction was Jaeger & Haas [2004] in Science, which also introduced the echo state network framework. Using a reservoir of $N = 1000$ neurons, spectral radius $\rho = 0.9$, and trained with ridge regression, they achieved valid prediction times of approximately 3–5 Lyapunov times on the Lorenz system.

This result was remarkable in context: at the time, no neural network had achieved such accurate chaotic prediction, and classical numerical integration from estimated initial conditions achieved comparable VPT only with far more prior knowledge. The ESN accomplished this with approximately 3000 training time steps (60 Lorenz time units) and a single offline ridge regression solve.

## Pathak et al. 2018: Scaling to Spatiotemporal Chaos

Pathak et al. [2018] extended ESN Lorenz prediction by applying a parallel reservoir architecture to the spatiotemporally chaotic Kuramoto–Sivashinsky (KS) equation:

$$\partial_t u + u \partial_x u + \partial_{xx} u + \partial_{xxxx} u = 0,$$

which is a canonical model of turbulent dynamics with many positive Lyapunov exponents. The KS equation was discretized on a grid of $L = 400$ spatial points, giving a 400-dimensional prediction problem that is too large for a single reservoir.

The parallel reservoir approach divides the spatial domain into overlapping local regions, assigns a separate reservoir to each region, and couples adjacent reservoirs through shared boundary state. This allows the architecture to scale to arbitrarily large spatial domains. On the KS equation with $L = 400$, parallel reservoirs achieved VPT $\approx 8$ Lyapunov times — outperforming all previous methods on this benchmark and establishing a new standard for reservoir-based spatiotemporal prediction [Pathak et al. 2018].

## Why Reservoirs Excel at Lorenz Prediction

The Lorenz attractor has fractal dimension $\approx 2.05$, meaning that despite living in a 3-dimensional ambient space, the dynamics lie effectively on a 2-dimensional manifold. A reservoir of $N$ neurons with the echo state property generates states that lie on an $N$-dimensional manifold that tracks the Lorenz attractor. For $N \gg 2$, the reservoir manifold is much larger than the attractor, providing ample room for accurate attractor representation.

The key property exploited is that the Lorenz attractor is an invariant set of the driving dynamics, and the reservoir's echo state property ensures that the reservoir state is uniquely determined by the input history — so the reservoir state uniquely encodes the position on the attractor. A linear readout can then extract the next-step position from this encoding [Jaeger & Haas 2004].

---

## References

- Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.
- Sprott, J. C. (2003). *Chaos and Time-Series Analysis*. Oxford University Press.
- Lorenz, E. N. (1963). Deterministic nonperiodic flow. *Journal of the Atmospheric Sciences*, 20(2), 130–141.
