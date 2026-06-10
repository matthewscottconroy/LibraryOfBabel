# Predicate Logic

## Why Propositional Logic Is Not Enough

Consider the argument: "All humans are mortal. Socrates is human. Therefore Socrates is mortal." This is obviously valid. Yet propositional logic cannot capture it. The argument essentially involves a predicate — "is mortal," "is human" — applied to an individual "Socrates," and a quantifier "all" ranging over the domain of individuals. None of these are expressible in propositional logic, which can only handle atomic, unanalyzed propositions.

Predicate logic (also called *first-order logic*, or FOL) is the extension of propositional logic that adds predicates, individual constants, function symbols, and — most importantly — quantifiers. With these tools, we can express almost all of ordinary mathematics: "every prime greater than 2 is odd," "there exists an irrational number," "for all ε > 0, there exists δ > 0 such that..."

The quantifiers ∀ and ∃ are where mathematics lives. And in dependent type theory, they become the dependent types Π and Σ. The universal quantifier ∀x:A. P(x) becomes the dependent product type, whose terms are functions mapping each a:A to a proof of P(a). The existential quantifier ∃x:A. P(x) becomes the dependent sum type, whose terms are pairs (a, p) where a:A and p is a proof of P(a).

This correspondence — quantifiers as dependent types — is the deepest part of the Curry-Howard correspondence, and it is the hinge on which Homotopy Type Theory turns. We lay its foundation here.

## Syntax: Signatures and Terms

A predicate logic language begins with a *signature* specifying the non-logical symbols:
- *Constant symbols*: c, d, 0, 1, ... (representing specific individuals)
- *Function symbols* with arities: f¹, g², ... (representing operations)
- *Predicate symbols* with arities: P¹, R², ... (representing relations)

**Terms** are built inductively from variables and constants using function symbols:
- Every variable x is a term.
- Every constant c is a term.
- If t₁, ..., tₙ are terms and f is an n-ary function symbol, then f(t₁, ..., tₙ) is a term.

Examples with the signature of arithmetic (constants 0, 1; function symbols +, ·, S; predicate <):
- Terms: 0, 1, x, S(x), x + y, S(x) · (y + 1)
- Not terms: 0 < 1 (this is a formula, not a term), x + (not a complete term)

Terms denote *individuals* in the domain of discourse. Predicates applied to terms yield *atomic formulas*.

## Syntax: Formulas

**Atomic formulas** are built from predicates applied to terms:
- If t₁, ..., tₙ are terms and R is an n-ary predicate, then R(t₁, ..., tₙ) is an atomic formula.
- The equality symbol t₁ = t₂ is also an atomic formula.

**Well-formed formulas** are built inductively:
- Every atomic formula is a wff.
- If φ is a wff, so is ¬φ.
- If φ and ψ are wffs, so are (φ ∧ ψ), (φ ∨ ψ), (φ → ψ), (φ ↔ ψ).
- If φ is a wff and x is a variable, then ∀x. φ and ∃x. φ are wffs.

The universal and existential quantifiers bind variables: ∀x says "for all x in the domain" and ∃x says "there exists x in the domain." The formula φ is the *scope* of the quantifier.

## Free and Bound Variables

This is a critical distinction. An occurrence of a variable x in a formula is *bound* if it lies within the scope of a quantifier ∀x or ∃x. Otherwise it is *free*.

**Examples:**
- In ∀x. P(x), the occurrence of x in P(x) is bound.
- In P(x) → ∀y. Q(x, y), the x in P(x) is free, while x in Q(x, y) is bound by neither quantifier... wait, that depends on whether x is captured. Let us be precise.
- In ∀x. (P(x) → ∃y. R(x, y)), all occurrences of x and y are bound.
- In ∃y. R(x, y), the occurrence of y is bound, but x is free.

