# Section 31.3: NISQ Implementations

## 31.3.1 The NISQ Era

The term *Noisy Intermediate-Scale Quantum* (NISQ) was coined by John Preskill in 2018 [Preskill2018] to describe the current generation of quantum computers: devices with 50-1000 qubits, without full quantum error correction, and with gate error rates of $10^{-3}$–$10^{-2}$. These devices are too noisy for fault-tolerant computation but too large to be efficiently classically simulated.

NISQ devices are the natural experimental platform for quantum reservoir computing, for several reasons:
1. **Noise is a feature, not just a bug**: As argued in Section 31.2.6, decoherence can implement fading memory. NISQ noise contributes to (rather than purely degrading) the reservoir dynamics.
2. **Only linear readout is needed**: Quantum RC requires only measuring observables and training a linear model — no complicated quantum algorithms.
3. **Short circuits suffice**: Reservoir computing does not require deep circuits (unlike quantum simulation or Shor's algorithm). The reservoir can operate with circuit depth $O(1)$ per time step.

## 31.3.2 Available Hardware Platforms

**IBM Quantum.** IBM's quantum network provides cloud access to devices with 5–433 qubits. The flagship device as of 2024 is IBM Heron (133 qubits, heavy-hex topology, gate error $\approx 0.1\%$ for two-qubit gates). The IBM ecosystem (Qiskit SDK) is the most widely used in reservoir computing experiments.

*Typical reservoir configuration*: 5–10 qubits, with parameterized rotations as the input encoding and Clifford circuits as the reservoir dynamics. Readout consists of measuring Pauli $Z$ operators on each qubit.

**Google Quantum AI.** Google's Sycamore processor (53–72 qubits, all-to-all coupling within groups) demonstrated "quantum supremacy" in 2019 for random circuit sampling. The Willow processor (2024, 105 qubits) reduced error rates significantly.

*Reservoir computing use*: Google devices have been used to demonstrate quantum advantage claims in random circuit sampling, but direct reservoir computing experiments have been less extensive than on IBM devices. The high connectivity of Sycamore makes it potentially attractive for reservoir dynamics.

**IonQ.** Ion trap quantum computers (IonQ Aria, IonQ Forte) with 11–35 "algorithmic" qubits. Key advantage: all-to-all connectivity (any two qubits can interact directly) and lower gate error rates ($\sim 0.05\%$ for two-qubit gates). Key disadvantage: slower gate times ($\sim 10\mu$s vs. $\sim 10$ns for superconducting qubits).

*Reservoir computing relevance*: All-to-all connectivity allows denser reservoir interaction graphs, potentially richer dynamics. Lower error rates mean less unwanted decoherence.

**Neutral atom arrays.** Devices from QuEra, Pasqal, and others: 256+ atoms in configurable 2D arrays, with programmable Rydberg interactions. The spatial configurability allows direct implementation of 2D reservoir topologies. Currently less developed for quantum computing but rapidly advancing.

**Photonic quantum systems.** Systems from PsiQuantum, Xanadu (Borealis): 216+ squeezed modes. Photonic systems naturally implement time-multiplexed quantum reservoirs (the quantum analogue of the optical delay-line reservoir discussed in Chapter 12). No decoherence in transmission (photons don't interact with the environment much), but measurement is probabilistic and hard.

## 31.3.3 A Concrete NISQ Reservoir Computing Experiment

We describe a prototypical NISQ reservoir computing experiment to make the discussion concrete.

**Task**: Predict the next value of a Mackey-Glass time series (a standard benchmark for temporal processing).

**Reservoir setup** (following [Ghosh2021]):
- $N = 4$ qubits on an IBM device.
- Input encoding: At each time step $t$, apply a single-qubit rotation $R_y(\theta u(t))$ to each qubit, where $\theta$ is a scaling parameter and $u(t)$ is the input value.
- Reservoir dynamics: Apply a random but fixed entangling circuit (a sequence of CNOT gates and random single-qubit rotations). This circuit is run once per time step after the input encoding.
- Readout: Measure the expectation values $\langle Z_i \rangle$ for $i = 1, \ldots, 4$ and $\langle Z_i Z_j \rangle$ for $i < j$ (6 two-qubit correlators), giving $4 + 6 = 10$ features.
- Output: Linear combination of features, trained by ridge regression on 500 time steps.

**Results** (typical for such experiments):
- NMSE $\approx 10^{-2}$ on Mackey-Glass prediction.
- A classical ESN with 10 units achieves comparable or better NMSE with less training overhead.
- The advantage of the quantum reservoir (if any) lies in compactness: 4 qubits generate 10 features; 10 classical neurons generate 10 features, but the quantum features may have different (richer) structure.

**Assessment.** The honest assessment of such experiments: they demonstrate that quantum reservoir computing is feasible on NISQ hardware and achieves results comparable to similarly-sized classical reservoirs. They do not demonstrate quantum advantage in any rigorous sense.

## 31.3.4 Quantum Reservoir Capacity and the NISQ Bottleneck

**Theoretical capacity vs. practical capacity.** A 4-qubit quantum reservoir has a Hilbert space of dimension $2^4 = 16$, with density matrix dimension $16 \times 16 = 256$ (as a real matrix). The theoretical number of independent observables is $4^4 - 1 = 255$. However, on a NISQ device:
- Measuring 255 observables requires $O(255 \times M)$ shots (measurements), which is expensive.
- Noise corrupts the reservoir state, effectively reducing the rank of $\rho$ toward the maximally mixed state $I/16$.
- Circuit depth is limited by coherence times, restricting the reservoir dynamics to shallow circuits.

**Effective capacity.** In practice, NISQ experiments use $O(N)$ or $O(N^2)$ observables (not the full $4^N$), giving a practical capacity comparable to a classical reservoir of size $N^2 \sim$ (number of qubits)$^2$. This is a polynomial improvement, not exponential.

## 31.3.5 Current Results and Limitations

**What NISQ quantum reservoirs have demonstrated (as of 2025)**:
1. Feasibility on IBM, Google, and IonQ devices for small-scale temporal tasks [NakajimaTanaka2021, WintermantelDa2023].
2. That quantum features (measuring multi-qubit correlators) can be richer than classical features of the same dimension, on specific synthetic tasks [FujiiNakajima2017].
3. That the decoherence rate of NISQ devices affects performance in a non-monotone way: some decoherence helps (fading memory), too much hurts [MatsubaraNakajima2022].

**What NISQ quantum reservoirs have NOT demonstrated**:
1. Quantum advantage over best classical baselines at any scale.
2. Scalability beyond $O(10-20)$ qubits for reservoir computing tasks.
3. Benefit from entanglement specifically (vs. classical random circuits of the same depth).

**The scaling problem.** For useful quantum reservoir computing, one needs $N$ qubits with $N$ large enough that classical simulation is infeasible ($N \gtrsim 50$) while maintaining low enough noise that the reservoir dynamics are meaningful. Current devices have either too few low-error qubits ($\leq 20$) or too many noisy qubits ($50-433$ but with error rates too high for deep circuits). This gap may close as hardware improves, but it is a real limitation.

**Prospects.** The most likely near-term path to practical quantum reservoir computing advantage is via:
1. Quantum sensors and transducers: processing quantum signals that are inherently quantum (e.g., quantum state discrimination, quantum channel estimation).
2. Analog quantum simulation: using quantum hardware to efficiently simulate classical reservoirs with specific (quantum-inspired) dynamics that are hard to simulate classically.
3. Photonic time-multiplexed quantum reservoirs: exploiting temporal modes of single photons for reservoir states, where the non-destructive Kerr interaction provides nonlinearity.

The longer-term path — fault-tolerant quantum reservoir computing with demonstrated exponential advantage — requires quantum error correction, larger device sizes, and a compelling task where the exponential Hilbert space dimension is genuinely necessary. This remains an important open problem.
