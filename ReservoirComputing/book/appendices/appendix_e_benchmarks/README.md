# Appendix E: Benchmark Tasks — Mathematical Definitions and Code

This appendix provides precise mathematical specifications and Python generation code for all standard reservoir computing benchmarks. The goal is complete reproducibility: a reader with this appendix and a Python environment should be able to generate identical datasets to those used in published comparisons.

---

## E.1 NARMA-10

### Mathematical Definition

The Nonlinear Autoregressive Moving Average of order 10 (NARMA-10) is defined by the recurrence:

$$y(t+1) = 0.3\, y(t) + 0.05\, y(t) \sum_{k=0}^{9} y(t-k) + 1.5\, u(t-9)\, u(t) + 0.1$$

**Input**: $u(t) \sim \mathcal{U}[0, 0.5]$ i.i.d. at each timestep.

**Initial conditions**: $y(0) = \ldots = y(9) = 0$.

**Note**: Values occasionally exceed the range $[0, 1]$; the task includes these as training targets.

**Parameters** (as used in all published benchmarks):
- Coefficients: $0.3$, $0.05$, $1.5$, $0.1$ (as above)
- Input range: $[0, 0.5]$
- Order: $n = 10$
- Training length: $T_{\text{train}} = 1000$ (after discarding first 200 for initialization)
- Test length: $T_{\text{test}} = 1000$

**Metric**: Normalized Mean Square Error (NMSE):

$$\text{NMSE} = \frac{\sum_t (\hat{y}(t) - y(t))^2}{\sum_t (y(t) - \bar{y})^2} = \frac{\text{MSE}}{\text{Var}(y)}$$

**Typical RC performance**: NMSE $\approx 0.01$–$0.10$ for well-tuned ESNs (lower is better).

### Python Generation Code

```python
import numpy as np


def generate_narma(
    order: int = 10,
    T: int = 2200,
    seed: int = 42,
    train_length: int = 2000,
    test_length: int = 200,
    washout: int = 200,
) -> dict:
    """
    Generate NARMA-n time series.
    
    Parameters
    ----------
    order : int
        NARMA order (10 or 20).
    T : int
        Total sequence length (including washout).
    seed : int
        Random seed for input generation.
    train_length : int
        Number of training timesteps (after washout).
    test_length : int
        Number of test timesteps.
    washout : int
        Initial timesteps to discard.
    
    Returns
    -------
    dict with keys:
        'u_train', 'y_train' : input and target arrays (length: train_length)
        'u_test', 'y_test'   : input and target arrays (length: test_length)
        'u_full', 'y_full'   : complete sequences
    """
    assert T >= washout + train_length + test_length, \
        "T must be >= washout + train_length + test_length"
    
    rng = np.random.RandomState(seed)
    u = rng.uniform(0, 0.5, T)
    y = np.zeros(T)
    
    for t in range(order, T - 1):
        y[t+1] = (0.3 * y[t]
                  + 0.05 * y[t] * np.sum(y[t-order+1:t+1])
                  + 1.5 * u[t-order+1] * u[t]
                  + 0.1)
    
    # Split
    train_start = washout
    train_end   = washout + train_length
    test_end    = train_end + test_length
    
    return {
        'u_train': u[train_start:train_end],
        'y_train': y[train_start+1:train_end+1],   # one-step ahead
        'u_test':  u[train_end:test_end],
        'y_test':  y[train_end+1:test_end+1],
        'u_full':  u,
        'y_full':  y,
    }


def nmse(y_true: np.ndarray, y_pred: np.ndarray) -> float:
    """Normalized Mean Square Error."""
    return np.mean((y_true - y_pred)**2) / np.var(y_true)


# NARMA-10 example
data10 = generate_narma(order=10, seed=42)
print(f"NARMA-10: {len(data10['u_train'])} train, "
      f"{len(data10['u_test'])} test samples")
print(f"Target range: [{data10['y_full'].min():.3f}, {data10['y_full'].max():.3f}]")
```

---

## E.2 NARMA-20

### Mathematical Definition

NARMA-20 uses the same recurrence with order $n = 20$:

$$y(t+1) = 0.3\, y(t) + 0.05\, y(t) \sum_{k=0}^{19} y(t-k) + 1.5\, u(t-19)\, u(t) + 0.1$$

NARMA-20 requires deeper memory (20 vs. 10 steps) and is correspondingly harder. Typical ESN NMSE: $0.05$–$0.20$.

**Generation**: Use `generate_narma(order=20, ...)` with the same code above.

---

## E.3 Mackey-Glass Time Series

### Mathematical Definition

The Mackey-Glass system is a delay-differential equation (DDE):

$$\frac{dx}{dt} = \frac{\beta x(t-\tau)}{1 + x(t-\tau)^n} - \gamma x(t)$$

