# 4.1 The Quillen Model Structure on Simplicial Sets

## What is a Model Structure?

A *model structure* on a category is a framework for doing homotopy theory. It specifies three classes of morphisms — weak equivalences, fibrations, and cofibrations — and axioms governing their interaction. Once you have a model structure, you can "invert" the weak equivalences and work in the resulting homotopy category.

Model structures were introduced by Quillen (1967) specifically to axiomatize the key properties of homotopy theory, allowing the same techniques to be applied in many different settings.

**Definition 4.1 (Model Category).** A *model category* is a category $\mathcal{C}$ with three distinguished classes of morphisms:
- **Weak equivalences** ($\xrightarrow{\sim}$): "essentially the same" maps
- **Fibrations** ($\twoheadrightarrow$): maps with "nice lifting properties" (right lifting property against acyclic cofibrations)
- **Cofibrations** ($\hookrightarrow$): maps with "nice extension properties" (left lifting property against acyclic fibrations)

satisfying:
1. **2-of-3:** If two of $f, g, g \circ f$ are weak equivalences, so is the third
2. **Retracts:** All three classes are closed under retracts
3. **Lifting:** Cofibrations have left lifting property against acyclic fibrations; acyclic cofibrations have left lifting property against fibrations
4. **Factorization:** Every map factors as (cofibration followed by acyclic fibration) and as (acyclic cofibration followed by fibration)

A *fibrant object* is one where $X \to *$ is a fibration. A *cofibrant object* is one where $\emptyset \to X$ is a cofibration.

## The Kan-Quillen Model Structure

**Theorem 4.2 (Quillen, 1967).** The category $\mathbf{sSet}$ of simplicial sets carries a model structure where:

**Weak equivalences:** $f : X \to Y$ is a weak equivalence iff $|f| : |X| \to |Y|$ is a weak homotopy equivalence of topological spaces (induces isomorphisms on all homotopy groups $\pi_n$ for all basepoints).

**Fibrations (Kan fibrations):** $p : X \to Y$ is a Kan fibration iff it has the *right lifting property* against all horn inclusions $\Lambda^n_k \hookrightarrow \Delta[n]$: for every commutative square
$$\begin{array}{ccc}
\Lambda^n_k & \to & X \\
\downarrow & \nearrow & \downarrow p \\
\Delta[n] & \to & Y
\end{array}$$
there exists a lift $\Delta[n] \to X$ making both triangles commute.

**Cofibrations:** $i : A \to B$ is a cofibration iff it is a *monomorphism* (injective on all $n$-simplices for all $n$).

**Fibrant objects:** $X$ is fibrant (i.e., $X \to *$ is a Kan fibration) iff $X$ is a Kan complex. This matches our earlier definition exactly: Kan complexes are exactly the fibrant objects.

**Cofibrant objects:** Every object is cofibrant (since any map from $\emptyset$ is vacuously injective).

## The Lifting Property: A Closer Look

The key structure is the *lifting property*. Let's spell out what a Kan fibration is concretely.

$p : X \to Y$ is a Kan fibration if: given any horn $\Lambda^n_k \to X$ that projects down to a filled simplex $\Delta[n] \to Y$ (i.e., the top row is a horn, but the bottom is already filled), we can fill the horn in $X$ consistently.

$$\begin{array}{ccc}
\Lambda^n_k & \xrightarrow{f} & X \\
\downarrow & \nearrow \exists\tilde{g} & \downarrow p \\
\Delta[n] & \xrightarrow{g} & Y
\end{array}$$

**Intuition:** The base $Y$ tells you what the "type" is; the total space $X$ is a fibration over $Y$. Given data in $X$ over a horn in $Y$, and a filling of the horn in $Y$, you can lift the filling to $X$. This is the type-theoretic property of transport: given a path in the base, you can transport fibers along it.

**Kan complexes as fibrant objects.** A Kan complex $X$ is a Kan fibration over the terminal simplicial set $\{*\}$ — every horn in $X$ fills (with no constraint from above since $\{*\}$ has no non-trivial simplices). This matches the definition.

## Acyclic Maps and Quillen Equivalence

**Acyclic fibration:** A fibration that's also a weak equivalence. These are maps with the right lifting property against all boundary inclusions $\partial\Delta[n] \hookrightarrow \Delta[n]$.

**Acyclic cofibration:** A cofibration that's also a weak equivalence. These are monomorphisms that are weak homotopy equivalences.

**The fundamental adjunction:**

$$|-| : \mathbf{sSet} \rightleftarrows \mathbf{Top} : \mathsf{Sing}$$

This adjunction is not just any adjunction — it's a *Quillen adjunction* (both functors preserve the relevant parts of the model structure) and in fact a *Quillen equivalence* (the induced functors on homotopy categories are inverse equivalences).

