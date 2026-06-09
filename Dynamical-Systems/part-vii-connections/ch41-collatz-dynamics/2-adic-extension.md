# 41.2 2-Adic Extension and Ergodicity

The natural home for the Collatz map is not $\mathbb{N}$ but the 2-adic integers $\mathbb{Z}_2$. The 2-adic integers are the completion of $\mathbb{Z}$ with respect to the 2-adic absolute value, which measures how divisible by 2 a number is. In $\mathbb{Z}_2$, numbers that differ only in high-order binary digits are close together — the opposite of the usual distance.

**Definition 41.2.1.** The *2-adic integers* ${\mathbb Z}_2 = \varprojlim {\mathbb Z}/2^n{\mathbb Z}$ is the completion of ${\mathbb Z}$ with respect to the 2-adic absolute value $|n|_2 = 2^{-v_2(n)}$ (where $v_2(n)$ is the 2-adic valuation).

As a set: ${\mathbb Z}_2 = \{x = \sum_{i=0}^\infty a_i 2^i : a_i \in \{0,1\}\}$ (formal power series in $2$).

Think of 2-adic integers as infinite sequences of binary digits, where the "significant" digits are on the left (the low-order bits). The 2-adic absolute value of $n$ is small if $n$ is divisible by a high power of 2.

**Theorem 41.2.2 (2-Adic Extension of $C$).** The Collatz map $C$ extends to a continuous map $C: {\mathbb Z}_2 \to {\mathbb Z}_2$ given by:
$$C(x) = \begin{cases}x/2 & \text{if } x \equiv 0 \pmod 2 \\ (3x+1)/2 & \text{if } x \equiv 1 \pmod 2\end{cases} \quad (\text{accelerated map})$$

The *accelerated* version $(3x+1)/2$ (combining two steps for odd $n$) is smoother in ${\mathbb Z}_2$.

The continuity is the key point: in the 2-adic topology, the Collatz map is continuous. This means $\mathbb{Z}_2$ is the right space to study the Collatz map — it's the completion that makes the map well-behaved.

**Theorem 41.2.3 (Ergodicity of $C$ on ${\mathbb Z}_2$).** The accelerated Collatz map $\tilde{C}: {\mathbb Z}_2 \to {\mathbb Z}_2$ is measure-preserving and ergodic with respect to Haar measure on ${\mathbb Z}_2$.

*(sketch)* The Haar measure on ${\mathbb Z}_2$ is the unique probability measure invariant under translation. The map $\tilde{C}$ locally expands by factor $3/2$ on odd elements and contracts by $1/2$ on even — on average, $\tilde{C}$ is "neutral." The ergodicity is proved using the spectral gap of the associated Markov operator.

**Interpretation:** Ergodicity of $C$ on ${\mathbb Z}_2$ means that a "random" 2-adic integer has an orbit that visits every measurable set with the correct frequency. The Collatz conjecture for positive integers asks about the behavior of the *positive* integers within ${\mathbb Z}_2$ — a measure-zero subset.

Here's the fundamental difficulty: ergodicity is a property of the Haar measure on $\mathbb{Z}_2$. The positive integers $\mathbb{N} \subset \mathbb{Z}_2$ have Haar measure zero. So the ergodicity theorem tells us about the typical 2-adic integer, not about positive integers specifically. The Collatz conjecture is a statement about a measure-zero exceptional set.
