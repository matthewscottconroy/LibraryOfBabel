# Cathedral V: A Machine-Learning-Guided Directed Evolution Campaign

---

## The Question

Can a surrogate model trained on a small number of experimental measurements predict high-fitness protein variants, enabling efficient navigation of the sequence-fitness landscape?

---

## Prerequisites

- [Tier 1.5](../curriculum/tier-1-bioinformatics/1.5-structural-bioinformatics.md): Protein structure
- [Tier 3.5](../curriculum/tier-3-synthetic-biology/3.5-directed-evolution.md): Directed evolution methods
- [Tier 4.2](../curriculum/tier-4-computational-tools/4.2-machine-learning-biology.md): ML for biology, protein language models

---

## The Project

This cathedral is partially or entirely computational. The full version includes wet lab validation, but a rigorous computational campaign using public datasets is publishable on its own.

### Phase 1: Problem Selection

Choose a protein engineering target with available sequence-fitness data:

**Public datasets to start with:**
- **GB1 (IgG-binding domain of protein G)**: fitness landscape for binding; Olson et al. 2014; ~150,000 variants
- **GFP fluorescence**: Sarkisyan et al. 2016; ~56,000 GFP variants with fluorescence
- **AAV capsid**: Bryant et al. 2021; library of AAV variants with packaging efficiency
- **Anti-EGFR antibody**: Mason et al. 2021; single-chain antibody library with binding data

Or generate your own dataset:
- Design a mutagenesis library for your protein of interest
- Express and assay the library
- This is the expensive part — a computational campaign can design which variants to test

### Phase 2: Data Preparation

1. Download and parse the fitness dataset:
   - Sequences: FASTA or CSV with amino acid sequences
   - Fitness values: fluorescence, binding Kd, catalytic rate, growth rate proxy

2. Quality filtering:
   - Remove sequences with measurement error > threshold
   - Check for bimodal distributions (often from expression vs. activity confounding)

3. Sequence embedding:
   ```python
   import torch
   from esm import pretrained
   
   model, alphabet = pretrained.esm2_t33_650M_UR50D()
   batch_converter = alphabet.get_batch_converter()
   model.eval()
   
   def get_embeddings(sequences, labels):
       data = list(zip(labels, sequences))
       batch_labels, batch_strs, batch_tokens = batch_converter(data)
       with torch.no_grad():
           results = model(batch_tokens, repr_layers=[33])
       # Mean-pool over residues → one vector per sequence
       embeddings = results["representations"][33].mean(1)
       return embeddings.numpy()
   ```

4. Split strategy:
   - Homology-based split: cluster sequences by identity; put similar sequences in same fold
   - Random split for initial benchmarking (but be honest about limitations)

### Phase 3: Surrogate Model Training

5. Train and benchmark multiple models:
   ```python
   from sklearn.ensemble import RandomForestRegressor, GradientBoostingRegressor
   from sklearn.linear_model import Ridge
   from sklearn.model_selection import cross_val_score
   from sklearn.metrics import r2_score, spearmanr
   
   models = {
       'Ridge': Ridge(alpha=1.0),
       'RF': RandomForestRegressor(n_estimators=500, n_jobs=-1),
       'GBM': GradientBoostingRegressor(n_estimators=200),
   }
   
   for name, model in models.items():
       scores = cross_val_score(model, X_train, y_train, cv=5, scoring='r2')
       print(f"{name}: R² = {scores.mean():.3f} ± {scores.std():.3f}")
   ```

6. Neural network regressor (if dataset > 1,000 variants):
   ```python
   import torch.nn as nn
   
   class FitnessNet(nn.Module):
       def __init__(self, input_dim=1280):
           super().__init__()
           self.net = nn.Sequential(
               nn.Linear(input_dim, 256),
               nn.ReLU(),
               nn.Dropout(0.2),
               nn.Linear(256, 64),
               nn.ReLU(),
               nn.Linear(64, 1)
           )
       def forward(self, x):
           return self.net(x).squeeze()
   ```

7. Evaluate model quality:
   - R² and Spearman ρ on held-out test set
   - Top-k recovery: does model rank the top-k true variants highly?
   - Fitness distribution of top-100 model-predicted variants vs. true distribution

### Phase 4: Sequence Proposal (Bayesian Optimization)

8. Define the sequence space to explore:
   - Combinatorial: all combinations of top mutations at N sites
   - Continuous: latent space interpolation from VAE
   - Mutation-based: generate neighborhood around known good sequences

9. Acquisition function:
   ```python
   from scipy.stats import norm
   
   def expected_improvement(mu, sigma, best_so_far):
       """Expected improvement acquisition function"""
       z = (mu - best_so_far) / (sigma + 1e-9)
       return (mu - best_so_far) * norm.cdf(z) + sigma * norm.pdf(z)
   
   # For models with uncertainty (GP, deep ensemble)
   predictions = model.predict(candidate_sequences)  # (n, 2) for mean + uncertainty
   ei_scores = expected_improvement(predictions[:, 0], predictions[:, 1], best_fitness)
   ```

10. Generate candidate variants:
    - Top-k by acquisition score
    - Diversity filtering: ensure candidates are not all identical
    - Constraint checking: remove sequences with known problematic features (e.g., cysteine in oxidizing environment)

### Phase 5: Simulated or Real Experimental Validation

**Option A: Simulated (in silico)**
- Use a subset of the known fitness landscape (10% of data) as training set
- Reserve remaining 90% as test oracle
- Propose variants using your model
- "Measure" proposed variants by looking up their fitness in the oracle
- Compare enrichment of high-fitness variants vs. random selection

**Option B: Experimental (if wet lab access)**
- Express proposed variants
- Measure fitness (fluorescence, binding, catalytic activity)
- Report: fraction of proposed variants above threshold
- Compare to: random library, naive single-mutation library

### Phase 6: Active Learning Loop

11. Add measured variants to training data; retrain model

12. Repeat: propose → measure → add → retrain

13. Track: how does model performance improve over cycles?

14. Compare to baselines:
    - Random: pick variants randomly from sequence space
    - Single-mutation walk: always mutate best known variant
    - Your model: should enrich high-fitness variants faster than baselines

---

## Expected Output

- Sequence embeddings and fitness predictions compared to ground truth (held-out data)
- Benchmarking of multiple model architectures
- Acquisition function analysis: which acquisition function finds top variants most efficiently?
- Active learning curves: fitness vs. number of experimental measurements
- List of proposed variants for experimental validation

---

## Key Tools

- ESM-2: protein language model embeddings (Meta)
- scikit-learn: classical ML models
- PyTorch: neural network models
- BoTorch: Bayesian optimization in PyTorch
- DEAP: evolutionary algorithm for sequence proposal
- evcouplings: evolutionary coupling analysis (baseline comparison)
