# Protein Language Models

Evolution is a massive experiment in protein sequence space, running for three billion years across all domains of life. By the time you sequence the hundredths of millions of proteins in UniProt, you are looking at the outcomes of that experiment: every sequence that survived was one that folded, functioned, and reproduced. The sequences that didn't work are absent from the database. This means that the distribution of natural protein sequences encodes enormous amounts of information about what proteins can and cannot do — information that a language model, trained to predict masked amino acids from context, can absorb implicitly.

**Protein language models (pLMs)** apply the transformer architecture — developed for natural language — to protein sequences, treating amino acids as tokens. Trained on hundreds of millions of sequences from UniProt/UniRef, these models learn rich representations that encode evolutionary constraints, structural tendencies, and functional properties. The result is a new paradigm: instead of building problem-specific models from scratch, researchers extract embeddings from pre-trained pLMs and fine-tune for specific tasks, or use the models directly for zero-shot predictions.

## The Training Objective: Masked Language Modeling

The flagship pLM training objective is **Masked Language Modeling (MLM)**: randomly mask a fraction of amino acids in a protein sequence, then train the model to predict the original amino acid from the remaining context. Formally, if $\mathcal{M}$ is the set of masked positions:

$$\mathcal{L}_{\text{MLM}} = -\sum_{i \in \mathcal{M}} \log P(x_i | x_{\setminus \mathcal{M}})$$

After training on 250 million sequences, the model implicitly learns:
- Which residues are evolutionarily conserved (high prediction confidence)
- Which residues are functionally interchangeable (similar probability distributions)
- Coevolutionary patterns (correlated changes between positions)
- Structural constraints (hydrophobic core packing, secondary structure preferences)

## ESM-2: The State-of-the-Art Protein Language Model

**ESM-2** (Evolutionary Scale Modeling, Lin et al. 2023, Science) is trained by Meta AI on UniRef50 (250M sequences) using a standard transformer architecture. Available in sizes from 8M to 15B parameters:

| Model | Parameters | Layers | Embedding dim | Use case |
|-------|-----------|--------|---------------|----------|
| ESM-2 8M | 8M | 6 | 320 | Fast inference |
| ESM-2 150M | 150M | 30 | 640 | Balanced |
| ESM-2 650M | 650M | 33 | 1280 | High quality |
| ESM-2 3B | 3B | 36 | 2560 | Structure prediction |
| ESM-2 15B | 15B | 48 | 5120 | Best accuracy |

```python
import torch
import esm  # pip install fair-esm
import numpy as np

# Load pre-trained ESM-2 (650M parameter version)
model, alphabet = esm.pretrained.esm2_t33_650M_UR50D()
batch_converter = alphabet.get_batch_converter()
model.eval()

# Batch of protein sequences
data = [
    ("GFP_WT", "MSKGEELFTGVVPILVELDGDVNGHKFSVRGEGEGDATYGKLTLKFICTTGKLPVPWPTLVTTLTYGVQCFSRYPDHMKQHDFFKSAMPEGYVQERTISFKDDGNYKTRAEVKFEGDTLVNRIELKGIDFKEDGNILGHKLEYNYNSHNVYIMADKQKNGIKVNFKIRHNIEDGSVQLADHYQQNTPIGDGPVLLPDNHYLSTQSALSKDPNEKRDHMVLLEFVTAAGITLGMDELYK"),
    ("P53_DBD", "VVRCPHHERCSDSDGLAPPQHLIRVEGNLRVEYLDDRNTFRHSVVVPYEPPEVGSDCTTIHYNYMCNSSCMGQMNRRPILTIITLEDSSGKLLGRNSFEVRVCACPGRDRRTEEENLRKKGQVLLKEIREGQRLKP")
]

batch_labels, batch_strs, batch_tokens = batch_converter(data)

print(f"Input token shape: {batch_tokens.shape}")

with torch.no_grad():
    results = model(
        batch_tokens,
        repr_layers=[33],           # extract from last transformer layer
        return_contacts=True        # predict contact map
    )

# Per-residue embeddings: (n_proteins, seq_len, embed_dim)
token_reps = results["representations"][33]
print(f"Embedding shape: {token_reps.shape}")

# Mean pooling for protein-level representation
protein_reps = []
for i, (label, seq) in enumerate(data):
    # Exclude [CLS] and [EOS] tokens (positions 0 and -1)
    seq_len = len(seq)
    protein_rep = token_reps[i, 1:seq_len+1, :].mean(dim=0)
    protein_reps.append(protein_rep.numpy())
    print(f"{label}: embedding dim {protein_rep.shape[0]}")

# Predicted contact map: (n_proteins, L, L)
contacts = results["contacts"]
print(f"\nContact map shape: {contacts.shape}")
print(f"GFP max contact probability: {contacts[0].max().item():.3f}")
```

## Zero-Shot Fitness Prediction

