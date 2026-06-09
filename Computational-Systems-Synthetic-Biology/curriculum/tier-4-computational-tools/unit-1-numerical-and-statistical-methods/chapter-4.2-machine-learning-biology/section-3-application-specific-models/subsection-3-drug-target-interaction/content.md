# Drug-Target Interaction Prediction

Drug discovery is one of the most expensive and time-consuming endeavors in all of science. The path from identifying a promising molecule to an approved drug takes over a decade and costs more than a billion dollars, largely because the experimental phase — testing whether a compound actually binds its target, whether it is absorbed by cells, whether it is toxic — proceeds one molecule at a time in the laboratory. Machine learning does not replace this experimental work. But it can reorder it, filtering a universe of billions of candidate molecules down to a handful of leads before a single assay is run.

Predicting whether a small molecule drug binds a protein target is central to drug discovery. Machine learning has transformed this space by enabling rapid virtual screening of billions of compound-target pairs, dramatically outpacing traditional experimental assays and physics-based docking in throughput.

## The Drug-Target Interaction Problem

**Drug-target interaction (DTI)** prediction is the task of estimating binding affinity — typically expressed as $K_d$, $K_i$, or $IC_{50}$ — between a chemical compound and a protein target. The challenge is dual: both the compound and the protein must be meaningfully encoded.

A naïve formulation treats DTI as binary classification (binder / non-binder), but the regression setting (predicting continuous affinity) is more informative and better reflects the underlying biochemistry. The benchmark datasets used most frequently are **Davis** (kinase inhibitors, $K_d$ values) and **KIBA** (kinase inhibitor bioactivity).

## Classical Approaches: QSAR and Molecular Docking

**QSAR (Quantitative Structure-Activity Relationship)** models map molecular fingerprints to activity scores using classical ML:

$$\hat{y} = f(\phi(\text{mol}))$$

where $\phi$ is a fixed fingerprint — Morgan circular fingerprints (radius 2, 2048 bits) being the most common. The function $f$ may be a random forest, gradient boosted trees, or SVM. QSAR models train on a single target and do not generalize across proteins.

**Molecular docking** (AutoDock Vina, Glide) evaluates binding poses by scoring steric complementarity and electrostatics. Docking is interpretable and gives binding mode predictions, but its scoring functions are approximate and its throughput is limited to millions of compounds rather than billions.

## Deep Learning Encodings

### Compound Representation

Modern DTI models encode compounds as one of:
- **SMILES strings**: fed directly to a 1D CNN or transformer (BERT over SMILES tokens)
- **Molecular graphs**: atoms as nodes, bonds as edges, processed with a GNN
- **3D point clouds**: atom positions from a conformation, processed with a geometric network

The molecular graph approach via **graph neural networks (GNNs)** is now the default because it is permutation-invariant and directly encodes bond topology.

### Protein Representation

Proteins can be encoded as:
- **Sequence**: amino acid characters, embedded via a pretrained protein language model (ESM-2) or 1D CNN
- **Structure**: residue contact map or 3D backbone coordinates
- **Pocket features**: physicochemical properties of the binding site residues (when structure is known)

## DeepDTA: Sequence-Based DTI

**DeepDTA** (Öztürk et al., 2018) was a landmark model using two parallel 1D CNNs — one for the SMILES string and one for the amino acid sequence — whose outputs are concatenated and fed into a fully connected network to predict binding affinity.

```python
import torch
import torch.nn as nn

class DeepDTA(nn.Module):
    def __init__(self, drug_vocab=64, prot_vocab=25,
                 drug_len=100, prot_len=1000, n_filters=32):
        super().__init__()
        self.drug_embed = nn.Embedding(drug_vocab, 128)
        self.prot_embed = nn.Embedding(prot_vocab, 128)

        # Compound CNN: three stacked convolutions
        self.drug_cnn = nn.Sequential(
            nn.Conv1d(128, n_filters, kernel_size=4), nn.ReLU(),
            nn.Conv1d(n_filters, n_filters*2, kernel_size=6), nn.ReLU(),
            nn.Conv1d(n_filters*2, n_filters*3, kernel_size=8), nn.ReLU(),
            nn.AdaptiveMaxPool1d(1)
        )

        # Protein CNN: same three-block architecture
        self.prot_cnn = nn.Sequential(
            nn.Conv1d(128, n_filters, kernel_size=4), nn.ReLU(),
            nn.Conv1d(n_filters, n_filters*2, kernel_size=8), nn.ReLU(),
            nn.Conv1d(n_filters*2, n_filters*3, kernel_size=12), nn.ReLU(),
            nn.AdaptiveMaxPool1d(1)
        )

        combined = n_filters * 3 * 2  # concatenated drug + protein features
        self.fc = nn.Sequential(
            nn.Linear(combined, 1024), nn.ReLU(), nn.Dropout(0.1),
            nn.Linear(1024, 1024), nn.ReLU(), nn.Dropout(0.1),
            nn.Linear(1024, 512), nn.ReLU(),
            nn.Linear(512, 1)  # predicted log(affinity)
        )

    def forward(self, drug_ids, prot_ids):
        d = self.drug_embed(drug_ids).permute(0, 2, 1)  # (B, C, L)
        p = self.prot_embed(prot_ids).permute(0, 2, 1)
        d_feat = self.drug_cnn(d).squeeze(-1)
        p_feat = self.prot_cnn(p).squeeze(-1)
        return self.fc(torch.cat([d_feat, p_feat], dim=1))
```

## Graph-Based DTI with GNNs

The state of the art encodes the drug as a molecular graph and the protein sequence (or structure) with a pretrained language model, then learns a cross-attention interaction layer.

