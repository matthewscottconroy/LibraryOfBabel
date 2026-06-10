# 5.1 Paths in Specific Types

## The General Strategy

We know what paths are in the abstract: elements of the identity type. But for specific type constructors — products, dependent pairs, function types — we want to understand what paths *look like*. What does it mean for two pairs to be equal? What does it mean for two functions to be equal?

The general answer has a pleasant structure: **paths in a compound type decompose into paths in the components**. A path in $A \times B$ is essentially a pair of paths, one in $A$ and one in $B$. A path in $\sum_{x:A} B(x)$ is a path in $A$ together with a transport condition. A path in $A \to B$, assuming function extensionality, is a pointwise equality.

This section derives these characterizations systematically, always using the tools we've developed: J, transport, and ap.

## Paths in Product Types

Let's start with the simplest case: the product type $A \times B$.

**Claim:** A path $(a_1, b_1) =_{A \times B} (a_2, b_2)$ is essentially the same thing as a pair of paths $(p : a_1 =_A a_2) \times (q : b_1 =_B b_2)$.

More precisely:

**Theorem 5.1 (Paths in products).** There is an equivalence:
$$((a_1, b_1) =_{A \times B} (a_2, b_2)) \simeq (a_1 =_A a_2) \times (b_1 =_B b_2)$$

**One direction — pairing of aps.** Given a path $p : (a_1, b_1) = (a_2, b_2)$ in $A \times B$, we get:
- $\mathsf{ap}_{\pi_1}(p) : a_1 = a_2$ by applying $\mathsf{ap}$ to the first projection $\pi_1 : A \times B \to A$
- $\mathsf{ap}_{\pi_2}(p) : b_1 = b_2$ by applying $\mathsf{ap}$ to the second projection $\pi_2 : A \times B \to B$

So we define: $\mathsf{pair\text{-}eq}^{-1}(p) = (\mathsf{ap}_{\pi_1}(p), \mathsf{ap}_{\pi_2}(p))$.

**Other direction — pairing paths into a product path.** Given paths $p_A : a_1 = a_2$ and $p_B : b_1 = b_2$, we want to construct $(a_1, b_1) = (a_2, b_2)$.

By J on $p_A$: reduces to the case $a_1 = a_2 = a$ (definitionally). Then by J on $p_B$: reduces to the case $b_1 = b_2 = b$. In this case, $(a, b) = (a, b)$, which is $\mathsf{refl}_{(a,b)}$.

Define: $\mathsf{pair\text{-}eq}(p_A, p_B) : (a_1, b_1) = (a_2, b_2)$.

**These are inverses.** The round trips:
- $\mathsf{pair\text{-}eq}(\mathsf{ap}_{\pi_1}(r), \mathsf{ap}_{\pi_2}(r)) = r$ for $r : (a_1, b_1) = (a_2, b_2)$: by J on $r$, reduces to the reflexivity case where both sides are $\mathsf{refl}$.
- $\mathsf{ap}_{\pi_i}(\mathsf{pair\text{-}eq}(p_A, p_B)) = p_i$: by J on $p_A$ then $p_B$, reduces to the reflexivity case.

So $\mathsf{pair\text{-}eq}$ is an equivalence. $\square$

**Computation:** $\mathsf{pair\text{-}eq}(\mathsf{refl}_a, \mathsf{refl}_b) \equiv \mathsf{refl}_{(a,b)}$.

This is very satisfying: the reflexivity path in a product type is literally the pair of reflexivity paths. The identity type of a product type decomposes completely into the identity types of the components.

**Topological picture.** A path in $X \times Y$ (a product space) is exactly a pair of paths: one in $X$ and one in $Y$. The path traverses both spaces simultaneously, at the same rate. This is exactly what we've proved: paths in $A \times B$ decompose as pairs of paths.

## Paths in Dependent Pair Types (Σ-types)

Now for the more interesting case: the dependent pair type $\sum_{x:A} B(x)$.

Elements of $\sum_{x:A} B(x)$ are pairs $(a, b)$ where $a : A$ and $b : B(a)$. The first component $a$ is in $A$, and the second component $b$ depends on $a$. What does it mean for two such pairs to be equal?

**Theorem 5.2 (Paths in Σ-types).** There is an equivalence:
$$((a_1, b_1) =_{\sum_{x:A} B(x)} (a_2, b_2)) \simeq \sum_{p : a_1 =_A a_2} \mathsf{transport}^B(p, b_1) =_{B(a_2)} b_2$$

