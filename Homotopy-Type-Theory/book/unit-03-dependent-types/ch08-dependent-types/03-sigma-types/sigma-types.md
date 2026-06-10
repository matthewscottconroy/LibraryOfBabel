# Σ Types: Dependent Pair Types

## The Pair That Knows What It Contains

In standard type theory, a pair type A × B contains an A and a B, full stop. The two components do not interact at the type level. A pair (3, "hello") has type ℕ × String, and the string has no awareness that the number is 3.

The Σ type changes this. A Σ type Σ(x:A).B(x) pairs an element a : A with an element b : B(a) — an element of the type family evaluated at the specific first component. The second component's *type* depends on the first component's *value*. The pair knows what it contains, and the type knows it too.

The simplest example: the type of "a natural number together with a proof that it is even." The number is the first component. The proof is the second component, and its type (IsEven n) depends on which number n was chosen. You cannot separate the number from the proof of its property, because the proof's type refers to the number.

This is not a quirk or a minor extension. It is the formal rendering of existence with a witness.

## The Formation Rule

$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma,\, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \sum_{x:A} B(x) : \mathsf{Type}} \qquad (\Sigma\text{-Form})$$

The same premises as Π-Form: A is a type, B is a type family over A. The conclusion is the Σ type. Both Π and Σ are formed from exactly the same data — a type and a type family. They are the "two faces" of the same dependent structure: Π is the space of sections, Σ is the total space.

**The non-dependent special case.** If B(x) = C for all x (constant family), then Σ(x:A).B(x) = Σ(x:A).C = A × C. The ordinary product type is a degenerate Σ type. Σ strictly subsumes ×.

## The Introduction Rule: Pairing

$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B(a)}{\Gamma \vdash (a,\, b) : \sum_{x:A} B(x)} \qquad (\Sigma\text{-Intro})$$

To construct a Σ type, you provide a first component a : A and a second component b : B(a). The second component's type must match the type family applied to the specific first component.

This is the key constraint: you cannot just provide any b : B(y) for some abstract y. You must provide an element of B(a), where a is the specific first component you chose. The types are linked.

**Example: even number with witness.**
$$\mathsf{evenPair} : \sum_{n:\mathbb{N}} \mathsf{IsEven}\, n$$
$$\mathsf{evenPair} = (4,\, \mathsf{witness}_4)$$

where witness₄ : IsEven 4 is the proof that 4 is even. This pair inhabits Σ(n:ℕ).IsEven n — it is an even number, paired with a proof of its evenness.

**Example: a type together with an element.**
$$\sum_{A:\mathsf{Type}} A$$

An element of this type is a pair (A, a) where A : Type and a : A — a type together with a specific element of that type. This is Σ-type thinking: the second component's type (namely A) depends on the first component (the specific type A).

## The Elimination Rules: Projections

Given a Σ type, there are two projections:

$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \mathsf{fst}(p) : A} \qquad (\Sigma\text{-}\mathsf{fst})$$

$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \mathsf{snd}(p) : B(\mathsf{fst}(p))} \qquad (\Sigma\text{-}\mathsf{snd})$$

The first projection fst(p) extracts the A-component. The second projection snd(p) extracts the B-component — and notice its type: B(fst(p)). The type of the second projection refers to the first projection. This is dependent typing in the elimination rule itself.

**Computation rules:**
$$\mathsf{fst}((a, b)) \equiv a$$
$$\mathsf{snd}((a, b)) \equiv b$$

Both projections compute on concrete pairs by definitional equality.

There is also a general elimination principle (the recursor/inductor for Σ types). Given a motive C : Σ(x:A).B(x) → Type and a function f : Π(a:A).Π(b:B(a)).C((a,b)), we can define:

$$\mathsf{ind}_\Sigma(C, f, (a, b)) \equiv f(a)(b) : C((a, b))$$

This is pattern-matching: to handle an arbitrary Σ-pair, it suffices to handle the case where both components are explicit.

## Σ Types as Existential Quantification

The logical reading of the Σ type is existential quantification. The type Σ(x:A).P(x) is the type of proofs that "there exists an x : A such that P(x) holds." An element of Σ(x:A).P(x) is a *pair* (a, p) where a : A is the *witness* (the specific element for which P holds) and p : P(a) is the *proof* that P holds at a.

$$\exists x \in A.\, P(x) \quad \longleftrightarrow \quad \sum_{x:A} P(x)$$

