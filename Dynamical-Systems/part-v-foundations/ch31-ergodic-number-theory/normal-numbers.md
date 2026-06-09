# 31.3 Normal Numbers

Take a number like $\pi = 3.14159265358979...$. Look at its decimal digits: $1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, \ldots$. Do these digits behave like a "random" sequence? In particular, does each digit $0, 1, \ldots, 9$ appear with frequency $1/10$? Does each two-digit block $00, 01, \ldots, 99$ appear with frequency $1/100$? Does every finite string of digits appear with the frequency you'd expect from a uniformly random sequence?

If so, $\pi$ is called *normal in base 10*. If it's normal in every base, it's absolutely normal. This is expected — in fact, almost every real number is absolutely normal in the measure-theoretic sense. But we don't know for a single "natural" constant whether it's normal in any base.

**Definition 31.3.1.** A real number $x \in [0,1)$ is *normal in base $b$* if every finite string $w \in \{0,\ldots,b-1\}^k$ appears in the base-$b$ expansion of $x$ with frequency $b^{-k}$.

**Definition 31.3.2.** $x$ is *absolutely normal* if it is normal in every integer base $b \geq 2$.

**Theorem 31.3.3 (Borel's Normal Number Theorem, 1909).** Lebesgue-almost every $x \in [0,1)$ is absolutely normal.

*Ergodic Proof:* For base $b$: normality of $x$ is equivalent to the orbit of $x$ under the map $T_b(x) = bx \pmod 1$ (the $b$-fold expanding map) being equidistributed for Lebesgue measure. By Birkhoff's theorem and the fact that Lebesgue is the unique absolutely continuous invariant measure for $T_b$, a.e. $x$ is generic for Lebesgue, hence normal.

Let's unpack the ergodic proof. The base-$b$ expansion of $x$ is just the symbolic orbit of $x$ under the map $T_b(x) = bx \pmod 1$: the $n$-th digit is determined by which interval $[k/b, (k+1)/b)$ the $n$-th iterate falls in. Normality says each finite string appears with frequency equal to the length of the corresponding cylinder set — precisely Birkhoff equidistribution for the cylinder set indicators.

Since Lebesgue measure is the unique absolutely continuous $T_b$-invariant measure, and $T_b$ is ergodic for Lebesgue, Birkhoff's theorem says a.e. orbit is equidistributed. "A.e." here means Lebesgue-almost every. The set of non-normal numbers has Lebesgue measure zero.

**Open Problem 31.3.4.** It is not known whether $\pi$, $e$, or $\sqrt{2}$ are normal in any base. These are probably absolutely normal, but no proof exists. The Champernowne number $0.123456789101112\ldots$ (concatenating all positive integers) is normal in base 10 but not absolutely normal.

The gap between "almost every number is normal" and "we can't prove any specific interesting number is normal" is one of the deepest frustrations in analytic number theory. The measure-theoretic argument is soft — it uses the Borel-Cantelli lemma in disguise — and gives no information about individual numbers. Proving normality of specific numbers requires arithmetic structure that we don't know how to extract from $\pi$, $e$, or $\sqrt{2}$.

The Champernowne number is the exception: it's normal in base 10 by construction (you concatenate all the integers, so every finite string eventually appears), and Champernowne verified this carefully. But it's not absolutely normal — it has a special structure in base 10 that doesn't extend to other bases.

Normality is connected to algorithmic randomness (Chapter 18): a number is normal in base $b$ if and only if its base-$b$ expansion passes all "frequency" tests. ML-randomness is strictly stronger — it requires passing all effective statistical tests, not just frequency tests. An ML-random number is absolutely normal (this follows from Birkhoff's theorem applied to computable systems), but not conversely.