Let's unpack this. A path between $(a_1, b_1)$ and $(a_2, b_2)$ consists of:
1. A path $p : a_1 = a_2$ in the first component
2. A path $\mathsf{transport}^B(p, b_1) = b_2$ in the second component *after transporting $b_1$ along $p$*

Why do we need transport? Because $b_1 : B(a_1)$ and $b_2 : B(a_2)$, and $B(a_1)$ and $B(a_2)$ are (generally) different types. You can't directly compare elements of different types! You first need to move $b_1$ to the same type as $b_2$ using transport along $p$.

This is the key difference from the product case. In a non-dependent product $A \times B$, we can compare the two second components directly. In a Σ-type, the second components live in different fibers and must be compared via transport.

**One direction.** Given $r : (a_1, b_1) = (a_2, b_2)$:
- First component: $p = \mathsf{ap}_{\pi_1}(r) : a_1 = a_2$
- Second component: This requires more work. We need to show $\mathsf{transport}^B(p, b_1) = b_2$.

The key is to use the "path-over" or "dependent path" induced by $r$. By J on $r$, we reduce to the reflexivity case where $a_1 = a_2 = a$ and $b_1 = b_2 = b$ (definitionally), and we need $\mathsf{transport}^B(\mathsf{refl}_a, b) = b$, which holds by the computation rule for transport.

**Other direction.** Given $p : a_1 = a_2$ and $q : \mathsf{transport}^B(p, b_1) = b_2$:
By J on $p$: reduces to $a_1 = a_2 = a$ and $\mathsf{transport}^B(\mathsf{refl}_a, b_1) \equiv b_1$. Then $q : b_1 = b_2$ in $B(a)$. By J on $q$: reduces to $b_1 = b_2 = b$. Then $(a, b) = (a, b)$, which is $\mathsf{refl}$.

**These are inverses** — proved by J on the relevant paths, reducing to the reflexivity case. $\square$

**Special case: Non-dependent products.** When $B(x) = B$ (constant), transport along any path is the identity (Example 4.4 from the previous section). So the Σ-path characterization reduces to:

$$((a_1, b_1) =_{A \times B} (a_2, b_2)) \simeq (a_1 = a_2) \times (b_1 = b_2)$$

which matches Theorem 5.1.

**Example: Paths in the natural numbers with a predicate.** Consider $\sum_{n:\mathbb{N}} \mathsf{isEven}(n)$, where $\mathsf{isEven}(n)$ is a proposition (has at most one proof). A path $(m, p) = (n, q)$ consists of:
- A path $m = n$ in $\mathbb{N}$
- A path $\mathsf{transport}^{\mathsf{isEven}}(m=n, p) = q$ in $\mathsf{isEven}(n)$

Since $\mathsf{isEven}(n)$ is a proposition, the second path is automatic — there's at most one element. So $\sum_{n:\mathbb{N}} \mathsf{isEven}(n)$ has the same paths as $\mathbb{N}$ (restricted to even numbers). This is the type-theoretic version of "a subtype's paths are the original type's paths."

**The fiber picture.** Recall that $\sum_{x:A} B(x)$ is like a fibration $E \to A$ where the fiber over $a$ is $B(a)$. A path in $E$ that starts at $(a_1, b_1)$ and ends at $(a_2, b_2)$ projects to a path from $a_1$ to $a_2$ in $A$, and the fiber part "covers" this path via transport. This is the homotopy-theoretic notion of a *path in a total space covering a path in the base*.

## Paths in Function Types

What does it mean for two functions $f, g : A \to B$ to be equal?

The identity type $f =_{A \to B} g$ is a perfectly valid type, but it's less clear what its elements look like from the inside. Intuitively, $f$ and $g$ should be "equal" if they agree on all inputs: for all $x : A$, $f(x) = g(x)$.

**Definition 5.3 (Homotopy).** A *homotopy* between $f, g : A \to B$ is:
$$f \sim g := \prod_{x:A} f(x) =_B g(x)$$

If $f = g$ (they're equal as functions), then certainly $f \sim g$ (they're pointwise equal). This direction is easy:

**Lemma 5.4 (Equality implies homotopy).** If $p : f = g$, then $\mathsf{happly}(p) : f \sim g$, defined by:
$$\mathsf{happly}(p)(x) = \mathsf{ap}_{\mathsf{ev}_x}(p)$$

