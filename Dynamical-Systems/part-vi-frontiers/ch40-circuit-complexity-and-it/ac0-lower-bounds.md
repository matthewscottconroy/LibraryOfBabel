# 40.3 AC⁰ Lower Bounds: Switching Lemma

For constant-depth circuits with polynomial size — the class AC$^0$ — we have provably exponential lower bounds for specific functions. The key tool is Håstad's switching lemma, combined with Fourier analysis of Boolean functions.

**Theorem 40.3.1 (Håstad's Switching Lemma, 1987).** Let $f: \{0,1\}^n \to \{0,1\}$ be computed by a size-$m$ DNF (disjunctive normal form with clauses of width $w$). For a random restriction $\rho$ that fixes each variable independently to $0$ or $1$ with probability $1-p$ each and leaves it free with probability $p$:
$$\Pr_\rho[f|_\rho \text{ requires decision tree of depth} > t] \leq (5pw)^t.$$

A random restriction fixes most variables to constants, leaving a few free. The switching lemma says: after a random restriction, a DNF becomes "simple" — it can be computed by a shallow decision tree, with high probability. This lets you show that constant-depth circuits can't compute parity.

**Corollary 40.3.2 (Parity Not in AC⁰).** The parity function $\oplus_n(x) = x_1 \oplus \cdots \oplus x_n$ requires exponential size in $\mathbf{AC}^0$. More precisely, any depth-$d$ circuit for parity requires size $\exp(\Omega(n^{1/(d-1)}))$.

**Information-Theoretic Intuition:** Parity is information-theoretically "maximally sensitive" — flipping any one bit flips the output. Constant-depth circuits cannot track this sensitivity without exponential size.

## 40.3.2 Fourier Analysis of Boolean Functions

The Fourier perspective on Boolean functions gives a cleaner way to understand AC$^0$ lower bounds and proves a much stronger quantitative statement about sensitivity.

**Definition 40.3.3.** Every $f: \{0,1\}^n \to \{-1,1\}$ has a *Fourier expansion* $f(x) = \sum_{S \subseteq [n]} \hat{f}(S) \prod_{i \in S} (-1)^{x_i}$, where $\hat{f}(S) = \mathbb{E}_x[f(x)\prod_{i \in S}(-1)^{x_i}]$.

**Parseval's Identity:** $\sum_S \hat{f}(S)^2 = 1$.

**Theorem 40.3.4 (Total Influence = Noise Sensitivity).** The *total influence* $I(f) = \sum_i \Pr_x[f(x) \neq f(x \oplus e_i)]$ satisfies $I(f) = \sum_S |S| \hat{f}(S)^2$. Functions in $\mathbf{AC}^0$ have $I(f) = O((\log n)^{d-1})$ (polynomial of log), while parity has $I(\oplus_n) = n$.

The total influence of parity is $n$ — flipping any bit changes the output, so parity is sensitive to every bit. AC$^0$ functions have influence at most polylog$(n)$. Since parity has influence $n$, parity cannot be in AC$^0$. This is the Fourier-analytic proof of the parity lower bound.

The Fourier expansion of Boolean functions is a genuinely useful tool, not just for proving lower bounds. It appears in learning theory (the spectral learning algorithm), in derandomization, and in social choice theory (Arrow's theorem has a clean Fourier proof).
