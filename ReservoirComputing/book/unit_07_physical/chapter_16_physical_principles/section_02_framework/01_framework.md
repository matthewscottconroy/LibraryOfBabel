# The Physical Reservoir Computing Framework

## General Formulation

A physical reservoir is a driven dynamical system. Its state $\mathbf{x}(t) \in \mathcal{X}$ (which may be continuous or discrete, low or high dimensional) evolves under input forcing $\mathbf{u}(t)$ according to a physical law:

$$\dot{\mathbf{x}}(t) = F(\mathbf{x}(t), \mathbf{u}_{\text{masked}}(t)),$$

where $F$ encodes the physical dynamics (Maxwell's equations for optical systems, Newton's laws for mechanical systems, etc.) and $\mathbf{u}_{\text{masked}}(t)$ is the input signal after preprocessing through a mask. The output is a linear functional of the state, sampled at discrete times [Appeltant et al. 2011].

This general formulation encompasses all physical reservoir computing implementations. The specific choices of physical substrate, masking procedure, readout mechanism, and operating parameters define the implementation.

## Virtual Nodes via Time-Multiplexing

The most important innovation in physical reservoir computing is the time-multiplexing scheme, which allows a single physical degree of freedom to emulate an $N$-node reservoir [Appeltant et al. 2011]. The scheme works as follows.

The input signal $\mathbf{u}_t$ (arriving at discrete time $t$) is held constant over one reservoir period $\tau_R$. This period is divided into $N$ equal sub-intervals of duration $\theta = \tau_R / N$. During the $i$-th sub-interval ($i = 1, \ldots, N$), the input to the physical system is modulated by the $i$-th mask value:

$$u_{\text{masked}}(t') = m_i \cdot u_t, \quad t' \in [t\tau_R + (i-1)\theta, t\tau_R + i\theta),$$

where $m_i \in \mathbb{R}$ is the $i$-th element of the mask vector $\mathbf{m} = [m_1, \ldots, m_N]^\top$.

The physical state at the $N$ sampling times $\{t\tau_R + i\theta\}_{i=1}^N$ constitutes the $N$ virtual node states for input step $t$. These are the analog of the $N$ reservoir neuron states in a simulated ESN [Appeltant et al. 2011].

## The Masking Procedure

The mask $\mathbf{m}$ plays the role of the input weight matrix $\mathbf{W}^{\text{in}}$ in a simulated reservoir. It projects the scalar (or low-dimensional) input into the $N$-dimensional virtual node space. A random binary mask $m_i \in \{-1, +1\}$ is the standard choice; sinusoidal masks and random Gaussian masks have also been used.

Formally, the masking operation for an input $u_t \in \mathbb{R}$ is:

$$\mathbf{m}(t') = m_i \cdot u_t, \quad t' \in [(i-1)\theta, i\theta),$$

where the sub-interval index $i = \lfloor t' / \theta \rfloor + 1$ and the reservoir input interval is normalized to $[0, \tau_R)$. For a $d_{\text{in}}$-dimensional input $\mathbf{u}_t \in \mathbb{R}^{d_{\text{in}}}$, the mask becomes a matrix $\mathbf{M} \in \mathbb{R}^{N \times d_{\text{in}}}$ and the masked input is $\mathbf{M}\mathbf{u}_t$ [Larger et al. 2012].

## The Delay-Feedback Reservoir

The standard physical implementation uses a delay-feedback loop to create internal connectivity between virtual nodes. The feedback loop introduces a delay of exactly $\tau_R$ (one input interval), so the physical state at node $i$ for input $t$ depends on the physical state at node $i$ for input $t-1$ (previous period's same-position state) and on the states at nearby nodes in the current period (through the physical dynamics within $\tau_R$).

The delay-feedback reservoir is governed by:

$$\tau \dot{x}(t) + x(t) = f\!\left(\eta x(t - \tau_R) + \varepsilon m(t) u(t)\right),$$

where $\tau \ll \tau_R$ is the physical response time of the nonlinear node, $\eta$ is the feedback gain, $\varepsilon$ is the input coupling strength, and $f$ is the physical nonlinearity [Appeltant et al. 2011]. The delay $\tau_R$ creates the "ring reservoir" connectivity: each virtual node receives signal from the corresponding node at the previous time step plus coupling from neighboring nodes within the delay period.

## Four Key Parameters

The performance of a delay-feedback physical reservoir is determined primarily by four parameters:

**Mask interval $\theta$:** Controls the coupling between adjacent virtual nodes. Shorter $\theta$ (faster sampling) increases inter-node coupling, effectively increasing the reservoir connectivity.

**Delay $\tau_R$:** Determines the number of virtual nodes $N = \tau_R / \theta$. Longer delay allows more virtual nodes but requires longer physical loops (more fiber, larger feedback circuit).

**Feedback gain $\eta$:** Controls the effective spectral radius of the virtual reservoir. $\eta$ near 1 gives a near-critical reservoir with long effective memory; $\eta < 1$ gives a stable, short-memory reservoir.

**Input coupling $\varepsilon$:** Analogous to the input scaling $\sigma_{\text{in}}$ of a simulated ESN. Small $\varepsilon$ drives the reservoir weakly; large $\varepsilon$ can drive it out of the echo state regime [Larger et al. 2012].

## Readout

After each input period $\tau_R$, the physical state is sampled at $N$ times, yielding the virtual node state vector $\mathbf{x}_t = [x(t\tau_R + \theta), x(t\tau_R + 2\theta), \ldots, x(t\tau_R + N\theta)]^\top$. The readout is a linear combination:

$$y_t = \mathbf{w}^{\text{out} \top} \mathbf{x}_t,$$

trained by ridge regression on offline collected state-target pairs. In hardware implementations, the readout weights are applied using digital-to-analog converters and an analog weighted summing circuit, or digitally after analog-to-digital conversion of the sampled states.

---

## References

- Appeltant, L., Soriano, M. C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., ... & Fischer, I. (2011). Information processing using a single dynamical node as complex artificial neural network. *Nature Communications*, 2(1), 468.
- Larger, L., Soriano, M. C., Brunner, D., Appeltant, L., Gutiérrez, J. M., Pesquera, L., ... & Fischer, I. (2012). Photonic information processing beyond Turing: An optoelectronic implementation of reservoir computing. *Optics Express*, 20(3), 3241–3249.
