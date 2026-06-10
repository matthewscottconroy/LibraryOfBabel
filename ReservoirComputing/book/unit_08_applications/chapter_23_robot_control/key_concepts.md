# Chapter 23: Key Concepts

## Central Pattern Generator (CPG)

A neural circuit in the spinal cord or brainstem that generates rhythmic motor patterns (walking, swimming, breathing) without requiring sensory feedback for each cycle. CPGs consist of coupled oscillators with specific phase relationships. From the reservoir computing perspective, a CPG is a small, specialized reservoir designed to produce periodic outputs. Larger random reservoirs can generalize the CPG concept, producing richer multi-frequency oscillatory dynamics adaptable by a trained readout.

## Reservoir Policy

A reinforcement learning policy in which the observation $s_t$ is encoded by a fixed reservoir into a high-dimensional state $\mathbf{x}(t)$, and the action is a linear function $a_t = W_{\text{out}}\mathbf{x}(t)$. Only $W_{\text{out}}$ is trained. The reservoir provides a nonlinear, temporally extended feature map that enables a simple linear controller to achieve complex behavior. Key advantage: $W_{\text{out}}$ has far fewer parameters than a deep policy network, enabling faster training and easier online adaptation.

## REINFORCE Algorithm

A Monte Carlo policy gradient algorithm that estimates $\nabla_\theta J(\pi_\theta)$ by collecting complete trajectories and computing $\nabla_\theta J \approx \frac{1}{M}\sum_m G_m \sum_t \nabla_\theta \log \pi_\theta(a_t | s_t)$, where $G_m$ is the discounted return of episode $m$. For a reservoir policy with linear Bernoulli readout, the gradient computation is simple and involves only linear operations on the reservoir state $\mathbf{x}(t)$.

## Evolution Strategies (ES)

A black-box optimization algorithm that estimates the gradient of a fitness function by evaluating perturbed parameter vectors: $\theta \leftarrow \theta + \frac{\eta}{\lambda\sigma}\sum_k J(\theta + \sigma\epsilon_k)\epsilon_k$. Preferred for reservoir RL because: no backpropagation required, highly parallelizable, works with non-differentiable rewards, naturally handles the low-dimensional parameter space of reservoir readouts.

## CartPole Benchmark

The canonical RL benchmark: balance a pole on a moving cart by applying left/right forces. State: $(x, \dot{x}, \theta, \dot{\theta})$. Reward: +1 per timestep pole remains upright. Episode ends when pole falls ($|\theta| > 12°$) or cart moves off track ($|x| > 2.4$). Considered "solved" when mean return $\geq 475$ over 20 consecutive episodes. ESN + ES typically solves CartPole in $\sim 5000$–$10000$ environment interactions.

## Hopf Oscillator

A 2D dynamical system with a stable limit cycle: $\dot{x} = (\mu - r^2)x - \omega y$, $\dot{y} = (\mu - r^2)y + \omega x$, where $r = \sqrt{x^2 + y^2}$. The limit cycle has radius $\sqrt{\mu}$ and frequency $\omega/(2\pi)$. The key nonlinear term $-r^2 x$ provides amplitude regulation (small oscillations grow, large ones shrink). Adding a drive signal $s(t)$ causes frequency locking (entrainment) within an Arnold tongue around the natural frequency.

## Arnold Tongue

The region in (drive frequency, coupling strength) parameter space where a forced oscillator locks to the drive frequency. Named for mathematician Vladimir Arnold. For a Hopf oscillator with coupling $\epsilon$, the tongue has width $\propto \epsilon$ centered on the natural frequency $f_0$. In CPG reservoirs, Arnold tongues determine the range of gait frequencies achievable through external drive modulation.

## Gait Frequency

The rate of locomotion cycles per second. Typical ranges: walking (0.5–1.5 Hz), trotting (1.5–3 Hz), galloping (3–6 Hz). Reservoir-based CPGs should have dominant eigenvalue frequencies matching or spanning this range. Spectral engineering — placing complex conjugate eigenvalue pairs at target frequencies — is the primary tool for designing locomotion reservoirs.

## Simulation-to-Real (Sim2Real) Gap

The performance degradation observed when a controller trained in simulation is deployed on a physical robot. Sources include unmodeled dynamics (joint friction, motor dynamics, flexible links), sensor noise, model parameter errors, and contact force inaccuracies. Mitigation strategies: domain randomization (randomizing simulation parameters during training), system identification (fitting simulation to real robot measurements), and online adaptation (updating the readout on the physical robot).

## Domain Randomization

A technique for improving sim2real transfer by training on a distribution of simulation environments rather than a single fixed simulation. Physical parameters (mass, friction, joint stiffness, sensor noise) are sampled randomly for each episode. If the real robot's parameters fall within the training distribution, the policy will generalize to reality. Reservoir policies particularly benefit from domain randomization because only the linear readout needs to generalize — the reservoir's rich representation is robust to parameter variation.

## Online Policy Adaptation

Updating the readout weights $W_{\text{out}}$ of a reservoir policy during deployment using online learning rules (RLS, stochastic gradient). Because $W_{\text{out}}$ is a linear parameter, it can be updated stably and efficiently from each new observation, without the instability risks of online deep network training. This enables reservoir locomotion controllers to adapt to terrain changes, hardware degradation, or payload variation within seconds.