**Definition.** The set FV(φ) of *free variables* of a formula φ is defined recursively:
- FV(R(t₁,...,tₙ)) = all variables occurring in t₁,...,tₙ
- FV(¬φ) = FV(φ)
- FV(φ ★ ψ) = FV(φ) ∪ FV(ψ) for binary connectives
- FV(∀x. φ) = FV(φ) \ {x}
- FV(∃x. φ) = FV(φ) \ {x}

A *sentence* is a formula with no free variables: FV(φ) = ∅. Sentences are the formulas that can be assigned a definite truth value (relative to a structure). Formulas with free variables are like predicates — they say something about the free variables, but have no truth value until those variables are specified.

**Bound variable convention.** We treat formulas that differ only by renaming bound variables as identical. So ∀x. P(x) and ∀y. P(y) are the same formula. This is the *α-equivalence* of the lambda calculus: functions that differ only in the name of their argument are the same function.

## Substitution

*Substitution* is the operation of replacing free occurrences of a variable with a term. We write φ[t/x] for "φ with t substituted for x."

**Definition by recursion:**
- P(...)[t/x] = replace all free occurrences of x in P(...) with t
- (¬φ)[t/x] = ¬(φ[t/x])
- (φ ★ ψ)[t/x] = (φ[t/x]) ★ (ψ[t/x])
- (∀y. φ)[t/x] = ∀y. (φ[t/x]) if y ≠ x and y ∉ FV(t)
- (∃y. φ)[t/x] = ∃y. (φ[t/x]) if y ≠ x and y ∉ FV(t)

The condition y ∉ FV(t) prevents *variable capture*: substituting a term with a free variable into the scope of a binder that would accidentally bind that variable. Example: the substitution (∀y. y < x)[y/x] should give a formula saying y is a lower bound for y, but if we naively substitute we get ∀y. y < y, which says y is less than itself — completely wrong. The fix is to rename the bound variable first: (∀z. z < x)[y/x] = ∀z. z < y. This renaming is always possible.

Variable capture and substitution are foundational topics in programming language theory. In the lambda calculus, β-reduction (function application) involves substitution, and capture-avoiding substitution is the central technical subtlety. Type-theoretic proof systems are defined with explicit substitution operations, and getting them right is the source of much of the formal complexity.

## Semantics: Structures and Satisfaction

**Definition.** A *structure* M (also called an *interpretation* or *model*) for a signature consists of:
- A non-empty set |M|, the *domain* (or *universe*)
- For each constant c: an element cᴹ ∈ |M|
- For each n-ary function symbol f: a function fᴹ: |M|ⁿ → |M|
- For each n-ary predicate R: a relation Rᴹ ⊆ |M|ⁿ

A *variable assignment* is a function σ: Vars → |M|.

The *denotation* of a term t under M and σ is defined recursively:
- ⟦x⟧ₘ,σ = σ(x)
- ⟦c⟧ₘ,σ = cᴹ
- ⟦f(t₁,...,tₙ)⟧ₘ,σ = fᴹ(⟦t₁⟧ₘ,σ, ..., ⟦tₙ⟧ₘ,σ)

*Satisfaction*, written M,σ ⊨ φ (read: "M with assignment σ satisfies φ"), is defined recursively:
- M,σ ⊨ R(t₁,...,tₙ) iff (⟦t₁⟧ₘ,σ, ..., ⟦tₙ⟧ₘ,σ) ∈ Rᴹ
- M,σ ⊨ t₁ = t₂ iff ⟦t₁⟧ₘ,σ = ⟦t₂⟧ₘ,σ
- M,σ ⊨ ¬φ iff M,σ ⊭ φ
- M,σ ⊨ φ ∧ ψ iff M,σ ⊨ φ and M,σ ⊨ ψ
- M,σ ⊨ ∀x. φ iff M,σ[x↦a] ⊨ φ for every a ∈ |M|
- M,σ ⊨ ∃x. φ iff M,σ[x↦a] ⊨ φ for some a ∈ |M|

Here σ[x↦a] is the assignment identical to σ except that it maps x to a.

