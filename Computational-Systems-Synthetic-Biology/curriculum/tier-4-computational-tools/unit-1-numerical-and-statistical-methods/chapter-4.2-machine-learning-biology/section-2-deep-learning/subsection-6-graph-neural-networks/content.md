# Graph Neural Networks for Molecular Data

Ask a chemist to explain why aspirin has different properties than ibuprofen, and she will not reach for a list of elemental compositions. She will draw structures. Two molecules can have identical molecular formulas and radically different biological activities because what matters is not just which atoms are present, but how they are connected — the topology of bonds, the arrangement of functional groups, the three-dimensional geometry. This structural information is inherently relational, and it cannot be captured in a fixed-length feature vector without losing something essential.

Molecules are inherently graph-structured: atoms are nodes, chemical bonds are edges. This structure is not captured by fixed-length feature vectors (ECFP fingerprints lose the topology) or 1D sequences (SMILES lose the 2D geometry). **Graph Neural Networks (GNNs)** operate directly on molecular graphs via **message passing** — iteratively propagating and aggregating information between neighboring atoms — learning representations that capture local and global molecular structure in a permutation-invariant way.

## The Message Passing Framework

The **Message Passing Neural Network (MPNN)** framework (Gilmer et al. 2017) defines a general class of GNNs through two operations: **message computation** and **aggregation**:

**Message:** At each iteration $k$, node $v$ receives messages from its neighbors $u \in \mathcal{N}(v)$:

$$\mathbf{m}_v^{(k)} = \text{AGGREGATE}\!\left(\left\{M_k\!\left(\mathbf{h}_v^{(k-1)}, \mathbf{h}_u^{(k-1)}, \mathbf{e}_{uv}\right) : u \in \mathcal{N}(v)\right\}\right)$$

**Update:** The node's hidden state is updated:

$$\mathbf{h}_v^{(k)} = U_k\!\left(\mathbf{h}_v^{(k-1)}, \mathbf{m}_v^{(k)}\right)$$

where $\mathbf{e}_{uv}$ is the edge feature (bond type, distance, angle), $M_k$ is a learned message function, and $U_k$ is a learned update function.

After $K$ iterations (the **depth** of the GNN), node representations capture the local chemical environment within a $K$-hop neighborhood. A **readout function** aggregates node representations into a graph-level molecular fingerprint:

$$\mathbf{h}_G = R\!\left(\left\{\mathbf{h}_v^{(K)} : v \in G\right\}\right)$$

## Building a Molecular GNN with PyTorch Geometric

```python
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch_geometric.data import Data, DataLoader
from torch_geometric.nn import GCNConv, GATConv, global_mean_pool, global_max_pool
import numpy as np

class MolecularGNN(nn.Module):
    """
    Graph Neural Network for molecular property prediction.
    Architecture: 3 GCN layers + global pooling + MLP head.
    """
    def __init__(self, node_features=9, edge_features=3, 
                 hidden_dim=128, n_layers=3, n_classes=1):
        super().__init__()
        
        # Graph Convolutional layers
        self.convs = nn.ModuleList()
        self.bns = nn.ModuleList()
        
        in_dim = node_features
        for i in range(n_layers):
            self.convs.append(GCNConv(in_dim, hidden_dim))
            self.bns.append(nn.BatchNorm1d(hidden_dim))
            in_dim = hidden_dim
        
        # Global pooling: aggregate all node representations to graph representation
        # Using both mean and max pooling for richer representation
        self.mlp = nn.Sequential(
            nn.Linear(2 * hidden_dim, 128),  # 2x for concat(mean, max)
            nn.ReLU(),
            nn.Dropout(0.3),
            nn.Linear(128, 64),
            nn.ReLU(),
            nn.Linear(64, n_classes)
        )
    
    def forward(self, data):
        """
        data.x: node features (n_atoms_total, node_features)
        data.edge_index: connectivity (2, n_edges_total)
        data.batch: maps each atom to its molecule index
        """
        x, edge_index, batch = data.x, data.edge_index, data.batch
        
        # Message passing layers
        for conv, bn in zip(self.convs, self.bns):
            x = conv(x, edge_index)    # aggregate neighbor features
            x = bn(x)
            x = F.relu(x)
            x = F.dropout(x, p=0.2, training=self.training)
        
        # Graph-level pooling: aggregate atoms -> molecule
        x_mean = global_mean_pool(x, batch)   # mean over atoms per molecule
        x_max  = global_max_pool(x, batch)    # max over atoms per molecule
        x = torch.cat([x_mean, x_max], dim=1)
        
        return self.mlp(x).squeeze(-1)   # (n_molecules,)

# Molecule featurization
def atom_features(atom):
    """
    Compute node features for a single atom.
    Uses RDKit atomic properties.
    """
    from rdkit import Chem
    return [
        atom.GetAtomicNum(),                    # atomic number
        atom.GetDegree(),                       # number of bonds
        int(atom.GetIsAromatic()),             # aromatic
        atom.GetFormalCharge(),                # formal charge
        int(atom.IsInRing()),                  # in ring
        atom.GetTotalNumHs(),                  # implicit H count
        atom.GetNumRadicalElectrons(),         # radical electrons
        atom.GetMass() / 100.0,               # normalized mass
        int(atom.GetChiralTag() != 0)         # chiral
    ]

def mol_to_graph(smiles):
    """Convert SMILES string to PyG graph Data object."""
    from rdkit import Chem
    mol = Chem.MolFromSmiles(smiles)
    if mol is None:
        return None
    
    # Node features
    node_feat = torch.tensor([atom_features(a) for a in mol.GetAtoms()],
                              dtype=torch.float)
    
    # Edge connectivity (both directions for undirected graph)
    edges_src, edges_dst = [], []
    for bond in mol.GetBonds():
        i, j = bond.GetBeginAtomIdx(), bond.GetEndAtomIdx()
        edges_src.extend([i, j])
        edges_dst.extend([j, i])
    
    edge_index = torch.tensor([edges_src, edges_dst], dtype=torch.long)
    
    return Data(x=node_feat, edge_index=edge_index,
                num_nodes=mol.GetNumAtoms())

# Example: ESOL solubility dataset (simplified)
smiles_list = [
    "CC(=O)Oc1ccccc1C(=O)O",  # Aspirin
    "c1ccccc1",                 # Benzene
    "CC(N)=O",                  # Acetamide
    "OCC(O)C(O)C(O)C(O)CO",    # Sorbitol
]
solubility = [-1.27, -1.78, -0.77, -2.31]  # log(solubility)

# Convert to PyG graphs
graphs = [mol_to_graph(smi) for smi in smiles_list]
for g, sol in zip(graphs, solubility):
    if g is not None:
        g.y = torch.tensor([sol], dtype=torch.float)

valid_graphs = [g for g in graphs if g is not None]
print(f"Converted {len(valid_graphs)} molecules to graphs")

# Check graph structure
g = valid_graphs[0]
print(f"Aspirin: {g.num_nodes} atoms, {g.edge_index.shape[1]//2} bonds")
print(f"Node features: {g.x.shape}")

# Model and forward pass
model = MolecularGNN(node_features=9, hidden_dim=128, n_layers=3)
n_params = sum(p.numel() for p in model.parameters())
print(f"\nModel parameters: {n_params:,}")

loader = DataLoader(valid_graphs, batch_size=4, shuffle=False)
for batch in loader:
    pred = model(batch)
    print(f"Predicted solubility: {pred.detach().numpy()}")
    print(f"True solubility: {batch.y.numpy()}")
```

