# 23.3 Universal Source Coding for Stationary Ergodic Sources

The SMB theorem says: every stationary ergodic source has an entropy rate $h$, and almost every long sequence has probability approximately $2^{-nh}$. A consequence is that the sequence can be compressed to approximately $h$ bits per symbol.

But here is the practical problem: to use Shannon's optimal source code, you need to know $h$ — and to know $h$, you need to know the distribution of the source. For real data sources, you often don't know the distribution. You just have the data.

Lempel and Ziv solved this problem in 1977–78. Their algorithm — known as LZ77 and LZ78, and variants thereof as gzip, zlib, and deflate — achieves the optimal compression rate without knowing $h$ in advance. The algorithm is *universal*.

**Theorem 23.3.1 (Lempel-Ziv is Universal).** The Lempel-Ziv algorithm (LZ78 or LZ77) achieves the optimal compression rate $h$ for any stationary ergodic source, without knowing $h$ in advance.

*(Ziv-Lempel 1978, Wyner-Ziv 1994)*

The LZ algorithm works by parsing the sequence into *phrases*: read the input left to right, and whenever you encounter a string you've seen before, extend it by one symbol to get a new phrase not yet seen. Each phrase is encoded as a pointer to the previous occurrence plus the new symbol.

**Proof idea:** LZ parsing divides the sequence into phrases (longest phrases not seen before). By the SMB theorem, the number of phrases in a typical sequence of length $n$ is $\approx n h / \log n$. Each phrase is encoded with $\log$ of the number of phrases (dictionary pointer), giving compression rate $\to h$.

Let's trace through the argument. If the source has entropy rate $h$, a typical sequence of length $n$ has about $2^{nhe}$ distinct substrings of each length $\ell$ (for $\ell$ up to $n$). The LZ parsing terminates when it has seen all distinct substrings — the number of phrases $c_n$ grows like $n h / \log n$ by the SMB theorem. Each phrase requires $\log c_n \approx \log(nh/\log n)$ bits to encode. The total bit count is:

$$c_n \cdot \log c_n \approx \frac{nh}{\log n} \cdot \log\frac{nh}{\log n} \approx nh.$$

So the compression rate converges to $h$ bits per symbol.

**Theorem 23.3.2 (Optimality of LZ).** No compression algorithm can achieve a rate below $h$ for a stationary ergodic source with entropy rate $h$ (by the SMB theorem — the source cannot be compressed below $h$ bits/symbol).

The lower bound comes directly from the SMB theorem: any lossless compression algorithm must distinguish between the approximately $2^{nh}$ typical sequences of length $n$, which requires at least $nh$ bits.

What makes this result so powerful in practice is that LZ doesn't need to know the source — it *learns* the source adaptively as it reads. The dictionary it builds during encoding is implicitly discovering the statistical structure of the data. By the time it has read $n$ symbols of a stationary ergodic source, the dictionary is a near-optimal code for that source.

This is a beautiful convergence of ergodic theory and computer science. The SMB theorem — a deep result about the orbit structure of ergodic measure-preserving transformations — implies the universality of an algorithm designed by engineers for practical data compression. The theoretical foundation (stationary ergodic sources) covers the practical cases (natural language, DNA, sensor data) exactly because stationarity and ergodicity are the minimal assumptions for long-run stability of statistics.

A note on the time constants: the convergence of LZ to $h$ is logarithmically slow — you need sequence length $n$ to get within $O(1/\log n)$ of the optimal rate. For practical compression of small files, this slow convergence matters. For theoretical purposes and for large files, LZ is asymptotically optimal, and the SMB theorem is why.
