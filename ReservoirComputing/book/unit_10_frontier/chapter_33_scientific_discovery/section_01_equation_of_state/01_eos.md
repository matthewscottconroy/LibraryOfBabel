# Reservoir Computing for Equation-of-State Inference

## 33.1.1 The Equation-of-State Problem

An **equation of state (EOS)** specifies a constitutive relationship between thermodynamic variables for a material. The simplest example is the ideal gas law $p = \rho R T / M$, where $p$ is pressure, $\rho$ is density, $T$ is temperature, and $M$ is molar mass. For real materials, the EOS is far more complex: it encodes quantum mechanical effects, electron-ion coupling, phase transitions, and nonequilibrium processes that cannot be captured by simple analytical formulas.

The standard problem formulation is: given a physical state characterized by density $\rho$ and temperature $T$ (and possibly the history of how the system reached that state), predict a thermodynamic observable such as pressure $p$, internal energy $E$, or opacity $\kappa$. Formally, in the equilibrium case:

$$
p = f_{\mathrm{EOS}}(\rho, T),
$$

where $f_{\mathrm{EOS}}$ is the EOS function. This function can be computed from first principles using molecular dynamics (MD) simulations or density functional theory (DFT), but such computations are expensive: a single evaluation of $f_{\mathrm{EOS}}$ for dense plasma may require hours of CPU time. Practical applications (radiation-hydrodynamics codes, ICF simulations, astrophysical models) must evaluate the EOS millions of times per simulation, making first-principles evaluation intractable.

## 33.1.2 Why Temporal Structure Matters

In **equilibrium** thermodynamics, the EOS is a function of the instantaneous state: $p = f(\rho, T)$ depends only on the current values of $\rho$ and $T$. But many physically important regimes are **nonequilibrium**: the system evolves on timescales comparable to or shorter than the relaxation time, and the current state reflects the history of driving.

**Example.** In laser-driven warm dense matter (WDM) experiments, a femtosecond laser pulse heats a thin foil to plasma conditions. The electron and ion temperatures $T_e$ and $T_i$ evolve on different timescales (electron-phonon coupling time $\sim 1$–$10$ ps), and the pressure depends on both $T_e$, $T_i$, and their history of equilibration. A memoryless model $p = f(\rho, T_e, T_i)$ misses these nonequilibrium effects.

For nonequilibrium EOS, the correct formulation is:

$$
p(t) = F_{\mathrm{EOS}}\!\left[\rho(\cdot), T_e(\cdot), T_i(\cdot)\right](t),
$$

where $F_{\mathrm{EOS}}$ is a *functional* of the history. This is precisely the type of object that reservoir computing is designed to approximate (Chapter 26).

## 33.1.3 The Reservoir Approach

**Architecture.** The input to the reservoir at time step $t$ is the vector $\mathbf{u}(t) = (\rho(t), T(t))^T$ (or $(\rho(t), T_e(t), T_i(t))^T$ for the two-temperature model). The ESN state evolves as

$$
\mathbf{x}(t+1) = \tanh\!\left(W^{\mathrm{rec}}\mathbf{x}(t) + W^{\mathrm{in}}\mathbf{u}(t)\right),
$$

and the output is the readout

$$
\hat{p}(t) = \mathbf{w}^T\mathbf{x}(t).
$$

The reservoir implicitly maintains a memory of the thermodynamic path $(\rho(\tau), T(\tau))_{\tau \leq t}$, enabling prediction of nonequilibrium pressure contributions. Because the EOS functional $F_{\mathrm{EOS}}$ is causal and fading-memory (relaxation processes decay exponentially), the Boyd-Chua universality theorem guarantees that this architecture can approximate $F_{\mathrm{EOS}}$ to arbitrary precision (Section 26).

**Training.** A small library of molecular dynamics or DFT simulations is run for different thermodynamic trajectories $(\rho(t), T(t))_{t=1}^{T_{\max}}$. The resulting pressure trajectories $p(t)$ serve as training targets. Ridge regression solves for $\mathbf{w}$:

$$
\hat{\mathbf{w}} = \left(\mathbf{X}^T\mathbf{X} + \lambda\mathbf{I}\right)^{-1}\mathbf{X}^T\mathbf{p},
$$

where $\mathbf{X} \in \mathbb{R}^{T \times N}$ is the state matrix and $\mathbf{p} \in \mathbb{R}^T$ is the pressure vector.

## 33.1.4 Application to Dense Plasma

[Désert et al. 2022] applied reservoir computing to equation-of-state inference for hot dense plasma under conditions relevant to inertial confinement fusion (ICF). Their setup:

