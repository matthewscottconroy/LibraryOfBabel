# Reservoir Computing for Motor Control

## Motor Control as a Temporal Computation Problem

Motor control requires mapping desired movement trajectories to the muscle activation patterns (or joint torques) that produce them. This is not a static input-output mapping — it is an intrinsically temporal problem: the torque required at joint $j$ at time $t$ depends on the entire desired trajectory $\theta^*(t'), t' \leq t$ (through inertial dynamics), the current joint state $\theta(t)$ (through feedback), and the history of disturbances (through adaptation). A single-step regression cannot capture these temporal dependencies; a reservoir can.

The standard formulation of motor learning is the inverse dynamics problem: given a desired trajectory $\boldsymbol{\theta}^*(t) \in \mathbb{R}^J$ and its first and second derivatives, compute the required joint torques $\boldsymbol{\tau}(t) \in \mathbb{R}^J$. Newton–Euler dynamics give:

$$\mathbf{M}(\boldsymbol{\theta})\ddot{\boldsymbol{\theta}} + \mathbf{C}(\boldsymbol{\theta}, \dot{\boldsymbol{\theta}})\dot{\boldsymbol{\theta}} + \mathbf{g}(\boldsymbol{\theta}) = \boldsymbol{\tau},$$

where $\mathbf{M}$ is the mass matrix, $\mathbf{C}$ is the Coriolis/centrifugal matrix, and $\mathbf{g}$ is the gravity vector. These are nonlinear functions of the joint state and velocity, making inverse dynamics computation a nonlinear temporal regression task [Shadmehr & Mussa-Ivaldi 1994].

## Reservoir Approach to Inverse Dynamics

The reservoir input for inverse dynamics learning is:

$$\mathbf{u}_t = [\boldsymbol{\theta}^*(t)^\top, \dot{\boldsymbol{\theta}}^*(t)^\top, \ddot{\boldsymbol{\theta}}^*(t)^\top, \boldsymbol{\theta}(t)^\top, \dot{\boldsymbol{\theta}}(t)^\top] \in \mathbb{R}^{5J},$$

combining desired trajectory derivatives (feedforward path) and current state (feedback path). The reservoir state update is:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}}\mathbf{x}_{t-1} + \mathbf{W}^{\text{in}}\mathbf{u}_t),$$

and the output is:

$$\hat{\boldsymbol{\tau}}_t = \mathbf{W}^{\text{out}}\mathbf{x}_t \in \mathbb{R}^J.$$

The readout $\mathbf{W}^{\text{out}}$ is trained offline by ridge regression on demonstrated $({\mathbf{u}}_t, \boldsymbol{\tau}_t^*)$ pairs, where $\boldsymbol{\tau}_t^*$ is the ground-truth torque computed from the Newton–Euler equations during a demonstration.

The reservoir's temporal memory captures the history-dependent terms $\mathbf{C}(\boldsymbol{\theta}, \dot{\boldsymbol{\theta}})$ and the inertial coupling $\mathbf{M}(\boldsymbol{\theta})$, which depend on the trajectory history through their velocity-dependent terms.

## Online Adaptation with RLS

Real robot arms change their dynamics over time (tool attachment, wear, fatigue, payload changes). Online RLS allows the readout to adapt without full retraining:

$$\delta\boldsymbol{\tau}(t) = \boldsymbol{\tau}^*(t) - \hat{\boldsymbol{\tau}}(t) \qquad (\text{error signal})$$

$$\mathbf{W}^{\text{out}}(t) \leftarrow \mathbf{W}^{\text{out}}(t-1) + \frac{\delta\boldsymbol{\tau}(t)}{1 + \mathbf{x}_t^\top\mathbf{P}(t-1)\mathbf{x}_t} \mathbf{P}(t-1)\mathbf{x}_t^\top.$$

The error signal $\delta\boldsymbol{\tau}$ is available if a torque sensor measures actual joint torques (direct feedback) or can be estimated from position error through the Jacobian (indirect feedback).

Shadmehr & Mussa-Ivaldi [1994] demonstrated in human subjects that internal models of arm dynamics are learned over practice trials and can adapt rapidly to new dynamics (force-field perturbations). The reservoir-based model provides a computational implementation of this adaptive internal model.

## Sussillo & Abbott 2009 and Motor Cortex

Sussillo & Abbott [2009] demonstrated that a FORCE-trained recurrent network could generate complex motor trajectories matching those observed in monkey motor cortex during reaching tasks. The motor output $\mathbf{y}(t)$ from the trained network reproduced the correct joint velocity profiles for arm movements to 8 peripheral targets, using a network of 1000 neurons.

This result established that the motor generation problem — producing time-varying joint velocity profiles — can be solved by a recurrent network learning to modify its own attractor through output feedback, without any external timing signal. The network becomes an autonomous generator of motor commands once trained, exactly as a biological CPG generates stereotyped movement patterns without continuous sensory input.

## Physical Reservoir for Robotic Arm: Hauser 2011

Hauser et al. [2011] demonstrated that a compliant silicone robotic arm could serve as a physical reservoir for motor control. Strain gauges at 10 positions along the arm provided the readout. The arm's viscoelastic dynamics naturally implement fading memory: forces applied in the recent past are reflected in the current strain distribution, while older forces have relaxed away.

The trained readout (linear combination of the 10 strain gauge readings) could track desired arm trajectories with RMSE $< 5\%$ of maximum displacement — demonstrating that the body's physical dynamics provide sufficient computational resources for simple trajectory following, consistent with the morphological computation hypothesis [Hauser et al. 2011].

---

## References

- Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
- Hauser, H., Ijspeert, A. J., Füchslin, R. M., Pfeifer, R., & Maass, W. (2011). Towards a theoretical foundation for morphological computation with compliant bodies. *Biological Cybernetics*, 105(5–6), 355–370.
- Shadmehr, R., & Mussa-Ivaldi, F. A. (1994). Adaptive representation of dynamics during learning of a motor task. *Journal of Neuroscience*, 14(5), 3208–3224.
