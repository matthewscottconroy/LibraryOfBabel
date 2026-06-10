# Applied Exercises

Identity types connect abstract homotopy theory to everyday programming in a surprisingly direct way. A proof that two programs are equal is a *path* between them in the type of programs, and transport along that path lets you substitute one for the other in any context. Refactoring becomes a formal notion. Testing becomes provability. The exercises below make these connections concrete, moving between formal proof assistants and practical software engineering, with the homotopy-theoretic picture always in the background.

---

## Exercise B.1: Implementing Path Operations in Agda
*Domain: Formal Verification / Functional Programming*

**Setup:** Agda's built-in equality type `_≡_` is exactly the identity type, with the single constructor `refl : a ≡ a`. All path operations must be derived from `J` (the eliminator). Agda provides `J` through pattern matching on `refl`, which gives you the computation rule directly: when `p : a ≡ a` is `refl`, you can assume `a` and the proof are in the base case.

In Agda, the three basic path operations are:

```agda
sym   : {A : Set} {a b : A} → a ≡ b → b ≡ a
trans : {A : Set} {a b c : A} → a ≡ b → b ≡ c → a ≡ c
cong  : {A B : Set} (f : A → B) {a b : A} → a ≡ b → f a ≡ f b
```

**Questions:**

1. Implement all three operations using only pattern matching on `refl` (no `subst` or `J` directly). For each, write out what the type of the goal becomes after you pattern match on `refl`, and explain why the base case is provable by `refl`.

2. Prove the groupoid laws in Agda: write proofs of `left-unit`, `right-unit`, `assoc`, `right-inv`, and `left-inv` (see §2.1 of the chapter). Which laws hold by `refl` (definitionally) and which require a proof by `J`? This asymmetry is not a bug — explain why it is a feature of how J computes.

3. Prove `cong₂ : (f : A → B → C) → a₁ ≡ a₂ → b₁ ≡ b₂ → f a₁ b₁ ≡ f a₂ b₂` from `cong` and `trans`. Then prove `ap-comp : cong (g ∘ f) p ≡ cong g (cong f p)`, which says that `ap` respects function composition. This is the statement that `ap` makes every function a "functor" on path spaces.

*Abstract concept illustrated: All path operations derive from J (path induction); the asymmetric computation rules for left vs. right unit reflect the asymmetric definition of concatenation by induction on the second argument.*

---

## Exercise B.2: Transport and Program Substitution
*Domain: Programming Language Theory / Software Refactoring*

**Setup:** Transport is the fundamental operation connecting identity types to the rest of dependent type theory. Given a type family $P : A \to \mathsf{Type}$ and a path $p : a =_A b$, transport gives $\mathsf{transport}^P(p, -) : P(a) \to P(b)$. In programming terms: if you prove that type $A$ is equal to type $B$ (as a path in the universe), you get a coercion function $A \to B$ for free.

Consider a practical scenario: you have defined `Vec A n` (vectors of length `n` over `A`) and `List A` (linked lists), and you have proved `length-roundtrip : ∀ (xs : List A) → toList (fromList xs) ≡ xs`. You want to transfer properties of `Vec` to `List` along this path.

**Questions:**

1. In Agda or Coq, define `subst : (P : A → Type) → a ≡ b → P a → P b` using pattern matching on `refl`. This is transport for a type family $P$. Now define `subst-sym : (P : A → Type) → a ≡ b → P b → P a` from `subst` and `sym`.

2. Define a predicate `IsSorted : List ℕ → Type` (a list is sorted in non-decreasing order). Using `subst` and `length-roundtrip` (or an appropriate analogous path), show how you could transport a proof `IsSorted-fromList : IsSorted (fromList xs)` to a proof about the original list. Identify precisely what type family $P$ you are transporting along.

3. More generally: in a large software codebase, a "refactoring" replaces one implementation `f : A` with another `g : A`. A proof `p : f ≡ g` (in the identity type of `A`) allows you to substitute `f` for `g` in any predicate `P : A → Type` via `subst P p : P f → P g`. Describe the limitations of this approach: for what kinds of predicates $P$ does `subst` transfer properties cleanly, and for what kinds does it give you a proof obligation that is hard to discharge? (Hint: consider predicates that talk about the internal implementation vs. predicates that only use the external interface.)

*Abstract concept illustrated: Transport $\mathsf{transport}^P(p, -)$ is the semantic content of the J rule — every property of a type is transferred along any path in the universe. In HoTT, this is related to univalence: a path `A ≡ B` in the universe gives a genuine equivalence, and transport is the underlying function.*

