# Convolutional Neural Networks for Biological Sequences

The genome is written in an alphabet of four letters, but its meaning is not encoded letter by letter. It is encoded in patterns — short stretches of DNA that transcription factors recognize, splice signals that ribosomes detect, codon contexts that influence translation efficiency. These patterns are local, and they can appear anywhere along the chromosome. This is precisely the kind of structure that 1D Convolutional Neural Networks are built to exploit: they scan a sequence with learned filters, detecting local patterns regardless of where they appear.

DNA and RNA sequences are discrete, one-dimensional, and carry biological information through local patterns (binding motifs, splice signals, codon context) that can appear at any position. **1D Convolutional Neural Networks (CNNs)** are architecturally matched to this structure: they learn position-invariant filters that detect local sequence patterns, stacked into hierarchical representations that capture increasingly complex regulatory logic.

## The 1D Convolution Operation

A **convolutional filter** (kernel) of length $k$ and $c_{\text{in}}$ input channels is a weight tensor $W \in \mathbb{R}^{c_{\text{out}} \times c_{\text{in}} \times k}$. Applied to a sequence of length $L$ with $c_{\text{in}}$ channels, it produces an output of length $L - k + 1$ per filter. For DNA, the input is typically **one-hot encoded**: a matrix of shape $(4, L)$ where the four rows correspond to A, C, G, T at each position.

$$y_j = \sum_{c=1}^{c_{\text{in}}} \sum_{i=0}^{k-1} W_{c,i} \cdot x_{c, j+i} + b$$

**Biological interpretation:** A filter that learns to detect the TATAAA motif will have high weights for T at position 0, A at 1, T at 2, A at 3, A at 4, A at 5. Its output $y_j$ will be large when the motif is present starting at position $j$.

## CNN Architecture for Splice Site Prediction

```python
import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np

class SpliceSiteCNN(nn.Module):
    """
    CNN for splice site classification.
    Predicts donor/acceptor/non-site from 400 nt sequence windows.
    Architecture inspired by SpliceAI (Jaganathan et al. 2019).
    """
    def __init__(self, n_filters=64, kernel_sizes=(4, 8, 16), seq_len=400):
        super().__init__()
        
        # Multi-scale convolutional filters capture motifs of different lengths
        self.conv_branches = nn.ModuleList([
            nn.Conv1d(4, n_filters, k, padding=k//2)  # 4 input channels (one-hot)
            for k in kernel_sizes
        ])
        
        # After concatenating multi-scale features: 3*n_filters channels
        in_channels = n_filters * len(kernel_sizes)
        
        self.residual_blocks = nn.Sequential(
            ResidualBlock(in_channels, 128, kernel_size=8),
            ResidualBlock(128, 64, kernel_size=8),
        )
        
        self.global_pool = nn.AdaptiveMaxPool1d(1)  # max over sequence positions
        
        self.classifier = nn.Sequential(
            nn.Linear(64, 32),
            nn.ReLU(),
            nn.Dropout(0.3),
            nn.Linear(32, 3)  # donor / acceptor / non-site
        )
    
    def forward(self, x):
        # x: (batch, 4, seq_len) — one-hot encoded
        
        # Multi-scale convolution
        branch_outputs = []
        for conv in self.conv_branches:
            out = F.relu(conv(x))           # (batch, n_filters, seq_len)
            branch_outputs.append(out)
        
        x = torch.cat(branch_outputs, dim=1)   # (batch, 3*n_filters, seq_len)
        x = self.residual_blocks(x)             # (batch, 64, seq_len)
        x = self.global_pool(x).squeeze(-1)    # (batch, 64)
        return self.classifier(x)              # (batch, 3)

class ResidualBlock(nn.Module):
    """Residual block with skip connection — critical for deep networks."""
    def __init__(self, in_channels, out_channels, kernel_size=8):
        super().__init__()
        padding = kernel_size // 2
        self.conv1 = nn.Conv1d(in_channels, out_channels, kernel_size, padding=padding)
        self.bn1 = nn.BatchNorm1d(out_channels)
        self.conv2 = nn.Conv1d(out_channels, out_channels, kernel_size, padding=padding)
        self.bn2 = nn.BatchNorm1d(out_channels)
        
        # Projection shortcut if dimensions change
        self.shortcut = (nn.Conv1d(in_channels, out_channels, 1)
                         if in_channels != out_channels else nn.Identity())
    
    def forward(self, x):
        identity = self.shortcut(x)
        out = F.relu(self.bn1(self.conv1(x)))
        out = self.bn2(self.conv2(out))
        return F.relu(out + identity)  # skip connection adds gradient flow

# DNA sequence encoding
def one_hot_encode(sequence, max_len=400):
    """Encode DNA sequence as (4, L) one-hot matrix."""
    nucleotide_map = {'A': 0, 'C': 1, 'G': 2, 'T': 3}
    L = min(len(sequence), max_len)
    encoding = np.zeros((4, max_len), dtype=np.float32)
    for i, nt in enumerate(sequence[:L]):
        if nt in nucleotide_map:
            encoding[nucleotide_map[nt], i] = 1.0
    return encoding

# Simulate splice site data
def generate_splice_data(n_pos=1000, n_neg=2000, seq_len=400, seed=42):
    rng = np.random.default_rng(seed)
    sequences, labels = [], []
    
    # Positive donor sites: contain GT at center (consensus: AG|GTAAGT)
    for _ in range(n_pos):
        seq = list(rng.choice(list('ACGT'), size=seq_len))
        center = seq_len // 2
        seq[center:center+2] = ['G', 'T']    # GT donor dinucleotide
        seq[center+2:center+4] = ['A', 'A']  # AAG context
        sequences.append(''.join(seq))
        labels.append(0)  # donor
    
    # Negative: random sequences
    for _ in range(n_neg):
        sequences.append(''.join(rng.choice(list('ACGT'), size=seq_len)))
        labels.append(2)  # non-site
    
    X = np.array([one_hot_encode(s, seq_len) for s in sequences])
    y = np.array(labels)
    return torch.FloatTensor(X), torch.LongTensor(y)

X, y = generate_splice_data()
print(f"Data shape: {X.shape}, labels: {y.shape}")

# Training
model = SpliceSiteCNN(n_filters=64, kernel_sizes=(4, 8, 16), seq_len=400)
optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)

# Count parameters
n_params = sum(p.numel() for p in model.parameters())
print(f"Model parameters: {n_params:,}")

# One forward pass
model.eval()
with torch.no_grad():
    logits = model(X[:16])
    print(f"Output shape: {logits.shape}")   # (16, 3)
    probs = torch.softmax(logits, dim=-1)
    print(f"Class probabilities: {probs[0].numpy()}")
```

