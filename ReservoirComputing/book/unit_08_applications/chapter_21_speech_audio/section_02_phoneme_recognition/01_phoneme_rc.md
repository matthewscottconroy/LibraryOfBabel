# Section 21.2: Phoneme Recognition with Reservoir Computing

## 21.2.1 The Phoneme Recognition Problem

Phoneme recognition — identifying the phoneme category of each speech frame — is a fundamental problem in automatic speech recognition. It serves both as a standalone application (for phoneme-level analysis, language identification, speech synthesis quality assessment) and as a component of larger ASR systems.

The input is a sequence of acoustic feature vectors $\mathbf{u}(1), \mathbf{u}(2), \ldots, \mathbf{u}(T)$ (e.g., 39-dimensional MFCC + delta frames at 100 Hz). The output is a sequence of phoneme labels $y(1), y(2), \ldots, y(T)$, one per frame, from a phoneme inventory of size $K$ (typically 39–48 phonemes for English plus silence).

This is a sequence-to-sequence classification problem with two key challenges:

**Context dependence**: The acoustic realization of a phoneme depends heavily on its context (neighboring phonemes, stress, speaking rate). The phoneme /t/ sounds very different at word onset (/top/) versus word-medial position after a vowel (/letter/) versus word-final (/bit/). A phoneme recognizer must maintain sufficient temporal context to resolve these ambiguities.

**Co-articulation**: Speech is continuously produced — articulators move smoothly from one phoneme to the next, so the acoustic signal during one phoneme already anticipates the next. This means that the acoustic features at any frame reflect not just the current phoneme but a window of several phonemes around it.

Both challenges play to the strengths of reservoir computing: fading memory provides the temporal context needed for disambiguation, and the high-dimensional nonlinear state provides the separation needed for context-sensitive classification.

## 21.2.2 Benchmark Datasets

### FSDD: Free Spoken Digit Dataset

The Free Spoken Digit Dataset [Jackson2018FSDD] is an open-source collection of spoken English digits (0–9) recorded by multiple speakers. As of 2024, it contains:
- 3,000 recordings (6 speakers × 10 digits × 50 repetitions)
- 8 kHz mono WAV format
- Typical utterance duration: 0.5–1.5 s

FSDD is a good entry-level benchmark: small enough to train quickly, large enough to reveal overfitting, and multi-speaker so it tests generalization across speaker variability. The standard evaluation protocol is leave-one-speaker-out cross-validation.

**Baseline performance** (as of 2023):
- MFCC + HMM: $\sim 96\%$ accuracy
- MFCC + ESN (N=500): $\sim 97\%$ accuracy
- MFCC + deep LSTM: $\sim 99\%$ accuracy
- Raw waveform + CNN: $\sim 99.5\%$ accuracy

### TI-46 Corpus

The TI-46 corpus [Liberman1993TI46] is a more demanding spoken digit dataset:
- 46 speakers (26 male, 20 female)
- 10 digits + 26 letters
- ~16 kHz sample rate
- 10 repetitions per speaker per token

The multi-speaker, high-variability nature of TI-46 makes it a more realistic benchmark. RC systems typically achieve 94–97% accuracy on the digit recognition subset, depending on feature preprocessing and reservoir size.

## 21.2.3 Reservoir Architecture for Speech

The standard ESN architecture for speech processing [VerstraetEtAl2006, SchrauwenEtAl2007] uses the following design:

**Input**: $\mathbf{u}(n) \in \mathbb{R}^{d_{\text{in}}}$, where $d_{\text{in}} = 13$ or $39$ (MFCCs ± deltas), fed at each frame $n$.

**Reservoir update**:
$$\mathbf{x}(n) = (1 - \alpha)\mathbf{x}(n-1) + \alpha \tanh(W_{\text{res}}\mathbf{x}(n-1) + W_{\text{in}}\mathbf{u}(n) + \mathbf{b})$$

where $\alpha \in (0,1]$ is the leaking rate, controlling the effective time constant of the reservoir neurons: $\tau_{\text{eff}} = \Delta t / \alpha$ where $\Delta t = 10$ ms is the frame period. For $\alpha = 0.1$, $\tau_{\text{eff}} = 100$ ms, matching the phoneme timescale.

