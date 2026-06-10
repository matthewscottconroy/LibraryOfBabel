# Chapter 22 Exercises

## ECG and Biomedical Signal Exercises

**22.1** (ECG Beat Morphology). The PR interval (from P-wave onset to QRS onset) is prolonged to 240 ms in a patient with first-degree AV block (normal: 120–200 ms).

(a) At a sampling rate of 360 Hz, how many additional samples does this prolonged PR interval add to a beat segment compared to a normal beat?

(b) A reservoir trained on normal beats has a fixed beat window of 201 samples. How would the abnormal PR interval affect the input feature vector for this patient? What class might the classifier incorrectly predict?

(c) Propose a preprocessing step that would make the beat representation invariant to PR interval duration.

**22.2** (Class Imbalance in MIT-BIH). The MIT-BIH training set has approximately 50,000 N beats, 1,500 S beats, 3,500 V beats, 200 F beats, and 100 Q beats.

(a) Compute the inverse-frequency class weights $\omega_c = n_{\text{total}} / (n_{\text{classes}} \cdot n_c)$ for each class.

(b) Show that using these weights in the ridge regression is equivalent to solving an ordinary least squares problem on an oversampled dataset where each class has equal representation.

(c) An alternative is to train separate one-vs-rest binary classifiers for each class. What are the advantages and disadvantages of this approach versus the multi-class weighted regression?

**22.3** (EEG Band Power Features). A 1-second EEG segment at 256 Hz has $N = 256$ samples.

(a) Compute the DFT and identify the frequency resolution (Hz per bin).

(b) Define the delta band as $f \in [0.5, 4]$ Hz. Which DFT bins fall in this range?

(c) Show that the band power $E_b = \sum_{k \in \text{band}} |X_k|^2 / N$ equals the variance of the bandpass-filtered signal (Parseval's theorem).

(d) A seizure increases delta and theta band power by a factor of 3–5. Compute the resulting change in the normalized feature $\log E_b / \sum_b \log E_b$ and show that the normalized feature better distinguishes seizures from normal activity than the raw power.

**22.4** (Multi-Timescale Architecture). A hierarchical reservoir has two layers with leaking rates $\alpha_1 = 0.5$ and $\alpha_2 = 0.05$.

(a) What are the effective time constants (in milliseconds, at 256 Hz sampling) for each layer?

(b) An epileptic seizure evolves on two timescales: rapid HFO (high-frequency oscillations) at 80–120 Hz and slow ictal rhythm at 2–4 Hz. Which layer is better suited to capture each timescale, and why?

(c) Compare the memory capacity of a single 200-neuron ESN with $\alpha = 0.2$ versus a hierarchical ESN with two 100-neuron layers with $\alpha_1 = 0.5$ and $\alpha_2 = 0.05$. Which has greater total MC?

## Anomaly Detection Exercises

**22.5** (Prediction vs. Reconstruction). Consider two reservoir anomaly detectors: one based on prediction error $e_{\text{pred}}(t) = |u(t+1) - \hat{u}(t+1)|$ and one based on reconstruction error $e_{\text{recon}}(t) = |u(t) - \hat{u}_{\text{recon}}(t)|$.

(a) Give an example of an anomaly type that would be well-detected by prediction error but not reconstruction error.

(b) Give an example of the opposite: well-detected by reconstruction but not prediction.

(c) Propose a combined score $s_{\text{combined}}(t) = \lambda_1 e_{\text{pred}}(t) + \lambda_2 e_{\text{recon}}(t)$. How would you choose $\lambda_1$ and $\lambda_2$ on a validation set?

**22.6** (Threshold Selection). A reservoir anomaly detector trained on 1000 timesteps of normal data produces anomaly scores $\{s(t)\}$ on the training set. Empirically, the scores follow a roughly exponential distribution with mean $\mu = 0.1$.

(a) If the scores are exactly exponentially distributed with mean $\mu$, what threshold $\theta$ gives a false positive rate of 1% on normal data?

(b) In practice, extreme value theory suggests that the maximum of $n$ i.i.d. exponential random variables has a Gumbel distribution. Derive the CDF of $\max_{t=1}^n s(t)$ and use it to set a threshold for the expected maximum false alarm over a 24-hour monitoring window (sampling rate 1 Hz, so $n = 86400$).

(c) Run the provided anomaly detection code and compare the empirical false positive rate at the $3\sigma$ threshold to the theoretical prediction.

**22.7** (Energy Load Forecasting). Download a publicly available electricity load dataset (e.g., from ENTSO-E or US EIA) or use the synthetic generator below.

(a) Implement an ESN for 1-hour-ahead load forecasting. Use input features: past 24 hourly loads, hour-of-day (cyclically encoded), day-of-week (cyclically encoded).

(b) Compare MAPE on a 1-month test set for: (i) persistence model ($\hat{y}(t+1) = y(t)$), (ii) ESN, (iii) ARIMA(2,1,2).

(c) Implement recursive least squares (RLS) online updating for the ESN readout and show that online adaptation reduces MAPE by tracking seasonal pattern changes.

```python
def generate_load_data(T=8760, seed=42):
    """Synthetic electricity load: daily + weekly cycles + noise."""
    rng = np.random.RandomState(seed)
    t = np.arange(T)
    # Daily cycle (peak evening, trough 3am)
    daily = 3000 * (1 + 0.3 * np.sin(2*np.pi*(t % 24 - 6)/24))
    # Weekly cycle (lower weekend)
    weekly = 1.0 - 0.15 * ((t // 24) % 7 >= 5)
    # Temperature effect (summer/winter peaks)
    seasonal = 500 * np.cos(2*np.pi*t/(24*365))
    noise = 200 * rng.randn(T)
    return daily * weekly + seasonal + noise
```

**22.8** (Anomaly Type Discrimination). Run the `ReservoirAnomalyDetector` on the synthetic data generated by `generate_synthetic_anomaly_data` with each anomaly type separately (spike, plateau, frequency shift).

(a) For each anomaly type, compute the AUROC separately. Which is easiest/hardest to detect?

(b) The frequency shift anomaly introduces a periodic component at period 5 — very different from the AR(3) normal signal. Yet the prediction error might not capture this well immediately. Explain why and propose a modified anomaly score that would be more sensitive to frequency content changes.

(c) Implement a multi-score detector that combines prediction error with a score based on the Mahalanobis distance of the reservoir state from its training distribution. Does this improve AUROC?

## Advanced Exercises

**22.9** (Online Adaptation for EEG). In long-term EEG monitoring, the signal statistics change slowly due to drift in electrode impedance and changes in the patient's state. 

(a) Implement an online-updating ESN where the readout weights $W_{\text{out}}$ are updated every 60 seconds using the most recent 5-minute window of data.

(b) The challenge: during a seizure, updating on the seizure data would cause the model to adapt to the anomaly rather than flagging it. Propose a strategy for deciding when to update and when to freeze the model.

(c) Compare sensitivity and specificity with and without online adaptation on a simulated dataset where normal EEG amplitude increases by 50% over 30 minutes (e.g., due to drowsiness).

**22.10** (Comparative Study). The following methods are commonly used for ECG arrhythmia classification:

- ESN (this chapter)
- Random forest with hand-crafted features
- 1D CNN
- LSTM
- Transformer with positional encoding

For each method: (a) estimate the number of trainable parameters, (b) estimate training time on the MIT-BIH training set (CPU), (c) estimate inference time per beat, (d) discuss data efficiency (minimum training set size for acceptable performance).

On the basis of this comparison, for which deployment context (clinical workstation, wearable device, cloud service) would you recommend each method?
