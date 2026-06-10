# Chapter 19: Key Concepts

## Memristor

The fourth fundamental passive circuit element (alongside resistor, capacitor, and inductor), predicted by Chua [Chua1971] and physically realized by Strukov et al. [StrukoveEtAl2008]. Defined by the constitutive relation $V = M(q) \cdot I$ where the memristance $M$ depends on the total charge $q$ that has flowed through the device. Physically implemented as a thin-film device whose internal state (dopant profile width $w$) changes with applied current. The state-dependent resistance implements analog, non-volatile memory in the same device that performs computation.

## HP Memristor Equations

The governing equations of the Strukov-Williams memristor:
$$V = R(w) \cdot I, \quad R(w) = R_{\text{ON}}\frac{w}{D} + R_{\text{OFF}}\left(1 - \frac{w}{D}\right), \quad \frac{dw}{dt} = \mu_v \frac{R_{\text{ON}}}{D} I \cdot f_{\text{window}}\left(\frac{w}{D}\right)$$
where $w \in [0, D]$ is the doped-region width, $D$ is the total film thickness, $\mu_v$ is the ion mobility, and $f_{\text{window}}$ is a boundary-enforcing function. The state equation $dw/dt = f(w, I)$ is the general form of a memristive state equation.

## Crossbar Array

A two-dimensional grid of row and column wires with a device at each intersection, allowing parallel matrix-vector multiplication via Kirchhoff's current law: $I_j = \sum_i G_{ij} V_i$. For memristive crossbars, $G_{ij}(t)$ changes with use, implementing a form of in-memory computation. The crossbar is the natural hardware architecture for reservoir computing: input voltages on rows, state evolution in device conductances, output currents as readout features.

## Landau-Lifshitz-Gilbert (LLG) Equation

The equation of motion for a macroscopic magnetic moment $\mathbf{m}$ (unit vector):
$$\frac{d\mathbf{m}}{dt} = -\gamma \mathbf{m} \times \mathbf{H}_{\text{eff}} + \alpha \mathbf{m} \times \frac{d\mathbf{m}}{dt}$$
Describes precession (first term) around the effective field and viscous damping toward equilibrium (second term). Modified by spin-transfer torque to sustain oscillation above a critical current.

## Spin-Transfer Torque (STT)

The transfer of spin angular momentum from a spin-polarized current to a magnetic layer, exerting a torque on the layer's magnetization. Above the critical current $J_c$, the STT term in the LLG equation overcomes Gilbert damping, leading to sustained magnetization precession. The physical mechanism enabling spin-torque nano-oscillators.

## Spin-Torque Nano-Oscillator (STNO)

A nanoscale magnetic device ($\sim$ 100 nm diameter) in which a spin-polarized current drives sustained GHz-frequency magnetization precession. The oscillation frequency is tunable by current and field, and depends nonlinearly on oscillation amplitude (nonlinear frequency shift). STNOs are the primary spintronic platform for reservoir computing, demonstrated by the Grollier group to achieve competitive performance on spoken digit recognition [TorrejonEtAl2017] at nanowatt power consumption.

## Nonlinear Frequency Shift

The dependence of an STNO's oscillation frequency on its oscillation amplitude: $\omega(A) = \omega_0 + N_f A^2$, where $N_f$ is the nonlinear frequency shift coefficient. This nonlinearity is the primary source of computational richness in STNO reservoirs — it creates input-dependent phase trajectories that enable nonlinear classification of temporal patterns.

## Magnetic Skyrmion

A topologically protected spin texture in a 2D magnetic film, characterized by topological charge $Q = \pm 1$. Skyrmions behave as particle-like objects whose positions evolve under applied forces according to the Thiele equation. A collection of skyrmions at positions $\{\mathbf{r}_i(t)\}$ forms a 2D reservoir state; their nucleation/annihilation under threshold currents provides a nonlinear activation mechanism. Skyrmion reservoirs are a theoretical proposal awaiting experimental realization.

## Quantum Reservoir Computing

The use of a quantum system (qubits, photonic modes, etc.) as a reservoir. The Hilbert space of $n$ qubits has dimension $2^n$, offering exponentially large state spaces in principle. Input is encoded via quantum gates; the reservoir dynamics are governed by a Hamiltonian; measurements of quantum observables provide the readout features. The echo state property is enforced by decoherence: open quantum systems converge to a unique stationary state under constant input. Theoretical framework developed by Fujii and Nakajima [FujiiNakajima2017].

## Neuromorphic Computing

The design of computing hardware that mimics the structure and function of biological neural circuits. Reservoir computing is particularly amenable to neuromorphic implementation because the complex (and potentially variable) reservoir dynamics need not be precisely controlled — only the linear readout requires precise computation. Grollier et al. [GrollierEtAl2020] propose spintronics as a natural neuromorphic platform.

## Von Neumann Bottleneck

The performance limitation in conventional computing caused by the separation of memory and processing: data must be fetched from memory, processed by the CPU, and written back. Memristive in-memory computing bypasses this bottleneck by performing computation (matrix-vector multiplication) directly in the memory array, eliminating data movement energy.

## Topological Protection

The stability of topological spin textures (skyrmions) against small perturbations, arising from the integer topological charge $Q$. Topological protection means that a skyrmion cannot be destroyed by small thermal fluctuations — the magnetization configuration must cross an energy barrier to change its topological class. This stability is crucial for reliable skyrmion-based reservoir operation.
