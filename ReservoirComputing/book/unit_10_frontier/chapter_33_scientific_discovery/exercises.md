# Chapter 33 Exercises: Reservoir Computing for Scientific Discovery

## Section 33.1 — Equation of State

**Exercise 33.1.** Explain why an equilibrium equation of state $p = f(\rho, T)$ can be learned by a feedforward network but a nonequilibrium EOS requires a recurrent architecture. What property of the ESN (fading memory, echo state property, or Lyapunov stability) is the relevant one?

**Exercise 33.2.** Suppose the nonequilibrium EOS relaxes to its equilibrium value on timescale $\tau_\mathrm{relax}$. How should the reservoir memory be tuned (e.g., spectral radius $\rho$, leak rate $\alpha$) to match this timescale? Derive the relationship between $\tau_\mathrm{relax}$ and the optimal $\rho$ using the leaky-ESN time constant $\tau_\mathrm{ESN} = -\Delta t / \log \alpha$.

**Exercise 33.3.** The ensemble uncertainty estimate $\hat{\sigma}^2(t) = \frac{1}{M}\sum_m (\hat{p}_m(t) - \hat{p}(t))^2$ measures variance across reservoir initializations. This is an estimate of epistemic uncertainty but not aleatoric uncertainty (irreducible noise). How would you modify the training procedure to also estimate aleatoric uncertainty?

**Exercise 33.4.** A Gaussian process with kernel $k((\rho, T), (\rho', T')) = \exp(-\|(\rho,T)-(\rho',T')\|^2/(2\ell^2))$ is trained on the same EOS data as the reservoir. For equilibrium EOS (no temporal dependence), under what conditions would the GP outperform the reservoir? Under what conditions would the reverse hold?

**Exercise 33.5** (Hard). Suppose the EOS has a first-order phase transition at $\rho_c$, where $f$ is discontinuous. Propose a modification to the reservoir architecture or training procedure that can handle this discontinuity. What are the limitations of your approach?

## Section 33.2 — Surrogate PDEs

**Exercise 33.6.** The Kuramoto-Sivashinsky (KS) equation $\partial_t u = -u\partial_x u - \partial_{xx} u - \partial_{xxxx} u$ is spatially extended. Explain why a single global reservoir of size $N$ is less efficient than $M$ local reservoirs of size $N/M$ (with $M$ = number of grid points) for this system. Consider both computational cost and effective expressiveness.

**Exercise 33.7.** The valid prediction horizon $T_\mathrm{valid}$ is defined as the largest $t$ such that $\|u_\mathrm{RC}(\cdot,t) - u_\mathrm{true}(\cdot,t)\|_2/\sigma_u < 0.2$. For the KS equation with Lyapunov exponent $\lambda_1 \approx 0.089$, what is the maximum possible $T_\mathrm{valid}$ for a perfect surrogate, and what limits it?

**Exercise 33.8.** Error accumulation in autonomous rollout can be modeled as a dynamical system: let $\delta(t) = u_\mathrm{RC}(\cdot,t) - u_\mathrm{true}(\cdot,t)$ be the prediction error. Linearizing the surrogate dynamics around the true trajectory, show that $\delta(t) \approx e^{J_t \Delta t}\delta(t-1) + \epsilon(t)$, where $J_t$ is the Jacobian of the surrogate dynamics and $\epsilon(t)$ is the one-step prediction error. Under what conditions on $J_t$ does $\delta(t)$ remain bounded?

**Exercise 33.9.** Implement a reservoir surrogate for the 1D heat equation $\partial_t u = \alpha \partial_{xx} u$ with periodic boundary conditions. Use $M = 50$ grid points, $N_\mathrm{local} = 20$ neurons per local reservoir, and neighborhood radius $\kappa = 2$. Compare the surrogate's predictions with the analytical solution for initial conditions $u(x,0) = \sin(2\pi x)$.

**Exercise 33.10** (Research). Propose a method for estimating the valid prediction horizon $T_\mathrm{valid}$ of a surrogate without running the full high-fidelity simulation. *Hint: consider using the singular values of the Jacobian of the surrogate's one-step-ahead mapping.*