In set-theoretic logic, existential quantification is proof-irrelevant: "there exists an x" does not tell you which x, and two proofs of the same existential statement are considered the same. In dependent type theory, the Σ type is proof-*relevant*: the element (a, p) carries the witness a explicitly. Different witnesses give genuinely different elements of the Σ type.

This is a feature, not a bug. It means you can extract the witness computationally. If you have a proof that "there exists a prime greater than 1000000," you can run fst on it and get the actual prime number.

## Σ Types as Subsets and Structures

The Σ type is the correct way to form "subsets" in dependent type theory. The type of even natural numbers is:

$$\mathsf{Even} = \sum_{n:\mathbb{N}} \mathsf{IsEven}(n)$$

An element of Even is a pair (n, witness) where n is a natural number and witness is a proof that n is even. The "subset" is the total space of the predicate IsEven.

More generally, any predicate P : A → Type gives a "subtype" Σ(x:A).P(x) ↪ A, with the injection sending (x, p) to x. This subsetting construction has good computational properties: extracting the underlying element (via fst) is a computable function.

Mathematical structures are also naturally expressed as Σ types:

$$\mathsf{Group} = \sum_{G:\mathsf{Type}} \sum_{\cdot\, : G \to G \to G} \sum_{e : G} \sum_{\mathsf{inv}: G \to G} \mathsf{IsGroup}(G, \cdot, e, \mathsf{inv})$$

A group is: a type G, together with a binary operation, together with a unit, together with an inverse operation, together with a proof that the group axioms hold. Each layer of the Σ type adds another piece of structure, and the final layer adds the axioms as a proof. This is the type of groups.

## The Axiom of Choice as a Theorem

In set theory, the Axiom of Choice says: for any family of nonempty sets, there exists a choice function that picks one element from each. It is an *axiom* — not provable from ZF alone.

In MLTT, the Axiom of Choice is a theorem. Specifically:

$$\mathsf{AC} : \prod_{A:\mathsf{Type}} \prod_{B:A\to\mathsf{Type}} \prod_{C:{\textstyle\sum_{x:A}} B(x)\to\mathsf{Type}} \left(\prod_{x:A}\sum_{b:B(x)} C((x,b))\right) \to \sum_{f:\prod_{x:A} B(x)} \prod_{x:A} C((x, f(x)))$$

In plain language: if for every x : A there exists a b : B(x) with C(x, b), then there exists a function f choosing b(x) for each x, such that C(x, f(x)) holds for all x.

The proof? Given the hypothesis h : Π(x:A).Σ(b:B(x)).C((x,b)), define f = λx. fst(h x) (pick the first component) and define the proof-part as λx. snd(h x). That is the entire proof.

Why is this trivial in MLTT? Because the hypothesis already contains the choice, explicitly, as the first component of each Σ type. The "choice" is just extraction of what is already there. Set-theoretic AC is surprising because sets do not carry the witnesses explicitly. In MLTT, Σ types do. The axiom is already built into the logic.

This is philosophically significant. It means that constructive mathematics — the kind you can do in MLTT — has the full power of the axiom of choice built in, not as an extra assumption, but as a consequence of the logical structure.

## Σ Types and Π Types: Duality

Π and Σ types are dual in a precise sense. Given A : Type and B : A → Type:

- Π(x:A).B(x) is the *product* of the family B — all sections at once
- Σ(x:A).B(x) is the *coproduct* (sum) of the family B — the total space

The duality extends to their logical readings:
- Π is universal quantification: proofs for all x
- Σ is existential quantification: a proof for some specific x

And to their topological readings:
- Π is the space of sections of the fibration
- Σ is the total space of the fibration

There is also a logical relationship between them (a dependent version of the product-coproduct adjunction), but the precise statement requires function extensionality and so will appear later.

## Why Σ Types Lead to the Identity Type

Here is a preview. Consider the type:

$$\sum_{b:A} (a = b)$$

for a fixed a : A. This is "the type of all elements of A that are equal to a, together with a proof of their equality." Every element of A is equal to a or it is not. If b = a, then (b, p) ∈ Σ(b:A).(a = b) via the path p. If b ≠ a, there is no such pair.

This Σ type has a special property: it is *contractible*. There is exactly one element, up to higher equality: (a, refl_a). All other inhabitants (b, p) are equal to (a, refl_a) via the path p. This contractibility — the based path space is contractible — is a theorem derivable from the J elimination rule for identity types.

This is not a curiosity. The contractibility of Σ(b:A).(a = b) is the key fact behind path induction, which is the foundation of all HoTT reasoning about paths and spaces. The Σ type, then, is not just for encoding structures and subsets — it is at the heart of the geometric content of HoTT.
