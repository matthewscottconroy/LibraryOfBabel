# The Model Structure on Simplicial Sets

## Model Categories: The Abstract Setting

A *model category* is a category equipped with three distinguished classes of morphisms — called *cofibrations*, *fibrations*, and *weak equivalences* — satisfying axioms that allow one to do homotopy theory. The axioms ensure the existence of a well-behaved *homotopy category* (obtained by inverting the weak equivalences) and the tools needed to compute in it: lifting properties, factorization theorems, and the machinery of derived functors.

The concept was introduced by Quillen in 1967 as a way of doing homotopy theory in contexts beyond topological spaces: in simplicial sets, chain complexes, categories, and many other settings. The key insight is that all of these contexts share the same abstract structure.

**Definition.** A *Quillen model structure* on a category $\mathcal{C}$ consists of three classes of morphisms — $\mathsf{Cof}$ (cofibrations), $\mathsf{Fib}$ (fibrations), $\mathsf{W}$ (weak equivalences) — satisfying:
1. **Two-out-of-three:** If any two of $f$, $g$, $g \circ f$ are in $\mathsf{W}$, so is the third.
2. **Retracts:** Each of $\mathsf{Cof}$, $\mathsf{Fib}$, $\mathsf{W}$ is closed under retracts.
3. **Lifting:** Acyclic cofibrations (cofibrations in $\mathsf{W}$) have the left lifting property with respect to fibrations; cofibrations have the left lifting property with respect to acyclic fibrations (fibrations in $\mathsf{W}$).
4. **Factorization:** Every morphism factors as an acyclic cofibration followed by a fibration, and also as a cofibration followed by an acyclic fibration.

(Here "left lifting property" means: given a commutative square with the left morphism in $\mathsf{Cof}$ and the right morphism in $\mathsf{Fib}$, there exists a diagonal filler making both triangles commute.)

## The Quillen Model Structure on $\mathbf{sSet}$

The category $\mathbf{sSet}$ of simplicial sets carries a canonical model structure, called the *Quillen model structure* (or the *Kan-Quillen model structure*):

- **Cofibrations:** Monomorphisms of simplicial sets (injective maps on all $X_n$).
- **Fibrations:** Kan fibrations (maps satisfying the horn-lifting condition).
- **Weak equivalences:** Maps $f : X \to Y$ such that $|f| : |X| \to |Y|$ is a weak homotopy equivalence (inducing isomorphisms on all homotopy groups).

This is a clean and elegant model structure:
- Cofibrations are the simplest possible: just injections.
- Fibrations are the Kan fibrations: the horn-lifting condition is explicit and checkable.
- Weak equivalences are defined via the geometric realization, which connects back to classical topology.

**Theorem (Quillen, 1967).** The above data defines a model structure on $\mathbf{sSet}$.

The verification of the axioms is non-trivial, particularly the factorization axiom. The key technical tool is the *small object argument*: given a set of maps with the left lifting property, you can factor any map as a transfinite composition of pushouts of those maps followed by a map with the right lifting property. Applied to the horn inclusions $\{\Lambda^n_k \hookrightarrow \Delta[n]\}$, this gives the factorization of any map as a (acyclic cofibration, Kan fibration) pair.

## Fibrant and Cofibrant Objects

In a model category, objects play special roles:
- A *fibrant* object is one for which the map to the terminal object $X \to *$ is a fibration.
- A *cofibrant* object is one for which the map from the initial object $\emptyset \to X$ is a cofibration.

In $\mathbf{sSet}$:
- Fibrant objects are the Kan complexes (the map $X \to *$ being a Kan fibration is exactly the Kan condition).
- Every simplicial set is cofibrant (the map $\emptyset \to X$ is an injection, hence a monomorphism, hence a cofibration).

The fact that every object is cofibrant is a special feature of $\mathbf{sSet}$ — it simplifies many arguments. In the model structure on topological spaces, not every space is cofibrant (only the CW complexes and their retracts are).

## The Quillen Equivalence with Topological Spaces

