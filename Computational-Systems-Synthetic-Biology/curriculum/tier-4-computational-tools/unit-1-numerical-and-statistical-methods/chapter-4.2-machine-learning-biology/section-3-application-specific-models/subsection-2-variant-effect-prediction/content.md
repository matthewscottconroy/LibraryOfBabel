# Variant Effect Prediction

Somewhere in the genome of every patient with a rare disease is a variant that is causing it. The challenge is that each of us carries roughly four to five million places where our genome differs from the reference — and the vast majority of these are harmless. Finding the one variant responsible for a child's mysterious neurological disorder, or predicting whether a mutation detected in a clinical cancer panel will respond to a particular drug, requires the ability to distinguish the rare functional variants from the overwhelming background of benign ones. This is variant effect prediction: the computational task of assigning biological consequence to individual nucleotide changes.

Every human genome differs from the reference at approximately 4–5 million positions. The vast majority of these variants are benign; a small fraction disrupt gene function, alter protein structure, or modify regulatory activity. **Variant effect prediction** is the computational task of distinguishing consequential from neutral variants — prioritizing variants for functional follow-up, interpreting clinical genetic tests, and understanding the molecular basis of disease.

## Classes of Variants and Prediction Approaches

| Variant type | Location | Prediction approach |
|-------------|----------|---------------------|
| Missense SNV | Coding, protein | Protein structure/function models |
| Synonymous SNV | Coding | Splicing and regulatory models |
| Nonsense/frameshift | Coding | Loss-of-function predictions |
| Regulatory SNV | Promoter/enhancer | Sequence-to-expression models |
| Splice site | Intron/exon boundary | Splice prediction models |
| Structural variant | Any | Copy number + breakpoint analysis |

## In Silico Mutagenesis

The fundamental computational strategy: run the same predictive model on the reference allele and the alternate allele, then compute the **log-fold change** or **delta score**:

$$\Delta s(i, a \to b) = f(x_1, \ldots, b, \ldots, x_n) - f(x_1, \ldots, a, \ldots, x_n)$$

where $f$ is a model trained to predict some functional output (binding, accessibility, expression) and position $i$ is changed from allele $a$ to $b$.

```python
import numpy as np
import torch

def in_silico_mutagenesis(model, sequence, position, seq_len=200):
    """
    Compute predicted effect of all possible single-nucleotide substitutions
    at a given position in a sequence.
    
    Returns: dict mapping alt_allele -> delta_score
    """
    nt_map = {'A': 0, 'C': 1, 'G': 2, 'T': 3}
    idx_to_nt = 'ACGT'
    
    # Reference sequence
    ref_onehot = sequence_to_onehot(sequence, seq_len)
    with torch.no_grad():
        ref_score = model(ref_onehot.unsqueeze(0)).item()
    
    ref_base = sequence[position]
    deltas = {}
    
    for alt_base in 'ACGT':
        if alt_base == ref_base:
            continue
        # Create mutant sequence
        mut_seq = list(sequence)
        mut_seq[position] = alt_base
        mut_onehot = sequence_to_onehot(''.join(mut_seq), seq_len)
        
        with torch.no_grad():
            alt_score = model(mut_onehot.unsqueeze(0)).item()
        
        deltas[f"{ref_base}{position+1}{alt_base}"] = alt_score - ref_score
    
    return deltas

def sequence_to_onehot(seq, max_len=200):
    """Convert nucleotide string to (4, max_len) one-hot tensor."""
    nt_map = {'A': 0, 'C': 1, 'G': 2, 'T': 3}
    x = torch.zeros(4, max_len)
    for i, base in enumerate(seq[:max_len]):
        if base in nt_map:
            x[nt_map[base], i] = 1.0
    return x

# Saturation mutagenesis: all positions, all substitutions
def saturation_mutagenesis(model, sequence, seq_len=200):
    """
    Compute delta scores for every possible single-nucleotide variant in a sequence.
    Returns: (4, L) matrix of delta scores
    """
    ref_onehot = sequence_to_onehot(sequence, seq_len)
    with torch.no_grad():
        ref_score = model(ref_onehot.unsqueeze(0)).item()
    
    L = min(len(sequence), seq_len)
    delta_matrix = np.zeros((4, L))
    
    for pos in range(L):
        for nt_idx, alt_base in enumerate('ACGT'):
            mut_onehot = ref_onehot.clone()
            mut_onehot[:, pos] = 0  # zero out position
            mut_onehot[nt_idx, pos] = 1  # set to alt base
            
            with torch.no_grad():
                alt_score = model(mut_onehot.unsqueeze(0)).item()
            delta_matrix[nt_idx, pos] = alt_score - ref_score
    
    return delta_matrix
```

## ESM-1v: Zero-Shot Protein Variant Effect

**ESM-1v** (Meier et al. 2021) uses the masked marginal likelihood from a protein language model to predict variant effects without any labeled training data:

```python
import torch
import esm

def esm1v_variant_score(model, alphabet, batch_converter, 
                         sequence, position, mutant_aa):
    """
    Compute ESM-1v zero-shot variant effect score.
    
    Uses masked marginal probability:
    score = log P(mutant | context) - log P(wildtype | context)
    Positive = mutant more probable = likely tolerated
    Negative = mutant less probable = likely deleterious
    """
    wt_aa = sequence[position]
    
    # Mask the position of interest
    masked = list(sequence)
    masked[position] = '<mask>'
    masked_seq = ''.join(masked)
    
    data = [("seq", masked_seq)]
    _, _, tokens = batch_converter(data)
    
    with torch.no_grad():
        logits = model(tokens)["logits"]
    
    # +1 offset for BOS token
    log_probs = torch.log_softmax(logits[0, position + 1], dim=-1)
    
    wt_idx = alphabet.get_idx(wt_aa)
    mut_idx = alphabet.get_idx(mutant_aa)
    
    return log_probs[mut_idx].item() - log_probs[wt_idx].item()

# AlphaMissense: structure-informed variant pathogenicity
# Uses AlphaFold2 structure + sequence to predict missense pathogenicity
# Score in [0, 1]: 0 = benign, 1 = pathogenic
# Validated against ClinVar and functional assay data

# Example: load AlphaMissense predictions from DeepMind
import pandas as pd

def load_alphamissense_scores(gene_name, alphamissense_tsv):
    """
    Load precomputed AlphaMissense scores for a gene.
    Available at: https://zenodo.org/record/8208688
    """
    df = pd.read_csv(alphamissense_tsv, sep='\t',
                     comment='#',
                     names=['uniprot_id', 'protein_variant', 'genome_coords',
                            'am_pathogenicity', 'am_class'])
    gene_df = df[df['uniprot_id'] == gene_name].copy()
    return gene_df

# Interpretation
def interpret_alphamissense(score):
    if score < 0.34:
        return "Likely benign"
    elif score < 0.56:
        return "Ambiguous"
    else:
        return "Likely pathogenic"
```

## Splice Variant Effect with SpliceAI

**SpliceAI** (Jaganathan et al. 2019) predicts the probability that a variant creates or disrupts a splice donor or acceptor site, using a deep residual CNN trained on 56 nucleotide variant-effect pairs:

```python
# Using the SpliceAI delta scores precomputed lookup table
# Or running the model directly (pip install spliceai)

from spliceai.utils import get_delta_scores
from keras.models import load_model

def predict_splice_effect(sequence, position, models, ref, alt, 
                          dist=50, mask=True):
    """
    Compute SpliceAI delta scores for a variant.
    
    Returns dict with:
    - DS_AG: delta score (acceptor gain)
    - DS_AL: delta score (acceptor loss)
    - DS_DG: delta score (donor gain)
    - DS_DL: delta score (donor loss)
    - DP_AG, DP_AL, DP_DG, DP_DL: positions of maximum effect
    """
    # SpliceAI uses 10,000 nt context window
    scores = get_delta_scores(
        sequence, position, ref, alt,
        dist=dist, mask=mask,
        ann=None,   # annotation for junction masking
        models=models
    )
    return scores

# Threshold interpretation:
# DS > 0.2: low-confidence splicing effect
# DS > 0.5: medium-confidence
# DS > 0.8: high-confidence splicing disruption/creation
```

## CADD: Combined Annotation-Dependent Depletion

**CADD** (Kircher et al. 2014) trains a C-SVM on the difference between human-derived alleles (presumed to be under selection and thus enriched for functional variants) and simulated variants (presumed neutral). CADD scores are in **Phred scale**: CADD 20 = top 1% of variants, CADD 30 = top 0.1%.

```python
import requests

def query_cadd_api(chrom, pos, ref, alt):
    """
    Query the CADD REST API for a single variant.
    Returns CADD score and PHRED score.
    """
    url = f"https://cadd.gs.washington.edu/api/v1.0/{chrom}:{pos}_{ref}_{alt}"
    try:
        response = requests.get(url, timeout=10)
        if response.status_code == 200:
            data = response.json()
            return {
                'raw': data[0].get('RawScore', None),
                'phred': data[0].get('PHRED', None)
            }
    except Exception as e:
        print(f"CADD API error: {e}")
    return None

# Batch variant interpretation workflow
def prioritize_variants(vcf_variants, cadd_threshold=20, spliceai_threshold=0.5):
    """
    Prioritize variants by combining multiple functional scores.
    
    vcf_variants: list of (chrom, pos, ref, alt) tuples
    Returns: DataFrame ranked by predicted pathogenicity
    """
    import pandas as pd
    
    results = []
    for chrom, pos, ref, alt in vcf_variants:
        row = {'chrom': chrom, 'pos': pos, 'ref': ref, 'alt': alt}
        
        # CADD score
        cadd = query_cadd_api(chrom, pos, ref, alt)
        if cadd:
            row['cadd_phred'] = cadd['phred']
        
        # Additional flags
        row['high_cadd'] = row.get('cadd_phred', 0) >= cadd_threshold
        
        results.append(row)
    
    df = pd.DataFrame(results)
    return df.sort_values('cadd_phred', ascending=False)
```

## Why This Matters

Variant effect prediction is a critical clinical tool: variants of uncertain significance (VUS) are classified as potentially pathogenic or benign, influencing treatment decisions for patients with rare diseases and hereditary cancers. Computational predictions — AlphaMissense, ESM-1v, SpliceAI, CADD — are already used in clinical variant curation pipelines according to ACMG/AMP guidelines. For researchers, in silico saturation mutagenesis of entire protein domains guides rational protein engineering. Understanding the mechanistic basis of these predictions — what features the models use, how they were trained, and what their failure modes are — is essential for using them responsibly.
