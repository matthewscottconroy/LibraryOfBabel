# 24.1 Subshifts as Stationary Sources

A subshift is a set of sequences that satisfies certain forbidden-pattern constraints and is closed under the shift. From the information-theoretic viewpoint, this is exactly a *constrained source*: a source that can only produce certain sequences, because the channel or storage medium imposes constraints on what sequences are legal.

This situation is pervasive in engineering. A hard disk drive cannot store arbitrary binary sequences: the read/write physics require that consecutive 1's not appear too many times in a row (runlength limits), to ensure reliable timing recovery. A digital video stream must satisfy coding constraints for error detection. DNA has codon structure — not all four-symbol sequences over $\{A, C, G, T\}$ appear with equal probability, and some patterns are biologically impossible. In all these cases, the source is constrained to live in a subshift.

**Definition 24.1.1.** A subshift $(X, \sigma)$ on alphabet $\mathcal{A}$ defines a *constrained source*: the set of sequences that can be produced. The source produces sequences $x \in X$ according to some $\sigma$-invariant probability measure $\mu$.

The measure $\mu$ is the statistical description of the source — it tells you which allowed sequences appear and how often. The topological entropy $h_{\text{top}}(X)$ is the maximum possible entropy rate for any measure supported on $X$.

**The Encoder View:** An encoder for a constrained source must produce sequences in $X$. The rate of the source is $h_\mu(\sigma)$ (KS entropy), bounded above by $h_{\text{top}}(X) = \log\lambda_{\text{PF}}(A)$ for an SFT with transition matrix $A$.

Here $\lambda_{\text{PF}}(A)$ is the Perron-Frobenius eigenvalue, the largest real eigenvalue of the non-negative matrix $A$. This is both a spectral quantity (determined by linear algebra) and an information quantity (determined by the orbit structure). The Perron-Frobenius theorem is doing double duty.

The relationship $h_{\text{top}} = \log \lambda_{\text{PF}}$ is worth dwelling on. The number of allowed words of length $n$ in an SFT with transition matrix $A$ is the sum of entries of $A^n$, which grows like $\lambda_{\text{PF}}^n$. So $h_{\text{top}} = \lim_n \frac{1}{n} \log(\text{number of length-}n\text{ words}) = \log \lambda_{\text{PF}}$. This is the information-theoretic content: the number of distinct messages of length $n$ grows exponentially at rate $h_{\text{top}}$, so $h_{\text{top}}$ bits per symbol are needed to encode an arbitrary allowed sequence.

**Theorem 24.1.2 (Source Coding for Constrained Sources).** For a subshift $X$ with topological entropy $h_{\text{top}}$ and an ergodic measure $\mu$ with $h_\mu < h_{\text{top}}$:
- Lossless coding of $\mu$-typical sequences achieves rate $h_\mu$ bits/symbol.
- Coding arbitrary sequences in $X$ requires up to $h_{\text{top}}$ bits/symbol.

The gap between $h_\mu$ and $h_{\text{top}}$ is the slack between the actual source distribution and the worst-case constrained source. If the source only uses a fraction of the allowed sequences — for example, if it has strong statistical regularities beyond the hard constraints — then it can be compressed below $h_{\text{top}}$.

The two extremes: if $\mu$ is the Parry measure (the MME), then $h_\mu = h_{\text{top}}$ and no compression below $h_{\text{top}}$ is possible. If $\mu$ concentrates on a single periodic orbit, then $h_\mu = 0$ and the source is entirely predictable.

For the golden mean shift (forbidden word: $11$), the topological entropy is $h_{\text{top}} = \log \phi$ where $\phi = (1+\sqrt{5})/2 \approx 1.618$. In nats: $h_{\text{top}} \approx 0.481$. In bits: $h_{\text{top}} \approx 0.694$ bits per symbol. This means a binary source at rate $0.694$ bits/symbol can just barely be encoded into the golden mean shift — at any higher rate, there aren't enough allowed sequences to encode all messages.
