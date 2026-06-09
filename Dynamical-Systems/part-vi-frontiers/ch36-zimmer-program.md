# Chapter 36 — The Zimmer Program

> *Can a lattice in $SL(n, {\mathbb R})$ act on a compact manifold of dimension less than $n-1$? Zimmer conjectured: no. This connects the deepest parts of Lie group theory, ergodic theory, and differential geometry — and was largely resolved by Brown-Fisher-Hurtado in 2020.*

**Prerequisites:** Chapter 7 (ergodic theory, cocycles), Chapter 33 (orbit equivalence, property (T)), Chapter 14 (Hamiltonian systems, Lie groups). Some familiarity with Lie groups helpful.

---

## 36.1 Background: Lattices and Lie Groups

**Definition 36.1.1.** Let $G$ be a connected semisimple Lie group. A *lattice* $\Gamma \leq G$ is a discrete subgroup with $G/\Gamma$ of finite volume. Examples:
- $SL(n, {\mathbb Z}) \leq SL(n, {\mathbb R})$ (the standard arithmetic lattice)
- $\pi_1(M) \leq G$ for compact hyperbolic manifold $M$ (cocompact lattice)

**Theorem 36.1.2 (Margulis Superrigidity, 1974).** Let $G, H$ be semisimple Lie groups of real rank $\geq 2$ and $\Gamma \leq G$ a lattice. Every homomorphism $\phi: \Gamma \to H$ with Zariski-dense image extends to a Lie group homomorphism $\Phi: G \to H$.

**Interpretation:** Lattices in higher-rank groups are "rigid" — they cannot have unexpected representations. Their actions are all "algebraic" in origin.

---

## 36.2 The Zimmer Program

**Definition 36.2.1 (Zimmer, 1980s).** The *Zimmer program* investigates smooth actions of lattices $\Gamma$ in semisimple Lie groups on compact manifolds. The central question:

*If $\Gamma \leq SL(n, {\mathbb R})$ acts smoothly on a compact manifold $M$, what is the minimal dimension of $M$?*

**Zimmer's Conjecture (strong form).** If $\Gamma$ is an irreducible lattice in a semisimple Lie group $G$ of real rank $r$, and $\Gamma \curvearrowright M$ is a smooth volume-preserving action on a compact manifold $M$, then:
$$\dim M \geq r = \text{rank}_{\mathbb R}(G).$$

For $\Gamma = SL(n, {\mathbb Z})$ (rank $n-1$): any volume-preserving action on a compact manifold $M$ has $\dim M \geq n-1$.

---

## 36.3 Cocycle Superrigidity

**Definition 36.3.1.** For a measure-preserving action $\Gamma \curvearrowright (X, \mu)$, a *measurable cocycle* is a measurable map $\alpha: \Gamma \times X \to H$ (into a group $H$) satisfying:
$$\alpha(\gamma_1\gamma_2, x) = \alpha(\gamma_1, \gamma_2 \cdot x)\alpha(\gamma_2, x).$$

