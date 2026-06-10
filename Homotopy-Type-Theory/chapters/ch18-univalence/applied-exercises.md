# Applied Exercises

The univalence axiom might seem like a purely foundational result — a philosophical statement about what equality *means* for types — but it has concrete consequences wherever types are used to model real computational and mathematical structures. The exercises below connect univalence to practical software engineering (transporting proofs across equivalent data structure implementations), verified mathematics (the structure identity principle for groups), type-safe generic programming, and the foundations of function equality in programming language semantics. Each exercise draws on the formal machinery of Section 2–3: the `ua` function, transport, function extensionality as a consequence of univalence, and the SIP.

---

## Exercise A.1: Transporting a Proof from Association Lists to Hash Maps
*Domain: Software Engineering / Verified Data Structures / Proof Reuse*

**Setup:** A common software engineering scenario: you have a correct, verified implementation of a key-value store as an association list (a list of pairs), and you want to switch to a hash map for efficiency. In a dependently typed language, the association list might carry a proof — say, a proof that all keys are distinct, or that the lookup function satisfies a specification. Univalence gives a principled way to *transport* proofs from one representation to the other without re-proving them.

Concretely, let `AssocList(K, V)` be the type of association lists with keys of type `K` and values of type `V`, and let `HashMap(K, V)` be an abstract hash map type. Suppose there is an equivalence `e : AssocList(K, V) ≃ HashMap(K, V)` (i.e., a bijection that preserves the lookup and insert operations).

**Questions:**
1. Using `ua(e) : AssocList(K, V) = HashMap(K, V)`, explain how to transport a proof `p : P(AssocList(K, V))` — where `P` is any type-theoretic predicate — to a proof `transport^P(ua(e), p) : P(HashMap(K, V))`. What is the formal type of `transport^P`? Write the transport computation explicitly.
2. Let `lookup-correct : ∀ (k : K)(m : AssocList(K, V)), lookup k (insert k v m) = some v` be the correct specification for the association list. Show how to produce the analogous statement for `HashMap(K, V)` using transport along `ua(e)`, without re-proving the specification from scratch.
3. (Extension) In practice, the equivalence `e` must respect the operations (lookup, insert, delete) — that is, it must be a *structure equivalence*, not just a set-theoretic bijection. Formalize what "respects lookup" means as a type, and explain why a bare equivalence of types (without this condition) would not give you a useful transported proof about the hash map's lookup function.

*Abstract concept illustrated: transport along `ua(e)`, and the requirement that equivalences used with the SIP carry proof-relevant structure.*

---

## Exercise A.2: The Structure Identity Principle for Groups
*Domain: Abstract Algebra / Verified Mathematics / Formal Foundations*

**Setup:** The structure identity principle (SIP) is the theorem that, for a suitably formalized type of mathematical structures, *equality coincides with isomorphism*. For groups, the natural formalization in HoTT is:

$$\mathsf{Group} :\equiv \sum_{G : \mathsf{hSet}} \sum_{(\cdot) : G \to G \to G} \sum_{e : G} \sum{i : G \to G} \mathsf{GroupAxioms}(G, \cdot, e, i)$$

where `GroupAxioms` is the conjunction of associativity, identity laws, and inverse laws. Two groups $(G_1, \cdot_1, e_1, i_1, \alpha_1)$ and $(G_2, \cdot_2, e_2, i_2, \alpha_2)$ are equal as elements of `Group` iff they are isomorphic as groups.

**Questions:**
1. Write out the type `Group` as a $\Sigma$-type in full detail. What does an element of `Group` consist of?
2. Using univalence, show that two groups $(G_1, \mathsf{str}_1)$ and $(G_2, \mathsf{str}_2)$ are equal in `Group` iff there is an equivalence `e : G_1 ≃ G_2` that is a *group homomorphism* — i.e., such that `e(a ·₁ b) = e(a) ·₂ e(b)` for all `a, b : G₁`. (Hint: use the characterization of equality in $\Sigma$-types: `(a₁, b₁) = (a₂, b₂)` iff there exists `p : a₁ = a₂` and `transport^B(p, b₁) = b₂`.)
3. (Extension) Consider the cyclic group $\mathbb{Z}/6\mathbb{Z}$ and the product group $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/3\mathbb{Z}$. These are isomorphic groups (by the Chinese Remainder Theorem). Using the SIP, conclude that they are *equal* as elements of `Group`. What does this equality say, concretely, about transporting a proof that $\mathbb{Z}/6\mathbb{Z}$ is abelian to $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/3\mathbb{Z}$?

