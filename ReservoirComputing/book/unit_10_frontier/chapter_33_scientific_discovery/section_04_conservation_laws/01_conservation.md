# Enforcing Conservation Laws in Reservoir Surrogates

## 33.4.1 The Problem of Physical Constraint Violation

Physical systems are governed not only by differential equations but by conservation laws: energy, mass, momentum, angular momentum, charge, and particle number are conserved by the equations of motion. These laws are consequences of Noether's theorem [Noether 1915]: every continuous symmetry of the action corresponds to a conserved quantity.

A surrogate model trained by minimizing prediction error has no reason to respect these constraints. A reservoir trained to predict fluid velocity fields may produce outputs that violate incompressibility ($\nabla \cdot \mathbf{v} \neq 0$). A reservoir trained on molecular dynamics trajectories may produce energy-nonconservative predictions. These violations are not merely aesthetic: they can cause the surrogate trajectory to drift to physically impossible states, resulting in catastrophic failure of long autonomous rollouts.

The challenge is to train reservoir surrogates that are accurate *and* physically consistent. This section reviews three approaches: soft constraints, hard constraints, and architecture modifications.

## 33.4.2 The Conservation Law as a Linear Constraint

Many physical conservation laws take the form of a linear constraint on the output:

$$
C\,\hat{\mathbf{y}}(t) = \mathbf{b},
$$

where $C \in \mathbb{R}^{k \times d_{\mathrm{out}}}$ is the constraint matrix, $\hat{\mathbf{y}}(t) \in \mathbb{R}^{d_{\mathrm{out}}}$ is the predicted output, and $\mathbf{b} \in \mathbb{R}^k$ is a target vector (often $\mathbf{0}$). Examples:

- **Mass conservation for discretized fluid:** $\sum_i \rho_i V_i = M_{\mathrm{total}}$ (sum of density times volume equals total mass).
- **Divergence-free velocity:** $\sum_j (\partial v_j / \partial x_j) = 0$, discretized as $C\,\mathbf{v} = 0$.
- **Total energy conservation:** $\sum_i E_i = E_{\mathrm{total}}$ (for isolated systems).

When the constraint is linear in the output, it can be incorporated into the readout training directly.

## 33.4.3 Hard Constraints via Constrained Least Squares

**Constrained ridge regression.** The readout weights $W^{\mathrm{out}}$ (viewed as a matrix $\mathbf{W} \in \mathbb{R}^{d_{\mathrm{out}} \times N}$) must satisfy $C\mathbf{W} = \mathbf{0}$ (so that $C\hat{\mathbf{y}} = C\mathbf{W}\mathbf{x} = \mathbf{0}$ for all $\mathbf{x}$). This is an **equality-constrained** least squares problem:

$$
\hat{\mathbf{W}} = \arg\min_{\mathbf{W}:\, C\mathbf{W} = \mathbf{0}} \|\mathbf{X}\mathbf{W}^T - \mathbf{Y}\|_F^2 + \lambda\|\mathbf{W}\|_F^2.
$$

By the method of Lagrange multipliers, the solution is

$$
\hat{\mathbf{W}} = \left(\mathbf{X}^T\mathbf{X} + \lambda\mathbf{I}\right)^{-1}\mathbf{X}^T\mathbf{Y}\,\Pi_C,
$$

where $\Pi_C = \mathbf{I} - C^T(CC^T)^{-1}C$ is the orthogonal projector onto the null space of $C$ [Lawson & Hanson 1974]. The projected readout automatically satisfies the constraint for all inputs.

**Key observation.** The projection $\Pi_C$ can be precomputed once and does not affect the training cost significantly. This makes hard constraint enforcement via null-space projection a computationally cheap technique [Raissi et al. 2019].

## 33.4.4 Soft Constraints via Regularization

An alternative is to add a penalty term to the training loss:

$$
\mathcal{L}_{\mathrm{soft}}(\mathbf{W}) = \frac{1}{T}\|\mathbf{X}\mathbf{W}^T - \mathbf{Y}\|_F^2 + \lambda_1\|\mathbf{W}\|_F^2 + \lambda_2\|C\mathbf{W}\mathbf{X}^T\|_F^2,
$$

where the third term penalizes constraint violations on the training data. The solution is:

$$
\hat{\mathbf{W}} = \mathbf{Y}^T\mathbf{X}\left(\mathbf{X}^T\mathbf{X} + \lambda_1\mathbf{I} + \lambda_2\mathbf{X}C^TC\mathbf{X}^T\right)^{-1}.
$$

Soft constraints are simpler to implement but do not guarantee exact constraint satisfaction. The violation is approximately $O(\lambda_2^{-1})$: larger $\lambda_2$ reduces violation but may increase prediction error.

