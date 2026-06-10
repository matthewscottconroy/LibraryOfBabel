# Prediction from Partial Observations

## The Partial Observation Problem

Real physical systems are rarely fully observed. Weather data comes from surface stations, radiosondes, and satellites — not from a dense volumetric array. Neural recordings capture a small fraction of cortical neurons. Economic data misses informal markets. In all these cases, the observer has access to a subset of the system's state variables, and must forecast the full trajectory from this incomplete information.

For a chaotic system with state $\mathbf{s}(t) = (s_1(t), \ldots, s_d(t))$, suppose only $s_1(t)$ is observed. The question is: can a reservoir predict future values of $(s_1(t), \ldots, s_d(t))$ — including unobserved variables — from the observed scalar stream $\{s_1(t)\}$?

The generic answer, grounded in Takens' theorem [Takens 1981], is yes: a delay embedding of the scalar observation generically reconstructs the full attractor, enabling prediction of all state variables.

## The Generic Embedding Theorem

For a generic observation function $h : M \to \mathbb{R}$ and generic delay $\tau$, the delay embedding

$$\boldsymbol{\psi}(s(t)) = [h(s(t)), h(s(t-\tau)), \ldots, h(s(t-2m\tau))]^\top \in \mathbb{R}^{2m+1}$$

is a diffeomorphism onto the image $\boldsymbol{\psi}(M) \subset \mathbb{R}^{2m+1}$, provided $m \geq d_A$ (the attractor dimension) [Takens 1981]. The word "generic" means the result holds for almost every $(h, \tau)$ in a suitable function space — near-coincidences with the natural symmetries of the attractor are the exceptions.

The practical implication: if one constructs the delay embedding from $s_1(t)$ and runs a reservoir with this embedding as input, the reservoir state lies in a space homeomorphic to the full attractor. A linear readout can then extract any smooth function of the full attractor state — including the values of unobserved variables $s_2(t), \ldots, s_d(t)$.

## Reservoir Approach with Delay Embedding

For a scalar observation $u_t = s_1(t)$, the reservoir input is the $m$-step delay vector:

$$\mathbf{v}_t = [u_t, u_{t-\tau}, u_{t-2\tau}, \ldots, u_{t-(m-1)\tau}]^\top \in \mathbb{R}^m.$$

The reservoir update is:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{v}_t),$$

and the readout predicts all state variables:

$$\hat{\mathbf{s}}_{t+\Delta t} = \mathbf{W}^{\text{out}} \mathbf{x}_t.$$

The delay vector $\mathbf{v}_t$ serves as an explicit Takens embedding; the reservoir provides an additional nonlinear transformation of this embedding into a higher-dimensional feature space. Even without the explicit delay embedding (using $m = 1$), the reservoir's internal dynamics implement an implicit delay embedding through the fading memory of the recurrent connections [Lu et al. 2017].

## Lu et al. 2017: Single-Variable Lorenz Reconstruction

Lu et al. [2017] demonstrated this approach on the Lorenz system with only $x(t)$ observed ($y(t)$ and $z(t)$ hidden). Two experiments:

**Experiment 1: Closed-loop reconstruction.** Train a reservoir to reconstruct $(y_t, z_t)$ from a history of $x_t$ values. The readout provides $(\hat{y}_t, \hat{z}_t)$; the input is $x_t$ only. Result: after $T_{\text{train}} = 10^4$ training steps, the reservoir correctly reconstructed $(y, z)$ with correlation $> 0.99$ over short windows.

**Experiment 2: Autonomous attractor reconstruction.** Train a reservoir to predict $x_{t+1}$ from $x_t$ using delay embedding, then run autonomously (predicted $\hat{x}_t$ fed back as input). The autonomous reservoir produced a trajectory with correct Lorenz attractor statistics ($D_2 \approx 2.05$, correct power spectrum) [Lu et al. 2017].

## The Noise Challenge

The delay embedding amplifies noise. For a noisy observation $u_t = s_1(t) + \epsilon_t$ with $\epsilon_t \sim \mathcal{N}(0, \sigma^2)$, the delay vector has noise:

$$\mathbf{v}_t^{\text{noisy}} = \mathbf{v}_t + [\epsilon_t, \epsilon_{t-\tau}, \ldots, \epsilon_{t-(m-1)\tau}]^\top.$$

The noise variance in the delay vector is $m\sigma^2$ — growing linearly with the number of delays. For $m = 10$ and $\sigma^2 = 0.01$, the delay vector noise variance is 0.1 — potentially large enough to degrade the embedding quality.

Regularization is crucial: the ridge parameter $\lambda$ must be chosen to balance noise amplification from the delay embedding against underfitting. Optimal $\lambda$ increases with $m\sigma^2$, as derived from the bias-variance tradeoff for ridge regression. Cross-validation on a validation segment is the standard approach [Stark 1999].

## Application: Weather Prediction from Surface Measurements

The partial observation framework directly motivates using surface weather observations (temperature, pressure, humidity at ground stations) to forecast upper-atmosphere conditions. Upper-atmosphere state variables (wind shear, jet stream position) are unobserved but causally related to surface variables through the atmosphere's dynamics.

The reservoir approach: run the reservoir on a long record of surface observations, train a readout to predict both future surface observations and available upper-atmosphere measurements (from radiosonde launches or satellite retrievals). The Takens/Boyd–Chua guarantee provides the theoretical backing for this extrapolation from partial to full state.

---

## References

- Lu, Z., Pathak, J., Hunt, B., Girvan, M., Brockett, R., & Ott, E. (2017). Reservoir observers: Model-free inference of unmeasured variables in chaotic systems. *Chaos*, 27(4), 041102.
- Takens, F. (1981). Detecting strange attractors in turbulence. In *Dynamical Systems and Turbulence*, pp. 366–381. Springer.
- Stark, J. (1999). Delay embeddings for forced systems. I. Deterministic forcing. *Journal of Nonlinear Science*, 9(3), 255–332.
