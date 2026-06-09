# Generative Models in Biology

For most of the history of molecular biology, the relationship between researchers and sequence space was one-way: nature presented sequences, and scientists studied them. You could mutate a protein, but you were navigating by trial and error in a space so vast — $20^{300}$ possible sequences for a 300-residue protein — that systematic exploration was unthinkable. Generative models change this. They learn the probability distribution over sequences or structures that evolution has sampled, and then they let you sample from it yourself — not randomly, but intelligently, conditioned on whatever functional properties you want.

**Generative models** learn the probability distribution over biological sequences or structures, then sample from it to generate novel, realistic examples. In biology, this enables protein design (generate sequences with desired function), drug discovery (generate molecules with specified properties), and data augmentation (generate synthetic training data). The three dominant architectures are **Variational Autoencoders (VAEs)**, **Generative Adversarial Networks (GANs)**, and **Diffusion Models**.

## Variational Autoencoders for Protein Design

A **VAE** learns a compressed **latent space** $\mathbf{z}$ that captures the essential degrees of variation in a set of sequences. The architecture has two components:

**Encoder:** $q_\phi(\mathbf{z}|\mathbf{x})$ maps a sequence to a Gaussian distribution in latent space
$$\mathbf{z} \sim \mathcal{N}(\boldsymbol{\mu}_\phi(\mathbf{x}), \text{diag}(\boldsymbol{\sigma}^2_\phi(\mathbf{x})))$$

**Decoder:** $p_\theta(\mathbf{x}|\mathbf{z})$ maps a latent point back to a sequence probability distribution

**Training objective (ELBO):** maximize the evidence lower bound:
$$\mathcal{L}_{\text{ELBO}} = \mathbb{E}_{q_\phi}[\log p_\theta(\mathbf{x}|\mathbf{z})] - \text{KL}(q_\phi(\mathbf{z}|\mathbf{x}) \| p(\mathbf{z}))$$

The KL divergence term regularizes the latent space toward the prior $\mathcal{N}(\mathbf{0}, \mathbf{I})$, ensuring smooth interpolation between sequences.

```python
import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np

class ProteinVAE(nn.Module):
    """
    VAE for protein sequence generation.
    Inspired by EVE (Evolutionary model of Variant Effect, Fraternali et al. 2021).
    """
    AA_VOCAB = 'ACDEFGHIKLMNPQRSTVWY-'  # 20 AAs + gap
    VOCAB_SIZE = len(AA_VOCAB)
    
    def __init__(self, seq_len=100, latent_dim=32, hidden_dim=256):
        super().__init__()
        self.seq_len = seq_len
        self.latent_dim = latent_dim
        
        input_dim = seq_len * self.VOCAB_SIZE  # flattened one-hot
        
        # Encoder: sequence -> mean and log-variance of q(z|x)
        self.encoder = nn.Sequential(
            nn.Linear(input_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU()
        )
        self.fc_mu = nn.Linear(hidden_dim, latent_dim)
        self.fc_logvar = nn.Linear(hidden_dim, latent_dim)
        
        # Decoder: z -> sequence probability (categorical per position)
        self.decoder = nn.Sequential(
            nn.Linear(latent_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, input_dim)
        )
    
    def encode(self, x):
        """x: (batch, seq_len, vocab) one-hot"""
        h = self.encoder(x.flatten(1))
        return self.fc_mu(h), self.fc_logvar(h)
    
    def reparameterize(self, mu, logvar):
        """Reparameterization trick: z = mu + eps * sigma"""
        if self.training:
            std = torch.exp(0.5 * logvar)
            eps = torch.randn_like(std)
            return mu + eps * std
        return mu  # deterministic at inference
    
    def decode(self, z):
        """z: (batch, latent_dim) -> (batch, seq_len, vocab_size)"""
        h = self.decoder(z)
        return h.view(-1, self.seq_len, self.VOCAB_SIZE)
    
    def forward(self, x):
        mu, logvar = self.encode(x)
        z = self.reparameterize(mu, logvar)
        logits = self.decode(z)
        return logits, mu, logvar
    
    def elbo_loss(self, logits, x_onehot, mu, logvar, beta=1.0):
        """
        ELBO = reconstruction_loss + beta * KL_divergence
        beta-VAE: beta > 1 enforces disentangled latent space
        """
        # Reconstruction: cross-entropy over sequence positions
        # logits: (batch, seq_len, vocab_size), targets: argmax of one-hot
        targets = x_onehot.argmax(dim=-1)  # (batch, seq_len)
        recon_loss = F.cross_entropy(
            logits.view(-1, self.VOCAB_SIZE),
            targets.view(-1),
            reduction='mean'
        )
        
        # KL divergence: closed form for Gaussian
        kl_loss = -0.5 * torch.mean(1 + logvar - mu.pow(2) - logvar.exp())
        
        return recon_loss + beta * kl_loss, recon_loss, kl_loss
    
    @torch.no_grad()
    def generate(self, n_samples, temperature=1.0):
        """Sample new sequences by decoding random latent points."""
        self.eval()
        z = torch.randn(n_samples, self.latent_dim)
        logits = self.decode(z)
        # Sample from categorical distribution at each position
        probs = F.softmax(logits / temperature, dim=-1)
        samples = torch.multinomial(
            probs.view(-1, self.VOCAB_SIZE), num_samples=1
        ).view(n_samples, self.seq_len)
        return samples
    
    @torch.no_grad()
    def interpolate(self, seq_a, seq_b, n_steps=10):
        """
        Interpolate in latent space between two sequences.
        Produces a smooth sequence trajectory.
        """
        self.eval()
        mu_a, _ = self.encode(seq_a.unsqueeze(0))
        mu_b, _ = self.encode(seq_b.unsqueeze(0))
        
        interpolated_sequences = []
        for alpha in torch.linspace(0, 1, n_steps):
            z_interp = (1 - alpha) * mu_a + alpha * mu_b
            logits = self.decode(z_interp)
            seq = logits.argmax(dim=-1)
            interpolated_sequences.append(seq.squeeze(0))
        
        return interpolated_sequences

# Training the VAE
model = ProteinVAE(seq_len=100, latent_dim=32, hidden_dim=256)
optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)

# Simulate MSA (multiple sequence alignment) data
# Real use: UniRef50 clusters or family-specific MSAs (like EVE)
def generate_random_msa(n_seqs=1000, seq_len=100, seed=42):
    """Generate random sequences (stand-in for real MSA)."""
    rng = np.random.default_rng(seed)
    vocab_size = len(ProteinVAE.AA_VOCAB)
    # One-hot encoding
    indices = rng.integers(0, vocab_size, size=(n_seqs, seq_len))
    onehot = torch.zeros(n_seqs, seq_len, vocab_size)
    for i in range(n_seqs):
        for j in range(seq_len):
            onehot[i, j, indices[i, j]] = 1.0
    return onehot

X = generate_random_msa(n_seqs=1000, seq_len=100)
print(f"MSA shape: {X.shape}")

# Training loop (one epoch)
model.train()
batch_size = 32
n_batches = len(X) // batch_size
total_loss = 0

for i in range(n_batches):
    batch = X[i*batch_size:(i+1)*batch_size]
    optimizer.zero_grad()
    logits, mu, logvar = model(batch)
    loss, recon, kl = model.elbo_loss(logits, batch, mu, logvar, beta=1.0)
    loss.backward()
    torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
    optimizer.step()
    total_loss += loss.item()

print(f"Epoch loss: {total_loss/n_batches:.4f}")

# Generate new sequences
new_seqs = model.generate(n_samples=10, temperature=0.8)
print(f"Generated sequences shape: {new_seqs.shape}")
```