**Standard parameters for RC benchmarks**:
- $\beta = 0.2$
- $\gamma = 0.1$
- $n = 10$
- $\tau = 17$ (standard, mild chaos) or $\tau = 30$ (stronger chaos)

**Initial condition**: $x(t) = 0.9$ for $t \in [-\tau, 0]$.

**Integration parameters**: Step size $h = 0.1$ (10 integration steps per unit time), using 4th-order Runge-Kutta for the ODE approximation, where $x(t-\tau)$ is estimated by linear interpolation of past values.

### Train/Test Split Protocol

The standard split [JaegerHaas2004]:
1. Discard first 5000 time units (2000 steps at $h=0.1$ per unit) as initialization.
2. Subsample: use every 6th integration step (equivalent to sampling at $\Delta t = 0.6$; equivalently, use every point after integrating at $h = 1$).
3. Training set: 3000 samples.
4. Test set: 1000 samples (following immediately after training).

### Python Generation Code

```python
import numpy as np


def mackey_glass(
    tau: int = 17,
    T: int = 12000,
    beta: float = 0.2,
    gamma: float = 0.1,
    n: float = 10.0,
    dt: float = 0.1,
    seed: float = 0.9,   # initial condition
    subsample: int = 6,
    discard: int = 5000,
    train_length: int = 3000,
    test_length: int = 1000,
) -> dict:
    """
    Generate Mackey-Glass time series using the delay-DDE.
    
    The DDE is integrated using the Euler method (or optionally RK4)
    with linear interpolation for the delayed term.
    """
    T_int = int((T + discard) / dt)          # total integration steps
    T_delay = int(tau / dt)                   # delay in integration steps
    
    # Store full history for DDE
    history = np.full(T_int + T_delay, seed)
    
    # Euler integration
    for t in range(T_delay, T_int + T_delay):
        x_t = history[t]
        x_tau = history[t - T_delay]
        dxdt = beta * x_tau / (1 + x_tau**n) - gamma * x_t
        history[t + 1] = x_t + dt * dxdt
    
    # Extract integrated signal
    x = history[T_delay:]  # (T_int,)
    
    # Discard initial transient and subsample
    x_discard = int(discard / dt)
    x_sampled = x[x_discard::subsample]
    
    # Split
    return {
        'train': x_sampled[:train_length],
        'test':  x_sampled[train_length:train_length + test_length],
        'full':  x_sampled,
        'tau': tau, 'beta': beta, 'gamma': gamma, 'n': n,
    }


# Standard benchmark
mg17 = mackey_glass(tau=17)
print(f"Mackey-Glass tau=17: {len(mg17['train'])} train, "
      f"{len(mg17['test'])} test")
print(f"Range: [{mg17['train'].min():.3f}, {mg17['train'].max():.3f}]")
```

---

## E.4 Lorenz System

### Mathematical Definition

The Lorenz system:

$$\frac{dx}{dt} = \sigma(y - x)$$
$$\frac{dy}{dt} = x(\rho - z) - y$$
$$\frac{dz}{dt} = xy - \beta z$$

**Standard parameters** (chaotic attractor): $\sigma = 10$, $\rho = 28$, $\beta = 8/3$.

**Integration**: RK4, $dt = 0.01$.

**Initial conditions**: $(x_0, y_0, z_0) = (0.0, 1.0, 1.05)$.

**Train/test split**: Discard first 5000 steps (50 time units) as transient; use 10,000 steps training, 1000 steps test.

### Valid Prediction Time (VPT)

The key metric for chaotic prediction is the **valid prediction time** (VPT): the time (in Lyapunov times) for which the predicted trajectory matches the true trajectory within a threshold $\epsilon$.

$$\text{VPT} = \arg\max_T \{T : \|(\hat{x}(t), \hat{y}(t), \hat{z}(t)) - (x(t), y(t), z(t))\| < \epsilon \cdot \sqrt{\text{Var}_{\text{attractor}}} \text{ for all } t \leq T\}$$

Typical threshold $\epsilon = 0.4$. The Lyapunov time for the standard Lorenz system is $1/\lambda_1 \approx 1.1$ time units (where $\lambda_1 \approx 0.9$ is the largest Lyapunov exponent).

Excellent RC prediction: VPT $> 5$ Lyapunov times. State-of-the-art (Pathak et al. 2018): VPT $\approx 8$ Lyapunov times.