## Section 33.3 — Data Assimilation

**Exercise 33.11.** The nudging correction in the reservoir DA formulation is $\kappa W^\mathrm{obs}(\mathbf{y}(t) - \hat{\mathbf{y}}(t))$. For $\kappa = 0$, the reservoir runs in free forecast mode. For $\kappa \to \infty$, the reservoir state is forced to be consistent with observations. Describe the behavior in each limit and derive the optimal $\kappa$ in terms of the observation error covariance $R$ and the reservoir state covariance.

**Exercise 33.12.** The Kalman filter is optimal for linear-Gaussian systems. Explain why reservoir DA is not guaranteed to be optimal in this sense. What property of the reservoir dynamics would be needed for reservoir DA to be optimal?

**Exercise 33.13.** In the Lorenz-63 data assimilation experiment of [Brajard et al. 2020], only one of three state variables was observed. Explain why the echo state property allows the reservoir to reconstruct all three variables from observations of one. What would fail if the ESP were violated?

**Exercise 33.14.** The ensemble Kalman filter (EnKF) requires $N_e$ model runs per analysis cycle. The reservoir DA requires one reservoir run. For a complex PDE model (e.g., global atmosphere), compare the computational cost of EnKF (with $N_e = 100$ ensemble members) to reservoir DA (with $N = 10^4$ neurons). Which is cheaper? What are the tradeoffs?

## Section 33.4 — Conservation Laws

**Exercise 33.15.** Derive the constrained ridge regression solution $\hat{\mathbf{W}} = (\mathbf{X}^T\mathbf{X} + \lambda\mathbf{I})^{-1}\mathbf{X}^T\mathbf{Y}\,\Pi_C$ using Lagrange multipliers. Show that this solution satisfies $C\hat{\mathbf{W}} = 0$.

**Exercise 33.16.** For the incompressibility constraint $\nabla \cdot \mathbf{v} = 0$ on a 2D grid with $M \times M$ grid points, write the constraint matrix $C$ explicitly for a finite-difference discretization. What is the rank of $C$?

**Exercise 33.17.** Compare the soft-constraint and hard-constraint approaches for a mass conservation law $\sum_i w_i \hat{y}_i = m$, where $w_i$ are fixed weights. (a) Write the soft-constraint training objective. (b) Write the KKT conditions for the hard-constraint problem. (c) For what value of $\lambda_2$ (soft) are the solutions approximately equal?

**Exercise 33.18** (Hard). Consider a Hamiltonian system $\dot{q} = \partial H/\partial p$, $\dot{p} = -\partial H/\partial q$ with $H(q,p) = p^2/2m + V(q)$. Design a reservoir architecture whose autonomous dynamics (no input) are symplectic. What constraints does this place on $W^\mathrm{rec}$? Is the resulting reservoir still a universal approximator for fading-memory functionals?

## Section 33.5 — High-Energy Physics

**Exercise 33.19.** The jet classification problem requires processing a sequence of $k = 100$ constituents in $\leq 100$ ns. At a clock speed of 10 GHz (100 ps per step), this gives 100 time steps — just enough. However, the readout (linear multiply) adds latency. Estimate the FPGA latency for a linear readout with $N = 50$ reservoir neurons, assuming each multiply-accumulate takes 1 clock cycle.

**Exercise 33.20.** The photonic reservoir for jet classification uses a delayed-feedback architecture with one physical node. How does the delay $\tau$ (in units of the constituent time spacing $\Delta t = 1$ ns) affect the effective reservoir size? Propose a value of $\tau$ that balances memory with computational feasibility.

**Exercise 33.21.** The ROC curve (receiver operating characteristic) for jet classification measures the trade-off between signal efficiency (true positive rate) and background rejection (1/false positive rate). For a classifier with AUC = 0.90, what is the background rejection at 50% signal efficiency? Compare to the corresponding value for AUC = 0.95.

**Exercise 33.22** (Research). Propose a physical reservoir architecture (not optoelectronic, but another physical substrate) that could achieve $< 10$ ns latency for jet classification. Justify your choice of substrate based on the physical timescale of the dynamics, the achievable reservoir size, and the noise characteristics.
