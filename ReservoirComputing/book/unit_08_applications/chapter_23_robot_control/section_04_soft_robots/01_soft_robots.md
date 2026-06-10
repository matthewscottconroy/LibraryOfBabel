# Soft Robotics and Body-as-Reservoir Computing

## The Soft Robotics Paradigm

Soft robots are constructed from compliant, deformable materials — silicone elastomers, hydrogels, textiles, pneumatic bladders — rather than rigid links and discrete joints. This construction enables safe human–robot interaction, adaptable grasping of irregularly shaped objects, and locomotion in unstructured environments. However, it also dramatically complicates the control problem: a soft continuum arm has infinite-dimensional kinematics (the full shape of the arm is the state), compared to the $2J$-dimensional state of a rigid robot with $J$ joints.

Reservoir computing provides a solution to this control complexity through a key observation: the soft body's dynamics are themselves a computation. The deformation of a compliant body under applied forces is a nonlinear, history-dependent mapping of input forces to output strains — exactly the input-output structure of a reservoir. The body computes, and the controller need only learn to read out what the body has computed [Hauser et al. 2011].

## Morphological Computation

Morphological computation, introduced by Pfeifer & Bongard [2006] and formalized for soft robotics by Hauser et al. [2011], asserts that the physical form (morphology) of a robot can perform part of the computation required for behavior, thereby simplifying or eliminating the need for explicit computation by the controller.

For reservoir computing, the specific claim is:

**Thesis:** A compliant robot body driven by an input signal $\mathbf{F}(t)$ (forces, torques, or pneumatic pressures) generates a body state $\mathbf{s}(t)$ (strains, curvatures, contact forces) that constitutes a high-dimensional, nonlinear, history-dependent representation of the input history — i.e., a reservoir state.

The controller then only needs to learn a linear readout on $\mathbf{s}(t)$, offloading all nonlinear and temporal computation to the body.

## Octopus-Inspired Arms as Reservoirs

Nakajima et al. [2013] simulated an octopus arm model as a reservoir for locomotion tasks. The octopus arm is a muscular hydrostat: a three-dimensional structure of longitudinal and transverse muscles embedded in a fluid-filled body. Its deformation dynamics are governed by coupled nonlinear PDE and constitute a high-dimensional physical dynamical system.

In the simulation, the arm model was driven by low-dimensional control inputs (activation levels of proximal/distal muscle groups), and 8 strain-like sensor variables were read out from distributed points along the arm. A linear readout trained on these 8 variables could generate the complex spatiotemporal activation patterns needed for locomotion in an undulatory swimming robot.

Key result: the octopus arm model as reservoir achieved the locomotion task with a linear readout trained in $\sim 1000$ examples — far fewer than required by a direct control approach. The body's dynamics computed the nonlinear temporal features; the readout merely selected the linear combination that produced forward motion [Nakajima et al. 2013].

## Soft Gripper Reservoir

A soft gripper — a compliant finger-like structure that grasps objects by deformation rather than joint actuation — can serve as a reservoir for grasp quality estimation. The deformation state of the fingers under an applied force depends on the shape, stiffness, and contact history of the grasped object. Different objects produce different strain patterns, which can be read out by embedded flexible strain sensors.

The reservoir interpretation: the gripper's strain pattern is a nonlinear, history-dependent encoding of the contact forces and object geometry. A linear readout trained to classify objects from the strain pattern (offline, using labeled examples) can identify objects from novel grasp configurations without any object-specific reprogramming. The gripper "recognizes" what it holds through the embodied computation of its mechanics.

## Design Principles for Body-as-Reservoir

**Maximize distinguishable states per input:** The number of distinct body states distinguishable by the readout sensors determines the effective reservoir dimension. More sensor points, more compliant structures, and richer nonlinear dynamics increase this number. The design objective is to maximize the information processing capacity of the body state, measured by the IPC metric of Dambre et al. [2012].

**Tune compliance for task timescale:** The relaxation time of the soft material should match the timescale of the input signals. Too stiff (short relaxation) and the body forgets inputs too fast, losing relevant history. Too soft (long relaxation) and the body retains irrelevant old inputs, confusing the readout.

**Distribute sensors for linear independence:** Sensor placement should maximize the number of linearly independent readings. Sensors too close together will produce correlated readings; sensors spanning different structural modes (bending, torsion, extension) provide complementary information [Hauser et al. 2011].

## Limits of Morphological Computation

Morphological computation is most effective for tasks where the required computation is matched to the body's natural dynamics — locomotion, grasping, impact absorption. For tasks requiring precise trajectory tracking or rapid switching between behaviors, the body's slow, high-inertia dynamics become a limitation rather than a resource. In these cases, a hybrid approach — physical body as reservoir for coarse control, digital controller for fine correction — is most appropriate [Pfeifer & Bongard 2006].

---

## References

- Nakajima, K., Hauser, H., Kang, R., Guglielmino, E., Caldwell, D. G., & Pfeifer, R. (2013). A soft body as a reservoir: Case studies in a dynamic model of octopus-inspired soft robotic arm. *Frontiers in Computational Neuroscience*, 7, 91.
- Hauser, H., Ijspeert, A. J., Füchslin, R. M., Pfeifer, R., & Maass, W. (2011). Towards a theoretical foundation for morphological computation with compliant bodies. *Biological Cybernetics*, 105(5–6), 355–370.
- Pfeifer, R., & Bongard, J. (2006). *How the Body Shapes the Way We Think*. MIT Press.
