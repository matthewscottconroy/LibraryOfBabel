# 1.1 The Definition of a Group

## Starting From Symmetry

Before writing down any axioms, let's think about what we're trying to capture.

Consider a square lying flat on a table. You can rotate it by 90°, 180°, or 270°, or flip it along any of its four axes of symmetry (two through opposite vertices, two through midpoints of opposite sides). After any of these operations, the square looks exactly the same as before — it's back in the same position, at least in terms of which spot in space it occupies. These operations are the *symmetries* of the square.

Now notice a few things about these symmetry operations:

- You can **combine** them: doing a 90° rotation and then a flip is itself a symmetry operation (a different flip).
- One of them **does nothing**: the "identity" rotation by 0°. It's still a valid symmetry — doing nothing is a perfectly good operation.
- Every operation can be **undone**: the reverse of a 90° clockwise rotation is a 90° counterclockwise rotation; the reverse of a flip is the same flip again.
- Combining operations is **associative**: it doesn't matter how you parenthesize a sequence of operations, because operations don't care about your mental bookkeeping — they just act on the square one at a time.

These four properties — closure under combination, existence of an identity, existence of inverses, and associativity — are exactly the axioms of a group. They capture the essence of "symmetry" in the most abstract possible way.

## The Formal Definition

**Definition (Group).** A *group* is a pair $(G, \cdot)$ where $G$ is a set and $\cdot : G \times G \to G$ is a binary operation satisfying:

1. **Associativity:** $(a \cdot b) \cdot c = a \cdot (b \cdot c)$ for all $a, b, c \in G$.
2. **Identity:** There exists an element $e \in G$ such that $e \cdot a = a \cdot e = a$ for all $a \in G$.
3. **Inverses:** For each $a \in G$ there exists an element $a^{-1} \in G$ such that $a \cdot a^{-1} = a^{-1} \cdot a = e$.

A group is *abelian* (or *commutative*) if additionally:

4. **Commutativity:** $a \cdot b = b \cdot a$ for all $a, b \in G$.

That's it. Four axioms (or three for a general group, four for an abelian group). The entire edifice of group theory rests on these.

## Notation and Conventions

We often drop the explicit multiplication dot and write $ab$ instead of $a \cdot b$. When the group is abelian and we want to emphasize its additive nature (like the integers $\mathbb{Z}$ under addition), we use $+$ for the operation, $0$ for the identity, and $-a$ for the inverse of $a$.

The identity element is often called the *neutral element* or *unit*. Its defining property is that it does nothing: multiplying by $e$ on either side leaves every element unchanged.

The integer $n$ in the expression $a^n$ means: multiply $a$ by itself $n$ times (for $n > 0$), or multiply $a^{-1}$ by itself $|n|$ times (for $n < 0$), or take the identity (for $n = 0$). So $a^3 = aaa$, $a^{-2} = a^{-1}a^{-1}$, and $a^0 = e$.

## Why These Axioms and Not Others?

A natural question: why these four? Why not more? Why not fewer?

**Why associativity?** Associativity means the *order in which you evaluate* a product doesn't matter, only the *order of the factors*. This is what allows us to write $abc$ without parentheses, confident it means the same thing no matter how we parse it. Without associativity, the "generalized associativity" theorem fails, and sequences of operations become much harder to reason about. Almost every algebraic structure we care about is associative.

Notice that associativity is *not* commutativity. Associativity says $(ab)c = a(bc)$: you can regroup, but you cannot reorder. Commutativity says $ab = ba$: you can reorder. Many important groups are associative but not commutative (like $S_n$ for $n \geq 3$, or matrix groups).

**Why identity?** The identity is the "zero-information" operation. Its existence is what allows inverses to be meaningful: $a^{-1}$ is defined by its relationship to $e$. Without an identity, "inverse" doesn't make sense.

One subtlety: the identity is required to work on *both* sides — $ea = ae = a$. This means left-identity and right-identity. We'll see below that you only actually *need* one-sided versions, and the other side comes for free.

**Why inverses?** Inverses are what make a group a *group* and not just a *monoid* (an associative structure with identity). Inverses allow you to "undo" operations, which is the algebraic essence of reversibility. Symmetry operations are reversible — that's built into the concept.

**Why not commutativity?** Because most interesting symmetry groups are *not* commutative. The order in which you rotate and flip a square matters — try it! Commutativity is a special property that some groups have, but it's not fundamental to the notion of symmetry.

## Consequences: What the Axioms Force

The axioms seem minimal, but they force quite a bit. Let's derive some immediate consequences — the kind of reasoning that constitutes the "first moves" in any group-theoretic argument.

**Theorem (Uniqueness of Identity).** The identity element $e$ is unique.