**Reservoir parameters** for speech:
- $N = 500$–$2000$ neurons
- Spectral radius $\rho = 0.9$–$0.99$ (near-critical, for long memory)
- Input scaling $s_{\text{in}} = 0.1$–$1.0$
- Leaking rate $\alpha = 0.1$–$0.5$

**Readout**: For frame-level phoneme labeling, the output is:

$$\hat{y}(n) = \arg\max_k \left(\mathbf{w}_k^\top \mathbf{x}(n)\right)$$

where $\mathbf{w}_k \in \mathbb{R}^N$ is the learned weight vector for phoneme class $k$. All $K$ weight vectors are stacked into the matrix $W_{\text{out}} \in \mathbb{R}^{K \times N}$.

Training minimizes the mean squared error (or cross-entropy) between the one-hot target vector $\mathbf{t}(n)$ and the soft output $\hat{\mathbf{y}}(n) = \text{softmax}(W_{\text{out}} \mathbf{x}(n))$:

$$W_{\text{out}} = \arg\min_{W} \sum_n \|\mathbf{t}(n) - W\mathbf{x}(n)\|^2 + \lambda \|W\|_F^2$$

This is a standard ridge regression problem, solved in closed form.

### Utterance-Level Classification

For digit recognition (classify the entire utterance rather than each frame), the per-frame outputs must be aggregated. Common approaches:

1. **Majority vote**: $\hat{y}_{\text{utterance}} = \arg\max_k \sum_n \mathbb{1}[\hat{y}(n) = k]$
2. **Max pooling**: $\hat{y}_{\text{utterance}} = \arg\max_k \max_n \hat{y}_k(n)$  
3. **Mean pooling**: $\hat{y}_{\text{utterance}} = \arg\max_k \frac{1}{T}\sum_n \hat{y}_k(n)$
4. **Final state**: $\hat{y}_{\text{utterance}} = \arg\max_k \hat{y}_k(T)$ (classify based on end-of-utterance state)

For isolated digit recognition, mean pooling typically performs best [VerstraetEtAl2006]. The final-state approach is efficient but sensitive to utterance length variation.

## 21.2.4 Python Implementation: Spoken Digit ESN Classifier

The following complete implementation trains an ESN on the FSDD dataset for spoken digit recognition.

