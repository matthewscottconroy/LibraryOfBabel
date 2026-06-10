# Applied Exercises

Higher category theory may seem like it belongs purely in the realm of abstract foundations, but its core concepts — coherence, weak composition, pasting diagrams, and the homotopy hypothesis — appear in concrete computational and engineering contexts. Software architecture diagrams, type-theoretic equality, computational models of simplicial sets, and extended topological field theories all instantiate higher-categorical ideas in forms where the mathematical distinctions are practically meaningful. The exercises below draw these connections explicitly, with the goal of making the abstract machinery of ∞-categories feel necessary rather than merely possible.

---

## Exercise A.1: Pasting Diagrams and Software Architecture Coherence
*Domain: Software Architecture / Formal Methods*

**Setup:** Software architects use "pasting diagrams" — diagrams of components (boxes) connected by data flows or service calls (arrows) — to reason about system composition. When component $A$ calls service $B$ which calls service $C$, this gives a composed data-flow $A \to C$. In practice, architects compose these diagrams in two ways: "horizontally" (composing the calls themselves) and "vertically" (composing the transformations of data, e.g., two serialization steps). These two composition operations interact.

In a strict 2-category, horizontal and vertical composition are required to satisfy the *interchange law* strictly on the nose: $(f' \circ f) * (g' \circ g) = (f' * g') \circ (f * g)$ (where $\circ$ is vertical and $*$ is horizontal composition). In a bicategory, the interchange law holds only up to a coherent invertible 2-cell (called an *interchanger*), and the coherence theorem (Mac Lane, 1971; proved more carefully by Gordon-Power-Street, 1995) ensures that all diagrams of such cells commute.

**Questions:**
1. Model a three-layer service architecture (client $\to$ gateway $\to$ database) as a 2-category where objects are system states, 1-morphisms are service calls, and 2-morphisms are behavioral refinements (e.g., "this implementation satisfies this contract"). Write down the data a strict 2-category requires and identify what the interchange law says about composing behavioral refinements.
2. Suppose behavioral refinements are only specified "up to observational equivalence" (two implementations are indistinguishable if they produce the same outputs on all inputs). Why does this force you to work with a bicategory rather than a strict 2-category? What does the coherence theorem for bicategories guarantee about reasoning with such refinements?
3. The coherence theorem for bicategories says "every bicategory is biequivalent to a strict 2-category." This is an existence result. In practice, the strict replacement may be astronomically larger. In formal verification of software, why would you prefer to work with the bicategory rather than its strict replacement, even though the strict replacement is "simpler" algebraically?

*Abstract concept illustrated: The coherence theorem for bicategories (Mac Lane coherence); the distinction between strict and weak 2-categories; why weakness is the natural setting for mathematical structures defined up to equivalence.*

---

## Exercise A.2: Type Equality in HoTT as an ∞-Groupoid
*Domain: Programming Language Theory / Dependent Types*

**Setup:** In a dependently typed language like Agda or Coq, the equality type $a = b$ between two terms of a type $A$ is itself a type, and proofs of equality between proofs of equality form higher types. In HoTT, this tower of equalities has the structure of an ∞-groupoid.

Consider a type $A = \mathsf{Fin}(4)$ (the type of natural numbers less than 4, representing four colors). There are two terms $x, y : A$ and two proofs $p, q : x = y$ (say, both asserting that $x = y$ via different proof terms — for instance, in a setting with computational rules, two definitional paths that are propositionally equal but not definitionally so). A 2-path (homotopy) $H : p = q$ is a term of the iterated identity type.

**Questions:**
1. In Agda (or pseudocode), write out the type of a "path of paths" $H : p =_{(a=_A b)} q$ for $p, q : a =_A b$. This is the 2-level of the ∞-groupoid structure. What does the J eliminator (path induction) say you can do with $H$?
2. The ∞-groupoid structure on $A$ requires: (a) reflexivity ($\mathsf{refl}_a : a = a$), (b) concatenation ($p \cdot q : a = c$ from $p : a = b$ and $q : b = c$), (c) inversion ($p^{-1} : b = a$ from $p : a = b$), and (d) these satisfying associativity and unitality *up to higher paths*. Show concretely why associativity of concatenation is not definitionally true in MLTT (compute $(p \cdot q) \cdot r$ and $p \cdot (q \cdot r)$ using the definition via $J$ and observe they are not definitionally equal). Produce the coherence 2-path between them.
3. Now consider the universe type $\mathsf{Type}$ with univalence. A path $P : A =_{\mathsf{Type}} B$ in the universe corresponds to an equivalence $e : A \simeq B$. At the ∞-groupoid level, this means that paths in the universe are equivalences, 2-paths are "homotopies between equivalences," and so on. Explain how the completeness condition in Rezk's complete Segal spaces mirrors the univalence axiom: in both cases, the data of an "equivalence" coincides with the data of a "path."

