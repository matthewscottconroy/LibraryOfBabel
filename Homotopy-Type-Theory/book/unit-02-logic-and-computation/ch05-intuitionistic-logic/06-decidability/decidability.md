# Decidability

## When LEM Is Actually Provable

The Law of Excluded Middle — $P \vee \neg P$ — is not constructively valid in general. But for many specific propositions, it is constructively provable. This is the notion of *decidability*: a proposition $P$ is decidable if we can prove $P \vee \neg P$ — not as an axiom, but as a theorem with explicit computational content.

This is not a retreat to classical logic. It is a precise distinction: LEM as an axiom schema (for all $P$, without restriction) is non-constructive. But LEM for *specific* decidable propositions is a theorem we can prove by exhibiting a decision procedure.

The study of decidability — which propositions are decidable and which are not — is one of the central topics of constructive mathematics, and it connects directly to the h-level hierarchy in HoTT.

## Decidable Propositions: Definition

**Definition.** A proposition $P$ is *decidable* if $P \vee \neg P$ is provable (in whatever constructive system we are working in).

**Definition.** A predicate $P : A \to \text{Prop}$ is *decidable* if $\forall a : A, P(a) \vee \neg P(a)$ is provable.

**Definition.** A type $A$ has *decidable equality* if $\forall a, b : A, (a = b) \vee \neg(a = b)$ is provable.

Decidable equality for $A$ means there is an algorithm that determines, given any two elements $a, b : A$, whether they are equal or not.

## Examples of Decidable Propositions

**The natural numbers have decidable equality.** Given $m, n : \mathbb{N}$, we can determine whether $m = n$ by induction: both zero means equal; one zero and one successor means unequal; both successors means reduce to the predecessors. This is a finite algorithm.

**Equality of booleans is decidable.** There are only four cases: $(\mathsf{true}, \mathsf{true})$, $(\mathsf{true}, \mathsf{false})$, $(\mathsf{false}, \mathsf{true})$, $(\mathsf{false}, \mathsf{false})$. Case-split and check directly.

**Decidability of bounded quantification.** If $P : \mathbb{N} \to \text{Prop}$ is decidable and $n : \mathbb{N}$, then $\exists k < n, P(k)$ and $\forall k < n, P(k)$ are both decidable: check $P(0), P(1), \ldots, P(n-1)$ in sequence.

**Decidability is not preserved by unbounded quantification.** Even if $P : \mathbb{N} \to \text{Prop}$ is decidable (there is an algorithm for each instance), $\exists n : \mathbb{N}, P(n)$ need not be decidable — this would solve the halting problem.

## Hedberg's Theorem

The connection between decidable equality and proof theory is captured by a remarkable result:

**Theorem (Hedberg's Theorem, 1998).** If a type $A$ has decidable equality, then $A$ is an *h-set* — any two proofs of $a = b$ in $A$ are themselves equal.

In other words: for types with decidable equality, equality proofs are unique. There is at most one proof that $a = b$, and it does not matter which one you use.

*Proof sketch.* Given decidable equality, define for each $a, b : A$ a function that picks a "canonical" proof of $a = b$ if one exists (and the unique element of the empty type otherwise). Use this canonical proof to show that any two proofs of $a = b$ are equal via a path-contraction argument. $\square$

Hedberg's theorem tells us that decidable equality is a *sufficient condition* for a type to be an h-set. It is not necessary: there are h-sets without decidable equality (in general constructive settings), but having decidable equality always implies h-set status.

In HoTT terms: types with decidable equality are exactly those for which the space of identity proofs is homotopically discrete — a set, not a more complex topological object.

## The h-Level Hierarchy Foreshadowed

The concept of decidability is the first step toward the h-level hierarchy in HoTT.

**h-Level -2 (Contractible types):** A type $A$ is contractible if there is a distinguished element $a : A$ and a proof that every element equals $a$. Contractible types have "up to homotopy" exactly one element.

**h-Level -1 (Mere propositions, h-propositions):** A type $A$ is a mere proposition if any two of its elements are propositionally equal: $\forall x, y : A, x = y$.

**h-Level 0 (h-Sets):** A type $A$ is an h-set if for any $a, b : A$, the identity type $a =_A b$ is a mere proposition. There is at most one proof of any equality.

**Higher h-levels:** $A$ is an h-groupoid (h-level 1) if identity types are h-sets; and so on.

Decidable propositions are always mere propositions (h-propositions): a decidable proposition either has a proof or it doesn't, and any two proofs of a proposition that has a unique proof are equal. Types with decidable equality are always h-sets (by Hedberg's theorem).

