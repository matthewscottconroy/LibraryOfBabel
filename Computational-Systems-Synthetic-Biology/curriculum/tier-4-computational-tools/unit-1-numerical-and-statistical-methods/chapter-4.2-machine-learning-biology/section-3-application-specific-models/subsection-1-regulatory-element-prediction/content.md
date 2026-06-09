# Regulatory Element Prediction

The human genome contains roughly 20,000 protein-coding genes, but the sequence that determines when and where those genes are expressed — the regulatory genome — is spread across hundreds of thousands of non-coding elements: promoters that position RNA polymerase, enhancers that can act from tens of thousands of base pairs away, silencers that repress expression in specific tissues, and transcription factor binding sites that integrate signals from dozens of pathways. Most of the genetic variants that cause disease map to this regulatory genome, not to protein-coding sequences. Yet for most of the history of genomics, we had almost no tools to decode it.

**Regulatory elements** — promoters, enhancers, silencers, insulators, and transcription factor binding sites (TFBSs) — are non-coding DNA sequences that control when, where, and how much a gene is expressed. Computational prediction of regulatory elements from sequence is both a fundamental problem in genomics and a rapidly evolving area where machine learning has dramatically outperformed earlier rule-based approaches.

## From PWMs to Deep Learning

The traditional approach to TFBS prediction uses **Position Weight Matrices (PWMs)**: a $4 \times k$ matrix where entry $M_{b,i}$ is the log-ratio of the frequency of base $b$ at position $i$ in known binding sites versus a background model. Scanning a sequence with a PWM gives a binding score at each position.

Limitations:
- Assumes independence between positions (ignores inter-position correlations)
- Requires known binding sites for training
- Cannot capture variable-length motifs or complex combinatorial logic

**DeepBind** (Alipanahi et al. 2015, Nature Biotechnology) replaced PWMs with 1D CNNs trained on SELEX-seq data, capturing position-dependent nucleotide interactions and achieving substantially better performance.

```python
import torch
import torch.nn as nn
import numpy as np
from itertools import product

# Position Weight Matrix scanning (traditional approach)
def scan_pwm(sequence, pwm, background={'A':0.25,'C':0.25,'G':0.25,'T':0.25}):
    """
    Scan sequence with a PWM, return position scores.
    pwm: dict mapping position -> {A: freq, C: freq, G: freq, T: freq}
    """
    k = len(pwm)
    scores = []
    for i in range(len(sequence) - k + 1):
        score = 0
        for j, base in enumerate(sequence[i:i+k]):
            if base in pwm[j]:
                score += np.log(pwm[j][base] / background.get(base, 0.25) + 1e-10)
        scores.append(score)
    return np.array(scores)

# Example: TATA box PWM
tata_pwm = {
    0: {'T': 0.82, 'A': 0.08, 'C': 0.06, 'G': 0.04},
    1: {'A': 0.97, 'T': 0.01, 'C': 0.01, 'G': 0.01},
    2: {'T': 0.93, 'A': 0.03, 'C': 0.02, 'G': 0.02},
    3: {'A': 0.92, 'T': 0.04, 'C': 0.02, 'G': 0.02},
    4: {'A': 0.73, 'T': 0.11, 'C': 0.09, 'G': 0.07},
    5: {'A': 0.83, 'T': 0.07, 'C': 0.06, 'G': 0.04},
}

test_seq = "ACGATCTATAAAGATCGATCG"
scores = scan_pwm(test_seq, tata_pwm)
max_pos = np.argmax(scores)
print(f"PWM max score: {scores[max_pos]:.2f} at position {max_pos}")
print(f"Matching sequence: {test_seq[max_pos:max_pos+6]}")

# CNN-based TFBS predictor (DeepBind style)
class DeepBind(nn.Module):
    """
    CNN for TF binding site prediction from sequence.
    Input: one-hot encoded sequence (4, L)
    Output: binding score (scalar)
    """
    def __init__(self, n_filters=32, filter_width=24, seq_len=101):
        super().__init__()
        
        self.conv = nn.Conv1d(4, n_filters, filter_width)
        self.pool = nn.MaxPool1d(kernel_size=seq_len - filter_width + 1)  # global max
        
        self.classifier = nn.Sequential(
            nn.Flatten(),
            nn.Linear(n_filters, 32),
            nn.ReLU(),
            nn.Dropout(0.5),
            nn.Linear(32, 1)
        )
    
    def forward(self, x):
        """x: (batch, 4, seq_len)"""
        x = torch.relu(self.conv(x))           # (batch, n_filters, L-k+1)
        x = self.pool(x)                       # (batch, n_filters, 1)
        return self.classifier(x).squeeze(-1)  # (batch,)
    
    def extract_motifs(self, X, threshold=0.5, n_seqs=200):
        """Extract sequence windows maximally activating each filter."""
        self.eval()
        motifs = []
        with torch.no_grad():
            acts = torch.relu(self.conv(X[:n_seqs]))  # (n, n_filters, L-k+1)
            for f in range(self.conv.out_channels):
                filter_acts = acts[:, f, :]
                max_vals, max_pos = filter_acts.max(dim=1)
                # Get sequences at max activation positions
                active_seqs = []
                for i, (pos, val) in enumerate(zip(max_pos, max_vals)):
                    if val > threshold:
                        start = pos.item()
                        window = X[i, :, start:start+self.conv.kernel_size[0]]
                        active_seqs.append(window.numpy())
                if active_seqs:
                    motifs.append(np.mean(active_seqs, axis=0))
        return motifs

# One-hot encoding for sequences
def one_hot_batch(sequences, max_len=101):
    mapping = {'A':0,'C':1,'G':2,'T':3}
    X = np.zeros((len(sequences), 4, max_len), dtype=np.float32)
    for i, seq in enumerate(sequences):
        for j, base in enumerate(seq[:max_len]):
            if base in mapping:
                X[i, mapping[base], j] = 1.0
    return torch.FloatTensor(X)

# Generate simulated CHIP-seq data
rng = np.random.default_rng(42)
n_pos, n_neg = 1000, 1000

def gen_sequences(n, motif=None, seq_len=101):
    seqs = []
    for _ in range(n):
        seq = list(rng.choice(list('ACGT'), size=seq_len))
        if motif:
            pos = rng.integers(40, 60)
            seq[pos:pos+len(motif)] = list(motif)
        seqs.append(''.join(seq))
    return seqs

pos_seqs = gen_sequences(n_pos, motif='TGAATCAG')  # simulated motif
neg_seqs = gen_sequences(n_neg, motif=None)

X_all = one_hot_batch(pos_seqs + neg_seqs)
y_all = torch.FloatTensor([1]*n_pos + [0]*n_neg)

# Train
model_db = DeepBind(n_filters=32, filter_width=8, seq_len=101)
optimizer = torch.optim.Adam(model_db.parameters(), lr=1e-3)
criterion = nn.BCEWithLogitsLoss()

for epoch in range(3):
    perm = torch.randperm(len(X_all))
    for i in range(0, len(X_all), 64):
        idx = perm[i:i+64]
        optimizer.zero_grad()
        pred = model_db(X_all[idx])
        loss = criterion(pred, y_all[idx])
        loss.backward()
        optimizer.step()
    print(f"Epoch {epoch+1}: loss = {loss.item():.4f}")
```