**Theorem 4.3 (Quillen equivalence).** The adjunction $|-| \dashv \mathsf{Sing}$ is a Quillen equivalence. This means:
- For a cofibrant simplicial set $X$ (any simplicial set, since all are cofibrant) and a fibrant simplicial set $K$ (= a Kan complex): a map $|X| \to Y$ in **Top** corresponds to a map $X \to \mathsf{Sing}(Y)$ in **sSet**, and both are weak equivalences iff the other is
- The unit $X \to \mathsf{Sing}(|X|)$ and counit $|\mathsf{Sing}(Y)| \to Y$ are weak equivalences (for CW complexes $Y$)

## Fibrations as "Dependent Types"

The model structure on **sSet** mirrors the structure of dependent type theory in a precise way.

| Model Structure | Type Theory |
|---|---|
| Fibrant objects (Kan complexes) | Types |
| Kan fibration $E \to B$ | Type family $B : A \to \mathsf{Type}$ |
| Section of $E \to B$ | Term of type $B(a)$ for $a : A$ |
| Pullback of fibration | Substitution |
| Acyclic fibration | Contractible fibers (trivial type family) |

A Kan fibration $p : E \to B$ is the "right" notion of a "family of types over $B$": for each 0-simplex $b \in B_0$, the fiber $p^{-1}(b)$ is a Kan complex (a type), and these fibers vary "continuously" (in the homotopy-theoretic sense) as $b$ varies.

Transport along a path corresponds to the homotopy lifting property: given $p : a = a'$ (a 1-simplex in $B$) and an element $x$ of the fiber over $a$, the horn-filling lifts $p$ to a path in $E$, giving an element over $a'$.

## The Homotopy Category and Localization

The *homotopy category* of $\mathbf{sSet}$ is obtained by "inverting" the weak equivalences:
$$\mathsf{Ho}(\mathbf{sSet}) = \mathbf{sSet}[W^{-1}]$$

where $W$ is the class of weak equivalences.

**Theorem 4.4.** The homotopy category of simplicial sets is equivalent to the homotopy category of topological spaces:
$$\mathsf{Ho}(\mathbf{sSet}) \simeq \mathsf{Ho}(\mathbf{Top})$$

Both are equivalent to the "homotopy category of homotopy types" — the category where objects are homotopy types and morphisms are homotopy classes of maps.

## Minimal Fibrations

A special class of Kan fibrations that's particularly useful:

**Definition 4.5 (Minimal Fibration).** A Kan fibration $p : E \to B$ is *minimal* if for any two 1-simplices $f, g$ in $E$ with $\partial_0 f = \partial_0 g$ and $\partial_1 f = \partial_1 g$ (same endpoints) and $p(f) = p(g)$ (same image in $B$), if $f$ and $g$ are homotopic rel endpoints (via a 2-simplex in $E$), then $f = g$.

Every Kan fibration is (weakly) equivalent to a minimal one. Minimal fibrations are unique representatives in their homotopy class.

## The Universe Object

For the HoTT model, the most important object in $\mathbf{sSet}$ is the *universe* — a Kan complex $\hat{U}$ that "classifies" small Kan fibrations.

**Theorem 4.5 (Voevodsky).** There exists a Kan complex $\hat{U}$ (the universe) and a universal Kan fibration $\tilde{U} \to \hat{U}$ such that:
- Every small Kan fibration $E \to B$ is (up to homotopy) a pullback of $\tilde{U} \to \hat{U}$
- The path space of $\hat{U}$ is the space of equivalences: paths in $\hat{U}$ correspond to equivalences of Kan complexes

The last point is the content of Univalence: **paths in the universe = equivalences of types**.

This is a non-trivial theorem about the simplicial set model. It uses the fact that the "universe of Kan complexes" can be built from small Kan fibrations, and the path space of this universe has the right shape.

## Summary

| Concept | Model structure | Homotopy theory |
|---|---|---|
| Weak equivalence | Induces $\pi_n$ isomorphisms | "Same homotopy type" |
| Kan fibration | Right lifting against horns | "Fibration" in topology |
| Cofibration (mono) | Injective on all simplices | "Cofibration" |
| Fibrant object | Kan complex | "Good representative" |
| Quillen equivalence | $|-| \dashv \mathsf{Sing}$ | Equivalent homotopy theories |
| Universe $\hat{U}$ | Classifies Kan fibrations | Type of types in HoTT |

The Quillen model structure on simplicial sets is one of the most important structures in mathematics. It provides the precise foundation for the claim that "simplicial sets are as good as topological spaces for homotopy theory" — and it's the foundation of Voevodsky's consistency proof for HoTT.