*Abstract concept illustrated: Types as ∞-groupoids; the tower of identity types; the connection between univalence and the completeness condition in complete Segal spaces.*

---

## Exercise A.3: Simplicial Sets as a Computational Model for ∞-Groupoids
*Domain: Computer Science / Algorithms and Data Structures*

**Setup:** The homotopy hypothesis says that ∞-groupoids are the same as homotopy types, and that Kan complexes (simplicial sets satisfying the Kan condition) are the standard model for both. A simplicial set is a sequence of sets $X_0, X_1, X_2, \ldots$ (0-simplices, 1-simplices, 2-simplices, ...) with face maps $d_i : X_n \to X_{n-1}$ and degeneracy maps $s_i : X_n \to X_{n+1}$ satisfying the simplicial identities.

This is a purely combinatorial, discrete structure — there are no topological spaces, no epsilons and deltas. It can be represented on a computer as a collection of sets (or types) with operations between them.

**Questions:**
1. Represent the simplicial set for the circle $S^1$ (as a minimal Kan complex) on a computer. The minimal triangulation of $S^1$ has: two 0-simplices ($\{v_0, v_1\}$), three 1-simplices ($\{e_{01}, e_{10}, \ell\}$ where $\ell$ is the loop), and non-degenerate simplices in all higher degrees arising from degeneracies. Write out the face maps $d_0, d_1 : X_1 \to X_0$ for the three 1-simplices. Verify that the Kan horn-filling condition for $\Lambda^2_1$ holds (the inner horn, corresponding to "composition").
2. The singular simplicial set of a topological space $X$ is defined by $\mathsf{Sing}(X)_n = \mathsf{Hom}(\Delta^n_{\mathsf{top}}, X)$ (continuous maps from the topological $n$-simplex). For $X = S^1$, what are $\mathsf{Sing}(S^1)_0$, $\mathsf{Sing}(S^1)_1$, and $\mathsf{Sing}(S^1)_2$ as sets? Explain why this set is uncountably infinite even though the "minimal" simplicial model of $S^1$ is finite.
3. For a computer-assisted proof assistant (like Agda with HoTT), the relevant simplicial sets are the ones generated by types and their identity types. The "computational homotopy hypothesis" says these should be Kan complexes. Explain the obstacle: why does an arbitrary dependent type in MLTT not directly give a *decidable* simplicial set (one where you can algorithmically determine membership in $X_n$)? What role does decidable equality play, and why does this mean that the homotopy-theoretic and computational perspectives come apart at this point?

*Abstract concept illustrated: Kan complexes as the simplicial model of ∞-groupoids; geometric realization vs. singular complex; the relationship between combinatorial and topological formulations of the homotopy hypothesis.*

---

## Exercise A.4: Extended TFTs and ∞-Categorical Data
*Domain: Mathematical Physics / Quantum Field Theory*

**Setup:** A *topological field theory* (TFT) in the sense of Atiyah assigns to each closed $(n-1)$-manifold $\Sigma$ a vector space $Z(\Sigma)$, and to each $n$-dimensional cobordism $M : \Sigma_0 \to \Sigma_1$ a linear map $Z(M) : Z(\Sigma_0) \to Z(\Sigma_1)$. This is a functor from the category of cobordisms to the category of vector spaces.

An *extended* TFT goes further: it also assigns data to manifolds with corners, and in fact to manifolds of all codimensions down to 0. Formally, an extended $(n, k)$-TFT is a (higher) functor from a k-fold symmetric monoidal $(\infty, k)$-category of cobordisms to a target $(\infty, k)$-category (typically of some kind of linear category). The **cobordism hypothesis** (Baez-Dolan 1995, proved by Lurie 2009) says that a fully extended $n$-TFT (with $k = n$) is completely determined by a single object in the target: a "fully dualizable" object.