**Theorem 36.3.2 (Zimmer's Cocycle Superrigidity, 1980).** Let $\Gamma \leq G$ (semisimple, higher rank) be a lattice and $\Gamma \curvearrowright (X, \mu)$ an ergodic action. Every measurable cocycle $\alpha: \Gamma \times X \to GL(n, {\mathbb R})$ is cohomologous (a.e.) to a group homomorphism $\rho: \Gamma \to GL(n, {\mathbb R})$ twisted by a measurable map into the Zariski closure of the image.

**Consequence:** The derivative cocycle of a smooth volume-preserving action is constrained. If $\Gamma \curvearrowright M$, the derivative $D\gamma: TM \to TM$ defines a cocycle $\alpha(\gamma, x) = D_x\gamma \in GL(\dim M, {\mathbb R})$. Cocycle superrigidity says this derivative cocycle is "almost algebraic."

---

## 36.4 The Lyapunov Spectrum and Volume

**Theorem 36.4.1 (Zimmer's Inequality).** For a smooth volume-preserving action $\Gamma \curvearrowright M$ with $\Gamma \leq SL(n, {\mathbb R})$ a lattice, the Lyapunov exponents of the action satisfy constraints from the representation theory of $G = SL(n, {\mathbb R})$.

Specifically, the Lyapunov spectrum of the derivative cocycle must "come from" a representation of $G$, so the possible Lyapunov exponents are the weights of a $GL(\dim M, {\mathbb R})$-representation of $G$.

**The Dimension Bound:** The minimal faithful representation of $SL(n, {\mathbb R})$ has dimension $n-1$ (the standard representation on ${\mathbb R}^n$ restricted to the Lie algebra). If the derivative cocycle comes from a representation of $G$, then $\dim M \geq n - 1$.

---

## 36.5 The Brown-Fisher-Hurtado Resolution

**Theorem 36.5.1 (Brown-Fisher-Hurtado, 2020).** Let $n \geq 3$ and $\Gamma \leq SL(n, {\mathbb R})$ be a cocompact lattice (or $\Gamma = SL(n, {\mathbb Z})$). If $\Gamma \curvearrowright M$ is a $C^\infty$ volume-preserving action on a compact manifold $M$ with $\dim M < n - 1$, then the action factors through a finite group action.

**Corollary 36.5.2 (Zimmer's Conjecture for SL(n)).** $SL(n, {\mathbb Z})$ (and lattices in $SL(n, {\mathbb R})$ for $n \geq 3$) cannot act faithfully by $C^\infty$ volume-preserving diffeomorphisms on any compact manifold of dimension $< n - 1$.

**Key Tools in the Proof:**
1. *KAM theory* (Chapter 14): Local linearization of nearly-integrable systems
2. *Cocycle superrigidity* (Theorem 36.3.2): Derivative constraints
3. *Non-stationary normal forms*: Extending KAM to the non-commutative setting
4. *Harmonic analysis on homogeneous spaces*: Spectral estimates for $G/\Gamma$

**Remark 36.5.3.** The proof uses dynamical systems methods (KAM, Lyapunov exponents) in an essential way — it is not purely algebraic. The interplay between dynamics and group theory is the heart of the Zimmer program.

---

## 36.6 Connections to Geometric Group Theory

**Theorem 36.6.1 (Margulis Normal Subgroup Theorem).** If $\Gamma \leq G$ (semisimple, higher rank) is an irreducible lattice, every normal subgroup of $\Gamma$ is either finite or of finite index.

**Implication for Dynamics:** Ergodic actions of $\Gamma$ are classified by their *algebraic degree* — they must essentially come from algebraic actions of $G$ on algebraic varieties. The Zimmer program makes this precise in the differential geometry setting.

**Theorem 36.6.2 (Stuck-Zimmer, 1994).** Every faithful ergodic measure-preserving action of a higher-rank lattice with finite stabilizers is essentially free (has a.e. trivial stabilizer).

---

## Exercises

**Exercise 36.1.** Verify that $SL(2, {\mathbb Z})$ acts on ${\mathbb T}^2$ by the linear action $A \cdot x = Ax \pmod{{\mathbb Z}^2}$. Show this preserves Lebesgue measure. What is the dimension? Does this contradict Zimmer's conjecture (recall rank of $SL(2)$ is 1)?

**Exercise 36.2.** For $\Gamma = SL(3, {\mathbb Z})$ acting on ${\mathbb T}^3$ by the linear action: verify this is volume-preserving. Compute the Lyapunov exponents of the derivative cocycle. How many are there, and what are they?

**Exercise 36.3.** (Cocycle Superrigidity) For the action $SL(2, {\mathbb Z}) \curvearrowright {\mathbb T}^2$ and the derivative cocycle $\alpha(A, x) = A \in GL(2, {\mathbb R})$: verify that $\alpha$ is a constant cocycle (group homomorphism). This is consistent with Zimmer's cocycle superrigidity.

**Exercise 36.4.** Zimmer's conjecture is still open for non-cocompact lattices in general. Identify which cases are resolved by Brown-Fisher-Hurtado and which remain open.

---

## Chapter Notes

Zimmer's foundational papers: *Strong rigidity for ergodic actions of semisimple Lie groups* (Annals of Math., 1980) and *Ergodic Theory and Semisimple Groups* (Birkhäuser, 1984 — still the standard reference).

Margulis's superrigidity and normal subgroup theorems are in *Discrete Subgroups of Semisimple Lie Groups* (Springer, 1991). A readable account is in Witte Morris's *Introduction to Arithmetic Groups* (available free online).

Brown-Fisher-Hurtado: *Zimmer's conjecture: Subexponential growth, measure rigidity, and strong property (T)* (Annals of Math., 2022). Fisher's survey *Recent progress in the Zimmer program* (2020, arXiv) provides an accessible overview.
