# Section 22.3: Biomedical Signal Analysis — ECG and EEG

## 22.3.1 The Biomedical Signal Processing Challenge

Biomedical signals pose a distinctive set of challenges for time series analysis:

**Non-stationarity**: Physiological processes are inherently non-stationary. A patient's ECG changes with posture, activity, breathing, and emotion. An EEG recording changes with cognitive state, vigilance, and pathology. Methods must either model this non-stationarity explicitly or be robust to it.

**Label scarcity**: Clinical annotations are expensive. An expert cardiologist may spend several minutes annotating a single 30-second ECG segment. The total labeled data available for training is thus orders of magnitude smaller than in commercial speech or vision applications.

**Inter-subject variability**: Cardiac and neural signals vary substantially between individuals. A model trained on one population may not generalize to another. Subject-specific fine-tuning is often necessary.

**Safety criticality**: In clinical applications, false negatives (missed detections) have direct consequences for patient safety. This motivates careful attention to sensitivity and specificity beyond overall accuracy.

Reservoir computing addresses the first two challenges naturally: the reservoir provides a rich representation that adapts to changing signal statistics through its own dynamics, and the simple linear readout can be trained effectively from small labeled datasets. The third and fourth challenges require careful evaluation design, which we emphasize in the benchmark descriptions below.

## 22.3.2 ECG Arrhythmia Classification on MIT-BIH

### Signal Characteristics

The electrocardiogram (ECG) records the electrical activity of the heart as a time-varying voltage measured on the body surface. The characteristic morphology consists of:

- **P wave**: Atrial depolarization, amplitude $\sim 0.1$–$0.2$ mV, duration $\sim 80$–$120$ ms
- **QRS complex**: Ventricular depolarization, the dominant feature, amplitude $\sim 0.5$–$2.0$ mV, duration $\sim 60$–$100$ ms
- **T wave**: Ventricular repolarization, amplitude $\sim 0.1$–$0.5$ mV, duration $\sim 120$–$160$ ms

The RR interval — time between successive R peaks — carries information about heart rate and rhythm. Normal sinus rhythm has RR intervals of 600–1000 ms (60–100 bpm). Arrhythmias alter the morphology and/or timing of these waveforms in characteristic ways.

### MIT-BIH Arrhythmia Database

The MIT-BIH Arrhythmia Database [MoodyMark2001] is the standard benchmark for arrhythmia classification. It contains:
- 48 half-hour ECG recordings
- 47 subjects, two-lead recordings (lead MLII and one other)
- Sampling rate: 360 Hz
- 109,000+ annotations by two cardiologists
- 19 distinct rhythm types, grouped into 5 classes for the standard evaluation:
  - **N**: Normal beat
  - **S**: Supraventricular ectopic beat (includes atrial premature, APC)
  - **V**: Ventricular ectopic beat (PVC, ventricular escape)
  - **F**: Fusion beat
  - **Q**: Unknown/unclassifiable

The standard evaluation uses the ANSI/AAMI EC57 protocol: train on records 101, 106, 108, 109, 112, 114, 115, 116, 118, 119, 122, 124, 201, 203, 205, 207, 208, 209, 215, 220, 223, 230; test on the remaining 24 records.

### Reservoir Architecture for ECG

The feature pipeline for ECG classification consists of:

**Step 1: R-peak detection**. Use a QRS detector (e.g., Pan-Tompkins algorithm [PanTompkins1985]) to locate R peaks. This converts the continuous waveform into a sequence of beats.

**Step 2: Beat segmentation**. Extract a window of $W$ samples around each R peak: $\mathbf{b}(k) = p(r_k - W_1 : r_k + W_2)$, where $r_k$ is the $k$-th R peak index. Standard: $W_1 = 100$ samples (278 ms before R), $W_2 = 100$ samples (278 ms after R), giving $W = 201$ samples per beat.

**Step 3: Feature extraction**. Options:
- **Raw beat segment**: Feed the 201-sample waveform directly to the reservoir
- **Frequency features**: DFT of the beat segment (first 30 coefficients)
- **Time-domain features**: QRS duration, RR interval, peak amplitude, ST segment deviation
- **Combined**: Concatenate multiple feature types

**Step 4: Reservoir processing**. Feed the feature sequence beat-by-beat (treating each beat as one "input timestep") to the reservoir:

$$\mathbf{x}(k) = (1-\alpha)\mathbf{x}(k-1) + \alpha \tanh(W_{\text{res}}\mathbf{x}(k-1) + W_{\text{in}}\mathbf{b}(k) + \mathbf{b}_{\text{bias}})$$

