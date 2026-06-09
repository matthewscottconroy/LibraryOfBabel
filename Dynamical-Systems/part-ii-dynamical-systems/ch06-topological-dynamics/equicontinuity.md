# 6.5 Equicontinuity

We've been moving toward increasingly complex dynamical behavior. Let's pause and look at the opposite extreme — systems where the iterates of $f$ are "tame." The right notion is equicontinuity, which says that the whole family of iterates is simultaneously continuous in a uniform sense.

**Definition 6.5.1.** A TDS $(X, f)$ is *equicontinuous* if the family of iterates $\{f^n : n \geq 0\}$ is equicontinuous at every point: for all $\varepsilon > 0$ and $x \in X$, there exists $\delta > 0$ such that $d(x, y) < \delta$ implies $d(f^n(x), f^n(y)) < \varepsilon$ for all $n \geq 0$.

Think about what this is saying. Ordinary continuity of $f$ means that nearby points are mapped to nearby points under *one* application of $f$. Equicontinuity says nearby points stay nearby under *every* application of $f^n$, uniformly. No matter how many times you iterate, the initial proximity is preserved.

This is the antithesis of sensitive dependence. A system is equicontinuous when nearby points never separate — they stay close forever. The dynamics are stable in the strongest possible sense.

**Theorem 6.5.2.** An equicontinuous minimal TDS is conjugate to a group rotation on a compact group.

This is a beautiful structural theorem. It says that if you're both equicontinuous and minimal, you're essentially doing an irrational rotation, perhaps on a more exotic compact group than the circle. The dynamics are completely determined by the algebra of the group.

---

## Proximal and Distal Systems

Equicontinuity has a natural companion concept in the theory of pairs of points.

**Definition 6.5.3 (Distal and Proximal).** Two points $x, y \in X$ are *proximal* if $\inf_n d(f^n(x), f^n(y)) = 0$ (orbits can approach each other). They are *distal* if $\inf_n d(f^n(x), f^n(y)) > 0$ (orbits stay bounded away). A system is *distal* if all pairs of distinct points are distal.

Equicontinuous $\Rightarrow$ distal $\Rightarrow$ nonproximal-nontrivial.

The hierarchy here is worth internalizing. Equicontinuity is the strongest condition: not only do distinct orbits stay apart, they stay uniformly close to their initial configuration. Distality relaxes this to just "orbits of distinct points stay separated." Proximality — where orbits *can* approach each other — is the first step toward sensitive dependence and chaos.

**Remark 6.5.4.** Equicontinuous systems are the "opposite" of chaotic ones. They have no sensitive dependence. The dichotomy between equicontinuous and sensitive systems is Auslander-Yorke's theorem.

The Auslander-Yorke dichotomy is striking: for minimal systems, you're either equicontinuous (totally tame) or sensitive (every point has nearby points whose orbit eventually separates from it). There's no middle ground. This dichotomy won't be proved here, but keep it in mind as a conceptual anchor: the topological theory of dynamics is organized around this tension between tameness and sensitivity.

In the next section, we formalize when two dynamical systems are "the same."
