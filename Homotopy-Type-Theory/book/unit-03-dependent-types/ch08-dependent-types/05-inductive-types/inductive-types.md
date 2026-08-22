# Inductive Types: Recursion and Induction as One

## The Fundamental Idea

Mathematics builds its objects by putting together simpler objects. The natural numbers start from zero and keep adding one. Lists are either empty or an element prepended to a shorter list. Binary trees are either leaves or a node with two subtrees. Each of these is an *inductive type*: a type specified by saying how to build its elements (constructors) and how to reason about them (elimination principle).

The remarkable fact — and it took decades to make this precise — is that the construction rule and the reasoning rule are not separate. They are the same thing, seen from opposite directions. When you specify how to build elements (the constructors), you simultaneously specify how to prove things about all elements (the eliminator). Recursion and induction are two faces of the same principle.

This is not a philosophical claim. It is a theorem about the structure of inductive types in MLTT.

## The Natural Numbers

The canonical inductive type is ℕ. Its definition in MLTT:

**Constructors:**
- zero : ℕ
- succ : ℕ → ℕ

Every natural number is either zero or the successor of another natural number. These are the only elements.

**Eliminator (the recursor):**

$$\frac{\Gamma \vdash C : \mathbb{N} \to \mathsf{Type} \quad \Gamma \vdash c_z : C(\mathsf{zero}) \quad \Gamma \vdash c_s : \prod_{n:\mathbb{N}} C(n) \to C(\mathsf{succ}(n)) \quad \Gamma \vdash n : \mathbb{N}}{\Gamma \vdash \mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, n) : C(n)}$$

The eliminator says: to define a value (or prove a property) for every natural number n, it suffices to:
1. Handle n = zero: provide c_z : C(zero)
2. Handle n = succ(m): given the result for m, provide c_s(m) : C(m) → C(succ(m))

**Computation rules:**
$$\mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, \mathsf{zero}) \equiv c_z$$
$$\mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, \mathsf{succ}(n)) \equiv c_s(n, \mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, n))$$

The computation rules say: ind on zero gives c_z; ind on succ(n) applies c_s to n and the result for n.

**As recursion:** Take C = λn. ℕ (a constant family). Then ind gives a function ℕ → ℕ defined by the base case and the recursive step. This is primitive recursion.

**As induction:** Take C = λn. P(n) for some predicate P. Then ind gives a proof of Π(n:ℕ).P(n) from P(zero) and (Π(n:ℕ).P(n) → P(succ(n))). This is mathematical induction.

Same principle. Different motives.

## Other Basic Inductive Types

**Bool:** two constructors, no recursive structure.
- true : Bool
- false : Bool
- Eliminator: ind_Bool(C, c_t, c_f, b) : C(b) for any b : Bool and C : Bool → Type
- Computation: ind_Bool(C, c_t, c_f, true) ≡ c_t; ind_Bool(C, c_t, c_f, false) ≡ c_f

**𝟙 (unit type):** one constructor.
- ⋆ : 𝟙
- Eliminator: ind_𝟙(C, c, u) ≡ c for u : 𝟙 and c : C(⋆). Every element of 𝟙 is ⋆.

**𝟘 (empty type):** no constructors.
- Eliminator: ind_𝟘(C, n) : C(n) for any n : 𝟘 and any C. This is ex falso: from a proof of falsehood (an element of the empty type), prove anything.

**List A:** a type family indexed by an element type A.
- nil : List A
- cons : A → List A → List A
- Eliminator: to define something for every list, handle nil and handle cons(a, l) given the result for l.

**Vec A n:** a type family indexed by both element type and length.
- nil : Vec A 0
- cons : A → Vec A n → Vec A (n+1)
- Eliminator: to define something for every Vec A n (for all n), handle nil (for n=0) and handle cons(a, v) given the result for v : Vec A n.

Notice that the Vec constructors are *typed differently* from List constructors: cons for Vec has type A → Vec A n → Vec A (n+1). The output type changes — this is an *indexed* inductive type, more powerful than a simple inductive type.

## The General Pattern: FIEC for Inductive Types

Every inductive type T follows the same pattern:

**Formation:** T is a type (or type family, in the indexed case).

**Introduction (Constructors):** A finite list of constructor functions, each of which produces elements of T by combining smaller elements of T (recursive arguments) with elements of other types (non-recursive arguments).

