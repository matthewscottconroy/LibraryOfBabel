# 31.5 van der Waerden, Hales-Jewett, and Recurrence

The Szemerédi theorem (Section 31.2) required positive density. Van der Waerden's theorem requires only finite coloring — and yet produces arithmetic progressions. The Hales-Jewett theorem is the combinatorial colossus that contains both. All of them, from the ergodic perspective, are statements about multiple recurrence.

**Theorem 31.5.1 (van der Waerden's Theorem, 1927).** For any $k \geq 1$ and any finite coloring of $\mathbb{Z}$, there is a monochromatic arithmetic progression of length $k$.

No density assumption here — just finite coloring. At least one color class is dense enough (by the pigeonhole principle) to apply something like Szemerédi's theorem. But the argument is more elementary: van der Waerden proved this in 1927, before Szemerédi and before Furstenberg. The standard proof uses double induction and is finite but rapidly growing in complexity.

**Theorem 31.5.2 (Hales-Jewett Theorem, 1963).** For any $k \geq 1$ and finite alphabet, there is a dimension $N = N(k)$ such that any $k$-coloring of the combinatorial cube $\{1,\ldots,k\}^N$ contains a monochromatic combinatorial line.

A *combinatorial line* is a set of points obtained by fixing some coordinates and letting the others vary simultaneously (all move from 1 to $k$ together). The Hales-Jewett theorem is the most powerful of the Ramsey-type results — it implies van der Waerden's theorem as a special case (arithmetic progressions in $\mathbb{Z}$ correspond to combinatorial lines in $\{1, \ldots, k\}^N$).

The original Hales-Jewett proof is elementary but complicated. The ergodic proof, due to Furstenberg and Katznelson (1991), reveals the right abstract structure.

**Ergodic Reformulation (Furstenberg-Katznelson, 1991).** The Hales-Jewett theorem follows from a multiple recurrence theorem: for any MPT $(X, \mu, T_1, \ldots, T_k)$ with commuting $T_i$ and any $B$ with $\mu(B) > 0$:
$$\liminf_{N\to\infty} \frac{1}{N}\sum_{n=1}^N \mu(B \cap T_1^n B \cap \cdots \cap T_k^n B) > 0.$$

The multiple recurrence theorem says that any set $B$ of positive measure "remembers" itself under the simultaneous action of commuting transformations: the intersection $B \cap T_1^n B \cap \cdots \cap T_k^n B$ has positive measure for many $n$. This is the dynamical content of the Hales-Jewett theorem.

The commuting transformations $T_1, \ldots, T_k$ here correspond to the $k$ directions in which the combinatorial cube can be "moved." The set $B$ corresponds to the color class, and the intersection condition says the color appears along a combinatorial line.

Furstenberg and Katznelson's proof of this multiple recurrence theorem (1991) is one of the hardest results in ergodic theory, requiring the full structure theory of measure-preserving systems (Furstenberg-Zimmer tower, compact and distal extensions, weakly mixing extensions). The payoff is a proof of the Hales-Jewett theorem that, while longer than the combinatorial proof, is more transparent about why it's true.

What connects van der Waerden, Hales-Jewett, and Szemerédi? They are all instances of the Ramsey philosophy: any finite partition of a sufficiently structured set contains a "structured" piece. The ergodic-theoretic unification of these results — viewing them all as consequences of multiple recurrence in measure-preserving systems — is Furstenberg's great contribution. It turned a collection of isolated results into a coherent theory, and that theory had the Green-Tao theorem waiting at its boundary.
