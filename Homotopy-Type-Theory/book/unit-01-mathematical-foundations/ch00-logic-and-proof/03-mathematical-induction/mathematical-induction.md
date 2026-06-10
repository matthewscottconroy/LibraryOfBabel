# Mathematical Induction

## The Principle That Runs Through Everything

There is one proof technique so central to mathematics and computer science that it deserves a section of its own: mathematical induction. Not because it is exotic, but because it is ubiquitous — and because the different forms it takes illuminate a deep fact about how mathematical structures are organized.

In type theory, induction is not a proof technique. It is the *definition* of a type. The natural numbers are not defined and then given an induction principle — they *are* their induction principle. The induction principle for ℕ is the same object as the *recursor* for the natural number type. When you write a recursive function in Lean, you are doing induction. When you write a proof by induction, you are defining a term by recursion.

We will not develop this identification fully here — that comes in later chapters. But we will develop induction in increasing generality, so that the type-theoretic versions are natural when we reach them.

## Simple Induction

**The principle.** To prove a property P(n) for all natural numbers n ≥ 0, it suffices to:
1. Prove P(0) (the *base case*).
2. Prove that for every k ≥ 0, if P(k) holds then P(k+1) holds (the *inductive step*).

The *inductive hypothesis* is the assumption P(k) in the inductive step. It is not circular — you are not assuming what you want to prove. You are assuming P at a smaller value (k) and deriving P at a larger value (k+1).

Why does this work? The natural numbers are well-ordered: every non-empty set of natural numbers has a least element. Suppose P fails for some natural number. Then the set of natural numbers where P fails is non-empty, so it has a least element m. By the base case, m ≠ 0, so m = k+1 for some k. By minimality of m, P(k) holds. By the inductive step, P(k+1) holds. Contradiction.

**Example.** Prove: 1 + 2 + 3 + ... + n = n(n+1)/2 for all n ≥ 1.

*Base case.* n = 1: the sum is 1, and 1(1+1)/2 = 1. ✓

*Inductive step.* Assume 1 + 2 + ... + k = k(k+1)/2. We must show 1 + 2 + ... + k + (k+1) = (k+1)(k+2)/2.

Starting from the left: 1 + 2 + ... + k + (k+1) = [1 + 2 + ... + k] + (k+1) = k(k+1)/2 + (k+1) = (k+1)[k/2 + 1] = (k+1)(k+2)/2. □

**Example.** Prove: 2ⁿ > n for all n ≥ 0.

*Base case.* n = 0: 2⁰ = 1 > 0. ✓

*Inductive step.* Assume 2ᵏ > k. We show 2ᵏ⁺¹ > k+1. We have 2ᵏ⁺¹ = 2 · 2ᵏ > 2k (by the inductive hypothesis) ≥ k + 1 (since k ≥ 0 implies k ≥ 1 for k ≥ 1, and we verify k = 0 directly). □

## Strong Induction

In simple induction, the inductive step uses only P(k) to derive P(k+1). Sometimes we need all of P(0), P(1), ..., P(k) to derive P(k+1). This is *strong induction* (also called *complete induction*).

**The principle.** To prove P(n) for all n ≥ 0, it suffices to prove: for every k, if P(j) holds for all j < k, then P(k) holds.

This single step encompasses both the base case (when k = 0, the hypothesis "P(j) for all j < 0" is vacuously true, so you must prove P(0) from nothing) and the inductive step.

**Example.** Prove: every integer n ≥ 2 has a prime factorization.

*Proof by strong induction.* We prove P(n): "n has a prime factorization," for all n ≥ 2.

Assume P(j) holds for all 2 ≤ j < n. If n is prime, it is its own prime factorization. If n is not prime, then n = ab for some 2 ≤ a, b < n. By the inductive hypothesis, both a and b have prime factorizations. Combining them gives a prime factorization of n. □

This proof requires the full strength of the inductive hypothesis, not just the single step P(k) → P(k+1). The factors a and b can be anywhere in the range [2, n-1), not just at n-1.

**Example.** Prove: the Fibonacci sequence Fₙ defined by F₁ = F₂ = 1, Fₙ = Fₙ₋₁ + Fₙ₋₂, satisfies Fₙ ≤ 2ⁿ for all n ≥ 1.

*Proof.* Base cases: F₁ = 1 ≤ 2¹ = 2. F₂ = 1 ≤ 2² = 4. For n ≥ 3, assuming Fₖ ≤ 2ᵏ for all k < n: Fₙ = Fₙ₋₁ + Fₙ₋₂ ≤ 2ⁿ⁻¹ + 2ⁿ⁻² = 2ⁿ⁻²(2 + 1) = 3 · 2ⁿ⁻² ≤ 2ⁿ. □

## Structural Induction

Simple and strong induction work for the natural numbers. But inductive definitions produce many other data structures: lists, trees, formulas, types. We want to reason about all formulas, or all binary trees, or all syntax trees of a programming language. The right tool is *structural induction*.

**The principle.** Let S be a set defined by an inductive definition (base cases + construction rules). To prove a property P holds for all elements of S:
1. Prove P holds for all base cases.
2. For each construction rule, prove that if P holds for all sub-structures, then P holds for the structure built by the rule.

