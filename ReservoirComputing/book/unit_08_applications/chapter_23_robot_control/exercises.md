# Chapter 23 Exercises

## Reinforcement Learning with Reservoirs

**23.1** (Policy Gradient Theory). The REINFORCE update for the ESN policy is:

$$W_{\text{out}} \leftarrow W_{\text{out}} + \eta \frac{1}{M} \sum_{m=1}^M G_m \sum_t (a_t^{(m)} - \sigma(W_{\text{out}}\mathbf{x}_t^{(m)}))\mathbf{x}_t^{(m)\top}$$

(a) Show that this update is an unbiased estimator of $\nabla_{W_{\text{out}}} J(W_{\text{out}})$ for the Bernoulli policy $\pi(a=1 | \mathbf{x}) = \sigma(W_{\text{out}}\mathbf{x})$.

(b) The variance of the REINFORCE gradient estimate is inversely proportional to $M$ (number of episodes). For CartPole, a typical return variance is $\sigma_G^2 \approx 5000$. How many episodes $M$ are needed to estimate the gradient with relative precision $10\%$ (i.e., standard error $\leq 0.1 \|\nabla J\|$)?

(c) Adding a baseline $b$ to the return (replacing $G_m$ with $G_m - b$) reduces variance without biasing the gradient. Show mathematically that the baseline does not affect the gradient estimate (the baseline $b$ must not depend on the action $a_t$).

(d) The optimal baseline is $b^* = \mathbb{E}[G_m \|\nabla \log \pi\|^2] / \mathbb{E}[\|\nabla \log \pi\|^2]$. Implement a simple baseline $b = \text{moving average of } G_m$ and show empirically that it reduces variance on CartPole.

**23.2** (Evolution Strategies Analysis). For the ES update:

$$\theta \leftarrow \theta + \frac{\eta}{\lambda\sigma} \sum_{k=1}^\lambda J_k \epsilon_k$$

(a) Show that $\mathbb{E}_\epsilon\left[\sum_k J(\theta + \sigma\epsilon_k)\epsilon_k\right] = \lambda \sigma \nabla_\theta J(\theta) + O(\sigma^2)$.

(Hint: Taylor expand $J(\theta + \sigma\epsilon_k)$ to second order and use $\mathbb{E}[\epsilon_k] = 0$, $\mathbb{E}[\epsilon_k \epsilon_k^\top] = I$.)

(b) The antithetic sampling trick uses paired perturbations $\{(\epsilon_k, -\epsilon_k)\}_{k=1}^{\lambda/2}$. Show that this reduces variance compared to using $\lambda$ independent samples.

(c) For an ESN with $N = 200$ and $d_a = 1$ (CartPole), compute the number of parameters $|\theta|$. For population size $\lambda = 50$ and one rollout per member, how many environment interactions are required per ES iteration?

**23.3** (CartPole Implementation). Download and run the provided CartPole ESN code.

(a) Run both REINFORCE and ES training. Compare: (i) final performance (mean return over 20 episodes), (ii) sample efficiency (timesteps to reach return = 400), (iii) wall-clock time per iteration.

(b) Ablation study: replace the reservoir with a linear layer ($\mathbf{x}(t) = W_{\text{in}} s_t$, no recurrence). Does this still solve CartPole? What does this reveal about the role of temporal memory in CartPole?

(c) Sweep spectral radius $\rho \in \{0.5, 0.7, 0.9, 0.99, 1.05\}$ and report the ES iterations to convergence for each. Explain the trend.

(d) The CartPole state $s_t = (x, \dot{x}, \theta, \dot{\theta})$ is already low-dimensional and memoryless. Why might a reservoir still help? (Hint: consider what the reservoir provides beyond memory.)

## Central Pattern Generator Exercises

**23.4** (Hopf Oscillator). Simulate a single Hopf oscillator:

$$\dot{x} = (\mu - r^2)x - \omega y, \quad \dot{y} = (\mu - r^2)y + \omega x$$

where $r = \sqrt{x^2 + y^2}$.

(a) Show analytically that the system has a stable limit cycle at radius $r = \sqrt{\mu}$ and frequency $\omega/(2\pi)$.

