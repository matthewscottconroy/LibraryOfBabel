# Exercises — Chapter 24: Simplicial Type Theory

## Section 1: Two Intervals and Extension Types

**Exercise 1.** Explain the difference between the undirected interval $\mathbb{I}$ and the directed interval $\mathbf{2}$. For each of the following operations, state whether it makes sense for $\mathbb{I}$, $\mathbf{2}$, both, or neither:
- (a) Path reversal: `sym p : b → a` from `p : a → b`
- (b) Constant path: `id_a : a → a`
- (c) Path concatenation: from `p : a → b` and `q : b → c`, get `p · q : a → c`
- (d) Path inversion: from an isomorphism `f : a → b`, get `f⁻¹ : b → a`

**Exercise 2.** Write out the definition of the inner horn $\Lambda^2_1$ as a sub-shape of $\Delta^2$. Then:
- (a) Describe what a map $\Lambda^2_1 \to A$ consists of (two morphisms and an endpoint)
- (b) Describe what a map $\Delta^2 \to A$ consists of (three vertices, three morphisms, one 2-cell)
- (c) Write the restriction map $(\Delta^2 \to A) \to (\Lambda^2_1 \to A)$ explicitly

**Exercise 3.** The extension type $\langle \phi \to f \rangle_{\psi \to A}$ is the type of extensions. For the following cases, describe what the extension type contains:
- (a) $\psi = \mathbf{2}$, $\phi = \{0_\mathbf{2}, 1_\mathbf{2}\}$ (the full boundary), $f = [0 \mapsto a, 1 \mapsto b]$
- (b) $\psi = \Delta^2$, $\phi = \Lambda^2_1$, $f = (g, h)$ a composable pair
- (c) $\psi = \Delta^3$, $\phi = \mathsf{Sp}[3]$ (the 3-spine), $f$ = three composable morphisms

**Exercise 4.** In simplicial type theory, the identity morphism at $a : A$ is $\mathsf{id}_a = \lambda t. a : \mathsf{hom}_A(a, a)$. Show that in a Segal type $A$:
- (a) $f \circ \mathsf{id}_a = f$ for any $f : \mathsf{hom}_A(a, b)$ (right unit law)
- (b) $\mathsf{id}_b \circ f = f$ for any $f : \mathsf{hom}_A(a, b)$ (left unit law)
(You may use the Segal condition — the fact that composites are unique up to contractibility.)

**Exercise 5.** In Rzk, download and install the proof assistant (`rzk-lang.github.io`). Open the basic library and find the definition of `hom`, `isSegal`, and `Δ²`. Study the type signatures. Write a Rzk term witnessing the identity morphism and verify it type-checks.

## Section 2: Segal Types

**Exercise 6.** Show that every ∞-groupoid (HoTT type) is a Segal type. Specifically:
- (a) Define $\mathsf{hom}_A(a, b) = (a =_A b)$ for a HoTT type $A$
- (b) Show that the composable pair $(p : a = b, q : b = c)$ has a unique composite $p \cdot q : a = c$ (i.e., the space of composites $\Sigma_{r : a = c} \ldots$ is contractible)
- (c) Why is every path an isomorphism in this Segal structure?

**Exercise 7.** Verify that the universe $\mathsf{Type}$ is a Segal type with $\mathsf{hom}_\mathsf{Type}(A, B) = (A \to B)$:
- (a) The identity morphism at $A$ is $\mathsf{id}_A : A \to A$ — the identity function. Verify this is in $\mathsf{hom}_\mathsf{Type}(A, A)$.
- (b) The composite of $(f : A \to B, g : B \to C)$ is $g \circ f : A \to C$. Why is this unique?
- (c) Why does the Segal condition hold? Is the space of composites strictly a singleton or contractible?

**Exercise 8.** Let $P$ be a preorder (a set with a reflexive, transitive relation $\leq$). Define a Segal type $A_P$ with the same elements and $\mathsf{hom}_{A_P}(a, b) = (a \leq b)$ (a proposition).
- (a) What is the composition operation?
- (b) What are the identity morphisms?
- (c) Is $A_P$ Rezk? What additional condition on $P$ would be needed?

**Exercise 9.** Show that a Segal type $A$ has *associative* composition. Specifically: for composable morphisms $f, g, h$, show that $(h \circ g) \circ f$ and $h \circ (g \circ f)$ are homotopic. (Hint: both are composites filling the same 3-dimensional horn; use the 3-dimensional Segal condition.)

