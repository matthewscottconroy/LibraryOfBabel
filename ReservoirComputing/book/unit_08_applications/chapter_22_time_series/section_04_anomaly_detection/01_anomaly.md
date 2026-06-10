# Section 22.4: Anomaly Detection via Reservoir One-Class Classification

## 22.4.1 The Anomaly Detection Problem

Anomaly detection — identifying observations that deviate from expected normal behavior — is one of the most practically important problems in time series analysis. Applications span industrial fault detection, cybersecurity intrusion detection, fraud detection, medical monitoring, and predictive maintenance.

The key challenge distinguishing anomaly detection from standard classification is the absence of labeled anomaly data for training. Anomalies are, by definition, rare and often novel — a manufacturing defect or cyberattack may be of a type never seen before. Supervised approaches that require representative negative examples are therefore inapplicable. Instead, anomaly detection is framed as one-class classification: train a model of the normal behavior on unlabeled (or only positively labeled) data, then identify deviations from that model at test time.

Reservoir computing provides a natural approach to one-class anomaly detection in time series. The reservoir learns (implicitly, through its fixed dynamics) a representation of normal temporal structure. The readout is trained to predict the next observation from the current reservoir state. At test time, large prediction errors signal anomalous behavior — the reservoir "expects" certain future observations based on normal patterns, and violations of these expectations are flagged as anomalies.

## 22.4.2 One-Class Reservoir Classification

### Prediction-Based Anomaly Score

The most straightforward reservoir anomaly detection approach is prediction-based [ZimmerEtAl2019]:

1. **Training phase**: Train the reservoir to predict the next value $u(t+1)$ from the current state $\mathbf{x}(t)$:
   $$W_{\text{out}} = \arg\min_W \sum_t (u(t+1) - W\mathbf{x}(t))^2 + \lambda\|W\|^2$$
   
   Training is performed on normal data only.

2. **Test phase**: For each test timestep $t$, compute the prediction error:
   $$e(t) = |u(t) - W_{\text{out}}\mathbf{x}(t-1)|$$
   
3. **Anomaly score**: A smoothed version of the prediction error:
   $$s(t) = \frac{1}{W_{\text{smooth}}} \sum_{\tau=0}^{W_{\text{smooth}}-1} e(t-\tau)$$
   
4. **Decision**: Flag time $t$ as anomalous if $s(t) > \theta$, where $\theta$ is chosen based on a desired false positive rate on a validation set.

The intuition: on normal data, the reservoir has learned the temporal patterns, so predictions are accurate and $e(t)$ is small. Anomalies break these patterns, causing large prediction errors.

### Reconstruction-Based Anomaly Score

An alternative uses reconstruction error: train the reservoir readout to reconstruct the input from the reservoir state, penalizing reconstruction error. At test time, inputs that are poorly reconstructed are anomalous.

For multivariate time series $\mathbf{u}(t) \in \mathbb{R}^d$:
$$W_{\text{out}} = \arg\min_W \sum_t \|\mathbf{u}(t) - W\mathbf{x}(t)\|^2 + \lambda\|W\|^2$$

Anomaly score: $s(t) = \|\mathbf{u}(t) - W_{\text{out}}\mathbf{x}(t)\|$.

### State-Space Density Estimation

A more principled approach fits a density model to the reservoir states observed during training, then uses the density at test-time states as the normality score:

1. Collect training states $\{\mathbf{x}(t)\}_{t=1}^T$ on normal data.
2. Fit a Gaussian mixture model or kernel density estimator:
   $$p(\mathbf{x}) = \sum_{k=1}^K \pi_k \mathcal{N}(\mathbf{x}; \boldsymbol{\mu}_k, \Sigma_k)$$
3. Anomaly score at test time: $s(t) = -\log p(\mathbf{x}(t))$.

This approach is more principled but computationally expensive for high-dimensional reservoir states ($N \gg 100$). In practice, the reservoir state is often projected to a lower-dimensional space (e.g., via PCA retaining 95% variance) before density estimation.

## 22.4.3 Energy Load Forecasting

Energy load forecasting — predicting future electricity consumption — is a critical task for grid operators, energy traders, and renewable energy integration. Short-term forecasting (1 hour to 1 day ahead) informs dispatch decisions; medium-term forecasting (1 week to 1 month) informs maintenance scheduling; long-term forecasting (1 year) informs capacity planning.

The electricity load time series has strong periodic structure (daily and weekly cycles) plus stochastic variation due to weather, economic activity, and special events. The temporal dependencies span multiple timescales simultaneously — making reservoir computing a natural fit.

### Feature Engineering for Load Forecasting

