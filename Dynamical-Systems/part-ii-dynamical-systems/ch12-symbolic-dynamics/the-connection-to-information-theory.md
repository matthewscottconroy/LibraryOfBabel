# 12.9 The Connection to Information Theory

Throughout this chapter, we have been doing two things at once: developing symbolic dynamics and building an implicit dictionary with information theory. It is time to make that dictionary explicit.

The connection is not superficial. Symbolic dynamics and information theory are, in a precise sense, the same subject viewed from different angles. Shannon's theory of sources and channels is the probabilistic version of the topological theory of subshifts and factor maps. Here is the full translation:

| Symbolic Dynamics | Information Theory |
|---|---|
| Subshift $X$ | Stationary source (stationary stochastic process) |
| Topological entropy $h_{\text{top}}(X)$ | Maximum entropy rate achievable by any source on $X$ |
| Measure of maximal entropy | Source achieving the maximum entropy rate |
| Parry measure on an SFT | Markov source (achieving maximum entropy) |
| Sofic shift | Hidden Markov source |
| Factor map $\pi: X \to Y$ | Noisy channel (deterministic, shift-commuting) |
| Conjugacy $X \cong Y$ | Lossless, causal, invertible coding |
| Sliding block code $(N,M)$-block code | Block encoder with anticipation $N$ and delay $M$ |
| SFT property | Finite-memory (Markov) channel |
| Entropy $h(f, \xi)$ | Entropy rate of the coded process |

The deepest entry in this dictionary is the Parry measure:

**Theorem 12.9.1 (Parry Measure).** Every irreducible SFT $X_A$ has a unique measure of maximal entropy — the *Parry measure*. It is the Markov measure defined by the transition probabilities:
$$P(i \to j) = \frac{A_{ij} r_j}{\lambda_{\text{PF}} r_i},$$
where $r = (r_i)$ and $l = (l_i)$ are the right and left Perron-Frobenius eigenvectors of $A$ (so $Ar = \lambda_{\text{PF}} r$ and $l^T A = \lambda_{\text{PF}} l^T$), and the stationary distribution is $\pi_i = l_i r_i$ (normalized).

The Parry measure realizes $h_\mu(\sigma) = h_{\text{top}}(X_A) = \log \lambda_{\text{PF}}$.

What this is saying is: the measure of maximal entropy on an SFT is a Markov measure. Its transition probabilities are determined entirely by the Perron-Frobenius eigenvectors of the transition matrix. This is the symbolic dynamics version of the maximum entropy principle from information theory: among all probability measures on the SFT, the one that maximizes entropy per symbol is the Parry measure, and it is as "uniform" as the SFT structure allows.

In information theory, this corresponds to the maximum-entropy Markov source. If you have a finite-memory constraint on your source (encoded by the SFT), the maximum-entropy source that respects that constraint is the Parry measure. This is a version of the Jaynes maximum entropy principle: the least informative distribution consistent with the constraints.

The sofic-shift/hidden-Markov correspondence is equally beautiful. A sofic shift $Y = \pi(X_A)$ is the observation process of a hidden Markov model where the hidden state runs through the SFT $X_A$ and the observation function is the sliding block code $\pi$. The entropy rate of the observation process $Y$ is the entropy of the SFT $X_A$ minus the "hidden" uncertainty — a computation that requires conditioning on the hidden state.

We will make this connection precise in Chapter 24, where the ergodic asymptotic equipartition property (AEP) provides the information-theoretic foundation for the symbolic dynamics theory developed here.
