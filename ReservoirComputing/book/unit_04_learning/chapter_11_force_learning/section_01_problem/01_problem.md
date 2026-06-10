# The Problem FORCE Learning Solves

## Standard ESN: Filtering and Prediction

The canonical echo state network operates in a well-understood mode: a random, fixed reservoir processes an external input stream, and a trained linear readout extracts the relevant output. The reservoir state update is

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{u}_t),$$

and the readout is $z_t = \mathbf{w}^{\text{out} \top} \mathbf{x}_t$. The only learned component is $\mathbf{w}^{\text{out}}$, obtained by ridge regression on collected state-target pairs $\{(\mathbf{x}_t, z_t^*)\}_{t=1}^T$. This works well for filtering, function approximation, and short-horizon prediction tasks, because in all these cases the reservoir is continuously driven by external input that keeps its trajectory on or near the correct path [Jaeger & Haas 2004].

## The Generative Task Requirement

Generative tasks are qualitatively different. The requirement is not to filter or predict, but to produce a target time series $z^*(t)$ autonomously — without any external input to guide the reservoir. After training, the network must operate in closed loop:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{fb}} z_{t-1}), \qquad z_t = \mathbf{w}^{\text{out} \top} \mathbf{x}_t,$$

where $\mathbf{W}^{\text{fb}}$ are feedback weights from the output back to the reservoir. Examples include generating a periodic waveform (sine, square wave), a chaotic attractor trajectory, or a motor control signal. These are not filtering tasks — there is no external input to track, and any error in the output is immediately fed back as corrupted input.

## Why Teacher Forcing Fails for Generative Mode

The naive training approach is teacher forcing: during training, substitute the true target $z^*_{t-1}$ for $z_{t-1}$ in the feedback loop, collect reservoir states, and solve for $\mathbf{w}^{\text{out}}$. The problem is that the resulting $\mathbf{w}^{\text{out}}$ is trained under a dynamical regime it will never see again. In autonomous mode, the first error $e_1 = z^*_1 - z_1$ is fed back, perturbing the reservoir state. The perturbed state produces a slightly different $z_2$, generating error $e_2 > e_1$. The error dynamics are governed by the closed-loop Jacobian:

$$\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_{t-1}} = \text{diag}(\tanh'(\cdot)) \left(\mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}\right).$$

If this Jacobian has spectral radius greater than one, errors grow exponentially. Because $\mathbf{w}^{\text{out}}$ was optimized without regard for the closed-loop stability condition, there is no reason to expect stability, and in practice teacher-forced networks typically fail within a few time steps of autonomous operation [Sussillo & Abbott 2009].

## The Core Requirement

For autonomous generation to succeed, the target trajectory $z^*(t)$ must be an attracting invariant set of the closed-loop dynamics. Specifically, if $\mathbf{x}^*(t)$ denotes the reservoir trajectory when the output exactly matches the target, then the perturbation dynamics

$$\delta \mathbf{x}_t \approx \mathbf{J}_t \delta \mathbf{x}_{t-1} + \text{diag}(\tanh'(\cdot)) \mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top} \delta \mathbf{x}_{t-1}$$

must be contracting. This is a strong condition on $\mathbf{w}^{\text{out}}$ that teacher forcing cannot impose. The closed-loop network is a different dynamical system from the teacher-forced network, and the gap between them is not addressable by offline readout training [Sussillo & Abbott 2009].

## Why This Requires Adapting the Reservoir

An alternative framing: in autonomous mode, the effective recurrent matrix of the closed-loop system is

$$\mathbf{W}_{\text{eff}} = \mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}.$$

The rank-one perturbation $\mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}$ modifies the eigenspectrum of $\mathbf{W}^{\text{rec}}$ in ways that depend on the learned readout. The self-sustaining requirement is that the target trajectory is a stable orbit of the system governed by $\mathbf{W}_{\text{eff}}$. This is a constraint on $\mathbf{w}^{\text{out}}$ that cannot be imposed by simple regression — it requires that $\mathbf{w}^{\text{out}}$ be learned in the closed-loop regime, with feedback active throughout training.

## The Error Signal and RLS Framework

Define the instantaneous output error as

$$e(t) = z^*(t) - z(t) = z^*(t) - \mathbf{w}^{\text{out} \top}(t) \mathbf{x}(t).$$

The goal is to keep $e(t) \approx 0$ at all times during training. Recursive least squares (RLS) provides an online update to $\mathbf{w}^{\text{out}}(t)$ that minimizes the cumulative squared error $\sum_{s=1}^t e(s)^2$. The RLS update is derived from the normal equations: at each time step $t$, update the inverse correlation matrix $\mathbf{P}(t) = (\mathbf{X}^\top \mathbf{X} + \lambda \mathbf{I})^{-1}$ using the Sherman–Morrison formula:

$$\mathbf{P}(t) = \mathbf{P}(t-1) - \frac{\mathbf{P}(t-1) \mathbf{x}(t) \mathbf{x}(t)^\top \mathbf{P}(t-1)}{1 + \mathbf{x}(t)^\top \mathbf{P}(t-1) \mathbf{x}(t)},$$

$$\mathbf{w}^{\text{out}}(t) = \mathbf{w}^{\text{out}}(t-1) + e(t) \mathbf{P}(t) \mathbf{x}(t).$$

This is a $O(N^2)$ per-step update — expensive but tractable. When $e(t) \approx 0$ throughout training, the closed-loop system remains near its target trajectory, and the learned $\mathbf{w}^{\text{out}}$ implicitly satisfies the stability condition required for autonomous generation. This is the FORCE learning algorithm [Sussillo & Abbott 2009].

---

## References

- Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
- Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
