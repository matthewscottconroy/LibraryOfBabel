# Sequence Representations for ML-Guided Directed Evolution

A protein sequence is, at first glance, a string of letters. Machine learning algorithms are not built to reason about letters — they operate on numbers, specifically vectors in high-dimensional spaces. Before a model can learn the relationship between sequence and fitness, you have to translate your protein's amino acid sequence into a numerical form that captures something biologically meaningful. This translation problem — sequence to numbers — turns out to matter enormously. Get it wrong, and your model treats glycine and alanine as completely unrelated, just because they happen to occupy different positions in the alphabet. Get it right, and you hand the model a representation that encodes millions of years of evolutionary information about which amino acids are interchangeable, which positions co-vary, and which regions of the protein are structurally similar. In data-limited settings — which is almost always the case in directed evolution — the quality of this representation can make or break your model.

Before a machine learning model can learn the sequence-fitness relationship, protein sequences must be converted into numerical representations that capture biologically relevant features. The choice of representation fundamentally affects model performance, particularly when training data is limited — which it almost always is in directed evolution contexts.

## The Core Challenge

A protein sequence is a discrete string: `MKTAYIAKQRQISFVKSHFSRQLEERLGLIEVQAPILSRVGDGTQDNLSGAEKAVQVKVKALPDAQFEVVHSLAKWKRQTLGQHDFSAGEGLYTHMKALRPDEDRLSPLHSVYVDQWDWERVMGDGERQFSTLKSTVEAIWAGIKATEAAVSEEFGLAPFLPDQIHFVHSQELLSRYPDLDAKGRERAIAKDLGAVFLVGIGGKLSDGHRHDVRAPDYDDWSTPSELGHAGLNGDILVWNPVLEDAFELSSMGIRVDADTLKHQLALTGDEDRLELEWHQALLRGEMPQTIGGGIGQSRLTMLLLQLPHIGQVQAGVWPAAVRESVPSLL`

This is just a sequence of letters. Machine learning operates on numerical vectors. The representation must:
1. Preserve biologically relevant information (amino acid properties, structural context)
2. Be of fixed dimensionality (most ML models expect fixed-size inputs)
3. Generalize from training sequences to unobserved sequences

## One-Hot Encoding

The simplest representation: each amino acid at each position is represented as a binary indicator vector.

**Encoding**: for a sequence of length $L$, create a matrix of shape $(L, 20)$. For position $i$ with amino acid $a$:
$$\text{one\_hot}[i, j] = \begin{cases} 1 & \text{if } j = \text{index}(a) \\ 0 & \text{otherwise} \end{cases}$$

Flatten the matrix to a vector of length $L \times 20 = 20L$.

**Properties**:
- **No amino acid similarity information**: Ala and Gly (both small, nonpolar) are as different as Ala and Trp (large, aromatic) in one-hot space
- **No positional context**: the representation of position $i$ is independent of what amino acids flank it
- **Fixed length**: works for sequences of length $L$ only
- **Interpretable**: the weights learned on one-hot features are directly interpretable as the contribution of each amino acid at each position

**When to use**: for models that need interpretability (linear regression on one-hot features → additive model, directly shows which amino acid at which position contributes to fitness). For small training datasets where complex representations overfit.

## Physicochemical Features

Instead of a binary indicator, represent each amino acid by its physicochemical properties:

**VHSE (Vector of Hydrophobic, Steric, and Electronic descriptors)**: 8-dimensional vector per amino acid derived from PCA of 50 physicochemical properties.

**AAIndex**: 566 numerical indices characterizing amino acid properties. Typically 5–10 principal components are used (capturing 90% of variance) to reduce to a low-dimensional, information-rich representation.

**Example (simplified 3D representation)**:
$$\text{Ala} \rightarrow [0.62, 0.0, 0.0] \quad (\text{hydrophobicity, charge, size})$$
$$\text{Arg} \rightarrow [-1.8, 1.0, 0.8]$$

Physicochemical representations encode amino acid similarity: Ala and Val are nearby (similar physicochemical properties); Ala and Glu are distant. This enables better generalization to unobserved sequences — the model can infer the effect of Val from Ala measurements.

## Evolutionary/Language Model Embeddings