---

## Exercise B.3: The Encode-Decode Method and $\pi_1(S^1) = \mathbb{Z}$
*Domain: Algebraic Topology / Formal Mathematics*

**Setup:** One of the most striking computations in HoTT is the proof that the fundamental group of the circle is $\mathbb{Z}$: $\pi_1(S^1) \cong \mathbb{Z}$. This is done synthetically, without any topology. The circle $S^1$ is a higher inductive type with one point constructor `base : S^1` and one path constructor `loop : base ≡ base`. The encode-decode method computes the path space `base ≡ base` by finding a "code" type and showing it corresponds to $\mathbb{Z}$.

**Questions:**

1. Define `code : S^1 → Type` by `code base = ℤ` and `transport code loop = succ` (applying `loop` to the code at `base` maps integers by +1). Explain why defining `code` requires eliminating from `S^1` (using the HIT recursion principle), and why this works: the path constructor `loop` means we need `code base ≡ code base` as a path in `Type`, and this is given by the univalence image of the successor equivalence.

2. Define the "encode" function: `encode : (x : S^1) → base ≡ x → code x` by `encode x p = transport code p 0`. For `x = base`, this maps a loop `p : base ≡ base` to an integer: the "winding number" of the loop. Show that `encode base refl = 0` and that `encode base loop = 1` and `encode base (loop ∙ loop) = 2`. (Use the fact that `transport code loop = succ`.)

3. Define the "decode" function: `decode : (x : S^1) → code x → base ≡ x` by `decode base n = loop^n` (the $n$-fold concatenation of `loop`). This requires working by cases on whether $n$ is positive, zero, or negative (for $n < 0$, use the inverse loop). Assuming both `encode-decode` and `decode-encode` round-trips hold (they require full HIT induction principles to prove), state precisely what it means that `(base ≡ base) ≃ ℤ` — write out the type of this equivalence and identify what theorem from Chapter 4 (equivalences) it uses.

*Abstract concept illustrated: The encode-decode method reduces the problem of identifying a path space $a = b$ to finding a suitable "code" type and proving the two maps are inverse. It is the main computational tool for synthetic homotopy theory in HoTT.*

---

## Exercise B.4: Identity Types for Compiler Correctness
*Domain: Compiler Verification / Programming Language Semantics*

**Setup:** Compiler correctness says that a compiler $C$ mapping source programs $s : \mathsf{Src}$ to target programs $t : \mathsf{Tgt}$ is correct if, for every source program $s$, the semantics of $s$ equals the semantics of $C(s)$: $\llbracket s \rrbracket_{\mathsf{Src}} = \llbracket C(s) \rrbracket_{\mathsf{Tgt}}$. In dependent type theory, this can be phrased as an identity type: $\llbracket s \rrbracket = \llbracket C(s) \rrbracket$ (in the type of semantic values).

This is a precise, formal statement. Consider a tiny arithmetic language: source expressions are `Expr` (natural number constants, addition, multiplication), target "programs" are `Stack` (reverse-Polish notation programs), and the semantics maps both to $\mathbb{N}$.

**Questions:**

1. Define `eval : Expr → ℕ` and `run : Stack → List ℕ → List ℕ` (the stack machine interpreter). Define a compilation function `compile : Expr → Stack`. State the correctness theorem as an identity type: what is the precise type of `correct : (e : Expr) → eval e ≡ ?`.

2. Prove the correctness theorem by induction on the structure of `e : Expr`. For `e = Add e₁ e₂`, use `cong₂ (+) (correct e₁) (correct e₂)` — applying `ap` to the two sub-proofs. Identify exactly which path operations (`cong`, `trans`, `sym`) you use at each step of the proof, and relate them to the algebraic structure of the identity type.

3. The correctness proof constructs a *path* in $\mathbb{N}$ between two computations. Using `subst`, explain how this path can be used to transfer any property $P : \mathbb{N} \to \mathsf{Type}$ from the source semantics to the target semantics. As a concrete example: if `IsPrime : ℕ → Type` is the predicate "is prime", and you know `IsPrime (eval e)`, derive `IsPrime (run (compile e) [])` without reproving primality.

*Abstract concept illustrated: Compiler correctness is a statement about identity types (semantic equality of source and target). The J rule and transport give the formal machinery for substituting one semantics for another in any predicate — the type-theoretic version of the Liskov substitution principle.*

---

## Exercise B.5: Hedberg's Theorem in Practice
*Domain: Proof Assistant Programming / Decidable Equality*