```python
"""
Spoken Digit Recognition with Echo State Networks
Benchmarks on Free Spoken Digit Dataset (FSDD)

Requirements:
    pip install reservoirpy librosa scikit-learn soundfile
    
Dataset:
    git clone https://github.com/Jakobovski/free-spoken-digit-dataset.git
    (or: pip install free-spoken-digit-dataset)
"""

import os
import numpy as np
import librosa
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import accuracy_score, confusion_matrix
import matplotlib.pyplot as plt
from typing import List, Tuple, Optional


# ─── Feature Extraction ────────────────────────────────────────────────────

def extract_mfcc(
    audio_path: str,
    sr: int = 8000,
    n_mfcc: int = 13,
    n_fft: int = 512,
    hop_length: int = 80,   # 10 ms at 8 kHz
    win_length: int = 200,  # 25 ms at 8 kHz
    n_mels: int = 40,
    include_delta: bool = True,
    include_delta2: bool = True,
) -> np.ndarray:
    """
    Extract MFCC features from an audio file.
    
    Returns
    -------
    features : ndarray, shape (T, d)
        T frames of d-dimensional feature vectors.
        d = n_mfcc * (1 + include_delta + include_delta2)
    """
    y, sr_loaded = librosa.load(audio_path, sr=sr, mono=True)
    
    # Extract MFCCs
    mfcc = librosa.feature.mfcc(
        y=y, sr=sr, n_mfcc=n_mfcc, n_fft=n_fft,
        hop_length=hop_length, win_length=win_length, n_mels=n_mels
    )  # shape: (n_mfcc, T)
    
    features = [mfcc]
    
    if include_delta:
        delta = librosa.feature.delta(mfcc, order=1)
        features.append(delta)
    if include_delta2:
        delta2 = librosa.feature.delta(mfcc, order=2)
        features.append(delta2)
    
    features = np.vstack(features)  # (d, T)
    
    # Cepstral mean normalization (per utterance)
    features = features - features.mean(axis=1, keepdims=True)
    
    return features.T  # (T, d)


def load_fsdd_dataset(
    data_dir: str,
    speakers: Optional[List[str]] = None,
) -> Tuple[List[np.ndarray], List[int], List[str]]:
    """
    Load the Free Spoken Digit Dataset.
    
    Parameters
    ----------
    data_dir : path to recordings/ directory of FSDD
    speakers : list of speaker IDs to include (None = all)
    
    Returns
    -------
    features : list of (T_i, d) arrays, one per utterance
    labels   : list of digit labels (0-9)
    speaker_ids : list of speaker identifiers
    """
    features_list, labels, speaker_ids = [], [], []
    
    # FSDD filename format: {digit}_{speaker}_{index}.wav
    for fname in sorted(os.listdir(data_dir)):
        if not fname.endswith('.wav'):
            continue
        parts = fname.replace('.wav', '').split('_')
        digit = int(parts[0])
        speaker = parts[1]
        
        if speakers is not None and speaker not in speakers:
            continue
        
        fpath = os.path.join(data_dir, fname)
        feats = extract_mfcc(fpath)
        
        features_list.append(feats)
        labels.append(digit)
        speaker_ids.append(speaker)
    
    return features_list, labels, speaker_ids


# ─── Echo State Network ─────────────────────────────────────────────────────

class EchoStateNetwork:
    """
    Echo State Network for sequence classification.
    
    Parameters
    ----------
    n_reservoir : int
        Number of reservoir neurons.
    spectral_radius : float
        Target spectral radius of W_res. Should be < 1 (usually 0.9–0.99).
    input_scaling : float
        Scaling of W_in. Controls input influence.
    leaking_rate : float
        Leaking rate alpha in [0,1]. Smaller = longer time constant.
    bias_scaling : float
        Scaling of bias vector.
    ridge_alpha : float
        L2 regularization for readout regression.
    sparsity : float
        Fraction of reservoir weights set to zero (0 = fully connected).
    seed : int
        Random seed for reproducibility.
    """
    
    def __init__(
        self,
        n_reservoir: int = 500,
        spectral_radius: float = 0.95,
        input_scaling: float = 0.5,
        leaking_rate: float = 0.3,
        bias_scaling: float = 0.1,
        ridge_alpha: float = 1e-4,
        sparsity: float = 0.9,
        seed: int = 42,
    ):
        self.N = n_reservoir
        self.rho = spectral_radius
        self.s_in = input_scaling
        self.alpha = leaking_rate
        self.s_bias = bias_scaling
        self.ridge = ridge_alpha
        self.sparsity = sparsity
        self.rng = np.random.RandomState(seed)
        
        self.W_res = None   # initialized on first call
        self.W_in = None
        self.bias = None
        self.W_out = None
        self.scaler = StandardScaler()
        
    def _initialize_reservoir(self, n_inputs: int) -> None:
        """Initialize weight matrices for given input dimension."""
        N = self.N
        
        # Sparse random reservoir weights
        W = self.rng.randn(N, N)
        mask = self.rng.rand(N, N) < self.sparsity
        W[mask] = 0.0
        
        # Scale to target spectral radius
        eigenvalues = np.linalg.eigvals(W)
        current_rho = np.max(np.abs(eigenvalues))
        if current_rho > 1e-10:
            W = W * (self.rho / current_rho)
        self.W_res = W
        
        # Input weights (dense, random sign)
        self.W_in = self.s_in * self.rng.randn(N, n_inputs)
        
        # Bias
        self.bias = self.s_bias * self.rng.randn(N)
    
    def _drive_reservoir(
        self,
        U: np.ndarray,
        washout: int = 10,
    ) -> np.ndarray:
        """
        Drive reservoir with input sequence U.
        
        Parameters
        ----------
        U : ndarray, shape (T, d_in)
        washout : int
            Number of initial steps to discard.
        
        Returns
        -------
        X : ndarray, shape (T - washout, N)
        """
        T, d_in = U.shape
        if self.W_in is None or self.W_in.shape[1] != d_in:
            self._initialize_reservoir(d_in)
        
        x = np.zeros(self.N)
        states = []
        
        for t in range(T):
            pre = self.W_res @ x + self.W_in @ U[t] + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            if t >= washout:
                states.append(x.copy())
        
        return np.array(states)  # (T - washout, N)
    
    def _get_utterance_repr(
        self,
        U: np.ndarray,
        method: str = 'mean',
    ) -> np.ndarray:
        """
        Collapse sequence of reservoir states to single feature vector.
        
        method : 'mean', 'final', 'max', or 'concat_stats'
        """
        states = self._drive_reservoir(U)  # (T', N)
        
        if method == 'mean':
            return states.mean(axis=0)
        elif method == 'final':
            return states[-1]
        elif method == 'max':
            return states.max(axis=0)
        elif method == 'concat_stats':
            # Concatenate mean and std — often the best single-vector repr
            return np.concatenate([states.mean(axis=0), states.std(axis=0)])
        else:
            raise ValueError(f"Unknown aggregation method: {method}")
    
    def fit(
        self,
        X_train: List[np.ndarray],
        y_train: List[int],
        aggregation: str = 'concat_stats',
    ) -> 'EchoStateNetwork':
        """Train the ESN readout on a list of variable-length sequences."""
        # Preprocess: normalize features globally
        all_frames = np.vstack(X_train)
        self.scaler.fit(all_frames)
        
        # Collect reservoir representations
        reprs = []
        for U in X_train:
            U_scaled = self.scaler.transform(U)
            r = self._get_utterance_repr(U_scaled, method=aggregation)
            reprs.append(r)
        
        R = np.array(reprs)          # (n_train, N_feat)
        y = np.array(y_train)        # (n_train,)
        n_classes = len(np.unique(y))
        
        # One-hot encode targets
        Y = np.zeros((len(y), n_classes))
        for i, label in enumerate(y):
            Y[i, label] = 1.0
        
        # Ridge regression: W_out = (R^T R + lambda I)^{-1} R^T Y
        A = R.T @ R + self.ridge * np.eye(R.shape[1])
        b = R.T @ Y
        self.W_out = np.linalg.solve(A, b).T  # (n_classes, N_feat)
        self._aggregation = aggregation
        
        return self
    
    def predict(self, X_test: List[np.ndarray]) -> np.ndarray:
        """Classify a list of variable-length sequences."""
        if self.W_out is None:
            raise RuntimeError("Model not trained. Call fit() first.")
        
        preds = []
        for U in X_test:
            U_scaled = self.scaler.transform(U)
            r = self._get_utterance_repr(U_scaled, method=self._aggregation)
            scores = self.W_out @ r          # (n_classes,)
            preds.append(np.argmax(scores))
        
        return np.array(preds)
    
    def score(self, X_test: List[np.ndarray], y_test: List[int]) -> float:
        """Return classification accuracy."""
        return accuracy_score(y_test, self.predict(X_test))


# ─── Evaluation ─────────────────────────────────────────────────────────────

def leave_one_speaker_out_eval(
    features: List[np.ndarray],
    labels: List[int],
    speaker_ids: List[str],
    esn_kwargs: Optional[dict] = None,
) -> Tuple[float, np.ndarray]:
    """
    Leave-one-speaker-out cross-validation on FSDD.
    
    Returns
    -------
    mean_accuracy : float
    confusion : ndarray, shape (10, 10), summed over folds
    """
    if esn_kwargs is None:
        esn_kwargs = {}
    
    unique_speakers = sorted(set(speaker_ids))
    n_classes = 10
    all_preds, all_true = [], []
    
    print(f"Leave-one-speaker-out CV over {len(unique_speakers)} speakers")
    
    for hold_out in unique_speakers:
        # Split
        train_idx = [i for i, s in enumerate(speaker_ids) if s != hold_out]
        test_idx  = [i for i, s in enumerate(speaker_ids) if s == hold_out]
        
        X_tr = [features[i] for i in train_idx]
        y_tr = [labels[i]   for i in train_idx]
        X_te = [features[i] for i in test_idx]
        y_te = [labels[i]   for i in test_idx]
        
        # Train and evaluate
        esn = EchoStateNetwork(**esn_kwargs)
        esn.fit(X_tr, y_tr)
        preds = esn.predict(X_te)
        acc = accuracy_score(y_te, preds)
        
        print(f"  Speaker {hold_out}: {acc*100:.1f}% accuracy "
              f"({sum(y == p for y, p in zip(y_te, preds))}/{len(y_te)})")
        
        all_preds.extend(preds)
        all_true.extend(y_te)
    
    overall_acc = accuracy_score(all_true, all_preds)
    cm = confusion_matrix(all_true, all_preds, labels=list(range(n_classes)))
    
    print(f"\nOverall accuracy: {overall_acc*100:.1f}%")
    return overall_acc, cm


def hyperparameter_search(
    features: List[np.ndarray],
    labels: List[int],
    speaker_ids: List[str],
    n_trials: int = 20,
    seed: int = 0,
) -> dict:
    """
    Random hyperparameter search for ESN on FSDD.
    Best parameters are returned.
    """
    rng = np.random.RandomState(seed)
    
    param_grid = {
        'n_reservoir':    [200, 500, 1000, 2000],
        'spectral_radius': np.linspace(0.7, 0.99, 15),
        'input_scaling':   np.logspace(-2, 0, 10),
        'leaking_rate':    np.linspace(0.1, 0.9, 9),
        'ridge_alpha':     np.logspace(-6, 0, 7),
    }
    
    best_acc, best_params = 0.0, {}
    
    # Use only 2 speakers for fast hyperparameter search
    hp_speakers = list(set(speaker_ids))[:3]
    mask = [s in hp_speakers for s in speaker_ids]
    hp_feats = [f for f, m in zip(features, mask) if m]
    hp_labs  = [l for l, m in zip(labels,   mask) if m]
    hp_spkrs = [s for s, m in zip(speaker_ids, mask) if m]
    
    for trial in range(n_trials):
        params = {k: rng.choice(v) for k, v in param_grid.items()}
        params['seed'] = trial
        
        try:
            acc, _ = leave_one_speaker_out_eval(
                hp_feats, hp_labs, hp_spkrs,
                esn_kwargs=params
            )
            if acc > best_acc:
                best_acc = acc
                best_params = params
                print(f"  Trial {trial}: new best {acc*100:.1f}%: {params}")
        except Exception as e:
            print(f"  Trial {trial} failed: {e}")
    
    print(f"\nBest params (acc={best_acc*100:.1f}%): {best_params}")
    return best_params


# ─── Main ────────────────────────────────────────────────────────────────────

def main():
    # Adjust this path to your FSDD recordings directory
    data_dir = "./free-spoken-digit-dataset/recordings"
    
    if not os.path.exists(data_dir):
        print(f"Dataset not found at {data_dir}")
        print("Please clone: github.com/Jakobovski/free-spoken-digit-dataset")
        print("\nRunning with synthetic demonstration data instead...")
        _demo_synthetic()
        return
    
    print("Loading FSDD dataset...")
    features, labels, speaker_ids = load_fsdd_dataset(data_dir)
    print(f"Loaded {len(features)} utterances from "
          f"{len(set(speaker_ids))} speakers")
    
    # ESN with default (decent) hyperparameters
    esn_params = {
        'n_reservoir':    500,
        'spectral_radius': 0.95,
        'input_scaling':   0.3,
        'leaking_rate':    0.3,
        'ridge_alpha':     1e-4,
        'sparsity':        0.9,
        'seed':            42,
    }
    
    print("\nRunning leave-one-speaker-out evaluation...")
    acc, cm = leave_one_speaker_out_eval(
        features, labels, speaker_ids,
        esn_kwargs=esn_params
    )
    
    # Plot confusion matrix
    fig, ax = plt.subplots(figsize=(8, 7))
    im = ax.imshow(cm, cmap='Blues')
    ax.set_xticks(range(10))
    ax.set_yticks(range(10))
    ax.set_xlabel("Predicted digit")
    ax.set_ylabel("True digit")
    ax.set_title(f"ESN spoken digit recognition\n"
                 f"Overall accuracy: {acc*100:.1f}%")
    plt.colorbar(im, ax=ax)
    plt.tight_layout()
    plt.savefig("confusion_matrix_fsdd_esn.png", dpi=150)
    print("Confusion matrix saved to confusion_matrix_fsdd_esn.png")


def _demo_synthetic():
    """
    Demonstrate the ESN architecture on synthetic sinusoidal signals,
    one frequency per 'digit' class.
    """
    print("Generating synthetic classification dataset (10 frequency classes)...")
    np.random.seed(42)
    
    n_per_class = 50
    n_classes = 10
    T = 100    # frames per utterance
    d_in = 13  # feature dimension
    
    features, labels = [], []
    for k in range(n_classes):
        for _ in range(n_per_class):
            # Each class is a sinusoid at a different frequency + noise
            t = np.linspace(0, 2*np.pi, T)
            base = np.sin((k + 1) * t)
            feat = np.tile(base[:, None], (1, d_in))
            feat += 0.5 * np.random.randn(T, d_in)
            features.append(feat)
            labels.append(k)
    
    # Train/test split (80/20)
    n_total = len(features)
    idx = np.random.permutation(n_total)
    n_train = int(0.8 * n_total)
    
    X_tr = [features[i] for i in idx[:n_train]]
    y_tr = [labels[i]   for i in idx[:n_train]]
    X_te = [features[i] for i in idx[n_train:]]
    y_te = [labels[i]   for i in idx[n_train:]]
    
    esn = EchoStateNetwork(n_reservoir=200, spectral_radius=0.95,
                           input_scaling=0.5, leaking_rate=0.3)
    esn.fit(X_tr, y_tr)
    acc = esn.score(X_te, y_te)
    print(f"Synthetic 10-class accuracy: {acc*100:.1f}%")


if __name__ == "__main__":
    main()
```

