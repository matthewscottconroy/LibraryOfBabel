# 36.6 Connections to Geometric Group Theory

The Zimmer program connects to deep algebraic theorems about the structure of higher-rank lattices. These connections run both ways: the algebraic structure constrains the dynamics, and the dynamics can illuminate the algebra.

**Theorem 36.6.1 (Margulis Normal Subgroup Theorem).** If $\Gamma \leq G$ (semisimple, higher rank) is an irreducible lattice, every normal subgroup of $\Gamma$ is either finite or of finite index.

This is a striking algebraic fact: higher-rank lattices have almost no normal subgroups. They're "simple, up to finite error." Free groups, by contrast, have enormous numbers of normal subgroups — one for every quotient group you can imagine.

The Normal Subgroup Theorem has an immediate implication for dynamics: any ergodic action of $\Gamma$ on a standard probability space must have essentially trivial kernel — the kernel is a normal subgroup, so it's either finite or all of $\Gamma$.

**Implication for Dynamics:** Ergodic actions of $\Gamma$ are classified by their *algebraic degree* — they must essentially come from algebraic actions of $G$ on algebraic varieties. The Zimmer program makes this precise in the differential geometry setting.

**Theorem 36.6.2 (Stuck-Zimmer, 1994).** Every faithful ergodic measure-preserving action of a higher-rank lattice with finite stabilizers is essentially free (has a.e. trivial stabilizer).

The Stuck-Zimmer theorem is a rigidity result for the stabilizer structure of the action. For higher-rank lattices, if you have an ergodic action with "small" stabilizers, the stabilizers are actually trivial almost everywhere. There's no room for the stabilizers to be anything other than everything or nothing.

This constrains the kinds of actions possible: a higher-rank lattice acting on a space must either have a fixed point a.e. (the action is not ergodic) or the action is essentially free. The possibility of "interesting" stabilizer structure — like free groups can have — is ruled out.

The Zimmer program thus fits into a broader picture: higher-rank lattices are algebraically rigid (Margulis's theorems), metrically rigid (property (T)), cocycle-superrigid (Zimmer), and action-superrigid (Brown-Fisher-Hurtado). Each result constrains one more type of "complexity" the group can exhibit.
