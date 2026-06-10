# Chapter 24: Simplicial Type Theory

## Category Theory Lives on the Boundary Between Mathematics and Philosophy.

Category theory lives on the boundary between mathematics and philosophy. It says: to understand a mathematical object, look not at what it IS but at how it RELATES to everything else. A group is not its elements and operation — a group is what it does to the category of sets via its action, what representations it has, how it maps to and from other groups. This is not a claim about groups specifically. It is a claim about everything.

Simplicial type theory takes this philosophical stance and makes it foundational. Not as a slogan, but as a mathematical object: a type theory in which the fundamental things are not sets or spaces but *categories*. Where morphisms are first-class citizens. Where the Yoneda lemma is not a theorem you prove after developing all the machinery — it is a principle that lives in the language itself, nearly definitional.

The question that motivates the entire program: *what would it mean to have a type theory where types are ∞-categories?*

Not ∞-groupoids — those are what HoTT gives you. In an ∞-groupoid, every morphism is invertible, every path has a reverse, every equivalence has a quasi-inverse. This is the right setting for homotopy theory, where paths model homotopies between points and every homotopy can be run backward. But it is the wrong setting for category theory. In a category, morphisms go one way. The morphism from $A$ to $B$ is not the same as the morphism from $B$ to $A$. Functions compose in one direction. The identity is special precisely because it is the unit for a directed composition.

Emily Riehl and Michael Shulman found the answer in 2017: add a *second interval* to HoTT. One interval $\mathbb{I}$, undirected with complement, for ordinary homotopy paths. A second interval $\mathbf{2}$, directed without complement, for categorical morphisms. The result is *simplicial type theory* — a type theory with two kinds of "paths," one symmetric and one not.

---

### The Two Intervals

The contrast is sharp:

| Feature | $\mathbb{I}$ (cubical/undirected) | $\mathbf{2}$ (simplicial/directed) |
|---------|-----------------------------------|-------------------------------------|
| Endpoints | $0, 1$ | $0_\mathbf{2}, 1_\mathbf{2}$ |
| Complement | Yes ($\sim i$) | No |
| Symmetry | Yes | No |
| Path type | Undirected paths (homotopies) | Directed paths (morphisms) |
| Reversal | Automatic via $\sim$ | Only for invertible morphisms |

Paths in $\mathbb{I}$ are homotopies: they can be composed, reversed, and concatenated freely. Paths in $\mathbf{2}$ are morphisms: they compose but need not reverse. The asymmetry of $\mathbf{2}$ is what makes it suitable for category theory.

---

### Segal Types: When a Type is a Category

Not every type has categorical structure. A Segal type is one that does: a type $A$ where every composable pair of morphisms has a unique composite.

Precisely: $A$ is Segal if the restriction map $(\Delta^2 \to A) \to (\Lambda^2_1 \to A)$ is an equivalence. The left side is the type of "triangles" in $A$ (2-simplices); the right side is the type of "composable pairs" (inner horns). The map forgets the hypotenuse. The Segal condition says: every composable pair has a unique hypotenuse.

This is a condition on a type, not a structure you add. Either a type satisfies it or it doesn't. Types that do — ∞-categories, in the synthetic sense — have a composition operation that is uniquely determined by the simplicial structure. No coherence data to specify. No pentagon identities to check. The Segal condition absorbs all of that into the contractibility of horn-fillers.

---

### Rezk Types: When Isomorphism Implies Equality

Segal types have composition. Rezk types additionally have the property that *isomorphic objects are equal* — the categorical analogue of univalence.

A Segal type $A$ is Rezk if the canonical map $(a =_A b) \to \mathsf{Iso}_A(a, b)$ is an equivalence for all $a, b$. This is exactly the condition that makes the universe $\mathsf{Type}$ Rezk: $(A = B) \simeq (A \simeq B)$. Univalence is the Rezk condition for $\mathsf{Type}$.

This connection illuminates both sides. Univalence is not a mysterious additional axiom about types — it is the instance, for the specific Segal type $\mathsf{Type}$, of the general principle that a Segal type should be "complete" in the sense of identifying isomorphic objects.

---

### The Synthetic Yoneda Lemma

The Yoneda lemma states that for a locally small category $\mathcal{C}$ and an object $c \in \mathcal{C}$, the functor $\mathsf{Hom}(c, -) : \mathcal{C} \to \mathsf{Set}$ represents the evaluation at $c$. Precisely: natural transformations $\mathsf{Hom}(c, -) \Rightarrow F$ are in bijection with elements of $F(c)$.

In simplicial type theory, this becomes a theorem that is nearly a tautology: the evaluation map is an equivalence between the type of natural transformations and $F(c)$. The proof does not require developing the theory of simplicial sets, or checking naturality conditions, or verifying coherences. It follows from the basic properties of extension types and the Segal condition. This is what "synthetic" means: the machinery of the type theory replaces the machinery of simplicial set theory.

---

### Rzk: The Proof Assistant

The Rzk proof assistant (`rzk-lang.github.io`) is the primary implementation of simplicial type theory. It was designed specifically for this research program: formalizing the foundations of ∞-category theory in a synthetic setting.

Rzk is young (2021+) and fast-moving. The formalized results include:
- The Segal and Rezk conditions and their basic properties
- Functors, natural transformations, and their composition
- The synthetic Yoneda lemma
- Adjunctions between Segal types
- The beginnings of the theory of limits and colimits

Working in Rzk reveals something that no abstract account can capture: the synthetic approach is genuinely *easier* to work with, for certain problems, than the classical simplicial set approach. The absence of coherence problems, the direct expression of universal properties as extension type conditions — these are not just aesthetically pleasing. They reduce proof complexity in a quantifiable way.

---

### Chapter Roadmap

**Section 1: Two Intervals** — The undirected interval $\mathbb{I}$ and the directed interval $\mathbf{2}$. Extension types for specifying partial morphisms. The hom type as functions from $\mathbf{2}$. The two-level structure of simplicial type theory (outer: shapes; inner: spaces).

**Section 2: Segal Types** — The Segal condition as horn-filling. Composition as unique horn-filler. Examples: sets, posets, the universe. Why Segal types are ∞-categories. The spine condition.

**Section 3: Rezk Types** — Isomorphisms in a Segal type. The Rezk (completeness) condition. Univalence as Rezk for the universe. Rezk completion. The directed univalence conjecture.

**Section 4: Functors and the Yoneda Lemma** — Functors are functions (no extra conditions on Segal types). Natural transformations as directed paths in function types. The synthetic Yoneda lemma in Rzk. Comparison with the classical statement.
