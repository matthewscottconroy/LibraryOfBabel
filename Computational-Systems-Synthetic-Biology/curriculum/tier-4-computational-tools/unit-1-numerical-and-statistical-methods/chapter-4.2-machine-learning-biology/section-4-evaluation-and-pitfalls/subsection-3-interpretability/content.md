# Interpretability Methods for Biological ML Models

A radiologist who looks at a scan and says "this is cancer" but cannot explain what features led to that conclusion is not practicing medicine — they are making an oracle pronouncement. We hold human experts to a higher standard: they must show their reasoning, and that reasoning must be coherent with known biology, anatomy, and pathophysiology. The same standard should apply to machine learning models in biology, and for similar reasons: a model that predicts correctly but cannot be understood is limited in scientific utility. It cannot generate hypotheses, it cannot detect when it is wrong for the wrong reasons, and it cannot be trusted in clinical contexts where explanations are legally and ethically required.

A model that predicts well but cannot be understood is limited in scientific utility. In biology, we often want not just predictions but mechanistic hypotheses: which features drive a prediction, which residues matter for binding, which sequence positions explain a phenotype. Interpretability methods bridge the gap between model performance and scientific insight.

## Why Interpretability Matters in Biology

Biological ML has three interpretability goals distinct from other application domains:

1. **Discovery**: identify novel sequence features, residues, or regulatory elements associated with function
2. **Validation**: confirm that models have learned biologically coherent features (not experimental artifacts)
3. **Regulatory and clinical contexts**: predictions used in clinical decision-making require explanations

A model that achieves high AUC by exploiting a batch effect or a database artifact — rather than true sequence-function relationships — will appear perfectly confident but be biologically meaningless. Interpretability methods help detect such failures.

## SHAP: SHapley Additive Explanations

**SHAP** is grounded in cooperative game theory. The SHAP value $\phi_i$ for feature $i$ measures its average marginal contribution to the model prediction across all possible feature subsets:

$$\phi_i(f, x) = \sum_{S \subseteq F \setminus \{i\}} \frac{|S|!(|F|-|S|-1)!}{|F|!} \left[ f(S \cup \{i\}) - f(S) \right]$$

where $F$ is the full feature set and $f(S)$ is the model prediction with only features in $S$ available. SHAP values satisfy desirable axioms: efficiency, symmetry, dummy, and additivity. The predicted value for a sample equals the base rate plus the sum of all SHAP values:

$$f(x) = \mathbb{E}[f(X)] + \sum_{i=1}^{|F|} \phi_i$$

```python
import shap
import numpy as np
import pandas as pd
from sklearn.ensemble import RandomForestClassifier
import matplotlib.pyplot as plt

# Example: predict gene essentiality from genomic features
# X: (n_genes, n_features) — codon usage, GC content, expression level, etc.
# y: 1 = essential, 0 = non-essential

rf = RandomForestClassifier(n_estimators=500, max_features="sqrt",
                             n_jobs=-1, random_state=42)
rf.fit(X_train, y_train)

# TreeExplainer is exact and fast for tree-based models
explainer = shap.TreeExplainer(rf)
shap_values = explainer.shap_values(X_test)
# shap_values is a list: [class_0_shap, class_1_shap]
# For binary classification, use class_1 (positive class)
sv = shap_values[1]  # shape: (n_test_samples, n_features)

# Summary plot: beeswarm; one dot per (sample, feature)
# Color = feature value (red = high, blue = low)
# x-axis = SHAP value (positive = pushes toward essential)
shap.summary_plot(sv, X_test, feature_names=feature_names, show=False)
plt.savefig("shap_summary.pdf", bbox_inches="tight")

# For a single gene prediction, explain individual prediction
gene_idx = 42
shap.waterfall_plot(shap.Explanation(
    values=sv[gene_idx],
    base_values=explainer.expected_value[1],
    data=X_test[gene_idx],
    feature_names=feature_names
))

# Bar chart: mean absolute SHAP (global feature importance)
mean_abs_shap = np.abs(sv).mean(axis=0)
top_features = pd.Series(mean_abs_shap, index=feature_names).sort_values(ascending=False)
print("Top 10 features by mean |SHAP|:")
print(top_features.head(10))
```

For deep learning models, use `shap.DeepExplainer` (gradient-based approximation) or `shap.KernelExplainer` (model-agnostic, but slow for large datasets).

## Saliency Maps for Neural Networks

**Saliency maps** compute the gradient of the model output with respect to each input position. A large gradient magnitude at position $i$ indicates that a small perturbation there would strongly affect the prediction — that position is "important."

$$\text{saliency}_i = \left| \frac{\partial f(x)}{\partial x_i} \right|$$

For sequence models (CNNs on DNA or protein sequences), this reveals which nucleotides or amino acids drive a prediction:

