# Chapter 24 — Symbolic Dynamics as Information Theory

> *A subshift is a constrained source. The topological entropy is the maximum entropy rate. A factor map is a noisy channel. The full symbolic dynamics / information theory dictionary, made rigorous.*

**Prerequisites:** Chapters 12 (symbolic dynamics), 16 (channel capacity, source coding), 23 (ergodic information theory).

---

## 24.1 Subshifts as Stationary Sources

**Definition 24.1.1.** A subshift $(X, \sigma)$ on alphabet $\mathcal{A}$ defines a *constrained source*: the set of sequences that can be produced. The source produces sequences $x \in X$ according to some $\sigma$-invariant probability measure $\mu$.

**The Encoder View:** An encoder for a constrained source must produce sequences in $X$. The rate of the source is $h_\mu(\sigma)$ (KS entropy), bounded above by $h_{\text{top}}(X) = \log\lambda_{\text{PF}}(A)$ for an SFT with transition matrix $A$.

**Theorem 24.1.2 (Source Coding for Constrained Sources).** For a subshift $X$ with topological entropy $h_{\text{top}}$ and an ergodic measure $\mu$ with $h_\mu < h_{\text{top}}$:
- Lossless coding of $\mu$-typical sequences achieves rate $h_\mu$ bits/symbol.
- Coding arbitrary sequences in $X$ requires up to $h_{\text{top}}$ bits/symbol.

---

## 24.2 Factor Maps as Channels

**Definition 24.2.1.** A *factor map* $\pi: (X, \sigma) \to (Y, \sigma)$ between two subshifts is a continuous surjection commuting with $\sigma$. By the Curtis-Hedlund-Lyndon theorem, $\pi$ is a sliding block code.

**The Channel View:** Given an input sequence $x \in X$, the factor map $\pi$ produces the "output" $y = \pi(x) \in Y$. But $\pi$ may be many-to-one (multiple inputs produce the same output) — this is the "noise" in the channel.

**Theorem 24.2.2.** For a factor map $\pi: X \to Y$:
- $h_{\text{top}}(Y) \leq h_{\text{top}}(X)$ (factor cannot increase entropy)
- $h_\mu(Y) = h_\mu(X) - h_\mu(X|Y)$ (conditional entropy measures "hidden information")

An *information-lossless* factor map has $h_{\text{top}}(X) = h_{\text{top}}(Y)$ (no entropy lost).

---

## 24.3 Hidden Markov Processes and Sofic Shifts

**Definition 24.3.1.** A *hidden Markov model (HMM)* consists of:
- A Markov chain $(S_n)$ on a finite state space (hidden states)
- An observation function: at each step, emit symbol $Y_n = g(S_n, N_n)$ (where $N_n$ is i.i.d. noise)

The output process $(Y_n)$ is the *hidden Markov process*.

**Theorem 24.3.2.** The output process of an HMM (with a finite-state Markov chain) is exactly a *sofic process* — a process whose support is a sofic shift.

*(proof)* The states of the Markov chain are the hidden states of the SFT that presents the sofic shift. The observation function is the factor map.

**Entropy of HMMs:** The entropy rate of a sofic process is $h = H(Y_n | Y_{n-1}, Y_{n-2}, \ldots)$ — the conditional entropy given all past observations. Computing this requires knowledge of the *Blackwell measure* (the stationary distribution over the belief states of the hidden Markov filter). This is generally hard and does not have a closed form.

---

## 24.4 The Complete Dictionary

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

---

## 24.5 Data Compression via Subshift Constraints

**Application:** Many data sources produce constrained sequences — magnetic recording (runlength limited codes), DNA sequences (codon constraints), natural language (grammar constraints). Modeling these as subshifts allows optimal compression.

**Rate-Constrained Coding:** Given a source $\mu$ on alphabet $\mathcal{A}$ and a constraint (subshift $X \subseteq \mathcal{B}^{\mathbb Z}$), we want to code the source into $X$. The achievable rate is:
$$R = \frac{h_\mu(\sigma_\mathcal{A})}{h_{\text{top}}(X)} \text{ input symbols per output symbol}.$$

**Theorem 24.5.1 (Constrained Coding Capacity — Marcus-Siegel, Adler-Coppersmith-Hassner).** A binary source at rate $R$ bits/symbol can be coded into a constrained channel $X$ with entropy $h_{\text{top}}(X) > R$.

Practical finite-state encoders for this (with sliding window decoding) exist iff $h_{\text{top}}(X)$ is a rational linear combination of $\log 2$.

---

## Exercises

**Exercise 24.1.** The golden mean shift has $h_{\text{top}} = \log\phi$ where $\phi = (1+\sqrt{5})/2$. Can a $\text{Bernoulli}(1/2)$ source be coded into the golden mean shift? What input rate is achievable?

**Exercise 24.2.** Verify that the output of a 2-state HMM (fair coin hidden state, with asymmetric emission probabilities) is a sofic process. Compute its entropy rate using the stationary distribution over belief states.

**Exercise 24.3.** (DNA sequence) Model DNA as a stationary process on alphabet $\{A, C, G, T\}$. If DNA has an approximate Markov structure, how does the entropy rate of DNA relate to biological compression schemes?

---

## Chapter Notes

Lind-Marcus *An Introduction to Symbolic Dynamics and Coding* covers this dictionary in depth, particularly the coding theory applications in Part III. The connection to hidden Markov processes is surveyed in Ephraim-Merhav's *Hidden Markov Processes* (*IEEE Trans. Inf. Theory*, 2002).

For constrained coding in magnetic recording: the book by Marcus, Roth, and Siegel, *Constrained Systems and Coding for Recording Channels*, is the engineering reference.
