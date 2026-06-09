# Proof Techniques

Knowing the vocabulary of logic is not the same as knowing how to prove things. A proof is a directed argument that begins with hypotheses and ends with a conclusion, with every step justified by a prior result or a definition. The challenge is finding a path through this argument — and mathematicians have developed a small collection of general strategies that apply across an enormous range of problems. This section presents those strategies, illustrated with examples drawn from the kind of mathematics that appears throughout real analysis and differential equations.

## Direct Proof

A direct proof of $P \Rightarrow Q$ assumes $P$ is true and derives $Q$ through a chain of logical inferences, each step following from the previous by a known fact or definition. This is the default strategy: when nothing about a theorem suggests that an indirect approach is needed, try the direct route first.

**Example.** Prove: if $m$ and $n$ are even integers, then $m + n$ is even.

Assume $m$ and $n$ are even. By definition, there exist integers $a$ and $b$ such that $m = 2a$ and $n = 2b$. Then $m + n = 2a + 2b = 2(a + b)$. Since $a + b$ is an integer, $m + n$ is even. $\square$

The structure here is transparent: unpack the definition of the hypothesis, perform an algebraic manipulation, and recognize that the result fits the definition of the conclusion.

## Proof by Contrapositive

The implication $P \Rightarrow Q$ is logically equivalent to its contrapositive $\neg Q \Rightarrow \neg P$. When the hypothesis $P$ is hard to work with directly but the negation $\neg Q$ provides useful information, the contrapositive route is preferable.

**Example.** Prove: if $n^2$ is even, then $n$ is even (for $n \in \mathbb{Z}$).

The contrapositive is: if $n$ is odd, then $n^2$ is odd. Assume $n$ is odd, so $n = 2k + 1$ for some integer $k$. Then
$$n^2 = (2k+1)^2 = 4k^2 + 4k + 1 = 2(2k^2 + 2k) + 1,$$
which is odd. The contrapositive is proved, so the original implication holds. $\square$

Note that a direct proof — starting from "$n^2$ is even" — would require factoring $n^2$ in a way that reveals something about $n$, which is harder. The contrapositive form begins with a clean assumption.

## Proof by Contradiction

To prove $Q$ by contradiction, assume $\neg Q$ and derive a logical impossibility — a statement that contradicts an axiom, a hypothesis, or a previously established fact. The impossibility shows that $\neg Q$ cannot hold, so $Q$ must be true.

**Example.** Prove that $\sqrt{2}$ is irrational.

Assume, for contradiction, that $\sqrt{2}$ is rational. Then $\sqrt{2} = p/q$ where $p, q \in \mathbb{Z}$, $q \neq 0$, and $p/q$ is in lowest terms (so $p$ and $q$ are not both even). Squaring both sides, $2 = p^2/q^2$, so $p^2 = 2q^2$. Thus $p^2$ is even, which by the previous example implies $p$ is even. Write $p = 2m$. Then $4m^2 = 2q^2$, giving $q^2 = 2m^2$, so $q^2$ is even and $q$ is even. But then $p$ and $q$ are both even, contradicting the assumption that $p/q$ is in lowest terms. $\square$

This proof is a canonical example of contradiction: the only way to derive the conclusion is to suppose its negation and reach an impossibility.

## Mathematical Induction

Induction proves statements of the form $\forall n \in \mathbb{N}, P(n)$ (or $\forall n \geq n_0$, for some base case $n_0$). The method rests on a foundational property of the natural numbers: there is no infinite descending chain $n_1 > n_2 > n_3 > \cdots$ in $\mathbb{N}$, which implies that any property that "spreads upward" from a base case must hold for all natural numbers.

**The Principle of Mathematical Induction.** Suppose $P(n_0)$ is true, and suppose that for every $k \geq n_0$, $P(k) \Rightarrow P(k+1)$. Then $P(n)$ is true for all $n \geq n_0$.

**Example.** Prove: for all $n \geq 1$, $\displaystyle\sum_{k=1}^n k = \frac{n(n+1)}{2}$.

*Base case.* For $n = 1$: the left side is $1$, the right side is $\frac{1 \cdot 2}{2} = 1$. True.

