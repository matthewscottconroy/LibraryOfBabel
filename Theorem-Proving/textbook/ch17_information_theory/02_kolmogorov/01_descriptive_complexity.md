# Kolmogorov Complexity

Shannon entropy measures the information in a *distribution* — it needs a probability model. Kolmogorov complexity measures the information in an *individual string*: how short a description it has, how "random" it is, with no probabilities anywhere. Proposed independently by Solomonoff (1964), Kolmogorov (1965), and Chaitin (1966), it is the bridge from information theory to logic, because "shortest description" is a statement about programs and their outputs — and programs are the subject matter of computability (Chapter 10).

## Definition

Fix a universal Turing machine $U$ (Chapter 10): one that, given a program $p$, computes $U(p)$, and can simulate any other machine given a description of it. Work over binary strings, $x \in \{0,1\}^\ast$.

**Definition (Plain Kolmogorov complexity).** $C_U(x) = \min\{\, |p| : U(p) = x \,\}$ — the length of the shortest program that outputs $x$ on $U$ and halts.

**Definition (Prefix complexity).** If $U$ is restricted to a **prefix-free** program set (no valid program is a prefix of another — the machine reads its input with no end-marker and must decide when to stop), the resulting measure is written $K_U(x)$. Prefix complexity is the technically preferred variant: it satisfies the Kraft inequality $\sum_x 2^{-K(x)} \le 1$, so $2^{-K(x)}$ behaves like a probability, and it makes subadditivity and the symmetry-of-information theorem clean. We write $C$ and $K$ where the distinction matters and speak loosely of "Kolmogorov complexity" otherwise.

$K(x)$ is the **true description length** of $x$: the least information, in bits, needed to single it out.

## The Invariance Theorem

The definition mentions a particular machine $U$. The foundational result is that the choice barely matters.

**Theorem (Invariance; Solomonoff–Kolmogorov).** There is a universal machine $U$ such that for every machine $V$ there is a constant $c_V$ with
$$C_U(x) \le C_V(x) + c_V \qquad \text{for all } x.$$

*Proof.* Let $U$ be universal: on input $\langle V\rangle p$ — a self-delimiting description of $V$ followed by $p$ — it simulates $V$ on $p$. If $p$ is a shortest $V$-program for $x$, then $\langle V\rangle p$ is a $U$-program for $x$, so $C_U(x) \le |p| + |\langle V\rangle| = C_V(x) + c_V$, where $c_V = |\langle V\rangle|$ depends on $V$ but **not on $x$**. $\square$

Applying this with the roles of two universal machines swapped, $|C_U(x) - C_{U'}(x)| \le c$ for a constant $c$ independent of $x$. So Kolmogorov complexity is well-defined **up to an additive constant**: the choice of programming language shifts every value by at most a fixed amount. We henceforth fix a reference $U$ and drop the subscript.

## Basic Bounds

- **Upper bound.** $C(x) \le |x| + O(1)$: the program "print the following literal: $x$" has length $|x|$ plus a constant. For prefix complexity the self-delimiting overhead costs a logarithm: $K(x) \le |x| + 2\log_2 |x| + O(1)$ (write the length of $x$ in a prefix-free code, then $x$).
- **Complexity is subadditive.** $K(x,y) \le K(x) + K(y) + O(\log)$: describe $x$, then $y$.

## Uncomputability

**Theorem.** The function $C$ (and likewise $K$) is **not computable**.

*Proof (Berry's paradox, formalized).* Suppose $C$ were computable. Define a program $B$ that, given $n$, searches strings in length-lexicographic order and outputs the *first* string $x$ with $C(x) \ge n$. Such an $x$ exists for every $n$ (there are $2^n - 1$ strings of length $< n$ but infinitely many strings, so some string has complexity $\ge n$), and by assumption the search is effective. But $B$ on input $n$ has size $|B| + O(\log n)$ — a constant for $B$ plus the bits to write $n$ — and it *outputs a string of complexity $\ge n$*. Hence
$$n \le C(x) \le |B| + 2\log_2 n + O(1).$$
For large $n$ the right side is smaller than $n$: contradiction. So $C$ is not computable. $\square$

This is the same self-reference that drives the halting problem (Chapter 10) — indeed $C$ computable would decide halting — and, in Section 4, the same paradox yields Chaitin's incompleteness theorem. Berry's "the least integer not nameable in fewer than nineteen syllables" is not a curiosity but a proof technique.

## Most Strings Are Incompressible

Even though we cannot compute $C(x)$, we can count.

**Theorem (Incompressibility).** For every length $n$ and every $c \ge 1$, at most $2^{\,n-c} - 1$ strings of length $n$ satisfy $C(x) < n - c$. Hence at least a $(1 - 2^{-c})$ fraction of length-$n$ strings have $C(x) \ge n - c$.

*Proof.* Every string with $C(x) < n-c$ is the output of some program of length $< n-c$. The number of binary programs of length $< n-c$ is $\sum_{i=0}^{n-c-1} 2^i = 2^{\,n-c} - 1$. A program outputs at most one string, so at most $2^{\,n-c}-1$ strings of length $n$ are that compressible. There are $2^n$ strings of length $n$, so the compressible ones are a fraction $\le (2^{n-c}-1)/2^n < 2^{-c}$. $\square$

**Definition (Kolmogorov randomness).** A string $x$ is **incompressible** (or $c$-random) if $C(x) \ge |x| - c$. The theorem says almost every string is incompressible: randomness is the generic case, structure the exception. Crucially, incompressible strings *exist for every length* — a fact used constantly in the [next section](02_incompressibility_method.md), since it lets a proof "choose a random object" purely by counting, with no probabilistic machinery.

## Relation to Shannon Entropy

The two theories agree in the mean. For a string $x_{1:n}$ drawn i.i.d. from a computable source with per-symbol entropy $H$:
$$H \;\le\; \frac{1}{n}\,\mathbb{E}\big[K(x_{1:n})\big] \;\le\; H + \frac{O(\log n)}{n},$$
so expected Kolmogorov complexity per symbol converges to the Shannon entropy rate (Cover & Thomas, Thm. 14.3.1; Li & Vitányi, §8.1). Entropy is thus the *average* description length; Kolmogorov complexity resolves that average into the description length of each *individual* outcome — and reveals that within a typical sample almost every particular sequence is incompressible, which entropy alone cannot express.

## Conditional Complexity and Symmetry of Information

$K(x \mid y)$ is the length of the shortest program that outputs $x$ *given $y$ for free* on an auxiliary tape. The algorithmic **symmetry of information** (Kolmogorov–Levin) mirrors Shannon's chain rule up to a logarithmic term:
$$K(x, y) = K(x) + K(y \mid x^\ast) + O(\log),$$
where $x^\ast$ is a shortest program for $x$. The *algorithmic mutual information* $I_K(x{:}y) = K(x) - K(x \mid y^\ast)$ is symmetric up to $O(\log)$ — the individual-string counterpart of $I(X;Y)$. The parallel with Section 1 is exact: every entropy identity has a Kolmogorov analogue holding pointwise up to a logarithmic slack.

With incompressible strings shown to exist by counting, and $K$ shown uncomputable by Berry's paradox, we have both halves of the method that follows: a supply of objects no short program can produce, and the leverage to derive contradictions from describing them.

## Exercises
See [problems/ch17_information_theory/](../../../problems/ch17_information_theory/)
