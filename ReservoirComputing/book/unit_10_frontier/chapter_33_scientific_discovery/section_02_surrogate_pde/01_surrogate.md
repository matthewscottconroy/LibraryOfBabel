# Reservoir Surrogates for Partial Differential Equations

## 33.2.1 The Curse of Dimensionality in PDE Solvers

Partial differential equations (PDEs) govern the evolution of physical fields: fluid velocity, temperature, electromagnetic fields, quantum wavefunctions. Solving a PDE numerically requires discretizing the spatial domain into a grid with $N_x$ points per spatial dimension and the temporal domain into $N_t$ steps. The total computational cost is

$$
\mathcal{C}_{\mathrm{PDE}} = O\!\left(N_x^d \cdot N_t\right)
$$

where $d$ is the spatial dimension. For a 3D turbulence simulation with $N_x = 1024$ and $N_t = 10^5$, this is $\sim 10^{20}$ floating-point operations — clearly intractable for repeated evaluations.

**Surrogate modeling** addresses this by training a machine learning model to emulate the PDE solver. The surrogate takes as input the current state of the field (and optionally the parameters) and predicts future states. If the surrogate is faster to evaluate than the PDE solver by several orders of magnitude, and if its accuracy is sufficient for the application, it can replace the solver for the purposes of parameter sweeps, uncertainty quantification, and real-time control.

## 33.2.2 Reservoir Surrogates: The Pathak et al. Approach

[Pathak et al. 2018] introduced a parallel reservoir architecture for surrogate modeling of spatially extended chaotic systems. Their approach was demonstrated on the **Kuramoto-Sivashinsky (KS) equation**:

$$
\frac{\partial u}{\partial t} = -u\frac{\partial u}{\partial x} - \frac{\partial^2 u}{\partial x^2} - \frac{\partial^4 u}{\partial x^4},
$$

a one-dimensional PDE that exhibits spatiotemporal chaos. The KS equation is a canonical benchmark for spatiotemporally complex systems; its Lyapunov exponent $\lambda_1 \approx 0.089$ (in units where the spatial period $L = 22$) means that trajectories diverge on timescales $t \sim 1/\lambda_1 \approx 11$ time units.

**Architecture.** The domain is discretized into $M$ grid points. Rather than using a single large reservoir with $N \gg M$ neurons (which would be expensive and potentially ill-conditioned), Pathak et al. use $M$ **local reservoirs**, one per grid point. The $j$-th reservoir receives as input the local neighborhood:

$$
\mathbf{u}_j(t) = \left[u(j-\kappa, t), \ldots, u(j, t), \ldots, u(j+\kappa, t)\right],
$$

where $\kappa$ is the neighborhood radius. Each local reservoir has $N_{\mathrm{local}}$ neurons and produces a prediction for $u(j, t+\Delta t)$.

**Training.** Run the high-fidelity KS solver for a time $T_{\mathrm{train}}$ and collect the state matrix. Train each local reservoir independently via ridge regression to predict $u(j, t+1)$ from the reservoir state $\mathbf{x}_j(t)$. Total training cost: $O(M \cdot T_{\mathrm{train}} \cdot N_{\mathrm{local}}^2)$.

**Autonomous rollout (testing).** Replace the high-fidelity solver: at each step, feed the reservoir's own previous predictions as the new input, generating a long autonomous trajectory. This is the **closed-loop** or **autonomous** prediction mode that makes the surrogate useful.

## 33.2.3 Performance on the Kuramoto-Sivashinsky Equation

[Pathak et al. 2018] reported the following results for the KS equation with $M = 1024$ grid points:

- **Valid prediction time:** The autonomous reservoir trajectory remained consistent with the true KS trajectory for approximately $8/\lambda_1 \approx 90$ time units, compared to $\sim 0$–$3$ time units for naive persistence (copying the last observed state) and similar performance to Lorenz-96-based reduced-order models.

- **Long-term statistics:** Even after the trajectory diverges from the true KS solution (as it must, given the positive Lyapunov exponent), the autonomous reservoir trajectory exhibits the correct Lyapunov exponents, power spectrum, and spatial correlation functions. The surrogate has learned the attractor structure, not just the short-time behavior.

- **Speedup:** The parallel reservoir surrogate is $\sim 10^4$ times faster to evaluate than the high-fidelity KS solver.

The valid prediction time is quantified by the **valid prediction horizon** $T_\mathrm{valid}$, defined as the largest $t$ such that

