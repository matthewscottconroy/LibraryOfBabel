# Thought Experiments — Chapter 24: Simplicial Type Theory

## Thought Experiment 1: The Asymmetry of Time

Consider time. Physical time (as far as we know, and setting aside exotic physics) is directed: it flows forward. The future is not the past. There is no symmetry that exchanges tomorrow and yesterday. The laws of physics are, at a microscopic level, approximately time-symmetric, but the macroscopic experience of time is radically directed.

A morphism in a category is like a moment in time: it goes one way. If you have a process that turns an acorn into an oak tree, there is no corresponding process that turns an oak tree into an acorn. The morphism $f : \mathsf{acorn} \to \mathsf{oak}$ is not the same as $g : \mathsf{oak} \to \mathsf{acorn}$ — and in fact $g$ may not exist at all.

A path in HoTT, by contrast, is like a spatial direction: reversible. If you can go from $a$ to $b$, you can also go from $b$ to $a$. There is no preferred orientation.

*Why does mathematics have both time-like (directed) and space-like (undirected) structure? Give examples from each. What mathematical objects are intrinsically directed, and which are undirected? Can you think of mathematical structures that are "almost" directed — where reversal is possible but costly or non-canonical?*

## Thought Experiment 2: The Coherence-Free World

Classical higher category theory is plagued by coherence conditions. A bicategory has an associator $\alpha_{f,g,h} : (h \circ g) \circ f \Rightarrow h \circ (g \circ f)$, unitors $\lambda_f : \mathsf{id} \circ f \Rightarrow f$ and $\rho_f : f \circ \mathsf{id} \Rightarrow f$, and must satisfy the *pentagon identity* and the *triangle identity*. Moving to tricategories, the coherence conditions fill pages. At higher levels, the complexity is prohibitive.

In simplicial type theory, there are *no coherence conditions to specify*. The Segal condition says composition is unique (up to contractibility), and all coherences are automatic consequences.

*Is something lost when we pass from the explicit-coherence world to the contractibility world? Is there any mathematical content in specifying the associator explicitly that is lost when you just say "composition is unique"? Or is the explicit-coherence world just a messier version of the same mathematical content?*

*Here is a concrete question*: in a tricategory, the *pentagonator* is a specific coherence 3-cell between two ways of rebracketing five composable morphisms. In a Segal type, this corresponds to a contractible space of rebracketed composites. What does the pentagonator correspond to in the contractibility world? Is there a sense in which "specifying the pentagonator" is more information than "knowing the space of composites is contractible"?

## Thought Experiment 3: Functors Without Conditions

In simplicial type theory, functors between Segal types are just functions. The functoriality conditions — preservation of identities and composition — are automatically satisfied.

This seems almost too good to be true. In classical mathematics, we distinguish carefully between *maps* (arbitrary functions on objects) and *functors* (maps that respect the categorical structure). The distinction matters: a map that doesn't preserve composition is not a functor and doesn't give a well-defined categorical morphism.

*How does STT manage to collapse this distinction? Is there a sense in which "arbitrary functions between Segal types" includes functions that a classical category theorist would not call functors? Or does the Segal structure force every function to be functorial?*

*Here is a specific question*: suppose $A$ and $B$ are discrete types (∞-groupoids) with no non-trivial morphisms. Then every function $f : A \to B$ is trivially a functor. Now suppose $A$ is a non-trivial Segal type (with actual non-invertible morphisms). Does the hom type $\mathsf{hom}_A(a, b) = \{ g : \mathbf{2} \to A \mid g(0) = a, g(1) = b \}$ structure automatically force any $f : A \to B$ to map composites to composites? Work through the definition.

## Thought Experiment 4: Directed Univalence — What Would It Mean?

The Rezk condition for the universe of types is exactly the Univalence Axiom. The Rezk condition for a poset is antisymmetry. The pattern suggests:

*The Rezk condition is the general principle: "isomorphic objects in a Segal type are equal."*

Now consider the *universe of Segal types* — the type $\mathsf{Segal}$ of all Segal types. The isomorphisms in this Segal type (if it is itself Segal) would be the *categorical equivalences*: fully faithful and essentially surjective functors.

The Rezk condition for $\mathsf{Segal}$ would say: two Segal types are equal iff they are categorically equivalent.

*What would this mean philosophically?* Specifically: in ordinary HoTT, two types being equal means you can substitute one for the other in any context (by the Substitution Lemma). If two Segal types are equal iff categorically equivalent, then you can substitute one ∞-category for another whenever they are equivalent. This is the *Principle of Equivalence* in category theory.

Is the Principle of Equivalence a consequence of a foundational principle (univalence/Rezk), or is it an additional axiom? Is there a sense in which working mathematics *already* satisfies the Principle of Equivalence, even in classical foundations?

## Thought Experiment 5: The Yoneda Lemma as Tautology

The Yoneda lemma states: $\mathsf{Nat}(\mathsf{Hom}(c,-), F) \cong F(c)$.

In the classical proof, this requires:
1. Defining natural transformations with their naturality condition
2. Specifying the bijection explicitly
3. Checking that the bijection is well-defined (natural in $c$ and $F$)
4. Checking that it is a bijection (constructing the inverse and verifying)

In the synthetic proof, the statement is an equivalence of types, the proof uses the covariant fibration structure, and the naturality is automatic.

*Is the synthetic Yoneda lemma "less surprising" than the classical one?* The classical Yoneda lemma has a quality of being *unexpected*: it says something non-trivial about the relationship between a category and its representable functors. Does the synthetic version preserve this quality, or does it reduce the lemma to something trivially true from the definitions?

*Alternately*: is the classical Yoneda lemma "more surprising" only because the classical formulation adds complexity (naturality conditions, set-theoretic considerations) that the synthetic formulation correctly identifies as inessential?

## Thought Experiment 6: What Is a Directed Path?

In HoTT, a path from $a$ to $b$ is a proof that $a = b$ — an equality. The path is symmetric: from $p : a = b$ you get `sym p : b = a`.

In STT, a directed path from $a$ to $b$ (a morphism in the hom type) is... what? It is a function $f : \mathbf{2} \to A$ with $f(0) = a$ and $f(1) = b$. But what does this *mean*?

Here are three interpretations:
1. **Process**: $f$ describes a process that begins at $a$ and ends at $b$. The directionality is temporal — the process has a start and an end.
2. **Evidence**: $f$ is evidence that there is a morphism from $a$ to $b$ in some underlying structure. The directionality is logical — the morphism is a proof of a proposition.
3. **Geometry**: $f$ is a directed path in a "directed space" where paths have an orientation. The directionality is geometric — like the direction of a vector field or a flow.

*Which interpretation is correct? Are they all equivalent? Does it matter which interpretation you use for the mathematical development?*

*More specifically*: in a poset, the morphism $f : \mathsf{hom}_P(a, b) = (a \leq b)$ is clearly "evidence" — a proof of the ordering. In the Segal type $\mathsf{Set}$, the morphism $f : \mathsf{hom}_\mathsf{Set}(A, B)$ is a function — a "process" that transforms elements. What is a morphism in the Segal type $\mathsf{Type}$ (with hom = functions)? Is a function between types a process, evidence, or geometry?
