# Chapter 8 Overview: Number Theory

---

## Central Question

What properties do the integers have that follow from their basic structure as a commutative ring with the divisibility relation? And how do the prime numbers — defined by the simplest possible property — encode the arithmetic structure of all integers?

Number theory was for millennia the paradigm of pure mathematics: questions about integers that seemed to have no practical application. Today number theory underpins all of cryptography, and the proofs developed here are among the earliest and most beautiful illustrations of mathematical proof technique.

---

## Why This Chapter Matters

Number theory provides the testing ground for every proof strategy from Chapter 5: direct proofs, contradictions, case analysis, and induction all appear naturally. Modular arithmetic is the foundation of modern cryptography (RSA, elliptic curve cryptography, digital signatures). The deeper results — Dirichlet's theorem, quadratic reciprocity, the prime number theorem — illustrate how simple questions about integers lead to profound mathematics.

---

## Key Definitions

**Divisibility.** $a \mid b$ ("$a$ divides $b$") iff there exists an integer $k$ such that $b = ak$.

**Greatest common divisor.** $\gcd(a, b)$ is the largest positive integer dividing both $a$ and $b$. Equivalently, $\gcd(a,b)$ is the smallest positive integer in the set $\{ax + by : x, y \in \mathbb{Z}\}$.

**Coprime.** $a$ and $b$ are coprime if $\gcd(a, b) = 1$.

**Prime.** A natural number $p > 1$ is prime if its only positive divisors are 1 and $p$.

**Composite.** A natural number $n > 1$ is composite if it is not prime.

**Modular arithmetic.** $a \equiv b \pmod{m}$ ("$a$ is congruent to $b$ modulo $m$") iff $m \mid (a - b)$. This is an equivalence relation on $\mathbb{Z}$; the equivalence classes form the ring $\mathbb{Z}/m\mathbb{Z}$.

**Euler's totient function.** $\varphi(n)$ is the number of integers in $\{1, \ldots, n\}$ coprime to $n$. For prime $p$: $\varphi(p) = p - 1$. For $\gcd(m, n) = 1$: $\varphi(mn) = \varphi(m)\varphi(n)$.

**Diophantine equation.** A polynomial equation with integer coefficients for which integer solutions are sought.

---

## Main Theorems

### Euclid's Theorem: Infinitely Many Primes

**Theorem (Euclid, circa 300 BCE).** There are infinitely many prime numbers.

**Proof.** Suppose $p_1, p_2, \ldots, p_k$ is any finite list of primes. Let $N = p_1 p_2 \cdots p_k + 1$. Then $N$ is not divisible by any $p_i$ (since $N \equiv 1 \pmod{p_i}$ for each $i$). But $N > 1$, so $N$ has at least one prime factor $p$. This $p$ is a prime not in our list. $\square$

**Note:** This proof does not claim $N$ itself is prime — only that $N$ has a prime factor not in our list.

### Fundamental Theorem of Arithmetic

**Theorem.** Every integer $n > 1$ can be written uniquely as a product of primes (up to reordering).

**Proof (existence).** By strong induction on $n$. If $n$ is prime, it is already a prime product. If $n$ is composite, write $n = ab$ with $1 < a, b < n$. By the inductive hypothesis, $a$ and $b$ each have prime factorisations; their concatenation gives one for $n$. $\square$

**Proof (uniqueness).** Uses Euclid's lemma: if $p$ is prime and $p \mid ab$, then $p \mid a$ or $p \mid b$. This follows from Bézout's identity (below). By induction on the number of prime factors, any two prime factorisations of $n$ must match. $\square$

### Bézout's Identity and the Euclidean Algorithm

**Theorem (Bézout).** For any integers $a, b$, there exist integers $x, y$ such that $ax + by = \gcd(a, b)$.

**Proof.** Consider the set $S = \{ax + by : x, y \in \mathbb{Z}, ax + by > 0\}$. This is a non-empty set of positive integers; let $d$ be its minimum. By the division algorithm, any element of $S$ is a multiple of $d$, so $d \mid \gcd(a,b)$. Also $d \mid a$ and $d \mid b$ (by the minimality of $d$ and the division algorithm). So $d = \gcd(a,b)$. $\square$

**Euclidean algorithm.** Computes $\gcd(a, b)$ in $O(\log \min(a,b))$ steps:
$$\gcd(a, b) = \gcd(b, a \bmod b) \quad (b \neq 0), \qquad \gcd(a, 0) = a$$

**Extended Euclidean algorithm.** Also computes the Bézout coefficients $x, y$.

### Fermat's Little Theorem

