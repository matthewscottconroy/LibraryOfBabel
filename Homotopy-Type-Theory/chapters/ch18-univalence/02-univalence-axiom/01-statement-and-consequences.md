# 2.1 The Univalence Axiom: Statement and Immediate Consequences

## The Map idToEquiv

Before stating the axiom, let's understand the natural map it's about.

For any two types $A, B : \mathsf{Type}$, there is a canonical function:
$$\mathsf{idToEquiv} : (A =_\mathsf{Type} B) \to (A \simeq B)$$

How is it defined? By path induction (J rule). It suffices to define it on $\mathsf{refl}_A : A = A$:
$$\mathsf{idToEquiv}(\mathsf{refl}_A) :\equiv \mathsf{id}_A$$

The reflexivity path (the trivial path from $A$ to $A$) maps to the identity equivalence. By J, this extends to all paths $p : A = B$.

**What $\mathsf{idToEquiv}$ does.** Given a path $p : A = B$ (a proof that $A$ and $B$ are equal types), $\mathsf{idToEquiv}(p)$ is the equivalence "witnessed by" the path. The underlying function is $\mathsf{transport}^{\mathsf{id}_\mathsf{Type}}(p) : A \to B$ — transporting along the path in the universe.

**Is $\mathsf{idToEquiv}$ an equivalence?** That's exactly what Univalence says!

## The Univalence Axiom

**Axiom 2.1 (Voevodsky, 2006).** The map $\mathsf{idToEquiv} : (A = B) \to (A \simeq B)$ is an equivalence for all types $A, B : \mathsf{Type}$.

Equivalently: there is an equivalence $(A =_\mathsf{Type} B) \simeq (A \simeq B)$.

The inverse to $\mathsf{idToEquiv}$ is written:
$$\mathsf{ua} : (A \simeq B) \to (A =_\mathsf{Type} B)$$

So: given an equivalence between types, we get a path between them in the universe.

**Unfolding the axiom:** Univalence says:
1. Every path $A = B$ gives an equivalence $A \simeq B$ (via $\mathsf{idToEquiv}$) — and these are all the equivalences
2. Every equivalence $A \simeq B$ gives a path $A = B$ (via $\mathsf{ua}$) — and these are all the paths

Together: **equality of types = equivalence of types**.

## The Computation Rules

Since $\mathsf{ua}$ and $\mathsf{idToEquiv}$ are inverse equivalences:
$$\mathsf{idToEquiv}(\mathsf{ua}(e)) = e \quad \text{for all } e : A \simeq B$$
$$\mathsf{ua}(\mathsf{idToEquiv}(p)) = p \quad \text{for all } p : A = B$$

The first says: if you turn an equivalence $e$ into a path via $\mathsf{ua}$, then turn it back into an equivalence, you get $e$ back.

The second says: if you turn a path $p$ into an equivalence, then turn it back into a path, you get $p$ back.

**The transport computation.** A key consequence:
$$\mathsf{transport}^{\mathsf{id}_\mathsf{Type}}(\mathsf{ua}(e), a) = e.1(a)$$

Transporting $a : A$ along the path $\mathsf{ua}(e) : A = B$ gives $e.1(a) : B$ — the result of applying the equivalence function to $a$.

This is crucial: it tells you what $\mathsf{ua}(e)$ *does* to elements. The path in the universe acts on elements by the underlying function of the equivalence.

## Why Univalence is Not Provable From MLTT

The standard MLTT rules (without Univalence) can prove that $\mathsf{idToEquiv}$ is *injective* (different paths give different equivalences), but not that it's *surjective* (every equivalence comes from a path).

In fact, there exist models of MLTT where Univalence fails:

**The setoid model.** Take types to be sets (in the classical sense — no higher homotopy structure), and take paths $A = B$ to mean $A$ and $B$ are literally the same set. Then $A = B$ in the universe has at most one element (since sets are equal or not, no two distinct paths). But $A \simeq B$ can have many elements (all the bijections). So $\mathsf{idToEquiv}$ cannot be an equivalence — the domain is a proposition but the codomain is not.

In the setoid model, $\mathbb{Z}/2\mathbb{Z}$ viewed as a set has two self-bijections (identity and negation), but there's only one path $\mathbb{Z}/2\mathbb{Z} = \mathbb{Z}/2\mathbb{Z}$ (reflexivity). Univalence fails.

