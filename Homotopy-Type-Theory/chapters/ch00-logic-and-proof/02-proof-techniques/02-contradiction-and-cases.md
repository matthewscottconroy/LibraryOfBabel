# 2.2 Proof by Contradiction and Cases

## Proof by Contradiction

*Proof by contradiction* (also called *reductio ad absurdum*) works as follows:

To prove $\varphi$:
1. Assume $\neg\varphi$ (the negation of what you want to prove).
2. Derive a *contradiction* — a statement of the form $\psi \wedge \neg\psi$ for some $\psi$, or equivalently derive $\mathbf{F}$ (false).
3. Conclude: since $\neg\varphi$ leads to a contradiction, $\neg\varphi$ must be false, so $\varphi$ must be true.

The logical basis: $(\neg\varphi \to \mathbf{F}) \to \varphi$ — this uses the *law of excluded middle*, $\varphi \vee \neg\varphi$. If $\neg\varphi$ is impossible, then $\varphi$ is the only option.

**When to use contradiction:** When you have no direct path to $\varphi$, but you can squeeze something useful out of assuming $\neg\varphi$. It's especially effective when $\varphi$ asserts that something *doesn't exist* or is *impossible*, since negating gives you something that does exist, which you can then manipulate.

**Example 1: Irrationality of $\sqrt{2}$.**

*Claim:* $\sqrt{2}$ is irrational (cannot be written as $p/q$ with $p, q$ integers).

*Proof.* Assume for contradiction that $\sqrt{2}$ is rational. Then $\sqrt{2} = p/q$ for integers $p, q$ with $\gcd(p, q) = 1$ (we can always reduce to lowest terms). Squaring both sides:
$$2 = \frac{p^2}{q^2} \implies p^2 = 2q^2$$

So $p^2$ is even, which means $p$ is even (by the contrapositive proved earlier). Write $p = 2m$. Then:
$$p^2 = 4m^2 = 2q^2 \implies q^2 = 2m^2$$

So $q^2$ is even, hence $q$ is even. But then $p$ and $q$ are both divisible by 2, contradicting $\gcd(p, q) = 1$.

This contradiction shows our assumption was false. Therefore $\sqrt{2}$ is irrational. $\square$

The contradiction here is: "$p$ and $q$ share factor 2" AND "$\gcd(p, q) = 1$" (which means they share no factor).

**Example 2: Infinitely many primes.**

*Claim:* There are infinitely many prime numbers.

*Proof.* Assume for contradiction there are only finitely many primes: $p_1, p_2, \ldots, p_n$. Consider the number:
$$N = p_1 \cdot p_2 \cdots p_n + 1$$

$N$ is greater than 1, so it has a prime factor $p$. Now, $p$ must be one of the $p_i$ (since those are all the primes). But $N$ divided by any $p_i$ leaves remainder 1 (since $N = p_1 \cdots p_n + 1$), so no $p_i$ divides $N$. Contradiction.

Therefore, no finite list contains all primes, so there are infinitely many. $\square$

## The Constructive Caveat

Proof by contradiction is a *classical* technique. It relies on the law of excluded middle: $\varphi \vee \neg\varphi$. In *constructive* or *intuitionistic* mathematics (which is the foundation of Homotopy Type Theory), you cannot in general conclude $\varphi$ from $\neg\varphi \to \mathbf{F}$.

The constructive reading of "there exist infinitely many primes" is: give an algorithm that, given any finite list of primes, produces a prime not in the list. The proof above *does* provide such an algorithm (form $N$ and find its prime factor), but the reasoning was presented non-constructively.

For the irrationality of $\sqrt{2}$: this can also be proved constructively, but the argument looks different. The fact that we assumed $\neg\varphi$ and derived a contradiction is valid constructively when $\varphi$ is a *negative* statement (i.e., already a negation). "Irrational" means "not rational," so this case actually works constructively too.

We return to this in Chapter 5. For now, use contradiction freely — but be aware of when it might not transfer to a constructive setting.

## Proof by Cases

*Proof by cases* is the pattern: if $\varphi \vee \psi$ holds, and $\chi$ follows from $\varphi$ and $\chi$ also follows from $\psi$, then $\chi$ holds. This is *disjunction elimination* in natural deduction:

$$\frac{\Gamma \vdash \varphi \vee \psi \quad \Gamma, \varphi \vdash \chi \quad \Gamma, \psi \vdash \chi}{\Gamma \vdash \chi}$$

The cases must be *exhaustive* (together they cover all possibilities) and each case must lead to the same conclusion.

**Example 3.**

*Claim:* For all integers $n$, $n^2 + n$ is even.

*Proof.* Every integer is either even or odd (exhaustive cases).

*Case 1: $n$ is even.* Write $n = 2k$. Then $n^2 + n = 4k^2 + 2k = 2(2k^2 + k)$, which is even.

*Case 2: $n$ is odd.* Write $n = 2k + 1$. Then:
$$n^2 + n = (2k+1)^2 + (2k+1) = 4k^2 + 4k + 1 + 2k + 1 = 4k^2 + 6k + 2 = 2(2k^2 + 3k + 1)$$
which is even.

In both cases $n^2 + n$ is even. $\square$

**Example 4: Absolute value inequality.**

*Claim:* For all real $x$, $|x| \geq 0$ and $|x| = 0 \iff x = 0$.

*Proof.* Recall $|x| = x$ if $x \geq 0$, and $|x| = -x$ if $x < 0$.

*Case 1: $x \geq 0$.* Then $|x| = x \geq 0$. Also $|x| = 0 \iff x = 0$. ✓

*Case 2: $x < 0$.* Then $|x| = -x > 0$ (since $x < 0$). Also $|x| = 0 \iff -x = 0 \iff x = 0$, but $x < 0$ means $x \neq 0$, so $|x| \neq 0$. ✓

In both cases, $|x| \geq 0$ and the condition $|x| = 0 \iff x = 0$ holds. $\square$

## Organizing Multi-Case Proofs

When a proof has many cases, organization is essential. Standard practice:

1. **State the cases explicitly**: "We consider two cases: $x \geq 0$ and $x < 0$."
2. **Label each case**: "Case 1:", "Case 2:", etc.
3. **Within each case**, proceed as in a direct proof.
4. **Conclude**: "In all cases, the conclusion holds. $\square$"

For many cases, a table can be cleaner than narrative:

**Example 5: Parity multiplication table.**

*Claim:* even × even = even, even × odd = even, odd × odd = odd.

| $m$ | $n$ | $mn$ |
|-----|-----|------|
| $2j$ | $2k$ | $2(2jk)$ = even |
| $2j$ | $2k+1$ | $2j(2k+1) = 2(j(2k+1))$ = even |
| $2j+1$ | $2k+1$ | $(2j+1)(2k+1) = 4jk+2j+2k+1 = 2(2jk+j+k)+1$ = odd |

Each row is a case; the conclusion is verified by direct calculation. $\square$

## Contradiction vs. Contrapositive: Know the Difference

Both contradiction and contrapositive introduce $\neg\varphi$ somewhere. The difference:

- **Contrapositive** of $P \to Q$: assume $\neg Q$, derive $\neg P$. Only changes the direction of the implication.
- **Contradiction**: assume $\neg\varphi$ (any formula), derive $\mathbf{F}$. More powerful but less constructive.

In practice: if you're proving $P \to Q$ and want to use $\neg Q$, you're doing the contrapositive (which is constructively valid). If you're trying to prove $\varphi$ outright by assuming $\neg\varphi$, you're using contradiction (which requires excluded middle).

**Example 6: Which technique?**

*Claim:* For all integers $n$, if $3n + 2$ is odd then $n$ is odd.

*Contrapositive version:* If $n$ is even, then $3n + 2$ is even.

*Proof (by contrapositive).* Assume $n = 2k$. Then $3n + 2 = 6k + 2 = 2(3k + 1)$, which is even. $\square$

No contradiction needed — just the contrapositive. The contrapositive was easier.

## When Contradiction Is Unavoidable

Some existence results essentially require contradiction (or the law of excluded middle). For example:

*Claim:* For every real number $x$, either $x$ is rational or $x$ is irrational.

This follows immediately from $\varphi \vee \neg\varphi$ with $\varphi$ = "$x$ is rational." This is a tautology in classical logic. Constructively, we cannot always decide which case holds — there exist real numbers for which we cannot computably determine rationality.

This kind of example previews why constructive and classical mathematics diverge, a theme we'll explore systematically in Chapter 5.