**Theorem (Fermat 1640, Euler 1736).** If $p$ is prime and $\gcd(a, p) = 1$, then $a^{p-1} \equiv 1 \pmod{p}$.

**Proof.** Consider the $p - 1$ non-zero residues $a, 2a, 3a, \ldots, (p-1)a$ modulo $p$. These are all distinct (if $ia \equiv ja \pmod{p}$, then $p \mid (i-j)a$; since $\gcd(a,p)=1$, we get $p \mid (i-j)$, so $i \equiv j \pmod{p}$, contradiction if $1 \leq i, j \leq p-1$). So they are a permutation of $\{1, 2, \ldots, p-1\}$. Multiply all: $(a)(2a)\cdots((p-1)a) \equiv (p-1)! \pmod{p}$, giving $a^{p-1} \cdot (p-1)! \equiv (p-1)! \pmod{p}$. Cancel $(p-1)!$ (coprime to $p$): $a^{p-1} \equiv 1 \pmod{p}$. $\square$

**Corollary.** $a^p \equiv a \pmod{p}$ for all $a$ (including those divisible by $p$).

**Application.** RSA cryptography: key generation uses Euler's generalisation $a^{\varphi(n)} \equiv 1 \pmod{n}$ for $\gcd(a,n)=1$.

### Chinese Remainder Theorem

**Theorem.** If $m_1, m_2, \ldots, m_k$ are pairwise coprime, then for any $a_1, \ldots, a_k$, the system of congruences $x \equiv a_i \pmod{m_i}$ has a unique solution modulo $M = m_1 m_2 \cdots m_k$.

**Proof.** Existence: by Bézout, find $M_i = M/m_i$ and $y_i$ with $M_i y_i \equiv 1 \pmod{m_i}$. Then $x = \sum_i a_i M_i y_i$ works. Uniqueness: if $x$ and $x'$ both satisfy the system, then $m_i \mid (x - x')$ for all $i$. Since the $m_i$ are pairwise coprime, $M \mid (x - x')$. $\square$

**Application.** Efficient multi-precision arithmetic, fast algorithms for large integer computation, error-correcting codes.

---

## Diophantine Equations

A *Diophantine equation* is a polynomial equation over $\mathbb{Z}$ for which integer solutions are sought.

**Linear Diophantine equations.** $ax + by = c$ has integer solutions iff $\gcd(a, b) \mid c$. The solutions form an arithmetic progression (they are all of the form $(x_0 + \frac{b}{\gcd}t,\ y_0 - \frac{a}{\gcd}t)$ for $t \in \mathbb{Z}$).

**Pythagorean triples.** Integer solutions to $x^2 + y^2 = z^2$ are parametrised: primitive solutions (gcd = 1) are exactly $(m^2 - n^2, 2mn, m^2 + n^2)$ for $m > n > 0$, $\gcd(m,n) = 1$, $m \not\equiv n \pmod{2}$.

**Fermat's Last Theorem.** $x^n + y^n = z^n$ has no solution in positive integers for $n > 2$. Conjectured by Fermat in 1637; proved by Wiles (1995) using elliptic curves and modular forms.

---

## Historical Context

**Euclid (circa 300 BCE)** proved infinitely many primes and gave the Euclidean algorithm in the *Elements*. His proofs are among the earliest examples of rigorous mathematical argument.

**Diophantus of Alexandria (circa 250 CE)** studied integer solutions of polynomial equations in his *Arithmetica*. Fermat read this book and wrote his famous "Last Theorem" in the margin.

**Pierre de Fermat (1607–1665)** stated (without proof) many results including the Last Theorem and his Little Theorem.

**Leonhard Euler (1707–1783)** proved Fermat's Little Theorem, Fermat's theorem on sums of two squares, and countless other results. He introduced the totient function.

**Carl Friedrich Gauss (1777–1855)** published *Disquisitiones Arithmeticae* (1801), the masterwork of classical number theory, introducing modular arithmetic, proving quadratic reciprocity, and laying the groundwork for algebraic number theory.

**Andrew Wiles (1953–)** proved Fermat's Last Theorem in 1994 (published 1995), using tools from algebraic geometry (elliptic curves) and complex analysis (modular forms) — a testament to the depth hidden in simple Diophantine questions.

---

## Connections to Other Chapters

- **Chapter 7** (Induction and Recursion): mathematical induction is the primary proof technique throughout this chapter.
- **Chapter 19** (Abstract Algebra): the integers form a principal ideal domain; the theory of gcd and unique factorisation generalises to all PIDs.
- **Chapter 13** (Formal Verification): Lean 4 and Coq's `Mathlib` contain formal proofs of the Fundamental Theorem of Arithmetic, Euler's theorem, and the Chinese Remainder Theorem.