Input features for an electricity load forecasting reservoir:
- Past loads: $u(t-1), u(t-2), \ldots, u(t-k)$ (lagged values)
- Time-of-day: $\sin(2\pi h / 24)$, $\cos(2\pi h / 24)$ (hour of day, cyclically encoded)
- Day-of-week: $\sin(2\pi d / 7)$, $\cos(2\pi d / 7)$
- Temperature: $T(t)$ and $T(t)^2$ (nonlinear relationship with cooling/heating load)
- Holiday indicator: $\mathbb{1}[\text{holiday}(t)]$

For a dataset with 15-minute resolution, 96 steps per day, and 7 lags: input dimension $d_{\text{in}} = 7 + 4 + 1 + 1 = 13$.

### Benchmark: French National Grid

The French national electricity load dataset (available from RTE, the French TSO) contains hourly load measurements from 2001 to present. It is a standard benchmark for load forecasting.

ESN performance (day-ahead 24-hour forecast):
- MAPE (Mean Absolute Percentage Error): $\sim 2.5$–$3.5\%$
- Comparable to SVR and ARIMA models
- Deep models (seq2seq LSTM, Temporal Fusion Transformer): $\sim 1.5$–$2.0\%$

The RC approach is competitive with classical methods and substantially simpler to train. For real-time adaptive forecasting (updating the model online as new data arrives), the RC approach has a significant advantage: the readout weights can be updated with each new observation using recursive least squares, while deep models require expensive retraining.

## 22.4.4 Complete Python Implementation: Reservoir Anomaly Detector

