# Chapter 17: Information Theory and Logic

The quantitative side of knowledge. How many bits does it take to specify a message, a string, a proof? The answer turns out to reach all the way to Gödel: there is a fixed amount of information a formal system contains, and it cannot prove any object more complex than itself.

## Overview

Two measures of information anchor the chapter. **Shannon entropy** $H(X) = -\sum_i p_i \log_2 p_i$ measures the uncertainty of a *distribution*; the logician's way to motivate it is the Khinchin uniqueness theorem, which shows the formula is not chosen but *forced* by a short list of axioms — the "logic" of information. Entropy comes with a calculus (chain rule, mutual information $I(X;Y)$, the maximum-entropy bound via Jensen) whose operational meaning is Shannon's source coding theorem: entropy is the optimal compression rate.

**Kolmogorov complexity** $K(x)$ measures the information in a *single string* — the length of its shortest generating program on a universal machine — with no probability model at all. The **invariance theorem** makes it well-defined up to an additive constant; a counting argument shows *most strings are incompressible*; and $K$ is provably **uncomputable**. From incompressibility comes a genuine proof technique, the **incompressibility method**: to prove that most objects have a property, show that any object lacking it would be compressible. We work examples — a one-line proof that there are infinitely many primes, and a Turing-machine lower bound. Pushing further, **Martin-Löf randomness** gives a rigorous definition of a random infinite sequence, Schnorr's theorem ties it back to incompressibility, and Chaitin's halting probability **Ω** is a concrete random, uncomputable real. The climax is **Chaitin's incompleteness theorem**: for each formal system $T$ there is a constant $L$ — essentially the information content of $T$'s axioms — such that $T$ can prove "$K(x) > L$" for no $x$, though it holds for almost all $x$. This is Gödel's theorem weighed in bits. A final measure, Bennett's **logical depth**, distinguishes raw randomness from *organized* complexity — the crystallized computation of a theorem with a short statement and a long proof.

## Why It Matters

Information theory supplies logic with a *quantitative* incompleteness theorem and a *constructive* proof method. Chaitin's result recasts Gödel (Chapter 10) as a conservation law for information, and formalizes Berry's paradox the way Gödel formalized the Liar. The incompressibility method proves theorems — in number theory, combinatorics, and computational complexity — that are hard to reach otherwise, by turning "a random object cannot be described briefly" into a counting principle. And logical depth gives a precise sense in which a proof *is* compressed computation, linking proof length, speed-up phenomena, and the value (not just the quantity) of information.

## Chapter Roadmap

1. [Shannon Entropy and Information](01_entropy/01_entropy.md) — entropy, its axiomatic characterization, the chain rule, mutual information, and the maximum-entropy bound, with worked computations.
2. [Kolmogorov Complexity](02_kolmogorov/01_descriptive_complexity.md) — plain and prefix complexity, the invariance theorem, uncomputability, incompressibility counting, and the link to entropy.
3. [The Incompressibility Method](02_kolmogorov/02_incompressibility_method.md) — the method schematized, with worked proofs: infinitely many primes, and a one-tape Turing-machine lower bound.
4. [Algorithmic Randomness and Chaitin's Theorem](03_randomness/01_martin_lof_and_chaitin.md) — Martin-Löf tests, Schnorr's theorem, the number Ω, and the information-theoretic incompleteness theorem.
5. [Logical Depth and Meaningful Complexity](04_depth/01_logical_depth.md) — Bennett's depth, the slow-growth law, deep objects, and proofs as crystallized computation.

## Prerequisites

- [Chapter 10: Computability and Incompleteness](../ch10_computability_and_incompleteness/) — universal Turing machines, the halting problem, and Gödel's theorems, on which the uncomputability and incompleteness results rest.
- Basic discrete probability (finite random variables, expectation) for the entropy section.
- Helpful: [Chapter 7: Induction and Recursion](../ch07_induction_and_recursion/) — recursion and enumeration underlie the complexity arguments.