$$
\frac{\|u_{\mathrm{RC}}(\cdot, t) - u_{\mathrm{true}}(\cdot, t)\|_2}{\sigma_u} < 0.2,
$$

where $\sigma_u$ is the standard deviation of the KS field.

## 33.2.4 Error Accumulation and Stability

Autonomous rollout suffers from **error accumulation**: small prediction errors compound over time, eventually causing the surrogate trajectory to diverge. The divergence rate is governed by the Lyapunov exponents of the surrogate's dynamics.

**Key stability condition.** For the autonomous reservoir surrogate to produce long-term predictions that are statistically correct, the surrogate must:
1. Have a strange attractor close to the true system's attractor.
2. Be ergodic on this attractor with the correct invariant measure.
3. Have Lyapunov exponents close to those of the true system.

Condition 3 is the most stringent and is not guaranteed by ridge regression training. [Lu et al. 2018] showed that adding a regularization term penalizing the deviation of the surrogate's Lyapunov exponent from the target value can improve long-term stability:

$$
\mathcal{L}_{\mathrm{reg}}(\mathbf{W}^{\mathrm{out}}) = \underbrace{\|\mathbf{X}\mathbf{w} - \mathbf{y}\|^2}_{\text{prediction error}} + \underbrace{\lambda_\mathcal{L}\left(\hat{\lambda}_1(\mathbf{w}) - \lambda_1^{\mathrm{target}}\right)^2}_{\text{Lyapunov regularization}},
$$

where $\hat{\lambda}_1(\mathbf{w})$ is the estimated largest Lyapunov exponent of the closed-loop reservoir.

## 33.2.5 Extensions to Higher-Dimensional PDEs

**Navier-Stokes equations.** [Vlachas et al. 2020] applied parallel reservoirs to 2D turbulence governed by the incompressible Navier-Stokes equations. The approach requires careful domain decomposition: each local reservoir covers a spatial patch, and boundary conditions are exchanged between neighboring reservoirs.

**Wave equations.** [Sanchez-Gonzalez et al. 2020] used graph neural networks (a related approach) for surrogate modeling of fluid simulations on irregular meshes. The parallel reservoir idea extends naturally to irregular spatial domains by associating one reservoir per mesh node and using message-passing between neighbors.

**Climate models.** [Chattopadhyay et al. 2020] applied reservoir surrogates to emulate components of atmospheric general circulation models, including the convective parameterization — one of the most expensive and uncertain components of climate models. The reservoir surrogate was $\sim 100\times$ faster than the conventional parameterization while maintaining comparable accuracy on held-out trajectories.

## 33.2.6 When Does the Surrogate Fail?

Reservoir surrogates can fail in several regimes:

1. **Out-of-distribution inputs.** If the surrogate is evaluated at states far from the training attractor, predictions may be unreliable. Physical systems can be kicked out of the attractor by external forcing, parameter changes, or rare events.

2. **Regime changes.** If the physical system undergoes a bifurcation (e.g., transition from laminar to turbulent flow), the surrogate trained in one regime may fail in the other.

3. **Long memory.** PDEs with long-range temporal correlations (e.g., viscoelastic fluids) require longer reservoir memory than the typical fading-memory scale. Large spectral radius and long washout periods are needed.

4. **Stiff systems.** PDEs with widely separated timescales (stiff equations) require either very small time steps or implicit integration schemes. The standard explicit-time-step reservoir architecture may not handle stiffness well.

## References

- Chattopadhyay, A., Hassanzadeh, P., and Subramanian, D. (2020). Data-driven predictions of a multiscale Lorenz 96 chaotic system using machine-learning methods. *Nonlinear Processes in Geophysics*, 27(3), 373–389.
- Lu, Z., Pathak, J., Hunt, B., Girvan, M., Brockett, R., and Ott, E. (2018). Reservoir observers: Model-free inference of unmeasured variables in chaotic systems. *Chaos*, 27(4), 041102.
- Pathak, J., Hunt, B., Girvan, M., Lu, Z., and Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120, 024102.
- Sanchez-Gonzalez, A., Godwin, J., Pfaff, T., Ying, R., Leskovec, J., and Battaglia, P. (2020). Learning to simulate complex physics with graph networks. In *Proceedings of the 37th ICML*, 8459–8468.
- Vlachas, P. R., Byeon, W., Wan, Z. Y., Sapsis, T. P., and Koumoutsakos, P. (2020). Backpropagation algorithms and reservoir computing in recurrent neural networks for the forecasting of complex spatiotemporal dynamics. *Neural Networks*, 126, 191–217.
