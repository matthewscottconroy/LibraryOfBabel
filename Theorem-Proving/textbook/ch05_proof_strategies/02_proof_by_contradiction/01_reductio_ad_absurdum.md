# Proof by Contradiction: The Power of Impossibility

> *"Reductio ad absurdum, which Euclid loved so much, is one of a mathematician's finest weapons. It is a far finer gambit than any chess gambit: a chess player may offer the sacrifice of a pawn or even a piece, but a mathematician offers the game."*
> — G.H. Hardy, *A Mathematician's Apology*

---

Suppose you want to prove that a certain monster does not exist. You could search for it everywhere and fail to find it — but that just shows you have not looked in the right place. Alternatively, you could reason from the monster's alleged properties to an impossible conclusion, showing that the monster's existence is *logically incompatible* with known facts.

This is the strategy of proof by contradiction: assume the negation of what you want to prove, derive something that is demonstrably impossible, and conclude that your assumption was false. It is perhaps the most psychologically surprising of all proof techniques, and Hardy was right to compare it to a gambit: you "sacrifice" your preferred conclusion, assume the opposite is true, and use the opponent's position (the assumed opposite) to destroy itself.

## The Logical Structure

Formally, proof by contradiction (*reductio ad absurdum*, RAA) works like this:

To prove P:
1. Assume ¬P
2. Derive ⊥ (a logical contradiction — something of the form Q ∧ ¬Q, or a violation of a known theorem)
3. Conclude P

The justification: if ¬P leads to an absurdity, then ¬P cannot be true (in classical logic, where every proposition is either true or false and ⊥ is never true). So P must be true.

In natural deduction:

```
  [¬P]
    ⋮
    ⊥
──────── RAA
   P
```

Note the crucial distinction from ¬-introduction: ¬I proves ¬P from an assumed P; RAA proves P from an assumed ¬P. RAA is the *classical* rule; ¬I is *constructively valid*.

## The Irrationality of √2

The most famous proof by contradiction in mathematics is the ancient Greek proof that √2 is irrational. It is a perfect specimen of the technique — short, elegant, and seemingly impossible to avoid.

**Theorem**: √2 is irrational (has no representation as a ratio of integers).

**Proof**: Suppose, for contradiction, that √2 is rational. Then we can write √2 = p/q where p and q are positive integers and gcd(p, q) = 1 (the fraction is in its lowest terms — all common factors have been cancelled).

Squaring: 2 = p²/q², so p² = 2q².

This means p² is even. But if p were odd, p² would be odd (since odd × odd = odd). Therefore p must be even. Write p = 2k for some integer k.

Substituting: (2k)² = 2q², so 4k² = 2q², so q² = 2k².

Now q² is even, so q is even by the same argument.

But we said gcd(p, q) = 1 — they share no common factors. Yet we have just shown both p and q are even, so they share the factor 2. **Contradiction**.

Therefore our assumption was false: √2 is irrational. □

This proof is thousands of years old and still impresses. Notice its structure: we assumed √2 was rational, manipulated the assumption to derive a contradiction, and concluded it cannot be rational. The contradiction — "gcd = 1 but both p and q are even" — is not invented; it follows inevitably from the assumption. The proof does not *find* an explicit counterexample to rationality; it shows that no rational number can square to 2 by demonstrating that the assumption of rationality destroys itself.

## Infinitely Many Primes

Another classic, attributed to Euclid (though his original proof was slightly different in formulation):

**Theorem**: There are infinitely many prime numbers.

**Proof**: Suppose, for contradiction, that there are only finitely many primes: p₁, p₂, ..., pₙ.

Consider the number N = p₁ · p₂ · ... · pₙ + 1.

Since N > 1, it has at least one prime divisor (by the Fundamental Theorem of Arithmetic, Chapter 8). Call this prime divisor p.

Now, p must be one of p₁, ..., pₙ (by our assumption that these are all the primes). But p divides N and p divides p₁ · p₂ · ... · pₙ. Therefore p divides N - p₁ · p₂ · ... · pₙ = 1. But no prime divides 1 (primes are > 1 by definition, and 1 has no prime factors).

**Contradiction**.

Therefore there are infinitely many primes. □

Observe that the proof does not find an explicit infinite list of primes — it shows that *any* finite list is necessarily incomplete. We have proved an existence statement (there is a prime not on the list) without exhibiting it explicitly. This is characteristic of classical proof by contradiction.

## Constructive vs. Classical: When Does Contradiction Work?

Here lies a philosophical fault line. In **classical logic**, proof by contradiction for positive statements — proving P by assuming ¬P and deriving ⊥ — is fully valid. The law of excluded middle (P ∨ ¬P) guarantees that if ¬P leads to contradiction, P must hold.

In **intuitionistic logic** (the default of Lean and Coq without classical axioms), the situation is more nuanced. Proving ¬P by assuming P and deriving ⊥ is fine — that is just the definition of ¬P (a function from P to ⊥). But proving P from ¬¬P (double negation elimination) requires the classical axiom `Classical.em`.

For our infinitely many primes proof: the statement "there are infinitely many primes" can be expressed as ∀n, ∃ prime p with p > n. This can be proved constructively (and in fact our proof *is* essentially constructive: given n, compute N = n! + 1 or the product of primes up to n + 1, and find a prime factor greater than n). The proof-by-contradiction framing adds some elegance but is not *necessary* here.

For √2 irrational, the situation is similar: in constructive mathematics, irrationality of a number α is typically defined as "α ≠ p/q for all integers p, q with q ≠ 0," and this can be proved constructively too.

The cases where classical contradiction is *necessary* — where there is no constructive proof — are more exotic: the Bolzano-Weierstrass theorem, non-constructive existence results, etc. But those cases exist, and they explain why classical mathematicians and intuitionists disagree about the scope of valid proof.

## The Emotional Experience of a Contradiction Proof

Good proofs by contradiction have a particular dramatic shape. The setup is quiet and businesslike: "Suppose, for contradiction, that P is false." The argument proceeds calmly, following the consequences of the assumption. And then, at a key moment, two facts that the assumption has generated collide head-on — "and this contradicts our assumption that..." — and everything collapses.

The emotional effect is almost cinematic. The mathematician Godfrey Harold Hardy compared it to a chess gambit for good reason: you deliberately put yourself in a seemingly compromised position (accepting the negation of your conclusion) and use the resulting imbalance to achieve a decisive win.

But beneath the drama is a precise logical structure: the assumption generated a constraint (gcd = 1, or there are only finitely many primes) and then the analysis derived a fact that violated that constraint (both p and q are even, or a prime outside the finite list exists). The contradiction is always specific and derivable, never pulled from thin air.

---

*Next: Proof by contrapositive — a lighter-weight classical tool for conditional statements.*