**Questions:**
1. For $n = 1$: an ordinary (non-extended) 1-TFT assigns a vector space $V$ to the circle $S^0 = \{+, -\}$ and a linear map $Z([0,1]) : V \to V$ to the interval. Verify that this data must satisfy $Z([0,1])^2 = Z([0,1])$ (the interval glued to itself is the interval again). Now describe what data a *fully extended* 1-TFT requires: what does it assign to the point $\{*\}$, and what algebraic condition (the "fully dualizable" condition) must this data satisfy?
2. For $n = 2$: a 2-TFT assigns data to 2-manifolds, 1-manifolds, and points. The cobordism hypothesis says a fully extended 2-TFT is determined by a *fully dualizable object* in a symmetric monoidal 2-category (e.g., an algebra $A$ in the Morita 2-category). The Morita 2-category has algebras as objects, bimodules as 1-morphisms, and bimodule maps as 2-morphisms. Identify the fully dualizable objects in the Morita 2-category: they are the *separable* algebras (those satisfying a specific duality condition). What is the HoTT analog of "full dualizability"?
3. The cobordism hypothesis requires an (∞,n)-category of cobordisms. Sketch (at the level of objects, 1-morphisms, and 2-morphisms) why the cobordism category for $n = 2$ with corners naturally forms a 2-category: what are the objects, 1-morphisms, and 2-morphisms, and how does composition work? Why does the presence of "corners" (intersections of boundaries) require exactly the 2-categorical data, making a strict 1-categorical description insufficient?

*Abstract concept illustrated: The cobordism hypothesis; fully extended TFTs as classified by (∞,n)-categorical data; why higher-categorical structure is forced by the geometry of manifolds with corners.*

---

## Exercise A.5: Model Categories and ∞-Categories of Diagrams
*Domain: Distributed Systems / Concurrent Computation*

**Setup:** In distributed systems theory, a **consensus protocol** can be modeled as a diagram of "states" (processes, values, messages) connected by "transitions" (computation steps). Two protocols are considered equivalent if there is a "bisimulation" — a correspondence between their transition systems that is compatible with transitions. This is a form of weak equivalence between models, not strict isomorphism.

In homotopy theory, model categories provide a systematic way to invert weak equivalences: they axiomatize the necessary structure (fibrations, cofibrations, weak equivalences) to "localize" a category at its weak equivalences and obtain a well-defined homotopy category. Quillen's model structure on simplicial sets inverts the maps that induce isomorphisms on all homotopy groups, yielding the homotopy category of spaces.

**Questions:**
1. A model category has three distinguished classes of morphisms: weak equivalences ($W$), fibrations ($F$), and cofibrations ($C$), satisfying the axioms (MC1)–(MC5). Identify these three classes in the following setting: diagrams of Kripke frames (used in modal logic and distributed systems), where weak equivalences are bisimulations, fibrations are "image-closed" bisimulations, and cofibrations are cofibrant replacements. Check whether MC2 (the "two-out-of-three" axiom: if two of $f$, $g$, $fg$ are weak equivalences then so is the third) holds for bisimulations.
2. The homotopy category $\mathsf{Ho}(\mathcal{M})$ of a model category $\mathcal{M}$ is obtained by formally inverting all weak equivalences. In the distributed systems context, $\mathsf{Ho}(\mathsf{Kripke})$ is the category of Kripke frames modulo bisimulation. Explain why this is not the same as simply quotienting by bisimulation equivalence classes — what information about maps between frames is lost by passing to equivalence classes, and what is preserved?
3. An (∞,1)-category presents more information than just the homotopy category: instead of a *set* of morphisms $\mathsf{Ho}(X, Y)$, it gives a *space* (simplicial set) of morphisms $\mathsf{Map}(X, Y)$. In the distributed systems context, this space would encode not just "are $X$ and $Y$ bisimilar" but "what is the space of all bisimulations between them?" Describe a concrete example where two Kripke frames are bisimilar but have *non-contractible* bisimulation spaces (i.e., the space of bisimulations between them has nontrivial topology). Why does this matter for formal verification?

*Abstract concept illustrated: Model categories as presentations of (∞,1)-categories; the role of weak equivalences; the difference between a homotopy category and a full (∞,1)-category of maps.*
