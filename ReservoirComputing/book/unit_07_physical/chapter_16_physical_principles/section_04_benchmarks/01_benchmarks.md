# Standard Benchmarks for Physical Reservoir Computing

## The Role of Benchmarks

Benchmarks serve two functions in physical reservoir computing: (1) they enable comparison across disparate physical implementations that are otherwise incomparable (optical vs. mechanical vs. electronic), and (2) they provide a common vocabulary for discussing performance levels. The benchmarks used in the field range from simple nonlinear regression tasks to chaotic time series prediction to speech recognition. Each benchmark stresses different computational properties of the reservoir [Tanaka et al. 2019].

## NARMA-10

The NARMA-10 (Nonlinear Auto-Regressive Moving Average, order 10) task is defined by the recurrence:

$$y_t = 0.3 y_{t-1} + 0.05 y_{t-1} \sum_{i=0}^{9} y_{t-i} + 1.5 u_{t-9} u_t + 0.1,$$

where $u_t \sim \text{Uniform}[0, 0.5]$ is a random input. The target is to predict $y_t$ from the input stream $\{u_s\}_{s \leq t}$. The task requires integrating information over 10 time steps (memory) and computing the product $u_{t-9} u_t$ (nonlinearity of degree 2 in the inputs). The cubic term $y_{t-1} \sum y_{t-i}$ adds further nonlinearity.

Performance is measured by normalized mean squared error:

$$\text{NMSE} = \frac{\|\hat{\mathbf{y}} - \mathbf{y}\|^2}{\|\mathbf{y} - \bar{y}\mathbf{1}\|^2},$$

where $\bar{y}$ is the mean of the target. A perfect predictor has NMSE = 0; a constant predictor (predict the mean) has NMSE = 1. State-of-art results achieve NMSE $\approx 0.001$–$0.01$ for $N \geq 50$ virtual nodes [Appeltant et al. 2011].

## Mackey–Glass System

The Mackey–Glass equation is a nonlinear delay differential equation:

$$\dot{x}(t) = \frac{\beta x(t-\tau)}{1 + x(t-\tau)^n} - \gamma x(t),$$

with $\beta = 0.2$, $\gamma = 0.1$, $n = 10$, and $\tau = 17$ (chaotic regime). The prediction task is one-step-ahead prediction of $x(t+\Delta t)$ from the observed time series $\{x(s)\}_{s \leq t}$.

The Mackey–Glass system is less chaotic than Lorenz ($\lambda_{\max} \approx 0.007$ for $\tau = 17$), making it more tractable for reservoir prediction but still a meaningful test of temporal memory. Typical physical reservoir results achieve NRMSE $< 0.01$ for $N \geq 100$ nodes [Tanaka et al. 2019].

## Santa Fe Laser Dataset

The Santa Fe competition (1991) dataset B is a far-infrared laser time series measured over 10,000 time steps. The task is one-step-ahead prediction; performance is measured by NRMSE. The signal is approximately periodic with slow modulations, making it tractable but not trivial for reservoirs with moderate memory. Weigend & Gershenfeld [1994] report NRMSE $\approx 0.05$–$0.15$ for the winning competition entries; physical reservoirs with $N \geq 100$ nodes typically achieve NRMSE $\approx 0.02$–$0.08$ [Tanaka et al. 2019].

## TI-46 Spoken Digits

The TI-46 database consists of isolated spoken digit recordings (0–9) from 8 speakers, 10 repetitions each. Inputs are processed through a 10-channel cochleagram filter bank, producing a 10-dimensional time-varying input vector. Performance is measured by word error rate (WER) on a speaker-independent test set.

This task tests the reservoir's ability to discriminate between 10 temporal patterns despite speaker variability. Physical reservoirs with $N \geq 200$ nodes achieve WER $< 1\%$ on this task — competitive with LSTM baselines for this benchmark. Appeltant et al. [2011] achieved WER $= 0.4\%$ with $N = 400$ virtual nodes.

## Jaeger–Haas Channel Equalization

The wireless channel equalization task of Jaeger & Haas [2004] models a binary channel with nonlinear distortion and additive noise:

$$d_t = 0.08 u_{t+2} - 0.12 u_{t+1} + u_t + 0.18 u_{t-1} - 0.1 u_{t-2} + 0.091 u_{t-3} - 0.05 u_{t-4} + 0.04 u_{t-5} + 0.03 u_{t-6} + 0.01 u_{t-7},$$

$$q_t = d_t + 0.036 d_t^2 - 0.011 d_t^3,$$

$$r_t = q_t + \nu_t,$$

where $u_t \in \{-3, -1, +1, +3\}$, $\nu_t \sim \mathcal{N}(0, \sigma^2)$ is noise at SNR = 20 dB, and the task is to recover $u_{t-2}$ from the noisy received signal $r_t$. Performance is measured by symbol error rate (SER). Jaeger & Haas [2004] achieved SER $\approx 10^{-4}$ with a simulated ESN at 20 dB SNR.

## Performance Reference

The following table summarizes state-of-art physical reservoir results [Tanaka et al. 2019]:

| Benchmark | Metric | State-of-Art | Physical System |
|-----------|--------|--------------|-----------------|
| NARMA-10 | NMSE | 0.003–0.01 | Optoelectronic |
| Mackey–Glass $\tau=17$ | NRMSE | 0.003–0.01 | Optical fiber |
| Santa Fe laser | NRMSE | 0.02–0.05 | Silicon photonic |
| TI-46 spoken digits | WER | $< 0.5\%$ | Optoelectronic |
| Channel equalization | SER | $10^{-4}$–$10^{-3}$ | Optoelectronic |

---

## References

- Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- Mackey, M. C., & Glass, L. (1977). Oscillation and chaos in physiological control systems. *Science*, 197(4300), 287–289.
- Appeltant, L., et al. (2011). Information processing using a single dynamical node. *Nature Communications*, 2(1), 468.
- Tanaka, G., et al. (2019). Recent advances in physical reservoir computing. *Neural Networks*, 115, 100–123.
