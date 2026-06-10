# Chapter 19 Exercises

## Memristive Systems

**19.1** (HP Memristor Model). Consider the HP TiO$_2$ memristor with $R_{\text{ON}} = 100\ \Omega$, $R_{\text{OFF}} = 16\ \text{k}\Omega$, $D = 10$ nm, $\mu_v = 10^{-10}$ cm$^2$/(V·s).

(a) Starting from $w(0) = D/2$, compute $w(t)$ for a constant current $I = 0.1$ mA over $t \in [0, 1]$ s using the linear drift model (no window function). What is $R(w(t))$ at $t = 1$ s?

(b) Add the Strukov window function $f(x) = x(1-x)$ and numerically integrate the state equation. Compare the trajectory $w(t)$ to part (a). At what time does the window function significantly alter the dynamics?

(c) Compute the memristance $M(q)$ as a function of charge $q = \int_0^t I \, dt$ and sketch the $I$–$V$ characteristic (a Lissajous figure) for a sinusoidal current $I(t) = I_0 \sin(2\pi f t)$ with $I_0 = 0.1$ mA and $f = 1$ Hz.

(d) Show that the area enclosed by the $I$–$V$ Lissajous figure is a measure of the energy dissipated by the device and compute this area numerically for the parameters above.

**19.2** (Crossbar Reservoir). A $4 \times 4$ memristive crossbar operates as a reservoir. Each device has initial state $w_{ij}(0) \sim \mathcal{U}[0.2D, 0.8D]$ (uniform random initialization).

(a) Derive the Kirchhoff current law expression for the column currents $\{I_j(t)\}$ as a function of row voltages $\{V_i(t)\}$ and device conductances $\{G_{ij}(t) = 1/R(w_{ij}(t))\}$.

(b) Implement the crossbar numerically using the Joglekar window function with $p = 3$. Apply a random input voltage sequence $V_1(t) \sim \mathcal{N}(0, 0.1)$ V (all other rows held at zero).

(c) Compute the state matrix $X = [\mathbf{I}(t_1), \ldots, \mathbf{I}(t_T)]$ for $T = 1000$ timesteps and evaluate the participation ratio. How many effective dimensions does the 4-row crossbar have?

(d) Train a linear readout to perform NARMA-5 (defined in Appendix E) and report the NMSE. Compare to a random weight ESN of equivalent dimension.

**19.3** (Echo State Property for Memristors). Consider the linear drift memristor (no window function) with state $w(t)$.

(a) Show that two trajectories $w(t)$ and $w'(t)$ starting from different initial conditions $w(0) \neq w'(0)$ under the same current input $I(t)$ satisfy $w(t) - w'(t) = w(0) - w'(0)$ for all $t$. Conclude that the linear drift model does NOT satisfy the echo state property.

(b) Explain physically why this failure occurs — what property of the memristor prevents state convergence?

(c) Propose two modifications to the memristive device model that would restore the echo state property. For each, identify the physical mechanism.

(d) The Joglekar window function with $p \to \infty$ approaches a hard boundary (perfect clipping). Show that in this limit, the memristive reservoir satisfies a form of bounded-input bounded-output stability, and discuss whether this is sufficient for practical reservoir computing.

## Spintronic Systems

**19.4** (LLG Equation). A single-domain magnetic particle with magnetization $\mathbf{m} = (\sin\theta\cos\phi, \sin\theta\sin\phi, \cos\theta)$ is subject to an effective field $\mathbf{H}_{\text{eff}} = H_0 \hat{z}$.

(a) Write the LLG equation in spherical coordinates $(\theta, \phi)$.

(b) Show that in the absence of STT, the magnetization precesses at angular frequency $\omega_0 = \gamma H_0$ and decays to alignment with $\hat{z}$ with characteristic time $\tau_\alpha = 1/(\alpha\omega_0)$.

(c) Add STT with strength $a_J$ (current flowing along $\hat{z}$, polarization along $-\hat{z}$). Find the critical current density $J_c$ at which the STT exactly balances the Gilbert damping.

(d) Above $J_c$, the magnetization undergoes sustained precession. Compute the precession frequency $f_{\text{osc}}$ as a function of $(J - J_c)$ for small deviations from threshold.

**19.5** (STNO as Reservoir Node). A single STNO with delay feedback implements a virtual network of $N = 100$ nodes.

(a) If the STNO oscillates at $f_0 = 500$ MHz and the desired virtual node spacing is $\Delta t = 2$ ns, what feedback delay $\tau$ is required?

(b) The nonlinear frequency shift coefficient is $N_f = -0.5 \times 10^9$ rad/(s·A$^2$). For input amplitude $u = 0.1$ mA, estimate the frequency shift $\Delta f = N_f u^2 / (2\pi)$ in MHz.

(c) Explain qualitatively how the frequency shift creates nonlinear separation of inputs and why this is crucial for the STNO's classification performance.

(d) Implement a simplified STNO model as a Van der Pol oscillator with amplitude-dependent frequency: $\ddot{x} - \mu(1-x^2)\dot{x} + \omega^2(1 + \beta x^2)x = u(t)$, where $\mu = 0.1$, $\omega = 2\pi$, $\beta = 0.1$. Use $N = 50$ time-multiplexed virtual nodes to classify a sinusoidal vs. triangular wave input. Report accuracy.

