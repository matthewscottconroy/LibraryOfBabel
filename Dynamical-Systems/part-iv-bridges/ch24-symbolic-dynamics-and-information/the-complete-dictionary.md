# 24.4 The Complete Dictionary

We now have all the pieces. The full dictionary between symbolic dynamics and information theory is detailed below. This table extends the one in Chapter 22, now specialized to the setting where both sides are most explicit: symbolic dynamics on one side, source coding and channel coding theory on the other.

Read each row as a translation between two languages describing the same mathematical object.

| Symbolic Dynamics | Information Theory | Mathematical Object |
|---|---|---|
| Full $k$-shift $\mathcal{A}^{\mathbb Z}$ | i.i.d. source with $|\mathcal{A}|$ symbols | Product measure |
| Subshift $X \subseteq \mathcal{A}^{\mathbb Z}$ | Constrained stationary source | Shift-invariant support |
| SFT $X_A$ with trans. matrix $A$ | Markov source with transitions $A$ | Markov measure |
| Sofic shift | Hidden Markov source | HMM output |
| Topological entropy $h_{\text{top}}(X)$ | Max achievable entropy rate on $X$ | $\log\lambda_{\text{PF}}(A)$ |
| Parry measure | Measure achieving max entropy | Capacity-achieving input |
| KS entropy $h_\mu$ | Entropy rate of source | Limit of $\frac{1}{n}H(X_1,\ldots,X_n)$ |
| Variational principle | Max entropy rate = topological entropy | $h_{\text{top}} = \sup_\mu h_\mu$ |
| Factor map $\pi: X \to Y$ | Channel from $X$-source to $Y$-observations | Sliding block code |
| Conjugacy | Lossless coding (bijective channel) | Entropy-preserving factor |
| $\zeta$-function of $X_A$ | Generating function for codeword lengths | $1/\det(I-tA)$ |
| Sliding block code $(N,M)$ | $(N+M+1)$-block code | Block encoder |
| Automorphism group $\text{Aut}(\sigma)$ | Group of lossless self-codings | |
| Lyapunov exponent of $\sigma$ | $\log|\mathcal{A}|$ per symbol | Information rate |

A few rows deserve commentary.

**The zeta function and generating functions.** The zeta function of an SFT is $\zeta_{X_A}(t) = 1/\det(I - tA)$. Expanding as a power series, the coefficient of $t^n$ counts the number of fixed points of $\sigma^n$ — the number of allowed words of length $n$. In information theory, the generating function for codeword lengths in a Huffman code has the same form. The poles of the zeta function at $t = 1/\lambda_{\text{PF}}$ correspond to the capacity $h_{\text{top}} = \log \lambda_{\text{PF}}$ — the pole is at the radius of convergence, where the code runs out of capacity.

**The automorphism group.** $\text{Aut}(\sigma)$ is the group of all self-conjugacies of the shift — bijective sliding block codes from the subshift to itself. This is the group of "lossless recodings" of the source: different representations of the same information. For the full shift, $\text{Aut}(\sigma)$ is enormously complicated (it contains every finite group, by results of Boyle-Franks-Kitchens), reflecting the vast richness of lossless self-codings.

**The Lyapunov exponent.** For the shift map $\sigma$ on a full $k$-shift, the "Lyapunov exponent" is $\log k = \log |\mathcal{A}|$ — every symbol adds $\log k$ bits of information. For a subshift, the effective Lyapunov exponent is the topological entropy, which is generally less than $\log |\mathcal{A}|$ (because the constraint reduces the number of allowed sequences per step).

The row about conjugacy is particularly important: two subshifts are conjugate iff they are connected by a bijective sliding block code. In information theory, this is lossless coding: you can convert between the two representations without losing any information. The classification of SFTs up to conjugacy — what invariants distinguish non-conjugate SFTs — is an active research area, and entropy is the most important (but not the only) invariant.

This dictionary is not just decorative. It is a working tool. When you prove a theorem in one column, translate it to the other and ask whether it gives you something new. The history of symbolic dynamics and coding theory is full of results proved on one side and later recognized as known results on the other side, just wearing different clothes.