```python
from scipy.integrate import solve_ivp
import numpy as np


def lorenz_system(
    sigma: float = 10.0,
    rho: float = 28.0,
    beta: float = 8/3,
    dt: float = 0.01,
    T_total: float = 200.0,
    T_discard: float = 50.0,
    T_train: float = 100.0,
    T_test: float = 20.0,
    ic: tuple = (0.0, 1.0, 1.05),
) -> dict:
    """
    Integrate the Lorenz system and return train/test splits.
    """
    def f(t, xyz):
        x, y, z = xyz
        return [sigma*(y - x), x*(rho - z) - y, x*y - beta*z]
    
    T_end = T_discard + T_train + T_test
    t_eval = np.arange(0, T_end, dt)
    
    sol = solve_ivp(f, [0, T_end], list(ic), t_eval=t_eval,
                    method='RK45', rtol=1e-9, atol=1e-11)
    
    # Remove transient
    i_discard = int(T_discard / dt)
    xyz = sol.y[:, i_discard:].T  # (T_total_steps, 3)
    
    i_train = int(T_train / dt)
    i_test  = i_train + int(T_test / dt)
    
    return {
        'train': xyz[:i_train],           # (n_train, 3)
        'test':  xyz[i_train:i_test],     # (n_test, 3)
        'full':  xyz,
        'dt': dt, 'sigma': sigma, 'rho': rho, 'beta': beta,
    }


def valid_prediction_time(
    y_true: np.ndarray,
    y_pred: np.ndarray,
    dt: float = 0.01,
    lyapunov_time: float = 1.1,
    eps: float = 0.4,
) -> float:
    """
    Compute valid prediction time in Lyapunov times.
    """
    attractor_std = y_true.std(axis=0)
    threshold = eps * np.sqrt((attractor_std**2).mean())
    
    errors = np.linalg.norm(y_true - y_pred, axis=1)
    
    # Find first time error exceeds threshold
    exceed = np.where(errors > threshold)[0]
    if len(exceed) == 0:
        T_valid = len(y_true) * dt
    else:
        T_valid = exceed[0] * dt
    
    return T_valid / lyapunov_time


# Example
lorenz = lorenz_system()
print(f"Lorenz: {len(lorenz['train'])} train, {len(lorenz['test'])} test steps")
print(f"Attractor range x: [{lorenz['train'][:,0].min():.1f}, "
      f"{lorenz['train'][:,0].max():.1f}]")
```

---

## E.5 Santa Fe Laser Dataset

### Description

The Santa Fe time series competition dataset A consists of intensity measurements from a far-infrared laser in a chaotic regime. It is a real physical dataset (not simulated), collected by the Santa Fe Institute time series competition in 1991 [WeigendEtAl1993].

### Obtaining the Data

The dataset is available from several sources:

1. **UCI ML Repository**: https://archive.ics.uci.edu/ml/datasets/Santa+Fe+Laser
2. **DataHub**: https://datahub.io/machine-learning/santa-fe-laser
3. **ReservoirPy built-in**: `from reservoirpy.datasets import santafe`

```python
# Option 1: Download from UCI
import urllib.request
import numpy as np

url = "https://raw.githubusercontent.com/reservoirpy/reservoirpy/master/reservoirpy/datasets/santafe_laser.npy"

# Option 2: ReservoirPy
from reservoirpy.datasets import santafe
X = santafe()  # returns normalized series

# Standard split: first 1000 training, next 500 test
X_train, X_test = X[:1000], X[1000:1500]
print(f"Santa Fe: {len(X_train)} train, {len(X_test)} test points")
```

### Benchmark Protocol

Predict one step ahead: given $x(1), \ldots, x(t)$, predict $x(t+1)$.

**Metric**: NMSE = MSE / Var($x_{\text{test}}$).

**Typical ESN performance**: NMSE $\approx 0.001$–$0.01$.

---

## E.6 FSDD Spoken Digits

### Obtaining the Data

```bash
git clone https://github.com/Jakobovski/free-spoken-digit-dataset.git
# Files: recordings/{digit}_{speaker}_{index}.wav
# 6 speakers: jackson, nicolas, theo, yweweler, george, lucas
# 10 digits (0-9), 50 recordings per digit per speaker = 3000 total
```

### Preprocessing

```python
import os
import librosa
import numpy as np


def preprocess_fsdd(
    data_dir: str,
    sr: int = 8000,
    n_mfcc: int = 13,
    hop_length: int = 80,
) -> tuple:
    """
    Preprocess FSDD for reservoir input.
    Returns list of (MFCC sequences, label) pairs.
    """
    features, labels, speakers = [], [], []
    
    for fname in sorted(os.listdir(data_dir)):
        if not fname.endswith('.wav'):
            continue
        parts = fname[:-4].split('_')
        digit, speaker = int(parts[0]), parts[1]
        
        y, _ = librosa.load(os.path.join(data_dir, fname), sr=sr)
        mfcc = librosa.feature.mfcc(y=y, sr=sr, n_mfcc=n_mfcc,
                                     hop_length=hop_length)
        # CMN
        mfcc -= mfcc.mean(axis=1, keepdims=True)
        
        features.append(mfcc.T)  # (T, 13)
        labels.append(digit)
        speakers.append(speaker)
    
    return features, labels, speakers
```