**The simplicial set model.** Voevodsky's construction: take types to be Kan complexes, and the universe $\mathsf{Type}$ to be the Kan complex of all small Kan complexes. In this model:
- Paths $A = B$ in the universe correspond to maps $\Delta[1] \to \mathsf{Type}$ with endpoints $A$ and $B$ — which are equivalences of Kan complexes
- The type $A \simeq B$ (equivalences between the Kan complexes) has the same homotopy type as the path space $\mathsf{Type}^{\Delta[1]}_{A,B}$

So Univalence holds in the simplicial model. Voevodsky proved this in 2006, establishing the consistency of Univalence (relative to the consistency of Kan complexes / set theory).

## Univalence as J for the Universe

Here's a beautiful way to see what Univalence does.

The J rule says: to prove something about all paths $p : a = b$ in a type $A$, it suffices to prove it for $\mathsf{refl}_a$.

Univalence gives an analogous rule for paths in the *universe*: to prove something about all paths $p : A = B$ in $\mathsf{Type}$, it suffices to prove it for equivalences $e : A \simeq B$ (which by $\mathsf{ua}$ correspond to all such paths).

More precisely: by Univalence, paths in $\mathsf{Type}$ are equivalences. So to "induct on a path in the universe," we can instead "induct on an equivalence." And since equivalences have a canonical "reflexivity" case (the identity equivalence $\mathsf{id}_A$), this often reduces proofs to the case of the identity.

**Equivalence induction (Theorem 1.15 revisited).** To prove a property $P(A, B, e)$ for all $A, B : \mathsf{Type}$ and $e : A \simeq B$, it suffices to prove $P(A, A, \mathsf{id}_A)$ for all $A$.

*Proof.* By Univalence, $e$ corresponds to a path $\mathsf{ua}(e) : A = B$. By J on this path, reduce to the case $A = B$ and $e = \mathsf{id}_A$. $\square$

## Univalence and the Universe Is Not a Set

**Corollary 2.2.** The universe $\mathsf{Type}$ is not a set.

*Proof.* By Univalence, $(A = A) \simeq (A \simeq A) = \mathsf{Aut}(A)$ for any $A$.

Take $A = \mathsf{Bool}$. Then $\mathsf{Aut}(\mathsf{Bool}) = \mathsf{Bool} \simeq \mathsf{Bool}$ has exactly two elements: $\mathsf{id}$ and $\mathsf{neg}$ (the identity and negation functions).

So $\mathsf{Bool} = \mathsf{Bool}$ in $\mathsf{Type}$ has at least two distinct paths:
- $\mathsf{refl}_\mathsf{Bool} = \mathsf{ua}(\mathsf{id}_\mathsf{Bool})$
- $\mathsf{ua}(\mathsf{neg}) \neq \mathsf{refl}_\mathsf{Bool}$ (since $\mathsf{neg} \neq \mathsf{id}$, and $\mathsf{ua}$ is injective)

Since there are two distinct paths from $\mathsf{Bool}$ to $\mathsf{Bool}$, the identity type $\mathsf{Bool} = \mathsf{Bool}$ is not a proposition. So $\mathsf{Type}$ is not a set. $\square$

This is the type-theoretic version of: the universe of all sets is not itself a set (or at least, its "identity structure" is richer than that of a set).

## What Univalence Does to Mathematics

The practical consequence of Univalence is that any mathematical statement in type theory is *invariant under equivalence of types*.

**Formal statement.** For any type-theoretic predicate $P : \mathsf{Type} \to \mathsf{Type}$ and any equivalence $e : A \simeq B$:
$$\mathsf{transport}^P(\mathsf{ua}(e)) : P(A) \simeq P(B)$$

So transporting along the path $\mathsf{ua}(e)$ gives an equivalence between $P(A)$ and $P(B)$.

**Why this matters.** In classical mathematics, when we say "$G_1 \cong G_2$ are isomorphic groups, so $G_1$ is abelian iff $G_2$ is abelian," we're relying on the informal principle that isomorphic objects have the same properties. With Univalence, this is *literal*: $G_1 = G_2$ (they're equal as types, once we include the group structure), so any statement that is invariant under transport — which all type-theoretic statements are — holds for both.

