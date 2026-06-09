# 18.3 Prefix-Free Complexity

Plain Kolmogorov complexity $C$ is natural but has an annoying technical feature: the chain rule acquires a logarithmic overhead. When you try to describe a pair $(x, y)$ from descriptions of $x$ and $y$ separately, you need extra bits to tell the decoder where the description of $x$ ends and that of $y$ begins. This gives $C(x, y) \leq C(x) + C(y) + 2\log|x|$ — a $2\log$ overhead that clutters formulas.

The fix is to restrict to *prefix-free* machines.

**Definition 18.3.1 (Prefix-Free Complexity).** A *prefix-free machine* is a Turing machine whose domain (set of valid inputs) is a prefix-free set. The *prefix-free Kolmogorov complexity* $K(x)$ is defined using a universal prefix-free machine.

Because the domain is prefix-free, a prefix-free machine can "self-delimit" its programs: the machine knows when it has read enough input to halt, without needing an explicit end marker. This is the key to eliminating the logarithmic overhead.

**Theorem 18.3.2 (Kraft Inequality for Complexity).** $\sum_{x \in \{0,1\}^*} 2^{-K(x)} \leq 1$.

This is the analogue of the Kraft inequality for prefix-free codes: $K$ defines a valid probability measure (the algorithmic probability, Section 18.4). The strings $x$ and their complexities $K(x)$ satisfy the same structural constraint as codewords and their lengths in a prefix-free code.

**Key Properties of $K$:**
- $K(x) \leq |x| + O(1)$ (the identity program is short).
- $K(x) \leq K(y) + K(x \mid y) + O(1)$ (given $y$, you can describe $x$ using a description of $x$ conditional on $y$).
- $K(x, y) = K(x) + K(y \mid x) + O(\log K(x))$ (approximate chain rule — cleaner than for $C$).
- $K(f(x)) \leq K(x) + K(f) + O(1)$ for computable $f$ — computation cannot increase complexity much.

The approximate chain rule $K(x, y) \approx K(x) + K(y \mid x)$ is the AIT analogue of the information-theoretic chain rule $H(X, Y) = H(X) + H(Y \mid X)$. The $O(\log K)$ overhead is a technical artifact that vanishes in asymptotic arguments.

The last property — that computable functions cannot increase $K$ by more than a constant — is the AIT analogue of the data processing inequality. It says: if you apply a computable algorithm to a simple string, you get a simple string. Computation cannot create structure that wasn't already there.

Prefix-free complexity $K$ is the "correct" version of Kolmogorov complexity for most information-theoretic purposes. When people in AIT refer to "Kolmogorov complexity" without qualification, they often mean $K$ rather than $C$.
