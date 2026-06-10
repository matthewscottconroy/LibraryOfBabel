# Chapter 22: Key Concepts

## MIT-BIH Arrhythmia Database

The standard benchmark for ECG arrhythmia classification, containing 48 half-hour two-lead ECG recordings at 360 Hz with expert annotations. Beat classes are mapped to the ANSI/AAMI five-class scheme: N (normal), S (supraventricular ectopic), V (ventricular ectopic), F (fusion), Q (unclassifiable). The standard evaluation uses a 22/24 train/test record split as specified in the ANSI/AAMI EC57 standard. Severe class imbalance (N ≈ 90%) requires compensatory weighting or resampling.

## QRS Complex

The dominant waveform in the ECG corresponding to ventricular depolarization. Duration 60–100 ms; amplitude 0.5–2 mV. The R peak (positive peak of QRS) is used as the fiducial point for beat alignment and segmentation. R peak detection is typically the first preprocessing step in ECG analysis. RR interval = time between consecutive R peaks = 60/heart rate (bpm).

## ANSI/AAMI EC57

The American National Standards Institute standard for evaluating performance of cardiac arrhythmia detectors. Defines the five-class beat taxonomy, the train/test record split for MIT-BIH, and the performance metrics (sensitivity, specificity, positive predictive value per class) for regulatory approval of clinical arrhythmia monitoring systems.

## CHB-MIT Scalp EEG Database

Standard benchmark for EEG-based seizure detection, containing 686 hours of EEG from 22 pediatric subjects with medically refractory epilepsy, at 256 Hz. Each seizure event is annotated with onset and offset times by expert neurologists. ESN performance: sensitivity ~90–95%, false detection rate ~0.5–2 per hour (subject-specific models).

## EEG Frequency Bands

Clinically meaningful frequency ranges in the EEG: delta (0.5–4 Hz), theta (4–8 Hz), alpha (8–13 Hz), beta (13–30 Hz), gamma (30–100 Hz). Band power features are computed as the energy in each band from the short-time power spectrum. Seizures typically show increased delta/theta power and decreased alpha/beta power. HFOs (high-frequency oscillations at 80–500 Hz) are early seizure biomarkers.

## One-Class Classification

A classification paradigm where only examples of the "normal" class are available during training. The model learns a representation of normal behavior; test examples deviating from this representation are classified as anomalous. Natural for anomaly detection because anomalies are rare and often novel. Reservoir one-class classifiers train a normal-behavior predictor and use prediction error as the anomaly score.

## Prediction-Based Anomaly Score

For a reservoir trained to predict the next timestep on normal data, the anomaly score at time $t$ is $e(t) = \|u(t) - W_{\text{out}}\mathbf{x}(t-1)\|$. Large prediction errors indicate that the observed value is unexpected given the reservoir's learned model of normal dynamics. Smoothing $e(t)$ over a window of $W$ timesteps reduces sensitivity to single-timestep noise.

## Energy Load Forecasting

Predicting future electricity consumption from historical load, weather, calendar features, and economic indicators. A critical operational task for electricity grid management. ESNs are competitive with ARIMA and SVR for day-ahead 24-hour forecasts (MAPE ~2.5–3.5%). The key advantage of ESNs is online adaptability: the readout can be updated with recursive least squares as new observations arrive, tracking drift in load patterns.

## Hierarchical (Deep) Reservoir

A reservoir architecture with multiple layers, each operating at a different timescale (via different leaking rates $\alpha_i$). The first layer uses large $\alpha$ (fast time constant) to capture rapid signal variations; later layers use small $\alpha$ (slow time constant) to capture long-range dependencies. Effective for biomedical signals with multi-scale structure. The output layer observes states from all layers. Also called a deep echo state network (DeepESN) [GallicchioMicheli2017].

## AUROC (Area Under the ROC Curve)

The area under the Receiver Operating Characteristic curve, which plots true positive rate vs. false positive rate as the decision threshold varies. AUROC = 1 is perfect; AUROC = 0.5 is random. A threshold-free metric for anomaly detector performance, robust to class imbalance. For reservoir anomaly detectors on standard benchmarks, AUROC > 0.90 indicates useful detection performance.

## Class-Weighted Ridge Regression

A modification of ridge regression for imbalanced classification, where each training sample is assigned a weight $\omega_c$ inversely proportional to its class frequency. Equivalent to oversampling minority classes or undersampling majority classes. The solution is $W_{\text{out}} = (X^\top \Omega X + \lambda I)^{-1} X^\top \Omega Y$ where $\Omega = \text{diag}(\omega_{y(1)}, \ldots, \omega_{y(T)})$.
