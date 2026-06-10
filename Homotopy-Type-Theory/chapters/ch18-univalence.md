# Chapter 18: Equivalences and the Univalence Axiom

## Introduction

The univalence axiom is the defining innovation of Homotopy Type Theory. It states:

> **Equivalent types are equal.** $(A \simeq B) \simeq (A =_\mathsf{Type} B)$

This single axiom transforms type theory from a foundational system for mathematics into a homotopy-theoretic foundation that respects mathematical practice. Mathematicians routinely identify isomorphic groups, homeomorphic spaces, and equivalent categories — univalence says this identification is *literally valid* at the level of the type theory.

But before we can state univalence precisely, we need to understand *equivalence* carefully. The naive notion (bijection) does not work in HoTT — it is not a proposition. The correct notion requires a subtler definition.

---

## 1. What is Equivalence?

### 1.1 The Problem with Quasi-Equivalences

The naive definition of a bijection uses a single inverse:

**Definition 18.1 (Quasi-Equivalence / Quasi-Inverse).** A function $f : A \to B$ is a *quasi-equivalence* if there exists $g : B \to A$ with:
$$\eta : \Pi_{x:A}\, g(f(x)) = x \qquad \varepsilon : \Pi_{y:B}\, f(g(y)) = y$$

The triple $(g, \eta, \varepsilon)$ is called a *quasi-inverse* of $f$.

**Problem:** The type of quasi-inverses is **not** a proposition. Two different triples $(g_1, \eta_1, \varepsilon_1)$ and $(g_2, \eta_2, \varepsilon_2)$ may be elements of the type of quasi-inverses, without being equal.

**Why this matters:** We want $\mathsf{isEquiv}(f)$ to be a proposition (a mere proposition in the h-level sense). If the type of witnesses of equivalence has multiple inequivalent elements, then "being an equivalence" would not be a property of $f$ but additional structure — and paths in the type of equivalences would be more complicated than needed.

### 1.2 The Correct Definition: Bi-Invertible Maps

**Definition 18.2 (Bi-Invertible Map).** A function $f : A \to B$ is a *bi-invertible map* if it has both a left inverse and a right inverse:
$$\mathsf{isEquiv}(f) :\equiv \left(\Sigma_{g:B\to A}\, \Pi_{x:A}\, g(f(x)) = x\right) \times \left(\Sigma_{h:B\to A}\, \Pi_{y:B}\, f(h(y)) = y\right)$$

Note: the left inverse $g$ and right inverse $h$ need not be the same function.

**Theorem 18.3.** $\mathsf{isEquiv}(f)$ is a mere proposition (h-prop).

*Proof sketch.* The two components $\Sigma_{g} \Pi \ldots$ are h-props (the fiber of the "section" map is contractible when $f$ is already a left inverse), and the product of two h-props is an h-prop. $\square$

### 1.3 Alternative: Half-Adjoint Equivalences

Another correct definition uses a *coherence condition*:

**Definition 18.4 (Half-Adjoint Equivalence).** A function $f : A \to B$ is a *half-adjoint equivalence* if there exist $g : B \to A$, $\eta : g \circ f \sim \mathsf{id}_A$, $\varepsilon : f \circ g \sim \mathsf{id}_B$, and a *coherence witness*:
$$\tau : \Pi_{x:A}\, \mathsf{ap}_f(\eta_x) = \varepsilon_{f(x)}$$

The coherence condition says: the two ways of showing $f \circ g \circ f \sim f$ (via $\eta$ then $\mathsf{ap}$, or via $\varepsilon$) agree.

**Theorem 18.5.** The three definitions (bi-invertible, half-adjoint, contractible fibers — see below) are all equivalent.

### 1.4 Equivalence via Contractible Fibers

**Definition 18.6 (Contractible Fibers).** The *fiber* of $f : A \to B$ over $y : B$ is:
$$\mathsf{fib}_f(y) :\equiv \Sigma_{x:A}\, f(x) = y$$

$f$ is an *equivalence* if all its fibers are contractible:
$$\mathsf{isEquiv}(f) :\equiv \Pi_{y:B}\, \mathsf{isContr}(\mathsf{fib}_f(y))$$

**Theorem 18.7.** This is equivalent to the bi-invertible definition.

**Intuition:** $f$ is an equivalence iff every $y : B$ has exactly one preimage (the fiber is contractible — a single element with all other "elements" path-equal to it). This is the type-theoretic version of "bijection."

---

## 2. The Type of Equivalences

**Definition 18.8.** The *type of equivalences* from $A$ to $B$ is:
$$A \simeq B :\equiv \Sigma_{f : A \to B}\, \mathsf{isEquiv}(f)$$

