# 5.1 The Categorical Semantics of Identity Types

## The Challenge

We've seen that LCCC semantics handles Π and Σ types well, but the identity type is different. In an LCCC, the "obvious" model of $a =_A b$ would be the diagonal:

$$\Delta_A : A \to A \times A, \quad a \mapsto (a, a)$$

with the identity type $\mathsf{Id}_A$ being $A$ itself (with the diagonal as the projection to $A \times A$). But this gives a *trivial* identity type: the only inhabitant of $a =_A b$ is a proof that $a = b$, and there's at most one such proof. This makes UIP automatically true — not what we want for HoTT.

For the *intensional* identity type (where UIP can fail), we need models where the identity type $a =_A b$ can have multiple distinct elements (multiple "paths" from $a$ to $b$).

## Path Objects in Model Categories

The key categorical concept: *path objects*.

**Definition.** In a category $\mathcal{C}$, a *path object* for $A$ is a factorization:

$$A \xrightarrow{r} \mathsf{Path}(A) \xrightarrow{(s, t)} A \times A$$

where $r$ is a weak equivalence and $(s, t)$ is a fibration (in an appropriate model category sense).

The path object $\mathsf{Path}(A)$ represents "paths in $A$":
- $r : A \to \mathsf{Path}(A)$ sends each element to its constant path (reflexivity)
- $s, t : \mathsf{Path}(A) \to A$ extract the start and end of each path
- $(s, t)^{-1}(a, b) = \mathsf{Id}_A(a, b)$: the fiber over $(a, b)$ is the type of paths from $a$ to $b$

**In Kan simplicial sets:** $\mathsf{Path}(A)$ is the *simplicial mapping space* $A^{\Delta^1}$ — simplicial maps from the 1-simplex (interval) to $A$. An element is a 1-simplex in $A$, i.e., a path.

**In topological spaces:** $\mathsf{Path}(A)$ is the *path space* $A^{[0,1]}$ — continuous maps from $[0,1]$ to $A$. The endpoints are the source $s$ and target $t$.

**The J rule as a lifting property:** The J rule says: to prove a property of all paths starting at $a$, prove it for the constant path. Categorically, this is a *lifting property*:

$$\begin{array}{ccc} A & \xrightarrow{r} & \mathsf{Path}(A) \\ \downarrow & \nearrow & \downarrow (s, t) \\ A \times A & & A \times A \end{array}$$

The existence of the lift $A \to \mathsf{Path}(A)$ such that $(s, t) \circ r = (s, t) \circ \Delta_A$ (over the diagonal) is exactly the J rule. Any property of paths that holds for constant paths can be lifted to all paths.

## The Awodey-Warren Theorem

**Theorem (Awodey-Warren, 2009).** Every Quillen model category in which:
- Every object is fibrant and cofibrant, or
- The category has a suitable "universe" object

gives a model of Martin-Löf Type Theory with identity types.

*Key construction:* The identity type of $A$ is the path object $\mathsf{Path}(A)$. The J rule is the lifting property of the path object.

**The groupoid model (Hofmann-Streicher, 1994).** The simplest example of a model where UIP fails:
- Types: (small) groupoids
- Terms of type $A$: objects of the groupoid $A$
- Identity type $a =_A b$: the set of morphisms $\mathsf{Hom}_A(a, b)$
- Reflexivity: the identity morphism $\mathsf{id}_a$
- Concatenation: morphism composition
- Inversion: inverse morphisms

In the groupoid model, $a = b$ can have multiple proofs: any morphism from $a$ to $b$ in the groupoid. UIP fails because hom-sets can have more than one element.

**Example.** The circle $S^1$, viewed as the groupoid $\mathbf{B}\mathbb{Z}$ (one object, automorphisms = $\mathbb{Z}$):
- The unique object: $\mathsf{base}$
- $\mathsf{Hom}(\mathsf{base}, \mathsf{base}) = \mathbb{Z}$: all integers
- $\mathsf{base} = \mathsf{base}$ has elements $0, 1, -1, 2, -2, \ldots$

So UIP says $0 = 1$ (as elements of $\mathsf{base} = \mathsf{base}$), but $0 \neq 1$ in $\mathbb{Z}$. UIP fails. This is exactly $\pi_1(S^1) = \mathbb{Z}$.

## Voevodsky's Simplicial Set Model

**Setup.** The model lives in the category $\mathsf{sSet} = [\Delta^{op}, \mathbf{Set}]$ of simplicial sets, where:
- $\Delta$ is the simplex category: objects $[n] = \{0, 1, \ldots, n\}$, morphisms are order-preserving maps
- Simplicial sets $X : \Delta^{op} \to \mathbf{Set}$: sets $X_n$ (the $n$-simplices), with face maps $d_i : X_n \to X_{n-1}$ and degeneracy maps $s_i : X_n \to X_{n+1}$

**Kan complexes.** A simplicial set $X$ is a *Kan complex* if it satisfies the *Kan extension condition*: every horn $\Lambda^n_k \to X$ (all faces of an $n$-simplex except the $k$-th) can be extended to a full $n$-simplex $\Delta^n \to X$.

This condition generalizes the horn-filling property of topological spaces: in a topological space, every horn can be filled because the space is a CW complex (or just a continuous space). Kan complexes are the simplicial analogs of topological spaces.

