# 6.1 The Full Curry-Howard Correspondence

## What We Can Now Say

In Chapter 6, we introduced the Curry-Howard correspondence for propositional logic: propositions correspond to types, proofs to programs, logical connectives to type formers. We had products for conjunction, sums for disjunction, function types for implication.

But we were missing quantifiers. First-order logic has $\forall$ and $\exists$, and these are essential for real mathematics: "for all natural numbers $n$, Goldbach's conjecture holds for $n$"; "there exists a prime number larger than any given bound."

With Π and Σ types in hand, the correspondence is now complete. Let's lay out the full dictionary and then see what it means for mathematics and proof assistants.

## The Complete Dictionary

| **Logic** | **Type Theory** | **Proof/Program** |
|---|---|---|
| Proposition $P$ | Type $P : \mathsf{Type}$ | Type (with elements = proofs) |
| Proof of $P$ | Term $p : P$ | Program of type $P$ |
| True ($\top$) | Unit type $\mathbf{1}$ | $\mathsf{tt} : \mathbf{1}$ |
| False ($\bot$) | Empty type $\mathbf{0}$ | (no elements — no proof) |
| $P \land Q$ | Product type $P \times Q$ | Pair $(p, q)$ |
| $P \lor Q$ | Sum type $P + Q$ | $\mathsf{inl}(p)$ or $\mathsf{inr}(q)$ |
| $P \Rightarrow Q$ | Function type $P \to Q$ | Function $\lambda p. (\ldots)$ |
| $\neg P$ | $P \to \mathbf{0}$ | Function to empty type |
| $\forall x \in A, P(x)$ | Π type $\prod_{x:A} P(x)$ | Function $\lambda x. p(x)$ |
| $\exists x \in A, P(x)$ | Σ type $\sum_{x:A} P(x)$ | Pair $(a, p)$ with $p : P(a)$ |
| Proof irrelevance | h-proposition | At most one proof |
| Classical logic | Double negation | $\neg\neg P \to P$ as axiom |

This table is the Curry-Howard correspondence in its full form. Every row is an *exact* correspondence, not an analogy — the type formation rules are precisely the logical rules.

## The Key New Entries

### Universal Quantification as Π

The logical rule for $\forall x \in A, P(x)$:
- *Introduction:* To prove $\forall x, P(x)$, let $x$ be arbitrary and prove $P(x)$.
- *Elimination:* From $\forall x, P(x)$ and a specific $a$, derive $P(a)$.

This is exactly:
- $\Pi$-intro: $\lambda x. t : \prod_{x:A} P(x)$ where $t : P(x)$ in context with $x : A$
- $\Pi$-elim: $f\, a : P(a)$ from $f : \prod_{x:A} P(x)$ and $a : A$

### Existential Quantification as Σ

The logical rule for $\exists x \in A, P(x)$:
- *Introduction:* To prove $\exists x, P(x)$, exhibit a specific $a$ and prove $P(a)$.
- *Elimination:* From $\exists x, P(x)$ and a proof of $Q$ from any witness + evidence, derive $Q$.

This is exactly:
- $\Sigma$-intro: $(a, p) : \sum_{x:A} P(x)$ where $a : A$ and $p : P(a)$
- $\Sigma$-elim: using $\pi_1$ and $\pi_2$ (or pattern matching) to extract witness and evidence

## Proofs as Programs: What This Means

Under Curry-Howard, every proof is a program and every program is a proof. This has practical consequences.

**Running proofs:** A proof of "there exists an $n$ such that $n$ is prime and $n > 1000$" is a program that *computes* the witness. Running the proof (normalizing the term) gives you an explicit prime number $> 1000$.

**Composing proofs:** Just as programs can be composed (functions applied to arguments, outputs fed into inputs), proofs can be composed. The composition lemma $P \Rightarrow Q$, $Q \Rightarrow R$, therefore $P \Rightarrow R$ corresponds to function composition $(g \circ f)(x) = g(f(x))$.

**Proof transformations:** Logical equivalences (like $\neg(P \land Q) \Leftrightarrow \neg P \lor \neg Q$) correspond to computable isomorphisms between types — programs that convert proofs in one direction to proofs in the other.

**Proof search = program synthesis:** Finding a proof of a proposition corresponds to synthesizing a program of a given type. This is the connection between type theory and automated theorem proving.

## Example: Working Out an Existential Proof

Let's write the proof of "there exists an even prime number" as a type-theoretic term.