An element of $A \simeq B$ is a pair $(f, e)$ where $f : A \to B$ is a function and $e : \mathsf{isEquiv}(f)$ is a proof that $f$ is an equivalence.

**Theorem 18.9 (Equivalences form a Groupoid).** Equivalences compose, and:
- $\mathsf{id}_A : A \simeq A$ (identity is an equivalence)
- If $e : A \simeq B$, then $e^{-1} : B \simeq A$ (equivalences have inverses)
- Composition of equivalences is an equivalence

**Theorem 18.10 (Two-out-of-three).** In any composable triple $A \xrightarrow{f} B \xrightarrow{g} C$: if any two of $f$, $g$, $g \circ f$ are equivalences, so is the third.

---

## 3. The Univalence Axiom

We are ready to state the central axiom.

**The transport map:** For any two types $A, B : \mathsf{Type}$, there is a canonical function:
$$\mathsf{idToEquiv} : (A =_\mathsf{Type} B) \to (A \simeq B)$$
defined by: $\mathsf{idToEquiv}(\mathsf{refl}_A) = \mathsf{id}_A$.

(This uses the J rule: it suffices to define the case for $\mathsf{refl}$, which sends the reflexive equality to the identity equivalence.)

**Axiom 18.11 (Univalence Axiom, Voevodsky).** The map $\mathsf{idToEquiv} : (A = B) \to (A \simeq B)$ is an equivalence for all types $A, B : \mathsf{Type}$.

Equivalently:
$$(A =_\mathsf{Type} B) \simeq (A \simeq B)$$

The inverse function $\mathsf{ua} : (A \simeq B) \to (A = B)$ takes an equivalence and produces a path between types.

### 3.1 What Univalence Says

Univalence makes two non-obvious claims:

1. **Every path between types is an equivalence.** (The function $\mathsf{idToEquiv}$ is a surjection.) This says: paths in the universe correspond exactly to equivalences.

2. **Every equivalence gives a path between types.** (The function $\mathsf{ua}$ exists and is a section of $\mathsf{idToEquiv}$.) This says: if two types are equivalent, they are equal.

**Together:** Equality of types is the same as equivalence of types.

### 3.2 The Computation Rule

The computation rule for univalence:
$$\mathsf{idToEquiv}(\mathsf{ua}(e)) = e$$

and

$$\mathsf{transport}^{\mathsf{id}_\mathsf{Type}}(\mathsf{ua}(e)) = e.1$$

The second says: transporting along the path $\mathsf{ua}(e)$ (where $e : A \simeq B$) is the same as applying the underlying function $e.1 : A \to B$.

### 3.3 Why Univalence is Not Provable in MLTT

In the simplicial set model, types are Kan complexes, and the universe $\mathsf{Type}$ is a Kan complex whose points are Kan complexes and whose path space between two Kan complexes is their space of equivalences. This is a non-trivial geometric fact — the universe of spaces has the "right" topology.

In MLTT without univalence, the term $\mathsf{ua}$ cannot be defined. There is a model (the *setoid model*) where all types are sets and all paths between types are identities — in this model, univalence fails because $A \simeq B$ can be non-trivial even when $A = B$ is a mere proposition (since the only path is $\mathsf{refl}$, but there may be many equivalences).

---

## 4. Consequences of Univalence

### 4.1 Function Extensionality

**Theorem 18.12 (Funext from Univalence).** Univalence implies function extensionality.

*Proof sketch.* For $f, g : A \to B$, a homotopy $H : f \sim g$ defines an equivalence between the identity types of the Kan complex of functions... (The full proof uses the fact that paths in the function type $A \to B$ correspond to homotopies, which follows from the path-space computation for function types in the simplicial set model.) $\square$

### 4.2 Propositional Extensionality

**Theorem 18.13 (Propext from Univalence).** For mere propositions $P, Q$: $(P \simeq Q) \to (P = Q)$.

Since $P$ and $Q$ are propositions, $P \simeq Q$ iff $(P \to Q) \times (Q \to P)$ — they are logically equivalent. So univalence implies: logically equivalent propositions are equal. This is propositional extensionality.

### 4.3 Structure Invariance

**Theorem 18.14 (Structure Invariance / Univalent Mathematics).** Any property of a type $A$ that can be stated in type theory is invariant under equivalence.

More formally: for any type-theoretic predicate $P : \mathsf{Type} \to \mathsf{Type}$, if $e : A \simeq B$ then $\mathsf{transport}^P(\mathsf{ua}(e)) : P(A) \simeq P(B)$.

**What this means for mathematics:**

