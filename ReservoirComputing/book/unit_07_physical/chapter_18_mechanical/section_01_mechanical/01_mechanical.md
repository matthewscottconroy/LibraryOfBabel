# Mechanical Reservoir Computing

## The Body as a Computational Resource

Mechanical reservoir computing arises from a simple but profound observation: physical bodies have dynamics. A silicone arm deforms under force, vibrates, and relaxes — these transient dynamics are nonlinear functions of the input force history. If the arm's state can be read out at multiple points, those readings constitute a high-dimensional state vector that can serve as a reservoir representation.

This idea is systematized under the label of **morphological computation**: the body's physical dynamics perform part of the computation required for behavior, offloading it from the nervous system or controller [Hauser et al. 2011]. A robot with a compliant body can use its body's dynamics to compute functions of the input force history, reserving the central controller for high-level planning.

The concept is related to but distinct from physical reservoir computing in photonics. Mechanical systems operate at much lower speeds (milliseconds to seconds vs. nanoseconds for optics), making them suitable for robotics, haptics, and locomotion tasks rather than communications or signal processing [Nakajima et al. 2013].

## Hauser et al. 2011: Nonlinear Function Approximation

Hauser et al. [2011] demonstrated reservoir computing using a silicone rubber arm with embedded sensors. The arm was a compliant cylinder of silicone with 10 strain gauges placed at different positions along its length. Input was a force applied at one end; output was the readings from all 10 strain gauges over time.

The key result was that the 10-dimensional strain gauge state vector was sufficient to compute several nonlinear functions of the input force history — including functions requiring memory of past forces — using a linear readout trained by ridge regression. The arm served as the reservoir; no random recurrent network was simulated. The arm's intrinsic compliance, viscoelasticity, and geometric nonlinearity provided the required nonlinearity and fading memory.

Formally, if $F(t)$ is the input force and $s_i(t)$ is the strain at sensor $i$, the reservoir state is $\mathbf{x}(t) = [s_1(t), \ldots, s_{10}(t)]^\top$, and the output is $y(t) = \mathbf{w}^{\text{out} \top} \mathbf{x}(t)$ with $\mathbf{w}^{\text{out}}$ trained offline [Hauser et al. 2011].

## Tensegrity Structures

Tensegrity (tensional integrity) structures consist of rigid compression members (rods) connected by tensile members (cables or springs) in a configuration that is stable under tension. The restoring forces are entirely tensile, giving these structures their characteristic combination of rigidity and flexibility. Tensegrity structures exhibit rich nonlinear dynamics: as the structure deforms, the tension in cables changes, altering the effective stiffness — a form of geometric nonlinearity.

Nakajima et al. [2015] demonstrated that a tensegrity robot could serve as a reservoir for locomotion pattern generation. The robot's body (rods and elastic cables) was driven by actuators, and accelerometers at multiple points provided the readout signal. The body dynamics computed the required temporal patterns for locomotion with a simple linear readout trained offline [Nakajima et al. 2015].

## Wave-Based Reservoirs

Standing waves in elastic media provide another class of mechanical reservoir nodes. A rectangular elastic plate, excited by a force at one point, develops a superposition of standing wave modes. Each mode corresponds to a different spatial pattern and temporal frequency. The displacement at any point is a sum of mode contributions, each decaying at a different rate (depending on material damping). The displacement field at $M$ measurement points constitutes an $M$-dimensional reservoir state.

The richness of the reservoir depends on the number of distinguishable modes excited by the input. For a thin elastic plate of dimensions $L_x \times L_y$, the resonant frequencies are approximately:

$$f_{mn} = \frac{\pi}{2}\sqrt{\frac{D}{\rho h}}\left(\frac{m^2}{L_x^2} + \frac{n^2}{L_y^2}\right),$$

where $D$ is the plate flexural stiffness, $\rho$ is density, $h$ is thickness, and $m, n$ are mode integers. For a steel plate with $L_x = L_y = 0.1$ m, the first 20 modes are distributed between $\sim 100$ Hz and $\sim 10$ kHz, covering acoustic frequencies relevant to audio processing [Nakajima et al. 2013].

## Readout Requirements

For mechanical systems, the readout requires physical sensors distributed across the mechanical structure: strain gauges, accelerometers, pressure sensors, or cameras for full-field displacement measurement. The number of independent sensor readings determines the effective reservoir dimension.

Increasing the number of sensors improves the reservoir's representational capacity. However, sensors add mass, change the mechanical properties, and require wiring — all design constraints. The trade-off between sensor count and mechanical interference is a key engineering challenge for mechanical reservoir computing [Hauser et al. 2011].

---

## References

- Hauser, H., Ijspeert, A. J., Füchslin, R. M., Pfeifer, R., & Maass, W. (2011). Towards a theoretical foundation for morphological computation with compliant bodies. *Biological Cybernetics*, 105(5–6), 355–370.
- Nakajima, K., Hauser, H., Kang, R., Guglielmino, E., Caldwell, D. G., & Pfeifer, R. (2013). A soft body as a reservoir: Case studies in a dynamic model of octopus-inspired soft robotic arm. *Frontiers in Computational Neuroscience*, 7, 91.
- Nakajima, K., Li, T., Hauser, H., & Pfeifer, R. (2015). Exploiting short-term memory in soft body dynamics as a computational resource. *Journal of the Royal Society Interface*, 12(104), 20141373.
