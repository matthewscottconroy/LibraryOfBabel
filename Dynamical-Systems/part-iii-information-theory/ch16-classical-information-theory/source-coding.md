# 16.3 Source Coding

## 16.3.1 Lossless Data Compression

Armed with the AEP, we can now prove the most fundamental theorem in data compression: entropy is the exact limit of lossless compression. No code can compress a source below its entropy, and codes exist that achieve it.

Let's set up the problem precisely. We have a source producing symbols from an alphabet $\mathcal{X}$, distributed as $p$. We want to assign binary codewords to symbols — an encoding scheme — so that we can later decode the original sequence uniquely. The question is: how long do the codewords have to be?

**Definition 16.3.1 (Code).** A *code* for $X$ is a function $C: \mathcal{X} \to \{0,1\}^*$ (mapping outcomes to binary strings). The *expected length* is $L(C) = \sum_x p(x) |C(x)|$ where $|C(x)|$ is the length of the codeword for $x$.

Not all codes are useful. We need codes where the encoder's output can be uniquely read back by the decoder, even without knowing where one codeword ends and the next begins. The cleanest class of such codes are the prefix-free codes:

**Definition 16.3.2 (Prefix-Free Code).** A code is *prefix-free* if no codeword is a prefix of another. Prefix-free codes are uniquely decodable: the decoder can unambiguously parse any concatenation of codewords.

A nice example: in Morse code, common letters get short codes. But Morse is not quite prefix-free (it requires pauses between letters). A binary prefix-free code is like a binary tree where codewords label leaves — no leaf is an ancestor of another.

Prefix-free codes are exactly characterized by the Kraft inequality:

**Theorem 16.3.3 (Kraft Inequality).** Codeword lengths $\ell_1, \ldots, \ell_m$ correspond to a prefix-free code if and only if $\sum_i 2^{-\ell_i} \leq 1$.

This is a beautiful result: the codeword lengths must "fit" into the unit interval, in the sense that the intervals $[k/2^{\ell_i}, (k+1)/2^{\ell_i})$ can be packed without overlap. The Kraft inequality is the gateway between combinatorics and analysis in this subject.

Now the main theorem: entropy is the exact compression limit.

**Theorem 16.3.4 (Shannon's Source Coding Theorem).** For a source $X$ with entropy $H(X)$, the optimal prefix-free code $C^*$ satisfies:
$$H(X) \leq L(C^*) < H(X) + 1.$$

For $n$-tuples: the optimal code for $(X_1, \ldots, X_n)$ achieves $L/n \to H(X)$ as $n \to \infty$.

*Achievability:* Set $\ell_x = \lceil -\log p(x) \rceil$ (the smallest integer $\geq -\log p(x)$). Then $\sum_x 2^{-\ell_x} \leq \sum_x p(x) = 1$ (Kraft inequality satisfied), so a prefix-free code with these lengths exists. The expected length is:
$$L \leq \sum_x p(x)(-\log p(x) + 1) = H(X) + 1.$$

*Converse:* For any prefix-free code with lengths $\ell_x$, the Kraft inequality gives $\sum_x 2^{-\ell_x} \leq 1$. By the AM-GM inequality (or the log-sum inequality):
$$L(C) = \sum_x p(x)\ell_x \geq \sum_x p(x)(-\log p(x)) = H(X).$$

The gap of 1 bit in the upper bound is just an artifact of rounding. By encoding long blocks of symbols together — coding $(X_1, \ldots, X_n)$ as a unit — we can drive the per-symbol overhead to zero, and the optimal rate converges to $H(X)$.

## 16.3.2 Huffman Coding

Shannon's theorem tells us that codes achieving $L \approx H$ exist, but it doesn't tell us how to find them efficiently. That problem was solved by David Huffman in 1952, as a term paper assignment.

The Huffman code achieves the optimal (minimum expected length) prefix-free code via a greedy algorithm:

1. Build a priority queue of (symbol, probability) pairs.
2. Repeatedly combine the two lowest-probability nodes into a new node with their combined probability.
3. Assign 0/1 labels to the two branches at each combination.
4. Read off codewords by tracing paths from root to leaves.

**Theorem 16.3.5.** Huffman coding achieves the optimal average code length for prefix-free codes.

The algorithm is elegant: by always combining the rarest symbols first, it ensures that the rarest symbols end up deepest in the tree — with the longest codewords. Common symbols float near the root and get short codewords. This is the binary analogue of what any efficient communication system does: use short representations for common things, longer ones for rare things.

Huffman coding still has the $+1$ gap: it can require up to one extra bit per symbol compared to the entropy. Arithmetic coding (a generalization) can approach the entropy limit to arbitrary precision, even for non-integer entropies. But for our theoretical purposes, the $+1$ overhead is irrelevant — what matters is that $H(X)$ is the exact asymptotic compression limit, and it is achievable.

In the next section, we turn from the source to the channel — and find that entropy plays an equally fundamental role on the communication side of the problem.