**Tradeoff.** For scientific applications where the conservation law must hold exactly (e.g., charge conservation in electromagnetic simulations), hard constraints are preferred. For applications where the constraint is approximate (e.g., turbulence closures, where mass conservation holds only in a statistical sense), soft constraints are more flexible.

## 33.4.5 Hamiltonian Reservoir Networks

For conservative mechanical systems, the most natural approach is to modify the reservoir architecture to explicitly preserve a **Hamiltonian**. A Hamiltonian system evolves as:

$$
\dot{q}_i = \frac{\partial H}{\partial p_i}, \qquad \dot{p}_i = -\frac{\partial H}{\partial q_i},
$$

where $H(q, p)$ is the Hamiltonian (total energy), $q_i$ are generalized coordinates, and $p_i$ are conjugate momenta. The phase space flow is symplectic: it preserves the symplectic form $\omega = \sum_i dq_i \wedge dp_i$.

**SympNets [Jin et al. 2020].** SympNets are neural networks designed to exactly preserve the symplectic structure. They decompose the phase space flow into compositions of shear maps:

$$
\Phi_{g, a, b}: (q, p) \mapsto (q + a \odot \tanh(Sp + b),\, p),
$$

where $S$ is a positive definite matrix and $a, b$ are learnable. By composing shear maps alternately in $q$ and $p$, SympNets produce symplectic maps to arbitrary accuracy [Jin et al. 2020].

**Reservoir Hamiltonian architecture.** The SympNet idea can be adapted to reservoir computing: use the reservoir state $\mathbf{x}(t) = (q(t), p(t))$ and require that the autonomous reservoir dynamics (with no input) are symplectic. This is achieved by requiring $W^{\mathrm{rec}}$ to have symplectic structure:

$$
W^{\mathrm{rec}} = J^{-1}\nabla^2 H_{\mathrm{res}},
$$

where $J = \begin{pmatrix} 0 & I \\ -I & 0 \end{pmatrix}$ is the symplectic matrix and $H_{\mathrm{res}}$ is a Hamiltonian function for the reservoir. This is highly restrictive and limits the reservoir's expressiveness, but guarantees exact energy conservation.

## 33.4.6 Lagrangian Neural Networks

Related to Hamiltonian networks are **Lagrangian neural networks** [Cranmer et al. 2020], which parameterize the system dynamics through a learned Lagrangian $\mathcal{L}(q, \dot{q})$ and enforce the Euler-Lagrange equations:

$$
\frac{d}{dt}\frac{\partial \mathcal{L}}{\partial \dot{q}_i} - \frac{\partial \mathcal{L}}{\partial q_i} = F_i^{\mathrm{ext}},
$$

where $F_i^{\mathrm{ext}}$ are external forces. By design, the learned dynamics conserve the energy associated with the learned $\mathcal{L}$. [Greydanus et al. 2019] demonstrated this approach for simple mechanical systems (pendulum, spring-mass); [Cranmer et al. 2020] extended it to multi-body systems.

The connection to reservoir computing: the Lagrangian $\mathcal{L}(q, \dot{q})$ could be computed by a reservoir, with the Euler-Lagrange equations applied as a hard constraint on the reservoir dynamics. This hybrid approach remains largely unexplored.

## 33.4.7 Epistemic Status

**Hard constraints via null-space projection** are well-established and straightforward to implement; this is recommended practice for any reservoir surrogate where the constraint is linear and exact conservation is required.

**Hamiltonian and Lagrangian reservoir architectures** are promising but are an active research area with few large-scale demonstrations as of 2025. The fundamental tension — between the expressiveness needed to represent complex dynamics and the architectural restrictions needed to preserve physical structure — has not been fully resolved. Researchers should approach these methods with appropriate caution and always validate conservation law compliance on held-out test trajectories.

## References

- Cranmer, M., Greydanus, S., Hoyer, S., Battaglia, P., Spergel, D., and Ho, S. (2020). Lagrangian neural networks. *ICLR Workshop on Integration of Deep Neural Models and Differential Equations*.
- Greydanus, S., Dzamba, M., and Yosinski, J. (2019). Hamiltonian neural networks. *NeurIPS*, 32.
- Jin, P., Zhang, Z., Zhu, A., Tang, Y., and Karniadakis, G. E. (2020). SympNets: Intrinsic structure-preserving symplectic networks for identifying Hamiltonian systems. *Neural Networks*, 132, 166–179.
- Lawson, C. L. and Hanson, R. J. (1974). *Solving Least Squares Problems*. Prentice-Hall.
- Noether, E. (1915). Invariante Variationsprobleme. *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 235–257.
- Raissi, M., Perdikaris, P., and Karniadakis, G. E. (2019). Physics-informed neural networks: A deep learning framework for solving forward and inverse problems involving nonlinear partial differential equations. *Journal of Computational Physics*, 378, 686–707.
