# 18.5 AIT Analogues of Information Theory

One of the pleasures of algorithmic information theory is that it provides a "dictionary" between the classical information-theoretic concepts (entropy, mutual information, conditional entropy) and their algorithmic analogues (complexity, algorithmic mutual information, conditional complexity). The dictionary is not exact — there are logarithmic correction terms — but it is faithful to the spirit.

**Complexity and Entropy:**

For a random variable $X$ with finite support, the expected Kolmogorov complexity approximates the Shannon entropy:
$$H(X) \approx E[K(X)] \quad \text{(up to } O(\log n)\text{ terms)},$$
where $n = \log |\mathcal{X}|$. Shannon entropy is the *expected* complexity; Kolmogorov complexity is the *individual* complexity. The two coincide in expectation, up to logarithmic corrections from the difference between $C$ and $K$.

**Mutual Complexity:**

The algorithmic analogue of mutual information between two strings $x$ and $y$ is:
$$I(x : y) = K(x) + K(y) - K(x, y) + O(\log K(x,y)).$$

This is symmetric (up to logarithmic terms) and nonnegative. It measures the "shared information" between $x$ and $y$ — how much knowing one reduces the complexity of describing the other.

The logarithmic correction $O(\log K)$ is a genuine limitation: the algorithmic mutual information is not exactly symmetric, unlike its Shannon counterpart. The exact statement of symmetry is:
$$K(x) + K(y \mid x, K(x)) = K(x, y) + O(1),$$
which gives $I(x : y) = I(y : x) + O(\log K)$.

**Complexity-Based Data Processing:**

For any computable function $f$ and strings $x, y$:
$$K(f(x) \mid y) \leq K(x \mid y) + K(f) + O(1).$$

Computable functions do not increase conditional complexity (beyond the cost of describing $f$ itself). This is the AIT analogue of the data processing inequality: processing cannot increase the algorithmic information content.

These analogues are more than formal. They suggest deep connections between computability and information: the reason Shannon entropy and Kolmogorov complexity are so closely related is that both measure, in different but equivalent senses, the "irreducible information content" of a source or a string. The difference is that Shannon entropy averages over a distribution while Kolmogorov complexity applies to individual objects. The logarithmic gaps between them reflect the cost of specifying which individual you are looking at.
