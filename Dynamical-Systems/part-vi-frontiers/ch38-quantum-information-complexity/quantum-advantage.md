# 38.4 Quantum Advantage: Shor and Grover

The two most important quantum algorithms — Shor's factoring algorithm and Grover's search algorithm — represent the two main types of quantum speedup: exponential (Shor) and quadratic (Grover).

**Theorem 38.4.1 (Shor's Algorithm, 1994).** Factoring an $n$-bit integer takes $O(n^3)$ quantum gate operations on a quantum computer, versus the best classical algorithm (NFS) which takes $e^{O(n^{1/3})}$.

*Key step*: Period finding on ${\mathbb Z}/N{\mathbb Z}$ via the Quantum Fourier Transform (QFT). The QFT is $U_{QFT}|j\rangle = \frac{1}{\sqrt{N}}\sum_{k=0}^{N-1}e^{2\pi ijk/N}|k\rangle$ — a quantum circuit implementing the discrete Fourier transform in $O(n^2)$ gates.

Shor's algorithm is based on the number-theoretic fact that factoring $N$ reduces to finding the period of the function $f(x) = a^x \pmod{N}$. Classically, finding this period requires essentially trying all possibilities — exponential time. Quantum Fourier analysis finds the period in polynomial time. The QFT creates interference patterns that encode the period in the amplitudes of the output state.

**Theorem 38.4.2 (Grover's Algorithm, 1996).** Unstructured database search (find one item in $N$ satisfying $f(x) = 1$) takes $O(\sqrt{N})$ quantum queries versus $O(N)$ classical.

Grover's algorithm works by "amplitude amplification": starting from a uniform superposition over all $N$ items, the algorithm repeatedly applies an oracle (which marks the target item) and a "diffusion" operator. Each iteration rotates the amplitude distribution closer to the target. After $O(\sqrt{N})$ iterations, measurement gives the target with high probability.

**Theorem 38.4.3 (BBBV, 1994 — Quantum Search Lower Bound).** Any quantum algorithm for unstructured search requires $\Omega(\sqrt{N})$ queries. Grover's algorithm is optimal.

This is a proven lower bound — quantum search provably cannot do better than $O(\sqrt{N})$. The gap with classical search ($O(N)$) is quadratic, not exponential. For factoring, the gap is exponential. This difference reflects the structure of the problem: factoring has algebraic structure (group structure on $\mathbb{Z}/N\mathbb{Z}$) that quantum Fourier analysis can exploit; unstructured search has no structure at all.
