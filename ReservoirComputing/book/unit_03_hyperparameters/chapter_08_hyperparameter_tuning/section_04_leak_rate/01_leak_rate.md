# Section 8.4: Leak Rate

## 8.4.1 The Leaky Integrator Neuron

The standard ESN update $\mathbf{r}(t) = \tanh(W\mathbf{r}(t-1) + \mathbf{w}^{in} u_t)$ treats each timestep equally — the previous state is completely replaced at each update. But many physical and biological systems integrate their inputs over time rather than resetting at each step. A leaky integrator neuron interpolates between the old state and a new update:

$$\mathbf{r}(t) = (1 - \alpha)\mathbf{r}(t-1) + \alpha \tanh\bigl(W\mathbf{r}(t-1) + \mathbf{w}^{in} u_t\bigr),$$

where $\alpha \in (0, 1]$ is the **leak rate**.

The two extremes are instructive:
- $\alpha = 1$: the standard ESN — the old state is completely discarded at each timestep.
- $\alpha \to 0$: the new input has negligible effect and the reservoir barely moves. The state is essentially frozen (which is useless for computation, but illustrates the extreme).

For intermediate $\alpha$, the reservoir is a weighted average of its old state and the newly computed state, implementing a form of temporal smoothing.

## 8.4.2 The Leaky Integrator as a Low-Pass Filter: Frequency Response Derivation

To understand the leak rate's effect, we derive the frequency response of a linear leaky integrator. Consider the *linear* version (no tanh):

$$r(t) = (1-\alpha)r(t-1) + \alpha(w r(t-1) + w^{in} u_t)$$
$$= (1 - \alpha + \alpha w) r(t-1) + \alpha w^{in} u_t$$
$$= \tilde{w} r(t-1) + \alpha w^{in} u_t,$$

where $\tilde{w} = 1 - \alpha(1 - w)$ is the effective recurrent weight.

Taking the $z$-transform with $R(z) = \mathcal{Z}\{r(t)\}$ and $U(z) = \mathcal{Z}\{u(t)\}$:

$$R(z) = \tilde{w} z^{-1} R(z) + \alpha w^{in} U(z)$$
$$R(z)(1 - \tilde{w} z^{-1}) = \alpha w^{in} U(z)$$
$$H(z) = \frac{R(z)}{U(z)} = \frac{\alpha w^{in}}{1 - \tilde{w} z^{-1}} = \frac{\alpha w^{in} z}{z - \tilde{w}}.$$

The transfer function has a pole at $z = \tilde{w} = 1 - \alpha(1-w)$.

**Frequency response.** Evaluating on the unit circle $z = e^{i\theta}$ (where $\theta \in [-\pi, \pi]$ is the digital frequency in radians per sample):

$$H(e^{i\theta}) = \frac{\alpha w^{in} e^{i\theta}}{e^{i\theta} - \tilde{w}}.$$

The magnitude response is

$$|H(e^{i\theta})| = \frac{\alpha |w^{in}|}{|e^{i\theta} - \tilde{w}|}.$$

For real positive $\tilde{w}$, the denominator $|e^{i\theta} - \tilde{w}|$ is minimized at $\theta = 0$ (DC) and maximized at $\theta = \pi$ (Nyquist). Explicitly:

$$|e^{i\theta} - \tilde{w}|^2 = (1 - \tilde{w})^2 + 2\tilde{w}(1 - \cos\theta) = (1-\tilde{w})^2\left(1 + \frac{2\tilde{w}}{(1-\tilde{w})^2}(1-\cos\theta)\right).$$

**Low-frequency gain ($\theta \approx 0$):**

$$|H(1)| = \frac{\alpha |w^{in}|}{1 - \tilde{w}} = \frac{\alpha |w^{in}|}{\alpha(1-w)} = \frac{|w^{in}|}{1-w},$$

which is the familiar DC gain of a first-order IIR filter.

**High-frequency attenuation ($\theta = \pi$):**

$$|H(-1)| = \frac{\alpha |w^{in}|}{1 + \tilde{w}} = \frac{\alpha |w^{in}|}{2 - \alpha(1-w)}.$$

The ratio of DC gain to Nyquist gain is

$$\frac{|H(1)|}{|H(-1)|} = \frac{2 - \alpha(1-w)}{\alpha(1-w)} = \frac{2}{\alpha(1-w)} - 1.$$

For small $\alpha$ (strong leak), this ratio is large: the filter strongly attenuates high-frequency components relative to low-frequency components. This is the **low-pass filtering** behavior of the leaky integrator.

## 8.4.3 The Effective Time Constant

The pole of $H(z)$ is at $z = \tilde{w} = 1 - \alpha(1-w)$. In continuous-time terms, the discrete-time pole $\tilde{w} = e^{-T_s/\tau}$ where $T_s$ is the sampling period. Setting $\tilde{w} = 1 - \alpha(1-w)$ and using the approximation $e^{-x} \approx 1 - x$ for small $x$:

$$T_s / \tau \approx \alpha(1-w) \implies \tau \approx \frac{T_s}{\alpha(1-w)}.$$

Setting $T_s = 1$ (discrete time):

$$\boxed{\tau_{eff} = \frac{1}{\alpha(1-w)}.}$$

This is the **effective time constant** of the leaky integrator. It has the following properties:

1. **$\alpha \to 0$**: $\tau_{eff} \to \infty$. The neuron integrates indefinitely — its state barely changes. This is a very long time constant.

2. **$\alpha = 1$**: $\tau_{eff} = 1/(1-w)$. This recovers the time constant of the standard (non-leaky) reservoir, set by the recurrent weight $w$.

3. **Large $w$ (close to 1)**: $\tau_{eff} \to \infty$ regardless of $\alpha$. The recurrent connection is doing the integration, not the leak.

