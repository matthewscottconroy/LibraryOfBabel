# Neural ODEs and Continuous-Time Reservoirs

## 30.4.1 From Discrete to Continuous Dynamics

Standard ESNs operate in discrete time: the state updates from $\mathbf{x}(t)$ to $\mathbf{x}(t+1)$ in steps. Many physical systems, however, evolve in continuous time, described by differential equations. The natural continuous-time analog of the ESN is the **continuous-time ESN** (CT-ESN) [Jaeger 2001]:

$$
\tau \dot{\mathbf{x}}(t) = -\mathbf{x}(t) + \tanh\!\left(W^{\text{rec}}\mathbf{x}(t) + W^{\text{in}}u(t)\right),
$$

where $\tau > 0$ is the time constant. This is equivalent to the leaky-integrator ESN (Section 5.4 of this textbook) in the continuous-time limit.

The **neural ODE** framework [Chen et al. 2018] generalizes this: instead of a fixed reservoir matrix $W^{\text{rec}}$, the dynamics are parameterized by a neural network that is learned from data.

## 30.4.2 Neural ODEs: The Framework

**Definition 30.1 (Neural ODE [Chen et al. 2018]).** A **neural ODE** is a dynamical system of the form

$$
\frac{d\mathbf{h}(t)}{dt} = f_\theta(\mathbf{h}(t), t), \qquad \mathbf{h}(0) = \mathbf{h}_0,
$$

where $f_\theta: \mathbb{R}^N \times \mathbb{R}_+ \to \mathbb{R}^N$ is a neural network with parameters $\theta$. The solution $\mathbf{h}(T)$ at time $T$ is obtained by integrating with an ODE solver (Runge-Kutta, Dormand-Prince, etc.).

The output is a function of $\mathbf{h}(T)$: $y = g(\mathbf{h}(T))$.

**Key innovation of Chen et al. 2018:** Backpropagation through the ODE solver is possible via the **adjoint method**, which computes gradients without storing intermediate states:

$$
\frac{d\mathcal{L}}{d\theta} = -\int_T^0 \mathbf{a}(t)^T \frac{\partial f_\theta(\mathbf{h}(t), t)}{\partial \theta} \, dt,
$$

where $\mathbf{a}(t) = d\mathcal{L}/d\mathbf{h}(t)$ is the **adjoint variable**, satisfying

$$
\frac{d\mathbf{a}(t)}{dt} = -\mathbf{a}(t)^T \frac{\partial f_\theta(\mathbf{h}(t), t)}{\partial \mathbf{h}}.
$$

The adjoint equation is itself an ODE, solved **backward in time** from $\mathbf{a}(T) = d\mathcal{L}/d\mathbf{h}(T)$. The total memory requirement for computing $d\mathcal{L}/d\theta$ is $O(1)$ in the number of parameters (no checkpointing needed), compared to $O(T)$ for BPTT.

## 30.4.3 The CT-ESN as a Fixed-Parameter Neural ODE

The continuous-time ESN is a neural ODE with **fixed parameters** (no learning in the reservoir):

$$
\frac{d\mathbf{x}(t)}{dt} = \frac{1}{\tau}\left[-\mathbf{x}(t) + \tanh\!\left(W^{\text{rec}}\mathbf{x}(t) + W^{\text{in}}u(t)\right)\right] = f_{W^{\text{rec}}, W^{\text{in}}}(\mathbf{x}(t), u(t)).
$$

Because the reservoir parameters $W^{\text{rec}}, W^{\text{in}}$ are not learned, the adjoint method is not needed — the reservoir is simply integrated forward, and only the readout $y = \mathbf{w}^T\mathbf{x}(T)$ is trained.

**Equivalence.** The CT-ESN is a neural ODE where $f_\theta$ is restricted to the form $f = (-\mathbf{x} + \tanh(W^{\text{rec}}\mathbf{x} + W^{\text{in}}u))/\tau$, with $\theta = (W^{\text{rec}}, W^{\text{in}}, \tau)$ fixed and the only trainable parameters being the readout $\mathbf{w}$.

## 30.4.4 Liquid Neural Networks and Input-Dependent Time Constants

**Liquid Neural Networks (LNNs)** [Hasani et al. 2021] are a variant of neural ODEs inspired by the nervous system of the nematode *C. elegans*. The key innovation is an **input-dependent time constant**:

$$
\frac{d\mathbf{x}(t)}{dt} = -\frac{\mathbf{x}(t)}{\tau(\mathbf{x}(t), u(t))} + f(\mathbf{x}(t), u(t)),
$$

where the effective time constant $\tau(\mathbf{x}, u)$ depends on both the current state and input.