where $\mathsf{ev}_x : (A \to B) \to B$ is the evaluation function $h \mapsto h(x)$.

Or directly: by J on $p$, the base case gives $\mathsf{happly}(\mathsf{refl}_f)(x) \equiv \mathsf{refl}_{f(x)}$, which is a valid homotopy.

**The question:** Is the converse true? If $f \sim g$, is $f = g$?

**The problem:** This is *not provable from J alone*!

This is a subtle and important point. The J rule tells us about *paths that already exist* — it's the eliminator for the identity type. But it can't create new identities between things that might be "genuinely different" in some interpretation of the type theory.

To see why, consider a model where functions are truly computational objects. Two functions can be extensionally equal (same input-output behavior) but intensionally different (different algorithms). In such a model, function extensionality would be false.

In the standard set-theoretic interpretation of type theory (before HoTT), functions are sets of input-output pairs, and so if $f$ and $g$ have the same input-output pairs, they are literally the same function. But in the *proof-relevant* interpretation, functions carry computational content that might differ.

## Function Extensionality

**Axiom (Function Extensionality — funext).** The map:
$$\mathsf{happly} : (f = g) \to \prod_{x:A} f(x) = g(x)$$

is an equivalence. That is, $\mathsf{funext} : (f \sim g) \to (f = g)$ exists, and is inverse to $\mathsf{happly}$.

With funext:
$$(f =_{A \to B} g) \simeq \prod_{x:A} f(x) =_B g(x)$$

Paths between functions are exactly homotopies.

**Status of funext in HoTT.** Function extensionality is not an axiom in classical mathematics — it's a theorem about functions-as-sets. In HoTT:
- It's **not provable from the basic type theory** (MLTT + J)
- It's **consistent** with MLTT
- It **follows from Univalence** (Chapter 18)

The last point is remarkable: the Univalence axiom, which says that equivalent types are equal, implies function extensionality. Here's the sketch:

*Proof of funext from Univalence (sketch):* Consider the type family $P : A \to \mathsf{Type}$ sending $x \mapsto B$ (the constant family at $B$). Then $\prod_{x:A} P(x) = (A \to B)$. Now consider a homotopy $H : f \sim g$. This is a section of a certain type family that, after some work involving Univalence, yields a path $f = g$. The details require the full apparatus of Chapter 18. $\square$

So in HoTT, function extensionality is a theorem, not an axiom. This is one of the benefits of Univalence: it implies many useful consequences automatically.

**Dependent function extensionality.** The same holds for dependent functions. For $f, g : \prod_{x:A} B(x)$:

$$\mathsf{happly} : (f = g) \to \prod_{x:A} f(x) = g(x)$$

is an equivalence (given Univalence or as a separate axiom). A path between dependent functions is a dependent homotopy — a family of paths, one for each input, between the output values.

**The computation rules for funext.** Assuming funext:
- $\mathsf{happly}(\mathsf{funext}(H)) = H$ (funext followed by happly recovers the homotopy)
- $\mathsf{funext}(\mathsf{happly}(p)) = p$ (happly followed by funext recovers the path)
- $\mathsf{funext}(\lambda x.\, \mathsf{refl}_{f(x)}) = \mathsf{refl}_f$ (the trivial homotopy gives reflexivity)

The last rule says: if the homotopy $H$ is the pointwise reflexivity ($H(x) = \mathsf{refl}_{f(x)}$), then funext turns it into $\mathsf{refl}_f$.

## Transport in Function Types

Let's combine the function extensionality picture with the transport picture from Section 4.

**Example 5.5 (Transport in function types, revisited).** For $P(x) = (B(x) \to C(x))$:
$$\mathsf{transport}^{B \to C}(p, f) = \lambda y \mapsto \mathsf{transport}^C(p,\, f(\mathsf{transport}^B(p^{-1}, y)))$$

This says: to transport a function $f : B(a) \to C(a)$ along $p : a = b$:
1. Take input $y : B(b)$
2. Transport $y$ backward along $p$ to get $\mathsf{transport}^B(p^{-1}, y) : B(a)$
3. Apply $f$ to get $f(\mathsf{transport}^B(p^{-1}, y)) : C(a)$
4. Transport the result forward along $p$ to get something in $C(b)$

**Under funext**, the transport of a function is *equal* (not just definitionally the same) to the function you'd get by the explicit formula above. So funext and transport are compatible.