**Statement:** $\sum_{n:\mathbb{N}} \mathsf{IsPrime}(n) \times \mathsf{IsEven}(n)$

**Proof term:** $(2, (\text{proof that 2 is prime}, \text{proof that 2 is even}))$

This is literally a pair: the witness (2) and a product of two proofs. In Lean 4:

```lean
example : ∃ n : ℕ, Nat.Prime n ∧ Even n :=
  ⟨2, Nat.prime_two, ⟨1, rfl⟩⟩
-- ⟨2, ...⟩ is the witness and evidence pair
-- Nat.prime_two : Nat.Prime 2
-- ⟨1, rfl⟩ : Even 2 (since 2 = 2 * 1)
```

The proof term is a program. Running it would give you `2` as the witness, which is the prime even number.

## Universal Statements and Verification

A universally quantified statement $\prod_{x:A} P(x)$ is a function from elements of $A$ to proofs. When you instantiate it at a specific $a$, you get a specific proof of $P(a)$.

**Example:** Proof that every natural number is either zero or a successor:

$$f : \prod_{n:\mathbb{N}} (n = 0) + \sum_{m:\mathbb{N}} (n = \mathsf{succ}(m))$$

Defined by induction:
- $f(\mathsf{zero}) = \mathsf{inl}(\mathsf{refl})$
- $f(\mathsf{succ}(m)) = \mathsf{inr}(m, \mathsf{refl})$

This is simultaneously a proof (of a universal logical statement) and a program (a function on natural numbers). When you apply $f$ to a specific number, it tells you whether it's zero or a successor — and provides the relevant equality proof.

## The Role of the Identity Type

The identity type $\mathsf{Id}_A(a, b)$ (also written $a =_A b$ or just $a = b$) completes the picture. Its constructors:

$$\mathsf{refl} : a = a \quad \text{(reflexivity)}$$

Its elimination principle (J-rule):

$$J : \prod_{A:\mathsf{Type}} \prod_{a:A} \prod_{P : \prod_{b:A} (a = b) \to \mathsf{Type}} P(a, \mathsf{refl}) \to \prod_{b:A} \prod_{p:a=b} P(b, p)$$

The J-rule says: to prove $P(b, p)$ for any $b : A$ and any path $p : a = b$, it suffices to prove it for $b = a$ and $p = \mathsf{refl}$ (the reflexivity path). This is the induction principle for the identity type.

Under Curry-Howard, this corresponds to the principle of substitution of equals: if $a = b$ and $P(a)$ holds, then $P(b)$ holds. The J-rule makes this into a formal theorem about identity types, not an axiom.

In HoTT, $a = b$ has a geometric meaning: it's the type of *paths* from $a$ to $b$ in the space $A$. The J-rule says: path induction allows you to reduce any path to the trivial (reflexivity) path, up to homotopy. This is a topological statement — you can contract any path to a point — and it holds for paths in spaces (but not for all equalities in classical mathematics, hence the interest of HoTT).

## Propositions vs. Types: The Subtlety

The Curry-Howard correspondence identifies propositions with types. But there's a subtlety: in classical logic, a proposition is either true or false — it has at most two "values" (its truth values). In type theory, a type can have *many* inhabitants.

For pure logic, we want propositions to be *proof-irrelevant*: all proofs of the same proposition are equal. This corresponds to *mere propositions* or *h-propositions*:

$$\mathsf{isProp}(P) = \prod_{p\, q : P} p = q$$

A type $P$ is an h-proposition if any two of its elements are equal (in the identity type sense). The logical propositions — $\top$, $\bot$, $P \land Q$, $P \lor Q$, $P \Rightarrow Q$, $\forall x, P(x)$, $\exists x, P(x)$ — should all be h-propositions if we want proof-irrelevance.

**The problem:** Σ types are not always h-propositions. $\sum_{n:\mathbb{N}} \mathsf{IsPrime}(n)$ has many distinct elements (one for each prime), even though it's the "same" existential statement.

**The solution in HoTT:** The *propositional truncation* $\|P\|$ (or $\|P\|_{-1}$) is the type that forces $P$ to be an h-proposition — it has the same truth value as $P$ but forgets which proof was used. The existential $\exists x, P(x)$ in the logical sense is $\|\sum_{x:A} P(x)\|$ — you assert existence but hide the witness.

This distinction between $\sum_{x:A} P(x)$ (computational existential: gives you the witness) and $\|\sum_{x:A} P(x)\|$ (logical existential: asserts existence without the witness) is one of the first places where HoTT refines the naive Curry-Howard picture.

