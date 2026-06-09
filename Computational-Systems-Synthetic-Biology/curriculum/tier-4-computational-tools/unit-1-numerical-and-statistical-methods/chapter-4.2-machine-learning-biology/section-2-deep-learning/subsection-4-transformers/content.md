# Transformers for Biological Sequences

Why does a single amino acid substitution 30 residues away from an active site sometimes destroy enzyme function? The answer lies in the protein's three-dimensional structure — a lysine at position 47 forms a salt bridge with a glutamate at position 12, and breaking that contact destabilizes a loop that contacts the substrate. To predict this from sequence alone, a model must learn to look across the entire sequence simultaneously and ask, for every pair of positions: do these two residues interact? This is precisely what the self-attention mechanism does.

The **transformer** (Vaswani et al. 2017, "Attention Is All You Need") is the dominant architecture in biological sequence modeling. Unlike RNNs which process sequences step-by-step, transformers process all positions **simultaneously** via the **self-attention mechanism**, allowing every position to directly attend to every other position. This global context, combined with parallelizable computation, enabled training at scales that unlocked protein language models, genomic foundation models, and AlphaFold2.

## The Self-Attention Mechanism

The core computation of a transformer is **scaled dot-product attention**. Given an input sequence of $L$ tokens each represented as a $d$-dimensional vector, the input matrix $X \in \mathbb{R}^{L \times d}$ is projected into three matrices:

$$Q = XW^Q, \quad K = XW^K, \quad V = XW^V$$

where $W^Q, W^K, W^V \in \mathbb{R}^{d \times d_k}$ are learned projection matrices. The attention output is:

$$\text{Attention}(Q, K, V) = \text{softmax}\!\left(\frac{QK^T}{\sqrt{d_k}}\right) V$$

**Interpretation:**
- $Q$ (queries) and $K$ (keys) compute pairwise compatibility scores $QK^T$
- Dividing by $\sqrt{d_k}$ prevents the softmax from saturating in high dimensions
- The softmax produces a distribution over all $L$ positions (the **attention weights**)
- These weights sum the $V$ (values) to produce the output at each position

For a protein, the attention weight $A_{ij}$ represents how much position $i$ attends to position $j$ — which can be interpreted as a learned measure of pairwise interaction, surprisingly correlated with structural contact maps.

## Multi-Head Attention

**Multi-head attention** runs $H$ attention operations in parallel, each with different projections, then concatenates the results:

$$\text{MultiHead}(Q, K, V) = \text{Concat}(\text{head}_1, \ldots, \text{head}_H)W^O$$
$$\text{head}_h = \text{Attention}(QW_h^Q, KW_h^K, VW_h^V)$$

Different heads can attend to different types of relationships simultaneously — one head might capture local context, another long-range contacts, another coevolutionary patterns.

