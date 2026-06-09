# 11.4 Fractal Geometry

Strange attractors are fractal sets. But "fractal" is not a definition — it is a description. To actually *measure* how complicated these sets are, we need a rigorous notion of dimension that can handle objects that are neither smooth curves nor surfaces nor solid regions. That notion is the Hausdorff dimension.

## 11.4.1 Hausdorff Dimension

The key idea is to measure a set by asking: what is the smallest total "size" we need to cover it, using balls of radius at most $\delta$? We measure size with $d$-dimensional "rulers" — covers where each piece contributes $|U_i|^d$ to the total. As $\delta \to 0$, for most values of $d$ the answer is either $0$ or $\infty$; there is a critical threshold where it jumps.

**Definition 11.4.1.** The *Hausdorff $d$-measure* of a set $A \subseteq \mathbb{R}^n$ is:
$$\mathcal{H}^d(A) = \lim_{\delta \to 0} \inf\left\{\sum_i |U_i|^d : A \subseteq \bigcup_i U_i,\ |U_i| \leq \delta\right\}.$$

The *Hausdorff dimension* of $A$ is
$$\dim_H(A) = \inf\{d : \mathcal{H}^d(A) = 0\} = \sup\{d : \mathcal{H}^d(A) = \infty\}.$$

What this is saying is: there is a unique critical value $d^*$ where $\mathcal{H}^d(A)$ jumps from $\infty$ to $0$. That critical value is the Hausdorff dimension. For smooth objects, it equals the topological dimension. For fractals, it is strictly larger than the topological dimension and non-integer.

**Theorem 11.4.2 (Basic Properties).**
1. $\dim_H(A) \in [0, n]$ for $A \subseteq \mathbb{R}^n$
2. $\dim_H(A) \leq \dim_H(B)$ if $A \subseteq B$ (monotonicity)
3. $\dim_H\!\left(\bigcup_n A_n\right) = \sup_n \dim_H(A_n)$ for countable unions
4. $\dim_H(\mathbb{R}^n) = n$; the Hausdorff dimension of a smooth $k$-manifold is $k$

**Landmark examples:**

- The standard middle-thirds Cantor set $C$: $\dim_H(C) = \log 2 / \log 3 \approx 0.631$. It is a subset of the real line with no isolated points, no intervals, and yet it is uncountable. Its dimension reflects exactly how efficiently it can be covered: at scale $3^{-n}$, you need $2^n$ intervals, so the dimension is $\log 2 / \log 3$.
- The Lorenz attractor: $\dim_H \approx 2.06$. It sits inside $\mathbb{R}^3$ but is "just above" a 2-dimensional surface.
- The Hénon attractor: $\dim_H \approx 1.26$. It sits inside $\mathbb{R}^2$ but is barely more than a curve.

## 11.4.2 Box-Counting Dimension

Hausdorff dimension is theoretically elegant but computationally expensive. For numerical work, there is a more tractable alternative: count how many boxes of side $\varepsilon$ you need to cover the set.

**Definition 11.4.3.** The *box-counting dimension* (or *Minkowski dimension*) is:
$$\dim_B(A) = \lim_{\varepsilon \to 0} \frac{\log N(A, \varepsilon)}{\log(1/\varepsilon)},$$
where $N(A, \varepsilon)$ is the minimum number of balls of radius $\varepsilon$ needed to cover $A$.

Box-counting dimension is easier to estimate numerically than Hausdorff dimension — you can compute it directly from simulated orbits by counting boxes. In general, $\dim_H(A) \leq \dim_B(A)$, but for "nice" self-similar fractal sets (those with enough regularity, like self-similar attractors satisfying an open set condition), $\dim_H = \dim_B$.

## 11.4.3 Information Dimension and the Kaplan-Yorke Conjecture

There is a third notion of dimension that takes into account the *distribution* of the invariant measure, not just the support.

**Definition 11.4.4.** The *information dimension* of a measure $\mu$ is:
$$d_1(\mu) = \lim_{\varepsilon \to 0} \frac{\int \log \mu(B(x,\varepsilon))\,d\mu(x)}{\log \varepsilon}.$$

Rather than asking how many boxes cover the set, we ask how much probability is contained in a typical ball of radius $\varepsilon$. The information dimension is the exponent: $\mu(B(x,\varepsilon)) \sim \varepsilon^{d_1}$ for $\mu$-typical $x$.

Now here is a remarkable conjecture connecting dimension to the dynamical invariants we computed in Chapter 8:

**Conjecture 11.4.5 (Kaplan-Yorke / Lyapunov Dimension).** For an SRB measure $\mu$ on a chaotic attractor with Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_n$, the information dimension is:
$$d_1(\mu) = j + \frac{\lambda_1 + \cdots + \lambda_j}{|\lambda_{j+1}|},$$
where $j$ is the largest index with $\lambda_1 + \cdots + \lambda_j \geq 0$.

What this formula is saying is: start adding up Lyapunov exponents from largest to smallest. The "dimension" of the attractor is the number of exponents you can add before the sum goes negative, plus the fractional leftover. It is the exact point where expansion and contraction balance.

This conjecture (the *Kaplan-Yorke conjecture*) has been proven in several special cases — for Axiom A attractors, Ledrappier and Young established it — but remains open in full generality. For the Hénon map with $a = 1.4$, $b = 0.3$: the Lyapunov exponents are $\lambda_1 \approx 0.419$ and $\lambda_2 \approx -1.623$ (with $\lambda_1 + \lambda_2 = \log 0.3 \approx -1.204$). The Kaplan-Yorke formula gives $d_{KY} = 1 + \lambda_1 / |\lambda_2| \approx 1.258$, which matches the numerically estimated Hausdorff dimension very well.

The information dimension connects naturally to the multifractal analysis of Section 11.6 — it is the first member ($q = 1$ in the Rényi dimension spectrum) of a whole family of dimension quantities that together characterize the fractal geometry of invariant measures.
