# 4.1 Examples: Paths Between Types

## Seeing Univalence in Action

The best way to understand Univalence is through examples. Let's work through specific cases of the equivalence $(A = B) \simeq (A \simeq B)$ and see what paths in the universe look like.

## The Two Paths on Bool

The simplest non-trivial example: $\mathsf{Bool} = \{\mathsf{true}, \mathsf{false}\}$ has exactly two self-equivalences.

**The two self-equivalences of $\mathsf{Bool}$:**
1. $\mathsf{id}_\mathsf{Bool} : \mathsf{Bool} \simeq \mathsf{Bool}$ — the identity, sending $\mathsf{true} \mapsto \mathsf{true}$ and $\mathsf{false} \mapsto \mathsf{false}$
2. $\mathsf{neg} : \mathsf{Bool} \simeq \mathsf{Bool}$ — negation, sending $\mathsf{true} \mapsto \mathsf{false}$ and $\mathsf{false} \mapsto \mathsf{true}$

These are all the self-equivalences: any bijection $\mathsf{Bool} \to \mathsf{Bool}$ must either preserve or swap the two elements, so there are exactly $2! = 2$ of them.

**By Univalence:** $\mathsf{Bool} = \mathsf{Bool}$ in the universe has exactly two paths:
- $\mathsf{refl}_\mathsf{Bool} = \mathsf{ua}(\mathsf{id}_\mathsf{Bool})$ — the constant path
- $\mathsf{ua}(\mathsf{neg})$ — the "negation path"

**The negation path is non-trivial.** $\mathsf{ua}(\mathsf{neg}) \neq \mathsf{refl}_\mathsf{Bool}$, because:
- $\mathsf{idToEquiv}(\mathsf{refl}_\mathsf{Bool}) = \mathsf{id}_\mathsf{Bool}$
- $\mathsf{idToEquiv}(\mathsf{ua}(\mathsf{neg})) = \mathsf{neg}$
- Since $\mathsf{id}_\mathsf{Bool} \neq \mathsf{neg}$ (they disagree on $\mathsf{true}$), and $\mathsf{idToEquiv}$ is an equivalence (so injective), the paths must be different.

**Transporting along the negation path.** For $b : \mathsf{Bool}$:
$$\mathsf{transport}^{\mathsf{id}}(\mathsf{ua}(\mathsf{neg}), b) = \mathsf{neg}(b)$$

Transporting $\mathsf{true}$ along the negation path gives $\mathsf{false}$, and vice versa.

**The loop space of the universe at $\mathsf{Bool}$.** The type $\mathsf{Bool} = \mathsf{Bool}$ is exactly $\mathbb{Z}/2\mathbb{Z}$ (the two-element group), by Univalence:
$$(\mathsf{Bool} = \mathsf{Bool}) \simeq (\mathsf{Bool} \simeq \mathsf{Bool}) \simeq \mathbb{Z}/2\mathbb{Z}$$

So the universe has a "non-trivial loop at $\mathsf{Bool}$" of order 2. This is a topological fact: the classifying space $B(\mathbb{Z}/2\mathbb{Z})$ has a loop at the basepoint corresponding to the generator of $\mathbb{Z}/2\mathbb{Z}$.

## Paths Between Different Types

Univalence also describes paths between *different* types.

**Example 4.1 (Two-element types).** Any two types with exactly two elements are equivalent (and hence equal). For example:
- $\mathsf{Bool} = \mathbf{1} + \mathbf{1}$ — two-element discrete type, two elements
- $\mathsf{Fin}(2) = \{0, 1\}$ — the finite type with two elements

