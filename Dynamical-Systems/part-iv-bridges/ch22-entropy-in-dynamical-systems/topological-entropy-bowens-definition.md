# 22.2 Topological Entropy — Bowen's Definition

The original definition of topological entropy, due to Adler, Konheim, and McAndrew in 1965, used open covers. It works, but the connection to information theory isn't immediately visible. Bowen's 1971 definition makes the coding interpretation explicit — it's a cleaner definition and a more transparent one.

The idea is this: two points in a dynamical system are "distinguishable" up to time $n$ at resolution $\varepsilon$ if their orbits separate by more than $\varepsilon$ somewhere in the first $n$ steps. The topological entropy is the exponential growth rate of the maximum number of distinguishable points, as $\varepsilon \to 0$ and $n \to \infty$. This is directly the number of bits per unit time needed to specify an orbit to arbitrary precision.

**Definition 22.2.1 (Bowen, 1971).** For a continuous map $f$ on a compact metric space $(X, d)$:

A set $E \subseteq X$ is $(n, \varepsilon)$-*separated* if for all distinct $x, y \in E$: $\max_{0 \leq k \leq n-1} d(f^k(x), f^k(y)) > \varepsilon$.

Let $s_n(\varepsilon) = $ maximum size of an $(n, \varepsilon)$-separated set. The *topological entropy* is:
$$h_{\text{top}}(f) = \lim_{\varepsilon \to 0} \limsup_{n\to\infty} \frac{1}{n}\log s_n(\varepsilon).$$

Here $s_n(\varepsilon)$ counts how many initial conditions are "distinguishable" by observing $n$ steps of the orbit at resolution $\varepsilon$. So $\frac{1}{n} \log s_n(\varepsilon)$ is the rate of information production at that resolution. As $\varepsilon \to 0$, we push to the limit: the topological entropy is the rate at which new distinguishable orbits appear, regardless of measurement precision.

The translation into information language: to specify which one of $s_n(\varepsilon)$ orbits you are observing requires $\log_2 s_n(\varepsilon)$ bits. The topological entropy (in nats, with natural log) gives the growth rate of this bit count. A system with $h_{\text{top}} = \log 2$ doubles the number of distinguishable orbits every time step.

**Theorem 22.2.2.** Bowen's definition agrees with the Adler-Konheim-McAndrew (open cover) definition and with the growth rate of periodic orbits (for Axiom A systems).

The agreement with periodic orbit growth is particularly striking: for Axiom A systems, the number of periodic orbits of period $n$ grows like $e^{n h_{\text{top}}}$. Periodic orbits are the skeleton of a hyperbolic system, and their exponential proliferation is measured exactly by the topological entropy.

Let's calibrate with the fundamental examples.

For the full $k$-shift (all sequences in $\{0, \ldots, k-1\}^{\mathbb Z}$), any two distinct points can be distinguished at time 0 if we use resolution $\varepsilon < 1$, so $s_n(\varepsilon) = k^n$ and $h_{\text{top}} = \log k$. This makes perfect sense: a source producing $k$-symbol sequences has maximum entropy rate $\log k$ bits per symbol.

For the doubling map $T: x \mapsto 2x \pmod 1$ on $[0,1]$, two points $x$ and $y$ with $|x - y| > \varepsilon \cdot 2^{-n}$ will be $\varepsilon$-separated by step $n$. So $s_n(\varepsilon) \approx 1/\varepsilon \cdot 2^n$, giving $h_{\text{top}} = \log 2$. The doubling map produces one binary digit of information per iterate — and indeed, it literally reads off the binary digits of the initial condition.

**Interpretation:** $s_n(\varepsilon)$ counts how many initial conditions are "distinguishable" by observing $n$ steps of the orbit at resolution $\varepsilon$. The topological entropy is the exponential growth rate of distinguishable orbits — the number of bits per unit time needed to specify an orbit.

This is why topological entropy is an invariant of *topological conjugacy*: two conjugate systems have the same orbit structure, hence the same number of distinguishable orbits, hence the same topological entropy.
