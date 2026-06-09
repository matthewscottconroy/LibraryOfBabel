# 11.1 What is Chaos?

"Chaos" is one of those words that means something precise in mathematics and something vague everywhere else. Weather forecasters use it to mean "unpredictable." Physicists use it to mean "turbulent." Newspaper writers use it to mean "complicated." For us, it will mean something specific — and the precision matters, because once you have the definition right, you can prove things.

The bad news is that there is not one definition, but three commonly used ones, each capturing a different facet of the same phenomenon. The good news is that for well-behaved systems on infinite spaces, they are closely related. Let's lay them all out and then compare.

## 11.1.1 Three Definitions Compared

The first definition is due to Robert Devaney, and it captures the topological structure of chaos: global mixing of orbits, ubiquitous periodicity, and sensitivity to initial conditions.

**Definition 11.1.1 (Devaney Chaos).** A continuous map $f: X \to X$ is *Devaney chaotic* if:
1. $f$ is topologically transitive (there is a dense orbit)
2. The periodic points of $f$ are dense in $X$
3. $f$ has sensitive dependence on initial conditions (SDIC): there exists $\delta > 0$ such that for every $x \in X$ and every $\varepsilon > 0$, there is $y$ with $d(x,y) < \varepsilon$ and $n$ with $d(f^n(x), f^n(y)) > \delta$.

What this is saying is: no matter where you start in the space, nearby starting points will eventually diverge (sensitivity); orbits mix everywhere (transitivity); and regular periodic behavior is interwoven densely throughout the chaos. It is a remarkable theorem — due to Banks, Brooks, Cairns, Davis, and Stacey — that (1) and (2) together already imply (3) when $X$ is infinite. Sensitivity comes for free.

The second definition is due to Li and Yorke, and it focuses on a combinatorial phenomenon: the existence of orbit pairs that behave wildly.

**Definition 11.1.2 (Li-Yorke Chaos).** A continuous map $f: X \to X$ is *Li-Yorke chaotic* if there exists an uncountable set $S \subseteq X$ (a *scrambled set*) such that for every pair $x \neq y$ in $S$:
$$\liminf_{n \to \infty} d(f^n(x), f^n(y)) = 0 \quad \text{and} \quad \limsup_{n \to \infty} d(f^n(x), f^n(y)) > 0.$$

Each pair is *proximal* (returns arbitrarily close) and *distal* (also separates infinitely often). The orbits are perpetually tangled — neither converging nor diverging, but oscillating between closeness and separation. Li and Yorke's 1975 paper, where they showed "period 3 implies chaos," introduced this notion and gave it its name.

The third definition takes the information-theoretic route: chaos is positive topological entropy.

**Definition 11.1.3 (Positive Entropy Chaos).** $f: X \to X$ is chaotic if $h_{\text{top}}(f) > 0$.

Topological entropy measures the exponential growth rate of distinguishable orbit segments. Positive entropy means that the system creates new, irreducible complexity at an exponential rate — it is informationally productive.

## Relationships Between the Definitions

These three definitions are related, but not equivalent. Here is the precise picture:

**Relationships:** Positive topological entropy implies Li-Yorke chaos (Blanchard-Glasner-Kolyada-Maass, 2002). Li-Yorke chaos does not imply positive entropy — there are zero-entropy systems with scrambled sets (Morse sequence, Toeplitz sequences). Devaney chaos on an infinite space implies Li-Yorke chaos and positive entropy.

So the implication chain runs roughly as: Devaney chaos (on infinite spaces) $\Rightarrow$ positive entropy $\Rightarrow$ Li-Yorke chaos. But the converses fail in general. For "generic" or "natural" systems — especially those arising from smooth dynamics — all three typically hold together, which is why the distinction is usually not emphasized in practice. It matters at the boundary cases, where subtle examples live.

The conceptual takeaway is this: chaos is not a single phenomenon but a family of related phenomena — sensitivity, mixing, information production, and orbit tangling — that tend to occur together. The choice of definition shapes what you can prove and how you measure it.

In the sections that follow, we will see all three definitions at work. The Lorenz system is Devaney chaotic; the multifractal analysis in Section 11.6 measures the information-theoretic complexity; and the relationship to entropy is made quantitative by Pesin's formula in Section 11.7.