- **Physical model:** Two-temperature hydrodynamics for hydrogen plasma at densities $\rho \in [0.1, 10]\,\mathrm{g/cm^3}$ and temperatures $T \in [10^3, 10^7]\,\mathrm{K}$.
- **Training data:** 200 DFT-MD trajectories of length 500 time steps each.
- **Reservoir:** $N = 500$ neurons, spectral radius $\rho = 0.95$.
- **Result:** Normalized root mean squared error (NRMSE) of $\sim 2\%$ on held-out test trajectories — comparable to Gaussian process regression but $\sim 100\times$ faster to evaluate.

The reservoir outperformed static regression models (polynomial fits, feedforward networks) on trajectories with rapid temperature changes, where the history-dependence of the EOS is most pronounced. This demonstrates that the temporal memory of the reservoir contributes predictive value beyond instantaneous state information.

## 33.1.5 Comparison with Gaussian Processes

Gaussian processes (GPs) [Rasmussen & Williams 2006] are the standard Bayesian nonparametric method for EOS regression. The GP approach is:

$$
f_{\mathrm{EOS}}(\rho, T) \sim \mathcal{GP}\!\left(\mu(\rho, T),\, k\!\left((\rho, T), (\rho', T')\right)\right),
$$

where $\mu$ is a mean function and $k$ is a covariance kernel. GP prediction gives both a mean and a variance (uncertainty estimate).

**GP vs. reservoir comparison:**

| Property | Gaussian Process | Reservoir |
|---|---|---|
| Temporal memory | No (instantaneous input) | Yes (fading memory) |
| Uncertainty quantification | Native | Requires ensemble |
| Training cost | $O(n^3)$ in training set size | $O(N^2 T)$ |
| Evaluation cost | $O(n)$ per query | $O(N^2)$ per time step |
| Nonequilibrium EOS | No | Yes |

For equilibrium EOS, GPs are superior: they provide uncertainty estimates and scale to large training sets with sparse approximations [Becker et al. 2020]. For nonequilibrium EOS with temporal structure, reservoirs are advantageous.

## 33.1.6 Uncertainty Quantification via Ensembles

A limitation of the standard reservoir approach is the absence of native uncertainty quantification. [Désert et al. 2022] addressed this by training an **ensemble** of reservoirs: $M$ reservoirs with different random initializations and/or trained on different bootstrap samples of the training data. The ensemble prediction is:

$$
\hat{p}(t) = \frac{1}{M}\sum_{m=1}^M \hat{p}_m(t), \quad \hat{\sigma}^2(t) = \frac{1}{M}\sum_{m=1}^M \left(\hat{p}_m(t) - \hat{p}(t)\right)^2.
$$

The ensemble variance $\hat{\sigma}^2(t)$ provides a measure of **epistemic uncertainty** — uncertainty due to limited training data. For well-explored regions of $(\rho, T)$ space, $\hat{\sigma}^2$ is small; for extrapolation regimes, it grows. This allows the surrogate to flag predictions that should be verified with additional high-fidelity simulations.

## 33.1.7 Limitations and Open Questions

**Extrapolation.** Like all machine learning methods, the reservoir surrogate may fail outside the training distribution. Physical extrapolation — predicting at densities or temperatures not seen during training — requires careful uncertainty quantification and validation against first-principles calculations.

**Interpretability.** The reservoir EOS model is a black box: it does not reveal the underlying physical mechanisms. For scientific understanding, a symbolic or physically motivated model may be preferable. Hybrid approaches combining reservoir surrogates with physics-informed constraints (Section 33.4) are an active research area.

**Phase transitions.** EOS functions have discontinuities at phase boundaries (first-order transitions). Reservoir surrogates with smooth activation functions cannot represent these discontinuities exactly; training must be restricted to single phases or special architecture modifications are needed.

## References

- Becker, A., Lorenzen, M., Redmer, R., and Schöttler, M. A. (2020). Calculating dense plasma equations of state with deep learning. *Physical Review E*, 101, 053301.
- Désert, T., Clérouin, J., Recoules, V., and Becker, A. (2022). Equation of state of hot dense matter with reservoir computing. *Physical Review E*, 105, 025210.
- Rasmussen, C. E. and Williams, C. K. I. (2006). *Gaussian Processes for Machine Learning*. MIT Press.
- Thomas, V. A., Herring, G. L., and Baus, M. (2020). Machine-learned potentials for next-generation matter simulations. *Nature Reviews Materials*, 5(12), 845–871.