```python
import torch
import torch.nn as nn
from torch_geometric.nn import GCNConv, global_mean_pool

class MolecularGNN(nn.Module):
    """Encode a drug molecule from its atomic graph."""
    def __init__(self, node_features=78, hidden=128, output=256):
        super().__init__()
        self.conv1 = GCNConv(node_features, hidden)
        self.conv2 = GCNConv(hidden, hidden)
        self.conv3 = GCNConv(hidden, output)
        self.relu = nn.ReLU()

    def forward(self, x, edge_index, batch):
        x = self.relu(self.conv1(x, edge_index))
        x = self.relu(self.conv2(x, edge_index))
        x = self.relu(self.conv3(x, edge_index))
        return global_mean_pool(x, batch)  # graph-level embedding

class GNNDTI(nn.Module):
    """GNN drug encoder + ESM protein encoder."""
    def __init__(self, esm_dim=1280, mol_dim=256, hidden=512):
        super().__init__()
        self.mol_encoder = MolecularGNN(output=mol_dim)
        # Project ESM embeddings to same hidden space
        self.prot_proj = nn.Linear(esm_dim, hidden)
        self.drug_proj = nn.Linear(mol_dim, hidden)
        self.predictor = nn.Sequential(
            nn.Linear(hidden * 2, 512), nn.ReLU(), nn.Dropout(0.2),
            nn.Linear(512, 1)
        )

    def forward(self, mol_graph, esm_emb):
        drug_feat = self.drug_proj(
            self.mol_encoder(mol_graph.x, mol_graph.edge_index, mol_graph.batch)
        )
        prot_feat = self.prot_proj(esm_emb)  # (B, hidden)
        return self.predictor(torch.cat([drug_feat, prot_feat], dim=1))
```

## Featurizing Molecules for Graph Models

The **RDKit** library converts SMILES strings to molecular graphs with standard atom features:

```python
from rdkit import Chem
from rdkit.Chem import Descriptors
import numpy as np

def atom_features(atom):
    """78-dimensional atom feature vector."""
    return np.array([
        atom.GetAtomicNum(),
        atom.GetDegree(),
        atom.GetFormalCharge(),
        int(atom.GetHybridization()),
        int(atom.GetIsAromatic()),
        atom.GetTotalNumHs(),
        int(atom.IsInRing()),
    ], dtype=np.float32)

def mol_to_graph(smiles):
    """Convert SMILES to (node_features, edge_index) tensors."""
    mol = Chem.MolFromSmiles(smiles)
    if mol is None:
        raise ValueError(f"Invalid SMILES: {smiles}")

    node_feat = np.vstack([atom_features(a) for a in mol.GetAtoms()])

    edges = []
    for bond in mol.GetBonds():
        i, j = bond.GetBeginAtomIdx(), bond.GetEndAtomIdx()
        edges += [[i, j], [j, i]]  # undirected

    edge_index = np.array(edges, dtype=np.int64).T
    return node_feat, edge_index

# Example: ibuprofen
feats, edges = mol_to_graph("CC(C)Cc1ccc(cc1)C(C)C(=O)O")
print(f"Atoms: {feats.shape[0]}, Bonds (undirected): {edges.shape[1] // 2}")
```

## Worked Example: Training on the Davis Dataset

```python
import pandas as pd
import torch
from torch.utils.data import DataLoader
from sklearn.metrics import mean_squared_error
import numpy as np

# Load Davis dataset (available via DeepPurpose or TDC)
# Columns: Drug, Target, Y (log Kd)
df = pd.read_csv("davis.csv")

# Convert pKd = -log10(Kd); Davis stores Kd in nM
df["pKd"] = -np.log10(df["Y"] / 1e9)

# Train/test split — IMPORTANT: split by drug cluster, not random
# (See section on data leakage for correct splitting)
train_df = df[df["Drug_cluster"] != df["Drug_cluster"].max()]
test_df  = df[df["Drug_cluster"] == df["Drug_cluster"].max()]

print(f"Train: {len(train_df)}, Test: {len(test_df)}")
print(f"Mean pKd (train): {train_df['pKd'].mean():.2f}")

# Evaluate with concordance index (Ci) and MSE
from lifelines.utils import concordance_index
ci = concordance_index(test_df["pKd"], predicted_pKd)
mse = mean_squared_error(test_df["pKd"], predicted_pKd)
print(f"CI: {ci:.3f}, MSE: {mse:.3f}")
```

## Molecular Docking vs. ML Scoring

| Aspect | Docking (Vina/Glide) | ML Scoring (DeepDTA/GNN) |
|---|---|---|
| Interpretability | Binding pose | Black box |
| Speed | ~1 min/compound | ~1 ms/compound |
| Generalization | Physics-based | Training data dependent |
| Accuracy | ~1-2 kcal/mol error | Similar on benchmarks |
| 3D structure needed | Yes | No (sequence-based) |

A common production pipeline combines both: use ML for ultra-large-scale virtual screening ($10^9$ compounds), then dock the top 10,000 hits for pose analysis and structural insight.

## Why This Matters

Drug discovery costs exceed \$1 billion per approved drug and takes over a decade. ML-based DTI prediction compresses early-stage screening from years of bench experiments to hours of computation. GNN-based models trained on public bioactivity databases (ChEMBL, BindingDB) can prioritize candidate compounds across an entire proteome, enabling polypharmacology analysis — understanding which off-target proteins a drug will engage. For neglected diseases where experimental resources are limited, in silico DTI prediction may be the only feasible path to identifying lead compounds.