Modern protein language models (pLMs) are trained on millions of protein sequences from UniProt using self-supervised objectives (masked language modeling). The resulting embeddings capture rich evolutionary and structural context.

### ESM-2 (Evolutionary Scale Modeling)

ESM-2 (Meta AI / Lin et al. 2023) is a transformer-based language model trained on 250M proteins:
- **Model sizes**: 8M, 35M, 150M, 650M, 3B, 15B parameters
- **Output**: 1280-dimensional embedding per residue (ESM-2 650M), or per-sequence embedding

For a sequence of length $L$, ESM-2 produces a $(L, 1280)$ embedding matrix. Pool over positions to get a single 1280-dimensional sequence vector.

```python
import esm
model, alphabet = esm.pretrained.esm2_t33_650M_UR50D()
batch_converter = alphabet.get_batch_converter()
model.eval()

data = [("seq1", "MKTAYIAKQ...")]
batch_labels, batch_strs, batch_tokens = batch_converter(data)

with torch.no_grad():
    results = model(batch_tokens, repr_layers=[33])
    
# Per-residue embeddings (shape: [1, L, 1280])
embedding = results["representations"][33]
# Per-sequence embedding (mean pooling)
seq_embedding = embedding[0, 1:-1].mean(0)  # shape: [1280]
```

**Why ESM embeddings outperform one-hot**: the model was trained to predict missing amino acids from context across millions of evolutionary sequences. It therefore encodes:
- Residue co-evolution (epistatic relationships common across the protein family)
- Structural context (residues in similar structural environments have similar embeddings)
- Amino acid interchangeability (residues that are often interchanged in the natural protein family are close in embedding space)

**Practical impact**: models trained on 100–500 fitness measurements with ESM embeddings consistently outperform models trained on thousands of one-hot-encoded measurements, particularly for predicting multi-mutation effects.

### ProtTrans

A family of protein language models (ProtBERT, ProtAlbert, ProtT5) trained by Elnaggar et al. (2021) on BFD (Big Fantastic Database, 2.1B sequences). Similar performance to ESM-2; per-residue embeddings of 1024–1024 dimensions.

### AlphaFold2/ESMFold Structural Embeddings

If a protein structure is available (experimentally or from structure prediction), graph neural network representations can be derived from the 3D coordinates:
- Atoms as graph nodes; edges for spatially adjacent atoms
- GNN-derived features encode structural context more directly than sequence-only embeddings

**When to use structural embeddings**: when the property being evolved is directly related to 3D structure (binding pocket geometry, thermostability determined by hydrophobic core packing). For general enzyme activity evolution where the structure-activity relationship is complex, sequence-based embeddings often perform equivalently or better.

## Choosing the Right Representation

| Representation | Training Data Needed | Interpretability | Handles Epistasis | Best For |
|---------------|---------------------|-----------------|-----------------|---------|
| One-hot | 100–1000 | High (additive model) | No | Small data, need interpretability |
| Physicochemical | 100–1000 | Medium | Partially | When AA similarity matters |
| ESM-2 embedding | 50–500 | Low | Partially (via pretraining) | Most directed evolution contexts |
| Structure-based GNN | 50–500 | Low | Partially | Structure-dependent properties |

**Benchmark result**: on the GB1 4-point landscape, ESM-2 + ridge regression with 100 training points achieves Spearman correlation $\rho \approx 0.7$ with the full landscape. One-hot + ridge regression achieves $\rho \approx 0.4$ at the same training set size.

## Why This Matters

Sequence representation is where the fundamental biological knowledge about proteins — evolutionary conservation, structural context, amino acid similarity — is encoded for machine learning models. A model that treats Ala and Gly as unrelated (one-hot) will learn more slowly than one that knows they are structurally similar (physicochemical) or that ESM tells it they are often interchangeable at specific positions (ESM embedding). In the data-limited regime of directed evolution (typically 100–1000 training measurements), this difference is not marginal — it often determines whether the surrogate model is accurate enough to be useful. The progression from one-hot to language model embeddings is therefore not merely technical sophistication; it is the operationalization of decades of structural biology and evolutionary genomics knowledge into a form that a statistical learning algorithm can use to improve protein function prediction from scarce data.
