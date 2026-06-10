# Applied Exercises

The Curry-Howard correspondence is not an abstract curiosity — it has immediate, practical consequences for how we write programs, design type systems, and structure proofs. The exercises below ground the correspondence in concrete domains: statically-typed functional programming, proof extraction, encoding data invariants in types, and the derivation of algorithms from their specifications. In each case, the goal is to see that propositions, proofs, types, and programs are not four different things but four names for one thing, encountered from different vantage points.

---

## Exercise A.1: Propositions as Types in Haskell
*Domain: Functional Programming / Type-Level Programming*

**Setup:** In Haskell, a type like `(a, b) -> (b, a)` is both a type and a proposition — the proposition "if A and B, then B and A." The function `\p -> (snd p, fst p)` is both a program and a proof. This exercise explores how far this identification goes within Haskell's type system, using standard Haskell without any extensions.

**Questions:**
1. For each of the following propositions, write a Haskell type signature and a function (proof term) that inhabits it. Do not use `undefined`. Verify that your functions type-check.
   - $(P \wedge Q) \to (Q \wedge P)$  (commutativity of conjunction)
   - $(P \to Q) \to (P \to R) \to P \to (Q \wedge R)$  (product of maps)
   - $(P \vee Q) \to (P \to R) \to (Q \to R) \to R$  (case analysis / disjunction elimination)
   - $P \to \neg\neg P$  (double negation introduction; recall $\neg P = P \to \bot$, and in Haskell `type Not a = a -> Void`)
2. Attempt to write a Haskell function of type `Either a (a -> Void) -> b`. Does one exist? What logical principle would this correspond to? Why can't you write it without `undefined` or `error`?
3. Using `Data.Void` (the empty type) and the fact that `absurd :: Void -> a` corresponds to ex falso ($\bot \to A$), write a function that proves: if $\neg P$ and $P$, then $Q$. What is the proof term? Reduce it manually (beta-expand and contract) to see what computation it encodes.

*Abstract concept illustrated: The full dictionary of Section 1 — conjunction/product, implication/function type, disjunction/sum type, falsehood/empty type.*

---

## Exercise A.2: Proof Terms and Program Extraction in Coq/Agda
*Domain: Verified Software / Proof Assistants*

**Setup:** In Coq, the command `Extraction` converts a proof term into an executable OCaml program. This exercise makes concrete the claim that proofs *are* programs: the proof of a constructive existence statement is, when extracted, a function that computes the witness. We work with the maximum function as a running example.

**Questions:**
1. In Coq (or Agda), prove the following without using any automation tactics — write out the proof term directly using `fun`, `match`, and constructor applications:
   ```coq
   Theorem max_exists : forall (m n : nat), 
     { k : nat | k >= m /\ k >= n }.
   ```
   After proving it, use `Extraction max_exists` and inspect the extracted OCaml code. What function does it compute?

2. Now prove a stronger version: not just that a maximum exists, but that it equals one of the two inputs:
   ```coq
   Theorem max_is_one_of : forall (m n : nat),
     { k : nat | (k = m \/ k = n) /\ k >= m /\ k >= n }.
   ```
   Again extract the proof and verify the OCaml function is a correct max implementation.

3. The extracted code is guaranteed to be correct *by construction* — the type checker verified the proof. But is the extracted code efficient? Compare the extracted code with a hand-written `max` function. What overhead, if any, does the proof term introduce? What does this tell you about the relationship between logical content and computational content?

*Abstract concept illustrated: Proof normalization as computation; the BHK (Brouwer-Heyting-Kolmogorov) interpretation of existential statements.*

---

## Exercise A.3: Encoding Invariants — Type-Safe Non-Empty Lists
*Domain: Type System Design / Haskell*

**Setup:** A common runtime error is calling `head` on an empty list. Under the Curry-Howard correspondence, the *type* of `head` encodes a proposition: "given a non-empty list, return the first element." If we encode non-emptiness in the type, the type checker enforces the precondition and `head` becomes total. This exercise develops the pattern of *indexed types* — the bridge from STLC to dependent types.

**Questions:**
1. Define a Haskell data type `NEList a` for non-empty lists. One clean encoding uses the structure of the list directly:
   ```haskell
   data NEList a = NEList a [a]   -- head and (possibly empty) tail
   ```
   Define `head :: NEList a -> a`, `tail :: NEList a -> [a]`, and `toList :: NEList a -> [a]`. Verify these are total functions (no `undefined`, no pattern match warnings).

2. Define `fromList :: [a] -> Maybe (NEList a)`. This is the only place where partiality needs to be handled — and it's forced into the type as a `Maybe`. Write `safeMin :: Ord a => NEList a -> a` (the minimum element of a non-empty list) without using `Maybe`.

3. Under the Curry-Howard correspondence, what proposition does the type `[a] -> Maybe (NEList a)` prove? Write out the logical statement in natural language. Now consider: can you write a version `fromNonEmpty :: { xs : [a] | xs /= [] } -> NEList a` in standard Haskell? What language feature would you need? (This is the segue to dependent types: the precondition `xs /= []` would become a type-level constraint.)

*Abstract concept illustrated: Type refinement and the representation of invariants as types; connection to Sigma types (Section 1.4, the predicate logic extension of Curry-Howard).*

---

## Exercise A.4: Sorting by Specification — Deriving an Algorithm from Its Type
*Domain: Algorithm Design / Verified Programming*

