# 31.4 The Collatz Map as Ergodic System

The Collatz conjecture is one of the most famous unsolved problems in mathematics, and one of the most deceptive. The rule is simple enough to explain to a child: start with any positive integer. If it's even, divide by two. If it's odd, multiply by three and add one. Repeat. The conjecture says you always eventually reach 1. For example: $27 \to 82 \to 41 \to 124 \to 62 \to 31 \to 94 \to 47 \to 142 \to 71 \to 214 \to 107 \to \ldots$ (and so on, for 111 steps before reaching 1).

Every computer science student has seen this. What's less well-known is that the Collatz map has a natural ergodic-theoretic interpretation that explains, heuristically, why the conjecture is probably true — while also explaining why it's so hard to prove.

**Definition 31.4.1.** The *Collatz map* (or $3x+1$ map) is:
$$C(n) = \begin{cases}n/2 & \text{if } n \equiv 0 \pmod 2 \\ 3n+1 & \text{if } n \equiv 1 \pmod 2\end{cases}$$

**Definition 31.4.2.** The *stochastic model* of the Collatz map treats consecutive bits of $n$ as i.i.d. Bernoulli($1/2$) random variables. The map $C$ reduces $n$ by a factor of $\sim 3/4$ on average (each step: halve half the time, triple-and-one the rest).

The stochastic model is simple: if you're equally likely to be even or odd (which the bits of a large random number essentially are), then each step of the Collatz map multiplies by $1/2$ with probability $1/2$ and by $3$ with probability $1/2$. The expected logarithmic change is $\frac{1}{2}\log(1/2) + \frac{1}{2}\log 3 = \frac{1}{2}\log(3/2) - \frac{1}{2}\log 2 = \frac{1}{2}\log(3/4) < 0$.

The expected value decreases. So heuristically, the orbit should eventually reach 1. But "heuristically" is not a proof.

The rigorous version of this heuristic uses 2-adic integers:

**Theorem 31.4.3 (Terras-Lagarias).** The Collatz map, viewed as a map on $\mathbb{Z}_2 =$ 2-adic integers, extends to a well-defined map $C: \mathbb{Z}_2 \to \mathbb{Z}_2$. On $\mathbb{Z}_2$ with Haar measure, the map $C$ is measure-preserving and ergodic.

The 2-adic integers $\mathbb{Z}_2$ are the completion of $\mathbb{Z}$ with respect to the 2-adic metric (two integers are "close" if their difference is divisible by a high power of 2). The Collatz map extends naturally to $\mathbb{Z}_2$, and with respect to Haar measure (the natural invariant measure), the map is ergodic.

Ergodicity on $\mathbb{Z}_2$ means: for Haar-almost-every 2-adic integer, the orbit of $C$ is equidistributed in $\mathbb{Z}_2$. This is the 2-adic version of the stochastic model — the orbit behaves like a random walk.

**Theorem 31.4.4 (Lagarias, 1985).** Let $\rho_k(n)$ = number of steps to reach 1 starting from $n$ under $C$. Then for "almost all" $n$ (in density):
$$\lim_{N\to\infty} \frac{1}{N}\#\{n \leq N : \rho_k(n) \leq c\log n\} \to 1$$
for some explicit constant $c$. The expected trajectory length to 1 is $O(\log n)$.

**Ergodic Interpretation:** The Collatz conjecture says the orbit of every positive integer under $C$ eventually reaches 1 (the fixed point of $C$). If the $\mathbb{Z}_2$ system is ergodic, then "almost all" integers have orbits that behave like random walks — supporting the conjecture heuristically but not proving it.

The gap between "almost all" and "all" is the heart of the difficulty. Tao's 2022 paper (cited in the notes) proves that *almost all* orbits of the Collatz map attain *almost bounded* values — they eventually come close to 1, in a precise density sense. This is the strongest unconditional result to date. But "almost all" orbits doing something doesn't prove every orbit does it; the exceptional set might contain one orbit, or infinitely many, that never reach 1.

The Collatz problem sits at the frontier of what ergodic methods can do: they give you the right heuristics, the right probabilistic picture, and increasingly strong "almost everywhere" results. But converting "almost everywhere" to "everywhere" — a question about individual orbits — typically requires different techniques.