*Abstract concept illustrated: the structure identity principle, equality in $\Sigma$-types, transport of algebraic structure.*

---

## Exercise A.3: Equivalent Encodings of Binary Trees
*Domain: Functional Programming / Type-Safe Data Representation / Compiler Internals*

**Setup:** Binary trees can be encoded in multiple ways, and often different parts of a codebase use different representations. Consider:

- `Tree₁(A)`: the standard inductive binary tree with labeled leaves — either `Leaf(a : A)` or `Node(Tree₁(A), Tree₁(A))`.
- `Tree₂(A)`: trees stored as lists of elements in their in-order traversal, together with a "shape" encoding the branching structure (a separate unlabeled tree `Shape`), so `Tree₂(A) = Shape × List(A)`.

There is a natural equivalence `e : Tree₁(A) ≃ Tree₂(A)` for any type `A` with a list (the in-order traversal is a bijection between trees with $n$ leaves and pairs of a shape with $n$ leaves and an $n$-element list).

**Questions:**
1. Describe the functions `encode : Tree₁(A) → Tree₂(A)` and `decode : Tree₂(A) → Tree₁(A)` that witness the equivalence, and sketch proofs that `decode ∘ encode ~ id` and `encode ∘ decode ~ id`.
2. Suppose you have proved a function `flatten : Tree₁(A) → List(A)` correct — specifically, a proof `flatten-correct : ∀ t, length(flatten t) = leaf-count(t)`. Using `ua(e) : Tree₁(A) = Tree₂(A)`, transport this proof to obtain `flatten-correct₂ : ∀ t : Tree₂(A), length(flatten₂ t) = leaf-count₂(t)`. What is `flatten₂`? What is `leaf-count₂`? (They are defined by transporting `flatten` and `leaf-count` along `ua(e)`.)
3. (Extension) In a real compiler, the intermediate representation (IR) of an expression often changes between compilation passes. Model two IR representations as types `IR₁` and `IR₂` and an equivalence `e : IR₁ ≃ IR₂`. What conditions on `e` would be necessary for transporting a correctness proof (e.g., "every expression in IR₁ that is well-typed is safe to evaluate") to remain valid in IR₂?

*Abstract concept illustrated: transport of proofs along `ua(e)`, computational content of equivalences.*

---

## Exercise A.4: Function Extensionality in Programming Language Semantics
*Domain: Programming Language Theory / Denotational Semantics / Proof Assistants*

**Setup:** Function extensionality — the principle that `f = g` iff `∀ x, f(x) = g(x)` — is a consequence of univalence in HoTT, but it is *not provable* in standard MLTT without additional axioms. This has practical consequences: in a proof assistant based on standard MLTT (like Coq without axioms, or Lean 4 without `funext`), two functions that are definitionally equal on all inputs may still not be *propositionally* equal without an additional invocation of `funext`.

Consider the two functions:
- `sum₁ : List(ℕ) → ℕ`, defined by left fold: `sum₁ [] = 0`, `sum₁ (x :: xs) = x + sum₁ xs`.
- `sum₂ : List(ℕ) → ℕ`, defined by right fold with an accumulator: `sum₂ xs = fold_right 0 (+) xs`.

These agree on all inputs (provable by list induction), so `∀ xs, sum₁ xs = sum₂ xs`. By function extensionality, `sum₁ = sum₂` as functions `List(ℕ) → ℕ`.

**Questions:**
1. Write a formal statement of function extensionality as a type. Why is this type not provable by the basic rules of MLTT (path induction, `ap`, transport)? What specifically is missing?
2. Using the HoTT proof of function extensionality from univalence (sketch: apply `ua` to the constant family equivalence and use the transport computation), explain informally why `sum₁ = sum₂` follows. You do not need to give the full formal proof, but identify the key steps.
3. (Extension) In denotational semantics, two programs are *semantically equivalent* if their denotations agree on all inputs. If we use a type theory with function extensionality as our meta-language, semantic equivalence of programs corresponds directly to equality of their denotation functions. Explain why this is important for the *compositionality* of program equivalences: if `f₁ = f₂` and `g₁ = g₂`, then `g₁ ∘ f₁ = g₂ ∘ f₂`. Where does function extensionality appear in this argument?