*Proof.* Suppose $e$ and $e'$ are both identities. Then:
$$e = e \cdot e' = e'$$
The first equality holds because $e'$ is an identity (so $e \cdot e' = e$). The second holds because $e$ is an identity (so $e \cdot e' = e'$). Therefore $e = e'$. $\square$

This is a beautiful proof: we don't need to assume $e$ and $e'$ are related at all — we just use each of their defining properties once. This style of argument (use the property of one thing, then the property of another) is extremely common in algebra.

**Theorem (Uniqueness of Inverses).** For each $a \in G$, the inverse $a^{-1}$ is unique.

*Proof.* Suppose $b$ and $c$ are both inverses of $a$, meaning $ba = e$ and $ac = e$. Then:
$$b = b \cdot e = b \cdot (ac) = (ba) \cdot c = e \cdot c = c$$
Every step uses either a property of $b$, a property of $c$, associativity, or the identity. No step requires knowing anything particular about $a$, $b$, or $c$ — only their relationships. $\square$

**Theorem (Shoes and Socks).** $(ab)^{-1} = b^{-1}a^{-1}$.

The name comes from putting on socks and shoes: to undo "put on socks then shoes," you must first take off shoes, then socks.

*Proof.* We need to show $b^{-1}a^{-1}$ is the inverse of $ab$. Check:
$$(ab)(b^{-1}a^{-1}) = a(bb^{-1})a^{-1} = a \cdot e \cdot a^{-1} = aa^{-1} = e$$
$$(b^{-1}a^{-1})(ab) = b^{-1}(a^{-1}a)b = b^{-1} \cdot e \cdot b = b^{-1}b = e$$
Since both sides give $e$, uniqueness of inverses gives $(ab)^{-1} = b^{-1}a^{-1}$. $\square$

**Theorem (Double Inverse).** $(a^{-1})^{-1} = a$.

*Proof.* The inverse of $a^{-1}$ is the element $x$ such that $a^{-1}x = xa^{-1} = e$. But $a$ satisfies this: $a^{-1} \cdot a = e$ and $a \cdot a^{-1} = e$. By uniqueness, $(a^{-1})^{-1} = a$. $\square$

**Theorem (Cancellation).** If $ab = ac$, then $b = c$ (left cancellation). If $ba = ca$, then $b = c$ (right cancellation).

*Proof of left cancellation.* Multiply both sides of $ab = ac$ on the left by $a^{-1}$:
$$a^{-1}(ab) = a^{-1}(ac)$$
$$(a^{-1}a)b = (a^{-1}a)c$$
$$eb = ec$$
$$b = c \quad \square$$

Cancellation is what makes linear equations $ax = b$ have unique solutions in groups: $x = a^{-1}b$. In a structure without inverses (like positive integers under multiplication), you can't always cancel.

## One-Sided Axioms Are Enough

Here's a pleasant economy theorem: we don't actually need to require both left and right inverses, or both left and right identity. The other side comes for free.

Suppose we only know:
- Associativity
- *Left identity*: there exists $e$ with $ea = a$ for all $a$
- *Left inverses*: for each $a$, there exists $b$ with $ba = e$

Can we conclude there's a right identity and right inverses? Yes — but the argument is a bit subtle, and working it out yourself is a good exercise in this kind of algebraic reasoning.

*Sketch:* Let $b$ be a left inverse of $a$. Then $b$ has a left inverse $c$. Compute:
$$ab = e(ab) = (cb)b \cdot \text{... hmm, need to be careful...}$$

Actually, let's just verify right inverses directly. We want to show $ab = e$ (i.e., $b$ is also a right inverse). We know $ba = e$. Let $c$ be a left inverse of $b$, so $cb = e$. Then:
$$ab = (e)ab = (cb \cdot b)... $$

This argument requires care. It's instructive to see that a priori it's not obvious — the one-sided axioms are logically weaker, but they imply the two-sided versions. For our purposes, we'll just use the two-sided definition.

## The Group as a Categorical Object

A final perspective, previewing the categorical thinking that will be essential throughout this curriculum.

A group $G$ can be viewed as a *category with one object*, where:
- The single object is just a placeholder (call it $\bullet$)
- The morphisms $\bullet \to \bullet$ are the elements of $G$
- Composition of morphisms is the group operation
- The identity morphism is $e$
- Every morphism is invertible (that's what inverses give us)

In other words, a group is precisely a category where there is one object and every morphism is an isomorphism. This viewpoint — thinking of groups as categories — will become increasingly important as we meet higher-dimensional analogs: groupoids (many objects, all morphisms invertible), ∞-groupoids (the fundamental concept of HoTT), and higher categories.

The group axioms, in this light, are just the category axioms (composition is associative, identity morphisms exist) plus the requirement that everything is invertible. The abstractness of group theory is not a defect but a feature: it captures exactly the structure that's present in a category of invertible morphisms, which is everywhere.