The category $\mathbf{Top}$ of topological spaces also carries a Quillen model structure:
- **Cofibrations:** Retracts of CW-inclusions (cellular maps).
- **Fibrations:** Serre fibrations (lifting against $|\Lambda^n_k| \hookrightarrow |\Delta^n|$).
- **Weak equivalences:** Weak homotopy equivalences (maps inducing isomorphisms on all $\pi_n$).

**Theorem (Quillen, 1967).** The adjunction $|-| \dashv \text{Sing}$ is a *Quillen equivalence* between $(\mathbf{sSet}, \text{Quillen})$ and $(\mathbf{Top}, \text{Serre})$:
$$|-| : \mathbf{sSet} \rightleftarrows \mathbf{Top} : \text{Sing}$$

A Quillen equivalence is a pair of Quillen adjoint functors (respecting the model structure) that induce an equivalence of homotopy categories. In this case:
- The induced functor on homotopy categories $\mathsf{Ho}(\mathbf{sSet}) \simeq \mathsf{Ho}(\mathbf{Top})$ is an equivalence of categories.
- This equivalence preserves and reflects all homotopy-theoretic information: homotopy groups, cohomology, fibration sequences, etc.

The Quillen equivalence says: for the purposes of homotopy theory, simplicial sets and topological spaces are the same thing. You can compute with whichever is more convenient — the algebraic/combinatorial simplicial sets or the geometric topological spaces — and the results will agree.

## Why This Model Structure Is "Right"

Several features distinguish the Quillen model structure as the canonical one:

**1. It models classical homotopy theory.** The homotopy category $\mathsf{Ho}(\mathbf{sSet})$ is equivalent to the classical homotopy category of topological spaces. Every classical theorem — Whitehead, Hurewicz, long exact sequences — translates into the simplicial setting.

**2. The fibrant objects are the "nice" ones.** Fibrant simplicial sets (Kan complexes) are the simplicial sets with the best homotopy-theoretic properties. Every simplicial set has a fibrant replacement (the Kan completion or $\text{Ex}^\infty$), which is a Kan complex weakly equivalent to it.

**3. The cofibrations are the right ones.** Every monomorphism being a cofibration makes the theory very clean: you never need to worry about whether your maps are cofibrant. This is why the small object argument works so cleanly in $\mathbf{sSet}$.

**4. The universe of Kan complexes is itself a Kan complex.** This is the key property for HoTT: the "universe" of (small) Kan complexes is a Kan complex. Paths in the universe correspond to equivalences of Kan complexes. This is what makes the univalence axiom true in the simplicial set model.

## Homotopy Limit and Colimit

One of the main applications of model categories is the computation of *homotopy limits* and *homotopy colimits* — derived versions of limits and colimits that are homotopy-invariant.

In $\mathbf{sSet}$, the homotopy colimit of a diagram $F : I \to \mathbf{sSet}$ is computed using a simplicial replacement construction (the *bar construction*), and the homotopy limit is computed using the *cobar construction*. These are the right derived functors of $\text{colim}$ and $\lim$ with respect to the model structure.

Homotopy colimits include:
- The homotopy pushout (double mapping cylinder): computes the homotopy type of a pushout, which is not always the pushout in $\mathbf{sSet}$ itself.
- The homotopy pullback: computes the homotopy type of a pullback.
- The Bousfield-Kan homotopy limit spectral sequence: computes the homotopy groups of a homotopy limit.

In HoTT, homotopy pushouts and pullbacks are computed using higher inductive types and the dependent sum and product. The simplicial model structure ensures that these type-theoretic operations compute the correct homotopy-theoretic answers.

## Connection to the Voevodsky Model

The Quillen model structure is the setting for Voevodsky's model of HoTT. In the simplicial set model:
- Types are Kan complexes.
- Type-theoretic operations (Π, Σ, identity types) correspond to operations on Kan complexes that respect the model structure.
- The univalence axiom corresponds to a property of the universe object — the Kan complex of small Kan complexes — in the model structure.

The model structure ensures that the type-theoretic operations are well-defined (preserve weak equivalences) and that the resulting type theory has the correct semantic behavior. The details of this verification are the content of Section 5.
