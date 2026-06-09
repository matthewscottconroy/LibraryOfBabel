# 41.4 Tao's Progress (2022)

Terence Tao's 2022 paper is the most significant advance on the Collatz conjecture in decades. He proved that almost all Collatz orbits attain almost bounded values — a result that goes well beyond what was previously known.

**Theorem 41.4.1 (Tao, 2022).** Almost all Collatz orbits attain almost bounded values. More precisely, for any function $f: {\mathbb N} \to {\mathbb R}$ with $f(n) \to \infty$ (however slowly):
$$\#\{n \leq N : \min_k C^k(n) \leq f(n)\} = (1 + o(1))N.$$

In particular: for almost all $n$ (in the density sense), the orbit of $n$ eventually reaches a value $\leq f(n)$ for any unbounded function $f$.

To appreciate what this says: take $f(n) = \log\log\log n$ — a function that grows extremely slowly. Tao's theorem says that for a density-1 set of $n$, the Collatz orbit reaches a value below $\log\log\log n$. The orbits don't just reach "smaller" values — they reach almost-bounded values.

**Proof Strategy:**
1. *Syracuse function*: Work with the map $S: 2{\mathbb Z}+1 \to 2{\mathbb Z}+1$ (odd numbers only), $S(n) = (3n+1)/2^{v_2(3n+1)}$
2. *$p$-adic analysis*: Track the 2-adic valuation of iterates
3. *Fourier analysis on ${\mathbb Z}_2$*: Use the circle of 2-adic characters to bound exponential sums
4. *Sieve theory*: Control the density of orbits not reaching small values
5. *Probabilistic argument*: Convert the Fourier bounds to density statements

The key new tool is Fourier analysis on $\mathbb{Z}_2$ — exponential sums involving 2-adic characters. These are the analogues of classical exponential sums in analytic number theory, but over the 2-adic integers rather than $\mathbb{Z}/N\mathbb{Z}$.

**The Gap from Full Conjecture:** Tao's theorem says orbits reach "small values" (any $f(n) \to \infty$) but not necessarily "1." The full conjecture requires reaching exactly 1. The final step — showing orbits do not "escape to infinity" — remains open.

The gap is real and significant. "Almost bounded" means the minimum value of the orbit grows more slowly than $f(n)$, for any unbounded $f$. But the orbit might still reach $f(n)$ rather than 1, and the density-1 set might miss infinitely many counterexamples. Tao's result leaves open the possibility that there exist counterexamples — just a density-zero set of them.

Nobody knows how to close the gap. The techniques that prove "almost bounded" don't obviously extend to "reaches 1."
