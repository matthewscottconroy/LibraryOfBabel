# Shannon Entropy and Information

Claude Shannon's 1948 paper "A Mathematical Theory of Communication" founded information theory. The central concept — entropy — measures uncertainty, information content, and the compressibility of data.

## The Definition

Given a discrete random variable X taking values {x₁, ..., xₙ} with probabilities {p₁, ..., pₙ}, the *Shannon entropy* is:

```
H(X) = -∑ᵢ pᵢ log₂ pᵢ
```

(with the convention 0 log₀ = 0)

Units: bits (if log₂), nats (if ln), hartleys (if log₁₀).

## Intuition

H(X) measures the *expected surprise* of observing X — the average number of yes/no questions needed to determine X's value.

- **Maximum entropy**: H is maximized when all outcomes are equally likely (uniform distribution). For n outcomes: H = log₂ n bits.
- **Minimum entropy**: H = 0 when one outcome has probability 1 — no uncertainty, no information.
- **Coin flip**: H = 1 bit (one question determines the outcome).
- **Fair die**: H = log₂ 6 ≈ 2.58 bits.

## Source Coding Theorem

Shannon's theorem: the average code length per symbol for an optimal lossless compression of source X is at least H(X) bits, and can be achieved arbitrarily closely by encoding long blocks.

This gives entropy a *operational* meaning: H(X) is the minimum number of bits needed to represent one sample from X.

## Logical Connections

Information theory and logic intersect in deep ways:

**Mutual information**: I(X; Y) = H(X) - H(X|Y) measures how much knowing Y reduces uncertainty about X. In logic: how much knowing one proposition reduces uncertainty about another.

**Compression and proofs**: A theorem with a short proof "compresses" the assertion — the proof is a compact certificate. Long proofs for short theorems suggest deep mathematical content.

**Entropy and randomness**: Maximum entropy = maximum randomness. Kolmogorov complexity (next section) gives a formal theory of randomness for individual strings, not distributions.

**Channel capacity**: The maximum rate at which information can be reliably transmitted over a noisy channel — the *Shannon capacity*. Logic enters: reliable transmission requires error-correcting codes, which are combinatorial structures with algebraic (logical) structure.

```python
import math

def entropy(probs):
    return -sum(p * math.log2(p) for p in probs if p > 0)

# Fair coin
print(entropy([0.5, 0.5]))    # 1.0 bit

# Biased coin
print(entropy([0.9, 0.1]))    # 0.469 bits

# Fair die
print(entropy([1/6]*6))       # log2(6) ≈ 2.585 bits

# Deterministic
print(entropy([1.0]))         # 0.0 bits
```

## Entropy in Logic and Combinatorics

*Information-theoretic proofs* use entropy as a tool to prove combinatorial results without explicit constructions.

**Example**: Lower bound for comparison-based sorting. Any sorting algorithm must distinguish n! permutations. Each comparison provides at most 1 bit. So we need at least log₂(n!) ≈ n log₂ n comparisons — matching the asymptotic complexity of mergesort and heapsort.

**Example**: Hamming bound (sphere-packing bound). A binary code with minimum distance d can correct ⌊(d-1)/2⌋ errors. The sphere of radius t around each codeword contains ∑ᵢ₌₀ᵗ C(n,i) words. These spheres are disjoint; the total cannot exceed 2ⁿ. This gives the Hamming bound — an information-theoretic constraint on error-correcting codes.