**Setup:** The Curry-Howard correspondence suggests an extreme programming discipline: *write the type of your function as a proposition, then derive the implementation by constructing the proof.* This exercise applies that discipline to insertion sort. The type of a sorting function states both that the output is sorted and that it is a permutation of the input; the implementation follows from the structure of the proof.

**Questions:**
1. In Agda or Lean 4, define the following:
   - A type `Sorted : List Nat -> Set` (or `Prop`) expressing that a list is non-decreasingly ordered.
   - A type `IsPermutation : List Nat -> List Nat -> Set` expressing that two lists are permutations of each other (hint: the simplest inductive definition uses the `_∈_` relation).
   - The type of a correct sorting function:
     ```agda
     sort-spec : (xs : List Nat) -> Sigma (List Nat) (lambda ys -> Sorted ys x IsPermutation xs ys)
     ```

2. Implement insertion sort with the type above — that is, prove the specification. Your implementation must produce both the sorted list and a proof of correctness. Where in the proof does the case analysis on list structure appear? How does the inductive step use the induction hypothesis?

3. The type `sort-spec` says the output is sorted and is a permutation. These two conditions together characterize sorted permutations. Is there a function of this type that simply returns the empty list for all inputs? Why or why not? (This question asks you to think carefully about what the type actually requires.)

*Abstract concept illustrated: The Curry-Howard correspondence as a programming methodology; Sigma types as specifications; the connection between program correctness and proof normalization.*

---

## Exercise A.5: The Böhm-Berarducci Encoding — Data Structures as Proofs
*Domain: Type Theory / Functional Programming*

**Setup:** In System F (second-order lambda calculus), every inductive data type can be *encoded* as a polymorphic function type. This is the Böhm-Berarducci encoding (the System F version of Church encodings). The encoded type is a proposition in second-order logic, and the encoded data is a proof. This exercise develops the encoding for lists and shows that it is computationally adequate.

**Questions:**
1. The Böhm-Berarducci encoding of `List a` in System F is:
   ```
   List a = forall r. r -> (a -> r -> r) -> r
   ```
   (Read: "a list is something that, given a nil-case and a cons-case, produces a result of any type $r$.")
   In Haskell with `RankNTypes`, write:
   ```haskell
   type BBList a = forall r. r -> (a -> r -> r) -> r
   ```
   Define `bbNil :: BBList a`, `bbCons :: a -> BBList a -> BBList a`, and `bbToList :: BBList a -> [a]`. Verify your definitions are correct by testing `bbToList (bbCons 1 (bbCons 2 bbNil)) == [1, 2]`.

2. Now define `bbFoldr :: (a -> b -> b) -> b -> BBList a -> b`. Notice that the implementation is trivial — it follows directly from the type. This is the "theorem for free": the type of `BBList a` *is* the type of a fold, so the fold implementation is forced on you. What proposition does the type `BBList a` prove? (Hint: what is `forall r. r -> (a -> r -> r) -> r` as a logical statement if `a` and `r` are propositions?)

3. Define `bbMap :: (a -> b) -> BBList a -> BBList b` without converting to and from `[a]`. Show that your definition satisfies the functor laws (`bbMap id = id` and `bbMap (f . g) = bbMap f . bbMap g`) — as *Haskell equalities*, not proofs. Then explain why these laws are *guaranteed* by parametricity (the type of `bbMap` uniquely determines its behavior).

*Abstract concept illustrated: Church encodings; parametricity (Reynold's "free theorems"); the encoding of inductive types as propositions in System F.*

---

## Exercise A.6: Classical Logic and Control — call/cc as a Proof
*Domain: Programming Language Semantics / Continuations*

**Setup:** Chapter 6, Section 1 noted that Peirce's law $((A \to B) \to A) \to A$ corresponds computationally to the control operator `call/cc` (call with current continuation). This exercise makes that concrete. Peirce's law is not provable in intuitionistic logic — there is no lambda term of that type in STLC. But in languages with first-class continuations (Scheme, Ruby, certain ML dialects), you can write a function `callcc` that has exactly that type. This exercise explores what it means to "add classical logic" to a programming language.

**Questions:**
1. In Scheme (or any language with `call-with-current-continuation`), implement a function `call-cc-peirce` that, given a function `f : (a -> b) -> a`, returns an `a`. The implementation uses `call/cc`. Write it out and trace the execution on a concrete example: let `a = Int`, `b = Int`, and `f = fun (k : Int -> Int) -> k 42 + 1`. What does your function return? What does `k` do when called?

2. Attempt to write a function of type `((a -> b) -> a) -> a` in Haskell (without `undefined`, `error`, or `unsafeCoerce`). Explain precisely why it is impossible. What proof-theoretic property of the simply typed lambda calculus (proved in Section 3) rules it out?

3. The continuation monad in Haskell has type `newtype Cont r a = Cont { runCont :: (a -> r) -> r }`. Show that `Cont r` is a monad. Then show that the *Peirce's law operator* can be written as a monadic operation in `Cont r`:
   ```haskell
   callCC :: ((a -> Cont r b) -> Cont r a) -> Cont r a
   ```
   Implement `callCC`. What does this implementation tell you about the relationship between monads, continuations, and classical logic?

*Abstract concept illustrated: Classical logic and computation; control operators as proof-theoretic constructs; the double-negation translation (classical logic embeds in intuitionistic logic via the continuation monad).*
