# 5.1 ∞-Groupoids: Definitions and Connection to HoTT

## The Concept

An ∞-groupoid is an ∞-category in which every morphism at every level is invertible. It's the most general notion of "a space with points, paths between points, paths between paths, ...," where at every level you can go backwards.

We've already seen the intuition:
- A 0-groupoid is just a set (objects, no non-trivial morphisms)
- A 1-groupoid is a groupoid (objects and invertible morphisms)
- A 2-groupoid has objects, invertible 1-morphisms, and invertible 2-morphisms
- An ∞-groupoid has morphisms at every dimension, all invertible

The challenge is making this precise. As we'll see, there are multiple competing definitions, with different trade-offs between simplicity and generality.

## Kan Complexes: The Simplicial Definition

The cleanest definition comes from simplicial sets.

**Definition 5.1 (Kan Complex).** A simplicial set $X$ is a *Kan complex* if for every $n \geq 1$ and every $0 \leq k \leq n$, every horn $\Lambda^n_k \to X$ extends to a simplex $\Delta^n \to X$:

$$\begin{array}{ccc} \Lambda^n_k & \to & X \\ \downarrow & \nearrow & \\ \Delta^n & & \end{array}$$

In words: all horns fill — both inner horns (composition) and outer horns (inverses).

**Why this defines ∞-groupoids:**
- Inner horn filling ($0 < k < n$): all morphisms compose
- Outer horn $\Lambda^n_0$ filling: every morphism has a "right inverse" (in a suitable sense)
- Outer horn $\Lambda^n_n$ filling: every morphism has a "left inverse"

Together, these give all morphisms at all dimensions, with composition, inverses, and all the higher coherence you could want.

