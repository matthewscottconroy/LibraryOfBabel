# 5.1 Voevodsky's Simplicial Set Model of HoTT

## The Consistency Problem

Before Voevodsky's work, HoTT (or more precisely, Martin-Löf Type Theory plus the Univalence Axiom) was a theory we *hoped* was consistent. We had strong reasons to believe it was — it built on well-established foundations, and no one had found a contradiction. But "no one has found a contradiction yet" is not a proof of consistency.

Voevodsky's great contribution (2006-2012) was to construct a *model* of HoTT in an established mathematical framework — simplicial sets — thereby proving relative consistency: if ZFC set theory (with some large cardinal axioms) is consistent, then so is HoTT plus Univalence.

The model lives in the category of Kan simplicial sets. Every type-theoretic construction — Π types, Σ types, identity types, universes — is interpreted as a specific simplicial set construction. And Univalence, instead of being an additional axiom, is a *theorem* in this model.

## The Basic Interpretation

Here's the systematic dictionary from HoTT to simplicial sets:

**Types:** A closed type $A$ is interpreted as a Kan complex $\llbracket A \rrbracket$.

**Terms:** A term $a : A$ is a 0-simplex $\llbracket a \rrbracket \in \llbracket A \rrbracket_0$.

**Open types (in context):** A type $\Gamma \vdash A\ \mathsf{type}$ is interpreted as a Kan fibration $\llbracket A \rrbracket \to \llbracket \Gamma \rrbracket$.

**Terms in context:** $\Gamma \vdash a : A$ is a section of the Kan fibration $\llbracket A \rrbracket \to \llbracket \Gamma \rrbracket$.

**Substitution:** $A[\sigma]$ for $\sigma : \Delta \to \Gamma$ is the pullback $\sigma^*\llbracket A \rrbracket$.

## Type Formers

**$\Pi$ types.** Given a Kan fibration $p : E \to B$ (interpreting $A$ over $\Gamma$) and a Kan fibration $q : F \to E$ (interpreting $B(x)$ over $\Gamma, x:A$), the $\Pi$ type is interpreted by the *right adjoint* to pullback:

$$\llbracket \prod_{x:A} B(x) \rrbracket = \Pi_p(F \to E) \to B$$

This is the "exponential fibration" construction: the fiber over $b \in B$ is the set of sections of $q$ over the fiber $p^{-1}(b)$.

**$\Sigma$ types.** The interpretation is composition of fibrations:

$$\llbracket \sum_{x:A} B(x) \rrbracket = (F \to E \xrightarrow{p} B) = F \to B$$

The total space of the Σ type is the total space of $F$ (the total space of $B$ over $A$ over $\Gamma$), fibered over $\Gamma$.

**The unit type $\mathbf{1}$.** Interpreted as the terminal object (one-point Kan complex). The map $\mathbf{1} \to \Gamma$ is the terminal map (constant at the single point).

**The empty type $\mathbf{0}$.** Interpreted as the empty Kan complex. The map $\mathbf{0} \to \Gamma$ is the initial map from the empty set.

## The Identity Type

This is the heart of the matter. The identity type $a =_A b$ must be interpreted as the "path space" between $a$ and $b$.

**The simplicial path space.** For a Kan complex $\llbracket A \rrbracket$ and points $a, b \in \llbracket A \rrbracket_0$, the *simplicial path space* from $a$ to $b$ is:

$$\llbracket a =_A b \rrbracket = \llbracket A \rrbracket^{\Delta[1]} \times_{\llbracket A \rrbracket \times \llbracket A \rrbracket} \{(a,b)\}$$

In words: the fiber of the map $\llbracket A \rrbracket^{\Delta[1]} \to \llbracket A \rrbracket \times \llbracket A \rrbracket$ (evaluating a path at its endpoints) over the pair $(a,b)$.

Concretely: $n$-simplices of $\llbracket a =_A b \rrbracket$ are $(n+1)$-simplices of $\llbracket A \rrbracket$ with source edge mapping to $a$ and target edge mapping to $b$. These are "paths between paths between ... between $a$ and $b$."

**Reflexivity.** The reflexivity map $\mathsf{refl} : A \to (x =_A x)$ is the diagonal: send $a$ to the constant path at $a$ (the degenerate 1-simplex $\sigma_0(a)$).

**The J rule.** Path induction says: to prove a property $P(a, b, p)$ for all $b : A$ and $p : a = b$, it suffices to prove $P(a, a, \mathsf{refl}_a)$. Categorically, this is the statement that the diagonal $a \to \llbracket a =_A a \rrbracket$ (reflexivity map) is an acyclic cofibration — any map out of $a$ into a fibrant codomain extends over the path space.

In the simplicial model: the factorization $\llbracket A \rrbracket \xrightarrow{r} \llbracket A \rrbracket^{\Delta[1]} \xrightarrow{(s,t)} \llbracket A \rrbracket \times \llbracket A \rrbracket$ is an acyclic cofibration followed by a fibration (this is the path object factorization in the model structure). The J rule is the lifting property of this factorization.