*Abstract concept illustrated: function extensionality as a consequence of univalence, the difference between definitional and propositional equality.*

---

## Exercise A.5: Type-Safe Generic Programming via Univalence
*Domain: Generic Programming / Software Engineering / Type Theory*

**Setup:** Generic programming is the practice of writing code that works uniformly over a class of types, relying only on a shared interface. In dependently typed languages, generic programs can be written by quantifying over types with specified structure. Univalence sharpens this: because equivalent types are equal, a generic program does not need to separately handle "isomorphic but syntactically different" types — they are the same type.

Consider a generic `sort` function that works over any type `A` with a decidable linear order `≤`. Given two implementations of the integers, `ℤ_standard` (as pairs of natural numbers modulo an equivalence relation) and `ℤ_gmp` (as a more efficient representation using machine integers), with an equivalence `e : ℤ_standard ≃ ℤ_gmp` that preserves the order, we want to conclude that `sort` works correctly over `ℤ_gmp` because it works over `ℤ_standard`.

**Questions:**
1. Formalize the type of "ordered types": `OrderedType ≡ Σ(A : Type), Σ(≤ : A → A → Prop), LinearOrder(A, ≤)`. What does an element of `OrderedType` consist of? Write it out as nested $\Sigma$-types.
2. Suppose `sort-correct(A, ≤) : ∀ xs : List(A), Sorted(≤, sort(A, ≤, xs)) × Permutation(xs, sort(A, ≤, xs))` is proved for `(ℤ_standard, ≤_std) : OrderedType`. Using univalence and the fact that `e` preserves the order (so that `(ℤ_standard, ≤_std) = (ℤ_gmp, ≤_gmp)` in `OrderedType` by the SIP), transport `sort-correct` to `ℤ_gmp`.
3. (Extension) Real generic programming often involves *type classes* or *interfaces*, not just bare equivalences of types. Explain the relationship between type classes in Haskell/Lean 4 and the $\Sigma$-type formalization of structured types in HoTT. When does the SIP apply, and when does it fail to apply (e.g., for type classes with operations that are not pure functions)?

*Abstract concept illustrated: the structure identity principle, equality in the type of structured types, generic programming via transport.*

---

## Exercise A.6: The Two Paths on Bool and Bit-Level Encoding
*Domain: Systems Programming / Data Representation / Verified Cryptography*

**Setup:** Univalence tells us that the paths `Bool = Bool` in the universe correspond exactly to the equivalences `Bool ≃ Bool`. There are exactly two such equivalences: the identity `id` and the negation `neg`. Correspondingly, there are exactly two paths `Bool = Bool`: `refl_Bool = ua(id)` and `ua(neg)`.

This has a concrete computational interpretation. In verified cryptography, the type `Bit ≡ Bool` is used to represent single bits, and the two representations "bit is 0 iff True" and "bit is 0 iff False" are both common (different hardware or protocol conventions). These two conventions correspond exactly to the two paths `Bool = Bool`.

**Questions:**
1. State and prove the theorem that `Bool ≃ Bool` has exactly two elements: `id_Bool` and `neg`. (Hint: an equivalence `Bool → Bool` must be determined by its values on `true` and `false`, and since it is a bijection, either it is `id` or it is `neg`.)
2. By univalence, `Bool = Bool` has exactly two elements: `refl` and `ua(neg)`. Show that `transport^id_Type(ua(neg), true) = false` — i.e., that transporting `true` along the non-trivial path gives `false`. This is the formal statement that the two bit conventions are genuinely different.
3. (Extension) Consider the type `Bit-Protocol : Bool = Bool → Specification` that assigns to each bit convention a formal protocol specification. If `Bit-Protocol(refl)` is the specification "a message with bit value `true` means ACCEPT," what is `Bit-Protocol(ua(neg))`? More generally, given a protocol specified in terms of `Bool`, what does transport along `ua(neg)` do to the protocol? How does this formalize the common cryptographic concern about "endianness" or "encoding convention" mismatches?

*Abstract concept illustrated: the two paths on Bool, transport as re-encoding, the computational content of `ua`.*
