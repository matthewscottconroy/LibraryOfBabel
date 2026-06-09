# 27.3 The Recursion Theorem and Fixed Points

Fixed-point theorems are everywhere in mathematics. Brouwer's theorem gives fixed points for continuous maps on compact convex sets. Banach's theorem gives them for contractions. Kleene's recursion theorem gives them for computable transformations on programs — and when you understand it as a statement about dynamical systems, it illuminates something that otherwise seems like a logical curiosity.

The setup: the space of Turing machines can be indexed by natural numbers (we write $\phi_e$ for the function computed by Turing machine $e$). Any computable function $f: \mathbb{N} \to \mathbb{N}$ induces a transformation on this space — reindexing the programs. The recursion theorem says every such transformation has a fixed point.

**Theorem 27.3.1 (Kleene's Recursion Theorem).** For any computable function $f: \mathbb{N} \to \mathbb{N}$, there exists $e \in \mathbb{N}$ such that $\phi_e = \phi_{f(e)}$ (where $\phi_e$ is the function computed by Turing machine $e$).

**Dynamical Interpretation:** Let the space of Turing machines be $T = \{0, 1, 2, \ldots\}$ with the step function $e \mapsto f(e)$ (reindexing). The recursion theorem says $f$ has a "self-reproducing" fixed point — a program that outputs its own index.

The dynamical picture: $f$ is a map on the space of programs; the recursion theorem is a fixed-point theorem for this map. The fixed point $e$ satisfies $f(e) \approx e$ in the sense that the programs $\phi_e$ and $\phi_{f(e)}$ compute the same function — they have the same behavior even if they have different codes.

**Application: Self-Replicating Programs.** Quines (programs that output their own source code) exist in any sufficiently powerful language, by the recursion theorem. This is the dynamical statement: the program is a fixed point of the "run and print" transformation.

A quine isn't magic — it's a forced existence result. If you define the transformation $f$ that takes any program and outputs a program that would print $f$'s output, then $f$ must have a fixed point: a program that, when run, does exactly what $f$ said it would. Self-reference is compulsory.

Rice's theorem takes the fixed-point idea and derives something darker: no computable algorithm can decide any nontrivial behavioral property of programs.

**Theorem 27.3.2 (Rice's Theorem).** Let $P$ be any nontrivial property of computable functions (neither all functions have $P$ nor none do). Then the problem "does $\phi_e$ have property $P$?" is undecidable.

**Dynamical Reformulation:** Rice's theorem says that nontrivial asymptotic properties of the dynamical system $(\mathbb{N}, e \mapsto \phi_e(0))$ are undecidable. "Does this orbit converge?", "Is this orbit eventually periodic?", "Is this orbit bounded?" — all undecidable for the universal Turing machine.

Every question about the long-time behavior of a computable process — does it converge? does it cycle? does it stay in a bounded region? — is undecidable in general. This is not a statement about exotic systems; it's a theorem about all computable dynamical systems. The undecidability is not a technical nuisance to be worked around; it's a fundamental feature of computation-as-dynamics.

We turn next to what can be said when we restrict to computable real numbers — the setting where dynamics lives in practice.