**19.6** (Skyrmion Dynamics). A single skyrmion in a 2D film of size $L \times L$ satisfies the Thiele equation (neglect the Magnus force for simplicity):

$$\alpha \mathcal{D} \dot{\mathbf{r}} = \mathbf{F}(\mathbf{r}, t)$$

where $\mathcal{D} = 4\pi$ and $\mathbf{F}(\mathbf{r}, t) = F_0 u(t) \hat{x}$ is a current-driven force.

(a) Solve for the skyrmion trajectory $\mathbf{r}(t)$ given $u(t) = A\sin(2\pi f t)$ and initial position $\mathbf{r}(0) = (L/2, L/2)$.

(b) For a 10-skyrmion system with random initial positions, show that the position matrix $X = [\mathbf{r}_1(t), \ldots, \mathbf{r}_{10}(t)]$ forms a reservoir state. What is the dimensionality of this state?

(c) The skyrmion Hall effect causes motion in both $\hat{x}$ and $\hat{y}$ directions under an $\hat{x}$ force. Including the gyrovector term $\mathbf{G} \times \dot{\mathbf{r}}$ with $\mathbf{G} = 4\pi\hat{z}$, recompute the trajectory and show how this coupling enhances the effective nonlinearity.

## Quantum Substrates

**19.7** (Quantum Reservoir Computing — Basics). A quantum reservoir consists of $n = 3$ qubits initially in state $|000\rangle$.

(a) The Hilbert space has dimension $2^n = 8$. Write down a complete basis of states and identify all $8$ basis vectors.

(b) An input encoding maps scalar inputs $u(t) \in [-1,1]$ to rotation angles: $R_x(u(t)\pi/2) \otimes I \otimes I$ applied to qubit 1. What is the state after this encoding?

(c) The reservoir dynamics are governed by a Hamiltonian $H = J\sum_{i<j} \sigma_z^{(i)} \otimes \sigma_z^{(j)} + h\sum_i \sigma_x^{(i)}$ applied for time $\Delta t$. For $J = 1$, $h = 0.5$, $\Delta t = 0.3$, compute the unitary $U = e^{-iH\Delta t}$ (use a numerical matrix exponential).

(d) The readout observes the expectation values $\langle\sigma_z^{(i)}\rangle$ and $\langle\sigma_x^{(i)}\rangle$ for all $i$, giving a 6-dimensional readout vector. Apply the input/evolution/measure procedure for 100 timesteps of a random binary input and compute the participation ratio of the resulting state matrix. Does the quantum system use its full 6-dimensional readout space?

**19.8** (Decoherence and Echo State Property). In an open quantum system, the density matrix $\rho(t)$ evolves under the Lindblad master equation:

$$\frac{d\rho}{dt} = -\frac{i}{\hbar}[H, \rho] + \sum_k \left(L_k \rho L_k^\dagger - \frac{1}{2}L_k^\dagger L_k \rho - \frac{1}{2}\rho L_k^\dagger L_k\right)$$

where $L_k$ are jump operators modeling decoherence.

(a) For a single qubit with dephasing ($L = \sqrt{\Gamma}\sigma_z$), show that the off-diagonal elements of $\rho$ decay as $e^{-\Gamma t}$, while the diagonal elements are preserved.

(b) Argue that decoherence enforces the echo state property for quantum reservoirs: regardless of the initial state $\rho(0)$, the state converges to a unique stationary state $\rho_{\text{ss}}$ for fixed input.

(c) However, strong decoherence also reduces the effective dimensionality of the reservoir state. Explain this trade-off and relate it to the analogous trade-off between memory and nonlinearity in classical reservoirs.

(d) Propose an operating point for a quantum reservoir: what decoherence rate $\Gamma$ (relative to the input rate $1/\Delta t$ and Hamiltonian coupling $J$) would you target, and why?

## Advanced Exercises

**19.9** (Literature Synthesis). The Grollier group's 2020 Nature Electronics paper [GrollierEtAl2020] surveys spintronic neuromorphic computing broadly.

(a) Identify and describe three spintronic phenomena (beyond STNOs) that the authors propose for neuromorphic functions. For each, explain the mapping to a neural network operation (neuron, synapse, etc.).

(b) The authors argue that reservoir computing is particularly suitable for spintronic hardware. Identify the three main reasons they give and evaluate each.

(c) What would be needed to move from the current state of spintronic RC (small experimental demonstrations) to a commercially viable spintronic RC chip? Estimate the required improvements in at least three metrics.

**19.10** (Cross-Platform Design). You need to build a real-time reservoir computing system for processing EEG signals (sampling rate 1 kHz, 64 channels) for brain-computer interface applications. You must choose between a memristive crossbar, an STNO network, and a simulated ESN on a GPU.

(a) Compute the required throughput (multiplications per second) for each platform to process the EEG stream in real time with a reservoir of $N = 500$ nodes.

(b) Estimate the power consumption for each option based on the data given in Sections 19.1–19.2.

(c) Identify two critical requirements of the BCI application (beyond throughput and power) that would influence your platform choice.

(d) Write a one-paragraph recommendation, with justification.
