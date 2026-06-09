# 22.4 Measures of Maximal Entropy

The variational principle guarantees that $h_{\text{top}}(f)$ is a supremum of KS entropies. The natural next question is: is the supremum achieved? And if so, by how many measures?

For nice systems — irreducible subshifts of finite type, Axiom A attractors — the answer is yes, and the maximizing measure is unique. For more complicated systems, the picture can be messier. But in all cases, the maximizing measures (when they exist) carry deep structural information.

**Definition 22.4.1.** A *measure of maximal entropy (MME)* is an invariant measure $\mu$ achieving $h_\mu(f) = h_{\text{top}}(f)$.

The information-theoretic translation is immediate: an MME is a capacity-achieving input distribution. In the channel analogy, the topological entropy is the channel capacity (the maximum rate of information transmission), and the MME is the input distribution that achieves this maximum.

**Theorem 22.4.2 (Existence and Uniqueness for SFTs).** Every irreducible subshift of finite type has a unique MME — the *Parry measure* (see Section 12.9).

The Parry measure is beautiful. If the SFT has transition matrix $A$ with Perron-Frobenius eigenvalue $\lambda$ and corresponding left eigenvector $u = (u_i)$ and right eigenvector $v = (v_i)$ (with $uv = 1$), then the Parry measure assigns to each cylinder $[a_0 a_1 \cdots a_{n-1}]$ the probability:

$$\mu([a_0 a_1 \cdots a_{n-1}]) = u_{a_0} \cdot \frac{A_{a_0 a_1}}{\lambda} \cdot \frac{A_{a_1 a_2}}{\lambda} \cdots \frac{A_{a_{n-2} a_{n-1}}}{\lambda} \cdot v_{a_{n-1}}.$$

This is a Markov measure with transition probabilities $P_{ij} = A_{ij} v_j / (\lambda v_i)$. Its entropy is $\log \lambda = h_{\text{top}}(X_A)$. The Parry measure is the natural "equilibrium" distribution on the SFT — it assigns probability proportional to how many future orbits each state can generate.

**Theorem 22.4.3 (MMEs for Axiom A Systems — Bowen, Ruelle).** Every Axiom A attractor has a unique MME. It is ergodic and is supported on the closure of periodic orbits.

Support on the closure of periodic orbits might seem like a technical condition, but it's actually remarkable: the measure of maximal entropy "lives" on the densest, most complex part of the system. Periodic orbits are countable; the closure of all periodic orbits can be a fractal attractor.

**Non-uniqueness:** For general continuous maps, multiple MMEs can coexist. For the quadratic family $f_\mu$ at special parameter values, the MME can be supported on a strange attractor.

This non-uniqueness is a genuine phenomenon, not a pathology. When multiple MMEs exist, they represent different "phases" of the system — different ways of maximally distributing orbit complexity. The analogy with statistical mechanics is not accidental; see Section 22.5.

Here is a useful way to think about MMEs. Every invariant measure $\mu$ can be thought of as a "way of exploring the system": $\mu$ tells you how often the orbit visits each region. The KS entropy $h_\mu(f)$ is the complexity of the exploration pattern. The MME is the exploration strategy that maximizes complexity — it visits different parts of the system as uniformly as possible, consistent with the dynamics.

For the doubling map $T: x \mapsto 2x \pmod 1$, the Lebesgue measure is the unique MME. Every point is equally likely; the dynamics produce maximum entropy. This is the only way to use the doubling map to generate maximally random bits.
