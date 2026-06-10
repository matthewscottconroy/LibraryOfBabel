# Data Assimilation with Reservoir Computing

## 33.3.1 The Data Assimilation Problem

**Data assimilation (DA)** is the process of combining a dynamical model with noisy, incomplete observations to produce optimal estimates of the true system state. The problem is ubiquitous in the geosciences: atmospheric reanalysis, ocean state estimation, and numerical weather prediction all rely on DA to correct model forecasts with observational data.

Formally, consider a discrete-time dynamical system:

$$
\mathbf{x}_{t+1} = \mathcal{M}(\mathbf{x}_t) + \boldsymbol{\eta}_t, \qquad \mathbf{y}_t = \mathcal{H}(\mathbf{x}_t) + \boldsymbol{\epsilon}_t,
$$

where $\mathbf{x}_t \in \mathbb{R}^N$ is the true system state, $\mathcal{M}$ is the forecast model (e.g., the Navier-Stokes equations), $\boldsymbol{\eta}_t \sim \mathcal{N}(0, Q)$ is model error, $\mathbf{y}_t \in \mathbb{R}^p$ is the observation vector with $p \ll N$, $\mathcal{H}$ is the observation operator, and $\boldsymbol{\epsilon}_t \sim \mathcal{N}(0, R)$ is observation error.

The DA problem is to compute the **analysis distribution** $p(\mathbf{x}_t | \mathbf{y}_{1:t})$ — the posterior over system states given all observations up to time $t$.

## 33.3.2 Classical Approaches: Kalman and Ensemble Kalman Filters

For **linear** systems ($\mathcal{M}$ and $\mathcal{H}$ linear, Gaussian noise), the **Kalman filter** [Kalman 1960] gives the exact optimal analysis:

$$
\mathbf{x}_t^a = \mathbf{x}_t^f + K_t(\mathbf{y}_t - \mathcal{H}\mathbf{x}_t^f), \qquad K_t = P_t^f \mathcal{H}^T\!\left(\mathcal{H}P_t^f\mathcal{H}^T + R\right)^{-1},
$$

where $\mathbf{x}_t^f$ is the forecast (prior) state, $\mathbf{x}_t^a$ is the analysis (posterior) state, $K_t$ is the **Kalman gain**, and $P_t^f$ is the forecast error covariance.

For **nonlinear** systems, the Kalman filter is approximated by the **ensemble Kalman filter (EnKF)** [Evensen 1994, 2009]: an ensemble $\{\mathbf{x}_t^{(i)}\}_{i=1}^{N_e}$ of $N_e$ model states propagates through the forecast model, and the covariance is estimated from the ensemble spread. The analysis step updates each ensemble member:

$$
\mathbf{x}_t^{a,(i)} = \mathbf{x}_t^{f,(i)} + K_t^{\mathrm{EnKF}}\!\left(\mathbf{y}_t + \boldsymbol{\epsilon}_t^{(i)} - \mathcal{H}\mathbf{x}_t^{f,(i)}\right),
$$

where $K_t^{\mathrm{EnKF}}$ is computed from the ensemble covariance. The EnKF is the workhorse of operational weather forecasting [Kalnay 2003].

**Limitation.** Both the Kalman filter and EnKF require an explicit forecast model $\mathcal{M}$. In many scientific applications, $\mathcal{M}$ is unknown, incorrect, or too expensive to evaluate. This motivates the reservoir computing approach.

## 33.3.3 Reservoir-Based Data Assimilation

The key idea of [Brajard et al. 2020] is to replace the forecast model $\mathcal{M}$ with a trained reservoir. The two-stage procedure is:

**Stage 1: Reservoir training.** Train an ESN to emulate the system dynamics using available observations (which may be sparse and noisy). Let the reservoir state $\mathbf{x}(t)$ evolve according to

$$
\mathbf{x}(t+1) = \tanh\!\left(W^{\mathrm{rec}}\mathbf{x}(t) + W^{\mathrm{in}}\mathbf{u}(t) + W^{\mathrm{obs}}\mathbf{y}(t)\right),
$$

where $\mathbf{u}(t)$ is any available external forcing and $\mathbf{y}(t)$ is the observation vector. During training, $\mathbf{y}(t)$ is available; during prediction, $\mathbf{y}(t)$ must be estimated.

**Stage 2: Assimilation with nudging.** The **nudging** approach [Auroux & Blum 2008] corrects the reservoir trajectory toward observations:

$$
\mathbf{x}(t+1) = \tanh\!\left(W^{\mathrm{rec}}\mathbf{x}(t) + W^{\mathrm{in}}\mathbf{u}(t)\right) + \underbrace{\kappa\,W^{\mathrm{obs}}\left(\mathbf{y}(t) - \hat{\mathbf{y}}(t)\right)}_{\text{nudging correction}},
$$

where $\hat{\mathbf{y}}(t) = \mathcal{H}\,\hat{\mathbf{x}}(t)$ is the predicted observation and $\kappa > 0$ is the nudging coefficient. When new observations arrive, the reservoir state is nudged toward consistency with those observations.