## Basenji and Enformer: Sequence-to-Expression Models

**Basenji** and its successor **Enformer** (Avsec et al. 2021, Nature Methods) take a fundamentally different approach: instead of predicting TF binding at individual sites, they predict genome-wide chromatin accessibility, histone modifications, and **gene expression** directly from 128 kb (Basenji) or 200 kb (Enformer) of DNA sequence.

The Enformer architecture:
1. Dilated 1D CNN tower (4 → 196 bp resolution)
2. Transformer encoder (self-attention over 128 bp bins)
3. Pointwise output head: 5,313 human and 1,643 mouse cell-type tracks

```python
# Using the Enformer model via Hugging Face
# pip install enformer-pytorch
from enformer_pytorch import Enformer, str_to_one_hot

model_enformer = Enformer.from_pretrained('EleutherAI/enformer-official-rough')

# Predict chromatin tracks for a 196,608 bp sequence
import torch

# One-hot encode sequence (must be exactly 196,608 bp)
sequence = "ACGT" * (196608 // 4)  # placeholder
seq_onehot = str_to_one_hot(sequence)  # (196608, 4)
seq_tensor = seq_onehot.unsqueeze(0).float()  # add batch dim

with torch.no_grad():
    predictions = model_enformer(seq_tensor)
# predictions['human']: (batch, 896, 5313) — 896 bins × 5313 tracks
print(f"Enformer output shape: {predictions['human'].shape}")

# Variant effect prediction: compare reference vs. alternative allele
def enformer_variant_effect(model, ref_seq, alt_seq, track_idx, center_bin=448):
    """
    Predict effect of single nucleotide variant on chromatin track.
    Compares center bins of reference vs. alternative predictions.
    """
    ref = str_to_one_hot(ref_seq).unsqueeze(0).float()
    alt = str_to_one_hot(alt_seq).unsqueeze(0).float()
    
    with torch.no_grad():
        pred_ref = model(ref)['human'][0, center_bin, track_idx]
        pred_alt = model(alt)['human'][0, center_bin, track_idx]
    
    return float(pred_alt - pred_ref)  # positive = increases chromatin signal
```

## Why This Matters

Regulatory element prediction is at the heart of understanding how genetic variants cause disease. The majority of disease-associated variants from GWAS studies fall in non-coding regulatory regions, not protein-coding genes. Models like Enformer, DeepSEA, and ChromDragoNN connect sequence variation to functional consequences — predicting which non-coding variants disrupt TF binding, alter chromatin accessibility, or change gene expression. This is essential for variant prioritization in clinical genetics, for understanding cis-regulatory evolution, and for designing synthetic regulatory elements in synthetic biology.
