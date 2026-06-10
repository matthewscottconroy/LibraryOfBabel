# Thought Experiments: Categorical Logic

## 1. The Geometry of Truth Values

In $\mathbf{Set}$, the truth values are $\{0, 1\}$ — exactly two. A proposition is either true or false.

In the sheaf topos $\mathsf{Sh}(X)$ over a topological space $X$, the truth values are the open sets of $X$. A proposition can be "true on the open set $U$" — true in the region $U$ of the underlying space. The logic is spatial.

This is not a metaphor. A theorem proved in the internal logic of $\mathsf{Sh}(X)$ — using only intuitionistic logic, no LEM — is genuinely true in the spatial sense. It holds at every point of $X$ (locally), and the truth can vary from region to region.

Now ask: what kind of mathematics do you get in $\mathsf{Sh}(X)$ for specific spaces $X$?

For $X = \mathbb{R}$ (the real line): the "real numbers" in the internal sense are not the constant functions $\mathbb{R} \to \mathbb{R}$ but the *continuous* functions $U \to \mathbb{R}$ on open sets. A "real number" in context is a continuous function. Addition and multiplication are pointwise operations on continuous functions. The "intermediate value theorem" — that a continuous function on $[0,1]$ that is negative at 0 and positive at 1 must be zero somewhere — is a theorem in this internal logic, but it comes out as: for any continuous function $f : [0,1] \to \mathbb{R}$ with $f(0) < 0 < f(1)$, the zero set of $f$ is a nonempty open set. This is the constructive IVT: you cannot pinpoint the zero, but you can show it exists on an open set.

For $X =$ a Cantor set or other fractal: the geometry becomes bizarre. What is the "internal logic" of a Cantor set?

The question: Is there a spatial interpretation of every intuitionistic theorem — a way to see every constructive proof as a statement about the topology of some space? And does the failure of LEM in spatial logic reveal a deep connection between logic and geometry?

## 2. What Topos Changes the Foundation

The Axiom of Choice (AC) is true in $\mathbf{Set}$. It is false in many toposes — for instance, in $\mathsf{Sh}(\mathbb{R})$, there are surjections without sections (continuous surjections $A \to B$ with no continuous section). AC fails because the "choice functions" need to be continuous, and continuity prevents arbitrary choices.

The effective topos $\mathsf{Eff}$ has Church's Thesis: every function $\mathbb{N} \to \mathbb{N}$ is computable. This is provably false in classical set theory (there are non-computable functions), but in $\mathsf{Eff}$, it is a theorem. The internal logic of $\mathsf{Eff}$ is the logic of *computable* mathematics.

Different toposes give different logics:
- $\mathbf{Set}$: classical logic + AC
- $\mathsf{Sh}(X)$: spatial intuitionistic logic
- $\mathsf{Eff}$: computability, Church's Thesis, Markov's Principle
- $\mathbf{sSet}$ (Kan): MLTT + Univalence

The question: is there a "universal" topos — one whose internal logic is exactly provable in some base type theory? The classifying topos of a geometric theory has this property: the internal logic of its classifying topos is exactly the geometric theory.

What would the "classifying topos" of HoTT look like? It would be an ∞-topos whose internal logic is exactly HoTT. Lurie's work suggests the answer: the classifying ∞-topos of HoTT is the ∞-category of ∞-groupoids (Kan complexes). Does this mean that every HoTT theorem is a theorem about homotopy types, with no additional structure?

## 3. The Groupoid Model's Surprise

Before Hofmann and Streicher's groupoid model, the general belief was that identity types in MLTT satisfied UIP — that any two proofs of the same identity were themselves identical. This seemed obvious: equality is equality, one proof is as good as another.

The groupoid model showed this intuition was wrong. In the groupoid model, the identity type $a =_A b$ is the set of morphisms from $a$ to $b$ in the groupoid $A$. Different morphisms are different proofs of $a = b$. UIP would say all morphisms are equal — but in a groupoid like $\pi_1(S^1)$, the morphisms $n : 0 \to 0$ (for each integer $n$, the path winding $n$ times) are all distinct.

The surprise: the failure of UIP is not a bug. It is a feature. It reveals that identity types are not mere "propositions" (true or false) but genuine *spaces* of paths. Different proofs of $a = b$ are different paths from $a$ to $b$, and these paths can carry geometric information (like the winding number).

Now ask: what would mathematics look like if we had always known this? Would we have formulated the identity relation differently? Would the development of algebra, topology, and analysis have gone differently if we had from the start treated equality as a space of paths rather than a proposition?

