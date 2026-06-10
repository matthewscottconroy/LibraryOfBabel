# Energy Load Forecasting with Reservoir Computing

## Task and Motivation

Electric power grid management requires accurate forecasts of electrical load (power demand) at horizons ranging from 1 hour ahead (real-time market) to 24 hours ahead (day-ahead planning). Load forecasting errors directly affect grid reliability: overestimation wastes generating capacity; underestimation risks blackouts. A 1% improvement in forecast MAPE (mean absolute percentage error) for a large grid operator translates to millions of dollars in reduced balancing costs annually [Hong & Fan 2016].

Load forecasting is a temporal regression problem well-suited to reservoir computing: load depends on recent load history (inertia of building thermal mass), weather conditions (temperature, humidity, cloud cover), time-of-day and day-of-week periodicity, and holidays. All these factors create temporal dependencies at multiple timescales (hourly, daily, weekly, annual), precisely where reservoir computing's multi-timescale representation is advantageous.

## Input Features

A standard input vector for reservoir-based load forecasting:

$$\mathbf{u}_t = [L_{t-1}, L_{t-2}, \ldots, L_{t-24}, T_t, H_t, W_t, \sin(2\pi t / 24), \cos(2\pi t / 24), \sin(2\pi d / 7), \cos(2\pi d / 7), \mathbf{1}_{\text{holiday}}] \in \mathbb{R}^p,$$

where $L_{t-i}$ is the lagged load (in GW or per unit), $T_t$ is temperature (°C), $H_t$ is humidity, $W_t$ is wind speed, the sinusoidal terms encode time-of-day and day-of-week periodicity, and $\mathbf{1}_{\text{holiday}}$ is a holiday indicator. The 24-hour lagged loads are the dominant predictors; weather contributes seasonally (air conditioning load in summer, heating load in winter) [Deihimi & Showkati 2012].

## Reservoir Configuration

For load forecasting, a leaky integrator ESN with moderate leak rate ($\alpha \approx 0.5$) and spectral radius ($\rho \approx 0.8$) works well. The leak rate controls how quickly the reservoir forgets past states; $\alpha = 0.5$ provides a balance between retaining the previous hour's load (relevant for thermal inertia) and tracking rapid weather changes.

The reservoir update:

$$\mathbf{x}_t = (1-\alpha)\mathbf{x}_{t-1} + \alpha \tanh(\mathbf{W}^{\text{rec}}\mathbf{x}_{t-1} + \mathbf{W}^{\text{in}}\mathbf{u}_t),$$

$$\hat{L}_{t+h} = \mathbf{w}_h^{\text{out} \top} \mathbf{x}_t \quad h = 1, \ldots, 24.$$

For multi-step-ahead forecasting, there are two strategies:

**Direct multi-step:** Train 24 separate readout vectors $\mathbf{w}_1^{\text{out}}, \ldots, \mathbf{w}_{24}^{\text{out}}$, each predicting $h$ steps ahead from the current state $\mathbf{x}_t$. Each readout is trained independently by ridge regression. This is robust to error accumulation.

**Autoregressive (recursive):** Use a 1-step-ahead readout and iterate: $\hat{L}_{t+1}$ is fed back as an input feature for predicting $\hat{L}_{t+2}$, etc. This can accumulate errors but captures temporal autocorrelation more explicitly.

For load forecasting beyond 6 hours, direct multi-step generally outperforms autoregressive due to error accumulation [Hong & Fan 2016].

## GEFCOM2012 Benchmark

The Global Energy Forecasting Competition 2012 (GEFCOM2012) provided hourly load data for a utility with a $\sim$5 GW peak load, along with weather observations, for 2004–2011. The task is day-ahead hourly load forecasting (24-step prediction).

Winning methods in GEFCOM2012 used gradient boosting and temperature-adjusted regression, achieving MAPE of 1.5–2.5%. Reservoir computing approaches achieve MAPE of approximately 2.5–4%, competitive for most practical applications though not state-of-art on this particular benchmark [Deihimi & Showkati 2012].

The performance gap between gradient boosting and ESN on this benchmark arises primarily from the boosting methods' ability to capture complex nonlinear interactions between weather variables and load response — interactions that are present in the data but require larger training sets than the ESN's ridge regression can fully exploit.

## Online Adaptation with RLS

Grid conditions change over time: new industrial customers, distributed solar generation, electric vehicle charging. An online-adaptive readout using RLS with forgetting (Chapter 5) updates $\mathbf{w}^{\text{out}}$ after each observed load value without full retraining:

$$\mathbf{w}^{\text{out}}(t) \leftarrow \mathbf{w}^{\text{out}}(t-1) + \frac{e(t)}{1 + \mathbf{x}_{t-h}^\top \mathbf{P}(t-1)\mathbf{x}_{t-h}} \mathbf{P}(t-1)\mathbf{x}_{t-h},$$

where $e(t) = L_t - \hat{L}_t$ is the forecast error and $\mathbf{x}_{t-h}$ is the reservoir state $h$ hours before. The RLS update is $O(N^2)$ per hour — trivial compared to full retrain. With forgetting factor $\mu = 0.999$ ($\tau_{\text{eff}} \approx 1000$ hours $\approx 6$ weeks), the readout adapts to seasonal load pattern changes automatically [Deihimi & Showkati 2012].

---

## References

- Hong, T., & Fan, S. (2016). Probabilistic electric load forecasting: A tutorial review. *International Journal of Forecasting*, 32(3), 914–938.
- Deihimi, A., & Showkati, H. (2012). Application of echo state networks in short-term electric load forecasting. *Energy*, 39(1), 327–340.
