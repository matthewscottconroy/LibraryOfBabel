# Π Types: Dependent Function Types

## From Arrows to Dependence

In the simply typed lambda calculus, a function type A → B means: give me an A, I return a B. The return type B is fixed. It does not change. The function `length : List A → ℕ` returns a natural number no matter which list you give it. The number itself can vary, but the *type* of what comes back never does.

Now imagine a function that does something different. It takes a natural number n and returns an element of Vec A n — a vector of exactly n elements. The return *type* changes with the input. When you apply the function to 0, you get something of type Vec A 0. When you apply it to 3, you get something of type Vec A 3. These are different types. The function crosses the boundary between terms and types in each application.

This is a Π type. Dependent function type. Dependent product. All names for the same thing: a function type where the codomain is a type family, not a fixed type.

## The Formation Rule

To form a Π type, we need a base type A and a type family over it:

$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma,\, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \prod_{x:A} B(x) : \mathsf{Type}} \qquad (\Pi\text{-Form})$$

The premises say: A is a type in context Γ, and in the extended context Γ, x : A, the expression B(x) is a type. The conclusion says: under those conditions, Π(x:A).B(x) is a type in context Γ.

Notice that x is bound in the Π type. The variable x is a formal parameter — it names the input. After forming the Π type, x no longer appears freely. The type Π(x:A).B(x) can be used in contexts that know nothing about x.

**The non-dependent special case.** If B does not depend on x — if B(x) = C for all x — then Π(x:A).B(x) = Π(x:A).C = A → C. The ordinary arrow type is a Π type where the family is constant. Every arrow type is a degenerate Π type. Π strictly subsumes →.

## The Introduction Rule: Lambda Abstraction

To construct an element of Π(x:A).B(x), we write a lambda abstraction:

$$\frac{\Gamma,\, x : A \vdash t(x) : B(x)}{\Gamma \vdash \lambda x.\, t(x) : \prod_{x:A} B(x)} \qquad (\Pi\text{-Intro})$$

Given a term t(x) of type B(x) in the context with x : A, we can abstract over x to form λx. t(x) of type Π(x:A).B(x). This is lambda abstraction — the same syntactic form as in STLC — but with dependent types, the body t(x) may use x in its type as well as its value.

**Example: the identity function at each type.** Define:

$$\mathsf{id} : \prod_{A:\mathsf{Type}} A \to A$$
$$\mathsf{id} = \lambda A.\, \lambda a.\, a$$

Here we have a Π type where the family maps each type A to the function type A → A. The function id takes a type A (as an explicit argument) and returns the identity function on A. This is the polymorphic identity — the System F term id that we defined before — but now the type quantification is a Π type where the first argument is a type from the universe.

**Example: the zeroes vector.** Define a function that takes n : ℕ and returns the zero vector of length n:

$$\mathsf{zeros} : \prod_{n:\mathbb{N}} \mathsf{Vec}\, \mathbb{N}\, n$$
$$\mathsf{zeros} = \lambda n.\, \underbrace{(0, 0, \ldots, 0)}_{n \text{ times}}$$

(Made precise by recursion on n.) The type says: for every n, you get an element of Vec ℕ n — a vector of exactly n zeros.

## The Elimination Rule: Application

To use an element of a Π type, we apply it:

$$\frac{\Gamma \vdash f : \prod_{x:A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B(a)} \qquad (\Pi\text{-Elim})$$

If f has a Π type and a : A, then f a has type B(a) — the type family evaluated at the specific argument a. The output type is not a generic "B" but B applied to the exact value a.

This precision is the point. When you write f 3 and f has type Π(n:ℕ).Vec A n, the type system knows you got Vec A 3 back — not Vec A n for some abstract n, but Vec A 3, the type of 3-element vectors. If you then try to concatenate this with a Vec A 5, the type checker computes 3 + 5 = 8 and expects Vec A 8 from the append function.

## The Computation Rule: β-Reduction

$$(\lambda x.\, t)\, a \equiv t[a/x] : B(a) \qquad (\Pi\text{-}\beta)$$

When you apply a lambda abstraction to an argument, you substitute the argument for the bound variable throughout the body. The result is definitionally equal to the original. The type checker accepts this substitution silently — it is not a proposition you prove, but a computation the checker performs.

**Example.** Let f = λn. zeros n where zeros : Π(n:ℕ).Vec ℕ n. Then f 5 ≡ zeros 5, which computes to (0, 0, 0, 0, 0) : Vec ℕ 5. The type checker verifies this without additional proof from you.

The η rule (sometimes called the uniqueness rule) says: any function f of Π type is definitionally equal to its η-expansion:

$$f \equiv \lambda x.\, f\, x \qquad (\Pi\text{-}\eta)$$

η-equality says that a function is determined by its values. This holds definitionally in extensional MLTT and propositionally (up to a proof) in intensional MLTT, depending on the specific system.

## Π Types Subsume Universal Quantification

In first-order logic, the universal quantifier ∀x ∈ A. P(x) says: for every element x of A, the predicate P holds at x. What is a proof of ∀x ∈ A. P(x)? It is a procedure that, given any x, produces a proof of P(x).

That is exactly a function. Specifically, it is an element of Π(x:A).P(x) — a function that takes x : A and returns an element of P(x) (the type of proofs that P holds at x).

$$\forall x \in A.\, P(x) \quad \longleftrightarrow \quad \prod_{x:A} P(x)$$

This is the Curry-Howard correspondence for universal quantification. The Π type is simultaneously a function type and a universally quantified proposition. We do not need two separate concepts — one for computation and one for logic. They are the same thing, seen from different perspectives.

**Example.** The statement "every natural number has a successor" is:

$$\prod_{n:\mathbb{N}} \mathbb{N}$$

An element of this type is a function ℕ → ℕ, e.g., the successor function λn. n+1. The proof that every number has a successor *is* the function that computes the successor. The proof carries its computational content.

## The Polymorphic Identity and Other Key Examples

**Polymorphic identity:**
$$\mathsf{id} : \prod_{A:\mathsf{Type}} \prod_{a:A} A$$
$$\mathsf{id} = \lambda A.\, \lambda a.\, a$$

The type says: for any type A, for any element a of A, we get back an A. This is the polymorphic identity function. In System F, this was ΛA. λa:A. a. Here it is just λA. λa. a — the type abstraction and term abstraction are both lambda abstractions, because types are terms in the universe.

**Dependent flip:**
$$\mathsf{flip} : \prod_{A\,B:\mathsf{Type}} \prod_{C:A\to B\to\mathsf{Type}} \left(\prod_{a:A}\prod_{b:B} C(a,b)\right) \to \prod_{b:B}\prod_{a:A} C(a,b)$$
$$\mathsf{flip} = \lambda A\, B\, C\, f\, b\, a.\, f\, a\, b$$

This is the dependent version of flipping the arguments of a function. Even in this fully general dependent setting, it is still just λA B C f b a. f a b. The simple lambda term works; the type does the heavy lifting.

**Function composition:**
$$({-}\circ{-}) : \prod_{A\,B\,C:\mathsf{Type}} (B \to C) \to (A \to B) \to (A \to C)$$
$$(g \circ f) = \lambda A\, B\, C\, g\, f\, a.\, g\, (f\, a)$$

In the non-dependent case. The fully dependent version, where C could depend on the output of f, is also expressible but more intricate.

## Π Types in Practice: Agda, Lean, Coq

In Agda, a Π type Π(x:A).B(x) is written `(x : A) → B x`. The lambda is `λ x → t`. Application is juxtaposition.

In Lean 4, the Π type is `(x : A) → B x` or `∀ (x : A), B x` (the two are definitionally equal in Lean). Lambda is `fun x => t`.

In Coq, the Π type is `forall (x : A), B x`. Lambda is `fun x => t`.

All three treat type-level abstraction and term-level abstraction uniformly. The universe is just another type, and quantifying over it is just another Π type.

## Why Π Types Matter for HoTT

The Π type is everywhere in HoTT. The statement of univalence is a Π type. Function extensionality is a Π type. Transport, path induction, the J rule's statement — all of these are Π types.

More fundamentally: in HoTT, equivalences between types are certain Π types (biinvertible maps or half-adjoint equivalences). The univalence axiom says that Equiv(A, B) — a Π type — is itself equivalent to the identity type A = B (where = lives in the universe). So Π types and identity types are deeply intertwined.

Every theorem in HoTT is an element of some Π type. Every proof is a function. This is not a metaphor. It is the formal structure of the theory.
