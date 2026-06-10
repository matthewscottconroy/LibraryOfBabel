# The Compactness Theorem

> "The compactness theorem is the most powerful non-constructive tool in model theory. It says: if you cannot find a finite obstruction to satisfiability, then you cannot find any obstruction at all."
> — Model-theorist's saying

## A Surprising Analogy to Topology

The name "compactness" comes from an analogy with compact topological spaces. In topology, a space $X$ is *compact* if every open cover has a finite subcover. This means: if you need to cover $X$, you never really need infinitely many open sets — finitely many always suffice.

In logic, the compactness theorem says something analogous about satisfiability: if you have an infinite set of sentences $\Gamma$ that you want to satisfy simultaneously, you never need to "check" infinitely many constraints. If every *finite* subset of $\Gamma$ is satisfiable, then the entire $\Gamma$ is satisfiable.

This analogy is not merely poetic. There is a deep connection: the compactness theorem for propositional logic is equivalent to Tychonoff's theorem (that products of compact spaces are compact) applied to the space $\{0,1\}^{\text{Atoms}}$.

## The Theorem

**Compactness Theorem**: A set $\Gamma$ of first-order sentences is satisfiable if and only if every finite subset of $\Gamma$ is satisfiable.

The "only if" direction is trivial: if $\Gamma$ has a model $\mathcal{M}$, then every finite subset $\Gamma_0 \subseteq \Gamma$ is satisfied by $\mathcal{M}$ as well.

The "if" direction is deep: from the bare fact that no finite collection of sentences contradicts each other, we can construct an actual model satisfying everything simultaneously. This is remarkable because the model we construct might be radically different from what we had in mind — for instance, it might be infinite even when we were "thinking about" finite structures.

**Equivalent formulation**: If $\Gamma \models \varphi$ (every model of $\Gamma$ satisfies $\varphi$), then there is a finite $\Gamma_0 \subseteq \Gamma$ with $\Gamma_0 \models \varphi$.

In other words: logical consequence is "compact." Any consequence of an infinite theory is already a consequence of some finite part of that theory.

## Proof Sketch: Via the Completeness Theorem

The easiest proof path uses Gödel's completeness theorem (ch09/02):

1. If $\Gamma \models \varphi$, then by Completeness, $\Gamma \vdash \varphi$ (there is a formal proof)
2. Any formal proof is a finite object — it uses only finitely many premises from $\Gamma$
3. So $\Gamma_0 \vdash \varphi$ for some finite $\Gamma_0 \subseteq \Gamma$
4. By Soundness, $\Gamma_0 \models \varphi$

Equivalently, if $\Gamma$ is not satisfiable, then $\Gamma \models \bot$, so $\Gamma \vdash \bot$ by Completeness, and this proof uses only finitely many premises $\Gamma_0$, making $\Gamma_0$ already unsatisfiable.

## Application 1: Non-Standard Models of Arithmetic

This is the classic application that reveals how powerful (and strange) Compactness is.

Let $\text{PA}$ be the Peano axioms for natural numbers. Consider the theory:
$$T = \text{PA} \cup \{c \neq \bar{0},\; c \neq \bar{1},\; c \neq \bar{2},\; c \neq \bar{3},\; \ldots\}$$

where $c$ is a new constant symbol and $\bar{n}$ denotes the numeral for $n$ (the formal term $S(S(\ldots S(0)\ldots))$ with $n$ applications of $S$).

**Claim**: Every finite subset of $T$ is satisfiable.

*Proof*: A finite subset uses only finitely many of the axioms $c \neq \bar{n}$. Say the largest $n$ appearing is $N$. Then the standard natural numbers $\mathbb{N}$, with $c$ interpreted as $N+1$, satisfies all the Peano axioms and all the finitely many sentences $c \neq \bar{0}, \ldots, c \neq \bar{N}$.

**By Compactness**: $T$ itself is satisfiable. So there is a model $\mathcal{M}$ of $\text{PA}$ containing an element — call it $c^{\mathcal{M}}$ — that is not equal to $0, 1, 2, 3, \ldots$ in the model. This element is "infinite" — larger than every standard natural number, yet living in a model that satisfies all the Peano axioms.

This is a **non-standard model of arithmetic**. Its elements include:
- The "standard" naturals: $0, 1, 2, 3, \ldots$
- "Non-standard" elements larger than all standard naturals, arranged in $\mathbb{Z}$-like blocks without a maximum

Non-standard models satisfy every theorem provable from PA. They provide a universe in which the axioms are satisfied yet the "numbers" are not what we naively imagined. This is both profound (it shows PA does not "pin down" the natural numbers uniquely up to isomorphism) and practical (non-standard analysis uses analogous models of the reals to give rigorous foundations to infinitesimals).

## Application 2: First-Order Logic Cannot Express Finiteness

**Theorem**: There is no first-order sentence $\varphi$ such that $\mathcal{M} \models \varphi$ if and only if the domain of $\mathcal{M}$ is finite.

