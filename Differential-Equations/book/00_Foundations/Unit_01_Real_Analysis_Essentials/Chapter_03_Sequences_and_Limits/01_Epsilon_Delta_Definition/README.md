# The Epsilon-Delta Definition of Sequence Limits

The statement "$a_n$ approaches $L$" or "$a_n$ converges to $L$" is familiar from calculus, but its everyday-language formulation — "the terms get closer and closer to $L$" — is not precise enough to be used in proofs. It fails to exclude, for instance, the sequence $1, 1/2, 1, 1/3, 1, 1/4, \ldots$ which "gets closer to $0$" in some vague sense but does not converge. The epsilon-N definition replaces intuition with a statement that can be directly verified and directly negated.

## The Definition

**Definition.** A sequence $(a_n)_{n=1}^\infty$ of real numbers **converges to $L \in \mathbb{R}$**, written $\lim_{n\to\infty} a_n = L$ or $a_n \to L$, if
$$\forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall n \in \mathbb{N},\ n > N \Rightarrow |a_n - L| < \varepsilon.$$

To parse this: given any positive number $\varepsilon$ (the tolerance), we can find a threshold $N$ such that every term of the sequence after the $N$-th one is within $\varepsilon$ of $L$. The crucial point is that the threshold $N$ may depend on $\varepsilon$ — as $\varepsilon$ gets smaller, $N$ typically gets larger — but for each fixed $\varepsilon$, a single $N$ works for all subsequent terms simultaneously.

**Non-convergence.** The sequence $(a_n)$ **diverges** (does not converge to $L$) if
$$\exists \varepsilon > 0,\ \forall N \in \mathbb{N},\ \exists n > N,\ |a_n - L| \geq \varepsilon.$$
This is the negation of the convergence statement. To show a sequence does not converge to $L$, exhibit a specific $\varepsilon$ and show that no matter how large $N$ is, there is always a term beyond $N$ that fails to be within $\varepsilon$ of $L$.

## Worked Examples

**Example 1.** Prove that $\lim_{n\to\infty} \frac{1}{n} = 0$.

Let $\varepsilon > 0$ be given. By the Archimedean Property, there exists $N \in \mathbb{N}$ with $N > 1/\varepsilon$. Then for any $n > N$, we have $n > N > 1/\varepsilon$, so $1/n < \varepsilon$. Therefore
$$\left|\frac{1}{n} - 0\right| = \frac{1}{n} < \varepsilon.$$
Since $\varepsilon > 0$ was arbitrary, $1/n \to 0$. $\square$

**Example 2.** Prove that $\lim_{n\to\infty} \frac{2n+1}{n+3} = 2$.

Let $\varepsilon > 0$. Compute:
$$\left|\frac{2n+1}{n+3} - 2\right| = \left|\frac{2n+1 - 2(n+3)}{n+3}\right| = \left|\frac{-5}{n+3}\right| = \frac{5}{n+3}.$$

We want $\frac{5}{n+3} < \varepsilon$, which holds when $n + 3 > 5/\varepsilon$, i.e., $n > 5/\varepsilon - 3$. Choose $N = \lceil 5/\varepsilon \rceil$ (which is larger than $5/\varepsilon - 3$ for all $\varepsilon > 0$). Then for any $n > N$:
$$\left|\frac{2n+1}{n+3} - 2\right| = \frac{5}{n+3} < \frac{5}{n} < \frac{5}{N} \leq \varepsilon. \quad \square$$

**Example 3.** Prove that $a_n = (-1)^n$ does not converge to any $L$.

For any $L$, choose $\varepsilon = 1/2$. The sequence alternates between $-1$ and $1$. If $L \geq 0$, then $|a_{2k+1} - L| = |-1 - L| = 1 + L \geq 1 > 1/2$ for all $k$. If $L < 0$, then $|a_{2k} - L| = |1 - L| = 1 - L > 1 > 1/2$ for all $k$. In either case, there are infinitely many terms at distance $\geq 1/2$ from $L$, so $a_n \not\to L$. $\square$

