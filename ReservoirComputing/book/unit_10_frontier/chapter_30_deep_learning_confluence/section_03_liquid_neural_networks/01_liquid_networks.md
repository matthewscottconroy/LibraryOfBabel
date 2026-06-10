# Section 30.3: Liquid Neural Networks

## 30.3.1 The Liquid Time-Constant Neuron

The Liquid Neural Network (LNN) paradigm [HasaniLechner2021] begins with a biologically inspired model of neural dynamics. A single neuron in an LNN is governed by the ordinary differential equation:

$$\frac{dx}{dt} = -\frac{x}{\tau} + f(x, I(t), \theta),$$

where $x \in \mathbb{R}$ is the neuron's activation, $\tau > 0$ is a time constant, $I(t)$ is the input current, and $f$ is a nonlinear function of $x$ and $I$. In the simplest form used by Hasani et al.:

$$\frac{dx_i}{dt} = -\frac{x_i}{\tau_i} + \sigma\!\left(\sum_j w_{ij} x_j + w_{\text{in},i} u(t) + b_i\right)(A_i - x_i).$$

Here $\sigma$ is a sigmoid, $A_i$ is a saturation parameter, and the term $(A_i - x_i)$ ensures the dynamics stay bounded (the neuron cannot exceed $A_i$). This is derived from the Hodgkin-Huxley model of conductance-based neural dynamics.

The key innovation: the *effective time constant* of neuron $i$ is:
$$\tau_i^{\text{eff}}(x, u) = \frac{\tau_i}{1 + \tau_i \cdot \sigma(w_i^\top x + w_{\text{in},i} u + b_i)},$$
which *depends on the current state and input*. When the input drives the neuron strongly (large sigmoid value), the effective time constant is small (fast dynamics). When the input is weak, the effective time constant is large (slow dynamics, strong memory). The neuron's memory horizon is *liquid* — it adapts to the input.

## 30.3.2 Liquid Networks as Reservoir Computers with Adaptive Dynamics

From a reservoir computing perspective, an LNN is an input-driven reservoir where the reservoir dynamics themselves depend on the input. Compare:

**Standard ESN**: $\frac{dx}{dt} = -\alpha x + \tanh(Wx + W_{\text{in}}u)$ — the time constant $1/\alpha$ is fixed.

**LNN**: $\frac{dx}{dt} = -x/\tau_{\text{eff}}(x, u) + \text{input-dependent term}$ — the time constant adapts.

This is precisely the selective state space mechanism of Mamba (Section 30.2.6) in continuous time. The LNN can "tune" its memory horizon based on the content of the input:
- High-frequency inputs cause small $\tau_{\text{eff}}$ (fast neurons that track rapid changes).
- Low-frequency inputs cause large $\tau_{\text{eff}}$ (slow neurons that integrate over long periods).

**Theorem 30.3.1 (Informal).** *A Liquid Neural Network with $N$ neurons, driven by a bounded input $u \in L^\infty$, implements a fading-memory functional of the input history. The fading memory rate is adaptive: it is determined by the recent statistics of the input, not by a fixed parameter.*

## 30.3.3 Closed-Form Continuous-Time Networks

A practical limitation of LNNs is that simulating the ODEs requires numerical integration, which is computationally expensive during both training and inference. The Closed-form Continuous-time (CfC) network [HasaniLechner2022] addresses this by deriving an approximate closed-form solution to the LNN ODEs.

**ODE solution.** For the scalar LNN ODE (simplified):
$$\frac{dx}{dt} = -\frac{x}{\tau} + g(t)(A - x),$$
where $g(t) = \sigma(w^\top x(t) + w_{\text{in}} u(t) + b)$ is the input-modulated gate. This is a linear ODE in $x$ once $g(t)$ is viewed as known:
$$\frac{dx}{dt} = -\left(\frac{1}{\tau} + g(t)\right)x + A g(t).$$

The solution is:
$$x(t) = x(t_0) e^{-\int_{t_0}^t (1/\tau + g(s))\, ds} + A\int_{t_0}^t g(s) e^{-\int_s^t (1/\tau + g(r))\, dr}\, ds.$$

**CfC approximation.** In practice, $g(t)$ is not known in closed form (it depends on $x(t)$, which is what we are solving for). The CfC network approximates the solution by treating $g$ as a step function over each time interval $[t_k, t_{k+1}]$:
$$x(t_{k+1}) \approx x(t_k) e^{-(1/\tau + g_k)\Delta t} + A g_k \cdot \frac{1 - e^{-(1/\tau + g_k)\Delta t}}{1/\tau + g_k},$$
where $g_k = \sigma(w^\top x(t_k) + w_{\text{in}} u(t_k) + b)$.

Defining $\phi_k = e^{-(1/\tau + g_k)\Delta t}$ (the decay factor), this becomes:
$$x(t_{k+1}) = \phi_k x(t_k) + (1 - \phi_k) A g_k / g_k \approx \phi_k x(t_k) + (1-\phi_k) A \cdot \text{sigmoid}(\ldots),$$
which looks like a *gated recurrent unit* (GRU) with biologically inspired gating. The gate $\phi_k$ is the continuous-time equivalent of the GRU's update gate.

**Connection to reservoir computing.** A CfC network with fixed $w, w_{\text{in}}$ and trained readout $W_{\text{out}}$ is a reservoir computer. The reservoir state update is the CfC approximation, which is differentiable and efficient. A CfC network with all parameters trained is a trainable reservoir.

## 30.3.4 Empirical Performance and Physical Deployments

Hasani et al. demonstrated that LNNs and CfC networks achieve strong performance with remarkably small models on time-series tasks — particularly autonomous driving from a few dozen neurons. This compactness is attributed to the adaptive time constants: instead of needing many neurons to cover a range of timescales (as in a standard ESN), an LNN can dynamically allocate its time constants to match the task.

**Comparison to standard ESNs.** 
- An ESN with 100 units and a fixed spectral radius covers timescales in a fixed range determined by the eigenvalues.
- An LNN with 100 units adapts its effective timescales to the input content, potentially covering a much wider range.

The trade-off is that LNNs require gradient-based training of all parameters (unlike ESNs, which fix the reservoir), and the adaptive dynamics are harder to analyze theoretically.

## 30.3.5 Interpretability: A Reservoir Computing Perspective

One of the claimed advantages of LNNs is interpretability. Because the ODE is derived from a biological model, individual neurons have interpretable roles: some integrate over long periods (large $\tau_{\text{eff}}$), others respond to rapid transients (small $\tau_{\text{eff}}$).

The reservoir computing framework provides a complementary interpretation: the LNN implements an adaptive fading-memory filter, where the memory horizon adapts to the local statistics of the input. The output of the LNN is a linear combination of these adaptive features — a trained readout, just as in classical reservoir computing.

**Limitation.** The interpretability claimed for LNNs is partly a consequence of the small network sizes (8–19 neurons in the original demonstrations). At larger scale, the interaction effects between neurons with different adaptive time constants become as complex as in any other recurrent network, and simple per-neuron interpretations may not be meaningful.
