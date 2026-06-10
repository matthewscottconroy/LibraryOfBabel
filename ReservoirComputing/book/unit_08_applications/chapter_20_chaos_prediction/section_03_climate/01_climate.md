# Reservoir Computing for Climate and Weather Prediction

## The Challenge

Weather and climate prediction represent perhaps the most difficult and consequential time series prediction tasks in science. The challenges are: (1) the dynamics are governed by partial differential equations on a continuous spatio-temporal domain — numerically discretized to $\sim 10^7$–$10^9$ degrees of freedom for operational models; (2) the system is chaotic (positive Lyapunov exponents at multiple scales); (3) observations are partial and noisy; (4) the system is non-stationary (climate change modifies the statistics over decadal scales); and (5) the data record is short relative to the relevant timescales [Pathak et al. 2018].

Reservoir computing addresses these challenges from a data-driven direction: instead of integrating a physical model, use historical observations to train a reservoir that can extrapolate the observed dynamics forward in time.

## Pathak et al. 2018: Spatially Parallel Reservoir

The key methodological contribution of Pathak et al. [2018] is the spatially parallel reservoir architecture, designed to handle high-dimensional spatiotemporal fields. The idea is to exploit the local spatial correlations of fluid dynamics: the future state at spatial location $i$ depends primarily on the current state in a local neighborhood $\mathcal{N}(i)$, not on the full global state.

The parallel reservoir architecture assigns one ESN to each spatial location $i$. The input to ESN $i$ is the current state in the neighborhood $\mathcal{N}(i)$, including states at $i$ and its nearest neighbors. The output is the predicted future state at $i$. All ESNs are trained simultaneously, with ridge regression on their respective local observations.

Formally, let $\mathbf{u}_t^{(i)} \in \mathbb{R}^{p \cdot d}$ be the concatenated states of $p$ spatial neighbors of location $i$ at time $t$ ($d$ variables per location). The ESN $i$ updates:

$$\mathbf{x}_t^{(i)} = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1}^{(i)} + \mathbf{W}^{\text{in}} \mathbf{u}_t^{(i)}),$$

and the output is $\hat{\mathbf{u}}_{t+1}^{(i)} = \mathbf{w}^{\text{out}(i) \top} \mathbf{x}_t^{(i)}$. The neighborhood size $p$ is typically 3–5 in 1D (1–2 neighbors on each side) or 5–9 in 2D [Pathak et al. 2018].

The full spatiotemporal prediction is the concatenation of all local predictions, and the system iterates autoregressively. The parallel structure means all ESNs can be trained and evaluated in parallel, keeping the cost linear in the number of spatial locations.

## Chattopadhyay et al. 2019: SST Prediction

Chattopadhyay et al. [2019] applied ESNs to sea surface temperature (SST) prediction in the Pacific Ocean, a task relevant to El Niño–Southern Oscillation (ENSO) forecasting. SST data on a $90 \times 30$ grid (2700 spatial locations) was reduced to $d = 20$ leading EOF (empirical orthogonal function) modes, giving a 20-dimensional dynamical input.

An ESN with $N = 500$ neurons trained on the EOF time series achieved SST prediction skill (correlation with true anomaly) exceeding 0.5 for forecasts 6–12 months ahead — competitive with state-of-art dynamical models at a fraction of the computational cost. The key insight is that ENSO operates primarily in a low-dimensional subspace (the leading EOFs capture $>80\%$ of SST variance), and the ESN can learn the dynamics of this reduced system effectively [Chattopadhyay et al. 2019].

## The Hybrid Physics-ML Approach

A promising middle ground is to combine physics-based models with reservoir corrections. The hybrid approach trains a reservoir to predict the model error (the difference between the physical model's output and the true observation), not the full state:

$$\hat{\mathbf{u}}_{t+1} = \mathbf{F}_{\text{phys}}(\mathbf{u}_t) + \hat{\mathbf{e}}_t,$$

where $\mathbf{F}_{\text{phys}}$ is the numerical weather model (known) and $\hat{\mathbf{e}}_t = \mathbf{W}^{\text{out}} \mathbf{x}_t$ is the ESN-predicted correction. This decomposition is attractive because: (1) the residual error $\mathbf{e}_t$ is typically smoother and smaller than the full state, making it easier to learn; (2) physical conservation laws are satisfied by $\mathbf{F}_{\text{phys}}$ and only corrected by the ESN; (3) the physics provides strong inductive bias that constrains the ESN's role to correcting known model deficiencies [Pathak et al. 2018].

## Limitations of Reservoir-Based Climate Prediction

**Training data length:** Climate prediction requires training on decades to centuries of historical data to capture low-frequency variability (ENSO, PDO, AMO). Reservoirs require $T \gg N$ for stable ridge regression, meaning either very long records or very small reservoirs, both of which limit capacity.

**Non-stationarity:** Climate non-stationarity (due to anthropogenic forcing) violates the stationarity assumption underlying ridge regression and standard ESN theory. Sliding-window training and online RLS updates partially address this but cannot fully adapt to slow trend changes.

**Partial observation:** Global reanalysis datasets provide complete spatial coverage, but paleoclimate records are spatially sparse. Reservoir prediction from partial observations requires careful embedding (Section 20.5).

---

## References

- Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data. *Physical Review Letters*, 120(2), 024102.
- Chattopadhyay, A., Hassanzadeh, P., & Subramanian, D. (2019). Data-driven predictions of a multiscale Lorenz 96 chaotic system using machine-learning methods: Reservoir computing, artificial neural network, and long short-term memory network. *Nonlinear Processes in Geophysics*, 27(3), 373–389.
