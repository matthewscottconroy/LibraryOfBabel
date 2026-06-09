# 5.6 The Perron-Frobenius Theorem

The Perron-Frobenius theorem is one of the most important theorems in applied mathematics. It says that a positive matrix has a dominant positive eigenvalue with a positive eigenvector — and that the matrix's powers converge to a rank-1 projection. It's the finite-dimensional prototype for the spectral theory of transfer operators and the foundation for computing entropy in symbolic dynamics.

**Theorem 5.6.1 (Perron-Frobenius).** Let $A \in M_n(\mathbb{R})$ have all entries $> 0$ (strictly positive matrix). Then:
1. $A$ has a real positive eigenvalue $\lambda_{\text{PF}}$ (the *Perron eigenvalue*) with $\lambda_{\text{PF}} > |\lambda|$ for all other eigenvalues $\lambda$.
2. The eigenspace for $\lambda_{\text{PF}}$ is one-dimensional with eigenvector $v > 0$ (all components positive).
3. $A^n / \lambda_{\text{PF}}^n \to v w^T$ (in a suitable sense) where $w^T A = \lambda_{\text{PF}} w^T$.

For nonneg irreducible $A$: $\lambda_{\text{PF}} = \max\{|\lambda| : \lambda \in \sigma(A)\}$ (still simple).

What this is really saying: for a strictly positive matrix, there's a unique "dominant" eigenvalue — bigger in absolute value than all others. The corresponding eigenvector is positive (all components positive), and the matrix's powers, normalized by $\lambda_{\text{PF}}^n$, converge to a rank-1 matrix. In probabilistic terms: if $A$ is a transition matrix for a Markov chain with strictly positive entries (every transition is possible), there's a unique stationary distribution, and the chain mixes to it exponentially fast.

The theorem has a clean proof strategy: consider the function $r(v) = \min_i (Av)_i / v_i$ on the positive orthant. This function increases under the action of $A$ (since $A$ is positive), and a compactness argument shows it attains its maximum at the Perron eigenvector. The spectral gap — the ratio $|\lambda_2| / \lambda_{\text{PF}}$ where $\lambda_2$ is the second eigenvalue — controls how fast the convergence in (3) happens.

**Application in Symbolic Dynamics.** For a subshift of finite type with irreducible transition matrix $A \in M_k(\{0,1\})$, the topological entropy is $h = \log \lambda_{\text{PF}}(A)$. This is one of the most elegant formulas in dynamical systems: the complexity of the symbolic system (measured by entropy) is the logarithm of the spectral radius of the transition matrix.

The *Parry measure* (the measure of maximal entropy for the subshift) is given by the left and right Perron-Frobenius eigenvectors: if $Av = \lambda_{\text{PF}} v$ and $w^T A = \lambda_{\text{PF}} w^T$, then the measure assigns weight $w_i v_j / (\lambda_{\text{PF}} w \cdot v)$ to the cylinder set $[i \to j]$. The left eigenvector $w$ gives the probability of being in state $i$; the right eigenvector $v$ gives the "future weight" of state $j$.

The Perron-Frobenius theorem is the finite-dimensional version of the Ruelle-Perron-Frobenius theorem for transfer operators, which gives the spectral gap and mixing rate for expanding maps and hyperbolic systems. We'll develop that theory in Chapter 12.
