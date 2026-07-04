# Chapter 8: Number Theory

The integers as a proving ground — the oldest theorems in mathematics, from Euclid's infinitude of primes to unique factorization, exercised with every strategy from Chapter 5 and formalized in Lean 4.

## Overview

The chapter's central question: how do the primes — defined by the simplest possible property — encode the arithmetic structure of all integers? We start from **divisibility**: $a \mid b \iff \exists k \in \mathbb{Z},\, b = ak$, with its basic properties (reflexivity, transitivity, antisymmetry up to sign, and the crucial **linearity**: if $a \mid b$ and $a \mid c$ then $a \mid mb + nc$). The **division algorithm** — unique $q, r$ with $a = bq + r$ and $0 \leq r < b$ — is proved by a least-element argument and drives everything that follows. **Primes** are the multiplicative atoms: **Euclid's theorem** (there are infinitely many primes, via $N = p_1 \cdots p_k + 1$), the sieve of Eratosthenes, and the **Fundamental Theorem of Arithmetic**: every $n \geq 2$ factors uniquely into primes. Existence is a showcase of strong induction; uniqueness rests on **Euclid's lemma** ($p \mid ab \Rightarrow p \mid a$ or $p \mid b$); and the failure of unique factorization in $\mathbb{Z}[\sqrt{-5}]$ (where $6 = 2 \cdot 3 = (1+\sqrt{-5})(1-\sqrt{-5})$) shows the theorem is a special property of $\mathbb{Z}$, motivating Kummer's and Dedekind's ideals.

**Modular arithmetic**, Gauss's language from the *Disquisitiones*: $a \equiv b \pmod{n} \iff n \mid (a - b)$ is an equivalence relation whose classes form the commutative ring $\mathbb{Z}/n\mathbb{Z}$ — a field $\mathbb{F}_p$ exactly when $n$ is prime. Highlights: computing $7^{100} \bmod 10$ by cycle-finding, **Fermat's little theorem** ($a^{p-1} \equiv 1 \pmod{p}$ when $p \nmid a$, proved by the permutation-of-residues argument), Euler's totient $\varphi(n)$, and the **Chinese Remainder Theorem** for systems of congruences with pairwise coprime moduli.

The gcd section develops $\gcd$ and $\mathrm{lcm}$ (with $\gcd(a,b) \cdot \mathrm{lcm}(a,b) = |ab|$), the **Euclidean algorithm** via $\gcd(a, b) = \gcd(b, a \bmod b)$ in $O(\log \min(a,b))$ steps, and **Bézout's identity**: $\gcd(a,b) = sa + tb$, the smallest positive linear combination. The extended Euclidean algorithm computes the coefficients, yielding modular inverses, solvability of linear Diophantine equations ($ax + by = c$ iff $\gcd(a,b) \mid c$), and the proof of Euclid's lemma itself.

## Why It Matters

Number theory is where the proof strategies of Chapter 5 and the induction of Chapter 7 earn their keep: direct proof (divisibility lemmas), contradiction (Euclid), cases (residues mod 3), and strong induction (FTA existence) all appear on natural material. Modular arithmetic underpins RSA, Diffie-Hellman, and elliptic-curve cryptography. Everything here is formalized: Mathlib's `Nat.gcd`, `Int.gcd_eq_gcd_ab` (Bézout), `ZMod n`, and Fermat's little theorem as `ZMod.pow_card_sub_one_eq_one`, with a dedicated Lean file closing the chapter. The gcd/UFD theory generalizes to principal ideal domains in Chapter 19.

## Chapter Roadmap

1. [Divisibility and Primes](01_divisibility_and_primes/01_divisibility.md) — the divisibility relation, linearity, and the division algorithm; further files prove Euclid's infinitude of primes (with the sieve and crypto context) and the Fundamental Theorem of Arithmetic, including where unique factorization fails.
2. [Modular Arithmetic](02_modular_arithmetic/01_congruences.md) — congruences as an equivalence relation, the ring $\mathbb{Z}/n\mathbb{Z}$ and field $\mathbb{F}_p$, Fermat's little theorem, and the Chinese Remainder Theorem, with a companion Python implementation.
3. [GCD and the Euclidean Algorithm](03_gcd_and_euclidean/01_gcd_and_lcm.md) — gcd, lcm, and the Euclidean algorithm; the second file proves Bézout's identity and applies the extended Euclidean algorithm to modular inverses and Diophantine equations, with a Python companion.
4. [Number Theory in Lean](04_number_theory_in_lean/01_number_theory_lean.lean) — the chapter's theorems formalized in Lean 4 with Mathlib.

## Prerequisites

Chapter 5 (direct proof, contradiction, case analysis) and Chapter 7 (weak and strong induction, well-founded termination of the Euclidean algorithm). Chapter 6's equivalence relations explain why congruence classes partition $\mathbb{Z}$.
