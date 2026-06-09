# 18.9 The Halting Probability Ω

We close this chapter with Chaitin's $\Omega$ — a number that is simultaneously the most information-dense real number ever constructed and provably, maximally random. It is a fitting conclusion to a chapter about the limits of what computation can know.

**Definition 18.9.1 (Chaitin's Halting Probability).** The *Chaitin halting probability* is:
$$\Omega = \sum_{p : U(p) \text{ halts}} 2^{-|p|}.$$

This is the probability that the universal Turing machine $U$ halts, if we choose a program by flipping fair coins. Each program $p$ of length $|p|$ contributes $2^{-|p|}$ to the sum. By the Kraft inequality, $\Omega \in (0, 1)$.

**Theorem 18.9.2.**
1. $\Omega \in (0, 1)$.
2. $\Omega$ is Martin-Löf random.
3. $\Omega$ is computably enumerable (r.e.): one can compute better and better lower bounds on $\Omega$.
4. The first $n$ bits of $\Omega$ allow one to decide, for all programs of length $\leq n$, whether they halt.
5. $\Omega$ is incomputable: no Turing machine computes $\Omega$ exactly.

Let's unpack these properties and why they are simultaneously true and remarkable.

Property (3) says that $\Omega$ is knowable from below: as more and more programs halt, you observe that they contribute their weight to $\Omega$, and your lower bound improves. But you can never know when you've seen all the halting programs of a given length — some might still be running.

Property (4) is the explosive one. If you knew the first $n$ bits of $\Omega$, you could determine, for every program of length $\leq n$, whether it halts. This is because you could compute an approximation to $\Omega$ from below and compare it to the known value: if the approximation equals the known value up to precision $2^{-n}$, all programs of length $\leq n$ have been accounted for.

Property (2) says $\Omega$ is ML-random — maximally random in the sense of the previous sections. Its bits pass every computable statistical test. Yet by property (4), those bits contain all the answers to all halting questions for programs up to length $n$.

**Remark 18.9.3.** $\Omega$ is the most information-dense number: its first $n$ bits encode the answer to all halting questions for programs of length $\leq n$. It is the "number that contains all mathematical truth" in a precise sense — and it is random.

This is the deepest paradox in algorithmic information theory. $\Omega$ contains more computable information than any computable sequence, yet it is itself completely incompressible — its bits pass every computable test for randomness. Incompressibility and information density, which might seem to be opposite properties, coexist in $\Omega$.

Chaitin used $\Omega$ to give information-theoretic proofs of Gödel's incompleteness theorem: for any formal system $F$ powerful enough to capture arithmetic, there is an $n$ such that $F$ cannot determine the $n$-th bit of $\Omega$. The bits of $\Omega$ are independent of any fixed formal system. Mathematics has no computable way to "reach" all the truths encoded in $\Omega$, even though $\Omega$ is in principle a definite real number.