Note: the input here is the full beat segment $\mathbf{b}(k) \in \mathbb{R}^{201}$ — a high-dimensional input per timestep. This is accommodated by the input weight matrix $W_{\text{in}} \in \mathbb{R}^{N \times 201}$.

**Step 5: Classification**. Train a readout on the reservoir state $\mathbf{x}(k)$ to predict the label $y(k) \in \{N, S, V, F, Q\}$.

### Class Imbalance

The MIT-BIH corpus is severely class-imbalanced:
- N: $\sim 90\%$ of beats
- S: $\sim 2.5\%$
- V: $\sim 7\%$
- F: $\sim 0.5\%$
- Q: $\sim 0.1\%$

Without correction, a classifier that predicts N for every beat achieves $\sim 90\%$ accuracy but is clinically useless. Standard corrections:

**Oversampling**: Replicate minority class examples (SMOTE generates synthetic examples by interpolation in feature space).

**Class weighting**: Weight the ridge regression objective by inverse class frequency:

$$W_{\text{out}} = \arg\min_W \sum_k \omega_{y(k)} \|e_k - W\mathbf{x}(k)\|^2 + \lambda\|W\|_F^2$$

where $\omega_c = n_{\text{total}} / (n_{\text{classes}} \cdot n_c)$ is the weight for class $c$.

**Threshold adjustment**: After training, move the decision boundary to equalize sensitivity across classes.

### Performance Benchmarks

| Method | Overall Acc | Sensitivity (V) | Specificity (V) |
|---|---|---|---|
| ESN (N=500, beat features) | $\sim 95\%$ | $\sim 88\%$ | $\sim 98\%$ |
| ESN + class weighting | $\sim 93\%$ | $\sim 92\%$ | $\sim 95\%$ |
| SVM (RBF kernel) | $\sim 97\%$ | $\sim 90\%$ | $\sim 99\%$ |
| Deep CNN (1D) | $\sim 99\%$ | $\sim 96\%$ | $\sim 99.5\%$ |
| Transformer | $\sim 99.5\%$ | $\sim 97\%$ | $\sim 99.8\%$ |

The ESN is competitive with classical methods (SVM) and usefully deployable in resource-constrained settings, but is outperformed by deep models with sufficient data. The RC advantage is in settings where labeled data is scarce or the model must be updated online.

