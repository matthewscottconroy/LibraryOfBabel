# 24.2 Factor Maps as Channels

A factor map between two subshifts takes input sequences and produces output sequences. From the information-theoretic perspective, this is exactly a channel: it transforms a source signal into a channel output, possibly with information loss.

The information loss is what makes the analogy interesting. A bijective factor map (a conjugacy) is a lossless channel — no information is lost, and the two systems are informationally equivalent. A proper factor map (many-to-one) loses some information — the channel is "noisy" in the sense that multiple input sequences produce the same output.

**Definition 24.2.1.** A *factor map* $\pi: (X, \sigma) \to (Y, \sigma)$ between two subshifts is a continuous surjection commuting with $\sigma$. By the Curtis-Hedlund-Lyndon theorem, $\pi$ is a sliding block code.

The Curtis-Hedlund-Lyndon theorem is the key: every factor map is a sliding block code. This means there is a window of size $N+M+1$ such that the output symbol at position $n$ is determined by the input symbols at positions $n-N, \ldots, n+M$. The channel is *finite memory* — it looks at a finite window of the input to produce each output symbol.

**The Channel View:** Given an input sequence $x \in X$, the factor map $\pi$ produces the "output" $y = \pi(x) \in Y$. But $\pi$ may be many-to-one (multiple inputs produce the same output) — this is the "noise" in the channel.

**Theorem 24.2.2.** For a factor map $\pi: X \to Y$:
- $h_{\text{top}}(Y) \leq h_{\text{top}}(X)$ (factor cannot increase entropy)
- $h_\mu(Y) = h_\mu(X) - h_\mu(X|Y)$ (conditional entropy measures "hidden information")

An *information-lossless* factor map has $h_{\text{top}}(X) = h_{\text{top}}(Y)$ (no entropy lost).

The first statement is the data processing inequality for topological entropy: you can't create information by applying a deterministic function. More precisely, the topological entropy of the output is at most that of the input, because any $(n,\varepsilon)$-separated set in $X$ maps to a separated set in $Y$ (of the same or smaller size).

The second statement is the chain rule for entropy: $h_\mu(X) = h_\mu(Y) + h_\mu(X|Y)$. The term $h_\mu(X|Y)$ is the conditional entropy — the information in $X$ that is not visible in $Y$. This is precisely the "hidden" information lost by the channel.

The conditional entropy $h_\mu(X|Y)$ has a beautiful dynamical interpretation: it is the entropy of the equivalence relation $\sim_\pi$ (where $x \sim_\pi x'$ iff $\pi(x) = \pi(x')$) — the entropy of the "fiber" structure of the factor map. If the fibers are all singletons (conjugacy), the conditional entropy is 0. If the fibers are large and complex, the conditional entropy is positive.

Information-lossless factor maps are special: they have $h_{\text{top}}(X) = h_{\text{top}}(Y)$ even though $\pi$ may not be bijective. These correspond to channels that preserve the *capacity* even while not being injective on sequences. In coding theory, these are called *right-resolving* encoders: given the current output symbol and state, the encoder can determine the input uniquely. The right-resolving property ensures that the encoding preserves all information about the source, even though different input sequences may produce the same output.

The full theory of sliding block codes and their information-theoretic properties is developed in Lind and Marcus (see Chapter Notes). The key insight is that the algebraic properties of the code (resolving, finite-to-one, entropy-preserving) have direct information-theoretic interpretations. Coding theory for constrained systems is applied ergodic theory.