This hierarchy is not just a classification — it is a mathematical tool. In HoTT, whether a type is an h-set determines what kind of mathematics can be done with it. Group theory works over h-sets. Category theory requires h-groupoids. Higher algebraic structures require higher h-levels.

## Decidability and Stability

There are weaker notions than decidability that are constructively useful:

**$\neg\neg$-Stability:** $P$ is $\neg\neg$-stable if $\neg\neg P \to P$. This says double negation elimination holds for $P$.

**Semidecidability:** $P$ is semidecidable if there is an algorithm that, if $P$ holds, eventually produces a proof of $P$ (but may not terminate if $P$ fails). This is the analogue of "recursively enumerable" for propositions.

**Decidability implies $\neg\neg$-stability.** If $P \vee \neg P$ holds and $\neg\neg P$ holds, then the $\neg P$ branch of the disjunction is impossible (it contradicts $\neg\neg P$), so $P$ must hold. Hence decidable propositions are $\neg\neg$-stable.

The converse fails: $\neg\neg$-stable propositions need not be decidable. Consider any proposition whose truth is unknown (like the Goldbach conjecture): it is either decidable (if we know the answer) or not, but it is $\neg\neg$-stable (assuming it's equivalent to an arithmetic statement, which is $\neg\neg$-stable by double-negation translation).

## Decidable Propositions in Practice

In formal proof assistants, decidable propositions have an efficient computational representation: they correspond to *boolean-valued* functions. A proposition $P$ is decidable if and only if there is a boolean function $f : \mathbb{B}$ such that $P \leftrightarrow (f = \mathsf{true})$.

In Coq, this is captured by the `Decidable` type class and the `sumbool` type `{P} + {~P}`. In Lean 4, it is the `Decidable` typeclass. In Agda, it is the `Dec` datatype.

Having a `Decidable` instance for a proposition allows:
- **Boolean reflection.** Replacing a proof search with a boolean computation. Instead of constructing a proof of $P$, compute the boolean $f$ and use the fact that $f = \mathsf{true}$ implies $P$.
- **Automation.** Proof automation can often handle decidable goals mechanically — no manual proof needed.
- **Program extraction.** The decision procedure is itself a computable function, which can be extracted and used in verified software.

The Ssreflect library in Coq exploits decidability systematically: many lemmas are stated with a boolean condition (computable) rather than a Prop (proof-based), and a reflection principle allows switching between the two. This enables very efficient formal proofs of combinatorial and algebraic results.

## Markov's Principle and Decidability

Markov's principle (Section 5) is precisely a statement about decidability: for decidable $P : \mathbb{N} \to \text{Prop}$, $\neg\neg \exists n, P(n) \to \exists n, P(n)$.

MP says: for decidable predicates, the unbounded search terminates if we know it's not the case that the search diverges. This is computationally justified: run the search; if MP's hypothesis holds, the search eventually terminates.

In the language of this section: MP says decidable propositions behave classically for *existential* statements. They are $\neg\neg$-stable with respect to existence claims, even though unbounded existential statements over decidable predicates need not themselves be decidable.

## Decidability in HoTT: The Excluded Middle for Propositions

In HoTT, there is a weaker form of LEM that is consistent with the univalence axiom:

**Propositional LEM (pLEM):** For every mere proposition $P$ (h-proposition), $P \vee \neg P$.

This is weaker than full LEM (which asserts $P \vee \neg P$ for every type $P$, including types with multiple elements). pLEM restricts LEM to the h-propositional fragment — the "classical" layer of the type theory.

pLEM is consistent with univalence. Adding it does not collapse the higher-dimensional structure of types, because it only makes claims about h-propositions (which have at most one element and no interesting path structure). The non-trivial topological content of HoTT is in the higher h-levels, which pLEM does not touch.

In fact, in many $\infty$-topos models of HoTT, pLEM holds: the sheaf semantics for a classical Grothendieck topos satisfies pLEM. The univalent foundations program therefore treats pLEM as a consistent but not assumed extension of HoTT — something that can be added to obtain "classical HoTT" without destroying the homotopy-theoretic content.

Decidable propositions are thus the constructive analogue of "classical h-propositions" — propositions that can be checked by algorithm, without appeals to oracles or non-constructive principles.
