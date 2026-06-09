# Recurrent Neural Networks and LSTMs

A protein is not just a collection of amino acids — it is a sequence with history. Whether a residue adopts a helical conformation depends not just on its own chemical identity, but on what came before it: the context stretching back dozens of positions in the primary sequence. The same is true of a gene's regulatory logic, a calcium imaging time series, or the translational context of a codon. How do you build a neural network that reads a sequence with memory?

**Recurrent Neural Networks (RNNs)** process sequences by maintaining a **hidden state** $\mathbf{h}_t$ that summarizes information from all previous positions. Unlike CNNs which see fixed-width windows, RNNs can in principle capture arbitrarily long-range dependencies. The **Long Short-Term Memory (LSTM)** architecture addresses the fundamental weakness of vanilla RNNs — the **vanishing gradient problem** — through a gating mechanism that selectively remembers and forgets information over long sequences.

## The Vanilla RNN and Its Failure Mode

A vanilla RNN updates its hidden state at each position:

$$\mathbf{h}_t = \tanh(W_h \mathbf{h}_{t-1} + W_x \mathbf{x}_t + \mathbf{b})$$

During backpropagation, the gradient of the loss with respect to an early hidden state $\mathbf{h}_1$ requires multiplying the weight matrix $W_h$ (approximately) $T-1$ times. If the largest eigenvalue of $W_h$ is less than 1, this product vanishes exponentially; if greater than 1, it explodes. In practice, vanilla RNNs cannot learn dependencies longer than ~10–20 steps.

## The LSTM Architecture

The **LSTM** (Hochreiter & Schmidhuber, 1997) adds a **cell state** $\mathbf{c}_t$ — a separate memory channel — controlled by three gates:

**Forget gate:** How much of the previous cell state to retain
$$\mathbf{f}_t = \sigma(W_f[\mathbf{h}_{t-1}, \mathbf{x}_t] + \mathbf{b}_f)$$

**Input gate:** What new information to write to the cell state
$$\mathbf{i}_t = \sigma(W_i[\mathbf{h}_{t-1}, \mathbf{x}_t] + \mathbf{b}_i)$$
$$\tilde{\mathbf{c}}_t = \tanh(W_c[\mathbf{h}_{t-1}, \mathbf{x}_t] + \mathbf{b}_c)$$

**Cell state update:**
$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

**Output gate:** What to expose as the hidden state
$$\mathbf{o}_t = \sigma(W_o[\mathbf{h}_{t-1}, \mathbf{x}_t] + \mathbf{b}_o)$$
$$\mathbf{h}_t = \mathbf{o}_t \odot \tanh(\mathbf{c}_t)$$

The **gradient highway** through the cell state allows gradients to flow without repeated matrix multiplication, enabling learning of dependencies over ~1,000 steps.

## Bidirectional LSTM for Sequence Labeling