**Example: formulas of propositional logic.** We have:
- Base case: atoms P, Q, R, ...
- Construction rules: ¬φ, (φ ∧ ψ), (φ ∨ ψ), (φ → ψ), (φ ↔ ψ)

**Theorem.** Every formula of propositional logic has an equal number of left and right parentheses.

*Proof by structural induction on formulas.*

Base case: atoms contain no parentheses. 0 = 0. ✓

Inductive step for ¬φ: by hypothesis, φ has n left and n right parentheses. ¬φ adds no parentheses. So ¬φ has n = n. ✓

Inductive step for (φ ★ ψ): by hypothesis, φ has j left and j right, ψ has k left and k right. The formula (φ ★ ψ) adds one left and one right parenthesis: total j+k+1 left and j+k+1 right. ✓ □

Structural induction is the workhorse of programming language theory. Every property of a programming language's syntax — type safety, termination, semantics — is proved by structural induction on the syntax tree. In Lean, when you do induction on a term, you are performing structural induction on the inductive type that defines the term.

**Example: binary trees.** Define binary trees inductively: either a leaf (base case), or a node with two subtrees (inductive case). Let leaves(T) and nodes(T) be the number of leaves and internal nodes.

**Theorem.** For any binary tree T, leaves(T) = nodes(T) + 1.

*Proof by structural induction.*

Base case: T = leaf. leaves(leaf) = 1, nodes(leaf) = 0. 1 = 0 + 1. ✓

Inductive step: T = node(L, R). By hypothesis: leaves(L) = nodes(L) + 1, leaves(R) = nodes(R) + 1. Then leaves(T) = leaves(L) + leaves(R) = [nodes(L) + 1] + [nodes(R) + 1] = [nodes(L) + nodes(R) + 1] + 1 = nodes(T) + 1. ✓ □

## Well-Founded Induction

Strong induction relies on the natural numbers being well-ordered: every non-empty subset has a least element. The most general form of induction abstracts this.

**Definition.** A *well-founded relation* on a set X is a relation < such that there is no infinitely descending chain x₀ > x₁ > x₂ > .... Equivalently, every non-empty subset has a minimal element (with respect to <).

**The principle of well-founded induction.** Let < be a well-founded relation on X. To prove P(x) for all x ∈ X, it suffices to prove: for every x, if P(y) holds for all y < x, then P(x) holds.

**Examples of well-founded relations:**
- The usual order on ℕ (simple induction is the special case X = ℕ)
- The subformula relation on formulas (structural induction is the special case X = Form)
- The lexicographic order on ℕ × ℕ: (a, b) < (c, d) iff a < c, or a = c and b < d
- The proper-subterm relation on terms of a programming language

**Example: the Euclidean algorithm terminates.** The Euclidean algorithm computes gcd(a, b) by repeated subtraction: gcd(a, b) = gcd(b, a mod b) when b ≠ 0. We claim it terminates.

*Proof.* Define the measure m(a, b) = b. Each recursive call goes from (a, b) to (b, a mod b). We have 0 ≤ a mod b < b. So the measure strictly decreases: m(b, a mod b) = a mod b < b = m(a, b). Since measures are non-negative integers and they strictly decrease, by well-founded induction (on ℕ with its usual order), the algorithm terminates. □

This "decreasing measure" argument is one of the most useful proof techniques in computer science. Every recursive program can be shown to terminate by exhibiting a well-founded measure that decreases with each recursive call.

## The Motive Concept

In Martin-Löf Type Theory, induction is not a proof rule — it is a *type-former eliminator*. The natural number type ℕ comes with an *induction principle* that says: given

- A *motive* P : ℕ → Type (what we want to prove, as a family of types indexed by ℕ)
- A base case: a term of type P(0)
- An inductive step: a term of type ∀(k : ℕ), P(k) → P(k+1)

there is a term of type ∀(n : ℕ), P(n).

The *motive* P is the predicate we are proving. Specifying P is sometimes called "choosing the induction motive" and in dependent type theory it must be specified explicitly, because the type system needs to know the family over which we are inducting.

For simple propositions like "2ⁿ > n," the motive is straightforward. For more complex situations — especially when the thing being inducted on appears in a type — the motive must be chosen carefully. This is one of the technically demanding aspects of formal proof in dependent type theory, and identifying it here as the "motive" concept prepares us for the formal treatment.

Every form of induction we have studied — simple, strong, structural, well-founded — is a special case of this general pattern. Simple induction: P is a predicate on ℕ, < is the usual order. Structural induction: P is a predicate on some inductively defined set, < is the subterm relation. Well-founded induction: P is a predicate on X, < is any well-founded relation.

In HoTT, the *identity type eliminators* — J and its variants — are a form of well-founded induction on paths. The ability to eliminate from a path type by "going by cases" on the path is the most powerful rule in the system, and it follows the same structural pattern as everything in this section.

Induction is the engine of mathematics. The natural numbers exist *because* there is something to induct over. Understanding induction deeply — its structure, its varieties, its connection to recursive definitions — is preparation for everything that follows.