## Graph Attention Networks (GAT)

**Graph Attention Networks** replace uniform aggregation with learned attention weights over neighbors, allowing the model to focus on the most relevant neighboring atoms:

```python
from torch_geometric.nn import GATConv

class MolecularGAT(nn.Module):
    """GAT version — attention-weighted message passing."""
    def __init__(self, node_features=9, hidden_dim=64, n_heads=4, n_layers=3):
        super().__init__()
        
        self.convs = nn.ModuleList()
        in_dim = node_features
        for i in range(n_layers):
            # GAT: n_heads attention heads, each of dimension hidden_dim//n_heads
            self.convs.append(GATConv(in_dim, hidden_dim // n_heads, 
                                       heads=n_heads, dropout=0.2))
            in_dim = hidden_dim  # n_heads * (hidden_dim // n_heads)
        
        self.predictor = nn.Sequential(
            nn.Linear(2 * hidden_dim, 64),
            nn.ReLU(),
            nn.Linear(64, 1)
        )
    
    def forward(self, data):
        x, edge_index, batch = data.x, data.edge_index, data.batch
        for conv in self.convs:
            x = F.elu(conv(x, edge_index))
        x = torch.cat([global_mean_pool(x, batch),
                        global_max_pool(x, batch)], dim=1)
        return self.predictor(x).squeeze(-1)
```

## Protein Graph Networks: GearNet

For proteins, the graph is defined over residues: nodes = residues, edges = residue pairs within a spatial cutoff (8 Å between Cα atoms). GearNet and subsequent models add **geometric features** (bond angles, dihedral angles, distances) as edge attributes:

```python
def protein_to_graph(coords, sequence, cutoff=8.0):
    """
    Convert protein structure to graph.
    coords: (L, 3) Cα coordinates
    Returns: PyG Data object
    """
    L = len(sequence)
    
    # Node features: amino acid identity (one-hot) + position features
    aa_to_idx = {aa: i for i, aa in enumerate('ACDEFGHIKLMNPQRSTVWY')}
    node_feat = torch.zeros(L, 20)
    for i, aa in enumerate(sequence):
        if aa in aa_to_idx:
            node_feat[i, aa_to_idx[aa]] = 1.0
    
    # Edges: connect residues within cutoff
    src, dst, edge_attr = [], [], []
    for i in range(L):
        for j in range(L):
            if i != j:
                dist = np.linalg.norm(coords[i] - coords[j])
                if dist < cutoff:
                    src.append(i)
                    dst.append(j)
                    edge_attr.append([dist, abs(i-j)])  # distance, sequence separation
    
    edge_index = torch.tensor([src, dst], dtype=torch.long)
    edge_features = torch.tensor(edge_attr, dtype=torch.float)
    
    return Data(x=node_feat, edge_index=edge_index, edge_attr=edge_features)
```

## Applications in Drug Discovery

GNNs have become standard tools in computational drug discovery:

- **Binding affinity prediction**: encode both protein pocket (graph) and ligand (graph) separately, then predict $K_d$
- **ADMET property prediction**: toxicity, solubility, membrane permeability from molecular graphs
- **Reaction yield prediction**: encode reactant and product graphs to predict yield
- **De novo drug design**: use graph-generative models (GraphVAE, JTVAE) to generate novel molecular graphs with desired properties

## Why This Matters

The success of GNNs in molecular property prediction has had concrete impacts on drug discovery timelines. Models like SchNet (atomistic force field surrogate), DimeNet (directional message passing), and PaiNN (equivariant GNN) have reduced the cost of predicting binding affinities from hours (MD simulation) to milliseconds. Graph-based drug-target interaction models screen billions of compounds for lead identification. Understanding message passing — the fundamental operation underlying all GNNs — gives you access to this entire ecosystem of tools.