**Exercise 10.** In Rzk, formalize the following:
- (a) The type $\mathsf{Bool}$ (with two elements `true` and `false`) as a Segal type with hom = equality
- (b) The type $\mathbb{N}$ as a Segal type with $\mathsf{hom}_\mathbb{N}(m, n) = (m \leq n)$ (the poset structure)
- (c) Prove that $\mathsf{hom}_\mathbb{N}(m, n)$ is a proposition for all $m, n$

**Exercise 11.** The spine of $\Delta^n$ is $\mathsf{Sp}[n] = \{(t_1, \ldots, t_n) : \mathbf{2}^n \mid t_1 \leq \cdots \leq t_n\}$. State the $n = 3$ Segal condition explicitly. What does it say geometrically? What does it say categorically (in terms of composition)?

**Exercise 12.** A *discrete type* in STT is a type where $\mathsf{hom}_A(a, b) \simeq (a =_A b)$ for all $a, b$. Show:
- (a) Every proposition is discrete (as a Segal type)
- (b) Every h-set is discrete
- (c) A discrete Segal type is automatically Rezk

## Section 3: Rezk Types

**Exercise 13.** For a Segal type $A$, define the type of isomorphisms $\mathsf{Iso}_A(a, b)$ precisely. Then:
- (a) Show that $\mathsf{Iso}_A(a, a)$ is inhabited (by the identity)
- (b) Show that if $f : \mathsf{Iso}_A(a, b)$, then $f^{-1} : \mathsf{Iso}_A(b, a)$
- (c) Show that $\mathsf{Iso}_A(a, b) \times \mathsf{Iso}_A(b, c) \to \mathsf{Iso}_A(a, c)$ (composition of isomorphisms)

**Exercise 14.** The map $\alpha_{a,b} : (a =_A b) \to \mathsf{Iso}_A(a, b)$ sends `refl_a` to the identity isomorphism. Show:
- (a) $\alpha_{a,b}$ is well-defined (by path induction)
- (b) The Rezk condition for $A$ is that this map is an equivalence
- (c) In an ∞-groupoid, $\alpha_{a,b}$ is already an equivalence (without needing any additional axiom)

**Exercise 15.** Prove the equivalence: $\mathsf{Type}$ (with $\mathsf{hom}$ = functions) is Rezk if and only if the Univalence Axiom holds. Specifically:
- (a) Show that $\mathsf{Iso}_\mathsf{Type}(A, B) \simeq (A \simeq B)$ (isomorphisms in $\mathsf{Type}$ are equivalences)
- (b) Conclude that the Rezk condition for $\mathsf{Type}$ is exactly univalence
- (c) What does this tell us about univalence as a foundational axiom?

**Exercise 16.** The Rezk completion of a Segal type $A$ is the initial Rezk type $\hat{A}$ with a functor $\iota : A \to \hat{A}$.
- (a) What is the Rezk completion of the preorder $(\mathbb{Z}, \leq)$? (Hint: $\mathbb{Z}$ with $\leq$ is already a partial order.)
- (b) What is the Rezk completion of the preorder $(\mathbb{Z}, \equiv_2)$ where $m \equiv_2 n$ iff $m \equiv n \pmod{2}$?
- (c) What is the Rezk completion of the Segal type $\mathsf{Type}$ without univalence?

**Exercise 17.** Define *directed univalence* for a Segal type $\mathsf{Segal}$ of Segal types. What would the isomorphisms in $\mathsf{Segal}$ be? What would the Rezk condition for $\mathsf{Segal}$ say? Is this condition provable from the axioms of STT? (This is a research-level question; a careful statement of what would need to be proved is sufficient.)

## Section 4: Functors and Yoneda

**Exercise 18.** In STT, functors are functions. Verify explicitly that a function $f : A \to B$ between Segal types is a functor by showing:
- (a) $f$ preserves identities: $f_*(\mathsf{id}_a) = \mathsf{id}_{f(a)}$ where $f_* : \mathsf{hom}_A(a, b) \to \mathsf{hom}_B(f(a), f(b))$ is defined by $f_*(g) = f \circ g$
- (b) $f$ preserves composition: $f_*(h \circ g) = f_*(h) \circ f_*(g)$ (using uniqueness of composites in $B$)

**Exercise 19.** Natural transformations are morphisms in function types. For Segal types $A$ and $B$, show:
- (a) An element $\alpha : \mathsf{hom}_{B^A}(f, g)$ assigns a morphism $\alpha_a : \mathsf{hom}_B(f(a), g(a))$ to each $a : A$
- (b) The naturality condition follows automatically: for $h : \mathsf{hom}_A(a, b)$, $g_*(h) \circ \alpha_a = \alpha_b \circ f_*(h)$
- (c) Why does (b) follow from the Segal structure, without needing to be checked explicitly?

