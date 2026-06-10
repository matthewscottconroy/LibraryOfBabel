# Chapter 16 — Key Concepts

---

## Four Conditions for Physical Reservoir Computing

The minimal requirements that a physical system must satisfy to function as a reservoir: (1) **Nonlinearity** — response must be nonlinear; (2) **High dimensionality** — many accessible state degrees of freedom; (3) **Fading memory** — influence of past inputs must decay; (4) **Separation** — distinguishably different states for computationally different input sequences. All known physical reservoir substrates satisfy these conditions to varying degrees; the quality of a reservoir depends on how well each condition is met.

---

## Time-Multiplexing

The technique by which a single physical node with delayed feedback emulates a large reservoir. The delay interval $\tau$ is divided into $N = \tau/\theta$ sub-intervals, each corresponding to one "virtual node." By measuring the physical state at each sub-interval position, $N$ simultaneous state values are obtained from a single physical node. The technique was introduced by Appeltant et al. [Appeltant2011] and is the basis for most single-node photonic and optoelectronic reservoir computers.

---

## Virtual Nodes

The $N$ effective neurons created by time-multiplexing a single physical node. Virtual node $k$ at time step $n$ is the physical state at time $t = n\tau + k\theta$. The virtual nodes are coupled through the feedback delay: node $k$ at step $n$ depends on node $k-1$ at step $n$ (within the same delay interval) and node $N-1$ at step $n-1$ (through the previous cycle's last state). This creates the ring topology of connections.

---

## Mask

The sequence $\mathbf{m} = (m_0, m_1, \ldots, m_{N-1})$ that multiplies the input signal for each virtual node sub-interval. The mask ensures that each virtual node receives a differently weighted version of the current input, creating diversity among virtual nodes even though they are all driven by the same physical node. Typical mask values are $\pm 1$ (random binary).

---

## Delay-Line Reservoir / Ring Reservoir

An ESN with a specific cyclic connection topology: node $k$ connects to node $k+1$ (mod $N$). The Appeltant single-node system is physically equivalent to this architecture. Rodan and Tino [Rodan2011] showed that simple ring topologies can be competitive with fully random reservoirs on standard benchmarks.

---

## NARMA-10 Benchmark

The Nonlinear AutoRegressive Moving Average benchmark of order 10:
$$y_{t+1} = 0.3 y_t + 0.05 y_t \sum_{i=0}^{9} y_{t-i} + 1.5 u_{t-9} u_t + 0.1$$
with input $u_t \sim \text{Uniform}(0, 0.5)$. Tests temporal memory of depth 10 and quadratic nonlinear computation. The standard synthetic benchmark for comparing physical and digital reservoirs.

---

## Santa Fe Laser Benchmark

One-step-ahead prediction of the chaotic time series from a NH₃ laser (Santa Fe Time Series Competition, Dataset A, 1991). A real experimental dataset (1000 training + 500 test points) that tests chaotic time series prediction. Performance is measured by NRMSE.

---

## Spoken Digit Recognition Benchmark

Classification of isolated spoken digits (0–9) from 5 speakers in the TI-46 corpus. Each audio sample is preprocessed (e.g., Lyon cochleagram or MFCCs) and fed to the reservoir as a multi-channel time series. The final reservoir state (or a time-averaged state) is classified by a linear readout. Tests multi-channel temporal integration over 50–100 ms.

---

## Channel Equalization Benchmark

Recovery of 4-level transmitted symbols from a nonlinearly distorted, noisy received signal (Jaeger-Haas model). The channel has memory depth 7 symbols and a cubic nonlinearity. Tests nonlinear memory and robust classification at various signal-to-noise ratios.

---

## NRMSE (Normalized Root Mean Squared Error)

The primary performance metric for regression tasks:
$$\text{NRMSE} = \frac{\text{RMSE}}{\text{std}(y_{target})}$$
A value of 1.0 is achieved by a constant predictor (no skill). Values below 0.1 indicate good performance; values below 0.05 indicate excellent performance for typical RC benchmarks.

---

## Edge of Stability (Physical Context)

In physical reservoir computing, the operating regime where the system is dissipative enough to have fading memory (not oscillating) but sensitive enough to separate different input sequences. Corresponds to loop gain just below the oscillation threshold in optoelectronic systems. Analogy to the ESN operating near spectral radius $\rho \approx 1$.