```python
import torch
import torch.nn.functional as F
import matplotlib.pyplot as plt
import numpy as np

def compute_saliency(model, sequence_onehot, target_class=1):
    """
    Compute saliency map for a sequence classification model.
    
    Args:
        model: trained PyTorch model
        sequence_onehot: (1, alphabet_size, seq_len) one-hot tensor
        target_class: which output class to differentiate
    Returns:
        saliency: (seq_len,) array of per-position importance scores
    """
    model.eval()
    x = sequence_onehot.clone().requires_grad_(True)

    output = model(x)
    # Differentiate the target class score w.r.t. input
    model.zero_grad()
    score = output[0, target_class]
    score.backward()

    # Take max over alphabet dimension (which base would increase prediction most)
    saliency = x.grad.abs().max(dim=1)[0].squeeze().detach().numpy()
    return saliency

# Example: visualize saliency for a promoter sequence
def plot_sequence_saliency(sequence, saliency, figsize=(14, 2)):
    """Plot saliency as bar heights above sequence characters."""
    fig, ax = plt.subplots(figsize=figsize)
    positions = np.arange(len(sequence))
    ax.bar(positions, saliency, color="steelblue", width=0.8)
    ax.set_xticks(positions[::10])
    ax.set_xticklabels(list(sequence[::10]), fontsize=7)
    ax.set_ylabel("|Gradient|")
    ax.set_xlabel("Position")
    ax.set_title("Saliency map — predicted binding site")
    plt.tight_layout()
    return fig

# Smooth-Grad: average saliency over noisy input copies for cleaner maps
def smooth_grad(model, x, target_class=1, n_samples=50, noise_std=0.1):
    """Average saliency over n_samples noisy copies of input."""
    saliencies = []
    for _ in range(n_samples):
        noise = torch.randn_like(x) * noise_std
        x_noisy = x + noise
        sal = compute_saliency(model, x_noisy, target_class)
        saliencies.append(sal)
    return np.mean(saliencies, axis=0)
```

## Attention Visualization for Transformer Models

Transformer models produce **attention weights** — a matrix $A \in [0,1]^{L \times L}$ where $A_{ij}$ represents how much position $i$ attends to position $j$ in a given head and layer. For protein language models, attention weights have been shown to correlate with:
- Residue-residue contacts (3D spatial proximity)
- Functional site conservation
- Secondary structure elements

```python
import torch
import numpy as np
import matplotlib.pyplot as plt
from esm import pretrained

def extract_attention_maps(model, alphabet, sequence, layer=20):
    """
    Extract attention matrices from ESM-2 for a protein sequence.
    Returns attention: (n_heads, seq_len, seq_len)
    """
    batch_converter = alphabet.get_batch_converter()
    data = [("prot", sequence)]
    _, _, tokens = batch_converter(data)

    with torch.no_grad():
        results = model(tokens, repr_layers=[layer],
                        return_contacts=True,
                        need_head_weights=True)

    # attentions: dict of layer -> (batch, heads, L+2, L+2)
    attn = results["attentions"][layer][0, :, 1:-1, 1:-1]  # remove BOS/EOS tokens
    return attn.numpy()

def plot_attention_head(attn, head_idx=0, seq=None):
    """Plot a single attention head as a contact-map-style heatmap."""
    fig, ax = plt.subplots(figsize=(6, 5))
    im = ax.imshow(attn[head_idx], cmap="Blues", aspect="auto")
    plt.colorbar(im, ax=ax, label="Attention weight")
    ax.set_xlabel("Key position")
    ax.set_ylabel("Query position")
    ax.set_title(f"Attention head {head_idx}")
    return fig

# Symmetrize and average over heads → predicted contact map
def attention_contact_map(attn):
    """Average attention over heads, symmetrize."""
    avg = attn.mean(axis=0)
    return (avg + avg.T) / 2
```

**Caution**: attention weights are not equivalent to feature importance. High attention from position $i$ to $j$ does not mean the model uses that information; the subsequent feed-forward layers may ignore it. Gradient-based attribution methods are more reliable for causal interpretability.

## Concept Activation Vectors (TCAV)

**TCAV (Testing with Concept Activation Vectors)** asks: does a model's internal representation encode a human-defined concept? For example, "does layer 4 of this splice site predictor represent the concept of polypyrimidine tract?"

The method trains a linear classifier (the CAV) to separate activations corresponding to concept-positive examples from random examples in an intermediate layer. The **TCAV score** measures the directional derivative of the model output in the direction of the CAV:

$$\text{TCAV}_{k,l,c} = \frac{|\{x \in X_k : S_{k,l,c}(x) > 0\}|}{|X_k|}$$

where $S_{k,l,c}(x) = \nabla f_k(h_l(x)) \cdot v_c$ is the directional derivative of class $k$ activation in layer $l$ along concept vector $v_c$.

```python
import numpy as np
from sklearn.linear_model import LogisticRegression
from sklearn.model_selection import train_test_split

def compute_cav(concept_activations, random_activations, layer_name):
    """
    Train a linear classifier to separate concept from random activations.
    Returns the normal vector to the separating hyperplane (the CAV).
    """
    X_cav = np.vstack([concept_activations, random_activations])
    y_cav = np.hstack([
        np.ones(len(concept_activations)),
        np.zeros(len(random_activations))
    ])
    X_train, X_val, y_train, y_val = train_test_split(X_cav, y_cav,
                                                        test_size=0.2, random_state=42)
    clf = LogisticRegression(C=1.0, max_iter=1000)
    clf.fit(X_train, y_train)
    val_acc = clf.score(X_val, y_val)
    print(f"CAV ({layer_name}): validation accuracy = {val_acc:.3f}")
    # The CAV is the weight vector of the linear classifier
    return clf.coef_[0]  # shape: (n_features_in_layer,)

def tcav_score(model_gradients, cav):
    """
    Compute TCAV score: fraction of examples for which gradient aligns with CAV.
    model_gradients: (n_examples, n_layer_features)
    cav: (n_layer_features,)
    """
    directional_derivatives = model_gradients @ cav  # (n_examples,)
    return (directional_derivatives > 0).mean()
```

## Why This Matters

Interpretability is not just a nice-to-have for biological ML — it is how computational predictions generate scientific hypotheses. A SHAP analysis of a drug resistance predictor may reveal that a single codon position drives most predictions, motivating targeted mutagenesis experiments. Attention maps from a protein language model may highlight conserved residues in a novel enzyme family, generating mechanistic insight without any structural data. Saliency maps from a splicing model can explain pathogenic variants: a predicted increase in splicing score at a synonymous mutation tells a molecular story. Without interpretability, ML in biology reduces to an oracle that makes predictions but cannot advance mechanistic understanding.
