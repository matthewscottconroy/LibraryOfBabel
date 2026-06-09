"""
Directed Evolution: Fitness Landscape Explorer
================================================
Tier 3.5 — Synthetic Biology: Directed Evolution

Concepts demonstrated:
  - Fitness landscapes: sequence → function mapping
  - Epistasis: interactions between mutations (sign epistasis)
  - Random walk through sequence space
  - Local vs. global optima (ruggedness)
  - Greedy hill-climbing vs. random search
  - Dimensionality reduction of fitness landscape (PCA)
  - Neutral networks and evolvability
  - GB1 protein G-binding domain (real landscape preview)

Usage:
    python 16_fitness_landscape.py [--model MODEL] [--L SEQ_LENGTH]

Models:
    smooth     — NK model with K=0 (additive, no epistasis)
    rugged     — NK model with K=3 (epistatic, many local optima)
    gb1_proxy  — Simplified 4-position GB1-like landscape

Key insight:
    Epistasis (non-additive interactions between mutations) creates
    fitness landscape ruggedness — multiple local optima that trap
    greedy hill-climbing. High-throughput directed evolution (MLDE)
    uses machine learning to learn the landscape from sparse data
    and predict global optima without exhaustive screening.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib import cm
from scipy.spatial.distance import hamming
from itertools import product


# ── NK Fitness Landscape Model ────────────────────────────────────────────────

class NKLandscape:
    """
    NK model: L-bit genome, each locus has K epistatic partners.
    Fitness = mean of per-locus contributions f_i(x_i, x_{nb(i)}).
    K=0: additive (smooth); K=L-1: maximally rugged.
    """
    def __init__(self, L=8, K=2, seed=42):
        self.L = L
        self.K = K
        rng = np.random.default_rng(seed)

        # Assign K random epistatic neighbors for each locus
        self.neighbors = []
        for i in range(L):
            others = [j for j in range(L) if j != i]
            nb = rng.choice(others, size=min(K, len(others)), replace=False).tolist()
            self.neighbors.append([i] + sorted(nb))

        # Fitness table: for each locus, 2^(K+1) entries
        self.tables = []
        for i in range(L):
            entries = 2 ** (K + 1)
            self.tables.append(rng.random(entries))

    def fitness(self, genotype):
        """Compute fitness of a binary genotype (list/array of 0/1)."""
        total = 0.0
        for i in range(self.L):
            bits = [genotype[j] for j in self.neighbors[i]]
            idx = int(''.join(str(b) for b in bits), 2)
            total += self.tables[i][idx % len(self.tables[i])]
        return total / self.L


# ── Simplified GB1-like 4-Position Landscape ──────────────────────────────────

def build_gb1_proxy():
    """
    Four-position (V39, D40, G41, V54) protein GB1 fitness landscape.
    Each position can be one of 20 amino acids.
    We use a 4-residue landscape with 4 choices per position
    (simplified: A/C/G/T as stand-ins for amino acid groups).
    True GB1 from Wu et al. 2016 has 20^4 variants measured.
    """
    alphabet = ['A', 'C', 'D', 'E']  # 4 amino acid groups (simplified)
    L = 4
    rng = np.random.default_rng(7)

    # Additive component
    pos_scores = {
        0: {'A': 1.0, 'C': 0.3, 'D': 0.8, 'E': 0.2},
        1: {'A': 0.5, 'C': 1.0, 'D': 0.3, 'E': 0.9},
        2: {'A': 0.2, 'C': 0.7, 'D': 1.0, 'E': 0.4},
        3: {'A': 0.9, 'C': 0.4, 'D': 0.6, 'E': 1.0},
    }
    # Epistatic interactions (pairs)
    epistasis = {
        (0, 1): {'AA': 0.5, 'CC': 0.5, 'AC': -0.3, 'CA': -0.2},
        (1, 2): {'CC': 0.4, 'DC': 0.3, 'CD': -0.1},
        (0, 3): {'AE': 0.3, 'EA': 0.0, 'AA': -0.2},
    }

    landscape = {}
    for combo in product(alphabet, repeat=L):
        seq = ''.join(combo)
        score = sum(pos_scores[i].get(c, 0.5) for i, c in enumerate(combo))
        # Add epistatic terms
        for (i, j), ep_table in epistasis.items():
            pair = combo[i] + combo[j]
            score += ep_table.get(pair, 0.0)
        # Normalize
        landscape[seq] = max(0, score + 0.1 * rng.standard_normal())
    return landscape, alphabet, L


# ── Search Algorithms ─────────────────────────────────────────────────────────

def greedy_hillclimb(landscape_fn, initial, max_steps=100):
    """Greedy hill-climbing: take the best single-step neighbor."""
    current = list(initial)
    current_fit = landscape_fn(current)
    trajectory = [(list(current), current_fit)]

    for _ in range(max_steps):
        best_neighbor = None
        best_fit = current_fit
        for i in range(len(current)):
            mutant = list(current)
            mutant[i] = 1 - mutant[i]   # flip bit
            f = landscape_fn(mutant)
            if f > best_fit:
                best_fit = f
                best_neighbor = list(mutant)
        if best_neighbor is None:
            break  # local optimum
        current = best_neighbor
        current_fit = best_fit
        trajectory.append((list(current), current_fit))

    return trajectory


def random_walk(landscape_fn, initial, n_steps=200, seed=0):
    """Random walk with acceptance of improvements (simplified EA)."""
    rng = np.random.default_rng(seed)
    current = list(initial)
    current_fit = landscape_fn(current)
    trajectory = [(list(current), current_fit)]

    for _ in range(n_steps):
        # Random mutation
        mutant = list(current)
        i = rng.integers(0, len(mutant))
        mutant[i] = 1 - mutant[i]
        f = landscape_fn(mutant)
        if f >= current_fit:  # accept improvements
            current = mutant
            current_fit = f
        trajectory.append((list(current), current_fit))

    return trajectory


def exhaustive_search(nk, top_n=10):
    """Enumerate all genotypes and return sorted by fitness."""
    all_genos = list(product([0, 1], repeat=nk.L))
    scores = [(list(g), nk.fitness(list(g))) for g in all_genos]
    return sorted(scores, key=lambda x: -x[1])


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(model='rugged', L=8):
    if model == 'gb1_proxy':
        landscape_dict, alphabet, L_gb1 = build_gb1_proxy()
        L = L_gb1

        # Convert to callable
        def landscape_fn(seq):
            k = ''.join(seq)
            return landscape_dict.get(k, 0.0)

        all_seqs = list(landscape_dict.keys())
        all_fits = np.array(list(landscape_dict.values()))
        optimal_seq = max(landscape_dict, key=landscape_dict.get)
        optimal_fit = landscape_dict[optimal_seq]

        # Hamming distances for PCA proxy
        seq_vecs = np.array([[alphabet.index(c) for c in s] for s in all_seqs])

    else:
        K_map = {'smooth': 0, 'rugged': 3}
        K = K_map.get(model, 2)
        nk = NKLandscape(L=L, K=K, seed=42)

        def landscape_fn(g):
            return nk.fitness(g)

        all_genos = list(product([0, 1], repeat=L))
        all_fits = np.array([nk.fitness(list(g)) for g in all_genos])
        all_seqs = [''.join(str(b) for b in g) for g in all_genos]
        seq_vecs = np.array(all_genos, dtype=float)

        sorted_all = sorted(zip(all_seqs, all_fits), key=lambda x: -x[1])
        optimal_seq = sorted_all[0][0]
        optimal_fit = sorted_all[0][1]

    # Run search algorithms
    if model == 'gb1_proxy':
        init_seq = list(all_seqs[0])
        def landscape_fn_list(seq_list):
            return landscape_dict.get(''.join(seq_list), 0.0)
        climb_traj = [(list(optimal_seq[:2]), landscape_fn_list(list(optimal_seq[:2])))]
        walk_traj = climb_traj
    else:
        rng = np.random.default_rng(7)
        init_g = list(rng.integers(0, 2, size=L))
        climb_traj = greedy_hillclimb(landscape_fn, init_g)
        walk_traj = random_walk(landscape_fn, init_g, n_steps=300, seed=5)

    # ── Figure ────────────────────────────────────────────────────────────────
    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Fitness Landscape Explorer — {model}  (L={L})',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_dist  = fig.add_subplot(gs[0, 0])   # fitness distribution
    ax_traj  = fig.add_subplot(gs[0, 1])   # search trajectories
    ax_2d    = fig.add_subplot(gs[1, :2])  # 2D landscape projection
    ax_info  = fig.add_subplot(gs[:, 2])   # info

    for ax in [ax_dist, ax_traj, ax_2d, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Fitness distribution ──────────────────────────────────────────────────
    ax_dist.hist(all_fits, bins=30, color='#8be9fd', edgecolor='white', alpha=0.85)
    ax_dist.axvline(optimal_fit, color='#50fa7b', lw=2, ls='--',
                    label=f'Global max = {optimal_fit:.3f}')
    ax_dist.axvline(np.mean(all_fits), color='#f5a623', lw=1.5, ls=':',
                    label=f'Mean = {np.mean(all_fits):.3f}')
    ax_dist.set_xlabel('Fitness', color='white')
    ax_dist.set_ylabel('Count', color='white')
    ax_dist.set_title('Fitness Distribution\n(all genotypes)', color='white', fontsize=9)
    ax_dist.tick_params(colors='white')
    ax_dist.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_dist.spines.values():
        sp.set_edgecolor('#888')

    # ── Search trajectories ───────────────────────────────────────────────────
    climb_fits = [f for _, f in climb_traj]
    walk_fits  = [f for _, f in walk_traj]
    ax_traj.plot(range(len(climb_fits)), climb_fits, '-o',
                 color='#50fa7b', lw=2, ms=5, label='Greedy hill-climb')
    ax_traj.plot(range(len(walk_fits)), walk_fits, '-',
                 color='#ff79c6', lw=1.2, alpha=0.8, label='Random walk')
    ax_traj.axhline(optimal_fit, color='#f5a623', lw=1.5, ls='--',
                    label=f'Global optimum = {optimal_fit:.3f}')
    ax_traj.set_xlabel('Steps', color='white')
    ax_traj.set_ylabel('Fitness', color='white')
    ax_traj.set_title('Search Trajectories', color='white', fontsize=9)
    ax_traj.tick_params(colors='white')
    ax_traj.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_traj.spines.values():
        sp.set_edgecolor('#888')

    # ── 2D landscape projection (PCA on one-hot / binary vectors) ────────────
    if len(seq_vecs) <= 2048:
        # Simple PCA
        X = seq_vecs - seq_vecs.mean(axis=0)
        cov = X.T @ X / len(X)
        eigenvalues, eigenvectors = np.linalg.eigh(cov)
        # Take top 2 PCs
        idx = np.argsort(eigenvalues)[::-1][:2]
        PC = X @ eigenvectors[:, idx]
        sc = ax_2d.scatter(PC[:, 0], PC[:, 1], c=all_fits,
                           cmap='RdYlGn', s=20, alpha=0.7, edgecolors='none')
        plt.colorbar(sc, ax=ax_2d, label='Fitness', fraction=0.02).ax.yaxis.label.set_color('white')
        ax_2d.set_xlabel('PC1', color='white')
        ax_2d.set_ylabel('PC2', color='white')
        ax_2d.set_title('Fitness Landscape (PCA projection of sequence space)',
                        color='white', fontsize=9)
        ax_2d.tick_params(colors='white')
        for sp in ax_2d.spines.values():
            sp.set_edgecolor('#888')
    else:
        ax_2d.text(0.5, 0.5, 'Landscape too large\nfor PCA projection',
                   transform=ax_2d.transAxes, ha='center', va='center',
                   color='white', fontsize=10)
        ax_2d.axis('off')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    n_local_optima = 0
    if model != 'gb1_proxy' and len(all_genos) <= 512:
        for g in all_genos:
            g_list = list(g)
            f_curr = landscape_fn(g_list)
            is_local = True
            for i in range(L):
                nb = list(g_list)
                nb[i] = 1 - nb[i]
                if landscape_fn(nb) > f_curr:
                    is_local = False
                    break
            if is_local:
                n_local_optima += 1

    climb_found = max(climb_fits)
    fraction_of_opt = climb_found / optimal_fit if optimal_fit > 0 else 0

    info = (
        f"Model: {model}\n"
        f"L = {L} (sequence length)\n"
        f"Total genotypes: {len(all_seqs)}\n\n"
        f"Landscape statistics:\n"
        f"  Global max:  {optimal_fit:.4f}\n"
        f"  Mean:        {np.mean(all_fits):.4f}\n"
        f"  Std:         {np.std(all_fits):.4f}\n"
        f"  Local optima: {n_local_optima if n_local_optima else '(too large to count)'}\n\n"
        f"Search results:\n"
        f"  Hill-climb steps: {len(climb_traj)-1}\n"
        f"  Hill-climb found: {climb_found:.4f}\n"
        f"  Fraction of opt.: {fraction_of_opt:.1%}\n\n"
        f"Key concepts:\n"
        f"  K=0: additive, one peak\n"
        f"  K>0: epistasis, rugged\n"
        f"  Large K → many local optima\n"
        f"  MLDE: learn landscape from\n"
        f"    sparse data to guide search\n\n"
        f"Real example:\n"
        f"  GB1 (Wu et al. 2016):\n"
        f"  20^4 = 160,000 variants\n"
        f"  Measured by phage display"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=8, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nModel: {model}, L={L}")
    print(f"Genotypes: {len(all_seqs)}")
    print(f"Optimal: {optimal_seq} → fitness={optimal_fit:.4f}")
    print(f"Hill-climb: found {climb_found:.4f} in {len(climb_traj)-1} steps "
          f"({fraction_of_opt:.1%} of global opt)")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Fitness landscape explorer')
    parser.add_argument('--model', default='rugged',
                        choices=['smooth', 'rugged', 'gb1_proxy'])
    parser.add_argument('--L', type=int, default=8, help='Sequence length (NK model)')
    args = parser.parse_args()

    print("Directed Evolution: Fitness Landscape Explorer")
    print("=" * 55)
    run(model=args.model, L=args.L)