## 33.3.4 The Echo State Property as the Key Tool

The echo state property (ESP) plays a crucial role in reservoir-based DA. Recall (Chapter 5) that the ESP means: for any input sequence, the reservoir state converges to a unique response determined by the input, independent of initial conditions.

**In the DA context:** If the reservoir satisfies the ESP with inputs $\mathbf{u}(t)$ and observations $\mathbf{y}(t)$, then the reservoir state $\mathbf{x}(t)$ is a deterministic function of the input-observation history. This means:

1. **Initialization independence.** After a washout period, the reservoir state does not depend on the initial conditions — so initialization errors are suppressed automatically.

2. **Observational correction.** When observations arrive, the nudging correction perturbs $\mathbf{x}(t)$. If the perturbation is consistent with the true system state, the ESP guarantees that the reservoir trajectory converges back to the correct manifold.

3. **Filter stability.** The exponential convergence guaranteed by the ESP implies that analysis errors are not amplified over time — a necessary condition for filter stability [Carrassi et al. 2018].

This is formalized as:

**Proposition 33.1.** If the driven reservoir satisfies the ESP and the observation operator $\mathcal{H}$ is injective (distinct states produce distinct observations), then the reservoir-based DA procedure converges to the true state as the nudging coefficient $\kappa \to 1/(1 + \sigma_{\max}(\mathcal{H}^T R^{-1}\mathcal{H}))$.

## 33.3.5 Results on the Lorenz System

[Brajard et al. 2020] demonstrated reservoir-based DA on the Lorenz-63 system:

$$
\dot{x} = \sigma(y-x), \quad \dot{y} = x(\rho-z)-y, \quad \dot{z} = xy-\beta z,
$$

with $\sigma = 10$, $\rho = 28$, $\beta = 8/3$. Observations were taken every $\Delta t = 0.1$ time units on a subset of components with 10% Gaussian noise.

**Key results:**
- The reservoir-based DA achieved state estimation error comparable to the EnKF, despite not having access to the true equations of motion.
- When the forecast model was intentionally incorrect (wrong $\rho$), the reservoir-based DA was more robust than the model-based EnKF.
- The method remained functional even when only one of three state variables was observed — demonstrating successful state reconstruction from partial observations.

## 33.3.6 Comparison: Reservoir DA vs. EnKF

| Property | EnKF | Reservoir DA |
|---|---|---|
| Requires explicit model | Yes | No |
| Handles model error | Partially | Naturally |
| Online learning | No | Possible |
| Computational cost | $O(N_e \cdot N)$ per step | $O(N^2)$ per step |
| Theoretical guarantees | Well-developed | Active research |
| Applicable to high-dimensional systems | Yes (localization) | With parallel architecture |

## 33.3.7 Applications: Atmospheric Reanalysis and Ocean Estimation

**Atmospheric reanalysis.** Reanalysis products (ERA5, NCEP-NCAR) combine historical observations with model forecasts to produce gridded estimates of atmospheric variables. Reservoir DA could potentially replace the expensive numerical forecast model in these systems, reducing computation time while handling model errors more gracefully [Kalnay 2003].

**Ocean state estimation.** Ocean state estimation faces additional challenges: the ocean is underobserved (satellite altimetry gives sea surface height but not interior state), and ocean models have large systematic biases. [Bocquet et al. 2019] demonstrated reservoir-based DA for simplified ocean models, with results competitive with variational methods (4D-Var).

## References

- Auroux, D. and Blum, J. (2008). A nudging-based data assimilation method: the back and forth nudging (BFN) algorithm. *Nonlinear Processes in Geophysics*, 15(2), 305–319.
- Bocquet, M., Brajard, J., Carrassi, A., and Bertino, L. (2019). Data assimilation as a learning tool to infer ordinary differential equation representations of dynamical models. *Nonlinear Processes in Geophysics*, 26(3), 143–162.
- Brajard, J., Carrassi, A., Bocquet, M., and Bertino, L. (2020). Combining data assimilation and machine learning to emulate a dynamical model from sparse and noisy observations. *Journal of Computational Science*, 44, 101171.
- Carrassi, A., Bocquet, M., Bertino, L., and Evensen, G. (2018). Data assimilation in the geosciences: An overview of methods, issues, and perspectives. *WIREs Climate Change*, 9(5), e535.
- Evensen, G. (1994). Sequential data assimilation with a nonlinear quasi-geostrophic model using Monte Carlo methods to forecast error statistics. *Journal of Geophysical Research*, 99(C5), 10143–10162.
- Evensen, G. (2009). *Data Assimilation: The Ensemble Kalman Filter*. Springer.
- Kalman, R. E. (1960). A new approach to linear filtering and prediction problems. *Transactions of the ASME — Journal of Basic Engineering*, 82(1), 35–45.
- Kalnay, E. (2003). *Atmospheric Modeling, Data Assimilation and Predictability*. Cambridge University Press.
