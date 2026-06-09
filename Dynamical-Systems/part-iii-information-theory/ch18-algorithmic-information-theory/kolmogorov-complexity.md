# 18.2 Kolmogorov Complexity

Fix a universal Turing machine $U$ — one that can simulate any other Turing machine, given a description of it as part of the input. Now we can define the complexity of any string.

**Definition 18.2.1 (Kolmogorov Complexity / Plain Complexity).** Fix a universal Turing machine $U$. The *Kolmogorov complexity* (or *plain complexity*) of a binary string $x \in \{0,1\}^*$ is:
$$C(x) = \min\{|p| : U(p) = x\},$$
the length of the shortest program $p$ such that $U$ on input $p$ outputs $x$.

The interpretation: $C(x)$ is the minimum number of bits needed to describe $x$ — the length of the shortest "explanation." A string with a lot of structure (like $000\ldots0$, or the first million digits of $\pi$) has a short description. A truly random-looking string has no short description: you essentially have to list the string itself.

The potential objection: doesn't $C(x)$ depend on the choice of $U$? Yes, but only up to an additive constant:

**Theorem 18.2.2 (Invariance Theorem).** For any two universal Turing machines $U$ and $U'$:
$$|C_U(x) - C_{U'}(x)| \leq c_{U, U'}$$
for some constant depending only on $U$ and $U'$, not on $x$.

*Proof:* Since $U$ is universal, there is a fixed-length program that tells $U$ how to simulate $U'$. Prepending this simulation prefix to any $U'$-program gives a $U$-program of the same length plus $c$. So $C_U(x) \leq C_{U'}(x) + c$. By symmetry, the bound holds in both directions. $\square$

The invariance theorem says: the choice of universal Turing machine does not matter for the theory. Different UTMs give complexities that differ by at most a constant, and all the theorems of AIT are about asymptotic or leading-order behavior where additive constants are irrelevant.

**Theorem 18.2.3 (Complexity of Most Strings).** For any $k > 0$:
$$|\{x \in \{0,1\}^n : C(x) < n - k\}| < 2^{n-k}.$$

Almost all strings of length $n$ have complexity $\geq n - O(1)$ — they are *incompressible*.

*Proof:* There are at most $\sum_{j < n-k} 2^j = 2^{n-k} - 1$ programs of length $< n-k$, so at most $2^{n-k} - 1$ strings can have $C(x) < n-k$. $\square$

This counting argument is one of the most useful tools in the subject. It says that "almost all" strings are incompressible — they have no short description. The strings that *do* compress are the exception: they have some pattern, some structure, that the compressor can exploit. Genuinely random strings have no such structure.

This is the foundation of the *incompressibility method* in combinatorics: to prove that some property holds for almost all strings (or almost all objects in some combinatorial class), you assume you have an incompressible string, derive properties that it must have, and observe that these properties are then shared by almost all strings.
