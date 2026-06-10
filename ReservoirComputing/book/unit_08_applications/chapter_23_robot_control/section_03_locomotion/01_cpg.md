# Section 23.3: Central Pattern Generators as Reservoir Oscillators

## 23.3.1 Biological Central Pattern Generators

Central pattern generators (CPGs) are neural circuits in the spinal cord and brainstem of vertebrates that generate the rhythmic, coordinated motor patterns underlying locomotion — walking, running, swimming, flying — without requiring sensory feedback or descending commands for each individual muscle activation [GrillnerWallen1985].

The discovery of CPGs revolutionized motor control theory. It established that the elementary rhythms of locomotion are intrinsic to the spinal cord — a local, peripheral computation — and that the brain need only modulate the CPG's frequency, phase, and amplitude to produce diverse locomotion patterns. The spinal cord does the heavy computational lifting.

From the perspective of reservoir computing, this architecture is deeply familiar: the CPG is a fixed dynamical system (the reservoir) whose state evolves autonomously, and the brain provides a low-dimensional modulation signal (the readout/control) that shapes the CPG output. The CPG's autonomous oscillatory dynamics are exploited rather than designed from scratch — exactly the principle of morphological computation applied to neural circuits.

### CPG Mathematical Model

A canonical CPG model uses coupled oscillators. For a legged robot with $K$ joints, each joint has an associated oscillator pair $(x_i, y_i)$ governing the flexor-extensor muscle activation:

$$\dot{x}_i = \omega_i y_i + x_i(\mu - x_i^2 - y_i^2) + \sum_{j \neq i} w_{ij} x_j + s_i(t)$$
$$\dot{y}_i = -\omega_i x_i + y_i(\mu - x_i^2 - y_i^2) + \sum_{j \neq i} c_{ij} y_j$$

where:
- $\omega_i$ is the intrinsic frequency of oscillator $i$
- $\mu > 0$ is the limit cycle radius squared (amplitude)
- $w_{ij}$, $c_{ij}$ are coupling weights between oscillators
- $s_i(t)$ is an external drive signal

Each oscillator converges to a limit cycle of radius $\sqrt{\mu}$ at frequency $\omega_i / (2\pi)$. The coupling terms synchronize oscillators with specified phase offsets — for example, the classic alternating gait requires $\pi$-phase offset between left and right legs.

### Connection to Reservoir Computing

The CPG network is precisely a small, specialized reservoir: it has autonomous dynamics, its state depends on the history of inputs $s_i(t)$, and the joint motor commands $\theta_i(t)$ are linear functions of the oscillator state $(x_i, y_i)$. The key difference from a standard ESN is that the CPG is designed to have specific oscillatory dynamics (limit cycles), whereas a standard ESN is designed to be near the edge of stability (close to a fixed point).

The reservoir computing framework generalizes this: instead of a small CPG network of $K$ oscillators, use a large reservoir of $N \gg K$ neurons. The reservoir will spontaneously develop oscillatory activity at multiple frequencies (determined by its spectral properties), and the readout can select and combine these oscillations to produce the desired joint trajectories.

This "reservoir as CPG" view [IjspeertEtAl2007, BuccinoEtAl2018] offers two advantages:
1. The reservoir can generate more complex, multi-frequency locomotion patterns than a small CPG model
2. The readout can be trained (rather than hand-designed) to produce specific gait patterns

## 23.3.2 Reservoir Design for Locomotion

### Spectral Engineering

For locomotion tasks, the reservoir should have strong oscillatory components at the target gait frequencies. Typical gait frequencies for legged robots: 0.5–3 Hz for walking, 3–8 Hz for running/trotting.

To engineer a reservoir with oscillatory behavior at frequency $f$, include complex conjugate eigenvalue pairs $\rho e^{\pm j 2\pi f \Delta t}$ in the reservoir weight matrix. This can be achieved by constructing a block-diagonal component:

$$B_k = \rho_k \begin{pmatrix} \cos(2\pi f_k \Delta t) & -\sin(2\pi f_k \Delta t) \\ \sin(2\pi f_k \Delta t) & \cos(2\pi f_k \Delta t) \end{pmatrix}$$

and placing these blocks on the diagonal of $W_{\text{res}}$ (or in a lower-dimensional projection of the reservoir). A reservoir with multiple such blocks at frequencies $f_1, f_2, \ldots, f_M$ will generate oscillatory dynamics at all these frequencies simultaneously.

### Input-Driven Frequency Locking

Alternatively, the CPG frequency can be controlled by an external drive signal $u(t)$. For a Hopf oscillator (each oscillator pair follows Hopf normal form), frequency locking to external input is guaranteed for drive strengths above a threshold [IjspeertEtAl2007]:

$$\dot{x} = (\mu - r^2)x - \omega y + \epsilon u(t)$$
$$\dot{y} = (\mu - r^2)y + \omega x$$

where $r = \sqrt{x^2 + y^2}$ and $\epsilon$ is the drive coupling strength. For $\epsilon > 0$ and $u(t) = \sin(2\pi f_{\text{drive}} t)$, the oscillator locks to $f_{\text{drive}}$ within a range $|f - f_0| < \epsilon/(2\pi)$ (the Arnold tongue).

A large reservoir of such oscillators, driven by a common or distributed input signal, will lock to the input frequency and generate a rich multi-dimensional oscillatory state at that frequency — ideal for locomotion control.

## 23.3.3 Online Adaptation of Locomotion

A key advantage of reservoir-based locomotion over pre-programmed CPGs is online adaptability. If the robot encounters unexpected terrain (soft ground, incline, obstacles), the readout can adapt to maintain stable locomotion.

### Adaptive Readout Training for Locomotion

During locomotion, the robot observes its state $s(t)$ (joint angles, velocities, foot contact forces, IMU data) and receives a reward $r(t)$ proportional to forward velocity minus energy cost:

$$r(t) = v_x(t) - c_E \|a(t)\|^2 - c_{\text{fall}} \mathbb{1}[\text{fall}]$$

The readout weights $W_{\text{out}}$ are updated online by a policy gradient step after each gait cycle:

$$W_{\text{out}} \leftarrow W_{\text{out}} + \eta G \nabla_{W_{\text{out}}} \log \pi$$

where $G$ is the return over the gait cycle. This online update allows the locomotion controller to adapt to changing terrain within a few gait cycles — approximately 2–5 seconds for a walking gait.

### Terrain Adaptation Experiment (Ijspeert et al.)

Ijspeert et al. [IjspeertEtAl2007] demonstrated terrain adaptation in a simulated salamander robot with a reservoir-based CPG controller. The robot transitioned between aquatic swimming and terrestrial walking by changing only the drive signal amplitude $s_i$ to the reservoir — the reservoir dynamics, spectral structure, and readout weights were fixed. The reservoir's multi-frequency oscillatory dynamics naturally accommodated both gaits: the swimming pattern used the low-frequency mode, while the walking pattern engaged higher-frequency modes with different phase relationships.

### Comparing Fixed-CPG and Reservoir Approaches

| Property | Fixed CPG | Reservoir CPG |
|---|---|---|
| Design complexity | High (hand-tuned) | Low (random initialization) |
| Gait diversity | Limited (designed modes) | Rich (all spectral modes) |
| Online adaptation | Manual redesign | Readout update |
| Theoretical guarantees | Strong (limit cycles) | Moderate (echo state property) |
| Simulation-to-real transfer | Easier (explicit model) | Harder (gap in dynamics) |

The reservoir approach trades theoretical guarantees for flexibility and adaptive capacity. For robots deployed in controlled environments, a fixed CPG may be preferable. For robots in complex, changing environments, a reservoir-based CPG provides essential adaptability.

## 23.3.4 Quadruped Locomotion: A Case Study

We outline the reservoir control architecture for a quadruped robot (four-legged walking robot) with 12 actuated joints (3 per leg: hip abduction/adduction, hip flexion/extension, knee flexion).

**Reservoir architecture**:
- $N = 400$ neurons
- Spectral design: 4 dominant frequency pairs at 0.5, 1.0, 2.0, 4.0 Hz (covering walking and trotting)
- Input: normalized joint angles and velocities ($d_{\text{in}} = 24$), plus 4 foot contact signals
- Readout: 12 joint torque commands ($d_{\text{out}} = 12$)

**Training procedure**:
1. Initialize $W_{\text{out}}$ to zero.
2. Run ES (population size 100, 500 iterations) in simulation to optimize $W_{\text{out}}$ for forward walking on flat ground.
3. Transfer to physical robot; run 50 gait cycles of online REINFORCE adaptation to correct for simulation-to-real gap.
4. Test on graded terrain: grass (soft), gravel (irregular), 15° slope.

**Results** (simulation, representative of literature):
- Flat ground: 0.8 m/s walking speed, comparable to fixed CPG
- Irregular terrain: 0.5 m/s, 20% improvement over fixed CPG (adaptive readout)
- Slope: 0.4 m/s (uphill), 0.6 m/s (downhill)
- Recovery from push: 85% success rate (fixed CPG: 60%)

The reservoir's advantage is most pronounced in the terrain adaptation and push-recovery tasks, where the ability to rapidly update $W_{\text{out}}$ is critical.