## The Algebra of Limits

Once the definition is in place, one derives the algebraic limit theorems.

**Theorem.** If $a_n \to L$ and $b_n \to M$, then:
1. $a_n + b_n \to L + M$.
2. $a_n \cdot b_n \to L \cdot M$.
3. If $M \neq 0$, then $a_n / b_n \to L / M$.

*Proof of (1).* Let $\varepsilon > 0$. Since $a_n \to L$, there exists $N_1$ with $|a_n - L| < \varepsilon/2$ for all $n > N_1$. Since $b_n \to M$, there exists $N_2$ with $|b_n - M| < \varepsilon/2$ for all $n > N_2$. Let $N = \max(N_1, N_2)$. For $n > N$:
$$|(a_n + b_n) - (L + M)| = |(a_n - L) + (b_n - M)| \leq |a_n - L| + |b_n - M| < \frac{\varepsilon}{2} + \frac{\varepsilon}{2} = \varepsilon. \quad \square$$

The $\varepsilon/2$ split is the standard technique for sums. For products, one writes $a_n b_n - LM = (a_n - L)b_n + L(b_n - M)$ and uses the fact that convergent sequences are bounded (to control $|b_n|$).

**Theorem (Uniqueness of Limits).** A sequence can have at most one limit.

*Proof.* Suppose $a_n \to L$ and $a_n \to M$. For any $\varepsilon > 0$, there exist $N_1, N_2$ such that $|a_n - L| < \varepsilon/2$ for $n > N_1$ and $|a_n - M| < \varepsilon/2$ for $n > N_2$. For $n > \max(N_1, N_2)$:
$$|L - M| \leq |L - a_n| + |a_n - M| < \frac{\varepsilon}{2} + \frac{\varepsilon}{2} = \varepsilon.$$
Since $|L - M| < \varepsilon$ for all $\varepsilon > 0$, and $|L - M| \geq 0$, the Archimedean "squeeze" gives $|L - M| = 0$, so $L = M$. $\square$

## The Squeeze Theorem

**Theorem (Squeeze Theorem).** Suppose $a_n \leq b_n \leq c_n$ for all sufficiently large $n$, and $\lim a_n = \lim c_n = L$. Then $\lim b_n = L$.

*Proof.* Let $\varepsilon > 0$. Choose $N$ large enough that $|a_n - L| < \varepsilon$, $|c_n - L| < \varepsilon$, and the inequalities $a_n \leq b_n \leq c_n$ hold for all $n > N$. Then $L - \varepsilon < a_n \leq b_n \leq c_n < L + \varepsilon$, so $|b_n - L| < \varepsilon$. $\square$

**Example.** $\lim_{n \to \infty} \frac{\sin n}{n} = 0$. Since $-1 \leq \sin n \leq 1$, we have $-1/n \leq \sin(n)/n \leq 1/n$. Both bounds converge to $0$, so by Squeeze, $\sin(n)/n \to 0$.

## Common Pitfalls

**Confusing large $N$ with large $n$.** The definition requires a single $N$ that works for all $n > N$ simultaneously. A proof that produces a different threshold for each term does not establish convergence.

**Circular reasoning.** Writing $|a_n - L|$ and then "taking $n$ to infinity to get $|L - L| = 0$" is circular — the limit cannot be used to establish the limit.

**Forgetting that $N$ may depend on $\varepsilon$.** This is the point. For small $\varepsilon$, a large $N$ is required. The definition is existential in $N$ for each $\varepsilon$.

The epsilon-N definition is the template for every subsequent limit notion in analysis. The epsilon-delta definition of continuity, the definition of a derivative as a limit, and the definition of a Riemann integral as a limit of sums all follow the same pattern: universal in the tolerance, existential in the threshold.