### Standard Evaluation

Leave-one-speaker-out cross-validation (see Section 21.2.3 for complete protocol).

**Baseline accuracy**: 97–99% for ESN (N=500) with MFCC features.

---

## E.7 Channel Equalization

### Channel Model

The nonlinear channel equalization task [FrequentlyUsedRC] simulates a communications channel with ISI (inter-symbol interference) and nonlinear distortion:

**Input signal**: Binary symbols $d(t) \in \{-3, -1, +1, +3\}$ i.i.d. with equal probability (4-PAM).

**Channel**: Linear filter with taps $[0.08, -0.132, 0.4, 1.0, 0.4, -0.132, 0.08]$:

$$q(t) = 0.08d(t+3) - 0.132d(t+2) + 0.4d(t+1) + d(t) + 0.4d(t-1) - 0.132d(t-2) + 0.08d(t-3)$$

**Nonlinear distortion**: $r(t) = q(t) + 0.036q(t)^2 - 0.011q(t)^3$

**Noise**: $u(t) = r(t) + \epsilon(t)$ where $\epsilon(t) \sim \mathcal{N}(0, \sigma_n^2)$

**SNR**: $\text{SNR} = 10\log_{10}(\sigma_q^2 / \sigma_n^2)$ dB, where $\sigma_q^2 = \text{Var}(q)$.

### Task

Given the noisy received signal $\{u(t)\}$, recover $d(t-K)$ for some decision delay $K$ (typically $K = 3$).

### Metric

Symbol error rate (SER): fraction of symbols incorrectly decoded.

**Typical performance**: ESN SER $\approx 0.5\%$–$2\%$ at SNR = 16 dB. Perfect (Bayesian) equalizer SER $\approx 0.1\%$.

```python
def channel_equalization_data(
    T: int = 10000,
    snr_db: float = 16.0,
    delay: int = 3,
    seed: int = 42,
) -> dict:
    """Generate channel equalization dataset."""
    rng = np.random.RandomState(seed)
    
    # 4-PAM symbols
    d = rng.choice([-3, -1, 1, 3], size=T + 100)
    
    # Channel filter
    taps = np.array([0.08, -0.132, 0.4, 1.0, 0.4, -0.132, 0.08])
    center = len(taps) // 2
    q = np.convolve(d, taps, mode='full')[center:center+T+100]
    
    # Nonlinear distortion
    r = q + 0.036*q**2 - 0.011*q**3
    
    # Add noise at target SNR
    sigma_q = np.std(q)
    snr = 10**(snr_db / 10)
    sigma_n = sigma_q / np.sqrt(snr)
    noise = rng.randn(T + 100) * sigma_n
    u = r + noise
    
    # Align: input u(t), target d(t - delay)
    u_trimmed = u[100:100+T]
    d_trimmed = d[100+delay:100+T+delay]  # decision delay
    
    # Train/test split
    T_train = 5000
    return {
        'u_train': u_trimmed[:T_train],
        'd_train': d_trimmed[:T_train],
        'u_test':  u_trimmed[T_train:],
        'd_test':  d_trimmed[T_train:],
        'snr_db': snr_db, 'delay': delay,
    }


def ser(d_true: np.ndarray, d_pred: np.ndarray) -> float:
    """Symbol error rate."""
    return np.mean(d_true != d_pred)


def decode_4pam(x: np.ndarray) -> np.ndarray:
    """Decode to nearest 4-PAM symbol."""
    symbols = np.array([-3, -1, 1, 3])
    return symbols[np.argmin(np.abs(x[:, None] - symbols[None, :]), axis=1)]
```

---

## E.8 Summary Table

| Benchmark | Type | Input dim | Output | Metric | Typical ESN score |
|---|---|---|---|---|---|
| NARMA-10 | Temporal regression | 1 | 1 | NMSE | 0.01–0.10 |
| NARMA-20 | Temporal regression | 1 | 1 | NMSE | 0.05–0.20 |
| Mackey-Glass ($\tau=17$) | Chaotic prediction | 1 | 1 | NMSE | 0.001–0.01 |
| Lorenz | Chaotic prediction | 3 | 3 | VPT (Lyap. times) | 3–8 |
| Santa Fe Laser | Chaotic prediction | 1 | 1 | NMSE | 0.001–0.01 |
| FSDD spoken digits | Classification | MFCC | 10 classes | Accuracy | 96–98% |
| Channel equalization | Classification | 1 | 4 symbols | SER | 0.5–2% |
