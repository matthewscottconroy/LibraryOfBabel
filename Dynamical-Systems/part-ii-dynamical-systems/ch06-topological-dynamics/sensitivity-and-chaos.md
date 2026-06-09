# 6.7 Sensitivity and Chaos

"Chaos" is one of those words that means something precise in mathematics and something vague everywhere else. The challenge is to define it in a way that captures what makes chaotic systems genuinely different: the simultaneous presence of global mixing, dense periodic structure, and unpredictability. Robert Devaney proposed a definition in the late 1980s that has become the standard starting point.

**Definition 6.7.1 (Devaney's Chaos).** A TDS $(X, f)$ is *chaotic in the sense of Devaney* if:
1. $f$ is topologically transitive
2. Periodic points are dense in $X$
3. $f$ has *sensitive dependence on initial conditions*: there exists $\delta > 0$ such that for any $x \in X$ and $\varepsilon > 0$, there exist $y \in B(x, \varepsilon)$ and $n \geq 0$ with $d(f^n(x), f^n(y)) > \delta$.

Transitivity says the system is globally indecomposable. Dense periodic points says the long-term structure is complicated everywhere — no open set is free of periodic behavior. Sensitive dependence says that no matter how precisely you know the initial condition, small errors will eventually produce large discrepancies.

---

## The Surprising Redundancy

Here is one of the most elegant theorems in the subject, proved in 1992.

**Theorem 6.7.2 (Banks et al., 1992).** If $(X, f)$ is transitive and periodic points are dense, then $f$ is sensitive. Hence sensitive dependence is implied by the first two conditions.

*Proof idea:* Take any $x$ and $\varepsilon > 0$. Find a periodic orbit $\mathcal{O}(p)$ and consider the distance between $\mathcal{O}(p)$ and $\mathcal{O}(q)$ for another periodic orbit. Transitivity allows one to find nearby points that shadow different periodic orbits and hence diverge.

What this is really saying: if you have a topologically transitive system with dense periodic orbits, then sensitive dependence comes for free. The system can't possibly be equicontinuous — the periodic orbits, being dense and distinct, force initial conditions to eventually separate. Sensitive dependence isn't an independent axiom; it's a *consequence* of the global dynamics.

This is a good lesson in how the three conditions in Devaney's definition interact. You'd expect three independent conditions, but the first two already imply the third. The definition was formulated to match our intuitions; the theorem reveals the logical structure underneath.

---

## Li-Yorke Chaos

Devaney's definition is topological. There's another notion of chaos, rooted in a 1975 paper by Tien-Yien Li and James Yorke, that focuses on pairs of points with mixed recurrence behavior.

**Definition 6.7.3 (Li-Yorke Chaos).** $(X, f)$ is *Li-Yorke chaotic* if there exists an uncountable set $S \subseteq X$ such that for all $x \neq y$ in $S$:
$$\limsup_n d(f^n(x), f^n(y)) > 0 \quad \text{and} \quad \liminf_n d(f^n(x), f^n(y)) = 0.$$

Such a set $S$ is a *scrambled set*.

The condition captures what it feels like to watch two orbits that are entangled: they come close to each other repeatedly ($\liminf = 0$), but they also pull apart again ($\limsup > 0$). Neither converges to the other, and neither escapes the other. They're in a kind of permanent, unresolved conflict.

**Theorem 6.7.4 (Li-Yorke, 1975).** If $f: [a,b] \to [a,b]$ has a period-3 orbit, then $f$ has orbits of every period and is Li-Yorke chaotic. More generally: "Period 3 implies chaos."

The Li-Yorke theorem was a landmark — published in the *American Mathematical Monthly* under the title "Period Three Implies Chaos," it brought the mathematics of chaos to a general audience and coined the term "chaos" in its modern dynamical sense. The proof is elementary but the conclusion is stunning: a single period-3 orbit forces the existence of an uncountable scrambled set and orbits of every period. Complexity is compelled by the simplest possible recurrence.

In the next section, we shift gears from orbit structure to measures — asking whether a given map has any natural probability measure that it preserves.
