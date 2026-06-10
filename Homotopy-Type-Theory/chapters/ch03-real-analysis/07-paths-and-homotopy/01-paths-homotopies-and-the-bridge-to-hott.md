# 7.1 Paths, Homotopies, and the Bridge to HoTT

## A Different Way to Think About Continuity

Throughout this chapter, we've been developing the theory of metric spaces, continuity, and convergence. Now we take a step back and look at what this machinery is actually *about* from a higher perspective.

The central objects in topology and homotopy theory are not points — they are *paths between points*, *homotopies between paths*, *homotopies between homotopies*, and so on. This infinite hierarchy of higher-dimensional structure is what makes homotopy theory rich, and it's exactly the structure that Homotopy Type Theory axiomatizes.

## Paths

**Definition.** A *path* in a topological space $X$ is a continuous map $\gamma : [0, 1] \to X$. The *start* of $\gamma$ is $\gamma(0)$ and the *end* is $\gamma(1)$.

The interval $[0, 1]$ plays the role of the "parameter space" for paths. Paths are precisely the continuous deformations that connect two points.

**Examples:**
- In $\mathbb{R}$: any path from $a$ to $b$ is a continuous function $\gamma : [0, 1] \to \mathbb{R}$ with $\gamma(0) = a$ and $\gamma(1) = b$. The simplest is the straight-line path $\gamma(t) = a + t(b - a)$.
- In $\mathbb{R}^2$: paths are curves connecting two points. There are many different paths between any two points — you can go straight, in an arc, or in a complicated loop.
- In $S^1$ (the unit circle): there are paths that go "once around" the circle, and paths that go "twice around," and paths that go around in the opposite direction. These are qualitatively different paths, and homotopy theory measures this difference.

## Constant Paths and Path Concatenation

Three operations on paths form the foundation of homotopy theory:

**Constant path.** For each $x \in X$, the constant path at $x$ is $c_x : [0,1] \to X$ defined by $c_x(t) = x$ for all $t$.

**Concatenation.** If $\gamma$ is a path from $x$ to $y$ and $\delta$ is a path from $y$ to $z$, their concatenation $\gamma \cdot \delta$ is the path from $x$ to $z$ defined by:
$$(\gamma \cdot \delta)(t) = \begin{cases} \gamma(2t) & 0 \leq t \leq 1/2 \\ \delta(2t - 1) & 1/2 \leq t \leq 1 \end{cases}$$

This is continuous (by the gluing lemma: two continuous functions on closed sets agree at the overlap, so their combination is continuous).

**Reversal.** If $\gamma$ is a path from $x$ to $y$, its reversal is $\bar\gamma(t) = \gamma(1 - t)$, a path from $y$ to $x$.

## Homotopies of Paths

Two paths $\gamma, \delta : [0, 1] \to X$ from $x$ to $y$ are *homotopic* (with endpoints fixed) if one can be continuously deformed into the other while keeping the endpoints fixed.

**Definition.** A *homotopy* (with fixed endpoints) from $\gamma$ to $\delta$ is a continuous function $H : [0, 1] \times [0, 1] \to X$ satisfying:
- $H(t, 0) = \gamma(t)$ for all $t$ (the "start" of the homotopy is $\gamma$)
- $H(t, 1) = \delta(t)$ for all $t$ (the "end" of the homotopy is $\delta$)
- $H(0, s) = x$ for all $s$ (the start point is fixed)
- $H(1, s) = y$ for all $s$ (the end point is fixed)

Think of $H(t, s)$ as a family of paths $\gamma_s(t) = H(t, s)$ parameterized by $s \in [0, 1]$. At $s = 0$, $\gamma_s = \gamma$; at $s = 1$, $\gamma_s = \delta$. As $s$ varies from 0 to 1, the path continuously deforms from $\gamma$ to $\delta$.

**Notation.** We write $\gamma \simeq \delta$ for "homotopic with endpoints fixed."

Homotopy is an equivalence relation on paths with a fixed start and end point. The equivalence classes are *homotopy classes of paths*.

## The Fundamental Group

Fix a basepoint $x_0 \in X$. A *loop based at $x_0$* is a path $\gamma : [0,1] \to X$ with $\gamma(0) = \gamma(1) = x_0$.

**Definition.** The *fundamental group* $\pi_1(X, x_0)$ is the set of homotopy classes of loops based at $x_0$, with group operation given by concatenation.

The group axioms:
- *Identity:* The constant loop $c_{x_0}$ (up to homotopy).
- *Inverse:* The reverse of a loop $\gamma$ is a loop $\bar\gamma$.
- *Associativity:* Up to homotopy, $(\alpha \cdot \beta) \cdot \gamma \simeq \alpha \cdot (\beta \cdot \gamma)$.

The fundamental group measures "how many holes" a space has in dimension 1.

**Examples:**
- $\pi_1(\mathbb{R}^n, x_0) = 0$ (trivial group): any loop in $\mathbb{R}^n$ can be contracted to the constant loop.
- $\pi_1(S^1, x_0) = \mathbb{Z}$: loops on the circle are classified by their winding number (how many times they go around, with sign for direction).
- $\pi_1(S^2, x_0) = 0$: the 2-sphere is simply connected.
- $\pi_1(\text{Torus}, x_0) = \mathbb{Z} \times \mathbb{Z}$: loops on the torus can wind independently in two directions.
- $\pi_1(\text{Figure eight}, x_0) = F_2$: the free group on two generators (as in Chapter 2!).