One of the most powerful capabilities of pLMs is **zero-shot variant effect prediction**: without any task-specific training data, pLMs can predict whether a mutation is likely to be tolerated or deleterious based on the log-likelihood ratio between the mutant and wildtype sequence.

$$\Delta \text{LLR}(a \to b, \text{ position } i) = \log P_\theta(x_i = b | x_{\setminus i}) - \log P_\theta(x_i = a | x_{\setminus i})$$

A positive score suggests the mutation is favorable (or at least tolerated); a negative score suggests it is deleterious. This is validated against deep mutational scanning (DMS) datasets:

```python
def compute_mutation_score(model, alphabet, batch_converter, 
                            wildtype_seq, mutant_aa, position):
    """
    Compute zero-shot mutation score using ESM-2 masked marginals.
    
    Returns log P(mutant|context) - log P(wildtype|context)
    Higher = mutant more likely than WT = potentially tolerated
    """
    # Mask the position of interest
    masked_seq = list(wildtype_seq)
    masked_seq[position] = '<mask>'  # ESM mask token
    masked_seq = ''.join(masked_seq)
    
    data = [("masked", masked_seq)]
    _, _, tokens = batch_converter(data)
    
    with torch.no_grad():
        logits = model(tokens)["logits"]
    
    # Get log probabilities at masked position
    # +1 for BOS token offset
    masked_pos = position + 1
    log_probs = torch.log_softmax(logits[0, masked_pos], dim=-1)
    
    wt_aa_idx = alphabet.get_idx(wildtype_seq[position])
    mut_aa_idx = alphabet.get_idx(mutant_aa)
    
    delta_llr = log_probs[mut_aa_idx].item() - log_probs[wt_aa_idx].item()
    return delta_llr

# Compute single-site scanning for GFP
gfp_wt = data[0][1][:50]  # first 50 residues for illustration
aa_list = list('ACDEFGHIKLMNPQRSTVWY')

print("GFP single-site mutational scan (first 10 positions):")
for pos in range(min(10, len(gfp_wt))):
    wt = gfp_wt[pos]
    best_mut = None
    best_score = float('-inf')
    for mut in aa_list:
        if mut != wt:
            score = compute_mutation_score(model, alphabet, batch_converter,
                                           gfp_wt, mut, pos)
            if score > best_score:
                best_score = score
                best_mut = mut
    print(f"  Position {pos+1} ({wt}): best tolerated = {best_mut} (score={best_score:.2f})")
```

## Fine-Tuning for Downstream Tasks

For specific prediction tasks with labeled data, fine-tune the pre-trained pLM:

```python
class ESM2FineTuned(torch.nn.Module):
    """
    Fine-tune ESM-2 with a linear head for protein property prediction.
    Task: thermostability prediction (Tm in Celsius)
    """
    def __init__(self, esm_model, esm_alphabet, freeze_backbone=True):
        super().__init__()
        self.esm = esm_model
        self.batch_converter = esm_alphabet.get_batch_converter()
        
        # Freeze backbone for efficient fine-tuning on small datasets
        if freeze_backbone:
            for param in self.esm.parameters():
                param.requires_grad = False
        
        embed_dim = 1280  # ESM-2 650M embedding dimension
        self.regression_head = torch.nn.Sequential(
            torch.nn.Linear(embed_dim, 256),
            torch.nn.ReLU(),
            torch.nn.Dropout(0.3),
            torch.nn.Linear(256, 1)   # predict Tm (regression)
        )
    
    def forward(self, sequences):
        data = list(enumerate(sequences))
        _, _, tokens = self.batch_converter(data)
        
        with torch.no_grad() if not any(p.requires_grad for p in self.esm.parameters()) else torch.enable_grad():
            reps = self.esm(tokens, repr_layers=[33])["representations"][33]
        
        # Mean pooling over sequence (exclude [CLS] and [EOS])
        protein_reps = torch.stack([
            reps[i, 1:len(seq)+1, :].mean(dim=0)
            for i, seq in enumerate(sequences)
        ])
        
        return self.regression_head(protein_reps).squeeze(-1)

# Training would use experimental Tm values from FireProt or ProThermDB databases
```

## pLDDT and Structural Quality

ESM-2 embeddings strongly correlate with **pLDDT** (predicted Local Distance Difference Test) scores from AlphaFold2 — a per-residue measure of structural prediction confidence:

- High pLDDT (>90): structurally ordered regions; embedding captures tertiary context
- Low pLDDT (<50): intrinsically disordered regions; embedding less informative for structure

## Why This Matters

Protein language models represent a fundamental shift in protein science. Before pLMs, predicting protein function required homology search, structural modeling, or expensive experimental assays. Now, a 1280-dimensional embedding from ESM-2 encodes enough information to predict thermostability, catalytic activity, binding affinity, and disorder — all from sequence alone. Zero-shot variant effect prediction enables screening of millions of variants computationally before selecting a handful for experimental validation. For protein engineering, directed evolution, and drug target identification, pLMs are now indispensable tools.
