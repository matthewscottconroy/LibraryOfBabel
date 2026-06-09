# 18.8 Randomness and Dynamical Systems

## 18.8.1 Fouché's Theorem

The connection between algorithmic randomness and dynamical systems is one of the most beautiful results in this area. For the doubling map — the simplest chaotic dynamical system — the ergodic-theoretic notion of "typical point" and the algorithmic notion of "ML-random sequence" coincide exactly.

Recall: the doubling map is $T: [0,1] \to [0,1]$, $T(x) = 2x \pmod 1$. Every point $x \in [0,1]$ has a binary expansion $x = 0.b_1 b_2 b_3 \ldots$, and the orbit of $x$ under $T$ corresponds exactly to the bit sequence $(b_1, b_2, b_3, \ldots)$ — each application of $T$ shifts the binary expansion left by one digit.

So there is a natural bijection between points $x \in [0,1]$ and binary sequences. Lebesgue measure on $[0,1]$ corresponds to the fair coin measure on binary sequences. A point $x$ is "typical" in the ergodic sense (its orbit equidistributes) iff the sequence $(b_1, b_2, \ldots)$ passes all reasonable frequency tests.

Fouché's theorem makes this precise:

**Theorem 18.8.1 (Fouché's Theorem).** Under the coding of orbits of the doubling map $T(x) = 2x \pmod 1$ by binary sequences (itinerary), an initial condition $x \in [0,1]$ is ML-random with respect to Lebesgue measure if and only if its orbit coding is ML-random as a binary sequence.

*This establishes the equivalence between ergodic-theoretic randomness (typicality of the orbit) and algorithmic randomness (Martin-Löf randomness of the sequence).*

The ergodic-theoretic notion of typicality says: $x$ is typical if $\frac{1}{n}\sum_{k=0}^{n-1} f(T^k(x)) \to \int f\,d\mu$ for all continuous $f$ (Birkhoff's theorem, Lebesgue-a.e.). Algorithmic randomness says: $x$ is random if $K(x_{|n}) \geq n - O(1)$ for all $n$ (the bit sequence is incompressible). These are superficially very different definitions, yet they describe exactly the same set of points.

The implication is remarkable. It means that "algorithmically random" and "chaotically typical" are the same thing, at least for the doubling map. An orbit that looks "random" to a computer is exactly an orbit that looks "typical" to a probabilist. Algorithmic complexity theory and ergodic theory are measuring the same property from different angles.

## 18.8.2 Computable Analysis and Dynamics

Not all dynamical questions are so clean. What if you want to know whether a specific orbit is computable — whether you can approximate it to arbitrary precision using an algorithm?

**Definition 18.8.2.** An orbit $\{f^n(x)\}_{n \geq 0}$ of a computable dynamical system is *computably describable* if $x$ is computable (there is a Turing machine that outputs better and better rational approximations to $x$).

For chaotic systems, computability of the initial condition becomes critical. A computable initial condition has a computable orbit, but the orbit may diverge exponentially from any nearby initial condition — this is sensitive dependence on initial conditions. If $x$ is computable, its orbit is computable, but any computational approximation using a nearby rational number will eventually diverge.

For Julia sets and the Mandelbrot set, the computability question becomes subtle:

**Theorem 18.8.3 (Braverman-Yampolsky).** The filled Julia set $\mathcal{K}(f_c)$ is computable (as a subset of $\mathbb{C}$) iff $c$ is computable and $c$ is not on the boundary of the Mandelbrot set (or $c$ is in a specific "computable" part of the boundary).

**Theorem 18.8.4 (Rettinger-Weihrauch).** There exist $c \in \partial\mathcal{M}$ for which $\mathcal{J}(f_c)$ is not computable (as a closed subset of $\mathbb{C}$). Julia sets can be algorithmically random.

These theorems show that the complexity of a dynamical system's attractor can be as high as possible — even algorithmically uncomputable. The Mandelbrot set boundary is in some sense the hardest part of complex dynamics to understand, not just visually but computationally.
