# Shannon Entropy and Information

Claude Shannon's 1948 paper "A Mathematical Theory of Communication" founded information theory. Its central concept — entropy — measures uncertainty, information content, and the compressibility of data. This section treats entropy the way a logician should: not as one plausible formula among many, but as the *unique* function satisfying a short list of axioms. That uniqueness theorem is the foundational result of the subject, in the same sense that soundness and completeness are logic's.

## The Definition

**Definition (Shannon entropy).** Let $X$ be a discrete random variable taking values in $\mathcal{X} = \{x_1, \dots, x_n\}$ with probabilities $p_i = \Pr[X = x_i]$. The **entropy** of $X$ is
$$H(X) \;=\; -\sum_{i=1}^{n} p_i \log_2 p_i,$$
with the convention $0 \log_2 0 = 0$, justified by continuity: $t \log_2 t \to 0$ as $t \to 0^{+}$.

Units: bits (base-2 logarithm), nats (natural logarithm), hartleys (base 10). We use bits throughout.

The quantity $-\log_2 p_i$ is the **surprisal** of outcome $x_i$: rare outcomes surprise more. Entropy is expected surprisal — equivalently, the average number of yes/no questions an optimal interrogator needs to pin down the value of $X$.

## Worked Computations

**Fair coin.** $p = (\tfrac12, \tfrac12)$: $H = -\tfrac12\log_2\tfrac12 - \tfrac12\log_2\tfrac12 = 1$ bit.

**Biased coin.** $p = (0.9, 0.1)$:
$$H = -(0.9\log_2 0.9 + 0.1\log_2 0.1) \approx 0.9(0.152) + 0.1(3.322) \approx 0.469 \text{ bits}.$$
A heavily biased coin carries less than half a bit per flip: its outcome sequences are compressible, a claim the next section makes exact.

**Fair die.** $p = (\tfrac16, \dots, \tfrac16)$: $H = \log_2 6 \approx 2.585$ bits. Minimum entropy is $0$, attained when one outcome has probability $1$: no uncertainty, no information.

```python
import math

def entropy(probs):
    return -sum(p * math.log2(p) for p in probs if p > 0)

print(entropy([0.5, 0.5]))   # 1.0 bit
print(entropy([0.9, 0.1]))   # 0.469 bits
print(entropy([1/6] * 6))    # 2.585 bits
print(entropy([1.0]))        # 0.0 bits
```

## The Axiomatic Characterization

Why $-\sum p_i \log_2 p_i$ and not some other formula? The logician's answer: write down what any reasonable measure of uncertainty must satisfy, then prove the axioms have exactly one model. Let $H_n(p_1,\dots,p_n)$ be a family of functions, one for each $n$. The **Khinchin axioms** (1953):

1. **Continuity.** Each $H_n$ is continuous in $(p_1,\dots,p_n)$.
2. **Maximality at uniform.** $H_n(p_1,\dots,p_n) \le H_n(\tfrac1n,\dots,\tfrac1n)$.
3. **Expansibility.** $H_{n+1}(p_1,\dots,p_n,0) = H_n(p_1,\dots,p_n)$: impossible outcomes carry no uncertainty.
4. **Grouping (strong additivity).** For a two-stage experiment, $H(X,Y) = H(X) + \sum_i p_i\, H(Y \mid X = x_i)$: total uncertainty is the uncertainty of the first stage plus the expected uncertainty of the second.

**Theorem (Uniqueness; Khinchin 1953, Faddeev 1956).** Any family satisfying axioms 1–4 has the form
$$H_n(p_1,\dots,p_n) = -c\sum_{i} p_i \log_2 p_i$$
for some constant $c > 0$. Normalizing $H_2(\tfrac12,\tfrac12) = 1$ yields Shannon entropy exactly.

*Proof idea.* Let $f(n) = H_n(\tfrac1n,\dots,\tfrac1n)$. Grouping applied to two independent uniform choices gives the functional equation $f(nm) = f(n) + f(m)$; expansibility plus maximality gives monotonicity $f(n) \le f(n+1)$. A classical argument pins such $f$ down: given $n$ and $t$, choose $k$ with $n^k \le 2^t < n^{k+1}$; multiplicativity and monotonicity sandwich $k\,f(n) \le t\,f(2) \le (k+1)f(n)$, and letting $t \to \infty$ forces $f(n) = f(2)\log_2 n$. Next, for rational probabilities $p_i = a_i/N$, apply grouping to the uniform distribution on $N$ outcomes partitioned into blocks of sizes $a_1, \dots, a_n$:
$$f(N) = H_n(p_1,\dots,p_n) + \sum_i p_i f(a_i),$$
so $H_n(p) = c\bigl(\log_2 N - \sum_i p_i \log_2 a_i\bigr) = -c\sum_i p_i \log_2 p_i$. Continuity extends the formula from rational to arbitrary probabilities. $\square$