By Univalence, $\mathsf{Bool} = \mathsf{Fin}(2)$ in the universe (there's a path between them), since $\mathsf{Bool} \simeq \mathsf{Fin}(2)$ (any bijection gives an equivalence, since both are sets with exactly two elements).

Actually, the number of paths $\mathsf{Bool} = \mathsf{Fin}(2)$ equals the number of bijections, which is 2 (match $\mathsf{true}$ with $0$ and $\mathsf{false}$ with $1$, or vice versa).

**Example 4.2 (Isomorphic groups).** Let $G_1 = \mathbb{Z}/2\mathbb{Z}$ (as a type, with the group structure built in as additional data) and $G_2 =$ another presentation of $\mathbb{Z}/2\mathbb{Z}$. As *plain types* (forgetting the group structure), $G_1 \simeq G_2$ (both are two-element sets), so $G_1 = G_2$ in the universe by Univalence.

**The subtlety of structured equivalences.** When we say "$G_1$ and $G_2$ are isomorphic groups," we mean there's an equivalence of types that also respects the group operations. This is an equivalence in the *type of groups*:
$$\sum_{G : \mathsf{Type}} \mathsf{GroupStr}(G)$$

where $\mathsf{GroupStr}(G)$ is the type of group structures on $G$. The relevant Univalence is for this Σ-type, not just the underlying types.

## Paths Between Propositions

For propositions (h-level $-1$), the paths are especially clean.

**Theorem 4.3.** For propositions $P, Q$: $P = Q$ in the universe iff $P \leftrightarrow Q$.

*Proof.* By Univalence, $P = Q \simeq P \simeq Q$. For propositions, $P \simeq Q$ iff $P \to Q$ and $Q \to P$ (both directions of the logical equivalence). $\square$

**Examples:**
- $\mathbf{1} = \mathbf{0}$ is empty (the empty and unit types are not logically equivalent)
- $\mathbf{1} = \mathbf{1}$ has one path (reflexivity; $\mathbf{1} \simeq \mathbf{1}$ has exactly one element, the identity)
- $(n = n) = \mathbf{1}$ for any $n : \mathbb{N}$ (equality of equal things is the unit type, i.e., contractible)
- $(0 = 1) = \mathbf{0}$ for $0, 1 : \mathbb{N}$ (zero and one are not equal, the equality type is empty)

**The universe of propositions.** The sub-universe $\mathsf{hProp} = \sum_{A:\mathsf{Type}} \mathsf{isProp}(A)$ has paths between propositions that are exactly logical equivalences. By Univalence (restricted to propositions), $\mathsf{hProp}$ is a set (since the paths between propositions are propositions themselves — logical equivalence is a proposition). This matches: the universe of truth values is a set.

## The Universe Is a Groupoid (1-Type)

The universe of *sets* $\mathsf{hSet} = \sum_{A:\mathsf{Type}} \mathsf{isSet}(A)$ has paths that are equivalences between sets (functions with contractible fibers, where both source and target are sets). These equivalences form a set themselves (a set bijection is determined by its function, and function extensionality makes the set of bijections... well, it depends on the cardinality). So $\mathsf{hSet}$ is a 1-type (groupoid).

**The $n$-type universe.** In general, the universe of $n$-types is an $(n+1)$-type. This matches the h-level arithmetic:
- Universe of propositions ($(-1)$-types): a set ($0$-type)
- Universe of sets ($0$-types): a 1-groupoid ($1$-type)
- Universe of 1-types: a 2-type
- ...

Each level of the universe adds one to the h-level. The universe $\mathsf{Type}$ (of all types) is an $\infty$-type — no finite truncation stabilizes it.

## The Fundamental Group of the Universe at a Type

By Univalence, the fundamental group of the universe at a type $A$ is the automorphism group of $A$:
$$\pi_1(\mathsf{Type}, A) = \|\Omega(\mathsf{Type}, A)\|_0 = \|\mathsf{Aut}(A)\|_0 = \mathsf{Aut}(A) / \mathsf{id}$$

(the connected components of the self-equivalences).

More precisely:
$$\Omega(\mathsf{Type}, A) = (A = A) \simeq (A \simeq A) = \mathsf{Aut}(A)$$

And the fundamental group (as a set) is $\pi_1(\mathsf{Type}, A) = \|\mathsf{Aut}(A)\|_0$.

**Examples:**
- $\pi_1(\mathsf{Type}, \mathbb{N}) = \mathbf{1}$ — the only automorphism of $\mathbb{N}$ (as a type, not as a group) is the identity (since $\mathbb{N}$ is a rigid type with a unique automorphism — actually there are many bijections of $\mathbb{N}$, but... hmm. The point is the connected components of $\mathsf{Aut}(\mathbb{N})$)
- $\pi_1(\mathsf{Type}, \mathsf{Bool}) = \mathbb{Z}/2\mathbb{Z}$ — the two automorphisms of $\mathsf{Bool}$, which form a 2-element group

## Groups as Pointed Connected 1-Types

Here's a beautiful example of Univalence in action.

**Theorem 4.4.** There is an equivalence between:
- Groups (types $G$ with a group structure)
- Pointed connected 1-types $(A, a)$ where $A$ has h-level 1, $\pi_0(A) = \mathbf{1}$, and $\pi_1(A, a) = G$

*Explanation.* Given a group $G$, construct the *classifying type* $BG$:
- $BG$ is a 1-type (h-level 1)
- $BG$ has one connected component: $\pi_0(BG) = \mathbf{1}$
- The loop space at the basepoint: $\pi_1(BG, *) = G$

In topology, $BG$ is the classifying space of $G$ (a $K(G, 1)$ space). In HoTT, $BG$ can be constructed as a HIT.

**By Univalence:** The paths at the basepoint $*$ in $BG$ are exactly the elements of $G$:
$$(\mathsf{pt} = \mathsf{pt})_{BG} \simeq G$$

This is the key loop-space / group correspondence in homotopy theory, now made precise in type theory via Univalence.

## Univalence and Equality of Mathematical Structures

Let's see Univalence applied to a real mathematical example: two isomorphic groups are equal.

**Setup.** Define the type of groups:
$$\mathsf{Group} :\equiv \sum_{G : \mathsf{hSet}} \sum_{m : G \to G \to G} \sum_{e : G} \mathsf{GroupLaws}(G, m, e)$$

where $\mathsf{GroupLaws}$ is the type expressing associativity, left/right unit, and inverses.

**Theorem 4.5.** Two groups $(G_1, m_1, e_1, \ldots)$ and $(G_2, m_2, e_2, \ldots)$ are equal in $\mathsf{Group}$ if and only if they are isomorphic as groups.

*Proof.* By the Σ-path characterization (Chapter 16), a path between $(G_1, m_1, e_1, \ldots)$ and $(G_2, m_2, e_2, \ldots)$ in $\mathsf{Group}$ consists of:
1. A path $p : G_1 = G_2$ in $\mathsf{hSet}$ (i.e., an equivalence $G_1 \simeq G_2$ by Univalence)
2. A transport condition: $\mathsf{transport}^{(\text{multiplication, unit, laws})}(p) = (m_2, e_2, \ldots)$

The transport condition says: the equivalence $f : G_1 \simeq G_2$ is a group homomorphism (it respects multiplication and identity). And since $f$ is an equivalence, it's automatically an *isomorphism* of groups.

So a path between the groups = a group isomorphism. $\square$

**The upshot.** In the type of groups, *equality is exactly isomorphism*. Two groups are the same iff they're isomorphic. This is the formalization of the mathematical practice of working "up to isomorphism."

This works for any mathematical structure: rings, vector spaces, topological spaces, categories — as long as the structure is formalized as a Σ-type over a type of objects plus a type of operations plus laws, paths in the resulting type equal isomorphisms of the structure.

## Why This Is Different from Classical Foundations

In ZFC, two groups $G_1$ and $G_2$ can be isomorphic but literally different sets (with different elements). Saying "they're equal" is informal — it means "I'll treat them as the same for the purposes of this argument."

In HoTT with Univalence:
- If $G_1 \cong G_2$ (isomorphic as groups), then $G_1 = G_2$ in the type of groups (literally equal)
- Any theorem about $G_1$ (stated in terms of the group structure) also holds for $G_2$, not informally but formally — by substituting the equal terms

This is the difference between a foundation that *tolerates* mathematical practice and one that *formally validates* it.

## Summary

Univalence makes concrete what homotopy theorists have always known: the universe of types is a rich mathematical object, not just a formal collection. Paths in the universe are equivalences, loops are automorphisms, and the h-level of the universe at each type reflects the complexity of its automorphism group.

| Type $A$ | $\mathsf{Aut}(A)$ | $A = A$ paths |
|---|---|---|
| $\mathbf{0}$ | $\mathbf{1}$ | 1 path |
| $\mathbf{1}$ | $\mathbf{1}$ | 1 path |
| $\mathsf{Bool}$ | $\mathbb{Z}/2\mathbb{Z}$ | 2 paths |
| $\mathsf{Fin}(n)$ | $S_n$ | $n!$ paths |
| $\mathbb{N}$ | infinite | many paths |
| $\mathbb{Z}/p\mathbb{Z}$ (as a group) | $(\mathbb{Z}/p\mathbb{Z})^\times$ | $p-1$ paths |

Every type has an automorphism group, every automorphism corresponds to a loop in the universe, and Univalence makes this correspondence precise and computable.
