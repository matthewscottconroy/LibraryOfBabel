# Output Feedback and Generative Reservoirs

## The Output Feedback Architecture

In the standard echo state network, the reservoir state $\mathbf{x}_t$ is driven entirely by the external input $\mathbf{u}_t$. Once trained, the network operates in a filtering mode: given new input, it produces output. Many interesting tasks, however, require the network to generate a target signal autonomously — to produce a time series without any external input driving it. This is the generative mode, and it requires output feedback.

The output feedback update equation is:

$$\mathbf{x}_t = f\!\left(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{u}_t + \mathbf{W}^{\text{fb}} \mathbf{y}_{t-1}\right),$$

where $\mathbf{W}^{\text{fb}} \in \mathbb{R}^{N \times d_{\text{out}}}$ is the feedback weight matrix connecting the readout output $\mathbf{y}_{t-1}$ back into the reservoir [Jaeger & Haas 2004]. The readout remains:

$$\mathbf{y}_t = \mathbf{W}^{\text{out}} \mathbf{x}_t.$$

In autonomous mode ($\mathbf{u}_t = \mathbf{0}$), the combined system

$$\mathbf{x}_t = f\!\left(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{fb}} \mathbf{W}^{\text{out}} \mathbf{x}_{t-1}\right)$$

is a fully recurrent dynamical system with effective recurrent matrix $\mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{W}^{\text{out}}$. The behavior of this closed-loop system is fundamentally different from the open-loop reservoir, and understanding this difference is essential.

## Multiple Attractors from Feedback

Output feedback can create multiple stable attractors in the joint reservoir-plus-readout dynamical system, even when the open-loop reservoir has a single stable fixed point. Consider a scalar readout $y_t = \mathbf{w}^\top \mathbf{x}_t$ with scalar feedback $w^{\text{fb}} y_{t-1}$ added to each reservoir neuron. The closed-loop system has effective recurrence:

$$\mathbf{x}_t = \tanh\!\left((\mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{w}^\top) \mathbf{x}_{t-1}\right).$$

The rank-one matrix $\mathbf{W}^{\text{fb}} \mathbf{w}^\top$ modifies the spectral radius of the effective recurrent matrix. If this perturbation creates eigenvalues with magnitude greater than one, the system bifurcates, potentially producing limit cycles or chaos. This is both a feature — enabling rich autonomous dynamics — and a hazard — the learned readout $\mathbf{W}^{\text{out}}$ may inadvertently destabilize the reservoir [Williams & Zipser 1989].

## Teacher Forcing During Training

During supervised training, the ground truth target $\mathbf{y}_{t-1}^*$ is available. The natural approach is to substitute $\mathbf{y}_{t-1}^*$ for $\mathbf{y}_{t-1}$ in the feedback loop:

$$\mathbf{x}_t = f\!\left(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{u}_t + \mathbf{W}^{\text{fb}} \mathbf{y}_{t-1}^*\right).$$

This is teacher forcing [Williams & Zipser 1989]. It stabilizes the reservoir during training by ensuring the feedback signal is always the true target, preventing error accumulation from corrupting the state trajectory. The reservoir states collected under teacher forcing are then used to solve for $\mathbf{W}^{\text{out}}$ via ridge regression.

## The Autonomous Mode Problem

Teacher forcing creates a fundamental mismatch. During training, the reservoir receives the exact target $\mathbf{y}_{t-1}^*$ as feedback. During autonomous generation, it receives its own output $\mathbf{y}_{t-1} = \mathbf{W}^{\text{out}} \mathbf{x}_{t-1}$, which is imperfect from the first step. Any small deviation $\epsilon_t = \mathbf{y}_t - \mathbf{y}_t^*$ is fed back into the reservoir and amplified, leading to exponentially growing errors. Formally, the linearized error dynamics satisfy:

$$\delta \mathbf{x}_t \approx \mathbf{J}_t (\delta \mathbf{x}_{t-1} + \mathbf{W}^{\text{fb}} \delta \mathbf{y}_{t-1}),$$

where $\mathbf{J}_t = \text{diag}(f'(\cdot)) (\mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{W}^{\text{out}})$ is the Jacobian of the closed-loop system. If any eigenvalue of this Jacobian has magnitude greater than one, errors grow without bound.

This instability is not a minor numerical artifact — it is a structural consequence of training under teacher forcing and evaluating in autonomous mode. The two regimes produce fundamentally different dynamical systems. Jaeger & Haas [2004] demonstrated that output feedback reservoirs trained with teacher forcing cannot reliably generate target signals autonomously, even for relatively simple periodic targets.

## FORCE Learning as the Solution

The resolution is FORCE (First-Order Reduced and Controlled Error) learning, introduced by Sussillo & Abbott [2009] and previewed fully in Chapter 11. The key insight is that the readout weights $\mathbf{W}^{\text{out}}$ must be updated continuously during the training run — not just solved once at the end — so that the error $e(t) = z^*(t) - z(t)$ is kept small throughout. When $e(t) \approx 0$ at all times, the distinction between teacher forcing and autonomous mode vanishes: the network has been trained in a regime that closely approximates autonomous operation.

FORCE achieves this through recursive least squares (RLS) updates to $\mathbf{W}^{\text{out}}$, as detailed in Chapter 11. The stability of autonomous generation after FORCE training depends on properties of the learned $\mathbf{W}^{\text{out}}$ and the closed-loop spectral structure.

## Stability of Autonomous Generation

For autonomous generation to be stable, the target trajectory must be an attracting invariant set of the closed-loop dynamics. A sufficient condition is that the Lyapunov exponents of the closed-loop system are all negative in directions transverse to the target trajectory. This cannot be guaranteed by construction — it must emerge from the learning process. FORCE learning implicitly shapes the closed-loop system to satisfy this condition by driving error to zero throughout training, ensuring the closed-loop trajectory remains near the target manifold [Jaeger & Haas 2004].

---

## References

- Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.