## Visualizing What CNNs Learn

A key advantage of CNNs over SVMs is that learned filters can be visualized and compared to known binding motifs:

```python
import matplotlib.pyplot as plt
import numpy as np

def extract_filter_motifs(model, X, layer_name='conv_branches', 
                           filter_idx=0, n_seqs=500, threshold=0.5):
    """
    Extract sequences that maximally activate a given filter.
    Compare to known TF binding motifs (JASPAR/HOCOMOCO).
    """
    model.eval()
    max_activations = []
    max_sequences = []
    
    with torch.no_grad():
        # Get filter output for the first conv branch
        conv = model.conv_branches[0]
        for i in range(0, min(n_seqs, len(X)), 32):
            batch = X[i:i+32]
            act = conv(batch)  # (batch, n_filters, seq_len)
            
            # Find position of max activation for filter_idx
            filter_acts = act[:, filter_idx, :]  # (batch, seq_len)
            for j, fa in enumerate(filter_acts):
                max_pos = fa.argmax().item()
                max_val = fa.max().item()
                if max_val > threshold:
                    # Extract 8-nt window around max activation position
                    k = 8  # filter size
                    start = max(0, max_pos)
                    end = min(X.shape[2], start + k)
                    seq_onehot = batch[j, :, start:end].numpy()
                    max_activations.append(max_val)
                    max_sequences.append(seq_onehot)
    
    if not max_sequences:
        return None
    
    # Average over top activating sequences = position weight matrix
    top_n = 50
    sorted_idx = np.argsort(max_activations)[-top_n:]
    pwm = np.mean([max_sequences[i] for i in sorted_idx], axis=0)  # (4, k)
    
    return pwm

# Compute PWM for filter 0
pwm = extract_filter_motifs(model, X, filter_idx=0)
if pwm is not None:
    fig, ax = plt.subplots(figsize=(8, 3))
    im = ax.imshow(pwm, cmap='RdBu_r', vmin=0, vmax=1)
    ax.set_yticks([0, 1, 2, 3])
    ax.set_yticklabels(['A', 'C', 'G', 'T'])
    ax.set_xlabel('Position')
    ax.set_title('Learned motif (filter 0)')
    plt.colorbar(im, ax=ax)
    plt.tight_layout()
    plt.savefig('cnn_filter_motif.pdf')
```

## Saliency Maps: Position-Level Attribution

```python
def compute_saliency(model, x, target_class=0):
    """
    Compute saliency map: gradient of output w.r.t. input sequence.
    High gradient = position is important for prediction.
    """
    x_input = x.unsqueeze(0).requires_grad_(True)
    model.eval()
    logit = model(x_input)[0, target_class]
    logit.backward()
    saliency = x_input.grad.abs().squeeze(0)  # (4, seq_len)
    return saliency.max(0).values.detach().numpy()  # max over nucleotides

# For a correctly predicted donor site
donor_seq = X[0]
saliency = compute_saliency(model, donor_seq, target_class=0)

fig, axes = plt.subplots(2, 1, figsize=(12, 5))
axes[0].imshow(donor_seq.numpy(), aspect='auto', cmap='Blues')
axes[0].set_yticks([0, 1, 2, 3])
axes[0].set_yticklabels(['A', 'C', 'G', 'T'])
axes[0].set_title('Input sequence (one-hot)')

axes[1].bar(range(len(saliency)), saliency, color='steelblue')
axes[1].axvline(200, color='red', linestyle='--', label='GT donor position')
axes[1].set_xlabel('Sequence position')
axes[1].set_ylabel('Saliency (gradient magnitude)')
axes[1].set_title('Saliency map — important positions for donor prediction')
axes[1].legend()
plt.tight_layout()
plt.savefig('saliency_map.pdf')
```

## Why This Matters

CNNs were the first deep learning architecture to achieve superhuman performance on biological sequence tasks. DeepBind (2015) showed that 1D CNNs on raw sequence outperformed JASPAR motif-matching for TF binding prediction. SpliceAI (2019) used a deep residual CNN to predict variant effects on splicing with clinical-grade accuracy. Enformer (2021) uses a dilated CNN followed by a transformer to predict gene expression from 200 kb sequence context. Understanding 1D CNNs — how they encode sequences, what their filters represent, and how to visualize what they learn — is essential for working with any of these tools or extending them to new problems.
