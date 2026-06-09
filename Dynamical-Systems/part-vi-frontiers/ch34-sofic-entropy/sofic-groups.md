# 34.1 Sofic Groups

A group is sofic if it looks like a symmetric group from far away. More precisely: you can approximate any finite portion of the group's multiplication table by permutations on a large finite set, with small errors. Finite groups are sofic trivially. Amenable groups are sofic. Free groups are sofic. But no one has ever proved that all groups are sofic, and no one has found a non-sofic group.

**Definition 34.1.1 (Gromov-Weiss, 1999).** A countable group $\Gamma$ is *sofic* if for every $\varepsilon > 0$ and every finite set $F \subseteq \Gamma$, there exists $n \in {\mathbb N}$ and a map $\sigma: \Gamma \to \text{Sym}(n)$ (the symmetric group on $\{1,\ldots,n\}$) such that:
1. $|\{i \in [n] : \sigma(\gamma\gamma')(i) = \sigma(\gamma)\sigma(\gamma')(i)\}| \geq (1-\varepsilon)n$ for all $\gamma, \gamma' \in F$ (almost a homomorphism)
2. $|\{i \in [n] : \sigma(\gamma)(i) \neq i\}| \geq (1-\varepsilon)n$ for all $1 \neq \gamma \in F$ (almost free)

The map $\sigma$ is called a *sofic approximation*. It's an approximate embedding of $\Gamma$ into symmetric groups — approximate in the sense that multiplication is respected on most inputs, and the non-identity elements really do move most points.

**Examples 34.1.2.**
- All amenable groups are sofic (Weiss, 2000)
- All residually finite groups are sofic (e.g., free groups, $SL(n, {\mathbb Z})$)
- All sofic groups from residually amenable groups
- No non-sofic group is known to exist (this is a major open problem)

Residually finite groups are sofic because you can use the action on the cosets of finite-index subgroups as your sofic approximation. Amenable groups are sofic because you can use the Følner sets.

**Open Problem 34.1.3.** Does there exist a non-sofic group? This is one of the biggest open problems in group theory. A non-sofic group would be a group with no finite approximations whatsoever.

This question has been open since Gromov introduced sofic groups in 1999. The best candidates for non-sofic groups — certain Burnside groups, certain groups with expander properties — have all turned out to be sofic or remain unresolved. We do not know if the class of sofic groups is everything or a proper subclass of all countable groups.