**Examples:**
- The singular complex $\mathsf{Sing}(X)$ of any topological space $X$ is a Kan complex
- The nerve $N(\mathcal{G})$ of a groupoid $\mathcal{G}$ is a Kan complex (but fills all horns uniquely)
- The nerve $N(\mathcal{C})$ of a non-groupoid category is *not* a Kan complex in general (outer horns don't fill)
- The Eilenberg-MacLane spaces $K(G,n)$ are Kan complexes

## Globular ∞-Groupoids: The Algebraic Definition

Grothendieck's original vision used *globular sets* — a more explicit algebraic definition.

**Definition 5.2 (Globular Set).** A *globular set* $G$ consists of:
- Sets $G_0, G_1, G_2, \ldots$ (objects, 1-cells, 2-cells, ...)
- Source and target maps $s, t : G_{n+1} \to G_n$ satisfying the globularity conditions:
  - $s \circ s = s \circ t$ and $t \circ s = t \circ t$ (sources and targets are compatible)

Intuitively: $G_0$ is objects, $G_1$ is morphisms (with source and target in $G_0$), $G_2$ is 2-morphisms between parallel 1-morphisms, etc.

A *strict ∞-groupoid* is a globular set with:
- Composition operations at each level
- Inverses at each level
- Identity cells at each level
- All laws holding strictly (on the nose, as equalities)

**The problem with strict ∞-groupoids.** As mentioned, strict ∞-groupoids are too rigid. They only model "linear" homotopy types — those that are products of Eilenberg-MacLane spaces. They can't model the circle $S^1$ properly, or most other interesting homotopy types.

**Theorem 5.3 (Simpson, 1998).** Not every homotopy type is modeled by a strict ∞-groupoid.

This is a fundamental theorem. It says: strictness and ∞-groupoids are incompatible for general homotopy theory. You must weaken the laws.

**Weak ∞-groupoids.** This requires laws to hold up to coherent cells at the next level. This is significantly harder to define. The combinatorics of "all composition laws hold up to all higher coherences, consistently" become complex.

There have been several proposed definitions:
- **Batanin's globular operads (1998):** Using globular sets and operads parametrizing composition laws
- **Leinster's globular operads:** A simplification of Batanin's definition
- **Trimble's definition:** Using operads and homotopy groups
- **Grothendieck's original (unpublished):** Using globular sets with specific horn-filling conditions

These definitions are not all equivalent, and their relationship to each other and to Kan complexes is an active research topic.

## Strict vs. Weak: The Key Lesson

The failure of strict ∞-groupoids to model all homotopy types is philosophically important. It says:

*The higher coherences of homotopy theory are genuinely complex and cannot be strictified.*

This is related to results in low-dimensional topology: you can strictify 2-categories but not 3-categories (in general). The lower the dimension, the more you can strictify; but at the limit (∞), nothing can be strictified.

This has implications for HoTT: the ∞-groupoid structure of types (via identity types) is genuinely weak. The groupoid laws hold propositionally (up to identity proofs), not definitionally (on the nose). Trying to make them definitional would be too rigid and would lose generality.

The right response (and what Voevodsky chose) is to embrace the weakness and use Kan complexes (or their cubical analogs) as the model.

## Types in HoTT are ∞-Groupoids

Let's make the connection between types and ∞-groupoids explicit.

**Theorem 5.4 (Informal).** Every type $A$ in HoTT is an ∞-groupoid, where:
- 0-cells: terms $a : A$
- 1-cells: identity proofs $p : a =_A b$
- 2-cells: identity proofs $H : p =_{a=b} q$ (homotopies between paths)
- $n$-cells: elements of the $n$-fold iterated identity type

The ∞-groupoid structure:
- **Composition:** Path concatenation at each level (defined by J)
- **Identities:** Reflexivity at each level
- **Inverses:** Path inversion at each level (defined by J)
- **Coherences:** Groupoid laws proved by J-induction; higher coherences by iterated J-induction

The composition, identities, and inverses are *weak* in the sense that the laws hold propositionally (there exist proof terms for them) but not definitionally (they're not judgmentally equal). This is exactly the weak ∞-groupoid structure.

**Caveat.** Making this precise — that types are ∞-groupoids in a technically rigorous sense — requires some care. The most rigorous statement is:
- The type $A$ gives a Kan complex $\llbracket A \rrbracket$ in the simplicial set model
- In the simplicial set model, Kan complexes are ∞-groupoids
- Therefore types are ∞-groupoids (in the model)

Inside the type theory itself, we can say: types satisfy all the laws of ∞-groupoids (provably), but the formal notion of "∞-groupoid" itself would require a substantial definition.

## The Coherence Tower

Let's trace out the first few levels of coherences that types satisfy.

**Level 0 (Groupoid laws):**
- Associativity: $(p \cdot q) \cdot r =_{a=d} p \cdot (q \cdot r)$
- Left unit: $\mathsf{refl} \cdot p =_{a=b} p$
- Right unit: $p \cdot \mathsf{refl} =_{a=b} p$
- Left inverse: $p^{-1} \cdot p =_{a=a} \mathsf{refl}$
- Right inverse: $p \cdot p^{-1} =_{b=b} \mathsf{refl}$

**Level 1 (Coherences between groupoid laws):**
- The Eckmann-Hilton argument (for $\Omega^2 X = \pi_2(X)$-level): 2-dimensional composition commutes in two dimensions, and commutativity follows
- Pentagon identity for associativity: the two ways to reparenthesize $(p \cdot q) \cdot (r \cdot s)$ are equal
- The various "Mac Lane coherence" diagrams

**Level 2 (Coherences between coherences):**
- These start getting complex. The key point is that they all hold (by J-induction or by abstract arguments), but spelling them out explicitly is tedious.

**The takeaway:** Types carry an enormous amount of coherent higher structure, all derivable from the J rule. This is the mathematical content of "types are ∞-groupoids."

## The Eckmann-Hilton Argument

One beautiful consequence of the ∞-groupoid structure: *higher homotopy groups are abelian*.

Consider a type $A$ with a point $a : A$. The loop space is $\Omega A = (a =_A a)$. The double loop space is $\Omega^2 A = (a =_A a) =_{\Omega A}$ (loops in the loop space).

$\Omega^2 A$ has two composition operations:
- Horizontal: $p \cdot q$ (concatenate loops in $\Omega A$)
- Vertical: compose loops as 2-paths

The **Eckmann-Hilton argument** shows: these two operations are equal to each other, and both equal the *other one reversed*, so they must be commutative!

More precisely: if a set has two binary operations $\star$ and $\circ$ that agree on units and satisfy the interchange law $(a \star b) \circ (c \star d) = (a \circ c) \star (b \circ d)$, then both operations are equal and commutative.

$\Omega^2 A$ satisfies this: the two ways to compose 2-loops (horizontally and vertically) satisfy the interchange law (from the interchange law for 2-categories). So both composition operations agree and are commutative.

**Corollary:** $\pi_2(A)$ (the second homotopy group of any type $A$) is abelian. More generally, $\pi_n(A)$ is abelian for all $n \geq 2$.

This is a classical result in algebraic topology, and it falls out of the ∞-groupoid structure of types in HoTT. Beautiful.

## h-Levels and the Postnikov Tower

The ∞-groupoid structure of types is most interesting when it's non-trivial. But sometimes it is trivial — and that's interesting too.

**Definition 5.5 (h-Levels).**
- A type $A$ is **contractible** (h-level $-2$) if $\sum_{a:A} \prod_{b:A} (a = b)$ — it has a unique element up to paths.
- A type $A$ is a **proposition** (h-level $-1$) if $\prod_{a,b:A} (a = b)$ — any two elements are equal.
- A type $A$ is a **set** (h-level 0) if $\prod_{a,b:A} \prod_{p,q:a=b} (p = q)$ — the identity types are propositions.
- A type $A$ is a **1-type** (h-level 1) if identity types are sets.
- A type $A$ is an **n-type** (h-level $n$) if identity types are $(n-1)$-types.

The h-levels measure "how much non-trivial higher structure" a type has.

**Connection to homotopy theory:**
- Contractible = $\pi_n = 0$ for all $n$ (contractible spaces)
- Proposition = $\pi_n = 0$ for all $n$ and $|\pi_0| \leq 1$ (empty or contractible)
- Set = $\pi_n = 0$ for all $n \geq 1$ (discrete space)
- 1-type = $\pi_n = 0$ for $n \geq 2$ ($K(\pi,1)$ spaces, or 1-truncated homotopy types)
- $n$-type = $\pi_k = 0$ for $k > n$ ($n$-truncated homotopy types)

An ∞-type (no truncation) can have non-trivial $\pi_n$ for any $n$. Examples: $S^n$ (the $n$-sphere), $K(\mathbb{Z},n)$, etc.

The *Postnikov tower* of a type $A$ is a sequence of truncations:

$$A \to \tau_0 A \to \tau_1 A \to \tau_2 A \to \cdots \to \tau_n A$$

where $\tau_n A$ is the $n$-truncation of $A$ (kill all homotopy above level $n$). The original $A$ is the limit of this tower.

## Complete Segal Spaces

For completeness (pun intended), let's mention another model of ∞-groupoids/∞-categories that's important in the literature.

**Segal conditions.** A simplicial space $X : \Delta^{op} \to \mathbf{Top}$ satisfies the Segal condition if for each $n$:
$$X_n \simeq X_1 \times_{X_0} X_1 \times_{X_0} \cdots \times_{X_0} X_1 \quad (n \text{ copies})$$

This says: the space of composable $n$-tuples of morphisms is determined by pairwise composability. It's a way of encoding "composition is determined by binary composition."

**Complete Segal spaces (Rezk, 2001).** A simplicial space is a *complete Segal space* if it satisfies:
1. The Segal condition
2. Completeness: the space of "equivalences" is $X_0$ (objects are determined by their identity morphisms)

Complete Segal spaces model (∞,1)-categories. A complete Segal space is an ∞-groupoid if and only if all "morphisms" are equivalences.

**Theorem 5.6 (Equivalence of models, various authors).** The following model structures are all Quillen equivalent:
- Simplicial sets (with Joyal model structure for (∞,1)-categories)
- Complete Segal spaces
- Segal categories
- Relative categories (with Barwick-Kan model structure)

This is the theorem that justifies calling all of these "models of (∞,1)-categories" — they all capture the same abstract notion.

## Connection to HoTT: The Internal Language Theorem

Let's state the big theorem, even though its full proof is ongoing work.

**Theorem 5.7 (Voevodsky, Shulman, and others — informal).** HoTT + Univalence is the internal language of the ∞-topos of spaces.

More precisely:
- The ∞-topos of spaces (Kan complexes) has an internal language
- This internal language includes all constructions of HoTT: Π types, Σ types, identity types, universes
- The Univalence axiom holds internally
- Higher inductive types correspond to ∞-categorical constructions (homotopy pushouts, etc.)

This means: every theorem you prove in HoTT is a theorem about homotopy types, and every property of homotopy types can be expressed in HoTT.

**The inverse direction** (from models to syntax) requires:
- An ∞-topos gives a model of HoTT (this is a theorem, partially proved by Shulman)
- Every model of HoTT + Univalence arises from an ∞-topos (this is the harder direction, still being formalized)

The significance: HoTT is not just one way to talk about homotopy types. It is *the* canonical language for talking about homotopy types, in the same way that intuitionistic logic is the canonical language for talking about sheaves.

## Summary

| Model | Definition | Strengths | Weaknesses |
|---|---|---|---|
| Kan complexes | Simplicial sets + all horn filling | Clean, well-developed theory | Doesn't capture composition direction |
| Strict ω-groupoids | Globular sets + strict laws | Easy to define | Too rigid, misses most homotopy types |
| Weak ω-groupoids | Various algebraic definitions | Closer to Grothendieck's vision | Competing definitions, hard to work with |
| Types in HoTT | Identity type structure | Native to the type theory | Requires careful formalization |

The lesson: Kan complexes are the right model for practical purposes. The connection to HoTT is the internal language theorem — types in HoTT are Kan complexes, and their internal ∞-groupoid structure is captured by the identity types.

Everything we've been building — from the J rule (Chapter 9) to the categorical semantics (Chapters 10-11) to the homotopy hypothesis (this chapter) — is pointing at the same thing: homotopy types and type theory are the same subject, viewed from different angles. HoTT is the unification.
