# Section 18.2: The Body as Reservoir — Compliant Robots and Morphological Computation

## 18.2.1 Morphological Computation: Conceptual Foundations

The concept of morphological computation arises from a dissatisfaction with the classical separation of body and brain in robotics and cognitive science. In the classical view, the body is a mechanical plant to be controlled; all computation occurs in the controller. Pfeifer and Iida [PfeiferIida2004, PfeiferBongard2006] challenged this separation by documenting a wide range of biological and robotic systems in which the body's physical structure — its geometry, compliance, mass distribution — actively shapes sensorimotor behavior in ways that reduce or eliminate the need for explicit neural computation.

Consider the passive dynamic walkers of McGeer [McGeer1990]: mechanical leg structures that walk stably down inclines with no active control whatsoever. The computation required to maintain stable gait is performed by the mechanical dynamics themselves. Or consider how the compliant tendons of the human hand pre-shape grip posture before conscious motor commands are issued. In both cases, the body solves a computational problem that a purely neural system would need to address explicitly.

Reservoir computing provides the formal language needed to elevate these observations from descriptive case studies to a precise theory. The key insight [HauserEtAl2011] is this: a compliant mechanical body, when driven by motor commands or environmental forces, evolves according to dynamics that map the history of inputs to a high-dimensional state. If those dynamics satisfy the echo state property — if the current state is a function only of the recent input history and not of initial conditions — then the body is a reservoir, and a linear readout trained on the body's state can approximate any fading-memory function of the input history.

## 18.2.2 The Hauser et al. Experiment: Compliant Robot Arm as Reservoir

The experimental demonstration by Hauser, Ijspeert, Füchslin, Pfeifer, and Maass [HauserEtAl2011] is the foundational result in mechanical reservoir computing. We describe it in detail because it is both historically important and pedagogically illuminating.

### Physical Setup

The system is a simulated compliant robot arm consisting of a series of mass-spring segments. The arm has $N$ degrees of freedom corresponding to joint angles $\theta_1, \ldots, \theta_N$. Each joint has an elastic restoring force and viscous damping, so the equations of motion take the general form:

$$M(\boldsymbol{\theta})\ddot{\boldsymbol{\theta}} + C(\boldsymbol{\theta}, \dot{\boldsymbol{\theta}})\dot{\boldsymbol{\theta}} + K\boldsymbol{\theta} + D\dot{\boldsymbol{\theta}} = \boldsymbol{\tau}(t)$$

where $M(\boldsymbol{\theta})$ is the configuration-dependent inertia matrix, $C(\boldsymbol{\theta}, \dot{\boldsymbol{\theta}})$ contains Coriolis and centrifugal terms, $K$ is the stiffness matrix, $D$ is the damping matrix, and $\boldsymbol{\tau}(t)$ is the torque input at the base joint. Crucially, the arm is compliant: the elastic restoring forces are significant, so the arm deforms substantially under forcing, and the deformation state encodes the history of applied forces.

The readout consists of measuring joint angles and angular velocities at all $N$ segments, yielding a $2N$-dimensional state vector $\mathbf{x}(t) = (\theta_1(t), \ldots, \theta_N(t), \dot{\theta}_1(t), \ldots, \dot{\theta}_N(t))$.

### Computational Tasks

Hauser et al. tested the arm on nonlinear, memory-requiring functions of the input signal $u(t)$. A representative task is to approximate:

$$y(t) = f(u(t), u(t-\tau_1), u(t-\tau_2), \ldots)$$

for various nonlinear $f$ and delays $\tau_i$. The output weights $\mathbf{w}_{\text{out}}$ are trained by ridge regression:

$$\mathbf{w}_{\text{out}} = \arg\min_{\mathbf{w}} \sum_t \left(y(t) - \mathbf{w}^\top \mathbf{x}(t)\right)^2 + \lambda \|\mathbf{w}\|^2$$

The key finding: despite the arm having no designed computational structure, the joint state trajectory provides sufficient richness to approximate a wide class of nonlinear functions of the input history. The arm functions as a physical echo state network.

### Formalizing the Echo State Property for Mechanical Systems

Let us state the theoretical result precisely. Define the arm's state evolution as:

$$\mathbf{x}(t) = F_t(u(\cdot); \mathbf{x}_0)$$

where $F_t$ maps the input history $u(\cdot)$ and initial condition $\mathbf{x}_0$ to the state at time $t$. The arm satisfies the echo state property if, for any two initial conditions $\mathbf{x}_0$ and $\mathbf{x}_0'$:

$$\lim_{t \to \infty} \|F_t(u(\cdot); \mathbf{x}_0) - F_t(u(\cdot); \mathbf{x}_0')\| = 0$$

for all bounded input signals $u(\cdot)$.

**Theorem** (Informal, after [HauserEtAl2011]): For a compliant arm with positive-definite damping matrix $D$ and stiffness matrix $K$, and bounded input torques, the echo state property holds. The rate of forgetting initial conditions is governed by the smallest eigenvalue of $D$, denoted $d_{\min}$.

**Proof sketch**: Consider two trajectories $\boldsymbol{\theta}(t)$ and $\boldsymbol{\theta}'(t)$ starting from different initial conditions under the same input $\boldsymbol{\tau}(t)$. Their difference $\boldsymbol{\delta}(t) = \boldsymbol{\theta}(t) - \boldsymbol{\theta}'(t)$ satisfies the linearized equation (in the small-$\delta$ regime):

$$M\ddot{\boldsymbol{\delta}} + (C + D)\dot{\boldsymbol{\delta}} + K\boldsymbol{\delta} \approx \mathbf{0}$$

The Lyapunov function $V = \frac{1}{2}\dot{\boldsymbol{\delta}}^\top M \dot{\boldsymbol{\delta}} + \frac{1}{2}\boldsymbol{\delta}^\top K \boldsymbol{\delta}$ satisfies:

$$\dot{V} = -\dot{\boldsymbol{\delta}}^\top (C + D) \dot{\boldsymbol{\delta}} \leq -d_{\min} \|\dot{\boldsymbol{\delta}}\|^2$$

Under the positive-definiteness of $D$ and mild conditions on $C$, this guarantees exponential decay of $V$, and hence exponential forgetting of initial conditions. $\square$

The timescale of this forgetting, $\tau_{\text{mem}} \sim d_{\min}^{-1}$, characterizes the arm's effective memory horizon and determines which temporal tasks it can support.

## 18.2.3 Stiffness Regimes and the Computational Trade-off

A critical design variable in compliant reservoirs is stiffness. Consider the limiting cases:

**High stiffness** ($K \to \infty$): The arm becomes rigid. Its state at time $t$ depends almost entirely on the instantaneous input $u(t)$, with little memory of past inputs. The reservoir collapses to a memoryless nonlinearity — useful for instantaneous function approximation but unable to perform temporal tasks.

**Low stiffness** ($K \to 0$): The arm becomes a free mass subject only to damping. Memory is long but the state trajectory is nearly linear in the input history. Nonlinear separation of inputs degrades.

**Intermediate stiffness**: A balance between memory depth and nonlinear richness. This is the regime where mechanical reservoirs are most computationally powerful, analogous to the critical-spectral-radius regime in echo state networks (Chapter 5).

This trade-off can be quantified using the linear memory capacity [JaegerMC2002] and the nonlinear capacity [DambreEtAl2012]:

$$\text{MC} = \sum_{k=1}^{\infty} \frac{\text{Cov}^2(\hat{u}(t-k), u(t-k))}{\text{Var}(u(t-k)) \cdot \text{Var}(\hat{u}(t-k))}$$

where $\hat{u}(t-k)$ is the best linear readout approximation to the delayed input $u(t-k)$. Empirically, for compliant arms, MC peaks at intermediate damping-to-stiffness ratios — precisely the parameter regime that Hauser et al. found to give the best task performance.

## 18.2.4 Physical Observables and State Dimensionality

A practical constraint of physical reservoirs is that not all state variables are directly observable. In a compliant arm, we might measure:

- Joint angles $\theta_i(t)$ via encoders
- Tip position $\mathbf{p}_{\text{tip}}(t)$ via vision or tracking
- Bending moments $M_i(t)$ via strain gauges embedded in the links
- Contact forces $\mathbf{F}_c(t)$ via force/torque sensors at the end-effector

Each observable provides a partial view of the full mechanical state. The theoretical question is: how much of the reservoir's computational capacity is accessible from a given set of observables?

The answer depends on the observability of the mechanical system. In control theory, a system is observable if the current state can be uniquely determined from a finite history of observations. For reservoirs, the relevant question is softer: we ask whether the observable state $\mathbf{z}(t) = g(\mathbf{x}(t))$ is rich enough to support linear readout for the task of interest.

Nakajima et al. [NakajimaEtAl2013] conducted systematic studies of observability in soft robotic arms, measuring how performance degrades as the number and placement of strain sensors varies. A key finding: distributed strain sensing along the arm's length provides much richer information than tip-position tracking alone, because bending modes at different spatial frequencies correspond to different temporal frequencies in the arm's dynamical response.

## 18.2.5 Tensegrity Reservoirs

Tensegrity structures [Fuller1962, Skelton2009] consist of rigid compression members (struts) connected by a continuous network of tension members (cables or tendons). Their name is a portmanteau of "tensional integrity." The defining property of tensegrity is that the structure achieves stability through the interplay of tension and compression rather than through rigid joints — no two rigid members touch; they are separated and held in position entirely by the tension network.

From the perspective of reservoir computing, tensegrity structures have several attractive properties:

**1. Distributed nonlinearity.** The tension members are inherently nonlinear: they can carry tension but not compression. Under loading, cables that would go slack become inactive, changing the effective connectivity of the structure. This creates a form of input-dependent connectivity that generates rich nonlinear responses.

**2. High-dimensional state space.** A tensegrity with $n_s$ struts and $n_c$ cables has a state space parameterized by $3n_s$ position variables and $3n_s$ velocity variables (for 3D), plus the cable tensions. Even small tensegrity structures have dozens of state variables.

**3. Physical robustness.** Tensegrity structures are known for their compliance and resilience to damage — desirable properties for a physical reservoir that must operate in an uncontrolled environment.

The dynamics of a tensegrity reservoir under external forcing $\mathbf{f}(t)$ can be written in terms of the nodal positions $\mathbf{q}(t)$ as:

$$M\ddot{\mathbf{q}} + D\dot{\mathbf{q}} = \mathbf{K}(\mathbf{q})\mathbf{q} + \mathbf{f}(t)$$

where $\mathbf{K}(\mathbf{q})$ is the configuration-dependent stiffness matrix. The crucial nonlinearity enters through $\mathbf{K}(\mathbf{q})$: cable elements contribute only when the corresponding cable length exceeds its rest length.

Let $l_i(\mathbf{q})$ denote the length of cable $i$ and $l_i^0$ its rest length. The tension in cable $i$ is:

$$T_i = k_i \max(l_i(\mathbf{q}) - l_i^0, 0)$$

The nonlinear activation function $\max(\cdot, 0)$ is precisely the ReLU function familiar from deep learning — here it arises from the physics of slack cables. The overall restoring force is:

$$\mathbf{f}_{\text{tens}}(\mathbf{q}) = \sum_i T_i \hat{\mathbf{e}}_i(\mathbf{q})$$

where $\hat{\mathbf{e}}_i(\mathbf{q})$ is the unit vector along cable $i$. This is a nonlinear function of $\mathbf{q}$, making tensegrity dynamics genuinely nonlinear even in the absence of contact or friction.

### Tensegrity as an Echo State Network

The structural analogy to an ESN is illuminating. The struts and cables correspond to reservoir neurons; the tension network provides the "recurrent weights." The forcing input $\mathbf{f}(t)$ plays the role of input weights. The readout sensor placement corresponds to the output weights. The echo state property is guaranteed by sufficient damping, just as in the compliant arm case.

One notable feature of tensegrity reservoirs is that the effective "weights" — the cable tensions — change with configuration. This means the reservoir is not time-invariant: its dynamics at time $t$ depend on the current configuration $\mathbf{q}(t)$. This is a departure from the standard ESN model but does not invalidate the reservoir computing framework, provided the configuration-dependence is accounted for in the readout (e.g., by including $\mathbf{q}(t)$ in the observable state).

## 18.2.6 Granular Media as Reservoirs

Granular media — collections of macroscopic particles that interact through contact forces — represent a qualitatively different kind of physical reservoir [NakajimaEtAl2015Granular]. Unlike continuous elastic bodies, granular media are inherently discrete, dissipative (through inelastic collisions), and history-dependent (through force chains and packing geometry).

A canonical granular reservoir consists of a container of sand or glass beads driven by a vibrating plate at the bottom. The surface displacement field $h(\mathbf{r}, t)$ — the height of the granular surface at position $\mathbf{r}$ — serves as the reservoir state. A spatially distributed array of position sensors measures $h(\mathbf{r}_i, t)$ at discrete points $\mathbf{r}_i$.

The physics of granular media is complex and not fully characterized by any closed-form model. However, for reservoir computing purposes, the relevant properties are:

**Nonlinearity**: Surface waves in granular media are highly nonlinear — they form sharp crests, break, and interact through mechanisms that have no linear analogue.

**Memory**: The packing geometry of a granular medium retains information about its forcing history. Force chains — load-bearing networks of particles — reconfigure slowly, giving the medium a long memory.

**Dimensionality**: A container of $10^4$ particles has $\mathcal{O}(10^4)$ degrees of freedom, though the accessible state is limited by the number of surface sensors.

Nakajima, Hauser, Li, and Pfeifer [NakajimaEtAl2015Granular] demonstrated that a vibrated granular medium with 16 surface sensors could perform the NARMA-10 task with normalized mean squared error (NMSE) comparable to software echo state networks of similar size. The result is surprising because granular dynamics are far from the smooth, differentiable dynamics assumed in most reservoir computing theory — suggesting that the theoretical requirements (fading memory, nonlinearity, separation property) can be met by a much wider class of physical systems than initially expected.

### Effective State Space Dimension

A key question for granular reservoirs (and physical reservoirs generally) is: what is the effective dimensionality of the reservoir state as seen by the readout? This can be quantified via the participation ratio:

$$d_{\text{eff}} = \frac{\left(\sum_i \sigma_i^2\right)^2}{\sum_i \sigma_i^4}$$

where $\sigma_i$ are the singular values of the matrix $X = [\mathbf{x}(t_1), \mathbf{x}(t_2), \ldots, \mathbf{x}(t_T)]$. For a $d$-dimensional reservoir with perfectly uniform singular values, $d_{\text{eff}} = d$; for a reservoir whose state lies on a low-dimensional manifold, $d_{\text{eff}} \ll d$.

For granular reservoirs with 16 sensors, Nakajima et al. found $d_{\text{eff}} \approx 8$–$12$, indicating that the sensors accessed a rich but not full-rank state space. Increasing the number of sensors or their spatial density increases $d_{\text{eff}}$ and improves task performance, up to a saturation point determined by the intrinsic dimensionality of the granular dynamics.

## 18.2.7 Design Principles for Mechanical Reservoirs

Drawing together the results of this section, we can state a set of empirically supported design principles for mechanical reservoirs:

**Principle 1: Prefer distributed sensing to point sensing.** The richness of the readout state scales with the spatial coverage of sensors. Tip-position sensing is qualitatively insufficient; distributed strain or displacement sensing is required for high-capacity reservoirs.

**Principle 2: Tune compliance for the target timescale.** The effective memory depth of the reservoir scales with the damping time $\tau_{\text{mem}} \sim d_{\min}^{-1}$. Tasks requiring long temporal context benefit from soft, lightly-damped structures; fast-timescale tasks benefit from stiffer, more damped systems.

**Principle 3: Exploit geometric nonlinearity.** Linear elastic bodies have limited nonlinear capacity. Structures with geometric nonlinearities — tensegrity slack cables, contact mechanics, large deformations — generate richer nonlinear state trajectories and support a broader class of computations.

**Principle 4: Consider the input coupling.** The input signal must couple effectively to the reservoir dynamics. Poor input coupling (e.g., forcing at a node with small displacement) can leave large portions of the state space inaccessible, reducing effective reservoir dimension.

**Principle 5: Validate with capacity measures.** Before attempting complex tasks, characterize the reservoir with memory capacity and nonlinear capacity benchmarks. These provide a rapid diagnostic of whether the physical system is operating in a computationally productive regime.

## 18.2.8 Relationship to Embodied Cognition

It is worth stepping back to appreciate the broader significance of mechanical reservoirs in the context of cognitive science and robotics. The morphological computation framework [PfeiferIida2004] argues that biological intelligence evolved in the context of bodies that actively participated in information processing — that cognition is not "brain in a box" but a whole-body phenomenon.

Reservoir computing provides the first formal theoretical framework for this claim. When a researcher shows that a robot arm's passive mechanics can approximate a nonlinear function of input history, they are not merely demonstrating an engineering trick. They are providing a proof of concept that the classical computational decomposition — body as plant, brain as controller — is not the only possible organization, and perhaps not the most efficient one.

This has implications for the design of artificial systems. A robot designed with a compliant, computationally rich body can potentially achieve behaviors with a simpler controller than would be required for a rigid-body robot. The computational work is distributed between the body and the controller, rather than being entirely centralized. This is the engineering realization of morphological computation.

The reservoir computing framework also suggests a new design methodology: rather than specifying robot morphology to minimize mechanical complexity, one might specify it to maximize computational richness — to create a body whose dynamics, when observed through an appropriate set of sensors, produce a state space well-suited to linear readout for the behaviors of interest. This is an exciting open research direction at the intersection of robotics, materials science, and computational neuroscience.