*Inductive step.* Assume $\sum_{k=1}^m k = \frac{m(m+1)}{2}$ for some $m \geq 1$ (the inductive hypothesis). Then
$$\sum_{k=1}^{m+1} k = \left(\sum_{k=1}^m k\right) + (m+1) = \frac{m(m+1)}{2} + (m+1) = (m+1)\left(\frac{m}{2} + 1\right) = \frac{(m+1)(m+2)}{2}.$$
This is the formula at $n = m+1$, completing the inductive step. By the Principle of Mathematical Induction, the formula holds for all $n \geq 1$. $\square$

**Strong Induction.** In the strong form, the inductive hypothesis assumes $P(j)$ holds for all $j$ with $n_0 \leq j \leq k$, and the inductive step shows $P(k+1)$. This is useful when $P(k+1)$ depends on earlier cases besides just $P(k)$.

**Example (sketch).** Every integer $n \geq 2$ has a prime factorization. The base case $n = 2$ is clear ($2$ is prime). For the inductive step, assume every integer $2 \leq j \leq k$ has a prime factorization. If $k+1$ is prime, it is its own prime factorization. If $k+1$ is composite, write $k+1 = ab$ with $2 \leq a, b \leq k$; by the strong inductive hypothesis, $a$ and $b$ each have prime factorizations, and their concatenation is a prime factorization of $k+1$.

## Existence and Uniqueness Proofs

Many theorems assert both existence ("there is an $x$ such that...") and uniqueness ("at most one $x$ satisfies..."). These are proved separately.

**Existence** is proved by exhibiting a specific object, by a constructive argument, or by a non-constructive argument (e.g., contradiction showing no such object cannot fail to exist).

**Uniqueness** is typically proved by assuming two objects satisfy the conditions and showing they must be equal. Suppose $x$ and $y$ both satisfy the property; then derive $x = y$.

**Example.** For any real numbers $a$ and $b$, the equation $a + x = b$ has a unique solution in $\mathbb{R}$.

*Existence.* Take $x = b - a$. Then $a + x = a + (b - a) = b$. $\checkmark$

*Uniqueness.* If $a + x_1 = b$ and $a + x_2 = b$, then $a + x_1 = a + x_2$, so $x_1 = x_2$. $\square$

## Proof by Cases

When a predicate involves objects that naturally fall into several categories, a proof by cases handles each category separately. The union of the cases must cover all possibilities.

**Example.** Prove: for all integers $n$, $n^2 + n$ is even.

If $n$ is even, then $n = 2k$, so $n^2 + n = 4k^2 + 2k = 2(2k^2 + k)$, which is even. If $n$ is odd, then $n = 2k+1$, so $n^2 + n = (2k+1)^2 + (2k+1) = 4k^2 + 4k + 1 + 2k + 1 = 4k^2 + 6k + 2 = 2(2k^2 + 3k + 1)$, which is even. Since every integer is either even or odd, the result holds in all cases. $\square$

## Writing Style and Common Pitfalls

**Be explicit about assumptions.** Begin each proof by stating what is assumed. The phrase "let $\varepsilon > 0$ be given" signals that you are introducing a universal variable; "choose $\delta = \varepsilon / 2$" signals an existential construction.

**Distinguish "if" from "only if."** Proving "A if B" means proving $B \Rightarrow A$. Proving "A only if B" means proving $A \Rightarrow B$. An "if and only if" proof requires both directions.

**Avoid circular reasoning.** In a proof by contradiction, the contradiction must be with something already established, not with the thing being proved.

**Mark the end of proofs.** The symbol $\square$ (or "QED" from the Latin quod erat demonstrandum) signals that the proof is complete. In the body of a proof, $\checkmark$ is sometimes used to mark that a subgoal has been achieved.

These techniques are the complete toolkit for the rest of this course. Every proof of every theorem in the subsequent chapters — the Completeness Axiom's consequences, the convergence of Picard iterates, the existence of eigenvalues — will use one or more of these strategies. Becoming fluent in all of them, particularly induction and contradiction, is the primary goal of this chapter.