**The key point:** Univalence doesn't just say "isomorphic things have the same properties." It says isomorphic things *are equal*, and so by path induction (or just substitution), all properties automatically transfer. There's no need to separately prove invariance.

## The Computation Rule in Practice

Let's see the computation rule $\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$ in action.

**Example 2.3 (Transporting a natural number).** Suppose $e : \mathbb{N} \simeq \mathbb{N}$ is the equivalence $e(n) = n + 1$ (shift by 1, which is an equivalence since $n \mapsto n - 1$ is its inverse, with appropriate natural number subtraction). 

Then $\mathsf{ua}(e) : \mathbb{N} = \mathbb{N}$ is a path in the universe. For $n : \mathbb{N}$:
$$\mathsf{transport}^{\mathsf{id}}(\mathsf{ua}(e), n) = e(n) = n + 1$$

So transporting along this path shifts the natural number by 1.

**Example 2.4 (Transporting Boolean negation).** The path $\mathsf{ua}(\mathsf{neg}) : \mathsf{Bool} = \mathsf{Bool}$ transports $\mathsf{true}$ to $\mathsf{neg}(\mathsf{true}) = \mathsf{false}$ and vice versa.

So "following the path" in the universe literally computes the equivalence function.

**Example 2.5 (Transporting structure).** If $G$ is a group and $e : G \simeq G'$ is a group isomorphism (more precisely, an equivalence of types respecting the group structure), then $\mathsf{ua}(e) : G = G'$, and transporting the group multiplication $\cdot_G : G \to G \to G$ along this path gives the group multiplication $\cdot_{G'} : G' \to G' \to G'$.

This is the formal expression of "isomorphic groups have the same multiplication table (up to relabeling)."

## Propositional Extensionality as a Special Case

**Theorem 2.6 (Propositional Extensionality).** For propositions $P, Q : \mathsf{hProp}$:
$$(P \simeq Q) \simeq (P = Q)$$

*Proof.* Since $P$ and $Q$ are propositions, $P \simeq Q$ iff $P \to Q$ and $Q \to P$ (any function between propositions is automatically an equivalence, since all elements are equal). So $P \simeq Q \simeq (P \leftrightarrow Q)$. By Univalence, $P \simeq Q \simeq (P = Q)$. $\square$

**What this says:** Logically equivalent propositions are equal. This is propositional extensionality — a classical principle that holds in all reasonable foundations. Univalence implies it automatically.

**Note:** In Lean 4, propositional extensionality is an axiom (`propext`) taken separately. In HoTT, it's a consequence of Univalence.

## Function Extensionality as a Consequence

**Theorem 2.7 (Function Extensionality from Univalence).** Univalence implies function extensionality:
$$(f = g) \simeq \prod_{x:A} f(x) = g(x)$$

The proof is less direct than propositional extensionality. The key steps:
1. Consider the type family $x \mapsto B$ (constant family at $B$) and the path $\mathsf{ua}(e)$ for some equivalence $e : B \simeq B$.
2. Functions $A \to B$ can be viewed as sections of this constant family over $A$.
3. The computation rule for transport of sections (using `apd`) connects paths between sections (i.e., paths between functions) to families of paths in $B$.
4. Taking $e = \mathsf{id}_B$, this gives the funext principle.

The full proof is in the HoTT Book (Theorem 4.9.4). Here, we note that Univalence implies funext, so in HoTT we don't need to add funext as a separate axiom — it comes for free.

## Summary

The Univalence Axiom states:
$$\mathsf{idToEquiv} : (A =_\mathsf{Type} B) \xrightarrow{\;\simeq\;} (A \simeq B)$$

This means:
- **Paths in the universe are equivalences.** The only ways types can be equal are via equivalences.
- **Equivalent types are equal.** Every equivalence gives a path.
- **The universe is not a set.** Types with non-trivial automorphisms give non-trivial loops in the universe.
- **Function extensionality follows.** Homotopic functions are equal.
- **Propositional extensionality follows.** Logically equivalent propositions are equal.
- **Structure invariance.** Any type-theoretic predicate is invariant under equivalence.

Univalence is the axiom that makes HoTT genuinely homotopy-theoretic: it gives the universe the structure of a space (a Kan complex) where paths are equivalences, completing the vision of "types as spaces."