**The LTC (Liquid Time-Constant) neuron.** The specific model proposed by [Hasani et al. 2021] is:

$$
\frac{dx_i}{dt} = -\underbrace{\left(\frac{1}{\tau_i} + f\!\left(\sum_j A_{ij}x_j + \sum_k B_{ik}u_k + \text{bias}_i\right)\right)}_{\text{state-dependent conductance}} x_i + \underbrace{f\!\left(\sum_j A_{ij}x_j + \sum_k B_{ik}u_k + \text{bias}_i\right)}_{\text{input drive}},
$$

where $f$ is the sigmoid function, $A_{ij}$ are synaptic weights, and $B_{ik}$ are input weights. This can be written compactly as:

$$
\dot{x}_i = -x_i/\tau_i(x, u) + g_i(x, u)(1 - x_i),
$$

where $g_i(x,u) = f(\sum_j A_{ij}x_j + \sum_k B_{ik}u_k + b_i)$ is the "gate" and $\tau_i(x,u) = \tau_i/(1 + g_i(x,u))$ is the effective time constant.

**Key property.** The LTC neuron interpolates between a slow integrator (when $g_i \approx 0$, $\tau_{\mathrm{eff}} \approx \tau_i$) and a fast follower (when $g_i \approx 1$, $\tau_{\mathrm{eff}} \approx \tau_i/2$). The dynamics automatically slow down in response to weak inputs and speed up in response to strong inputs — a form of input-gated time constant.

## 30.4.5 Comparison: Neural ODE vs. CT-ESN vs. LNN

| Property | Neural ODE | CT-ESN | Liquid Neural Network |
|---|---|---|---|
| Trainable dynamics | Yes (all parameters) | No (only readout) | Yes (all parameters) |
| Training cost | High (adjoint backprop) | Low (ridge regression) | High (adjoint backprop) |
| Interpretability | Low | Moderate | Low |
| ESP | Depends on $f_\theta$ | Guaranteed if $\|W^{\text{rec}}\|_\mathrm{op} < 1$ | Depends on architecture |
| Time constant | Fixed $\tau$ | Fixed $\tau$ | State/input-dependent $\tau(x,u)$ |
| Universal approx. | Yes (Universal ODE [Chen et al. 2018]) | Yes (Boyd-Chua) | Yes |

## 30.4.6 The Reservoir Perspective on Neural ODEs

From the reservoir computing perspective, a neural ODE is a reservoir with **learned** (not fixed random) internal dynamics. The trade-off:

**Advantages of learned dynamics (neural ODE):** The dynamics can be adapted to the specific task, potentially requiring a smaller state dimension $N$ than a random reservoir.

**Advantages of fixed dynamics (CT-ESN):** No backpropagation through time; no vanishing/exploding gradient issues; training is convex (ridge regression); no risk of learning reservoir dynamics that violate the ESP.

The practical question is: for a given task and training budget, which is more sample-efficient? For tasks where the relevant dynamical features are generic (present in random reservoirs), the CT-ESN wins. For tasks where the dynamics must be precisely tuned (e.g., matching a specific oscillation frequency), the neural ODE wins.

## 30.4.7 Augmented Neural ODEs

[Dupont et al. 2019] introduced **Augmented Neural ODEs (ANODEs)**, which concatenate additional dimensions to the state to allow crossing of trajectories (which standard neural ODEs cannot do, since they define a flow). The augmented state $\tilde{\mathbf{h}} = (\mathbf{h}, \mathbf{a})$ with $\mathbf{a}(0) = \mathbf{0}$ evolves as:

$$
\frac{d}{dt}\begin{pmatrix}\mathbf{h} \\ \mathbf{a}\end{pmatrix} = f_\theta(\mathbf{h}, \mathbf{a}, t).
$$

This is precisely the structure of a reservoir: the augmented dimensions $\mathbf{a}$ play the role of reservoir neurons, initialized to $\mathbf{0}$ (analogous to the washout initialization). The ANODE is therefore a neural ODE with a reservoir-like augmentation.

## References

- Chen, R. T. Q., Rubanova, Y., Bettencourt, J., and Duvenaud, D. (2018). Neural ordinary differential equations. In *Advances in Neural Information Processing Systems*, 31.
- Dupont, E., Doucet, A., and Teh, Y. W. (2019). Augmented neural ODEs. In *Advances in Neural Information Processing Systems*, 32.
- Hasani, R., Lechner, M., Amini, A., Rus, D., and Grosu, R. (2021). Liquid time-constant networks. In *Proceedings of the 35th AAAI Conference on Artificial Intelligence*, 35(9), 7657–7666.
- Jaeger, H. (2001). *The "echo state" approach to analysing and training recurrent neural networks*. GMD Technical Report 148.
