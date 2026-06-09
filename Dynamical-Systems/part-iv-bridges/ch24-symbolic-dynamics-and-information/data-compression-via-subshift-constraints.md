# 24.5 Data Compression via Subshift Constraints

The abstract dictionary becomes engineering when we ask: given a source and a constrained channel, how do we encode the source into the channel efficiently?

Many data sources produce constrained sequences. Magnetic recording (hard drives, tapes) uses runlength-limited (RLL) codes: the storage medium can only reliably handle sequences where consecutive same symbols don't run too long. DNA sequences have codon structure: not all length-3 words over $\{A, C, G, T\}$ code for amino acids, and the constraint structure affects the entropy rate. Natural language has grammar constraints: not all sequences of words are grammatical sentences. In each case, the source lives in (or is targeted toward) a subshift, and optimal coding requires understanding the subshift's entropy.

**Application:** Many data sources produce constrained sequences — magnetic recording (runlength limited codes), DNA sequences (codon constraints), natural language (grammar constraints). Modeling these as subshifts allows optimal compression.

**Rate-Constrained Coding:** Given a source $\mu$ on alphabet $\mathcal{A}$ and a constraint (subshift $X \subseteq \mathcal{B}^{\mathbb Z}$), we want to code the source into $X$. The achievable rate is:
$$R = \frac{h_\mu(\sigma_\mathcal{A})}{h_{\text{top}}(X)} \text{ input symbols per output symbol}.$$

This formula has a clean interpretation: you need $h_\mu$ bits per source symbol, and each constrained output symbol can carry up to $h_{\text{top}}(X)$ bits. The ratio $R$ is how many source symbols you can pack per channel symbol.

For a $\text{Bernoulli}(1/2)$ source ($h_\mu = \log 2 = 1$ bit/symbol) into the golden mean shift ($h_{\text{top}} = \log\phi \approx 0.694$ bits/symbol): the ratio is $R = 1/0.694 \approx 1.44$ source symbols per channel symbol — that is, you need 1.44 channel uses per source bit. Equivalently, the encoder must expand the sequence by a factor of $\approx 1.44$.

**Theorem 24.5.1 (Constrained Coding Capacity — Marcus-Siegel, Adler-Coppersmith-Hassner).** A binary source at rate $R$ bits/symbol can be coded into a constrained channel $X$ with entropy $h_{\text{top}}(X) > R$.

Practical finite-state encoders for this (with sliding window decoding) exist iff $h_{\text{top}}(X)$ is a rational linear combination of $\log 2$.

The second condition is the practical crux. For rational entropy — $h_{\text{top}}(X) = (p/q) \log 2$ for integers $p, q$ — the encoder can be built as a finite-state machine that processes $q$ input bits at a time and produces $p$ output symbols. The "p/q code" maps each $q$-bit input block to one of at most $2^q$ possible $p$-symbol output blocks, all chosen from the allowed words of $X$.

Adler, Coppersmith, and Hassner's state-splitting algorithm (1983) constructs such encoders automatically from the SFT. It's one of the most elegant constructive proofs in the field: you start with the Perron-Frobenius eigenvector of the transition matrix and iteratively split states until the branching numbers are all equal to $2^q$, giving a balanced encoder.

The algorithm was implemented in every hard drive built from the mid-1980s through the 2000s. The $d = 1, k = 7$ (d,k) RLL code — which forbids runs of more than 7 zeros and requires at most 1 zero between ones — was the industry standard, and the Adler-Coppersmith-Hassner algorithm was how the finite-state encoder was designed.

This is the point where pure mathematics (subshift theory, Perron-Frobenius) became engineering (hard drive coding). The connection is not superficial: the entire theory of constrained codes is symbolic dynamics applied to a channel capacity problem.
