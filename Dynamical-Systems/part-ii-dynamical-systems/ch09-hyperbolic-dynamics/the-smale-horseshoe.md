# 9.1 The Smale Horseshoe

## Construction

In the early 1960s, Stephen Smale invented a geometric construction that became the Rosetta Stone of chaotic dynamics. The horseshoe map shows, with complete mathematical clarity, how a simple geometric operation — stretch vertically, squeeze horizontally, fold into a U-shape — produces an invariant set of extraordinary complexity.

Let's see this in action.

**Construction 9.1.1 (Smale Horseshoe).** Start with the unit square $Q = [0,1]^2$. The horseshoe map $f: Q \to {\mathbb R}^2$ is constructed as follows:
1. Compress $Q$ vertically by factor $\lambda < 1/2$ and stretch horizontally by $\mu > 2$.
2. Bend the resulting strip into a horseshoe shape and place it so it intersects $Q$ in two vertical strips $V_0$ and $V_1$ (the preimages of the two "legs" of the horseshoe).

More precisely: $f$ maps two horizontal strips $H_0, H_1 \subset Q$ to two vertical strips $V_0, V_1 \subset Q$ via:
$$f(H_0) = V_0, \quad f(H_1) = V_1,$$
with expansion by $\mu$ in the horizontal direction and contraction by $\lambda$ in the vertical.

Think of it this way. You take the square, stretch it until it's very thin and wide, then fold it into a U-shape and lay it back over the original square so that the two legs of the U each land inside the square. Each leg is thin (because of the vertical contraction) and wide (because of the horizontal expansion). The map $f$ sends the two horizontal strips $H_0$ and $H_1$ (which are the parts of the square whose images land back in the square) to the vertical strips $V_0$ and $V_1$.

**Definition 9.1.2.** The *invariant Cantor set* of the horseshoe is:
$$\Lambda = \bigcap_{n \in {\mathbb Z}} f^n(Q).$$

This is the set of points whose entire orbit stays in $Q$ under both forward and backward iteration.

The invariant set $\Lambda$ is the set of points that stay in the square forever — under both forward and backward iteration. Most points eventually escape; only those in the intersection of all forward and backward images remain. This intersection is a Cantor set.

---

## Symbolic Description

The key to the horseshoe is that the Cantor set $\Lambda$ can be completely described by a simple symbolic coding.

**Theorem 9.1.3.** The invariant set $\Lambda$ of the horseshoe is homeomorphic to the full two-shift $\{0,1\}^{\mathbb Z}$ via the coding map $\pi: \Lambda \to \{0,1\}^{\mathbb Z}$, $\pi(x)_n = i$ iff $f^n(x) \in H_i$.

The map $\pi$ conjugates $f|_\Lambda$ to the shift $\sigma$ on $\{0,1\}^{\mathbb Z}$.

The coding is beautiful. For each point $x \in \Lambda$, you record which horizontal strip ($H_0$ or $H_1$) the orbit visits at each time step: $\pi(x)_n = 0$ if $f^n(x) \in H_0$, and $\pi(x)_n = 1$ if $f^n(x) \in H_1$. This gives a bi-infinite binary sequence. The theorem says this coding is a homeomorphism: the topology of $\Lambda$ is captured exactly by the product topology on $\{0,1\}^{\mathbb Z}$, and the dynamics of $f|_\Lambda$ is conjugate to the shift.

**Consequences:**
- $\Lambda$ is a Cantor set (compact, perfect, totally disconnected)
- $f|_\Lambda$ has a dense orbit (coded by a sequence visiting all words)
- $f|_\Lambda$ has a dense set of periodic orbits (coded by periodic sequences)
- $f|_\Lambda$ has $2^n$ periodic orbits of period $n$ (one for each binary string)
- The topological entropy of $f|_\Lambda$ is $\log 2$

**Remark 9.1.4.** The horseshoe shows that simple geometric operations (stretch, fold) produce extraordinary complexity in the orbit structure. This construction motivated Smale's abstract theory of hyperbolic sets.

The $2^n$ periodic orbits of period $n$ are coded by the $2^n$ binary strings of length $n$. The dense orbit is coded by a sequence that eventually contains every finite binary word as a substring — a de Bruijn sequence. The entropy $\log 2$ comes from the fact that the "symbol rate" is one bit per iterate.

This is the prototype for the entire theory of hyperbolic dynamics. In the next section, we extract the key geometric property that makes the horseshoe work, and abstract it into the definition of a hyperbolic set.