**Exercise 20.** State and prove the synthetic Yoneda lemma: for a Segal type $A$, $a : A$, and covariant fibration $C : A \to \mathsf{Type}$:
$$\mathsf{hom}_{(A \to \mathsf{Type})}(\mathsf{hom}_A(a, -), C) \simeq C(a)$$
Provide the evaluation map $\mathsf{ev}_a$ and its quasi-inverse $\Phi$, and verify both compositions.

**Exercise 21.** The *Yoneda embedding* is $\mathsf{y} : A \to (A^{op} \to \mathsf{Type})$ defined by $\mathsf{y}(a) = \mathsf{hom}_A(-, a)$.
- (a) Show that $\mathsf{y}$ is a function (hence a functor by the STT principle)
- (b) Show that $\mathsf{y}$ is fully faithful: $\mathsf{hom}_{A^{op} \to \mathsf{Type}}(\mathsf{y}(a), \mathsf{y}(b)) \simeq \mathsf{hom}_A(a, b)$
- (c) What does it mean for $\mathsf{y}$ to be fully faithful in the context of category theory?

**Exercise 22.** In Rzk, find the formalization of the Yoneda lemma (`rzk-lang/sHoTT` or the main Rzk library). Read the proof. Identify:
- (a) Where the covariant fibration condition is used
- (b) Where the Segal condition is used
- (c) The key step that in the classical proof requires checking naturality

**Exercise 23.** An *adjunction* between Segal types $A$ and $B$ consists of functors $L : A \to B$ and $R : B \to A$ with a natural equivalence $\mathsf{hom}_B(L(a), b) \simeq \mathsf{hom}_A(a, R(b))$.
- (a) Define "natural in $a$ and $b$" precisely in the language of STT
- (b) Show that adjoints are unique up to unique isomorphism (using the Yoneda lemma)
- (c) Give an example of an adjunction between two Segal types

**Exercise 24.** (Advanced) Define a *Segal object* in a Segal type: a Segal type $A$ "internal to" a Segal type $C$. Specifically, define what it means for a morphism $p : E \to A$ in $C$ to be a covariant fibration. Then define an internal Segal structure on $A$ using this notion.

**Exercise 25.** (Research-level) Formulate the statement of the *synthetic adjoint functor theorem*: a functor $R : B \to A$ between Rezk types has a left adjoint if and only if [condition]. What should the condition be, and why? Compare with the classical adjoint functor theorem (Solution Set Condition, Kan's Theorem).

**Exercise 26.** (Proof assistant project) In Rzk, formalize the following:
- (a) The Segal type $\mathsf{Poset}$ of posets (a covariant fibration over some base)
- (b) The functor $\mathsf{Forget} : \mathsf{Poset} \to \mathsf{Set}$ (forgetting the order)
- (c) Whether $\mathsf{Forget}$ has a left adjoint (the discrete poset construction)

**Exercise 27.** The *twisted arrow category* $\mathsf{Tw}(A)$ of a Segal type $A$ has objects which are morphisms in $A$ and morphisms which are "factorizations." In STT, this should be a specific type. Define $\mathsf{Tw}(A)$ using extension types and the directed interval.

**Exercise 28.** (Advanced) Prove that the type $\mathsf{Fun}(A, B)$ of functors from a Segal type $A$ to a Rezk type $B$ is itself Rezk. The isomorphisms in $\mathsf{Fun}(A, B)$ should be the *natural isomorphisms* (natural transformations where each component is an isomorphism).

**Exercise 29.** The classical *Grothendieck construction* sends a functor $F : \mathcal{C} \to \mathsf{Cat}$ to a fibered category over $\mathcal{C}$. Formulate the synthetic Grothendieck construction: for a covariant fibration $C : A \to \mathsf{Type}$ over a Segal type $A$, define the total space $\Sigma_{a:A} C(a)$ and show it is Segal.

**Exercise 30.** (Open problem discussion) The *directed univalence* conjecture says that the Segal type $\mathsf{Segal}$ of Segal types is itself Rezk, where isomorphisms are categorical equivalences. Identify three key difficulties in proving this:
- (a) What would you need to show about $\mathsf{Segal}$ as a Segal type?
- (b) What is the correct notion of "categorical equivalence" in STT?
- (c) Why is the Rezk condition for $\mathsf{Segal}$ harder to prove than the Rezk condition for $\mathsf{Type}$ (i.e., univalence)?
