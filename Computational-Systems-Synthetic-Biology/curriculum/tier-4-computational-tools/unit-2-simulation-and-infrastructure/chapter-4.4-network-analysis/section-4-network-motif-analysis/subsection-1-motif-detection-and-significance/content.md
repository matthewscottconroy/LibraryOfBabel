# Network Motif Detection and Significance

In the early 2000s, Uri Alon's lab at the Weizmann Institute asked a simple but pointed question: if you map out the regulatory connections of *E. coli* — which transcription factor regulates which gene — and then count every possible three-node circuit pattern, which ones appear more often than you would expect by chance? The answer surprised people. One pattern dominated: a regulator controls both a second regulator and a shared target gene, while the second regulator also controls the target. This is the feedforward loop, and it appears 42 times in *E. coli*'s regulatory network, where randomization predicts roughly 2. It appears repeatedly in yeast, in flies, in neural circuits. Evolution has discovered and reused this circuit pattern again and again, which strongly suggests it solves a real computational problem that cells need to solve. The task of this section is to understand what that circuit does, how to detect it, and how to decide whether its prevalence is meaningful or accidental.

**Network motifs** are small subgraph patterns that appear significantly more often in a real biological network than in randomized versions preserving the same degree sequence. They represent recurring regulatory logic — the design principles that evolution has repeatedly discovered and deployed.

## Definition and Significance Testing

A network motif of size $k$ is a connected subgraph pattern $m$ where:

$$Z_m = \frac{N_m^\text{real} - \langle N_m^\text{rand} \rangle}{\sigma_m^\text{rand}} > 2$$

where $N_m^\text{real}$ is the count of pattern $m$ in the real network, $\langle N_m^\text{rand} \rangle$ and $\sigma_m^\text{rand}$ are the mean and standard deviation of pattern counts in an ensemble of randomized networks preserving the degree sequence.

The **normalized significance profile** (SP) plots $Z$-scores for all possible patterns, revealing the motif "fingerprint" of the network.

## Generating the Null Model

The null model must preserve the degree sequence (same in-degree and out-degree for each node in directed networks). The standard approach is **edge swapping** (Markov chain Monte Carlo):

```python
import networkx as nx
import numpy as np
from itertools import combinations
from collections import defaultdict

def randomize_preserving_degree(G, n_swaps_factor=100, seed=42):
    """
    Generate a random graph preserving the degree sequence via edge swapping.
    For directed graphs: preserves both in-degree and out-degree of each node.
    n_swaps_factor: perform n_swaps_factor × |E| swap attempts.
    """
    G_rand = G.copy()
    rng = np.random.default_rng(seed)
    edges = list(G_rand.edges())
    n_swaps = n_swaps_factor * len(edges)

    successful_swaps = 0
    for _ in range(n_swaps):
        if len(edges) < 2:
            break
        # Pick two random edges
        idx1, idx2 = rng.choice(len(edges), size=2, replace=False)
        a, b = edges[idx1]
        c, d = edges[idx2]

        # Proposed swap: (a→b, c→d) → (a→d, c→b)
        # Conditions: no self-loops, no multi-edges
        if a != d and c != b and a != c and b != d:
            if not G_rand.has_edge(a, d) and not G_rand.has_edge(c, b):
                G_rand.remove_edge(a, b)
                G_rand.remove_edge(c, d)
                G_rand.add_edge(a, d)
                G_rand.add_edge(c, b)
                edges[idx1] = (a, d)
                edges[idx2] = (c, b)
                successful_swaps += 1

    swap_rate = successful_swaps / n_swaps
    print(f"Edge swapping: {successful_swaps}/{n_swaps} successful ({swap_rate:.1%})")
    return G_rand

# Verify degree sequence preservation
def verify_degree_sequence(G_orig, G_rand):
    orig_degrees = sorted(d for _, d in G_orig.degree())
    rand_degrees = sorted(d for _, d in G_rand.degree())
    assert orig_degrees == rand_degrees, "Degree sequences do not match!"
    print("Degree sequence preserved: OK")
```

## Counting Subgraph Patterns

For a size-3 motif analysis of a directed network, there are 13 possible connected 3-node directed subgraph patterns. The most biologically relevant are:

- **Pattern 5** (Feedforward loop, FFL): A→B, A→C, B→C — one regulator controls another while both regulate a common target
- **Pattern 6** (Bi-fan): A→C, A→D, B→C, B→D — two TFs co-regulate two genes
- **Pattern 3**: A→B, A→C, B→C, C→A — cycle with feedforward