```python
"""
ECG Arrhythmia Classification on MIT-BIH using Echo State Networks

Requirements:
    pip install wfdb numpy scikit-learn reservoirpy matplotlib

Dataset:
    wfdb.dl_database('mitdb', './mitdb_data')
"""

import numpy as np
import wfdb
from sklearn.metrics import classification_report, confusion_matrix
from sklearn.preprocessing import StandardScaler
import matplotlib.pyplot as plt

# ANSI/AAMI beat-type mapping
AAMI_MAP = {
    'N': 'N',   # Normal
    'L': 'N',   # Left bundle branch block
    'R': 'N',   # Right bundle branch block
    'e': 'N',   # Atrial escape
    'j': 'N',   # Nodal (junctional) escape
    'A': 'S',   # Atrial premature
    'a': 'S',   # Aberrated atrial premature
    'J': 'S',   # Nodal premature
    'S': 'S',   # Supraventricular premature
    'V': 'V',   # Premature ventricular
    'E': 'V',   # Ventricular escape
    'F': 'F',   # Fusion
    '/': 'Q',   # Paced
    'f': 'Q',   # Fusion of paced and normal
    'Q': 'Q',   # Unclassifiable
}
LABEL_TO_INT = {'N': 0, 'S': 1, 'V': 2, 'F': 3, 'Q': 4}

TRAIN_RECORDS = [
    '101','106','108','109','112','114','115','116','118','119',
    '122','124','201','203','205','207','208','209','215','220','223','230'
]
TEST_RECORDS = [
    '100','103','105','111','113','117','121','123','200','202',
    '210','212','213','214','219','221','222','228','231','232','233','234'
]


def load_mitbih_beats(
    record_ids: list,
    data_dir: str = './mitdb_data',
    window_before: int = 100,
    window_after: int = 100,
) -> tuple:
    """
    Load beats from MIT-BIH records and return feature matrices.
    Each beat is a segment of the ECG centered on the R-peak.
    """
    beats, labels = [], []
    
    for rec_id in record_ids:
        try:
            record = wfdb.rdrecord(f'{data_dir}/{rec_id}')
            annotation = wfdb.rdann(f'{data_dir}/{rec_id}', 'atr')
        except Exception as e:
            print(f"Could not load record {rec_id}: {e}")
            continue
        
        signal = record.p_signal[:, 0]  # Lead MLII
        
        for i, (sample, symbol) in enumerate(
            zip(annotation.sample, annotation.symbol)
        ):
            if symbol not in AAMI_MAP:
                continue
            aami_class = AAMI_MAP[symbol]
            
            start = sample - window_before
            end = sample + window_after + 1
            if start < 0 or end > len(signal):
                continue
            
            beat = signal[start:end]
            
            # Normalize beat amplitude
            beat = (beat - beat.mean()) / (beat.std() + 1e-8)
            
            beats.append(beat)
            labels.append(LABEL_TO_INT[aami_class])
    
    return np.array(beats), np.array(labels)


class ECGReservoirClassifier:
    """
    Reservoir computing classifier for ECG beat classification.
    
    The reservoir processes a sequence of beats over time, maintaining
    context about the rhythm history (not just the current beat morphology).
    """
    
    def __init__(
        self,
        n_reservoir: int = 500,
        spectral_radius: float = 0.9,
        leaking_rate: float = 0.5,
        input_scaling: float = 0.1,
        ridge_alpha: float = 1e-3,
        context_window: int = 5,
        seed: int = 42,
    ):
        self.N = n_reservoir
        self.rho = spectral_radius
        self.alpha = leaking_rate
        self.s_in = input_scaling
        self.ridge = ridge_alpha
        self.context = context_window
        self.rng = np.random.RandomState(seed)
        self._initialized = False
    
    def _init_weights(self, n_inputs: int) -> None:
        N = self.N
        # Sparse reservoir (10% connectivity)
        W = self.rng.randn(N, N)
        W[self.rng.rand(N, N) > 0.1] = 0.0
        ev = np.linalg.eigvals(W)
        W *= self.rho / (np.max(np.abs(ev)) + 1e-10)
        self.W_res = W
        self.W_in = self.s_in * self.rng.randn(N, n_inputs)
        self.b = 0.1 * self.rng.randn(N)
        self._initialized = True
    
    def _compute_states(self, beats: np.ndarray) -> np.ndarray:
        """Process beat sequence through reservoir."""
        n_beats, beat_len = beats.shape
        if not self._initialized:
            self._init_weights(beat_len)
        
        x = np.zeros(self.N)
        states = []
        for t in range(n_beats):
            pre = self.W_res @ x + self.W_in @ beats[t] + self.b
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            states.append(x.copy())
        return np.array(states)
    
    def fit(
        self,
        beats_train: np.ndarray,
        labels_train: np.ndarray,
        class_weight: bool = True,
    ) -> 'ECGReservoirClassifier':
        """
        Train the readout.
        beats_train : (n_beats, beat_length) — treated as a single sequence
        """
        self.scaler = StandardScaler().fit(beats_train)
        X = self.scaler.transform(beats_train)
        
        states = self._compute_states(X)  # (n_beats, N)
        
        n_classes = 5
        Y = np.eye(n_classes)[labels_train]  # one-hot
        
        # Class weighting
        if class_weight:
            counts = np.bincount(labels_train, minlength=n_classes)
            weights = len(labels_train) / (n_classes * (counts + 1))
            W_diag = np.diag([weights[l] for l in labels_train])
            A = states.T @ W_diag @ states + self.ridge * np.eye(self.N)
            b = states.T @ W_diag @ Y
        else:
            A = states.T @ states + self.ridge * np.eye(self.N)
            b = states.T @ Y
        
        self.W_out = np.linalg.solve(A, b).T  # (n_classes, N)
        return self
    
    def predict(self, beats_test: np.ndarray) -> np.ndarray:
        X = self.scaler.transform(beats_test)
        states = self._compute_states(X)
        scores = states @ self.W_out.T  # (n_beats, n_classes)
        return np.argmax(scores, axis=1)


def run_mitbih_benchmark(data_dir: str = './mitdb_data') -> None:
    print("Loading training data...")
    X_tr, y_tr = load_mitbih_beats(TRAIN_RECORDS, data_dir)
    print(f"  {len(X_tr)} training beats")
    print(f"  Class distribution: {np.bincount(y_tr)}")
    
    print("Loading test data...")
    X_te, y_te = load_mitbih_beats(TEST_RECORDS, data_dir)
    print(f"  {len(X_te)} test beats")
    
    clf = ECGReservoirClassifier(
        n_reservoir=500, spectral_radius=0.9,
        leaking_rate=0.5, ridge_alpha=1e-3
    )
    print("Training ESN readout...")
    clf.fit(X_tr, y_tr, class_weight=True)
    
    print("Evaluating...")
    y_pred = clf.predict(X_te)
    
    print("\nClassification Report:")
    target_names = ['N', 'S', 'V', 'F', 'Q']
    print(classification_report(y_te, y_pred, target_names=target_names))
    
    cm = confusion_matrix(y_te, y_pred)
    fig, ax = plt.subplots(figsize=(6,5))
    ax.imshow(cm, cmap='Blues', interpolation='nearest')
    ax.set_xticks(range(5)); ax.set_yticks(range(5))
    ax.set_xticklabels(target_names); ax.set_yticklabels(target_names)
    ax.set_xlabel("Predicted"); ax.set_ylabel("True")
    ax.set_title("MIT-BIH ESN Confusion Matrix")
    plt.tight_layout()
    plt.savefig("mitbih_esn_cm.png", dpi=150)
    print("Saved confusion matrix to mitbih_esn_cm.png")


if __name__ == "__main__":
    run_mitbih_benchmark()
```

