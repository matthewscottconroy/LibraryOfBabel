# Degree Distribution in Biological Networks

In 1999, Barabási and Albert published a paper in *Science* that reframed how biologists thought about networks. They were studying the World Wide Web, but the finding applied equally to protein-protein interaction networks, metabolic networks, and ecosystems: these networks are not random. In a random graph, every node has roughly the same number of connections — the degree distribution follows a bell curve. Real networks look nothing like this. Instead, a tiny fraction of nodes — hubs — collect vastly more connections than the rest, while the majority of nodes connect to only a handful of partners. The degree distribution follows a power law, with a tail so heavy that hubs like TP53 and EGFR accumulate hundreds of interacting partners while most proteins interact with one or two. This topology has consequences for robustness, drug targeting, and disease that run all the way through this chapter.

The degree distribution $P(k)$ — the probability that a randomly chosen node has exactly $k$ neighbors — is one of the most informative topological properties of a network. For biological networks, the shape of this distribution reveals whether the network has random connectivity or exhibits the hub-dominated architecture associated with scale-free networks.

## Scale-Free Networks and Power Laws

A **scale-free network** has a degree distribution that follows a power law:

$$P(k) \sim k^{-\gamma}$$

where $\gamma$ is the power-law exponent. For most empirical scale-free networks, $2 < \gamma < 3$. The key feature is a **heavy tail**: a small number of nodes (hubs) have very high degree, while the vast majority of nodes have low degree. This is fundamentally different from a Poisson distribution (where extremely high-degree nodes are exponentially rare).

**Biological significance**: PPI networks, metabolic networks, and gene regulatory networks were initially reported as scale-free ($\gamma \approx 2$–3). Hub proteins (like TP53, EGFR, BRCA1) interact with hundreds of partners, while most proteins interact with 1–3. This topology has implications:
- **Robustness to random failures**: removing a random node rarely disrupts the network (most nodes have low degree)
- **Vulnerability to targeted attacks**: removing the highest-degree hubs rapidly fragments the network
- **Hub proteins are often essential**: disrupting a hub has widespread effects

**Caution**: the scale-free nature of biological networks has been disputed (Broido & Clauset, 2019). Many PPI networks fit other distributions (log-normal, stretched exponential) equally well, and the data contain ascertainment biases (well-studied proteins have more reported interactions). Statistical rigor is essential.

## Erdős-Rényi Random Graphs (Poisson Degree Distribution)

The **Erdős-Rényi model** $G(n,p)$ creates a graph with $n$ nodes where each edge exists independently with probability $p$. In the limit $n \to \infty$, the degree distribution follows a Poisson distribution:

$$P(k) = e^{-\langle k \rangle} \frac{\langle k \rangle^k}{k!}$$

with mean degree $\langle k \rangle = p(n-1)$. The Poisson distribution has an exponential tail — no hubs; all nodes have degree close to the mean.

## Fitting and Testing Degree Distributions

```python
import networkx as nx
import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import poisson, kstest
from scipy.optimize import curve_fit

def degree_distribution(G):
    """Compute normalized degree distribution as (k, P(k)) arrays."""
    degrees = [d for _, d in G.degree()]
    k_values, counts = np.unique(degrees, return_counts=True)
    P_k = counts / counts.sum()
    return k_values, P_k

def plot_degree_distribution(G, title="Degree distribution"):
    """Plot degree distribution on linear and log-log scales."""
    k, P_k = degree_distribution(G)

    fig, axes = plt.subplots(1, 2, figsize=(12, 4))

    # Linear scale
    axes[0].bar(k, P_k, color="steelblue", alpha=0.7)
    axes[0].set_xlabel("Degree k")
    axes[0].set_ylabel("P(k)")
    axes[0].set_title(f"{title} (linear)")

    # Log-log scale: power law appears as straight line
    mask = P_k > 0
    axes[1].loglog(k[mask], P_k[mask], "o", color="steelblue",
                    markersize=5, alpha=0.8, label="Data")
    axes[1].set_xlabel("log(k)")
    axes[1].set_ylabel("log P(k)")
    axes[1].set_title(f"{title} (log-log)")

    # Fit power law: log P(k) = -γ log(k) + C
    log_k = np.log(k[mask])
    log_P = np.log(P_k[mask])
    if len(log_k) > 3:
        coeffs = np.polyfit(log_k, log_P, 1)
        gamma = -coeffs[0]  # power-law exponent
        k_range = np.linspace(k.min(), k.max(), 100)
        P_fit = np.exp(np.polyval(coeffs, np.log(k_range)))
        axes[1].loglog(k_range, P_fit, "r--", lw=2,
                        label=f"Power law fit: γ = {gamma:.2f}")
        print(f"Power-law exponent: γ = {gamma:.2f}")
        print(f"  Typical for PPI networks: γ ∈ [2.0, 3.0]")

    axes[1].legend()
    plt.tight_layout()
    return gamma if len(log_k) > 3 else None

# Load a real PPI network for analysis
# Here we generate a Barabási-Albert scale-free network as a proxy
G_sf = nx.barabasi_albert_graph(n=1000, m=2, seed=42)  # scale-free
G_er = nx.erdos_renyi_graph(n=1000, p=0.004, seed=42)  # random (Poisson)

gamma = plot_degree_distribution(G_sf, title="Barabási-Albert (scale-free proxy)")
```

## Rigorous Power-Law Testing

