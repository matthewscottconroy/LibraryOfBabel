# 3.1 Consequences of Univalence

## The Web of Consequences

Univalence is a surprisingly powerful axiom. Once you add it to type theory, a cascade of results follow — results that were either unprovable before, or had to be added as separate axioms.

The key consequences we explore here:
1. **Function extensionality** (funext): homotopic functions are equal
2. **Propositional extensionality** (propext): logically equivalent propositions are equal
3. **Structure invariance**: all mathematical structure is invariant under equivalence
4. **The Univalence Principle**: a formal theorem justifying the common mathematical practice

## Function Extensionality

**Theorem 3.1 (Funext from Univalence).** Univalence implies function extensionality: for $f, g : A \to B$:
$$(f = g) \simeq \prod_{x:A} f(x) =_B g(x)$$

*Proof sketch.* The key is to use the computation rule for transport and the path space of the function type.

Consider the type family over $\mathsf{Bool}$:
$$P(\mathsf{true}) = A, \quad P(\mathsf{false}) = B$$

A function $f : A \to B$ can be viewed as... actually, the standard proof is more subtle. Here's the cleaner approach.

**Approach via the path fibration.** Consider the type $A \to (A \to B)^I$ where $I = [0,1]$ (or rather $I = \Delta[1]$ in the simplicial model). Paths in $A \to B$ correspond to functions $A \to B^I$ (the "free path space" of $B$ over $A$). This gives funext in the classical homotopy-theoretic argument.

In type theory, the argument goes via the "interval" HIT or Univalence directly. The proof uses:
- The fact that paths in a product type $A \to B$ (viewed as $\prod_{x:A} B$) are families of paths in $B$
- The connection between paths in function types and the $\Pi$-path computation
- Univalence for the constant type family

