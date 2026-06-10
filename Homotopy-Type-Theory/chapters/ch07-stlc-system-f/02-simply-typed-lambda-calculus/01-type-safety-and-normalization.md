# 2.1 Simply Typed Lambda Calculus: Type Safety and Normalization

## The Type Discipline

The simply typed lambda calculus (STLC) adds type annotations to the untyped lambda calculus, ruling out the problematic terms while preserving the well-behaved ones.

We've already seen the syntax and typing rules in Chapter 6 (Section 2). Here we focus on the key *theorems* about STLC: type safety (progress + preservation) and strong normalization. These theorems show that the type discipline "works" — it prevents exactly the bad behavior we want to prevent.

## Type Safety: Progress and Preservation

Type safety says well-typed programs don't "go wrong." It's stated as two lemmas.

**Values.** A term is a *value* if it is in final (irreducible) form:
- $\lambda x. t$ (lambda abstractions are values — functions are "done")
- $(v_1, v_2)$ where $v_1, v_2$ are values (pairs of values)
- $\mathsf{inl}(v)$ and $\mathsf{inr}(v)$ where $v$ is a value
- $\star$ (the unit element)

Values represent "fully computed" results.

**Progress.** A well-typed closed term is either a value or can take a step.

**Theorem (Progress).** If $\vdash t : A$ (closed, well-typed), then either $t$ is a value, or there exists $t'$ with $t \to_\beta t'$.

*Proof.* By induction on the typing derivation.

- **Variable:** No closed variable (all variables are bound by some abstraction).
- **Lambda abstraction:** $\lambda x. t$ is a value.
- **Application $f\, a$:** Either $f$ is a value (hence a lambda abstraction $\lambda x. t$, so $(\lambda x. t)\, a \to t[a/x]$) or $f$ can take a step ($f \to f'$, so $f\, a \to f'\, a$).
- **Pair $(t, s)$:** Either $t$ can take a step ($t \to t'$, $(t, s) \to (t', s)$) or $t$ is a value and $s$ can take a step, or both are values (making $(t, s)$ a value).
- **And so on for all other cases.** $\square$

**Preservation (Subject Reduction).** Types are preserved by reduction.

**Theorem (Preservation).** If $\Gamma \vdash t : A$ and $t \to_\beta t'$, then $\Gamma \vdash t' : A$.

*Proof.* By induction on the reduction step. The key case is $\beta$-reduction:

**Case $(\lambda x : A. t)\, s \to t[s/x]$:** We have $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$. By the **Substitution Lemma** (see below), $\Gamma \vdash t[s/x] : B$. $\square$

**Substitution Lemma.** If $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$, then $\Gamma \vdash t[s/x] : B$.

*Proof.* By induction on the typing derivation for $t$. The interesting case is when $t$ is a variable: if $t = x$, then $t[s/x] = s$, which has type $A = B$ by hypothesis. If $t = y \neq x$, then $y : B \in \Gamma$ and $t[s/x] = y$, which still has type $B$. For compound terms, the substitution distributes over each constructor and the induction hypothesis handles the subterms. $\square$

The Progress + Preservation theorems together give type safety: starting from a well-typed closed term, every sequence of reduction steps either terminates at a value or keeps making progress. The term can't get "stuck" in a state that's not a value but also can't take a step.

## Decidability of Type Checking

**Theorem.** The problem "given $\Gamma$, $t$, $A$: is $\Gamma \vdash t : A$?" is decidable for STLC.

Moreover, type *inference* is also decidable: given $\Gamma$ and $t$, there is at most one type $A$ with $\Gamma \vdash t : A$ (principal type theorem), and this type is computable.

The algorithm: structurally recurse on $t$. For variables, look up in $\Gamma$. For abstractions $\lambda x : A. t$, recursively infer the type of $t$ in the extended context $\Gamma, x : A$. For applications $f\, a$, infer the type of $f$ (must be $A \to B$), infer the type of $a$ (must match $A$), return $B$.

This is why type checking in a proof assistant is purely mechanical — no heuristics, no guessing.

## Strong Normalization: The Reducibility Method

We've already proved strong normalization using the reducibility method in Chapter 6. Let's revisit the key ideas with more detail.

The fundamental challenge: why can't a well-typed term loop forever?

**Intuition:** In STLC, every beta reduction step either:
1. Reduces the *type complexity* of the term (the type gets "smaller"), or
2. If not, the term gets "more reduced" in some other measurable sense.

The reducibility method makes this precise by defining, for each type $A$, a set $\text{Red}(A)$ of "good" terms — terms that not only terminate but have a specific property reflecting their type.

**The reducibility predicates:**

For base type $o$: $\text{Red}(o) = \{t : o \mid t \text{ is strongly normalizing}\}$.

For function type: $\text{Red}(A \to B) = \{t : A \to B \mid \forall s \in \text{Red}(A): t\, s \in \text{Red}(B)\}$.

For product type: $\text{Red}(A \times B) = \{t : A \times B \mid \text{fst}(t) \in \text{Red}(A) \text{ and } \text{snd}(t) \in \text{Red}(B)\}$.

For sum type: $\text{Red}(A + B) = \{t : A + B \mid \forall C, \forall s$ and $u$ compatible: $\text{case}(t, x.s, y.u) \in \text{Red}(C)\}$.

**Key lemma:** Every element of $\text{Red}(A)$ is strongly normalizing (CR1), reducibility is closed under reduction (CR2), and neutral terms with all reducts reducible are themselves reducible (CR3).

**Main theorem:** Every well-typed term is reducible (proved by induction on the typing derivation).

**Conclusion:** Every well-typed term is strongly normalizing.

## What STLC Cannot Do

STLC is strongly normalizing — every program terminates. This means it's *not* Turing-complete.

**Theorem.** The Ackermann function is not definable in STLC.

*Proof sketch.* The Ackermann function grows faster than any primitive recursive function. Every STLC-definable function on Church numerals is primitive recursive. $\square$

More generally:
- Self-referential types ($A = A \to B$) are impossible (no finite type satisfies this).
- The Y combinator can't be typed (it requires self-application).
- No general recursion — only structural recursion on specific data types (which must be built in).

**Why this is a feature, not a bug:** For a proof assistant, non-termination would mean "proofs" that run forever — a form of inconsistency. The strong normalization of STLC means all proofs terminate, which guarantees consistency.

The trade-off: to get more expressiveness (handle more recursive functions), we need more powerful type systems. System F handles all primitive recursive functions. MLTT with inductive types handles all provably total functions. But full Turing-completeness requires relaxing strong normalization.

## The Role of Type Annotations

In STLC, lambda abstractions are annotated: $\lambda x : A. t$. This makes type checking decidable without any unification.

In practice, type inference for STLC (the Hindley-Milner algorithm) can infer types without annotations, using unification. This is the algorithm underlying OCaml and Haskell's type inference. But for dependent types, full type inference is undecidable, and annotations become necessary.

## Connection to Proof Assistants

The type-safety theorems translate directly to proof assistant guarantees:
- **Progress:** The proof checker never gets "stuck" on a well-typed proof term.
- **Preservation:** Proof checking is compositional — checking the whole proof reduces to checking the parts.
- **Decidability:** The proof checker terminates on any input.
- **Strong normalization:** Proof evaluation (running the proof) terminates, giving a canonical "verified" output.

These are not just theoretical guarantees — they're the engineering foundations of Lean 4, Agda, and Coq.