The combined time constant when both the leak and recurrent weights contribute is approximately

$$\tau_{eff} = \frac{1}{\alpha(1 - w)}$$

in the scalar case, and in the $N$-neuron case, each mode $\lambda_i$ of $W$ has its own time constant

$$\tau_i = \frac{1}{\alpha(1 - |\lambda_i|)}.$$

For the dominant eigenvalue $\lambda_1 = \rho$:

$$\tau_{dom} = \frac{1}{\alpha(1-\rho)}.$$

This formula connects the leak rate, spectral radius, and the reservoir's dominant timescale.

## 8.4.4 Matching Leak Rate to Input Signal Frequency

The practical goal is to match the reservoir's effective time constant to the relevant timescale of the input signal. For an input signal with characteristic frequency $f_0$ (in units of samples$^{-1}$) or equivalently characteristic period $T_0 = 1/f_0$, the optimal leak rate is approximately

$$\alpha^* \approx \frac{1}{T_0 (1-\rho)} = \frac{f_0}{1-\rho}.$$

**Example:** Suppose the input signal has a characteristic period of $T_0 = 20$ samples (e.g., a sinusoid with period 20) and the reservoir spectral radius is $\rho = 0.9$. The optimal leak rate is approximately

$$\alpha^* \approx \frac{1}{20 \cdot (1-0.9)} = \frac{1}{20 \cdot 0.1} = \frac{1}{2} = 0.5.$$

Setting $\alpha = 0.5$ tunes the reservoir's dominant time constant to match the input period.

**Too large $\alpha$ (fast update):** The reservoir's effective time constant $\tau_{eff}$ is shorter than the input period. The reservoir samples the input faster than necessary, creating redundant states and potentially overrepresenting high-frequency noise.

**Too small $\alpha$ (slow update):** The reservoir's effective time constant is longer than the input period. The reservoir smooths over the signal, losing temporal fine structure. This can be useful for slow, slowly-varying inputs but is harmful for inputs with rapid dynamics.

## 8.4.5 Heterogeneous Leak Rates

One of the most powerful extensions of the leak rate concept is to use *different* leak rates for different neurons. Consider a reservoir with $K$ groups of neurons, where group $k$ has leak rate $\alpha_k$:

$$\mathbf{r}_k(t) = (1-\alpha_k)\mathbf{r}_k(t-1) + \alpha_k \tanh\bigl(W_{kk}\mathbf{r}_k(t-1) + \sum_{l \neq k} W_{kl}\mathbf{r}_l(t-1) + \mathbf{w}^{in}_k u_t\bigr).$$

Different groups integrate the input at different rates, creating a reservoir that simultaneously represents the input at multiple timescales. This is analogous to a multirate filter bank.

**Choosing the leak rates:** For a signal with components at frequencies $f_1, \ldots, f_K$, set $\alpha_k \approx f_k / (1-\rho_k)$ for each group.

**Example:** Audio processing. Speech has components ranging from phoneme-level transitions (timescale $\sim 50$ ms) to syllable rhythm (timescale $\sim 200$ ms) to word rhythm (timescale $\sim 500$ ms). A three-group reservoir with $\alpha_1 = 0.8$ (fast), $\alpha_2 = 0.3$ (medium), $\alpha_3 = 0.05$ (slow) would represent all three timescales simultaneously.

**Theoretical justification:** The frequency response of the multi-rate reservoir is the union of the frequency responses of each group. For group $k$ with leak rate $\alpha_k$ and dominant pole $1 - \alpha_k(1-\rho_k)$, the transfer function $H_k(z)$ has a cutoff frequency approximately

$$f_{c,k} \approx \frac{\alpha_k(1-\rho_k)}{2\pi} \text{ cycles/sample.}$$

A heterogeneous reservoir therefore provides a broad-band representation of the input across multiple frequency channels, rather than the narrow-band representation of a homogeneous reservoir.

## 8.4.6 Analytical Connection to Continuous-Time Reservoirs

The leaky integrator equation

$$\mathbf{r}(t) = (1-\alpha)\mathbf{r}(t-1) + \alpha \tanh(W\mathbf{r}(t-1) + \mathbf{w}^{in} u_t)$$

can be viewed as the Euler discretization of the continuous-time differential equation

$$\dot{\mathbf{r}}(t) = -\mathbf{r}(t) + \tanh(W\mathbf{r}(t) + \mathbf{w}^{in} u(t)),$$

with step size $\Delta t = \alpha$ and time rescaled so that the membrane time constant is 1. (Substituting $\mathbf{r}(t) \approx \mathbf{r}(t-1) + \alpha \dot{\mathbf{r}}(t-1)$ gives the leaky integrator equation.)

This connection makes precise the sense in which $\alpha$ is a timescale parameter: it is the ratio of the integration step to the membrane time constant. When $\alpha = 1$, the discrete-time step equals the time constant, which is a relatively coarse discretization. When $\alpha \ll 1$, the discrete steps are much shorter than the time constant, and the reservoir accurately tracks the continuous-time dynamics.

For continuous-time inputs sampled at rate $f_s$ samples/second with step size $\Delta t = 1/f_s$, the connection to the continuous-time reservoir gives

$$\alpha = \frac{\Delta t}{\tau_m} = \frac{1}{f_s \tau_m},$$

where $\tau_m$ is the membrane time constant in seconds. This allows the reservoir to be designed to match a specific physical timescale.

---

*With the three main hyperparameters analyzed — spectral radius for memory timescale, input scaling for nonlinear character, leak rate for frequency matching — the exercises and lab work in the rest of this chapter will ask you to bring all three together in joint tuning experiments.*
