# Chapter 31 — Ergodic Methods in Number Theory

> *The primes are equidistributed in arithmetic progressions (Dirichlet). The digits of $\pi$ are normal (expected, not proved). The Collatz conjecture is about ergodic properties of a number-theoretic map. Dynamical systems methods — equidistribution, recurrence, combinatorial number theory — have transformed additive combinatorics.*

**Prerequisites:** Chapter 7 (ergodic theory, Birkhoff's theorem), Chapter 6 (topological dynamics, equicontinuity), Chapter 9 (symbolic dynamics).

---

## 31.1 Equidistribution and Weyl's Theorem

**Definition 31.1.1.** A sequence $(x_n)$ in $[0,1)$ is *equidistributed* (or *uniformly distributed mod 1*) if for every subinterval $[a,b) \subseteq [0,1)$:
$$\lim_{N\to\infty} \frac{1}{N}\#\{n \leq N : x_n \in [a,b)\} = b - a.$$

**Theorem 31.1.2 (Weyl's Equidistribution Theorem, 1916).** The sequence $(n\alpha \pmod 1)$ is equidistributed for any irrational $\alpha$.

*(proof)* By the Weyl criterion: $(x_n)$ is equidistributed iff for all $k \neq 0$: $\frac{1}{N}\sum_{n=1}^N e^{2\pi ikx_n} \to 0$. For $x_n = n\alpha$: $\frac{1}{N}\sum_{n=1}^N e^{2\pi ikn\alpha} = \frac{1}{N}\cdot\frac{e^{2\pi ik(N+1)\alpha}-e^{2\pi ik\alpha}}{e^{2\pi ik\alpha}-1} \to 0$ since $e^{2\pi ik\alpha} \neq 1$ for irrational $\alpha$.

**Ergodic Proof:** Equidistribution of $(n\alpha)$ is exactly Birkhoff's theorem for the rotation $R_\alpha: x \mapsto x + \alpha \pmod 1$ — the time average of the indicator $\mathbf{1}_{[a,b)}$ converges to the space average $b-a$ for Lebesgue-a.e. starting point. For $R_\alpha$ with irrational $\alpha$, the system is uniquely ergodic (Lebesgue is the unique invariant measure), so the convergence holds for *all* starting points.

**Theorem 31.1.3 (Weyl Polynomial Equidistribution).** For any polynomial $p(n) = \alpha_d n^d + \cdots + \alpha_0$ with at least one irrational non-constant coefficient, the sequence $(p(n) \pmod 1)$ is equidistributed.

---

## 31.2 Furstenberg's Correspondence Principle

### 31.2.1 From Combinatorics to Ergodic Theory

**Theorem 31.2.1 (Furstenberg Correspondence Principle, 1977).** Let $A \subseteq {\mathbb Z}$ with positive upper density $\bar{d}(A) = \limsup_{N\to\infty}\frac{|A \cap [1,N]|}{N} > 0$. Then there exists a measure-preserving system $(X, \mathcal{B}, \mu, T)$ and a set $B \in \mathcal{B}$ with $\mu(B) = \bar{d}(A)$ such that for all $n_1, \ldots, n_k \in {\mathbb Z}$:
$$\bar{d}(A \cap (A - n_1) \cap \cdots \cap (A - n_k)) \geq \mu(B \cap T^{-n_1}B \cap \cdots \cap T^{-n_k}B).$$

**Proof Sketch:** The system is the *Furstenberg compactification*: take the closure of $\{T^n \mathbf{1}_A : n \in {\mathbb Z}\}$ in $\{0,1\}^{\mathbb Z}$ under the product topology, with the shift $T$ and the natural invariant measure.

### 31.2.2 Szemerédi's Theorem via Ergodic Theory

**Theorem 31.2.2 (Szemerédi's Theorem, 1975; Ergodic proof: Furstenberg, 1977).** Any subset $A \subseteq {\mathbb Z}$ with $\bar{d}(A) > 0$ contains arithmetic progressions of arbitrary length.

**Ergodic Proof (Furstenberg, 1977).** By the correspondence principle, it suffices to show: for any MPT $(X, \mu, T)$ and $B$ with $\mu(B) > 0$:
$$\liminf_{N\to\infty}\frac{1}{N}\sum_{n=1}^N \mu(B \cap T^{-n}B \cap T^{-2n}B \cap \cdots \cap T^{-kn}B) > 0.$$

This is the *Furstenberg multiple recurrence theorem*, proved by extending Birkhoff to multiple commuting transformations using compact extensions and weakly mixing systems.

**Theorem 31.2.3 (Green-Tao Theorem, 2004).** The primes contain arithmetic progressions of arbitrary length. The proof uses a "relative" version of Szemerédi's theorem, combined with sieve theory.

---

## 31.3 Normal Numbers

**Definition 31.3.1.** A real number $x \in [0,1)$ is *normal in base $b$* if every finite string $w \in \{0,\ldots,b-1\}^k$ appears in the base-$b$ expansion of $x$ with frequency $b^{-k}$.

**Definition 31.3.2.** $x$ is *absolutely normal* if it is normal in every integer base $b \geq 2$.

**Theorem 31.3.3 (Borel's Normal Number Theorem, 1909).** Lebesgue-almost every $x \in [0,1)$ is absolutely normal.

*Ergodic Proof:* For base $b$: normality of $x$ is equivalent to the orbit of $x$ under the map $T_b(x) = bx \pmod 1$ (the $b$-fold expanding map) being equidistributed for Lebesgue measure. By Birkhoff's theorem and the fact that Lebesgue is the unique absolutely continuous invariant measure for $T_b$, a.e. $x$ is generic for Lebesgue, hence normal.

**Open Problem 31.3.4.** It is not known whether $\pi$, $e$, or $\sqrt{2}$ are normal in any base. These are probably absolutely normal, but no proof exists. The Champernowne number $0.123456789101112\ldots$ (concatenating all positive integers) is normal in base 10 but not absolutely normal.

---

## 31.4 The Collatz Map as Ergodic System

**Definition 31.4.1.** The *Collatz map* (or $3x+1$ map) is:
$$C(n) = \begin{cases}n/2 & \text{if } n \equiv 0 \pmod 2 \\ 3n+1 & \text{if } n \equiv 1 \pmod 2\end{cases}$$

**Definition 31.4.2.** The *stochastic model* of the Collatz map treats consecutive bits of $n$ as i.i.d. Bernoulli($1/2$) random variables. The map $C$ reduces $n$ by a factor of $\sim 3/4$ on average (each step: halve half the time, triple-and-one the rest).

**Theorem 31.4.3 (Terras-Lagarias).** The Collatz map, viewed as a map on ${\mathbb Z}_2 = $ 2-adic integers, extends to a well-defined map $C: {\mathbb Z}_2 \to {\mathbb Z}_2$. On ${\mathbb Z}_2$ with Haar measure, the map $C$ is measure-preserving and ergodic.

**Theorem 31.4.4 (Lagarias, 1985).** Let $\rho_k(n)$ = number of steps to reach 1 starting from $n$ under $C$. Then for "almost all" $n$ (in density):
$$\lim_{N\to\infty} \frac{1}{N}\#\{n \leq N : \rho_k(n) \leq c\log n\} \to 1$$
for some explicit constant $c$. The expected trajectory length to 1 is $O(\log n)$.

**Ergodic Interpretation:** The Collatz conjecture says the orbit of every positive integer under $C$ eventually reaches 1 (the fixed point of $C$). If the ${\mathbb Z}_2$ system is ergodic, then "almost all" integers have orbits that behave like random walks — supporting the conjecture heuristically but not proving it.

---

## 31.5 van der Waerden, Hales-Jewett, and Recurrence

**Theorem 31.5.1 (van der Waerden's Theorem, 1927).** For any $k \geq 1$ and any finite coloring of ${\mathbb Z}$, there is a monochromatic arithmetic progression of length $k$.

**Theorem 31.5.2 (Hales-Jewett Theorem, 1963).** For any $k \geq 1$ and finite alphabet, there is a dimension $N = N(k)$ such that any $k$-coloring of the combinatorial cube $\{1,\ldots,k\}^N$ contains a monochromatic combinatorial line.

**Ergodic Reformulation (Furstenberg-Katznelson, 1991).** The Hales-Jewett theorem follows from a multiple recurrence theorem: for any MPT $(X, \mu, T_1, \ldots, T_k)$ with commuting $T_i$ and any $B$ with $\mu(B) > 0$:
$$\liminf_{N\to\infty} \frac{1}{N}\sum_{n=1}^N \mu(B \cap T_1^n B \cap \cdots \cap T_k^n B) > 0.$$

---

## Exercises

**Exercise 31.1.** (Weyl) Prove that the sequence $(n^2\alpha \pmod 1)$ is equidistributed for irrational $\alpha$ using the van der Corput trick: apply Weyl's criterion to show $\frac{1}{N}\sum e^{2\pi ikn^2\alpha} \to 0$.

**Exercise 31.2.** (Normal Numbers) Show that if $x$ is normal in base 2, then $2x \pmod 1$ is also normal in base 2. Conclude that the set of normal numbers is invariant under the doubling map.

**Exercise 31.3.** (Furstenberg Correspondence) Apply the Furstenberg correspondence principle to the set of odd numbers $A = \{1, 3, 5, 7, \ldots\}$ (density $1/2$). What is the corresponding MPT and set $B$? Does $A$ contain arithmetic progressions of length 3?

**Exercise 31.4.** (Collatz) Compute the first 20 iterates of $C$ starting from $n = 27$. How many steps to reach 1? Compare to the heuristic prediction $O(\log n)$.

---

## Chapter Notes

Furstenberg's ergodic proof of Szemerédi's theorem is in *Ergodic Behavior of Diagonal Measures and a Theorem of Szemerédi on Arithmetic Progressions* (J. d'Analyse Math., 1977). His book *Recurrence in Ergodic Theory and Combinatorial Number Theory* (1981) is the definitive reference.

The Green-Tao theorem is in *The primes contain arbitrarily long arithmetic progressions* (Annals of Math., 2008). The ergodic-theoretic component uses Gowers uniformity norms, developed in Gowers's *A new proof of Szemerédi's theorem for arithmetic progressions of length four* (1998).

Lagarias's survey on Collatz: *The $3x+1$ Problem and its Generalizations* (American Mathematical Monthly, 1985) and the more comprehensive *The Ultimate Challenge: The $3x+1$ Problem* (AMS, 2010). Tao's recent progress (2022) on the Collatz conjecture is in *Almost all orbits of the Collatz map attain almost bounded values* (Forum of Mathematics, Pi).