## Diffusion Models for Protein Backbone Generation

**Diffusion models** learn to reverse a noise-adding process. The **forward process** gradually adds Gaussian noise to the data over $T$ steps until the data is pure noise. The **reverse process** (the model) learns to denoise, recovering samples from the data distribution.

**RFdiffusion** (Watson et al. 2023, Nature) applies diffusion to protein backbone coordinates (N, Cα, C, O positions), enabling de novo protein design conditioned on functional motifs:

```python
# Conceptual illustration of the diffusion forward process
# Real implementation is in RFdiffusion's PyTorch codebase

import numpy as np

def diffusion_forward(x0, t, T=1000, beta_min=0.0001, beta_max=0.02):
    """
    Add noise to backbone coordinates x0 at diffusion step t.
    x0: (L, 3) protein backbone coordinates
    Returns: noisy coordinates at step t
    """
    # Linear noise schedule
    betas = np.linspace(beta_min, beta_max, T)
    alphas = 1 - betas
    alpha_bar_t = np.prod(alphas[:t])
    
    noise = np.random.standard_normal(x0.shape)
    x_t = np.sqrt(alpha_bar_t) * x0 + np.sqrt(1 - alpha_bar_t) * noise
    return x_t, noise

# Real usage: run RFdiffusion via command line
# python run_inference.py \
#   "contigmap.contigs=[A1-10/0 100]" \
#   "motif_filepath=motif.pdb" \
#   output.prefix="designed_protein"
```

## Evaluating Generative Models

Generated sequences must be evaluated on multiple axes:

```python
from scipy.spatial.distance import hamming

def evaluate_generated_sequences(generated, reference_msa, top_n=100):
    """
    Evaluate quality of generated protein sequences.
    
    Metrics:
    1. Perplexity: how surprised is a language model by generated sequences?
    2. Diversity: average pairwise Hamming distance
    3. Naturalness: fraction passing basic biochemical filters
    4. Recovery: similarity to nearest neighbor in reference MSA
    """
    results = {}
    
    # Diversity: mean pairwise hamming distance
    n = min(len(generated), top_n)
    pairwise_distances = []
    for i in range(n):
        for j in range(i+1, n):
            d = hamming(generated[i], generated[j])
            pairwise_distances.append(d)
    results['mean_diversity'] = np.mean(pairwise_distances)
    
    # Recovery: nearest neighbor similarity to MSA
    recoveries = []
    for gen_seq in generated[:top_n]:
        min_dist = min(hamming(gen_seq, ref) for ref in reference_msa)
        recoveries.append(1 - min_dist)
    results['mean_recovery'] = np.mean(recoveries)
    
    return results

# For protein design, also evaluate:
# 1. ESM-2 log-likelihood (naturalness)
# 2. AlphaFold2 pLDDT of predicted structure (structural quality)
# 3. FoldSeek TM-score to design target (structural similarity)
# 4. Experimental validation (wet lab expression and activity assay)
```

## Why This Matters

Generative models for biology are transitioning from academic curiosities to core design tools. RFdiffusion generated de novo proteins that bound therapeutic targets with sub-nanomolar affinity — directly validated experimentally. ProteinMPNN and LigandMPNN design amino acid sequences for computed protein backbones with dramatically higher experimental success rates than traditional methods. VAE-based models like EVE predict variant pathogenicity across thousands of human disease genes without requiring any experimentally labeled variants. Understanding the encoder-decoder structure, the latent space, and the training objectives of these models allows you to use them intelligently and adapt them to new protein families or design challenges.