For sequence tasks where context from both directions matters (e.g., secondary structure prediction where a residue's propensity depends on both N- and C-terminal neighbors), a **bidirectional LSTM** processes the sequence in both directions and concatenates the resulting hidden states:

```python
import torch
import torch.nn as nn
import numpy as np

class ProteinSecondaryStructurePredictor(nn.Module):
    """
    Bidirectional LSTM for per-residue secondary structure prediction.
    Input: protein sequence as integer indices (amino acid vocabulary)
    Output: per-residue 3-class prediction (H=helix, E=sheet, C=coil)
    """
    def __init__(self, vocab_size=20, embed_dim=64, hidden_dim=128, 
                 n_layers=2, n_classes=3, dropout=0.3):
        super().__init__()
        
        # Amino acid embedding: each AA -> dense vector
        self.embedding = nn.Embedding(vocab_size + 1, embed_dim, padding_idx=0)
        
        # Bidirectional LSTM: processes sequence L->R and R->L
        self.lstm = nn.LSTM(
            input_size=embed_dim,
            hidden_size=hidden_dim,
            num_layers=n_layers,
            batch_first=True,      # input shape: (batch, seq_len, features)
            bidirectional=True,    # concatenate forward + backward
            dropout=dropout if n_layers > 1 else 0
        )
        
        # Classification head: project from 2*hidden_dim to n_classes per position
        self.classifier = nn.Sequential(
            nn.Linear(2 * hidden_dim, 64),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(64, n_classes)
        )
    
    def forward(self, x, lengths=None):
        """
        x: (batch, seq_len) integer amino acid indices
        lengths: actual lengths (for variable-length sequences)
        Returns: (batch, seq_len, n_classes) per-residue logits
        """
        # Embed amino acids
        embedded = self.embedding(x)  # (batch, seq_len, embed_dim)
        
        # Pack padded sequences for efficient LSTM processing
        if lengths is not None:
            packed = nn.utils.rnn.pack_padded_sequence(
                embedded, lengths.cpu(), batch_first=True, enforce_sorted=False
            )
            lstm_out, _ = self.lstm(packed)
            lstm_out, _ = nn.utils.rnn.pad_packed_sequence(
                lstm_out, batch_first=True
            )
        else:
            lstm_out, _ = self.lstm(embedded)
        
        # lstm_out: (batch, seq_len, 2*hidden_dim)
        logits = self.classifier(lstm_out)  # (batch, seq_len, n_classes)
        return logits

# Amino acid vocabulary
AA_TO_IDX = {aa: i+1 for i, aa in enumerate('ACDEFGHIKLMNPQRSTVWY')}

def encode_protein(sequence, max_len=512):
    """Encode protein sequence as integer indices."""
    return [AA_TO_IDX.get(aa, 0) for aa in sequence[:max_len]]

# Simulate protein SS dataset
def generate_ss_data(n_proteins=500, min_len=50, max_len=200, seed=42):
    rng = np.random.default_rng(seed)
    aa_list = list('ACDEFGHIKLMNPQRSTVWY')
    
    sequences, labels, lengths = [], [], []
    for _ in range(n_proteins):
        L = rng.integers(min_len, max_len)
        seq = ''.join(rng.choice(aa_list, size=L))
        # Simple label: consecutive runs tend to be helix, individual = coil
        label = rng.integers(0, 3, size=L)  # random for simulation
        
        # Pad to max_len
        seq_encoded = encode_protein(seq, max_len)
        padded_seq = seq_encoded + [0] * (max_len - len(seq_encoded))
        padded_label = list(label) + [-1] * (max_len - L)
        
        sequences.append(padded_seq)
        labels.append(padded_label)
        lengths.append(L)
    
    return (torch.LongTensor(sequences),
            torch.LongTensor(labels),
            torch.LongTensor(lengths))

X, y, lens = generate_ss_data(n_proteins=500)
print(f"Input shape: {X.shape}")  # (500, 200)

model = ProteinSecondaryStructurePredictor(
    vocab_size=20, embed_dim=64, hidden_dim=128, n_layers=2
)
n_params = sum(p.numel() for p in model.parameters())
print(f"Parameters: {n_params:,}")

# Forward pass
logits = model(X[:16], lens[:16])
print(f"Output shape: {logits.shape}")  # (16, 200, 3)

# Training with padded sequence loss (ignore padding positions)
criterion = nn.CrossEntropyLoss(ignore_index=-1)
optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)

# One training step
model.train()
optimizer.zero_grad()
logits_train = model(X[:32], lens[:32])
# Reshape for loss: (batch * seq_len, n_classes) and (batch * seq_len,)
loss = criterion(
    logits_train.reshape(-1, 3),
    y[:32].reshape(-1)
)
loss.backward()
optimizer.step()
print(f"Training loss (step 1): {loss.item():.4f}")
```

## Gated Recurrent Units: Simpler Alternative

**GRUs** (Cho et al. 2014) achieve comparable performance to LSTMs with fewer parameters by merging the forget and input gates:

```python
# GRU: often faster, similar accuracy to LSTM for biological sequences
class GRUPredictor(nn.Module):
    def __init__(self, vocab_size=20, embed_dim=64, hidden_dim=128, n_classes=3):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size + 1, embed_dim, padding_idx=0)
        self.gru = nn.GRU(embed_dim, hidden_dim, num_layers=2,
                          batch_first=True, bidirectional=True, dropout=0.3)
        self.fc = nn.Linear(2 * hidden_dim, n_classes)
    
    def forward(self, x):
        h = self.embedding(x)
        h, _ = self.gru(h)
        return self.fc(h)
```

## When to Use RNNs vs. Transformers

| Criterion | LSTM/GRU | Transformer |
|-----------|----------|-------------|
| Sequence length | Up to ~1,000 | Up to ~10,000+ |
| Training speed | Sequential; slow | Parallel; fast |
| Memory | O(L) | O(L²) for attention |
| Interpretability | Hidden states opaque | Attention weights |
| Small datasets | Often better | Prone to overfit |

LSTMs are no longer the default for most sequence tasks in biology — transformers and their protein-specific variants (ESM-2, AlphaFold) dominate. However, LSTMs remain valuable for:
- **Long time-series with sequential structure** (EEG, calcium imaging)
- **Autoregressive generation** of short sequences
- **Resource-constrained settings** where transformer training is impractical
- **Downstream heads** on transformer embeddings

## Why This Matters

Understanding LSTMs explains the core challenge of sequence modeling — long-range dependencies — and how gating solves the vanishing gradient problem. This mechanistic understanding transfers directly to transformers, which solve the same problem differently (through global attention instead of gating). Every practitioner working with biological sequence models benefits from understanding why RNNs were replaced and what the tradeoffs of the replacement are.
