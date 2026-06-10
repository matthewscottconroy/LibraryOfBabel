# What Makes a Good Physical Reservoir?

## The Four Necessary Properties

Physical reservoir computing replaces the simulated recurrent network with a physical dynamical system — optical, mechanical, electronic, or biological. The computational power of the reservoir derives entirely from the physical system's intrinsic dynamics. For a physical system to function as a reservoir, it must satisfy four necessary properties: nonlinearity, high dimensionality (or its equivalent), fading memory, and separation [Tanaka et al. 2019].

These four properties are not independent design choices; they are interrelated requirements that together enable a physical substrate to serve as a universal functional approximator of input-output histories, as guaranteed by the Boyd–Chua theorem. Each property can be quantified, and each creates specific demands on the physical substrate [Nakajima & Fischer 2021].

## Property 1: Nonlinearity

A reservoir must be nonlinear. Without nonlinearity, the reservoir state is a linear function of the input history:

$$\mathbf{x}_t = \sum_{s=0}^\infty \mathbf{H}_s \mathbf{u}_{t-s},$$

where $\mathbf{H}_s$ are the impulse response matrices. The readout is then also linear in the input history, limiting the system to linear time-invariant filtering. Any task requiring nonlinear transformations of the input history — which includes virtually all interesting tasks — cannot be solved.

More precisely, if the reservoir map $\Phi : \mathbf{u}_{(-\infty, t]} \mapsto \mathbf{x}_t$ is linear, the combined reservoir-readout map is in the class of linear filters, which has Rademacher complexity $O(1/\sqrt{T})$ — far below that of nonlinear function classes. Nonlinearity is what enables the reservoir to compute higher-order statistics of the input history [Nakajima & Fischer 2021].

In physical systems, nonlinearity arises from diverse mechanisms: saturation of optical gain, Hertz contact force between granular particles, the $\tanh$-like response of magnetic materials, Kerr nonlinearity in optical fibers. The specific form of the nonlinearity affects the types of functions the reservoir computes efficiently, but any smooth, bounded nonlinearity suffices in principle.

## Property 2: High Dimensionality

A rich reservoir must project the input history into a high-dimensional feature space. The number of linearly independent features determines the complexity of functions the linear readout can approximate. For a $d_{\text{out}}$-dimensional target function, the minimum reservoir dimension is $d_{\text{out}}$ (trivial bound); in practice, much larger dimensions are needed for robust generalization.

In time-multiplexed physical reservoirs (Section 16.3), the effective dimension is the number of virtual nodes $N$, which can be made large by choosing a small node interval $\theta = \tau_R / N$. A single physical degree of freedom can thus provide arbitrarily many virtual nodes, up to the bandwidth limit of the physical system [Appeltant et al. 2011].

The computational substrate view formalizes this: the reservoir is a kernel machine, and the number of virtual nodes determines the number of random features approximating the kernel. More features give a better kernel approximation, with error $O(N^{-1/2})$ by the Rahimi–Recht theorem.

## Property 3: Fading Memory

The fading memory property requires that the reservoir state $\mathbf{x}_t$ depends on recent input history more strongly than on distant history, with the dependence decaying uniformly:

$$\|\Phi(\mathbf{u}) - \Phi(\tilde{\mathbf{u}})\| \leq C \sum_{s=0}^\infty \mu(s) \|\mathbf{u}_{t-s} - \tilde{\mathbf{u}}_{t-s}\|,$$

where $\mu(s) \to 0$ as $s \to \infty$ [Boyd & Chua 1985]. In physical terms, fading memory requires dissipation: the physical system must lose energy to its environment so that old input information is eventually forgotten.

Without fading memory, the reservoir is either unstable (old inputs are amplified, producing unbounded states) or conservative (old inputs are preserved indefinitely, requiring infinite memory to initialize). Physical dissipation — resistive loss, optical absorption, mechanical friction — provides fading memory automatically, which is one of the practical advantages of physical reservoir computing [Nakajima & Fischer 2021].

## Property 4: Separation

The separation property requires that two distinct input histories $\mathbf{u} \neq \tilde{\mathbf{u}}$ produce distinct reservoir states $\mathbf{x} \neq \tilde{\mathbf{x}}$. Without separation, the reservoir cannot distinguish the two inputs, and the readout cannot compute different outputs for them.

Formally, the map $\Phi$ must be injective (one-to-one) on the class of input signals considered. For practical purposes, this means the reservoir states must be at least as diverse as the input signals. A reservoir with too few virtual nodes, or with states that are nearly identical for all inputs (collapsed dynamics), fails the separation property.

## Physical Substrate Requirements

A physical system suitable for reservoir computing must satisfy practical as well as theoretical requirements: (1) its dynamics must be fast enough to process the input signal at the required rate; (2) it must provide multiple, accessible degrees of freedom as output nodes; (3) it must be controllable — the input coupling must be adjustable; and (4) it must be readable — the state must be measurable without significantly perturbing it.

The "computational substrate" view [Nakajima & Fischer 2021] regards the physical system as a kernel machine whose kernel is determined by the physical dynamics. Different physical substrates compute different kernels, and the task is to match the substrate to the kernel structure of the target function.

---

## References

- Nakajima, K., & Fischer, I. (Eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer.
- Tanaka, G., Yamane, T., Héroux, J. B., Nakane, R., Kanazawa, N., Takeda, S., ... & Hirose, A. (2019). Recent advances in physical reservoir computing: A review. *Neural Networks*, 115, 100–123.
- Appeltant, L., Soriano, M. C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., ... & Fischer, I. (2011). Information processing using a single dynamical node as complex artificial neural network. *Nature Communications*, 2(1), 468.
