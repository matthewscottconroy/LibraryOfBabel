# Exercises — Chapter 18

These exercises develop the technical machinery of Kolmogorov complexity and algorithmic randomness. The later exercises push into open territory.

**Exercise 18.1.** Show that $C(x) \leq |x| + O(1)$ for all strings $x$ (the identity program). Show that $C(xx) \leq C(x) + O(\log C(x))$ (two copies can be described once). Can $C(xx) = C(x) - 1$ for some $x$?

**Exercise 18.2.** (Incompressibility) Show that at least $2^n - 1$ strings of length $n$ have $C(x) \geq n$. Exhibit a string of length 1000 with $C(x) \leq 100$ bits.

**Exercise 18.3.** Prove that $K(x) + K(y) = K(x,y) + O(\log K(x))$ (approximate chain rule for prefix-free complexity). (*Hint:* Given descriptions of $x$ and $y$, we can describe $(x,y)$; given $(x,y)$, we need a way to separate the two descriptions.)

**Exercise 18.4.** (AIT and Primes) The $n$-th prime $p_n$ satisfies $K(p_n) \leq \log n + O(1)$ (given $n$, we can compute $p_n$). But most integers $x < n$ have $K(x) \approx \log n$. So the primes are "simple" (low complexity) compared to random integers of the same size. Make this precise.

**Exercise 18.5.** Show that $\Omega$ is ML-random. (*Hint:* Use the Levin-Schnorr theorem: show that $K(\Omega_{|n}) \geq n - O(1)$. If the first $n$ bits of $\Omega$ could be compressed, we could solve more halting problems than $n$ bits should allow.)

**Exercise 18.6.** (Collatz Complexity) The Collatz sequence starting at $n$ has length $\ell(n)$ before reaching 1 (stopping time). What is $K(\ell(n))$ in terms of $K(n)$? What would it mean for the Collatz conjecture if $K(\ell(n)) = K(n) + O(1)$ for all $n$?

**Exercise 18.7.** State Van Lambalgen's theorem precisely. Use it to prove: if $\omega$ is ML-random and $\rho$ is a computable sequence, then $\omega \oplus \rho$ (interleaving) is ML-random.