## 22.3.3 EEG Seizure Detection

### Signal Characteristics

The electroencephalogram (EEG) records brain electrical activity from electrodes on the scalp. Standard clinical EEG uses 10–20 electrode placement (19–256 channels) sampled at 256–1024 Hz. The signal reflects the superposition of post-synaptic potentials from millions of neurons.

Clinically relevant frequency bands:
- Delta ($\delta$): 0.5–4 Hz — sleep, deep anesthesia
- Theta ($\theta$): 4–8 Hz — drowsiness, meditative states
- Alpha ($\alpha$): 8–13 Hz — relaxed wakefulness
- Beta ($\beta$): 13–30 Hz — active thinking
- Gamma ($\gamma$): 30–100 Hz — high cognitive load

### Seizure Detection Problem

Epileptic seizures are pathological, excessive, synchronized neural discharges. In the EEG, seizures appear as high-amplitude, rhythmic oscillations often starting in one region and spreading. Automatic seizure detection is clinically important: a system that flags seizure events in long-term EEG monitoring enables faster intervention and more detailed patient assessment.

The reservoir computing approach to seizure detection:

**Features**: EEG is typically preprocessed as follows:
1. Bandpass filter into frequency bands
2. Compute band energies: $E_b(n) = \sum_{f \in \text{band}_b} |\text{STFT}(n, f)|^2$
3. Form feature vector: $\mathbf{u}(n) = [E_\delta(n), E_\theta(n), E_\alpha(n), E_\beta(n), E_\gamma(n), \text{line length}(n)]$ for each channel

For a 19-channel EEG with 6 features per channel, the input dimension is $d_{\text{in}} = 114$.

**Reservoir**: Process the feature sequence at 1-second windows (or shorter for lower latency).

**Readout**: Binary classification: seizure / non-seizure per window.

### CHB-MIT Benchmark

The CHB-MIT Scalp EEG Database [ShocklockEtAl2010] contains long-term EEG recordings from 22 pediatric subjects with pharmacologically intractable seizures:
- 686 hours of EEG
- 198 seizure events (average duration: 72 s)
- 19 channels, 256 Hz sampling rate

ESN performance on CHB-MIT (subject-specific models):
- Sensitivity: $\sim 90$–$95\%$
- False detection rate: $\sim 0.5$–$2.0$ per hour
- Latency: $\sim 5$–$15$ s after seizure onset

These numbers are competitive with classical machine learning approaches (SVM, random forest) and within the range of clinical usefulness (a system with 0.5 false detections per hour and 90% sensitivity would be clinically deployable).

### Multi-Timescale Architecture for EEG

A distinctive feature of epileptic seizures is their multi-timescale structure: the seizure onset is characterized by rapid high-frequency oscillations ($\sim 80$–$120$ Hz), while the sustained ictal activity involves slower rhythms (2–10 Hz). A single-timescale reservoir cannot capture both simultaneously.

A multi-timescale (hierarchical) reservoir [GallicchioMicheli2017] addresses this by stacking multiple reservoirs with different leaking rates:

$$\mathbf{x}^{(1)}(n) = (1-\alpha_1)\mathbf{x}^{(1)}(n-1) + \alpha_1 \tanh(W_1 \mathbf{x}^{(1)}(n-1) + W_1^{\text{in}}\mathbf{u}(n))$$
$$\mathbf{x}^{(2)}(n) = (1-\alpha_2)\mathbf{x}^{(2)}(n-1) + \alpha_2 \tanh(W_2 \mathbf{x}^{(2)}(n-1) + W_2^{\text{in}}\mathbf{x}^{(1)}(n))$$

with $\alpha_1 > \alpha_2$ (fast first layer, slow second layer). The readout uses both $\mathbf{x}^{(1)}(n)$ and $\mathbf{x}^{(2)}(n)$, combining fast and slow features. For EEG seizure detection, using two layers with $\alpha_1 = 0.5$ (fast) and $\alpha_2 = 0.05$ (slow) improves sensitivity by $3$–$5$ percentage points compared to a single-layer ESN.
