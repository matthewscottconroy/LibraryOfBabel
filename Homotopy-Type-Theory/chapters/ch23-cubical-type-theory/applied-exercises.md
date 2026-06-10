# Applied Exercises

Cubical type theory is not merely a solution to a foundational problem — it is a theory with concrete computational implications. The canonicity theorem guarantees that programs written in cubical type theory have a well-defined operational semantics; the Glue type makes it possible to reason about equivalences in a way that computes; and the De Morgan algebra on the interval turns out to model structures that appear in distributed systems and concurrency. The exercises below develop these connections, ranging from implementing a small type checker to tracing through the univalence computation rule to exploring the interval algebra in a concurrent programming context.

---

## Exercise A.1: A Minimal Cubical Type Checker
*Domain: Programming Language Implementation / Type Theory Engineering*

**Setup:** The CCHM theory has precise reduction rules for `transp` and `hcomp`. Implementing these rules in a small type checker is the most direct way to understand them: you must give a precise account of every case, handle dimension variables and face formulas correctly, and verify that the rules compose properly. This exercise builds a miniature cubical type checker for a fragment of the theory.

**Questions:**
1. Define the syntax of CCHM terms in a language of your choice (Haskell, OCaml, or Agda itself using a deep embedding). You need at minimum:
   - Types: `ℕ`, `Path A u v`, `Sigma A B`, `Pi A B`
   - Terms: variables, `zero`, `suc`, `natrec`, `transp`, `hcomp`, `refl` (= `λ i → a`)
   - Dimension expressions: `I`, `i0`, `i1`, `~ r`, `r ∧ s`, `r ∨ s`
   - Face formulas: `⊥`, `⊤`, `(i = 0)`, `(i = 1)`, `φ ∧ ψ`, `φ ∨ ψ`
   Write the data types. How do you represent dimension variables in the context? What is the type of a partial element `[φ ↦ t]`?