**Types as Kan complexes.** In Voevodsky's model:
- Types: Kan complexes (or more precisely, fibrant objects in the Kan-Quillen model structure)
- Closed type $A$: a Kan complex $\llbracket A \rrbracket$
- Open type in context $\Gamma$: a Kan fibration $\llbracket A \rrbracket \to \llbracket \Gamma \rrbracket$
- Term: a section of the fibration

**Identity types as path spaces.** The identity type $a = b$ is:

$$\llbracket a =_A b \rrbracket = \text{the homotopy fiber of } \Delta_A \text{ over } (a, b)$$

More concretely: $\llbracket A \rrbracket^{\Delta^1}$ (simplicial maps from the standard 1-simplex $\Delta^1 = \{0 \to 1\}$ to $\llbracket A \rrbracket$). This is the *simplicial path space*.

For $a, b : \mathbf{1} \to \llbracket A \rrbracket$ (points of $A$), the fiber of the evaluation map $\llbracket A \rrbracket^{\Delta^1} \to \llbracket A \rrbracket \times \llbracket A \rrbracket$ over $(a, b)$ is exactly the type of "1-simplices in $A$ from $a$ to $b$" — paths.

**Why UIP fails.** The fundamental group $\pi_1(\llbracket S^1 \rrbracket) = \mathbb{Z}$ (the simplicial circle is the simplicial analog of the topological circle, and its fundamental group is $\mathbb{Z}$). The identity type $\mathsf{base} = \mathsf{base}$ is the loop space $\Omega S^1$, which has $\pi_0(\Omega S^1) = \mathbb{Z}$ (one component per integer). So UIP fails: there are infinitely many distinct "proofs" of $\mathsf{base} = \mathsf{base}$.

## The Univalence Axiom in the Simplicial Model

The central result:

**Theorem (Voevodsky, 2006-2009).** In the simplicial set model, the Univalence Axiom holds as a theorem.

**The Univalence Axiom:** For types $A, B : \mathsf{Type}$:

$$\mathsf{ua} : (A \simeq B) \simeq (A = B)$$

An equivalence between types is the same as an equality (path) between them in the universe.

**Why it holds in the simplicial model:** The universe $\hat{U}$ of Kan complexes is itself a Kan complex (with Kan fibrations as morphisms). The path space $\hat{U}^{\Delta^1}$ (paths in the universe) consists of *weak equivalences* between Kan complexes, which are exactly the *homotopy equivalences*. And homotopy equivalences are the same as equivalences of types.

So: paths in the universe = homotopy equivalences of types = equivalences of types. That's Univalence.

## The Full Model

Putting it together, Voevodsky's simplicial set model provides:

1. **MLTT:** All typing rules hold by the LCCC structure of Kan simplicial sets
2. **Identity types:** Modeled by simplicial path spaces; J rule holds by the Kan horn-filling
3. **Universes:** A cumulative hierarchy of universe objects $U_0 \hookrightarrow U_1 \hookrightarrow \cdots$
4. **Univalence:** Path spaces in the universe = equivalences (theorem)
5. **UIP fails:** Because $\pi_1(S^1) = \mathbb{Z} \neq \mathbf{1}$ (as a Kan complex)
6. **Consistency:** The simplicial set model is consistent (relative to ZFC + large cardinals), so HoTT + Univalence is consistent

**HITs in the simplicial model.** Higher Inductive Types correspond to "fibrant replacement" constructions in the model. The circle $S^1$ is the simplicial circle; the suspension $\Sigma A$ is the simplicial suspension; pushouts are homotopy pushouts.

## Why This Matters

The simplicial set model is the foundation of HoTT's legitimacy:

1. **Consistency:** HoTT is not just "formally consistent because we haven't found a contradiction yet" — it has a *concrete model* in which all axioms hold. This is as good as it gets in mathematics (Gödel's theorem prevents an absolute consistency proof, but relative consistency to ZFC is standard).

2. **Independence results:** The model shows which theorems of set theory fail in HoTT (like UIP for circle) and which hold (like the theorem that $\pi_1(S^1) = \mathbb{Z}$, which is the reason we built S¹ as a HIT).

3. **Guidance for new axioms:** The simplicial model suggests that HoTT axioms should be "homotopy-invariant" — all axioms that hold in the model should be preserved by homotopy equivalences. Univalence ensures this.

4. **Connection to existing mathematics:** Simplicial sets are the standard tool in algebraic topology, algebraic geometry (via étale homotopy theory), and category theory (via $\infty$-categories). HoTT's consistency proof is a theorem in established mathematics, connecting the new foundations to classical ones.

## Summary

| Model | Identity Type | UIP | Univalence | Notes |
|---|---|---|---|---|
| $\mathbf{Set}$ | Diagonal (trivial) | Yes | No | Set theory model |
| Groupoids | Hom-sets | No | Holds for groupoids | First non-trivial model |
| Kan simplicial sets | Simplicial path space | No | Yes (theorem) | Foundation of HoTT |
| Cubical sets | Cubical path space | No | Yes (theorem) | Cubical type theory model |

The simplicial set model is the proof that HoTT works — not just formally, but mathematically. Types are spaces, paths are identifications, and the universe satisfies univalence. Everything is consistent, and the theory is powerful enough to formalize all of mathematics.
