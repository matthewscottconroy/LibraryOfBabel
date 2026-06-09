# 12.4 Topological Entropy of Subshifts

For a general dynamical system, topological entropy is defined via open covers — a somewhat abstract construction. For subshifts, we get a much more concrete formula: the entropy is the exponential growth rate of the number of allowed words. This makes entropy directly computable from the combinatorics of the language.

The right tool is the *word complexity function*.

**Definition 12.4.1.** The *word complexity function* $p_X(n)$ counts the number of distinct allowed words of length $n$:
$$p_X(n) = |\{w \in \mathcal{A}^n : w \in \mathcal{L}(X)\}|.$$

For the full shift on $k$ symbols, $p_X(n) = k^n$. For the golden mean shift, $p_X(n)$ is roughly $(1+\sqrt{5})^n/2^n$ — Fibonacci-like growth. For a periodic orbit, $p_X(n)$ is eventually constant.

**Theorem 12.4.2.** The topological entropy of a subshift $X$ is:
$$h_{\text{top}}(X) = \lim_{n \to \infty} \frac{1}{n} \log p_X(n).$$

The limit exists by subadditivity: $p_X(m+n) \leq p_X(m) \cdot p_X(n)$ (each word of length $m+n$ is an allowed word of length $m$ followed by an allowed word of length $n$, so the count cannot exceed the product). Fekete's lemma then guarantees the limit exists and equals $\inf_n \frac{1}{n}\log p_X(n)$.

What this is saying is: topological entropy measures how fast the dictionary grows. Each additional symbol in a word either adds new possibilities or doesn't — and the per-symbol rate at which new words appear is exactly the entropy.

**Theorem 12.4.3 (Entropy of SFTs).** For an irreducible vertex SFT with transition matrix $A$:
$$h_{\text{top}}(X_A) = \log \lambda_{\text{PF}}(A),$$
where $\lambda_{\text{PF}}(A)$ is the Perron-Frobenius eigenvalue (the largest eigenvalue).

*Proof:* The number of allowed paths of length $n$ in the directed graph is $\sum_{i,j} (A^n)_{ij} = \mathbf{1}^T A^n \mathbf{1}$. By the Perron-Frobenius theorem, $(A^n)_{ij} \sim c_{ij} \lambda_{\text{PF}}^n$ for constants $c_{ij} > 0$. Therefore $p_X(n) \sim C \lambda_{\text{PF}}^n$ and $h_{\text{top}} = \log \lambda_{\text{PF}}$.

The Perron-Frobenius theorem — which says that an irreducible non-negative matrix has a unique largest real eigenvalue with positive eigenvectors — is the key algebraic fact underlying much of symbolic dynamics. It guarantees both the existence of a well-defined entropy and the existence of the Parry measure (the measure of maximal entropy, constructed from the Perron-Frobenius eigenvectors in Section 12.9).

For the golden mean shift: $\lambda_{\text{PF}}(A) = (1+\sqrt{5})/2$ and $h_{\text{top}} = \log(1+\sqrt{5})/2 = \log \phi$. For the full 2-shift: $\lambda_{\text{PF}} = 2$ and $h_{\text{top}} = \log 2$. The entropy of an SFT is computable exactly, in closed form, from the characteristic polynomial of its transition matrix.
