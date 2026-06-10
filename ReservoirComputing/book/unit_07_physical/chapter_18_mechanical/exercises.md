# Chapter 18 Exercises

## Conceptual Exercises

**18.1** (Fading Memory in Mechanical Systems). Consider a damped harmonic oscillator:

$$m\ddot{x} + c\dot{x} + kx = u(t)$$

(a) Show that this system has fading memory by computing the impulse response $g(t)$ and verifying that $\int_0^\infty |g(t)| \, dt < \infty$.

(b) Compute the memory capacity (as defined in [JaegerMC2002]) of this single-degree-of-freedom system as a function of the damping ratio $\zeta = c / (2\sqrt{mk})$.

(c) Show that in the underdamped case ($\zeta < 1$), the memory capacity is larger than in the overdamped case ($\zeta > 1$), and provide a physical interpretation.

**18.2** (Echo State Property for Mechanical Systems). A compliant arm has damping matrix $D = d I$ with scalar damping coefficient $d > 0$ and stiffness matrix $K = k I$ with $k > 0$. Consider two trajectories starting from different initial conditions $\mathbf{x}_0$ and $\mathbf{x}_0'$.

(a) Write the equation of motion for the difference trajectory $\boldsymbol{\delta}(t) = \boldsymbol{\theta}(t) - \boldsymbol{\theta}'(t)$.

(b) Construct a Lyapunov function $V(\boldsymbol{\delta}, \dot{\boldsymbol{\delta}})$ and show that $\dot{V} \leq -\alpha V$ for some $\alpha > 0$ depending on $d$, $k$, and the mass matrix eigenvalues.

(c) Hence bound the time for the echo state property to hold to within $\epsilon$: find $T(\epsilon, d, k)$ such that $\|\boldsymbol{\delta}(t)\| < \epsilon$ for all $t > T$.

**18.3** (Tensegrity Nonlinearity). A 1D tensegrity model consists of two rigid rods connected by three cables. Cable $i$ has rest length $l_i^0$ and spring constant $k_i$. The cables can only pull, not push.

(a) Write the potential energy $V(q)$ of this system as a function of the configuration variable $q$, where the cable tension law is $T_i = k_i \max(l_i(q) - l_i^0, 0)$.

(b) Show that $V(q)$ is piecewise quadratic and identify the "activation boundaries" (configurations where cables go slack).

(c) Explain qualitatively why this piecewise structure enhances the reservoir's nonlinear capacity compared to a purely linear elastic system.

**18.4** (Stiffness Optimization). You are designing a compliant arm reservoir for a target task that requires approximating the function $y(t) = u(t-5) \cdot u(t-3)$ (product of delayed inputs, in discrete time).

(a) Argue that this task requires both nonlinearity and memory of depth at least 5.

(b) Sketch qualitatively how you would tune the arm's stiffness and damping to match these requirements, referencing the trade-offs discussed in Section 18.2.3.

(c) What would happen if the arm were too stiff? Too soft? Support your answers with mathematical arguments.

## Computational Exercises

**18.5** (Simulated Compliant Arm). Implement a 5-segment compliant arm in Python using the Euler-Maruyama integrator. Each segment has mass $m_i = 1$ kg, length $l_i = 0.1$ m, stiffness $k = 50$ N/m, and damping $d = 2$ N·s/m. The input is a sinusoidal torque $\tau(t) = A\sin(2\pi f t)$ applied at the base.

(a) Simulate the arm for $T = 100$ s and record the joint angles and angular velocities at each time step (use $\Delta t = 0.01$ s).

(b) Construct the state matrix $X \in \mathbb{R}^{10 \times T}$ (10 state variables, $T$ time steps).

(c) Compute the participation ratio $d_{\text{eff}}$ and discuss what fraction of the 10 state variables carry useful information.

(d) Train a linear readout to approximate $y(t) = \sin(u(t-2))$ where $u(t) = \tau(t)/A$. Report the NMSE.

**18.6** (Memory Capacity Measurement). Using the arm from Exercise 18.5:

(a) Generate a random binary input $u(t) \in \{-1, +1\}$ sampled i.i.d. at each time step.

(b) For each delay $k = 1, 2, \ldots, 20$, train a linear readout to approximate $u(t-k)$ from the arm state $\mathbf{x}(t)$. Record the squared correlation $r_k^2$ between predicted and true $u(t-k)$.