```python
"""
Reservoir Computing Anomaly Detection
Prediction-based one-class classification for time series.

Demonstrates on: (1) synthetic data with injected anomalies
                 (2) ECG normal vs. anomalous beat detection
"""

import numpy as np
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import roc_auc_score, roc_curve
import matplotlib.pyplot as plt
from typing import Optional, Tuple


class ReservoirAnomalyDetector:
    """
    One-class anomaly detector based on reservoir prediction error.
    
    Trained on normal data only. Anomalies are detected as
    time points with prediction error significantly exceeding
    the distribution observed during training.
    
    Parameters
    ----------
    n_reservoir : int
        Reservoir size.
    spectral_radius : float
        Spectral radius (0 < rho < 1 recommended).
    leaking_rate : float
        Leaking rate alpha. Controls temporal integration timescale.
    input_scaling : float
        Input weight scaling.
    ridge_alpha : float
        Ridge regularization for readout.
    smoothing_window : int
        Window size for smoothing the anomaly score (in timesteps).
    threshold_sigma : float
        Anomaly threshold: mean + threshold_sigma * std of training errors.
    seed : int
        Random seed.
    """
    
    def __init__(
        self,
        n_reservoir: int = 300,
        spectral_radius: float = 0.9,
        leaking_rate: float = 0.3,
        input_scaling: float = 0.5,
        ridge_alpha: float = 1e-4,
        smoothing_window: int = 10,
        threshold_sigma: float = 3.0,
        seed: int = 42,
    ):
        self.N = n_reservoir
        self.rho = spectral_radius
        self.alpha = leaking_rate
        self.s_in = input_scaling
        self.ridge = ridge_alpha
        self.smooth_w = smoothing_window
        self.thresh_sigma = threshold_sigma
        self.rng = np.random.RandomState(seed)
        
        self.W_res = None
        self.W_in = None
        self.bias = None
        self.W_out = None
        self.scaler = StandardScaler()
        self.threshold_ = None
        self._train_error_stats = None
    
    def _init_weights(self, n_inputs: int) -> None:
        N = self.N
        W = self.rng.randn(N, N)
        W[self.rng.rand(N, N) > 0.1] = 0.0
        ev = np.linalg.eigvals(W)
        W *= self.rho / (np.max(np.abs(ev)) + 1e-10)
        self.W_res = W
        self.W_in = self.s_in * self.rng.randn(N, n_inputs)
        self.bias = 0.1 * self.rng.randn(N)
    
    def _run_reservoir(
        self,
        U: np.ndarray,
        washout: int = 50,
    ) -> np.ndarray:
        """Drive reservoir, return post-washout states."""
        T = len(U)
        if self.W_res is None:
            self._init_weights(U.shape[1] if U.ndim > 1 else 1)
        
        if U.ndim == 1:
            U = U[:, None]
        
        x = np.zeros(self.N)
        states = []
        for t in range(T):
            pre = self.W_res @ x + self.W_in @ U[t] + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            if t >= washout:
                states.append(x.copy())
        return np.array(states)
    
    def fit(
        self,
        U_normal: np.ndarray,
        washout: int = 50,
    ) -> 'ReservoirAnomalyDetector':
        """
        Fit the anomaly detector on normal time series data.
        
        Parameters
        ----------
        U_normal : ndarray, shape (T,) or (T, d)
            Normal training time series.
        washout : int
            Number of initial timesteps to discard.
        """
        if U_normal.ndim == 1:
            U_normal = U_normal[:, None]
        
        # Normalize
        U_scaled = self.scaler.fit_transform(U_normal)
        
        # Get reservoir states
        states = self._run_reservoir(U_scaled, washout)
        T_valid = len(states)
        
        # Predict next step: train on states[:-1] -> target U_scaled[washout+1:]
        X_train = states[:-1]               # (T_valid-1, N)
        y_train = U_scaled[washout+1:]      # (T_valid-1, d)
        
        if len(y_train) > len(X_train):
            y_train = y_train[:len(X_train)]
        
        # Ridge regression
        A = X_train.T @ X_train + self.ridge * np.eye(self.N)
        b = X_train.T @ y_train
        self.W_out = np.linalg.solve(A, b)  # (N, d)
        
        # Compute training errors
        y_pred_train = X_train @ self.W_out
        errs = np.mean((y_train - y_pred_train)**2, axis=1)  # (T_valid-1,)
        
        mu_err = errs.mean()
        sigma_err = errs.std()
        self.threshold_ = mu_err + self.thresh_sigma * sigma_err
        self._train_error_stats = (mu_err, sigma_err)
        
        print(f"Training error: mu={mu_err:.4f}, sigma={sigma_err:.4f}")
        print(f"Anomaly threshold: {self.threshold_:.4f}")
        
        return self
    
    def anomaly_score(
        self,
        U_test: np.ndarray,
        washout: int = 50,
    ) -> np.ndarray:
        """
        Compute per-timestep anomaly score on test data.
        
        Returns
        -------
        scores : ndarray, shape (T - washout - 1,)
            Smoothed prediction error at each timestep.
        """
        if U_test.ndim == 1:
            U_test = U_test[:, None]
        
        U_scaled = self.scaler.transform(U_test)
        states = self._run_reservoir(U_scaled, washout)
        T_valid = len(states)
        
        X_test = states[:-1]
        y_test = U_scaled[washout+1:washout+1+len(X_test)]
        
        y_pred = X_test @ self.W_out
        raw_err = np.mean((y_test - y_pred)**2, axis=1)
        
        # Smooth
        if self.smooth_w > 1:
            kernel = np.ones(self.smooth_w) / self.smooth_w
            scores = np.convolve(raw_err, kernel, mode='same')
        else:
            scores = raw_err
        
        return scores
    
    def predict(
        self,
        U_test: np.ndarray,
        washout: int = 50,
    ) -> np.ndarray:
        """Binary prediction: 1 = anomaly, 0 = normal."""
        scores = self.anomaly_score(U_test, washout)
        return (scores > self.threshold_).astype(int)


def generate_synthetic_anomaly_data(
    T_normal: int = 2000,
    T_test: int = 1000,
    anomaly_prob: float = 0.05,
    seed: int = 42,
) -> Tuple[np.ndarray, np.ndarray, np.ndarray]:
    """
    Generate a time series with injected anomalies.
    
    Normal signal: AR(3) process with sinusoidal component.
    Anomalies: random amplitude spikes or frequency shifts.
    
    Returns
    -------
    U_train : normal training signal (T_normal,)
    U_test  : test signal with anomalies (T_test,)
    labels  : binary anomaly labels for test (T_test,)
    """
    rng = np.random.RandomState(seed)
    
    def ar3_signal(T, rng):
        """AR(3) process: x(t) = 0.6x(t-1) - 0.3x(t-2) + 0.1x(t-3) + eps"""
        x = np.zeros(T + 100)
        for t in range(3, T + 100):
            x[t] = (0.6*x[t-1] - 0.3*x[t-2] + 0.1*x[t-3]
                    + 0.3*rng.randn())
        # Add sinusoidal trend
        t_arr = np.arange(T + 100)
        x += 0.5 * np.sin(2*np.pi*t_arr / 50)
        return x[100:]  # discard transient
    
    U_train = ar3_signal(T_normal, rng)
    U_test_base = ar3_signal(T_test, rng)
    
    # Inject anomalies
    labels = np.zeros(T_test, dtype=int)
    anomaly_mask = rng.rand(T_test) < anomaly_prob
    U_test = U_test_base.copy()
    
    for t in np.where(anomaly_mask)[0]:
        anomaly_type = rng.choice(['spike', 'plateau', 'freq_shift'])
        duration = rng.randint(3, 15)
        end = min(t + duration, T_test)
        
        if anomaly_type == 'spike':
            U_test[t] += rng.choice([-1, 1]) * rng.uniform(3, 6) * U_train.std()
        elif anomaly_type == 'plateau':
            U_test[t:end] = rng.uniform(-2, 2) * U_train.std()
        else:  # freq_shift
            dt = np.arange(end - t)
            U_test[t:end] += 2.0 * np.sin(2*np.pi*dt / 5)
        
        labels[t:end] = 1
    
    return U_train, U_test, labels


def run_anomaly_detection_demo() -> None:
    """Full anomaly detection demonstration."""
    print("Generating synthetic time series with anomalies...")
    U_train, U_test, y_true = generate_synthetic_anomaly_data(
        T_normal=2000, T_test=1000, anomaly_prob=0.03
    )
    
    print(f"Training samples: {len(U_train)}")
    print(f"Test samples: {len(U_test)}, Anomalies: {y_true.sum()}")
    
    # Fit anomaly detector
    detector = ReservoirAnomalyDetector(
        n_reservoir=300,
        spectral_radius=0.9,
        leaking_rate=0.2,
        smoothing_window=5,
        threshold_sigma=3.0,
    )
    detector.fit(U_train)
    
    # Compute scores
    scores = detector.anomaly_score(U_test)
    washout = 50
    y_true_aligned = y_true[washout+1:washout+1+len(scores)]
    
    # AUROC
    auc = roc_auc_score(y_true_aligned, scores)
    print(f"\nAUROC: {auc:.3f}")
    
    # Predicted labels
    y_pred = (scores > detector.threshold_).astype(int)
    tp = ((y_pred == 1) & (y_true_aligned == 1)).sum()
    fp = ((y_pred == 1) & (y_true_aligned == 0)).sum()
    fn = ((y_pred == 0) & (y_true_aligned == 1)).sum()
    sensitivity = tp / (tp + fn + 1e-8)
    fpr = fp / ((y_true_aligned == 0).sum() + 1e-8)
    print(f"Sensitivity: {sensitivity:.3f}, False positive rate: {fpr:.3f}")
    
    # Visualization
    fig, axes = plt.subplots(3, 1, figsize=(14, 8))
    t_ax = np.arange(len(scores))
    
    axes[0].plot(t_ax, U_test[washout+1:washout+1+len(scores)], 
                 'b', lw=0.7, alpha=0.8, label='Test signal')
    axes[0].set_ylabel("Signal")
    axes[0].set_title("Reservoir Anomaly Detection Demo")
    
    axes[1].plot(t_ax, scores, 'r', lw=0.8, label='Anomaly score')
    axes[1].axhline(detector.threshold_, color='k', ls='--', 
                    label=f'Threshold ($3\sigma$)')
    axes[1].set_ylabel("Anomaly score")
    axes[1].legend(loc='upper right', fontsize=8)
    
    axes[2].fill_between(t_ax, y_true_aligned, alpha=0.5, 
                         color='orange', label='True anomalies')
    axes[2].fill_between(t_ax, y_pred * 0.7, alpha=0.4, 
                         color='red', label='Detected anomalies')
    axes[2].set_ylabel("Anomaly flag")
    axes[2].set_xlabel("Timestep")
    axes[2].legend(loc='upper right', fontsize=8)
    
    plt.tight_layout()
    plt.savefig("reservoir_anomaly_detection.png", dpi=150)
    print("Saved to reservoir_anomaly_detection.png")
    
    # ROC curve
    fpr_arr, tpr_arr, _ = roc_curve(y_true_aligned, scores)
    fig2, ax2 = plt.subplots(figsize=(5,5))
    ax2.plot(fpr_arr, tpr_arr, 'b-', lw=2, label=f'ESN (AUC={auc:.3f})')
    ax2.plot([0,1],[0,1],'k--', lw=1)
    ax2.set_xlabel("False Positive Rate")
    ax2.set_ylabel("True Positive Rate")
    ax2.set_title("ROC Curve — Reservoir Anomaly Detector")
    ax2.legend()
    plt.tight_layout()
    plt.savefig("reservoir_anomaly_roc.png", dpi=150)
    print("Saved ROC to reservoir_anomaly_roc.png")


if __name__ == "__main__":
    run_anomaly_detection_demo()
```

## 22.4.5 Choosing the Anomaly Threshold

The threshold $\theta$ is the most operationally critical parameter of an anomaly detector. A threshold set too low produces many false alarms; too high, and real anomalies are missed. The trade-off is captured by the ROC curve.

For safety-critical applications (medical monitoring, industrial safety), a common criterion is to set the threshold such that the false positive rate on normal validation data does not exceed some acceptable level $\alpha_{\text{FP}}$:

$$\theta = F_\theta^{-1}(1 - \alpha_{\text{FP}})$$

where $F_\theta$ is the empirical CDF of anomaly scores on normal validation data.

Alternatively, Extreme Value Theory (EVT) can be used to model the tail of the normal score distribution and set the threshold based on a specified return period — the expected time between false alarms.