**Proof by Compactness**: Suppose such $\varphi$ existed. Consider:
$$\Gamma = \{\varphi\} \cup \{\exists x_1 \exists x_2 \cdots \exists x_n \bigwedge_{i \neq j} x_i \neq x_j \mid n \in \mathbb{N}\}$$

The sentence $\exists x_1 \ldots \exists x_n \bigwedge_{i \neq j} x_i \neq x_j$ says "there are at least $n$ distinct elements." For each $n$, this is satisfiable — any domain with $\geq n$ elements works, and finite domains exist of every finite size. So every finite subset of $\Gamma$ is satisfiable (by a large enough finite domain). By Compactness, $\Gamma$ has a model. But this model satisfies $\varphi$ (so it's "finite") and also has more than $n$ distinct elements for every $n$ (so it's infinite). Contradiction. $\square$

This is a striking result: FOL, despite its expressiveness, cannot distinguish finite from infinite structures. This is a fundamental limitation — one that motivates the study of **infinitary logics** ($\mathcal{L}_{\omega_1, \omega}$) and **second-order logic**, both of which can express finiteness.

## Application 3: Graph Coloring

The compactness theorem has concrete combinatorial applications.

**Theorem** (De Bruijn-Erdős): A graph $G$ is $k$-colorable (vertices can be colored with $k$ colors so no two adjacent vertices share a color) if and only if every finite subgraph of $G$ is $k$-colorable.

**Proof by Compactness**: Model graph coloring in FOL: for each vertex $v$, introduce $k$ propositional variables $C_1(v), \ldots, C_k(v)$ ("vertex $v$ gets color $i$"). The axioms say: each vertex gets exactly one color, and adjacent vertices get different colors.

Every finite subgraph being $k$-colorable means every finite subset of this theory is satisfiable. By Compactness, the whole theory is satisfiable, giving a $k$-coloring of $G$. $\square$

This is remarkable: to show an infinite graph is $k$-colorable, it suffices to check all finite subgraphs. The Compactness theorem converts a global (infinite) problem into a family of local (finite) problems.

## Non-Standard Analysis: A Deep Application

Abraham Robinson (1961) used non-standard models of the real numbers to create **non-standard analysis** — a rigorous foundation for the infinitesimals that Leibniz and Newton used informally in calculus.

The non-standard reals ${}^*\mathbb{R}$ are a model of the same first-order sentences as $\mathbb{R}$, but containing:
- **Infinitesimals**: elements $\varepsilon$ with $0 < \varepsilon < r$ for every positive real $r$
- **Infinite elements**: elements $H$ with $H > r$ for every real $r$

The **Transfer Principle** (a consequence of how ${}^*\mathbb{R}$ is built using an ultrafilter — related to Compactness): any first-order statement true in $\mathbb{R}$ is true in ${}^*\mathbb{R}$, and vice versa.

This lets us do calculus with genuine infinitesimals:
$$f'(x) = \text{st}\left(\frac{f(x + \varepsilon) - f(x)}{\varepsilon}\right)$$

where $\text{st}$ is the "standard part" function (rounding an infinitely-close-to-standard element to the nearest real), and $\varepsilon$ is a genuine infinitesimal. This is not merely formal — it is rigorous mathematics, proven consistent by the model-theoretic construction.

## The Löwenheim-Skolem Theorem

Closely related to Compactness is the **Löwenheim-Skolem theorem**:

**Downward**: If a countable first-order theory $T$ has an infinite model, it has a countable model.

**Upward**: If a first-order theory $T$ has an infinite model, it has models of every infinite cardinality.

Together, these say: first-order logic cannot control cardinality. If you have *any* infinite model, you can "blow it up" to any size or "shrink it down" to countable. The "Skolem paradox" is the striking instance: ZFC has a countable model (by Downward Löwenheim-Skolem), even though ZFC proves that uncountable sets exist. How? The model has an uncountable set — but "uncountable" *in the model* means there is no bijection in the model between the set and $\omega$; such a bijection might exist *outside* the model.

## Lean 4 Sketch

Compactness itself is not directly stated in Lean's Mathlib (it requires model-theoretic machinery), but the propositional case is accessible:

```lean
import Mathlib.Logic.Basic

-- Propositional compactness: a set of propositions is satisfiable
-- iff every finite subset is satisfiable.
-- In Lean, this connects to ultrafilter compactness.

-- The key idea: completeness gives compactness
-- If Γ ⊨ φ, then Γ ⊢ φ (completeness)
-- Any proof uses finitely many premises
-- So Γ₀ ⊢ φ for finite Γ₀ ⊆ Γ
-- Therefore Γ₀ ⊨ φ (soundness)
```

## Exercises
See [problems/ch09_model_theory/03_completeness_applications.md](../../../problems/ch09_model_theory/03_completeness_applications.md)