## Paths in the Universe

One final case worth mentioning: what are paths in the universe $\mathsf{Type}$?

A path $p : A =_\mathsf{Type} B$ in the universe is an equality between two types. Classically (in set theory), two sets are equal iff they have the same elements — set extensionality. 

In HoTT, **Univalence** says:
$$(A =_\mathsf{Type} B) \simeq (A \simeq B)$$

A path between types is an equivalence between them. This is the subject of Chapter 18. For now, we note that transport along a path $p : A = B$ in the universe gives a function $\mathsf{transport}^{\mathsf{id}}(p) : A \to B$, and Univalence says this function is an equivalence.

**The coercion map.** The transport $\mathsf{transport}^{\mathsf{id}}(p) : A \to B$ is sometimes called a *coercion*: if $A = B$ (in the universe), we can coerce elements of $A$ to elements of $B$ along the path. Under Univalence, this coercion is an equivalence, matching the univalent picture that "equal types are equivalent."

## Summary of Path Computations

Here's the complete picture of paths in all the basic type formers:

| Type | Paths $(x_1 = x_2)$ |
|---|---|
| $A \times B$ | $\simeq (a_1 = a_2) \times (b_1 = b_2)$ |
| $\sum_{x:A} B(x)$ | $\simeq \sum_{p:a_1=a_2} \mathsf{transport}^B(p, b_1) = b_2$ |
| $A \to B$ | $\simeq \prod_{x:A} f(x) = g(x)$ (with funext) |
| $\prod_{x:A} B(x)$ | $\simeq \prod_{x:A} f(x) = g(x)$ (with funext) |
| $A$ in $\mathsf{Type}$ | $\simeq (A \simeq B)$ (Univalence, Chapter 18) |
| $\mathbb{N}$ | $\simeq$ decidable, discrete (Chapter 17) |
| $\mathsf{Bool}$ | $\simeq \{\mathsf{refl}\}$ for same, empty for different |

Each of these is proved using J (plus funext/Univalence where needed), and each says: the path type of a compound type decomposes into paths in the components, with transport connecting the pieces in the dependent cases.

## The Univalence Connection

We've now seen that:
- Paths in $A \times B$ decompose into paths in $A$ and $B$
- Paths in $\sum_{x:A} B(x)$ decompose into a path in $A$ plus a transported equality
- Paths in $A \to B$ (with funext) are pointwise equalities

What about paths in the *universe*? This is Univalence, and it's the deepest path computation of all. It says that equality *of types* is the same as *equivalence of types* — a profound connection between the logical notion of equality and the mathematical notion of isomorphism.

Function extensionality is a weak version of this: functions that agree on all inputs are equal. Univalence is the strong version: types that are equivalent (have equivalent structure) are equal. Both are consequences of the homotopy-theoretic interpretation where "equality" means "connected by a path" and paths between complicated objects decompose into paths between their constituents.

## Why This Matters

The path characterizations in this section are not just mathematical curiosities. They're the foundation for *doing mathematics in HoTT*:

**Working with structured types.** When you have a type $\sum_{x:A} P(x)$ of "structured objects" (like groups, or graphs, or proofs), the path characterization tells you exactly what equality of structured objects means: it's a path in the underlying data that is compatible with all the structure via transport.

**Equivalences via paths.** The path characterization of Σ-types underlies the characterization of equivalences in terms of their fibers. An equivalence $f : A \simeq B$ is exactly a function where all fibers $\sum_{a:A} f(a) = b$ are contractible — and the paths in these fibers are exactly what you get from the Σ-path characterization.

**Function extensionality in proofs.** When proving that two functions are equal, funext lets you work pointwise. This is used everywhere in mathematics: to show that two group homomorphisms are equal, you show they agree on all elements. In HoTT, this is literally a consequence of funext (or Univalence).

**Inductive types.** For inductive types like $\mathbb{N}$, $\mathsf{Bool}$, $\mathsf{List}$, the path characterizations follow from the fact that these types are *sets* (have h-level 0, see Chapter 17): distinct constructors give distinct elements, and paths between constructors decompose based on the arguments. This connects to the next chapter's topic of h-levels.

The path computations in this section — products, Σ-types, function types, the universe — complete the picture of "how paths work in type theory." Together with transport and ap from the previous section, we have everything needed to reason about equality in any compound type that arises in mathematical practice.