Faddeev later showed a leaner axiom set suffices (continuity and symmetry of $H_2$, plus one recursion for splitting a single outcome in two). The moral stands either way: entropy is not chosen, it is *forced*. The axioms form a small theory whose model is unique up to the choice of logarithm base — the "logic" of information.

## Joint and Conditional Entropy

**Definition (Joint and conditional entropy).** For jointly distributed $X, Y$:
$$H(X,Y) = -\sum_{x,y} p(x,y)\log_2 p(x,y), \qquad H(Y \mid X) = -\sum_{x,y} p(x,y)\log_2 p(y \mid x).$$
$H(Y \mid X)$ is the expected entropy remaining in $Y$ after observing $X$.

**Theorem (Chain rule).** $H(X,Y) = H(X) + H(Y \mid X)$.

*Proof.* Since $p(x,y) = p(x)\,p(y \mid x)$,
$$H(X,Y) = -\sum_{x,y} p(x,y)\bigl[\log_2 p(x) + \log_2 p(y \mid x)\bigr]
= -\sum_{x} p(x)\log_2 p(x) \;-\; \sum_{x,y} p(x,y)\log_2 p(y \mid x),$$
using $\sum_y p(x,y) = p(x)$ to collapse the first sum. The two terms are $H(X)$ and $H(Y \mid X)$. $\square$

Notice that the chain rule *is* the grouping axiom: the axiomatization promotes a provable identity to a defining principle, exactly as algebra does with associativity.

## Mutual Information

**Definition (Mutual information).** $I(X;Y) = H(X) - H(X \mid Y)$: the number of bits that knowing $Y$ saves about $X$. By the chain rule,
$$I(X;Y) = H(X) + H(Y) - H(X,Y) = I(Y;X),$$
so information is symmetric. It is non-negative (see below) and zero exactly when $X$ and $Y$ are independent.

In logical dress: $I$ measures how much the truth-value of one proposition constrains another; $I = 0$ is the probabilistic analogue of logical irrelevance.

## Jensen's Inequality and Maximum Entropy

**Lemma (Jensen's inequality).** If $f$ is concave and $Z$ is a real random variable, then $\mathbb{E}[f(Z)] \le f(\mathbb{E}[Z])$; for strictly concave $f$, equality holds iff $Z$ is constant.

**Theorem (Maximum-entropy bound).** $H(X) \le \log_2 |\mathcal{X}|$, with equality iff $X$ is uniform on $\mathcal{X}$.

*Proof.* Restrict to the support $S = \{i : p_i > 0\}$ and apply Jensen to the strictly concave $\log_2$ with $Z = 1/p(X)$:
$$H(X) = \mathbb{E}\Bigl[\log_2 \tfrac{1}{p(X)}\Bigr] \;\le\; \log_2 \mathbb{E}\Bigl[\tfrac{1}{p(X)}\Bigr] = \log_2 \sum_{i \in S} p_i \cdot \tfrac{1}{p_i} = \log_2 |S| \;\le\; \log_2 |\mathcal{X}|.$$
Equality in Jensen requires $1/p(X)$ constant, i.e. $X$ uniform on its support; equality overall additionally requires $S = \mathcal{X}$. $\square$

The same technique proves $I(X;Y) \ge 0$, hence **conditioning cannot increase entropy**: $H(X \mid Y) \le H(X)$. On average, information never hurts — though for a *particular* observation $y$, $H(X \mid Y = y)$ can exceed $H(X)$.

## Looking Ahead

So far entropy is a definition with theorems attached. Its *operational* meaning is Shannon's **source coding theorem**: $H(X)$ is exactly the optimal rate, in bits per symbol, for lossless compression of an i.i.d. source — no code beats entropy, and codes approaching it exist (the Kraft inequality and Huffman/arithmetic coding supply the construction; see Cover & Thomas, ch. 5). We take that theorem as background and turn instead to a sharper question. The [next section](../02_kolmogorov/01_descriptive_complexity.md) replaces the probability distribution with an *individual string* — **Kolmogorov complexity** measures the information in one object, with no source model at all — and later sections turn incompressibility into a working [proof method](../02_kolmogorov/02_incompressibility_method.md) for combinatorics and number theory, connect it to [algorithmic randomness and an incompleteness theorem](../03_randomness/01_martin_lof_and_chaitin.md), and separate raw information from organized content ([logical depth](../04_depth/01_logical_depth.md)). Channel capacity, Shannon's other great theorem, lies off our route.

## Exercises
See [problems/ch17_information_theory/](../../../problems/ch17_information_theory/)
