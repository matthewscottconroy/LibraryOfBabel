# 16.3.1 Standard Benchmarks for Physical Reservoir Computing

## The Role of Benchmarks

Comparing physical reservoir computers is inherently difficult: a photonic system and a mechanical system run on different physical timescales, have different noise levels, and require different input/output interfaces. Without standard benchmarks, claims of "good performance" are not comparable across systems.

The physical RC community has converged on a small set of standard benchmarks that are:
1. Well-defined mathematically
2. Of graduated difficulty
3. Representative of different computational capabilities (memory, nonlinearity, temporal structure)
4. Expressible in both analog and digital settings

We give precise mathematical definitions of each below.

## Benchmark 1: NARMA-10

**Task:** Nonlinear AutoRegressive Moving Average of order 10. This is a synthetic time series task designed to test temporal processing capability.

**Equations:**

$$y_{t+1} = \alpha y_t + \beta y_t \sum_{i=0}^{9} y_{t-i} + \gamma u_{t-9} u_t + \delta$$

with standard parameters $\alpha = 0.3$, $\beta = 0.05$, $\gamma = 1.5$, $\delta = 0.1$.

The input $u_t$ is drawn i.i.d. from $\text{Uniform}(0, 0.5)$.

**Memory structure:** The output depends on the previous 10 outputs and 10 inputs. Successfully predicting $y_{t+1}$ from $u_t$ requires memory depth of at least 10 time steps and the ability to compute products of state values.

**Performance metric:** Normalized Root Mean Squared Error (NRMSE):

$$\text{NRMSE} = \frac{\sqrt{\frac{1}{T_{test}}\sum_{t=1}^{T_{test}}(y_t - \hat{y}_t)^2}}{\text{std}(y)}$$

where $\text{std}(y)$ is the standard deviation of the test targets.

**Typical results:** A good ESN (N=100) achieves NRMSE $\approx 0.04$–$0.15$ depending on hyperparameters. A random baseline (constant prediction) gives NRMSE $= 1.0$. Physical reservoirs achieving NRMSE $< 0.2$ are considered competitive.

**Limitation:** NARMA-10 is a purely synthetic task. Good performance does not guarantee good performance on real-world tasks.

## Benchmark 2: Santa Fe Laser Dataset

**Task:** One-step-ahead prediction of a chaotic laser time series.

**Background:** The Santa Fe Time Series Competition (1991) included a dataset of intensity measurements from a NH₃ laser operating in a chaotic regime [Hübner1994]. The competition data, known as Dataset A, consists of 1000 training points and 500 test points of scalar laser intensity measurements, sampled at a fixed interval. The raw data is available at the Santa Fe Institute competition website.

**Definition:** Predict $u_{t+1}$ from $\{u_t, u_{t-1}, \ldots\}$, where $u_t$ is the laser intensity at time $t$.

**Performance metric:** NRMSE, as defined above. For the Santa Fe dataset, a good reservoir achieves NRMSE $\approx 0.05$–$0.08$. The theoretical lower bound (from Lyapunov exponent estimation) is approximately 0.02.

**What it tests:** Short-to-medium-range memory, ability to track chaotic oscillations, robustness to the irregular amplitude variations of the real laser signal.

**Practical note:** The Santa Fe dataset is discrete (integer-valued, 8-bit sampled), which means physical reservoir computers working with analog inputs must quantize carefully. Quantization noise can be the dominant error source for high-performing digital reservoirs.

## Benchmark 3: Spoken Digit Recognition (TI-46 / ISOLET)

**Task:** Classify isolated spoken digits (0–9) from their waveform. The standard dataset used in the physical RC literature is the **TI-46 corpus**: 10 digits, spoken by 5 female speakers, 10 repetitions each, for a total of 500 samples. Each sample is approximately 50–100 ms of 12-kHz audio.

**Preprocessing:** Raw waveforms are typically preprocessed into:
- Lyon ear model cochleagram: 86 frequency channels, subsampled to 200 Hz
- Mel Frequency Cepstral Coefficients (MFCCs): 13 or 39 coefficients per 10-ms frame
- Simple bandpass filterbank: 8–16 channels

