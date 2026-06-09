# Chapter 05: Continuity

Continuity is the mathematical formalization of the idea that small changes in input produce small changes in output. It is the property that separates "well-behaved" functions — those that can be drawn without lifting a pen, that represent physically realizable quantities, that can be integrated and approximated — from pathological ones. In the theory of differential equations, continuity appears everywhere: the right-hand side $f(t, y)$ of the ODE $y' = f(t, y)$ must be continuous (or at least measurable) for solutions to make sense, and the Picard-Lindelof existence theorem requires Lipschitz continuity. This chapter builds the theory rigorously.

## The Definition and Basic Properties

The epsilon-delta definition of continuity at a point extends the limit definition from sequences to functions. A function $f: D \to \mathbb{R}$ is continuous at $a \in D$ if
$$\forall \varepsilon > 0,\ \exists \delta > 0,\ \forall x \in D,\ |x - a| < \delta \Rightarrow |f(x) - f(a)| < \varepsilon.$$
Section 1 develops this definition, proves the arithmetic properties (sums, products, and quotients of continuous functions are continuous), and establishes that compositions of continuous functions are continuous. The sequential characterization — $f$ is continuous at $a$ iff $f(x_n) \to f(a)$ for every sequence $x_n \to a$ — connects this chapter to Chapter 3.

## The Intermediate Value Theorem

Section 2 proves the Intermediate Value Theorem: if $f: [a,b] \to \mathbb{R}$ is continuous and $f(a) < c < f(b)$, then there exists $x \in (a,b)$ with $f(x) = c$. The proof uses the Completeness Axiom directly: let $s = \sup\{x \in [a,b] : f(x) \leq c\}$ and show $f(s) = c$. This theorem is the existence component of many results: it shows that equations have solutions without constructing them.

## The Extreme Value Theorem

Section 3 proves that a continuous function on a closed bounded interval attains its maximum and minimum. The key idea: the image of a compact set under a continuous map is compact (closed and bounded), and any closed bounded subset of $\mathbb{R}$ has a maximum and minimum. The proof proceeds via sequences, using Bolzano-Weierstrass.

## Uniform Continuity

Section 4 introduces uniform continuity, a stronger form of continuity where a single $\delta$ works for all base points $a$ simultaneously:
$$\forall \varepsilon > 0,\ \exists \delta > 0,\ \forall x, y \in D,\ |x - y| < \delta \Rightarrow |f(x) - f(y)| < \varepsilon.$$
The Heine-Cantor theorem states that every continuous function on a closed bounded interval is uniformly continuous. Uniform continuity is exactly the property needed to approximate an integral uniformly by Riemann sums, and it appears in the hypothesis of one standard form of the Picard theorem.

## How the Sections Connect

The four sections build logically: the epsilon-delta definition and basic properties are prerequisites for the IVT and EVT, which both use completeness arguments. Uniform continuity refines pointwise continuity using compactness (which the EVT proof makes precise). Together, these results form the complete theory of continuity on closed intervals that supports the integration theory of Chapter 7 and the ODE existence theory beyond.