**Elimination (Induction Principle):** To define a dependent function out of T — to provide an element of C(t) for every t : T — it suffices to handle each constructor. For a recursive constructor c(x, t'), you get to assume the result for the sub-term t'.

**Computation:** Applying the eliminator to a constructor reduces to the corresponding handler.

This pattern is very general. Essentially all the data types and proof-relevant structures you encounter in mathematics fit within it (or mild extensions of it — see Higher Inductive Types later).

## W-Types: The Universal Inductive Type

Is there a single inductive type that encodes all others? Yes: the W-type (well-founded tree type).

Given A : Type and B : A → Type, the W-type W(x:A).B(x) is defined by:

**Single constructor:** sup : Π(a:A). (B(a) → W(x:A).B(x)) → W(x:A).B(x)

An element of W(x:A).B(x) is a tree with:
- A root labeled by some a : A
- Exactly B(a) many children, each of which is itself a W-tree

**Eliminator:** To prove C(t) for every t : W(x:A).B(x), handle the case t = sup(a, f): given a : A, the "branching function" f : B(a) → W(x:A).B(x), and the induction hypothesis Π(b:B(a)).C(f(b)), produce C(sup(a, f)).

**Encoding ℕ as a W-type:**
- A = Bool (the root can be "zero" or "succ")
- B(false) = 𝟘 (zero has no children)
- B(true) = 𝟙 (succ has one child, the predecessor)

Then sup(false, f) ≡ zero (f is vacuously defined) and sup(true, g) ≡ succ(g(⋆)) where g : 𝟙 → W is the single-child function.

Every strictly positive inductive type — roughly, those where the type being defined does not appear negatively in constructor argument types — can be encoded as a W-type. W-types are the "universal" inductive type.

In practice, proof assistants like Agda and Coq allow you to define inductive types directly (they provide native syntax for each inductive definition). The W-type encoding is mostly of theoretical interest, showing that only a small core of primitives is actually needed.

## Strictly Positive Inductive Types

Not every definition of the form "T has constructors that take T-valued arguments" is valid. The restriction is *strict positivity*: T may appear in constructor argument types only in positive positions — not under a negation or on the left of an arrow.

**Valid (strictly positive):** cons : A → List A → List A. Here List A appears positively (as an argument).

**Invalid (not strictly positive):** bad : (T → T) → T. Here T appears on the left of an arrow in the argument type, which is a negative position. This is rejected because it can lead to paradox (it essentially encodes fixed-point combinators that destroy normalization).

The restriction to strictly positive definitions ensures that the eliminator is well-defined and the computation rules preserve termination. In a system like Agda or Coq, the positivity checker automatically verifies this for each inductive definition.

## Inductive Types and the Termination Problem

In MLTT (without special extensions), all well-typed programs terminate. This is crucial for the logic to be consistent: if you could write a non-terminating program, you could produce an element of any type, including the empty type, making the system inconsistent.

The termination guarantee for inductive types comes from the structure of the eliminator: recursive calls in the step case are always applied to *smaller* arguments (sub-terms of the original input). This guarantees that recursion terminates.

This is the type-theoretic version of well-founded induction: every descending chain in the sub-term ordering is finite. Agda's termination checker verifies that recursive definitions only call themselves on structurally smaller arguments.

When you want to write a function that does not obviously terminate structurally — like a function that steps a Turing machine until it halts — you need to either prove termination explicitly (provide a well-founded ordering) or accept working in a setting with non-termination (like Coq with a `Fixpoint` that passes a fuel parameter). This is not a limitation; it is a feature. The logic is consistent precisely because non-termination is controlled.

## Looking Toward HoTT: Identity Types as Inductive Types

The identity type, which we study in Chapter 9, is itself an inductive type:

**Formation:** a =_A b : Type for a, b : A.

**Constructor:** refl_a : a =_A a.

**Eliminator:** the J rule.

Understanding identity types as inductive types clarifies why the J rule has the form it does: it is exactly the eliminator for an inductive type with one constructor (refl). And since the identity type is inductive, it fits the general framework of this section — the same FIEC pattern applies.

But identity types are special inductive types. They are *indexed* by two elements of A (the two endpoints of a path). The behavior of their eliminators is subtler than the natural numbers. And crucially, they can have multiple distinct elements — unlike Bool or 𝟘 or 𝟙, where the number of elements is fixed. This non-triviality is where HoTT begins.
