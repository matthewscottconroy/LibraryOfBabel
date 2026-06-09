# 25.3 Algorithmic Randomness of Chaotic Orbits

Chapter 18 introduced Martin-Löf randomness: a sequence is ML-random if it passes all effective statistical tests — equivalently, if it has minimal Kolmogorov complexity per bit. For the doubling map, we saw informally that Lebesgue-a.e. initial condition gives an ML-random orbit. The precise statement is:

**Theorem 25.3.1 (Fouché, strengthened).** For the doubling map $T$ and Lebesgue measure:
- A point $x \in [0,1]$ has a Martin-Löf random binary expansion iff the symbolic orbit $(\lfloor 2^n x \rfloor \pmod 2)$ is ML-random as a sequence.
- ML-random points form a set of measure 1.
- Computable initial conditions give periodic or eventually periodic orbits — never random.

The equivalence in the first bullet is immediate from the construction: the symbolic orbit of $T$ *is* the binary expansion of $x$. The non-trivial content is that ML-randomness of a real number (a property about approximability by algorithms) is equivalent to ML-randomness of the sequence (a property about effective statistical tests). These are the same because ML-randomness of a real number is defined by the incompressibility of its binary expansion.

The third bullet is the key practical point. A computable initial condition $x = p/q$ gives a periodic binary expansion (if $q$ is a power of 2) or an eventually periodic one (if $q$ has only factors of 2 and 5 in base 10, but other denominators give periodic binary expansions too). More generally, any rational $x = p/q$ gives a purely periodic binary expansion of period equal to the multiplicative order of 2 modulo (odd part of $q$). This is entirely predictable — it's as far from random as possible.

**Theorem 25.3.2 (Effective Birkhoff Theorem).** For a computable ergodic system and a computable integrable function $\varphi$:
- If $x$ is ML-random, the time average $\frac{1}{n}\sum_{k<n} \varphi(f^k(x))$ converges to $\int \varphi\,d\mu$ at the ergodic rate.
- The convergence is *effectively computable*: the modulus of convergence is computable from $x$.

The effective Birkhoff theorem characterizes ML-random points as exactly those for which the ergodic theorem holds computably. For non-ML-random points, the ergodic averages still converge (by the classical Birkhoff theorem), but the modulus of convergence may be non-computable.

This is a beautiful confluence of ergodic theory and algorithmic randomness. Birkhoff's theorem says: for almost every $x$, the time averages converge to the space average. The effective Birkhoff theorem says: for exactly the ML-random $x$, the convergence is computable (you can tell, algorithmically, when you're within $\varepsilon$ of the limit). The set of ML-random points has measure 1 (agreeing with "almost every $x$"), and they're exactly the points where computability and measure theory agree.

Here is the philosophical upshot. Classical ergodic theory tells you that typical orbits behave like random orbits, in the sense that time averages agree with space averages. Algorithmic randomness tells you that "typical" means "ML-random." The effective Birkhoff theorem ties these together: an orbit is ML-random if and only if all its ergodic averages are computable from the initial point. Typical and computable are the same thing, for ergodic systems.

This also explains why numerical simulations of ergodic systems work: a computer computes a rational approximation to the initial condition, iterates a computable approximation to the map, and produces a trajectory that (though computable and hence technically non-random) behaves like an ML-random trajectory for the observable computations it performs. The effective Birkhoff theorem is the reason we can trust our simulations — the computable trajectories track the ML-random ones.
