# Chapter 18 — Algorithmic Information Theory

> *Shannon entropy is the entropy of a distribution. Kolmogorov complexity is the entropy of an individual object — defined without any probability. This is the right framework for asking: how complex is this string? This orbit? This theorem?*

**Prerequisites:** Chapter 16 (Shannon entropy), basic computability theory (Turing machines, halting problem).

---

Shannon's entropy is a property of a probability distribution, not of any individual object. If someone hands you a single binary string and asks "how much information does this contain?", Shannon's theory can only answer the question if you also specify a distribution over all possible strings. But what if there is no natural distribution? What if you want to say that the string $0101010101\ldots$ (repeating forever) is simpler than a random-looking string with no pattern, *intrinsically*, without reference to any ensemble?

Kolmogorov complexity — developed independently by Kolmogorov, Solomonoff, and Chaitin in the 1960s — answers this question. The complexity of a string is the length of the shortest program that produces it. Patterns compress; random-looking strings do not. The string $0101\ldots$ has a short program ("print '01' alternating, $n$ times"); a string with no pattern has no short description.

This shift from distributions to individual objects is profound. It allows us to ask: is this particular orbit of a dynamical system "random"? Is this specific theorem "complex"? Is this number "computable"? These are questions about individual objects, and algorithmic information theory is the right framework for them.

The chapter has two main threads. The first develops Kolmogorov complexity as a theory of individual complexity: we define it, prove it is well-defined (up to an additive constant), and derive its basic properties. The second thread is the theory of *algorithmic randomness*: what does it mean for an individual infinite sequence to be random? Martin-Löf's 1966 answer — rigorous, computable, and complete — turns out to connect beautifully to the dynamical systems perspective.

**What this chapter builds:**
- Turing machines and computability: the mathematical framework
- Kolmogorov complexity and its universality (choice of UTM doesn't matter)
- Prefix-free complexity $K$ and its cleaner properties
- Algorithmic probability: Solomonoff's universal prior
- AIT analogues of entropy, conditional entropy, mutual information
- Martin-Löf randomness: the gold standard definition of an individual random sequence
- Other randomness notions and their implications
- Fouché's theorem: ML-random orbits of the doubling map are exactly the ML-random sequences
- Chaitin's $\Omega$: the most information-dense number, which is random

**Sections:**
- [18.1 Computability](computability.md)
- [18.2 Kolmogorov Complexity](kolmogorov-complexity.md)
- [18.3 Prefix-Free Complexity](prefix-free-complexity.md)
- [18.4 Algorithmic Probability](algorithmic-probability.md)
- [18.5 AIT Analogues of Information Theory](ait-analogues-of-information-theory.md)
- [18.6 Martin-Löf Randomness](martin-lof-randomness.md)
- [18.7 Other Randomness Notions](other-randomness-notions.md)
- [18.8 Randomness and Dynamical Systems](randomness-and-dynamical-systems.md)
- [18.9 The Halting Probability Ω](halting-probability.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