2. Implement the reduction rule for `transp` on path types. The rule is:
   ```
   transp (λ i → Path (A i) (u i) (v i)) φ p
       = λ j → hcomp (A 1) [φ ↦ p j, j=0 ↦ transp (λ i → A i) φ (u 0),
                                       j=1 ↦ transp (λ i → A i) φ (v 0)]
                    (transp (λ i → A i) (φ ∨ ~j ∨ j) (p j))
   ```
   (This is Huber's thesis, Section 6.2.) Implement this rule as a recursive function `evalTransp : Ty → DimExpr → Term → Term`. Where does the recursion bottom out? What does the termination argument look like?

3. Implement the reduction rule for `transp` on `ℕ`:
   ```
   transp (λ _ → ℕ) φ n = n
   ```
   This is the simplest case: transporting over a constant family is the identity. Now implement `transp` on `Sigma`:
   ```
   transp (λ i → Σ (A i) (B i)) φ (a, b) =
       ( transp (λ i → A i) φ a
       , transp (λ i → B i[a₀/x]) φ b )
   ```
   where `a₀` is the transported base and the fiber is re-evaluated along the transported base path. Test your implementation on a concrete example: `transp (λ i → Σ ℕ (λ _ → ℕ)) i0 (3, 4)` should reduce to `(3, 4)`.

*Abstract concept illustrated: The reduction rules for `transp` as a recursive definition on the structure of types; the type-directedness of the rules and why they require a complete case analysis on type formers.*

---

## Exercise A.2: Tracing the Univalence Computation Rule
*Domain: Mathematical Formalization / Type Theory*

**Setup:** The key payoff of CCHM is that `transport (ua e) a` definitionally reduces to `e a` — transport along the path produced by univalence is the same as applying the equivalence. This exercise traces through this computation, making the reduction steps explicit, so the "magic" becomes transparent mechanism.

**Questions:**
1. Write out, in mathematical notation (not Agda), the definition of `ua : A ≃ B → A ≡ B` in CCHM. The path `ua e : A ≡ B` is defined as:
   ```
   ua e = λ i → Glue B [(i=0) ↦ (A, e), (i=1) ↦ (B, id)]
   ```
   where `Glue B [(i=0) ↦ (A, e), (i=1) ↦ (B, id)]` is a type that:
   - At `i=0` is `A` (via the equivalence `e : A ≃ B`)
   - At `i=1` is `B` (via the identity equivalence)
   - At intermediate `i` is some "gluing" of `A` and `B`
   
   Verify that this path has the right endpoints: what is `ua e i0`? What is `ua e i1`? (These should reduce definitionally to `A` and `B` respectively.)

2. Now trace the computation of `transport (ua e) a` for a specific example. Let `A = ℕ`, `B = ℕ`, and `e = succEquiv : ℕ ≃ ℕ` (the equivalence sending `n` to `n+1`). The term is:
   ```
   transport (ua succEquiv) 3
   ```
   Step through the reduction:
   - Step 1: Unfold `ua succEquiv` to get a path `λ i → Glue ℕ [...]`
   - Step 2: Apply the `transp` computation rule for Glue types
   - Step 3: The Glue computation rule says `transp (λ i → Glue B [φ ↦ (T, e)]) i0 a = e.fun a`
   
   What is the final reduced form? Is it `4`? (It should be: `transport (ua succEquiv) 3 ≡ succEquiv.fun 3 = 4`.) Write out each step with the typing judgment.

3. The `uaβ` computation rule is the name given to the reduction `transport (ua e) a ≡ e.fun a`. It is a *definitional* equality in CCHM — it holds by reduction, not by a proof. In the HoTT Book (with axiomatic univalence), this is only a *propositional* equality (it must be assumed as an additional axiom called the "univalence axiom's computation rule"). Explain in precise terms why the CCHM definition gives `uaβ` definitionally: which step of the computation uses the Glue reduction rule? What is the Glue reduction rule, and how does it apply to the `i=0` face?

*Abstract concept illustrated: The Glue type as a mechanism for computable univalence; the reduction `uaβ : transport (ua e) a ≡ e a` as a definitional equality.*

---

## Exercise A.3: Heterogeneous Equality and Generalized Rewriting
*Domain: Type-Safe Programming / Programming Language Design*

**Setup:** In dependently typed programming, it is often necessary to prove equalities between terms of *different but propositionally equal* types — what is called *heterogeneous equality* (written `a ≡_p b` where `p : A ≡ B`, `a : A`, `b : B`). In cubical type theory, this is the *PathP* type: `PathP (λ i → P i) a b` is a path in a varying family `P : I → Type`, and it generalizes both ordinary paths and heterogeneous equalities. This exercise explores PathP as a tool for verified generalized rewriting.

**Questions:**
1. The `PathP` type in Cubical Agda has the formation rule:
   ```agda
   PathP : (A : I → Type) → A i0 → A i1 → Type
   ```
   An ordinary path `a ≡ b : A` is `PathP (λ _ → A) a b`. A heterogeneous path `a ≡_p b` (where `p : A ≡ B`) is `PathP p a b`. Write the types of the following functions and implement them:
   ```agda
   -- Ordinary path to PathP
   toPathP : {A : I → Type} → A i0 ≡ transport (λ i → A i) ... → PathP A ... ...
   
   -- PathP to ordinary path (by transporting one endpoint)
   fromPathP : {A : I → Type} {a : A i0} {b : A i1}
               → PathP A a b → transport (λ i → A i) a ≡ b
   ```

2. Consider the following scenario in verified programming: you have two list types `ListA` and `ListB` for lists of `A` and `B` respectively, and a proof that `A ≡ B` (say, via `ua` of some equivalence). Define `subst-list : A ≡ B → List A → List B` using `transport`, and show it is definitionally equal to the result of mapping the underlying equivalence over the list. Specifically:
   ```agda
   substMap : {A B : Type} (p : A ≡ B) (l : List A)
              → transport (cong List p) l ≡ map (transport p) l
   ```
   Prove this by induction on `l`. In the inductive step, you will need `PathP` to handle the case where the head of the list has been transported.

3. *Design exercise:* In some dependently typed languages, the *rewrite* rule (the K axiom) allows you to replace any equality proof `p : a = a` with `refl`. In cubical type theory, this is not available. Instead, you must work with `PathP` and `transport`. Identify a programming pattern where K would be convenient but where the cubical alternative (using `PathP` or `transp`) is equally clean. Design a small Agda/Cubical DSL for a specific domain (e.g., verified matrix operations where dimensions must match) and show how `PathP` replaces K in the key lemmas.

*Abstract concept illustrated: `PathP` as the cubical generalization of heterogeneous equality; the relationship between K, UIP, and the cubical alternative.*

---

## Exercise A.4: Canonicity as a Correctness Guarantee for Verified Computation
*Domain: Software Verification / Foundations of Programming Languages*

**Setup:** The canonicity theorem for CCHM says: every closed term of type `ℕ` reduces to a numeral. This is a foundational guarantee with a direct practical interpretation — any program of type `ℕ` that type-checks in CCHM actually computes a number. No type-checked program can get "stuck" on an unresolved axiom. This exercise explores the canonicity theorem as a correctness argument for verified computation.

**Questions:**
1. In classical MLTT with the univalence axiom (axiomatic HoTT), canonicity fails. Exhibit a term of type `ℕ` that is closed (no free variables) but *stuck* — it cannot reduce to a numeral. The canonical example is:
   ```
   stuck : ℕ
   stuck = transport (ua succ-equiv) 3
   ```
   where `ua : ℕ ≃ ℕ → ℕ ≡ ℕ` is the univalence axiom and `succ-equiv` is the successor equivalence. Explain why this is stuck (no reduction rule for `transport (ua ...)`) and what the "expected" value is (`4`).

2. In CCHM, the same term *does* reduce: `transport (ua succEquiv) 3` reduces to `4` by the Glue computation rule. Now consider a more complex example: the recursive function
   ```agda
   f : ℕ → ℕ
   f n = transport (ua succEquiv) n
   ```
   What does `f 0` reduce to? `f (f 0)`? `f (f (f 0))`? Trace the reduction for each. Then define `g : ℕ → ℕ` as `g n = transport (ua pred-equiv) n` where `pred-equiv` is the predecessor equivalence (sending `0` to `0` and `n+1` to `n`). What does `f (g 3)` reduce to? Is this equal to `3`?

3. The canonicity theorem guarantees that for any *closed* term `t : ℕ` in CCHM, there is a numeral `n` such that `t ≡ n` definitionally. Why is the "closed" qualifier essential? Give an example of an *open* term of type `ℕ` (one with a free variable) that is not definitionally equal to any numeral. Now explain why canonicity is the right formulation for a programming language foundation: what would it mean for a programming language's type system to guarantee that "every well-typed program terminates with a value"? How does canonicity formalize this for the type `ℕ`?

*Abstract concept illustrated: Canonicity as the computational correctness property of CCHM; the contrast with axiomatic HoTT; `transp` reduction as the operational semantics of programs.*

---

## Exercise A.5: De Morgan Algebra, the Interval, and Distributed Systems
*Domain: Concurrency Theory / Distributed Systems*

**Setup:** The De Morgan algebra structure on the interval in CCHM — the operations `~`, `∧`, `∨` on the interval $\mathbb{I}$ satisfying $\sim(\sim r) = r$, $r \wedge \sim r = 0$, $r \vee \sim r = 1$, and the De Morgan laws — is not merely an artifact of the type-theoretic construction. It appears naturally as a model of *time* in distributed systems: the interval $[0, 1]$ represents a time window, meets represent simultaneous occurrence, joins represent "at some point," and the complement represents "the remaining time." This exercise explores this connection.

**Questions:**
1. Model a simple distributed protocol in which two processes `P` and `Q` must agree on a value within a time window $[0, 1]$. A "message delivery guarantee" can be modeled as a function `m : I → Bool` where `m i0 = false` (message not yet received) and `m i1 = true` (message received). A path `m : I → Bool` is a "delivery event." 
   
   Using the De Morgan algebra, define:
   - `before r s = r ∧ s` (both events occur by time `r ∧ s`)
   - `after r s = r ∨ s` (at least one event occurs by `r ∨ s`)
   - `complement r = ~ r` (the remaining time window after `r`)
   
   State (but you need not prove): if `P` and `Q` each deliver a message by time `r` and `s` respectively, then `P` delivers before `~ s` iff `r ≤ ~ s` iff `r ∧ s = r`. What does this say about the "happens-before" relation?

2. In concurrent programming, two computations are *independent* if their intervals of execution don't overlap. Model independence as: computations `f : I → A` and `g : I → B` are independent if there exists `r : I` such that `f` is constant on `[r, 1]` and `g` is constant on `[0, r]`. Formalize this in Cubical Agda using the interval operations:
   ```agda
   independent : {A B : Type} → (I → A) → (I → B) → Type
   independent f g = Σ[ r ∈ I ] ((i : I) → r ∧ i ≡ r → f i ≡ f i1)
                                × ((i : I) → r ∨ i ≡ r → g i ≡ g i0)
   ```
   Is this the right definition? What modifications would make it more faithful to the "no overlap" intuition?

3. In the theory of *concurrent games* and *event structures*, a "cube" of dimension $n$ represents $n$ independent events that can occur in any order. The De Morgan algebra on $\mathbb{I}^n$ (the $n$-cube) makes every face of the cube accessible as a projection $r \wedge 0$ or $r \vee 1$. This is precisely the face formula system in CCHM. Write a brief (1-2 paragraph) explanation of why the face formula system in CCHM — the partial elements $[i=0 \mapsto a, j=1 \mapsto b]$ — can be read as specifying "the value at this boundary condition of the concurrent execution." What is the `hcomp` operation in this reading? (Informally: `hcomp` takes a partial execution with known boundary behavior and completes it to a full execution.)

*Abstract concept illustrated: The De Morgan algebra as a model of concurrent time; face formulas as boundary conditions on concurrent executions; `hcomp` as a completion operation.*