This is not a purely historical question. It is a design question: the founders of HoTT (Voevodsky, Lumsdaine, van den Berg, and others) made choices about how to formalize this insight. Are there alternatives? Could you have a type theory where identity types are proof-irrelevant (UIP holds) but still connected to homotopy theory in some other way?

## 4. The Internal Logic of an ∞-Topos

Ordinary toposes have an internal higher-order intuitionistic logic. The logic is *propositional*: propositions are subobjects, and truth values are elements of $\Omega$.

An ∞-topos has an internal logic too: it is HoTT. But in HoTT, propositions are not the fundamental objects — *types* are. Propositions are a special class of types (the h-props, the -1-truncated types). Truth values are types, not just {true, false}.

This means the internal "logic" of an ∞-topos is not just a logic — it is a *type theory*. The universe $\mathcal{U}$ in HoTT is the "type of types" in the ∞-topos — the object classifier. The propositions form a sub-universe $\mathcal{U}_{-1} = \mathsf{Prop}$ corresponding to the subobject classifier in the truncated (1-categorical) picture.

The question: what would it mean to do mathematics "in" an arbitrary ∞-topos, using HoTT as the internal language? Some ∞-toposes would validate LEM (Boolean ∞-toposes). Some would validate specific cohomological axioms. Some would validate parametricity.

Does every ∞-topos have a "characteristic" mathematical theory — a set of axioms that are valid in that ∞-topos but not others? And can we classify ∞-toposes by their internal theories, the way we classify ordinary toposes by their internal set theories?

## 5. Substitution Is Pullback: The Semantic Content

The equation "substitution is pullback" is a slogan in categorical logic. What does it actually mean?

In type theory: substituting a term $a : A$ for a variable $x : A$ in a type $B : A \to \mathcal{U}$ gives the type $B(a)$.

Categorically: substitution of $a : \Gamma \to A$ into the type family $p : \tilde{B} \to A$ gives the pullback $a^* \tilde{B}$ — the object in $\mathcal{C}/\Gamma$ whose fiber over each $\gamma \in \Gamma$ is $B(a(\gamma))$.

The identification "substitution = pullback" is not just a semantic convention. It has consequences:
- Substitution is functorial: $(B[f])[g] = B[f \circ g]$ because pullback is functorial.
- Substitution preserves $\Pi$ types (Beck-Chevalley) because the adjunction $\Sigma \dashv \pi^* \dashv \Pi$ is preserved by pullback.
- Substitution distributes over identity types: $(a =_A b)[f] = f(a) =_A f(b)$ because the path object construction is natural.

The thought experiment: is there a type theory where substitution is *not* pullback? What would it mean for substitution to fail the Beck-Chevalley condition? You would get a type theory where $\Pi_{x:A} B(x)$ and $\Pi_{f(x):A} B(f(x))$ are *different types* even when $f$ is a bijection. Such a type theory would violate the basic substitution principle.

This shows that "substitution = pullback" is not just a convenience — it is a soundness condition. A categorical semantics that fails this condition would produce a type theory where basic logical principles fail.

## 6. The ∞-Topos and the Axiom of Choice

In classical set theory, the Axiom of Choice (AC) says: every surjection has a section. This is equivalent to many things: Zorn's Lemma, the Well-Ordering Theorem, the fact that every surjection splits.

In an ∞-topos, there are different versions of "the Axiom of Choice":
1. **AC:** Every surjection $f : A \twoheadrightarrow B$ (epimorphism of ∞-groupoids) has a section $s : B \to A$ with $f \circ s = \mathsf{id}_B$.
2. **Dependent Choice:** A weaker version for countably iterated choices.
3. **AC for 0-truncated objects:** AC restricted to maps between sets (0-truncated types).

HoTT's relationship to AC is subtle. The univalence axiom does not imply AC (by Diaconescu's theorem applied to ∞-toposes, AC would imply LEM in many contexts). But HoTT is compatible with AC: you can add AC as an axiom without inconsistency.

The ∞-topos of ∞-groupoids (the "basic" ∞-topos for HoTT) does not satisfy AC in general: there are surjections between Kan complexes with no continuous sections. But the subcategory of discrete Kan complexes (sets) does satisfy AC (in $\mathbf{Set}$, the Axiom of Choice holds).

The question: is HoTT's relationship to AC more like classical logic (where AC is "true") or like sheaf logic (where AC can fail for geometric reasons)? And does the ∞-topos perspective clarify when AC should and should not be assumed in homotopy-theoretic mathematics?