For sentences (FV(φ) = ∅), satisfaction is independent of σ, so we write M ⊨ φ.

**Logical consequence.** A sentence φ is a *logical consequence* of a set Γ of sentences, written Γ ⊨ φ, if M ⊨ φ whenever M ⊨ ψ for every ψ ∈ Γ.

## Proof Rules for Quantifiers

Natural deduction adds rules for the quantifiers to the propositional rules.

**Universal Quantifier:**

Introduction (∀I): To prove ∀x. φ, assume x is an arbitrary element (not appearing free in any hypothesis) and prove φ.

Elimination (∀E): From ∀x. φ, derive φ[t/x] for any term t.

**Existential Quantifier:**

Introduction (∃I): From φ[t/x], derive ∃x. φ.

Elimination (∃E): From ∃x. φ and a proof that "given any a with φ[a/x], we can derive ψ" (where a is not free in ψ or in any hypothesis), derive ψ.

These rules are precise and complete (the completeness theorem, which we prove in the next section, guarantees this). They also have direct type-theoretic interpretations:

- ∀I corresponds to lambda-abstraction: λ(x:A). (proof of P(x)).
- ∀E corresponds to function application: applying a proof of ∀x:A.P(x) to a term a:A.
- ∃I corresponds to pairing: (a, proof of P(a)).
- ∃E corresponds to the fst/snd projections from a dependent pair.

## Important Logical Facts

**Duality of quantifiers.** ¬∀x. φ ≡ ∃x. ¬φ and ¬∃x. φ ≡ ∀x. ¬φ. These are the first-order analogues of De Morgan's laws.

**Moving quantifiers.** If x is not free in ψ:
- ∀x. (φ ∧ ψ) ≡ (∀x. φ) ∧ ψ
- ∀x. (φ ∨ ψ) ≡ (∀x. φ) ∨ ψ
- ∃x. (φ ∧ ψ) ≡ (∃x. φ) ∧ ψ
- ∃x. (φ ∨ ψ) ≡ (∃x. φ) ∨ ψ

**Prenex normal form.** Any formula is logically equivalent to a *prenex* formula: one where all quantifiers are at the front. We can push quantifiers outward using the above equivalences.

**Uniqueness quantifier.** The formula ∃!x. φ (read "there exists a unique x such that φ") is defined as ∃x. (φ ∧ ∀y. (φ[y/x] → y = x)). This says there is some x satisfying φ, and any y satisfying φ must equal x. Uniqueness quantifiers appear constantly in mathematics: "there exists a unique identity element," "there exists a unique solution."

## Predicate Logic and Mathematics

Every mathematical field lives in predicate logic. Group theory is the theory of structures (G, ·, e, ⁻¹) satisfying:
- ∀x,y,z. (x · y) · z = x · (y · z) (associativity)
- ∀x. x · e = e · x = x (identity)
- ∀x. x · x⁻¹ = x⁻¹ · x = e (inverses)

Set theory (ZFC) is a first-order theory in the language of membership: a single binary predicate ∈. Every mathematical object is a set, and every mathematical fact is a sentence in the language of ∈.

Analysis says: a sequence (aₙ) converges to L iff ∀ε>0. ∃N. ∀n≥N. |aₙ - L| < ε. This is a ∀∃∀ sentence — and the alternation of quantifiers is exactly what makes analysis hard: ∀∃ (for every challenge, there is a response) is manageable, but ∃∀ (there is a single response to every challenge) is stronger.

Understanding predicate logic — its syntax, its semantics, its proof rules — means understanding the logical infrastructure of all of mathematics. When we formalize any field of math in a proof assistant, we are always working in predicate logic (or its dependent type-theoretic extension). The variable-binding, substitution, and quantifier-elimination rules in this section are the operations the proof assistant performs behind the scenes every time you apply a lemma, introduce a variable, or state a universally quantified theorem.

That is why predicate logic is not just background. It is the grammar of mathematical truth.