(c) Compute $\text{MC} = \sum_{k=1}^{20} r_k^2$ and plot $r_k^2$ as a function of $k$.

(d) Repeat for stiffness values $k \in \{5, 50, 500\}$ N/m and compare the resulting memory capacity curves. Identify which stiffness gives the longest memory.

**18.7** (Granular Media Toy Model). A simplified model of a granular reservoir can be constructed using a lattice of coupled nonlinear oscillators with asymmetric restoring forces (modeling the asymmetry between compression and tension in granular contacts).

(a) Implement a 1D chain of 20 oscillators with the Hertzian contact law $F = k \cdot \max(\delta, 0)^{3/2}$, where $\delta$ is the overlap between adjacent oscillators.

(b) Drive the chain from one end with a random input signal and measure the response at all 20 positions.

(c) Evaluate the NARMA-5 task performance and compare to a linear chain (replace Hertzian contacts with linear springs). What does this comparison reveal about the role of nonlinearity?

## Theoretical Exercises

**18.8** (Effective Dimensionality Upper Bound). Let $\mathbf{x}(t) \in \mathbb{R}^N$ be the state of a mechanical reservoir driven by a scalar input $u(t)$.

(a) Show that if the reservoir dynamics are exactly linear, the effective state dimensionality accessible to a linear readout is at most $\min(N, T)$ where $T$ is the length of the training sequence.

(b) Argue that geometric nonlinearity can increase the effective dimensionality beyond $N$ in the sense that a linear readout can access functions of the input history that span a space of dimension greater than $N$ — by viewing the nonlinear state trajectory as living in an infinite-dimensional function space.

(c) Relate this argument to the kernel interpretation of reservoirs discussed in Chapter 6.

**18.9** (Morphological Computation and Controller Simplicity). Let $C(\cdot)$ denote the complexity of a controller needed to achieve a target behavior $\mathcal{B}$.

(a) Formalize "morphological computation" as the statement: for a body with rich dynamics $\mathcal{F}$, $C(\mathcal{B} | \mathcal{F}) < C(\mathcal{B} | \emptyset)$, where the conditioning represents access to the body's state.

(b) In the reservoir computing framework, identify what $C(\mathcal{B} | \mathcal{F})$ corresponds to (Hint: it should involve the readout complexity).

(c) Give an example of a behavior $\mathcal{B}$ for which a compliant arm reduces controller complexity by at least a factor of $N$ (the number of reservoir state variables), and a behavior for which morphological computation provides no benefit.

**18.10** (Literature Review). Read the paper by Nakajima, Hauser, Li, and Pfeifer [NakajimaEtAl2015Granular] on granular media reservoirs.

(a) Identify the key differences between their experimental setup and a simulated echo state network of comparable size.

(b) They report that the granular medium achieves NMSE $\approx 0.15$ on the NARMA-10 task with 16 sensors. How does this compare to standard software ESN results? (Look up benchmark numbers from Chapter 10.)

(c) What are the three main practical challenges they identify for scaling granular reservoirs to larger task sizes? For each challenge, propose a potential solution.

## Advanced Exercises

**18.11** (Soft Robotic Arm Design). You are tasked with designing a soft robotic arm for use as a reservoir to process touch inputs in a prosthetic hand. The inputs are fingertip force patterns; the target output is object material classification (soft/hard/rough/smooth).

(a) Specify the material properties (stiffness, damping, geometry) you would target, justifying each choice in terms of the computational requirements of the task.

(b) Propose a sensor placement strategy (number, location, type of sensors) and justify it in terms of the observability analysis from Section 18.2.4.

(c) Identify at least two ways in which the real-world deployment conditions (temperature variation, mechanical wear, contact with unknown objects) might degrade reservoir performance, and propose mitigation strategies.

**18.12** (Comparison with Artificial ESN). An engineer proposes replacing a soft robotic reservoir with a simulated ESN of comparable dimension, arguing that simulation gives more control over reservoir properties.

Write a one-page response (as a hypothetical technical memo) that:
(a) Acknowledges the valid advantages of simulated ESNs.
(b) Identifies at least three properties of physical reservoirs that are difficult to replicate in simulation.
(c) Proposes a hybrid approach combining simulated and physical components.