**Setup:** Hedberg's theorem says: if $A$ has decidable equality (for all $x, y : A$, either $x = y$ or $x \neq y$, constructively), then $A$ is an h-set (any two paths $p, q : x = y$ are equal). This is crucial in practice: it tells you that "ordinary" types like $\mathbb{N}$, $\mathsf{Bool}$, `List A` (when $A$ has decidable equality) are sets, so you never need to worry about proof relevance for equality in these types.

**Questions:**

1. Implement `Nat-DecEq : DecEq ℕ` — decidable equality for natural numbers — in Agda or Coq by induction. Handle the four cases: `zero ≡? zero`, `zero ≡? succ n`, `succ m ≡? zero`, `succ m ≡? succ n`. For the last case, use the induction hypothesis and lift the result from `m ≡? n` to `succ m ≡? succ n`.

2. State and prove the key lemma in Hedberg's theorem: if $f : (x = y) \to (x = y)$ is a constant function (satisfying $\prod_{p, q : x=y} f(p) = f(q)$), then $x = y$ is a mere proposition (any two paths $p, q$ are equal). [Hint: use the fact that $f(p) = p$ is provable from $f$ being a *retraction* up to a higher path, which follows from $f$'s constancy by combining $p = f(p)^{-1} \cdot f(p) \cdot ?$... work this out carefully.]

3. Using `Nat-DecEq` and Hedberg's theorem (you may state Hedberg's theorem and apply it as a black box), conclude `Nat-isSet : isSet ℕ`. Then use this to prove: for any $m, n : \mathbb{N}$ and $p, q : m = n$, we have `p ≡ q`. Run this on a concrete example in Agda's normalizer or Coq's `compute` to see what the proof term looks like. Discuss: in an extensional type theory, this would be *definitionally* true; in intensional HoTT, it is *propositionally* true but requires Hedberg's theorem.

*Abstract concept illustrated: Hedberg's theorem characterizes the h-sets among types with constructive equality, and its proof is the prototype for the "constant endofunction" technique used throughout HoTT metatheory.*

---

## Exercise B.6: Dependent Paths and Module Refactoring
*Domain: Software Architecture / Module Systems*

**Setup:** In dependent type theory, when you change a base type, all types that depend on it change too. A path $p : A =_{\mathsf{Type}} B$ between types induces transport maps between all type families over $A$ and $B$. This models a real software phenomenon: when you change an interface (a type), you must update all implementations that depend on it. Dependent paths (PathP in cubical HoTT, or `HEq` in standard HoTT) formalize the idea of "a proof that two things are equal, even though they live in different types that are themselves related by a path."

Consider a module system: a "module type" `MT : Type → Type` parametrizes over a carrier type. Two implementations `impl₁ : MT A` and `impl₂ : MT B` over different carrier types `A` and `B` can only be compared if `A` and `B` are related.

**Questions:**

1. Define the "heterogeneous equality" type `HEq : (A B : Type) → A ≡ B → A → B → Type` where `HEq A B p a b` is the type of paths from `a` to `b` over the path `p`. Show how to define it using transport: `HEq A B p a b = (transport (id) p a ≡ b)`. Prove that `HEq A A refl a b ≃ (a ≡ b)` — heterogeneous equality over `refl` is the same as ordinary equality.

2. Suppose you have two implementations of a stack data structure: `Stack₁` using lists and `Stack₂` using arrays, with a proof `equiv : Stack₁ ≡ Stack₂` (as types). Using `transport`, construct a function `coerce-stack : Stack₁ → Stack₂`. In what sense is this function "canonical"? (Hint: transport along `equiv` is the unique map that respects all proofs about stacks — it transports every predicate.) Compare this to the ad-hoc coercions you might write in an unverified setting.

3. In a large codebase, a "breaking change" to a module replaces its interface type $A$ with a new type $B$. In HoTT, a path $p : A = B$ represents a non-breaking change: the types are homotopy equivalent, and everything can be transported. A breaking change would be represented by the absence of such a path. Describe how the machinery of transport and dependent paths could be used in a hypothetical proof-carrying code system to certify that a library update is backward compatible (i.e., that the new interface is homotopy equivalent to the old one, with explicit transport functions for all dependent types).

*Abstract concept illustrated: Dependent paths (PathP) and heterogeneous equality formalize substitution along a path in a type family; they arise naturally whenever the endpoints of a path live in different fibers of a dependent type — the fundamental structure underlying transport.*