```python
import torch
import torch.nn as nn
import torch.nn.functional as F
import math
import numpy as np

class MultiHeadAttention(nn.Module):
    """
    Multi-head self-attention module.
    Used in BERT-style protein language models (ESM, ProtBERT).
    """
    def __init__(self, d_model, n_heads, dropout=0.1):
        super().__init__()
        assert d_model % n_heads == 0
        
        self.d_model = d_model
        self.n_heads = n_heads
        self.d_k = d_model // n_heads
        
        self.W_q = nn.Linear(d_model, d_model)
        self.W_k = nn.Linear(d_model, d_model)
        self.W_v = nn.Linear(d_model, d_model)
        self.W_o = nn.Linear(d_model, d_model)
        
        self.dropout = nn.Dropout(dropout)
        self.scale = math.sqrt(self.d_k)
    
    def forward(self, x, mask=None):
        """
        x: (batch, seq_len, d_model)
        mask: (batch, 1, 1, seq_len) — True where positions should be ignored
        Returns: (batch, seq_len, d_model)
        """
        batch, L, _ = x.shape
        
        # Project and reshape to (batch, n_heads, L, d_k)
        def project_and_reshape(W, x):
            return W(x).view(batch, L, self.n_heads, self.d_k).transpose(1, 2)
        
        Q = project_and_reshape(self.W_q, x)
        K = project_and_reshape(self.W_k, x)
        V = project_and_reshape(self.W_v, x)
        
        # Scaled dot-product attention: (batch, n_heads, L, L)
        scores = torch.matmul(Q, K.transpose(-2, -1)) / self.scale
        
        if mask is not None:
            scores = scores.masked_fill(mask, float('-inf'))
        
        attn_weights = F.softmax(scores, dim=-1)
        attn_weights = self.dropout(attn_weights)
        
        # Weighted sum of values: (batch, n_heads, L, d_k)
        attended = torch.matmul(attn_weights, V)
        
        # Reshape and project: (batch, L, d_model)
        attended = attended.transpose(1, 2).contiguous().view(batch, L, self.d_model)
        return self.W_o(attended), attn_weights

class TransformerBlock(nn.Module):
    """One transformer encoder layer: attention + feedforward + layer norm."""
    def __init__(self, d_model=256, n_heads=8, ffn_dim=1024, dropout=0.1):
        super().__init__()
        self.attention = MultiHeadAttention(d_model, n_heads, dropout)
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
        self.ffn = nn.Sequential(
            nn.Linear(d_model, ffn_dim),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(ffn_dim, d_model),
            nn.Dropout(dropout)
        )
    
    def forward(self, x, mask=None):
        # Pre-norm variant (more stable than original post-norm)
        attn_out, attn_weights = self.attention(self.norm1(x), mask)
        x = x + attn_out                           # residual connection
        x = x + self.ffn(self.norm2(x))            # FFN + residual
        return x, attn_weights

class DNATransformer(nn.Module):
    """
    Small transformer for DNA sequence classification.
    Analogous to DNABERT or Nucleotide Transformer (at tiny scale).
    """
    def __init__(self, vocab_size=5, d_model=128, n_heads=4, 
                 n_layers=4, max_len=512, n_classes=2, dropout=0.1):
        super().__init__()
        
        # Token embedding + learned positional encoding
        self.token_embedding = nn.Embedding(vocab_size, d_model, padding_idx=0)
        self.pos_embedding = nn.Embedding(max_len, d_model)
        self.dropout = nn.Dropout(dropout)
        
        # Stack of transformer blocks
        self.layers = nn.ModuleList([
            TransformerBlock(d_model, n_heads, d_model * 4, dropout)
            for _ in range(n_layers)
        ])
        
        self.norm = nn.LayerNorm(d_model)
        
        # [CLS] token pooling for classification (position 0)
        self.classifier = nn.Linear(d_model, n_classes)
    
    def forward(self, x, return_attentions=False):
        """
        x: (batch, seq_len) integer token indices; position 0 = [CLS] token
        """
        batch, L = x.shape
        positions = torch.arange(L, device=x.device).unsqueeze(0).expand(batch, -1)
        
        # Combined embedding
        h = self.dropout(self.token_embedding(x) + self.pos_embedding(positions))
        
        all_attentions = []
        for layer in self.layers:
            h, attn = layer(h)
            if return_attentions:
                all_attentions.append(attn)
        
        h = self.norm(h)
        
        # Classification from [CLS] token (position 0)
        cls_output = h[:, 0, :]    # (batch, d_model)
        logits = self.classifier(cls_output)
        
        if return_attentions:
            return logits, all_attentions
        return logits

# Test model
model = DNATransformer(vocab_size=6, d_model=128, n_heads=4, n_layers=4, max_len=200)
n_params = sum(p.numel() for p in model.parameters())
print(f"DNA Transformer parameters: {n_params:,}")

# Tokenize DNA as k-mers (3-mers give vocab size 4^3 = 64 + special tokens)
def tokenize_dna(sequence, k=3):
    """k-mer tokenization of DNA sequence."""
    kmer_to_idx = {''.join(p): i+3 for i, p in enumerate(
        [('A','A','A')] * 0  # placeholder
    )}
    # Simplified: map each nucleotide to integer
    nt_map = {'A': 1, 'C': 2, 'G': 3, 'T': 4, 'N': 5}
    tokens = [0] + [nt_map.get(nt, 5) for nt in sequence]  # 0 = CLS
    return tokens

# Forward pass
batch_size = 8
seq_len = 200
x_test = torch.randint(1, 5, (batch_size, seq_len))
x_test[:, 0] = 0  # CLS token (using index 0 here for simplicity)

logits, attentions = model(x_test, return_attentions=True)
print(f"Output logits: {logits.shape}")   # (8, 2)
print(f"Attention layers: {len(attentions)}")
print(f"Attention shape (layer 0): {attentions[0].shape}")  # (8, 4, 200, 200)
```

## BERT-Style Pre-training (Masked Language Modeling)

Large protein and DNA language models are pre-trained with the **Masked Language Model (MLM)** objective: randomly mask 15% of tokens, then train the model to predict the masked tokens from context. This forces the model to learn the "grammar" and evolutionary constraints of sequences:

```python
def create_mlm_batch(sequences, mask_prob=0.15, vocab_size=20, mask_token=21):
    """
    Apply BERT-style masking: 80% replace with [MASK], 10% random, 10% unchanged.
    Returns masked input, original labels, and mask positions.
    """
    masked = sequences.clone()
    labels = sequences.clone()
    labels.fill_(-100)  # -100: ignore in loss
    
    # Sample positions to mask
    mask = torch.rand(sequences.shape) < mask_prob
    mask &= (sequences > 0)  # don't mask padding
    
    for i, j in mask.nonzero():
        r = torch.rand(1).item()
        labels[i, j] = sequences[i, j]   # record original token
        if r < 0.8:
            masked[i, j] = mask_token      # replace with [MASK]
        elif r < 0.9:
            masked[i, j] = torch.randint(1, vocab_size, (1,))  # random token
        # else: keep original (remaining 10%)
    
    return masked, labels
```

## Extracting Attention Maps for Structural Biology

A key finding in protein language model research: attention weights in certain heads of ESM-type models are correlated with residue-residue contacts in protein structures — even though the model was never trained on structural data.

```python
# Using pretrained ESM-2 to extract attention maps
# pip install fair-esm
import torch
import esm

model_esm, alphabet = esm.pretrained.esm2_t33_650M_UR50D()
batch_converter = alphabet.get_batch_converter()
model_esm.eval()

data = [("protein1", "MKTIIALSYIFCLVFADYKDDDDK")]
batch_labels, batch_strs, batch_tokens = batch_converter(data)

with torch.no_grad():
    results = model_esm(batch_tokens, repr_layers=[33], return_contacts=True,
                        need_head_weights=True)

# Attention maps: (n_layers, n_heads, L, L)
attention_maps = results["attentions"]  # shape: (1, 33, 20, L, L)
print(f"Attention tensor shape: {attention_maps.shape}")

# Predicted contacts (derived from attention)
contacts = results["contacts"]  # (1, L, L)
```

## Why This Matters

Transformers are the foundation of the current wave of biological AI: ESM-2 for protein properties, AlphaFold2's Evoformer for structure prediction, DNABERT and Nucleotide Transformer for regulatory genomics, and Enformer for gene expression prediction. Understanding the self-attention mechanism — how Q, K, V matrices work, what positional encoding does, and why multi-head attention allows different types of sequence relationships to be learned simultaneously — is essential for both using these models and extending them to new biological problems.