## The Full Correspondence Table (Extended)

| **Logic** | **Type Theory** | **HoTT Refinement** |
|---|---|---|
| True ($\top$) | $\mathbf{1}$ | $\mathbf{1}$ (contractible) |
| False ($\bot$) | $\mathbf{0}$ | $\mathbf{0}$ (empty) |
| $P \land Q$ | $P \times Q$ | $P \times Q$ |
| $P \lor Q$ | $\|P + Q\|$ (truncated) | Propositional truncation |
| $P \Rightarrow Q$ | $P \to Q$ | $P \to Q$ |
| $\neg P$ | $P \to \mathbf{0}$ | $P \to \mathbf{0}$ |
| $\forall x, P(x)$ | $\prod_{x:A} P(x)$ | $\prod_{x:A} P(x)$ |
| $\exists x, P(x)$ | $\|\sum_{x:A} P(x)\|$ (truncated) | With explicit witness: $\sum_{x:A} P(x)$ |
| Equality $a = b$ | $\mathsf{Id}_A(a, b)$ | Path space $a =_A b$ |
| Provable | Inhabited type | Non-empty type |

The "HoTT refinement" column shows the h-level-aware version of each connective, which behaves correctly as a proposition (proof-irrelevant) when that's what you want.

## Proofs Carry Computational Content

One of the most striking consequences of Curry-Howard: proofs carry computational content. This is not just a theoretical observation.

**Normalization theorem as proof extraction:** A proof of the normalization theorem for STLC (every term reduces to a normal form) is a program that, given a well-typed term, computes its normal form. The proof is the algorithm.

**The Cauchy completeness proof:** A proof that the real numbers are complete (every Cauchy sequence converges) contains, hidden inside it, an algorithm for extracting the limit of a Cauchy sequence. Running the proof extracts this algorithm.

**Consistency relative to a proof:** If you have a proof of $A \Rightarrow B$, you can apply it to any proof of $A$ to get a proof of $B$. This is program execution — applying a function to an argument.

This is what proof assistants like Lean 4, Agda, and Coq exploit: they're simultaneously proof checkers (verify logical correctness) and programming languages (run the programs). The extraction of verified algorithms from proofs is a direct consequence of Curry-Howard.

## Classical Logic as an Extension

Classical logic has $\neg\neg P \to P$ (double negation elimination) and $P \lor \neg P$ (law of excluded middle). Under Curry-Howard, these have type-theoretic translations:

$$\mathsf{dne} : \neg\neg P \to P$$

But in pure type theory, there's no program of this type — you can't compute a proof of $P$ from a proof that "assuming $P$ is false leads to a contradiction," without knowing which $P$ you're dealing with.

Classical axioms are *consistent* with dependent type theory (adding them doesn't introduce inconsistency), but they break the computational content. If you add LEM as an axiom:

$$\mathsf{lem} : P \lor \neg P$$

then you can use it in proofs, but the resulting proof terms may not compute to a canonical form — they're "stuck" at the `lem` axiom.

In Lean 4, classical axioms are available in the `Classical` namespace. Code in `Classical` context doesn't necessarily extract to running algorithms, but that's fine for pure proof verification.

## Summary: Dependent Types Complete the Correspondence

The Curry-Howard correspondence:

| Step | Added to logic | Added to type theory |
|---|---|---|
| Propositional | $\land, \lor, \Rightarrow, \bot, \top, \neg$ | Products, sums, functions, $\mathbf{0}$, $\mathbf{1}$ |
| Quantifiers | $\forall, \exists$ | Π types, Σ types |
| Equality | $a = b$ | Identity type $\mathsf{Id}_A(a,b)$ |
| Universes | Types of propositions | Universe hierarchy |

With all four rows, we have:
- A complete logic (propositional + predicate + equality)
- A complete programming language (all computable functions, modulo termination)
- A complete foundation for mathematics (comparable in strength to ZFC, if not stronger)

Everything in the rest of this book — HoTT, univalence, higher inductive types, synthetic homotopy theory — is built on this foundation. The type theory of Π types, Σ types, identity types, and universes is the bedrock.

The next chapter, Martin-Löf Type Theory, will formalize this foundation precisely: the four forms of judgment, the inference rules, the universe hierarchy, and the formal semantics. From there, we'll have everything we need to state the Univalence Axiom and begin doing homotopy theory inside type theory.
