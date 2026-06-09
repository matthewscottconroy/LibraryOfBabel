# 25.2 Pseudo-Randomness from Chaotic Maps

Chaotic maps are sometimes used as random number generators — the idea being that a chaotic orbit "looks random." The idea is partially right and partially wrong, and the information-theoretic picture clarifies exactly where the truth lies.

The doubling map is the clearest example. Define $T: [0,1] \to [0,1]$ by $T(x) = 2x \pmod 1$. Starting from $x_0$, the orbit generates the sequence of bits $b_n = \lfloor 2T^n(x_0) \rfloor \pmod 2$ — the $n$-th binary digit of $x_0$. If $x_0 = 0.b_0 b_1 b_2 \ldots$ in binary, then $T^n(x_0) = 0.b_n b_{n+1} b_{n+2} \ldots$ — the orbit just shifts the binary expansion left by one.

So the orbit of $T$ reads out the binary digits of $x_0$, in order. The question of whether the orbit is random is exactly the question of whether the binary expansion of $x_0$ is random.

**The Doubling Map as Generator:** The doubling map $T(x) = 2x \pmod 1$ with $x_0 \in [0,1]$ generates the sequence of bits $b_n = \lfloor 2^n x_0 \rfloor \pmod 2$ — the binary expansion of $x_0$. For Lebesgue-a.e. $x_0$, this is a Martin-Löf random sequence (Section 18.8).

**BUT:** If $x_0$ is computable (rational), the sequence is periodic and NOT random. The "randomness" comes from the initial condition, not from the map.

This is the key point. The doubling map doesn't *generate* randomness — it *reveals* randomness that was already in the initial condition. If you start with a generic (Lebesgue-random) initial condition, the orbit is random. If you start with a computable (rational or algebraic) initial condition, the orbit is periodic and as non-random as possible.

A chaotic map is not a source of randomness by itself. It is a transducer: it takes randomness in the initial condition and spreads it across the orbit. The information content of the orbit equals the information content of the initial condition (to the extent that the orbit is invertible), plus whatever information is added by the map's dynamics. For a conservative system (one that preserves a measure), the information is neither created nor destroyed — it is just rearranged.

**Theorem 25.2.1 (Pseudo-Randomness via Chaos).** For the logistic map $f_4(x) = 4x(1-x)$: the orbit $\{f_4^n(x)\}$ with $x$ chosen uniformly has the distribution of i.i.d. arcsine-distributed random variables. The sequence of bits $b_n = \lfloor f_4^n(x) \cdot 2 \rfloor$ is i.i.d. Bernoulli($1/2$) for Lebesgue-a.e. $x$.

The logistic map $f_4$ is conjugate to the doubling map via the semiconjugacy $h(x) = \sin^2(\pi x/2)$: we have $f_4 \circ h = h \circ T$. So the orbit of $f_4$ starting from $x$ corresponds to the orbit of $T$ starting from $h^{-1}(x) = (2/\pi)\arcsin(\sqrt{x})$. The arcsine distribution of the logistic orbit is the pushforward of Lebesgue measure under $h$.

The practical lesson: if you use a chaotic map as a random number generator, you must start from a genuinely random seed. The map does not add entropy — it processes entropy. For cryptographic applications, this means the seed must be as random as the desired output. Chaotic maps can be useful for stretching a small amount of good randomness into a larger amount of apparently random bits (pseudo-random number generation), but this is a statement about computational hardness, not about information-theoretic randomness.