**Definition of the task:** For each digit sample, run the reservoir on the (preprocessed) sequence. After the full sequence has been presented, read off the reservoir state (or a weighted average of states over the sequence) and train a linear classifier (ridge regression or linear SVM) to predict the digit label.

**Performance metric:** Word Error Rate (WER) on held-out speakers or fold.

**Typical results:** Human WER is approximately 0–2% on this task (it is quite easy for humans). Good digital ESN (N=100): 0–5% WER. Physical reservoir computers have achieved 0–5% WER on this benchmark, with some implementations (e.g., [Brunner2013]) achieving near-perfect performance.

**What it tests:** Temporal integration over 50–100 ms, frequency selectivity, robustness to within-class variation in duration and pitch.

## Benchmark 4: Channel Equalization

**Task:** Recover a transmitted binary symbol sequence from a signal that has been distorted by a nonlinear channel.

**Precise definition (Jaeger and Haas channel model):** The transmitted symbols $d_k \in \{-3, -1, +1, +3\}$ are passed through a nonlinear channel:

$$q_k = 0.08 d_{k+2} - 0.12 d_{k+1} + d_k + 0.18 d_{k-1} - 0.1 d_{k-2} + 0.091 d_{k-3} - 0.05 d_{k-4} + 0.04 d_{k-5} + 0.03 d_{k-6}$$

$$u_k = q_k + 0.036 q_k^2 - 0.011 q_k^3 + \nu_k$$

where $\nu_k \sim \mathcal{N}(0, \sigma_{noise}^2)$ is additive Gaussian noise with SNR defined as $\text{SNR} = 10\log_{10}(\text{var}(q) / \sigma_{noise}^2)$ dB.

**The task:** Recover $d_k$ from $\{u_k, u_{k-1}, \ldots\}$. The channel has memory depth $\approx 7$ symbols (the ISI span). The nonlinear term creates intermodulation products that a linear equalizer cannot fully remove.

**Performance metric:** Symbol Error Rate (SER) at a given SNR.

**Typical results:** For SNR = 32 dB, a good ESN (N=50) achieves SER $\approx 10^{-4}$–$10^{-3}$. A linear equalizer achieves SER $\approx 10^{-2}$. Physical reservoirs achieving SER $< 10^{-3}$ at SNR $= 32$ dB are considered excellent.

**What it tests:** Nonlinear memory of depth 7, four-class classification (the four symbol values), robustness to additive noise.

## Choosing the Right Benchmark

The four benchmarks test complementary capabilities:

| Benchmark | Memory | Nonlinearity | Real-world? | Dimensionality |
|---|---|---|---|---|
| NARMA-10 | 10 steps | Quadratic | No (synthetic) | Scalar |
| Santa Fe | 5–15 steps | Complex | Yes (real data) | Scalar |
| Spoken digit | 50–100 ms | Complex | Yes (speech) | Multi-channel |
| Channel eq. | 7 symbols | Cubic | Semi (model) | Scalar |

A complete evaluation of a physical reservoir should include at least NARMA-10 (for synthetic comparison) and one real-world benchmark (spoken digits or channel equalization). Reporting on all four is the gold standard.

---

## References

- [Appeltant2011] Appeltant, L. et al. (2011). Information processing using a single dynamical node as complex systems. *Nature Communications*, 2, 468.
- [Hübner1994] Hübner, U., Weiss, C.-O., Abraham, N.B., & Tang, D. (1994). Lorenz-like chaos in NH₃-FIR lasers. In *Spatio-Temporal Patterns in Nonequilibrium Complex Systems*. Addison-Wesley.
- [Jaeger2004] Jaeger, H. & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- [Nakajima2021] Nakajima, K. & Fischer, I. (eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer.
- [Brunner2013] Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013). Parallel photonic information processing at gigabyte per second data rates using transient states. *Nature Communications*, 4, 1364.
