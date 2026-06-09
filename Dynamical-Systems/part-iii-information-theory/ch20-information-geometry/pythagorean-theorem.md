# 20.4 KL Divergence and the Pythagorean Theorem

In Euclidean geometry, the Pythagorean theorem says: if you project $q$ onto a flat subspace $\mathcal{E}$ to get $p^*$, then the squared distances satisfy $\|p^* - q\|^2 + \|p - p^*\|^2 = \|p - q\|^2$ for all $p \in \mathcal{E}$. This is the orthogonality of the projection.

KL divergence satisfies an exact analogue — and the analogy is not just formal, it reflects the deep geometry of the dual connections.

**Theorem 20.4.1 (Pythagorean Theorem for KL Divergence).** Let $\mathcal{E}$ be an $e$-flat submanifold of $\mathcal{S}$ and $q \in \mathcal{S}$. Let $p^*$ be the $m$-projection of $q$ onto $\mathcal{E}$ (the point minimizing $D_{\text{KL}}(p \| q)$ over $p \in \mathcal{E}$). Then:
$$D_{\text{KL}}(p^* \| q) + D_{\text{KL}}(p \| p^*) = D_{\text{KL}}(p \| q) \quad \text{for all } p \in \mathcal{E}.$$

Similarly: if $p^*$ is the $e$-projection of $q$ onto an $m$-flat submanifold, the analogous identity holds with the roles of the two arguments of $D_{\text{KL}}$ swapped.

**Interpretation:** This is exact — not an approximation. The KL divergence "triangles" in statistical manifolds satisfy Pythagoras' theorem perfectly, provided one side is the $m$-projection (or $e$-projection) onto a flat submanifold, and "orthogonality" is interpreted via the dual connections.

In Euclidean terms: $D_{\text{KL}}(p \| q)$ plays the role of $\|p - q\|^2$, the $m$-projection plays the role of orthogonal projection, and the Pythagorean identity is the statement that the projection is orthogonal.

**Application — EM Algorithm:**

The *Expectation-Maximization (EM) algorithm* for maximum likelihood estimation admits a beautiful geometric interpretation. The problem is to minimize $D_{\text{KL}}(p_{\text{data}} \| p_\theta)$ over a parametric model $\theta \in \Theta$ — to find the model closest (in KL divergence) to the empirical distribution.

Geometrically, the EM algorithm is *alternating $m$- and $e$-projections*:

1. **E-step:** $m$-project the current parameter estimate onto the manifold of complete-data distributions. This amounts to computing the expected complete-data log-likelihood (the Q-function).
2. **M-step:** $e$-project back onto the parametric family. This maximizes the Q-function over $\theta$.

By the Pythagorean theorem, each step decreases $D_{\text{KL}}(p_{\text{data}} \| p_\theta)$ (or keeps it the same). The algorithm converges to a local minimum of KL divergence, which is a local maximum likelihood estimate.

This geometric view explains:
- *Why EM converges*: each step decreases a well-defined objective.
- *Why EM can be slow*: alternating projections converge slowly when the two flat submanifolds are nearly parallel.
- *How to accelerate EM*: use natural gradient (Section 20.6) or directly exploit the geometry.

The EM algorithm is ubiquitous in machine learning: it underlies training of mixture models, hidden Markov models, factor analysis, probabilistic PCA, and many variational inference methods. The information-geometric perspective unifies all of these as instances of the same alternating projection algorithm.
