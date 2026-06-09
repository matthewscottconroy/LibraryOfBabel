# 7.1 Measure-Preserving Transformations

## Definitions

The central object of ergodic theory is deceptively simple to state: a transformation that preserves a probability measure. Why do we care? Because this is the mathematical model of a *statistical equilibrium* — a system that doesn't settle down to a fixed point, but whose overall statistics don't change over time. Think of gas molecules bouncing around in a box: chaotic individually, but statistically stable in aggregate. That stability is exactly what measure-preservation captures.

**Definition 7.1.1.** A *measure-preserving transformation (MPT)* is a quadruple $(X, \mathcal{B}, \mu, f)$ where $(X, \mathcal{B}, \mu)$ is a probability space and $f: X \to X$ is measurable with $\mu(f^{-1}(A)) = \mu(A)$ for all $A \in \mathcal{B}$.

If $f$ is invertible (bijective, with $f^{-1}$ measurable), the system is an *invertible MPT* or an *automorphism*.

The measure-preservation condition $\mu(f^{-1}(A)) = \mu(A)$ is worth reading twice. It says: the set of points that map into $A$ has the same measure as $A$ itself. Equivalently, if you observe the system at two different times, the distribution of states is the same. Statistically, the system is in equilibrium.

**Definition 7.1.2.** Two MPTs $(X, \mathcal{B}, \mu, f)$ and $(Y, \mathcal{C}, \nu, g)$ are *measurably isomorphic* if there exists a measure-preserving bijection $\varphi: (X, \mu) \to (Y, \nu)$ with $\varphi \circ f = g \circ \varphi$ $\mu$-a.e.

This is the correct notion of "sameness" for MPTs: a bijection that preserves both the measure and the dynamics. Notice the "a.e." qualifier — in measure theory, things that differ on measure-zero sets are considered the same. This flexibility is important; it lets us ignore bad sets and focus on the essential structure.

---

## Standard Examples

Let's build intuition with the four canonical examples, each of which will reappear throughout the book.

**Example 7.1.3 (Circle Rotation).** $X = {\mathbb T} = {\mathbb R}/{\mathbb Z}$, $\mu$ = Lebesgue measure, $R_\alpha(x) = x + \alpha \pmod{1}$.
- $\mu(R_\alpha^{-1}(A)) = \mu(A + \alpha) = \mu(A)$ (Lebesgue measure is translation-invariant).

The circle rotation is the simplest non-trivial MPT. It's also uniquely ergodic (for irrational $\alpha$) and has zero entropy. Think of it as the "tame" benchmark — we'll use it to test what strong properties like mixing actually require.

**Example 7.1.4 (Doubling Map).** $X = [0,1]$, $\mu$ = Lebesgue, $f(x) = 2x \pmod{1}$.
- Check: $f^{-1}([a,b]) = [a/2, b/2] \cup [(a+1)/2, (b+1)/2]$, which has measure $b-a = \mu([a,b])$.

The doubling map is the simplest chaotic MPT. It's the continuous version of the shift map on binary sequences, and it has entropy $\log 2$. Notice that measure-preservation here is a calculation, not a theorem — you're checking the preimage formula directly.

**Example 7.1.5 (Bernoulli Shift).** $X = \{0, 1, \ldots, k-1\}^{\mathbb Z}$ (bi-infinite sequences), $\mu = p^{\otimes {\mathbb Z}}$ (product measure with weights $p_0, \ldots, p_{k-1}$), $\sigma(x)_n = x_{n+1}$ (left shift).
- The Bernoulli shift is the fundamental model of an independent process.

The Bernoulli shift is the probabilist's dream: each time-step produces an outcome drawn independently from the distribution $(p_0, \ldots, p_{k-1})$. Product measure is the canonical measure for independent processes, and it's preserved by the shift because shifting doesn't change any finite-dimensional marginal.

**Example 7.1.6 (Toral Automorphism).** $X = {\mathbb T}^2$, $\mu$ = Lebesgue, $f_A: (x,y) \mapsto (2x+y, x+y) \pmod{1}$ where $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix} \in SL(2,{\mathbb Z})$. Since $\det(A) = 1$, $f_A$ preserves area (Lebesgue measure).

The Arnold cat map (as this system is called, after Vladimir Arnold) is the standard example of a hyperbolic MPT. Its eigenvalues are $(3 \pm \sqrt{5})/2$, both irrational, and it's a prototypical example of the Anosov systems we'll study in Chapter 9.

**Example 7.1.7 (Gauss Map).** $X = [0,1]$, $\mu_G = \frac{1}{\ln 2} \frac{dx}{1+x}$ (Gauss measure), $G(x) = \{1/x\}$ (fractional part of $1/x$). This models the continued fraction expansion of $x$.

The Gauss map is more exotic: it has an invariant measure that is not Lebesgue, but is absolutely continuous with respect to Lebesgue. This is the measure governing the statistics of continued fraction expansions — a beautiful connection between dynamics and number theory that we'll develop in Chapter 23.

These four examples are not chosen arbitrarily. They represent four distinct "levels" of complexity: rotations (zero entropy, purely point spectrum), the Gauss map (zero entropy but more complicated), the doubling map and toral automorphisms (positive entropy), and Bernoulli shifts (maximum complexity). Understanding how they differ is the project of this chapter.