For the complete proof, see the HoTT Book (Theorem 4.9.4 or the book's Chapter 4). The key point for us: **Univalence implies funext**, so in HoTT we don't need to add funext as a separate axiom.

**What funext says, practically.** When working in HoTT (or in a proof assistant implementing HoTT like Cubical Agda), we can always prove two functions equal by proving they agree pointwise. This is the way mathematicians *think* about function equality — two functions are the same if they have the same values everywhere.

In MLTT without funext, this fails: two distinct-but-extensionally-equal functions (same output for every input, but defined differently) are not provably equal. Funext (via Univalence) fixes this.

## Propositional Extensionality

**Theorem 3.2 (Propext from Univalence).** Univalence implies propositional extensionality: for mere propositions $P, Q : \mathsf{hProp}$:
$$(P \simeq Q) \simeq (P = Q)$$

More usably: $(P \leftrightarrow Q) \simeq (P = Q)$ (logical equivalence of propositions gives equality of propositions).

*Proof.* 
- By Univalence: $(P = Q) \simeq (P \simeq Q)$.
- For propositions: $P \simeq Q$ iff $P \leftrightarrow Q$ (since any function between propositions is trivially an equivalence — all elements of a proposition are equal, so the inverse is automatically the identity up to equality).
- So $(P = Q) \simeq (P \simeq Q) \simeq (P \leftrightarrow Q)$. $\square$

**What propext says.** In classical logic, propositions are truth values. Two propositions with the same truth value are "the same" proposition (logically). Propext makes this formal: in HoTT, logically equivalent propositions are literally equal as types.

**Important in practice.** When formalizing mathematics in Lean 4, `propext` is an axiom. In HoTT/Cubical Agda, it follows from Univalence. The consequence is that reasoning about propositions matches logical reasoning: if you can prove $P \iff Q$, you can substitute $P$ for $Q$ everywhere.

## Structure Invariance

The most philosophically significant consequence of Univalence:

**Theorem 3.3 (Structure Invariance).** For any type-theoretic predicate (type family) $P : \mathsf{Type} \to \mathsf{Type}$ and any equivalence $e : A \simeq B$:
$$\mathsf{transport}^P(\mathsf{ua}(e)) : P(A) \simeq P(B)$$

In other words: if $A \simeq B$, then $P(A) \simeq P(B)$ for *any* $P$ definable in type theory.

*Proof.* $\mathsf{ua}(e) : A = B$ is a path in the universe. Transporting $P$ along this path gives $\mathsf{transport}^P(\mathsf{ua}(e)) : P(A) \to P(B)$, and transport along any path is always an equivalence (Theorem 4.2 of Chapter 16). $\square$

**What this means.** Structure invariance says: **any property or structure definable in type theory is automatically invariant under equivalence of types**. There's no need to separately prove "the property is preserved by isomorphisms" — it's automatic.

**Examples of structure invariance:**

*Example 3.4 (Group structure).* If $P(G) =$ "G has a group structure" (i.e., $P(G)$ is the type of group structures on $G$), then for any equivalence $e : G \simeq H$, transport along $\mathsf{ua}(e)$ gives an equivalence of the group structure types: $P(G) \simeq P(H)$. This means: a group structure on $G$ gives a group structure on $H$ (by transporting via $e$), and vice versa.

*Example 3.5 (Computability).* If $P(A) =$ "A has a decidable equality," then $P(A) \simeq P(B)$ for any $A \simeq B$. So if one presentation of a type has decidable equality, any equivalent presentation does too.

*Example 3.6 (Topology).* If $P(A) =$ "A is a topological space (with a certain topology)," then any equivalence preserves topological structure.

**The key philosophical point.** In classical mathematics, it's *obvious* that mathematical properties should be preserved by isomorphisms — because we think of mathematics as studying abstract structures, not particular representations. But in formal foundations, this needs to be verified for each property.

Univalence makes this automatic: since equivalent types are literally equal, any property of a type automatically holds for all equivalent types (just by substituting equals for equals).

## The Univalence Principle

The Structure Invariance theorem is a special case of a more general principle:

**Theorem 3.5 (Univalence Principle, Ahrens-Kapulkin-Shulman).** A statement about types is preserved and reflected by equivalences if and only if it can be stated using the type-theoretic language (without reference to specific internal representations).

More formally: a property $P : \mathsf{Type} \to \mathsf{Type}$ is "invariant under equivalence" (i.e., $A \simeq B \to P(A) \simeq P(B)$) if and only if $P$ is stated in the "correct" way — using the abstract language of type theory rather than set-theoretic tricks.

**The "incorrect" way.** In set theory, you can define properties like "$A$ contains the empty set as an element." This property is not preserved by bijections: it holds for $A = \{\emptyset, 1\}$ but not for $A = \{2, 3\}$, even though these sets are in bijection. The reason it fails: it refers to the *specific* element $\emptyset$, not to the abstract structure.

In HoTT, you *cannot* state such set-theoretic properties — the type theory doesn't have a "global element" like $\emptyset$ that can be checked for membership. Every property you can state is automatically an abstract structural property. And abstract structural properties are invariant under equivalence by Structure Invariance.

**Why this matters.** The Univalence Principle validates the mathematical practice of treating isomorphic objects as identical. When a mathematician says "let $G$ be the cyclic group of order $p$" (without specifying which presentation), they're relying on the fact that all presentations are equivalent and any property they care about is preserved. Univalence makes this rigorous.

## The Standard Library of Univalent Mathematics

One of the projects enabled by Univalence is the *univalent foundations* program — formalizing mathematics in HoTT in a way that fully respects the equivalence-invariance of mathematical structure.

Key features of univalent mathematics:

**Everything up to equivalence.** Instead of "set-level" mathematics where objects are defined by their elements, univalent mathematics defines objects by their *type-level structure*. Groups are not sets-with-operations; they're types-with-operations-up-to-equivalence.

**The universal property style.** Mathematical structures are defined by their universal properties (rather than explicit constructions). This makes them automatically unique up to equivalence (since any two objects satisfying the same universal property are equivalent).

**Categorical mathematics.** Category theory is natural in univalent foundations: categories have objects (types) and morphisms (functions), and the "right" notion of equality of objects is equivalence. The *Rezk completion* of a category (making all equivalences into equalities) corresponds to the univalent completion.

**Concretely:** The proof assistants implementing HoTT (Cubical Agda, Lean 4 with the right library) allow formalizing univalent mathematics with the full benefit of Univalence.

## Univalence Breaks Proof Irrelevance for Types

**Warning: Subtlety.** Univalence has an important consequence that can be surprising: it means that the type of types ($\mathsf{Type}$) is *not* proof-irrelevant in general.

In classical type theory with K (UIP), all identity types are propositions — equality has at most one proof. With Univalence, $A = A$ in the universe has as many proofs as $A \simeq A$, which can be more than one.

This is *intended* — it's what makes HoTT genuinely homotopy-theoretic. But it means you need to be careful about when you assume "equality is propositional."

- For sets (h-level 0): equality is a proposition, Univalence gives at most one self-equivalence (the identity)... wait, no. Sets can have non-trivial automorphisms ($\mathbb{Z}/2\mathbb{Z}$ has the identity and negation). Univalence correctly says $\mathbb{Z}/2\mathbb{Z} = \mathbb{Z}/2\mathbb{Z}$ has two paths (corresponding to the two automorphisms).

The lesson: the universe is *not* a set (as Corollary 2.2 showed). Working in the universe requires attention to the h-level.

## Univalence and Excluded Middle

**Theorem 3.6.** Univalence is consistent with the law of excluded middle (LEM): $\prod_{P : \mathsf{hProp}} P + \neg P$.

This is important: HoTT can support both constructive mathematics (without LEM) and classical mathematics (with LEM). The choice is left open:
- Without LEM: constructive mathematics, where proofs carry computational content
- With LEM: classical mathematics, where non-constructive arguments are allowed

In either case, Univalence holds and the mathematical benefits it provides are available.

**The classical HoTT.** HoTT + LEM + AC (Axiom of Choice) gives a foundation for classical mathematics that is compatible with HoTT. The category of h-sets in this system satisfies the axioms of classical set theory (ZFC). So classical mathematics embeds faithfully into HoTT.

## Summary

Univalence implies:

| Consequence | Statement | Importance |
|---|---|---|
| Function extensionality | $(f=g) \simeq \prod_x f(x)=g(x)$ | Pointwise equality of functions |
| Propositional extensionality | $(P=Q) \simeq (P \leftrightarrow Q)$ for props | Logic as type theory |
| Structure invariance | $A \simeq B \to P(A) \simeq P(B)$ | Isomorphic types are interchangeable |
| Univalence principle | Abstract properties are invariant | Formal basis for mathematical practice |
| Non-set universe | $\mathsf{Type}$ is not a set | Richer path structure in the universe |
| Equivalence induction | Reduce $e:A\simeq B$ to $\mathsf{id}_A$ | Powerful proof technique |

These consequences transform type theory from a formal logical system into a genuine foundation for mathematics that respects the way mathematicians actually work.