```python
def count_3node_motifs(G):
    """
    Count all distinct 3-node directed connected subgraph patterns.
    Returns: dict of {pattern_canonical: count}
    WARNING: O(N^3) for naive implementation — only practical for N < 1000.
    """
    if not G.is_directed():
        raise ValueError("Directed graph required for canonical motif analysis")

    nodes = list(G.nodes())
    N = len(nodes)
    motif_counts = defaultdict(int)

    for i, a in enumerate(nodes):
        for j, b in enumerate(nodes[i+1:], i+1):
            for c in nodes[j+1:]:
                triplet = [a, b, c]
                # Extract induced subgraph
                subg = G.subgraph(triplet)
                if not nx.is_weakly_connected(subg):
                    continue  # not a connected 3-node subgraph

                # Canonical pattern: adjacency matrix as tuple (sorted)
                adj = nx.to_numpy_array(subg, nodelist=triplet, dtype=int)
                pattern = tuple(adj.flatten())
                motif_counts[pattern] += 1

    return dict(motif_counts)

def feedforward_loop_count(G):
    """
    Efficiently count feedforward loops (FFL): A→B, A→C, B→C.
    More efficient than exhaustive 3-node subgraph enumeration.
    """
    count = 0
    for a in G.nodes():
        successors_a = set(G.successors(a))  # nodes regulated by A
        for b in successors_a:
            successors_b = set(G.successors(b))  # nodes regulated by B
            # FFL: any c regulated by both A and B (where A also regulates B)
            common_targets = successors_a & successors_b - {a, b}
            count += len(common_targets)
    return count

# Create a sample GRN (gene regulatory network)
GRN = nx.DiGraph()
GRN.add_edges_from([
    ("crp", "lacZ"), ("crp", "lacI"), ("lacI", "lacZ"),  # FFL type 1
    ("crp", "araC"), ("crp", "araBAD"), ("araC", "araBAD"),  # FFL type 1
    ("argR", "argA"), ("argR", "argB"), ("argA", "argB"),  # FFL
    ("rpoS", "katG"), ("rpoS", "oxyR"), ("oxyR", "katG"),  # FFL
    ("fnr", "fumB"), ("narL", "fumB"), ("fnr", "narL"),  # additional edges
])

n_ffl = feedforward_loop_count(GRN)
print(f"Feedforward loops: {n_ffl}")
```

## Z-Score Computation

```python
def compute_motif_zscore(G, motif_counter_func, n_rand=100, seed=42):
    """
    Compute Z-score for a specific motif type.
    motif_counter_func: function(G) -> count of motif in G
    """
    # Count in real network
    N_real = motif_counter_func(G)

    # Count in randomized networks
    N_rand_list = []
    for i in range(n_rand):
        G_rand = randomize_preserving_degree(G, n_swaps_factor=50,
                                              seed=seed + i)
        N_rand_list.append(motif_counter_func(G_rand))

    N_rand_mean = np.mean(N_rand_list)
    N_rand_std = np.std(N_rand_list)

    if N_rand_std > 0:
        Z = (N_real - N_rand_mean) / N_rand_std
    else:
        Z = np.inf if N_real > N_rand_mean else 0

    print(f"Feedforward loop analysis:")
    print(f"  Real network: {N_real}")
    print(f"  Random networks: {N_rand_mean:.1f} ± {N_rand_std:.1f}")
    print(f"  Z-score: {Z:.2f}")
    print(f"  {'MOTIF (Z > 2)' if Z > 2 else 'NOT A MOTIF'}")
    print(f"  Significance: ~{1 - 0.977 if Z > 2 else 0.023:.1%} chance by random")

    return {"N_real": N_real, "N_rand_mean": N_rand_mean,
            "N_rand_std": N_rand_std, "Z": Z}

result = compute_motif_zscore(GRN, feedforward_loop_count, n_rand=20)
```

## Canonical Biological Motifs

**Feedforward loop (FFL)**: The FFL is the most overrepresented motif in the *E. coli* and yeast transcriptional regulatory networks. Uri Alon's group (Mangan & Alon, 2003) showed that different FFL types implement different input/output functions:

- **Coherent FFL (type 1)**: A activates B; A and B both activate C. Function: pulse generator; sign-sensitive delay
- **Incoherent FFL (type 4)**: A activates B and C; B represses C. Function: pulse generator; adaptive response

**Bi-fan**: A→C, A→D, B→C, B→D. Two TFs co-regulate two genes. The most significant motif in the *E. coli* GRN by Z-score; forms Dense Overlapping Regulons (DORs) where many TFs combinatorially regulate many genes.

**Single input module (SIM)**: one master regulator controls a group of genes with no other regulation. Characteristic of operons. Enables coordinateregulation of gene batteries.

```python
# Motif significance profile for E. coli GRN (from Shen-Orr et al., 2002)
# Real data summary:
ecoli_motif_summary = {
    "Feedforward loop (FFL)": {"Z": 12.6, "n_real": 42, "interpretation": "Sign-sensitive delay"},
    "Bi-fan": {"Z": 25.3, "n_real": 203, "interpretation": "Dense overlapping regulon"},
    "3-cycles": {"Z": -1.8, "n_real": 0, "interpretation": "Anti-motif (avoided)"},
    "Feedback loops": {"Z": -0.5, "n_real": 5, "interpretation": "Slightly under-represented"},
}

print("E. coli GRN motif significance profile:")
print(f"{'Motif':<35} {'Z-score':>10} {'N_real':>10} {'Function'}")
print("-" * 75)
for motif, data in ecoli_motif_summary.items():
    print(f"{motif:<35} {data['Z']:>10.1f} {data['n_real']:>10} "
          f"{data['interpretation']}")

print("\nNote: 3-cycles are anti-motifs — actively avoided in GRNs.")
print("Biological reason: avoiding oscillations in transcriptional regulation")
```

## Why This Matters

Motif analysis reveals the evolutionary design principles of biological circuits. The feedforward loop is not randomly present in *E. coli*'s regulatory network — it has been independently identified in dozens of other organisms' regulatory networks, suggesting strong evolutionary selection for this circuit architecture. Understanding why these motifs are selected tells us what computational problems the cell is solving: sign-sensitive delays, noise filtering, adaptive responses to signals. For synthetic biologists, motif analysis provides a library of characterized "parts" with known input-output behaviors. Building a synthetic pulse generator? Use an incoherent FFL. Building a switch with memory? Use a positive feedback loop. Motif analysis connects network topology to biological function in a way that is both rigorously mathematical and immediately applicable.
