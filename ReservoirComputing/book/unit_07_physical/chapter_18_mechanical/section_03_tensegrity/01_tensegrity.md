# Tensegrity Reservoirs

## Tensegrity Structure and Mechanics

A tensegrity structure is a spatial framework consisting of a set of disconnected rigid compression members (struts) connected by a continuous network of tensile elements (cables or springs). The structure is in stable equilibrium when all compressive and tensile forces are balanced. Unlike rigid frames, tensegrity structures can deform substantially before failure, storing and releasing elastic energy in the cable network.

Formally, a tensegrity with $n_s$ struts and $n_c$ cables has configuration determined by the position vectors $\{\mathbf{p}_i\}$ of the node endpoints. The equilibrium condition requires that at each node $k$:

$$\sum_{j \in \text{cables}(k)} t_{kj} \hat{\mathbf{e}}_{kj} + \mathbf{F}_k^{\text{ext}} = \mathbf{0},$$

where $t_{kj} \geq 0$ is the tension in cable $kj$ and $\hat{\mathbf{e}}_{kj}$ is the unit vector from $k$ to $j$ [Skelton & de Oliveira 2009]. This tensile equilibrium, combined with the finite stiffness of the cables ($t_{kj} = k_{kj} \max(0, \|l_{kj}\| - l_{kj}^0)$ for rest length $l_{kj}^0$), gives the structure its characteristic nonlinear stiffness: effective stiffness increases with deformation (strain stiffening), providing an input-dependent nonlinearity.

## Nonlinear Dynamics as Reservoir

The nonlinear dynamics of a tensegrity arise from two sources: geometric nonlinearity (stiffness depends on deformation state) and material nonlinearity (cable pretension). Both create input-dependent responses that are appropriate for reservoir computing.

When a tensegrity is subjected to a time-varying forcing $\mathbf{F}^{\text{ext}}(t)$ (from actuators or external loads), the structural response $\mathbf{q}(t)$ (generalized coordinates — node displacements, rotations) satisfies:

$$\mathbf{M}\ddot{\mathbf{q}} + \mathbf{C}\dot{\mathbf{q}} + \mathbf{K}(\mathbf{q})\mathbf{q} = \mathbf{F}^{\text{ext}}(t),$$

where $\mathbf{M}$ is the mass matrix, $\mathbf{C}$ is the damping matrix (viscous damping from material and joint friction), and $\mathbf{K}(\mathbf{q})$ is the nonlinear (configuration-dependent) stiffness matrix. The configuration-dependent stiffness is the source of the nonlinearity required for reservoir computing [Nakajima et al. 2015].

## Vibration Modes as Virtual Nodes

The linearization of the tensegrity dynamics around the rest configuration gives a set of natural vibration modes $\{\boldsymbol{\phi}_i, \omega_i\}$, where $\boldsymbol{\phi}_i$ is the mode shape and $\omega_i$ is the natural frequency. Each mode is analogous to a virtual node in a time-multiplexed physical reservoir, with dynamics:

$$\ddot{q}_i + 2\zeta_i \omega_i \dot{q}_i + \omega_i^2 q_i = \boldsymbol{\phi}_i^\top \mathbf{F}^{\text{ext}}(t),$$

where $\zeta_i$ is the modal damping ratio. The modal response $q_i(t)$ is a filtered version of the input projected onto mode shape $\boldsymbol{\phi}_i$: a bandpass filter centered at $\omega_i$ with bandwidth $2\zeta_i\omega_i$.

Different modes respond to different frequency ranges of the input, providing naturally multi-scale temporal processing. Lower modes (lower $\omega_i$) integrate input over longer timescales; higher modes respond to rapid fluctuations. This is the mechanical analog of the timescale hierarchy in deep reservoirs [Nakajima et al. 2015].

## Nakajima et al. 2015 Demonstration

Nakajima et al. [2015] demonstrated reservoir computing using a 3-strut, 9-cable tensegrity robot (a "6-bar tensegrity sphere"). Three pairs of opposing struts were actuated by pneumatic muscles, providing input. Strain gauges on the cables and accelerometers at the nodes provided 24-dimensional readout.

The task was locomotion pattern generation: given a simple periodic command signal, the reservoir (the robot's body) should compute the complex spatiotemporal activation patterns needed for stable locomotion. Using a linear readout trained offline, the tensegrity reservoir generated stable, coordinated locomotion gaits that the robot could not achieve with the simple command signal alone.

Key metrics: the trained readout achieved $R^2 > 0.95$ on the locomotion pattern generation task, demonstrating high-quality temporal computation from the physical body dynamics.

## Connection to Biological Musculoskeletal System

The biological musculoskeletal system — muscles, tendons, ligaments, and bones — is itself a tensegrity-like structure. Muscles generate tension; bones provide compression; tendons and ligaments are passive tensile elements. The body's compliant dynamics naturally provide a biological analog of the mechanical reservoir.

This biological connection suggests that morphological computation is not merely a biomimetic idea but a genuine computational strategy exploited by nervous systems over evolutionary time [Nakajima et al. 2015]. The cerebellum and spinal cord may compute motor commands that are simplified precisely because the body's mechanics handle much of the required temporal processing.

## Designing for Memory: Soft Materials

Longer mechanical relaxation times directly increase the effective memory of the reservoir. For a modal oscillator with natural frequency $\omega_i$ and damping ratio $\zeta_i$, the effective memory timescale is $\tau_i = 1/(\zeta_i \omega_i)$. Soft elastomers (silicone, polyurethane) have $\zeta_i \sim 0.1$–$0.3$ and low $\omega_i$ (from low stiffness), giving $\tau_i$ of 0.1–1 s — well-matched to robotics timescales. Stiffer structures (steel tensegrity) have higher $\omega_i$ and correspondingly shorter memory [Skelton & de Oliveira 2009].

---

## References

- Nakajima, K., Li, T., Hauser, H., & Pfeifer, R. (2015). Exploiting short-term memory in soft body dynamics as a computational resource. *Journal of the Royal Society Interface*, 12(104), 20141373.
- Skelton, R. E., & de Oliveira, M. C. (2009). *Tensegrity Systems*. Springer.