## 21.2.5 Analysis of Architecture Choices

The implementation above embodies several design decisions that warrant explanation:

### Why `concat_stats` aggregation?

Concatenating the mean and standard deviation of the reservoir state over the utterance provides a simple summary of the entire trajectory. The mean captures average activation (correlated with phoneme identity and frequency), while the std captures the variability of the trajectory (correlated with the diversity of sounds). Together, they provide a richer representation than either alone. For digit recognition, this typically outperforms mean pooling by 1–2 percentage points.

For more sophisticated temporal aggregation, one can use the final state (good for tasks where the critical information accumulates at the end), max pooling (good for tasks with a key distinctive event), or a learned aggregation (by treating the frame-level states as the input to a second classifier — at the cost of a more complex readout).

### Reservoir Size vs. Performance

Empirically, accuracy on FSDD scales roughly logarithmically with reservoir size:

| $N$ | NMSE (synthetic) | Estimated FSDD accuracy |
|---|---|---|
| 100 | — | $\sim 90\%$ |
| 500 | — | $\sim 96\%$ |
| 1000 | — | $\sim 97\%$ |
| 2000 | — | $\sim 97.5\%$ |

Returns diminish rapidly above $N = 500$–$1000$. The computational cost scales as $O(N^2)$ for reservoir state updates (dominated by the matrix-vector product $W_{\text{res}}\mathbf{x}$), so the practical optimum is usually $N \approx 500$ for a standard single-machine implementation.

### Competing Approaches

For context, the FSDD benchmark performance of competing approaches (as of 2024):

| Method | FSDD Accuracy | Training Data Required |
|---|---|---|
| ESN (N=500, MFCC) | $\sim 97\%$ | 3000 utterances |
| DTW + 1-NN (MFCC) | $\sim 95\%$ | 3000 utterances |
| HMM (Gaussian mixture) | $\sim 96\%$ | 3000 utterances |
| LSTM (small, 64 units) | $\sim 98.5\%$ | 3000 utterances |
| Whisper (large) | $\sim 99.9\%$ | 680,000 hours |

The ESN is competitive with traditional methods (DTW, HMM) while requiring only linear training, and competitive with small LSTMs despite having no trained recurrent weights. The Whisper comparison illustrates the data efficiency advantage: the ESN achieves 97% with 3000 utterances versus Whisper's 99.9% with 680,000 hours of training data.
