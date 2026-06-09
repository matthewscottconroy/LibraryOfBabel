# Chapter 33 — Orbit Equivalence Theory

> *Two dynamical systems are orbit equivalent if their orbit structures are the same — even if the systems themselves differ. This is a coarser equivalence than isomorphism but captures more of the "dynamical structure." For countable group actions, the theory is dominated by a single theorem: all free ergodic actions of amenable groups are orbit equivalent.*

**Prerequisites:** Chapter 7 (ergodic theory, Ornstein's theorem), Chapter 28 (groupoids, C*-algebras), Chapter 32 (Borel equivalence relations).

---

## 33.1 Orbit Equivalence for Measure-Preserving Systems

**Definition 33.1.1.** Two free ergodic measure-preserving actions $\Gamma \curvearrowright (X, \mu)$ and $\Lambda \curvearrowright (Y, \nu)$ are *orbit equivalent (OE)* if there is a measure space isomorphism $\phi: X \to Y$ such that $\phi$ maps $\Gamma$-orbits to $\Lambda$-orbits: $\phi(\Gamma \cdot x) = \Lambda \cdot \phi(x)$ for a.e. $x$.

**Definition 33.1.2.** The *orbit equivalence relation* of $\Gamma \curvearrowright X$ is:
$$\mathcal{R}_\Gamma = \{(x, y) \in X \times X : y \in \Gamma \cdot x\}.$$

Two actions are OE iff their orbit equivalence relations are isomorphic as equivalence relations (there is a measure-space isomorphism identifying them).

**Key Question:** Which pairs $(\Gamma, \Lambda)$ of groups can have orbit-equivalent free ergodic actions? Is OE-class determined by the group, or can very different groups have the same orbit structure?

---

## 33.2 Dye's Theorem and the Amenable Case

**Theorem 33.2.1 (Dye's Theorem, 1959, 1963).** All free ergodic measure-preserving actions of ${\mathbb Z}$ are mutually orbit equivalent. In particular, all free ergodic ${\mathbb Z}$-actions are OE to all other free ergodic ${\mathbb Z}$-actions, regardless of their entropy or mixing properties.

**Theorem 33.2.2 (Ornstein-Weiss, 1980).** All free ergodic measure-preserving actions of any countably infinite amenable group $\Gamma$ are orbit equivalent to each other (and to ${\mathbb Z}$-actions). The orbit equivalence class is *unique* for all amenable groups.

**Corollary 33.2.3.** The orbit equivalence class of a free ergodic action of an amenable group is completely independent of the group $\Gamma$ — only the "orbit structure" (the hyperfinite equivalence relation) matters.

**Remark 33.2.4 (Amenability).** A group $\Gamma$ is *amenable* if it admits a finitely additive left-invariant probability measure. Examples: abelian groups, nilpotent groups, solvable groups, ${\mathbb Z}^d$. Non-examples: free groups $F_2$, lattices in $SL(n, {\mathbb R})$ for $n \geq 2$.

---

## 33.3 Rigidity for Non-Amenable Groups

**The Rigidity Revolution:** In the 1990s-2000s, Furman, Gaboriau, Popa, and others showed that non-amenable groups behave very differently — their OE classes carry substantial information about the group.

**Theorem 33.3.1 (Furman, 1999).** For an action of $\Gamma = SL(n, {\mathbb Z})$ on a standard probability space: if $\Lambda \curvearrowright Y$ is orbit equivalent to $\Gamma \curvearrowright X$, then $\Lambda$ is virtually isomorphic to $\Gamma$ (up to finite index).

**Theorem 33.3.2 (Gaboriau, 2000).** The *$\ell^2$-Betti numbers* $\beta_n^{(2)}(\mathcal{R})$ of the orbit equivalence relation $\mathcal{R}$ are OE-invariants. For a free ergodic action $\Gamma \curvearrowright X$: $\beta_n^{(2)}(\mathcal{R}) = \beta_n^{(2)}(\Gamma)$ (the $\ell^2$-Betti numbers of the group).

**Corollary 33.3.3.** Free groups $F_r$ and $F_s$ (with $r \neq s$) have non-orbit-equivalent free ergodic actions, since $\beta_1^{(2)}(F_r) = r - 1 \neq s - 1 = \beta_1^{(2)}(F_s)$.

---

## 33.4 Popa's Deformation-Rigidity Theory

**Theorem 33.4.1 (Popa's Cocycle Superrigidity, 2005).** For a Bernoulli action $\Gamma \curvearrowright (X, \mu) = (X_0, \mu_0)^\Gamma$ of a group with property (T) (or more generally, a "malleable" action), every measurable cocycle $c: \Gamma \times X \to \Lambda$ is cohomologous to a group homomorphism $\rho: \Gamma \to \Lambda$.

**Theorem 33.4.2 (Popa, 2006).** For any countable groups $\Gamma$ with property (T) and $\Lambda$: any two free ergodic actions of $\Gamma$ that are orbit equivalent are actually isomorphic (not just orbit equivalent). This is *OE-superrigidity*.

**Definition 33.4.3 (Property (T)).** A group $\Gamma$ has *Kazhdan's property (T)* if every unitary representation with almost-invariant vectors has a nonzero fixed vector. Examples: $SL(n, {\mathbb Z})$ for $n \geq 3$, lattices in higher-rank Lie groups.

**Theorem 33.4.4 (Ioana, 2011).** For $\Gamma = SL(2, {\mathbb Z})$, Bernoulli actions are OE-superrigid: two Bernoulli actions of $SL(2, {\mathbb Z})$ are orbit equivalent iff they are isomorphic iff they have the same base entropy.

---

## 33.5 Cost and $\ell^2$-Betti Numbers

**Definition 33.5.1 (Levitt, 1995).** The *cost* of a free ergodic MPT action $\Gamma \curvearrowright X$ is:
$$\text{cost}(\Gamma \curvearrowright X) = \inf_\Phi \int |(\Phi(x))|\ d\mu(x),$$
where the infimum is over all generating graphings $\Phi$ (Borel graphs on $X$ whose connected components are the orbits).

**Theorem 33.5.2 (Gaboriau, 2000).** For a free ergodic action:
$$\text{cost}(\Gamma \curvearrowright X) = 1 + \beta_1^{(2)}(\Gamma) - \beta_0^{(2)}(\Gamma).$$

For $\Gamma = F_r$ (free group on $r$ generators): $\text{cost}(F_r \curvearrowright X) = r$.

**Open Problem 33.5.3 (Fixed Price Problem).** Does every free ergodic action of $\Gamma$ have the same cost (independent of the action)? This is true for amenable groups and free groups, but open in general.

---

## Exercises

**Exercise 33.1.** Verify that orbit equivalence is indeed an equivalence relation on free ergodic MPT actions. Show that if $\phi$ is an OE between $\Gamma \curvearrowright X$ and $\Lambda \curvearrowright Y$, then $\phi^{-1}$ is an OE between $\Lambda \curvearrowright Y$ and $\Gamma \curvearrowright X$.

**Exercise 33.2.** (Dye's Theorem) For the doubling map $T: x \mapsto 2x \pmod 1$ and the rotation $R_{1/3}: x \mapsto x + 1/3 \pmod 1$, both with Lebesgue measure: verify they are both free ergodic ${\mathbb Z}$-actions. By Dye's theorem, they are OE. Can you construct an explicit orbit equivalence (or show it cannot be constructive)?

**Exercise 33.3.** Compute the $\ell^2$-Betti numbers $\beta_n^{(2)}({\mathbb Z}^2)$ for $n = 0, 1, 2$. What does Gaboriau's theorem say about OE-invariants of ${\mathbb Z}^2$-actions?

**Exercise 33.4.** (Research) Popa's deformation-rigidity theory uses "malleable actions" — actions that can be continuously deformed through OE maps. Look up the definition of malleability and show that Bernoulli actions are malleable.

---

## Chapter Notes

The foundational papers: Dye (1959, 1963) in *Ann. Math.*; Ornstein-Weiss (1980) in *J. d'Analyse Math.*. The modern theory is surveyed in Furman's *A survey of measured group theory* (in *Geometry, Rigidity, and Group Actions*, 2011).

Gaboriau's $\ell^2$-Betti numbers: *Invariants $\ell^2$ de relations d'équivalence et de groupes* (Publ. Math. IHES, 2002). Popa's superrigidity: *Strong rigidity of II$_1$ factors arising from malleable actions of $w$-rigid groups* (Invent. Math., 2006).

The connection to von Neumann algebras: the orbit equivalence relation $\mathcal{R}$ determines the group measure space von Neumann algebra $L(\mathcal{R})$. Connes's paper *Classification of Injective Factors* (1976) classifies hyperfinite factors and connects to Dye's theorem.