## Higher Homotopy Groups

The fundamental group only captures 1-dimensional topology. There are higher homotopy groups $\pi_n(X, x_0)$ for $n \geq 2$.

**Definition.** $\pi_n(X, x_0)$ is the set of homotopy classes of continuous maps $f : [0,1]^n \to X$ that send the boundary $\partial [0,1]^n$ entirely to $x_0$.

For $n = 1$: this is a loop, and $\pi_1$ is the fundamental group.
For $n = 2$: this is a "2-sphere of paths," and $\pi_2$ measures 2-dimensional holes.

Higher homotopy groups are abelian for $n \geq 2$ (by the Eckmann-Hilton argument). Computing them is notoriously difficult:
- $\pi_3(S^2) = \mathbb{Z}$ (the Hopf fibration, discovered 1931).
- $\pi_n(S^n) = \mathbb{Z}$ for all $n \geq 1$.
- Most $\pi_k(S^n)$ for $k > n$ are non-trivial and computing them is an active research area.

## The Connection to Type Theory

Here is the fundamental insight of Homotopy Type Theory:

**Types are spaces. Terms are points. Identity proofs are paths.**

More precisely: in HoTT, for a type $A$ and terms $a, b : A$, the *identity type* $\text{Id}_A(a, b)$ (the type of proofs that $a = b$) is interpreted as the *path space* from $a$ to $b$ in the "space" $A$.

Under this interpretation:
- A proof of $a = b$ is a path from $a$ to $b$.
- A proof of $a = b = c$ (i.e., a proof of $(a = b) \times (b = c)$) gives two paths, and concatenation is path composition.
- $\text{refl}_a : \text{Id}_A(a, a)$ (reflexivity) is the constant path at $a$.
- If $p : \text{Id}_A(a, b)$, then $p^{-1} : \text{Id}_A(b, a)$ is the reversed path.
- Transitivity of equality corresponds to path concatenation.

And crucially:

- **A proof that two proofs are equal** (a term of $\text{Id}_{\text{Id}_A(a,b)}(p, q)$) is a *homotopy* between two paths.
- **Higher equalities** are higher homotopies.

This correspondence is not just an analogy — it's a theorem. The groupoid laws for path composition (associativity and identity up to homotopy) correspond exactly to the structure of the identity type in Martin-Löf Type Theory.

## The Univalence Axiom as a Higher Homotopy

The Univalence Axiom says: for types $A, B : \mathcal{U}$ (in the universe of types), the identity type $\text{Id}_\mathcal{U}(A, B)$ is equivalent to the type $A \simeq B$ of type equivalences.

In the topological picture: the universe $\mathcal{U}$ is a "space of spaces," and a path between two types in this space corresponds to an equivalence between the types. Isomorphic mathematical structures are connected by a path in the space of types.

This is the Univalence Axiom: equivalences are paths in the type universe.

It makes the mathematician's practice of "treating isomorphic structures as the same" into a literal truth: isomorphic types *are* equal (connected by a path), because the path space of $\mathcal{U}$ at types $A$ and $B$ is exactly the type of equivalences between $A$ and $B$.

## Why Analysis Is the Right Starting Point

Real analysis gave us:
- The notion of *path* as a continuous map from $[0,1]$.
- The notion of *homotopy* as a continuous deformation.
- The *fundamental group* as the first example of a homotopy invariant.
- The *identity problem*: two constructions of $\mathbb{R}$ that are isomorphic but not equal in ZFC.

These are exactly the concepts that HoTT resolves and formalizes:
- Paths in a type are identity proofs.
- Homotopies between paths are proofs that proofs are equal.
- The fundamental group $\pi_1(S^1) = \mathbb{Z}$ is a theorem about the circle type in HoTT, proved using higher inductive types.
- The Univalence Axiom makes isomorphic types literally equal.

Real analysis is where the intuition lives. Homotopy type theory is where the intuition becomes axioms.

## What Comes Next

The remaining chapters of this curriculum build toward this picture systematically:
- **Chapter 7 (Intuitionistic Logic):** The logic underlying type theory, where "proofs are evidence."
- **Chapter 8 (Curry-Howard):** Proofs are programs; propositions are types.
- **Chapter 10 (Dependent Types):** The type-theoretic machinery that supports path types.
- **Chapter 11 (MLTT):** Martin-Löf Type Theory, the foundation of Lean and Agda.
- **Chapter 16 (Identity Types):** Paths in type theory, formally.
- **Chapter 18 (Univalence):** Isomorphism as equality.
- **Chapter 22 (Higher Inductive Types):** Type-theoretic analogs of CW complexes; proves $\pi_1(S^1) = \mathbb{Z}$.

The path (pun intended) from this chapter to those is clear. The analysis we've developed here is the motivating example — the warm-up before the formalism. Keep it in mind as we build the machinery.
