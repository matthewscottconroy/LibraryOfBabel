# 25.4 Undecidability in Dynamical Systems

Dynamical systems and computability theory seem like they should have nothing to do with each other. One studies orbits of continuous maps on metric spaces; the other studies algorithms and their limits. But when we ask decision problems about dynamical systems — "Is this system transitive?" "Does this cellular automaton have positive entropy?" — we are asking questions about algorithms, and some of those questions are as hard as the halting problem.

**Theorem 25.4.1 (Undecidable Properties of Dynamical Systems).** The following problems are undecidable (no algorithm can solve them for all inputs):
1. **Transitivity**: given a computable dynamical system, is it topologically transitive?
2. **Positive entropy**: given a cellular automaton rule, does it have positive topological entropy?
3. **Tiling**: given a finite set of Wang tiles, can they tile the plane?
4. **Emptiness of 2D SFTs**: given a finite set of forbidden patterns, is the corresponding 2D SFT nonempty?

*(These all reduce to the halting problem.)*

Let's understand why these are hard.

**Transitivity** of a computable system means there exists a dense orbit. But density is a $\Pi_2^0$ condition (for all open sets $U$, eventually the orbit enters $U$), and checking this requires knowing the long-term behavior of the system — which can encode the halting behavior of an arbitrary Turing machine.

**Positive entropy** for cellular automata: a cellular automaton with rule $R$ has $h_{\text{top}}(R) > 0$ iff it can generate exponentially many distinguishable orbits. But whether a CA generates complex behavior from simple initial conditions is tied to whether it can simulate universal computation — and universal computation implies the halting problem.

**Wang tiles** are unit squares with colored edges: a set of tiles tiles the plane if you can cover the plane with copies of the tiles (from the given set, with repetition) such that adjacent tiles have matching edge colors. The tiling problem — can a given finite set of Wang tiles tile the plane? — is undecidable:

**Theorem 25.4.2 (Berger 1966).** The Wang tiling problem is undecidable. As a consequence, there exist *aperiodic* tile sets — sets of Wang tiles that tile the plane but only aperiodically (Berger; Robinson's simpler example).

Berger's proof encodes halting computations in tilings. If the Turing machine halts, the tiling eventually becomes periodic; if it doesn't halt, the tiling is necessarily aperiodic. Since there's no algorithm for the halting problem, there's no algorithm for the tiling problem.

**Connection to Subshifts:** A set of Wang tiles defines a 2D subshift $X_\tau$. The emptiness problem for $X_\tau$ is $\Pi_1^0$-complete (all computably enumerable sets are involved). 2D SFTs can simulate Turing machines.

The key structural fact: a 2D SFT can enforce that certain patterns appear together — and these "together" constraints can encode computation. A tiling that simulates a Turing machine must (if it tiles at all) carry a trace of the machine's computation. If the machine runs forever, the trace extends infinitely and the tiling is aperiodic. If the machine halts, the trace terminates and the tiling can (in principle) be periodic.

This is why aperiodic tilings necessarily exist: the undecidability of the halting problem forces them. If every tile set either failed to tile or tiled periodically, the tiling problem would be decidable. Since it's not decidable, there must be tile sets that tile only aperiodically.

The Penrose tilings and the Ammann-Beenker tilings (which show up in quasicrystal physics) are explicit examples. The undecidability argument tells you they must exist; the explicit constructions tell you what they look like. The connection between aperiodic order (quasicrystals) and computational undecidability (Wang tiles, 2D SFTs) is one of the genuinely unexpected connections in modern mathematics.