1. If $G_1 \cong G_2$ are isomorphic groups (as types), then any group-theoretic property of $G_1$ holds for $G_2$ (since group-theoretic properties are expressed in terms of the type).

2. If two constructions of $\mathbb{R}$ (Dedekind cuts vs. Cauchy sequences) are equivalent, they are equal as types — so any statement about $\mathbb{R}$ holds for both simultaneously.

3. Any mathematical theorem that holds for a type $A$ automatically holds for any equivalent type $B$.

This is the formal version of the mathematical intuition that "isomorphic objects have the same properties."

### 4.4 The Univalence Principle (Ahrens-Kapulkin-Shulman)

**Theorem 18.15 (Univalence Principle, AKS).** A statement in type theory is preserved by equivalence of types iff it is stated in the "correct" way — using equivalences (not equalities) as the notion of sameness for types.

This theorem validates the claim that univalence "does the right thing": it makes the formal notion of equality line up with the mathematical notion of sameness.

---

## 5. Examples of Paths Between Types

### 5.1 Paths from Isomorphisms

If $f : G_1 \cong G_2$ is a group isomorphism (viewing groups as types with structure), then $\mathsf{ua}(f) : G_1 = G_2$ — the groups are literally equal as types.

**Warning:** This requires the type-theoretic definition of groups. The "type" of a group is $\Sigma_{G:\mathsf{Type}} \mathsf{GroupStr}(G)$, and the path between two groups requires not just a bijection but an equivalence respecting the group structure.

### 5.2 Paths Between Propositions

If $P$ and $Q$ are propositions and $P \leftrightarrow Q$ (logically equivalent), then $\mathsf{ua}(\text{logical equivalence}) : P = Q$.

### 5.3 The Two Equivalences of $\mathsf{Bool}$

$\mathsf{Bool} = \{0, 1\}$ has exactly two self-equivalences:
- $\mathsf{id} : \mathsf{Bool} \simeq \mathsf{Bool}$ (the identity)
- $\mathsf{neg} : \mathsf{Bool} \simeq \mathsf{Bool}$ (swap 0 and 1)

By univalence, $\mathsf{Bool} =_\mathsf{Type} \mathsf{Bool}$ has exactly two paths:
- $\mathsf{ua}(\mathsf{id}) = \mathsf{refl}$
- $\mathsf{ua}(\mathsf{neg})$: a non-trivial loop in the universe at $\mathsf{Bool}$

This demonstrates that the universe is *not* a set: it has non-trivial paths (between a type and itself).

---

## 6. Why Univalence Is Not Provable Without Extra Axioms

**Theorem 18.16 (Streicher).** There exists a model of MLTT (the *setoid model*) in which:
- All types are sets (h-level 0)
- All paths between types are equalities
- But many types have non-trivial equivalences

In this model, univalence fails because $A \simeq B$ may have multiple elements but $A = B$ is a proposition.

**The simplicial set model** provides the *correct* model: in it, the universe $\mathsf{Type}$ is a Kan complex whose path space between $A$ and $B$ is equivalent to the space of equivalences between them (as Kan complexes). This makes univalence a theorem in the model.

---

## Exercises

**18.1.** Show that the identity function $\mathsf{id}_A : A \to A$ is an equivalence (in all three senses: quasi-inverse, bi-invertible, contractible fibers).

**18.2.** Show that the composition of two equivalences is an equivalence.

**18.3.** Show that the type $A \simeq B$ is a proposition when $A$ and $B$ are propositions (so that equivalence of propositions is again a proposition).

**18.4.** Using univalence, show: if $A$ and $B$ are sets with the same cardinality (a bijection between them), then $A = B$ as types.

**18.5.** Show that the type of self-equivalences $A \simeq A$ forms a group (the *automorphism group* of $A$). For $A = \mathsf{Bool}$, show this group is $\mathbb{Z}/2\mathbb{Z}$.

**18.6.** Using univalence, construct a non-trivial path $\mathsf{Bool} = \mathsf{Bool}$ (the path corresponding to the negation equivalence). Show that this path is different from $\mathsf{refl}$.

**18.7.** Formalize the statement "any property of types is invariant under equivalence" in dependent type theory, and show how univalence implies it.

**18.8 (Challenge).** The *J rule for the universe*: univalence gives us a J-like rule for paths in $\mathsf{Type}$. Specifically, to prove a property of $e : A = B$ for all $A, B : \mathsf{Type}$, it suffices to prove it for $A = B$ an equivalence $A \simeq B$... which reduces to the case $A = B$ and $e = \mathsf{refl}$ (by an argument similar to path induction). Make this precise and prove it.