(b) Simulate with $\mu = 1$, $\omega = 2\pi$ (1 Hz) and initial condition $(x(0), y(0)) = (0.1, 0.0)$. Verify convergence to the limit cycle.

(c) Add an external drive $s(t) = \epsilon\sin(2\pi f_d t)$ with $\epsilon = 0.5$. For $f_d = 1.2$ Hz (slightly above natural frequency), does the oscillator lock to $f_d$? Measure the frequency of $x(t)$ after transient as a function of $f_d \in [0.8, 1.2]$ Hz.

(d) Plot the Arnold tongue: the range of $f_d$ for which locking occurs as a function of $\epsilon \in [0.1, 1.0]$.

**23.5** (Coupled CPG for Bipedal Walking). A simplified bipedal walking CPG has two oscillators (left leg, right leg) with $\pi$-phase offset coupling:

$$\dot{x}_1 = (\mu - r_1^2)x_1 - \omega y_1 + c \cdot x_2$$
$$\dot{y}_1 = (\mu - r_1^2)y_1 + \omega x_1$$
$$\dot{x}_2 = (\mu - r_2^2)x_2 - \omega y_2 + c \cdot x_1$$
$$\dot{y}_2 = (\mu - r_2^2)y_2 + \omega x_2$$

(a) For coupling strength $c = -1$ (anti-phase coupling), show analytically that the steady-state solution has $x_2(t) = -x_1(t)$ (alternating gait).

(b) Simulate with $\mu = 1$, $\omega = 2\pi$, $c = -1$, and random initial conditions. Verify that the oscillators converge to the anti-phase solution.

(c) Change to $c = +1$ (in-phase coupling). What gait does this correspond to biologically? Verify in simulation.

(d) Now replace the two-oscillator CPG with a reservoir of $N = 50$ neurons. Design the reservoir to reproduce both anti-phase and in-phase modes by spectral engineering. How many extra degrees of freedom does the reservoir provide beyond the two-oscillator model?

**23.6** (Spectral Engineering). You want to design a reservoir for a quadruped robot that needs to produce gait patterns at 0.5, 1.0, and 2.0 Hz.

(a) Construct the $6 \times 6$ block-diagonal matrix $B$ with three $2 \times 2$ rotation blocks at these frequencies (use $\Delta t = 0.01$ s). What are the eigenvalues of $B$?

(b) Embed $B$ into a $100 \times 100$ reservoir by placing $B$ as the top-left block and filling the remainder with sparse random weights (scaled to spectral radius 0.9). Simulate the reservoir driven by white noise and verify the three frequency peaks in the power spectrum of the reservoir output.

(c) Train a readout to generate a target signal $y(t) = 0.5\sin(2\pi \cdot 1.0 t) + 0.3\sin(2\pi \cdot 2.0 t)$ (walking + trotting superposition). What is the NMSE?

## Advanced Exercises

**23.7** (Simulation-to-Real Transfer). A reservoir locomotion policy trained in simulation achieves mean return 450 on the simulated environment. When transferred to the physical robot, mean return drops to 280.

(a) Identify three sources of simulation-to-real gap that could explain this performance drop for a legged robot.

(b) Propose a domain randomization strategy: which physical parameters should be randomized during simulation training, and over what ranges? Justify each choice.

(c) Implement a simplified domain randomization in simulation: randomize ground friction $\mu_f \sim \mathcal{U}[0.3, 0.8]$ and link mass $m \sim \mathcal{U}[0.8, 1.2] \times m_{\text{nominal}}$ for each training episode. Does this improve transfer to a "different" simulator with fixed $\mu_f = 0.4$?

**23.8** (Comparison with Deep RL). Compare the CartPole performance of the ESN policy to a standard deep RL baseline.

(a) Implement a small MLP policy (2 hidden layers, 32 units each, $\sim 1200$ parameters) trained with PPO (use `stable-baselines3`).

(b) Compare training curves (return vs. timesteps) for ESN+ES versus MLP+PPO.

(c) Compare the robustness of the two policies to observation noise: add Gaussian noise $\mathcal{N}(0, 0.1)$ to each state observation at test time. Which policy degrades more gracefully?

(d) Compute the inference time per step for each policy. For an embedded controller running at 100 Hz, is either policy computationally prohibitive?