Fitting a line to a log-log plot is not sufficient to claim a power law — many distributions appear linear on log-log plots over a limited range. The Clauset-Shalizi-Newman (CSN) method provides rigorous testing:

```python
def test_power_law_clauset(degrees, kmin=None):
    """
    Test whether degrees follow a power law using the powerlaw package.
    pip install powerlaw
    Returns: fit object with alpha (exponent), xmin, and goodness-of-fit p-value
    """
    import powerlaw

    fit = powerlaw.Fit(degrees, discrete=True, xmin=kmin)
    alpha = fit.power_law.alpha
    xmin  = fit.power_law.xmin
    sigma = fit.power_law.sigma  # standard error on alpha

    # Goodness of fit test: KS p-value
    # p > 0.1: cannot reject power law; p < 0.1: likely not power law
    ks_stat, p_value = fit.power_law.KS()

    # Compare to log-normal (often better fit for PPI data)
    R, p_logn = fit.distribution_compare("power_law", "lognormal", normalized_ratio=True)

    print(f"Power-law fit:")
    print(f"  α (exponent) = {alpha:.3f} ± {sigma:.3f}")
    print(f"  x_min = {xmin}")
    print(f"  KS p-value = {p_value:.3f} (> 0.1: consistent with power law)")
    print(f"Comparison to log-normal:")
    print(f"  R = {R:.2f} (positive = power law better; negative = log-normal better)")
    print(f"  p = {p_logn:.3f}")

    return fit

degrees_sf = [d for _, d in G_sf.degree()]
fit = test_power_law_clauset(degrees_sf, kmin=2)
```

## Comparing Network Types

```python
import pandas as pd

def compare_network_topologies(networks_dict):
    """
    Compare degree distribution statistics across multiple networks.
    networks_dict: {name: G} where G is a NetworkX graph
    """
    results = []
    for name, G in networks_dict.items():
        degrees = [d for _, d in G.degree()]
        k_arr = np.array(degrees)
        results.append({
            "Network": name,
            "N nodes": G.number_of_nodes(),
            "M edges": G.number_of_edges(),
            "Mean degree": f"{k_arr.mean():.2f}",
            "Std degree": f"{k_arr.std():.2f}",
            "Max degree": k_arr.max(),
            "CV (σ/μ)": f"{k_arr.std()/k_arr.mean():.2f}"
        })

    df = pd.DataFrame(results).set_index("Network")
    print("Network topology comparison:")
    print(df.to_string())
    print("\nInterpretation:")
    print("  High CV (σ/μ) >> 1 → scale-free-like (hubs dominate)")
    print("  CV ≈ 1 → Poisson-like (random connectivity)")

networks = {
    "Scale-free (BA model)": G_sf,
    "Random (ER model)": G_er,
    "Small-world (WS model)": nx.watts_strogatz_graph(1000, 4, 0.1, seed=42)
}
compare_network_topologies(networks)
```

## Worked Example: Analyzing the Yeast PPI Network

```python
def analyze_yeast_ppi():
    """
    Load yeast PPI network from BioGRID and analyze degree distribution.
    (Data available at https://downloads.thebiogrid.org/BioGRID)
    """
    # In practice: download BioGRID TSV file
    # Here we simulate with a BA graph (similar properties)
    np.random.seed(42)
    G_yeast = nx.barabasi_albert_graph(n=5000, m=3, seed=42)

    degrees = [d for _, d in G_yeast.degree()]
    k_arr = np.array(degrees)

    print("Yeast PPI network (simulated scale-free):")
    print(f"  Proteins: {G_yeast.number_of_nodes()}")
    print(f"  Interactions: {G_yeast.number_of_edges()}")
    print(f"  Mean connectivity: {k_arr.mean():.1f}")

    # Hub proteins: top 1% of nodes by degree
    threshold = np.percentile(k_arr, 99)
    hubs = [(n, d) for n, d in G_yeast.degree() if d >= threshold]
    hubs.sort(key=lambda x: -x[1])
    print(f"\nTop hub proteins (top 1%, degree ≥ {threshold:.0f}):")
    for node, degree in hubs[:5]:
        print(f"  Node {node}: degree = {degree}")

    print(f"\nNetwork fragmentation analysis:")
    # Remove top 10 hubs: catastrophic failure of scale-free network
    G_attack = G_yeast.copy()
    top_hubs = sorted(G_yeast.degree(), key=lambda x: -x[1])[:10]
    G_attack.remove_nodes_from([n for n, _ in top_hubs])
    gcc_fraction = max(len(c) for c in nx.connected_components(G_attack)) / len(G_attack)
    print(f"  After removing top 10 hubs: GCC = {gcc_fraction*100:.1f}% of network")
    print(f"  (vs. random removal: GCC stays near 100%)")

analyze_yeast_ppi()
```

## Why This Matters

The degree distribution is the entry point for understanding network vulnerability, evolution, and function. Scale-free topology explains why targeting high-degree hub proteins is an effective drug strategy — but also why it causes side effects (hub proteins have many interaction partners, so disrupting them affects many pathways). It explains why cancers that mutate hub genes like TP53 (~50% of all cancers) have such diverse downstream effects. Statistically, correctly testing whether a network is scale-free (vs. just having some highly connected nodes) requires methods beyond visual inspection of log-log plots — an important lesson in rigor that applies broadly to data analysis in computational biology.