## Why UIP Fails

The simplicial model shows concretely why the univalence principle of identity proofs (UIP) is not provable in MLTT.

UIP says: any two paths $p, q : a =_A b$ are equal. In the simplicial model, this would say: any two 1-simplices in $\llbracket A \rrbracket$ with the same endpoints are equal as elements of $\llbracket A \rrbracket_1$. But in a non-trivial Kan complex, there can be many distinct 1-simplices between the same pair of vertices.

**Example.** The simplicial circle $S^1$ (as a Kan complex): there are infinitely many distinct 1-simplices from the basepoint to itself (one for each element of $\pi_1(S^1) = \mathbb{Z}$). So the type $\mathsf{base} =_{S^1} \mathsf{base}$ has many distinct elements.

This is the model-theoretic proof that UIP is not provable: the simplicial set model validates all the rules of MLTT but not UIP.

## Universes and Univalence

**Universes.** The universe $\mathsf{Type}$ in HoTT is interpreted as a specific Kan complex $\hat{U}$ — the "universe of small Kan complexes." Roughly:
- 0-simplices of $\hat{U}$: small Kan complexes (types)
- 1-simplices of $\hat{U}$: weak equivalences between Kan complexes (paths)
- Higher simplices: higher homotopies between equivalences

The precise construction uses the fact that there's a universal small Kan fibration $\tilde{U} \to \hat{U}$ classifying all small Kan fibrations.

**Univalence.** The Univalence axiom says:

$$\mathsf{ua} : (A \simeq B) \simeq (A =_{\mathsf{Type}} B)$$

In the simplicial model:
- $A =_{\mathsf{Type}} B$ is a path in $\hat{U}$ from $A$ to $B$
- A path in $\hat{U}$ from $A$ to $B$ is (by the construction of $\hat{U}$) a weak equivalence from $A$ to $B$
- A weak equivalence of Kan complexes = a homotopy equivalence = an equivalence of types

So: paths in the universe = equivalences of types. That's Univalence!

**Theorem 5.1 (Voevodsky).** In the Kan simplicial set model, the Univalence Axiom is a *theorem* (not an extra assumption).

This is the most important theorem in the whole story: Univalence is not just a convenient axiom — it's a mathematical theorem about the simplicial world.

## Higher Inductive Types

HITs also have interpretations in the simplicial model.

**The circle $S^1$.** Interpreted as the simplicial circle: a Kan complex with:
- Exactly one 0-simplex (the basepoint)
- Non-degenerate 1-simplices corresponding to elements of $\mathbb{Z}$ (the loops)
- 2-simplices and higher: the composites and coherences

More concretely: $S^1 = N(\mathbf{B}\mathbb{Z})$ — the nerve of the one-object groupoid with automorphisms $\mathbb{Z}$.

**General HITs.** The interpretation uses "Reedy fibrant replacement" and related constructions. A HIT is defined by its generators (constructors), and the simplicial model provides a specific Kan complex realizing those generators.

**The universe of HITs.** HITs live in the same universe as ordinary inductive types — the same Kan complex $\hat{U}$. There's no need for a separate universe.

## The Cubical Set Alternative

Voevodsky's simplicial model proves consistency, but it has a limitation: it doesn't give a *computational* interpretation. Univalence is a theorem, but there's no canonical "computation rule" for it in the simplicial model.

This motivated the development of *cubical type theory* (Chapters 23), where:
- The model is cubical sets (presheaves on the cube category)
- Univalence holds by construction and has explicit computation rules
- Function extensionality holds *definitionally* (not just propositionally)

Cubical type theory is in some ways "better" for computation; simplicial type theory is "better" for mathematics (because of the cleaner connection to classical homotopy theory). Both are valuable.

## Summary

| HoTT Construction | Simplicial Set Interpretation |
|---|---|
| Closed type $A$ | Kan complex $\llbracket A \rrbracket$ |
| Term $a : A$ | 0-simplex in $\llbracket A \rrbracket$ |
| Type family $B : A \to \mathsf{Type}$ | Kan fibration over $\llbracket A \rrbracket$ |
| $\Pi$ type | Right adjoint to pullback |
| $\Sigma$ type | Composition of fibrations |
| Identity type $a = b$ | Simplicial path space |
| Reflexivity | Degenerate simplex |
| J rule | Lifting property of path object |
| Universe $\mathsf{Type}$ | Universe Kan complex $\hat{U}$ |
| Univalence | Paths in $\hat{U}$ = equivalences (theorem) |
| HITs | Reedy fibrant replacements |

The simplicial set model is the proof that HoTT works. It shows that all the rules of HoTT are consistent (relative to ZFC), that Univalence is mathematically true (not just assumed), and that HITs have legitimate interpretations. Everything is consistent, and the theory is powerful enough to formalize all of mathematics.

This is the mathematical foundation beneath all of HoTT. The rest of the curriculum develops the type theory and its applications, but this chapter is why we know the whole edifice stands on solid ground.
