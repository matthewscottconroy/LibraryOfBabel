# Sets, Elements, and Notation

> "A set is a gathering together into a whole of definite, distinct objects of our perception or thought — which are called elements of the set."
> — Georg Cantor, *Beiträge*, 1895

## The Audacity of the Idea

When Georg Cantor proposed, in the 1870s and 1880s, that one could treat an *infinite collection* as a single mathematical object — a "completed infinity" — he faced fierce resistance. His own doctoral advisor, Leopold Kronecker, called him a "corrupter of youth." Yet Cantor's intuition proved spectacularly correct: virtually all of modern mathematics can be founded on the single primitive idea of a **set**.

What is a set? Informally, a set is any collection of objects — numbers, people, colors, other sets — considered as a single unified thing. The objects in the collection are called its **elements** or **members**. What makes this powerful is the shift from thinking about individual objects to thinking about *collections as objects in their own right*, collections that can then be collected into further sets, and so on without end.

This section introduces the notation and basic ideas. The deeper structural questions — how to avoid paradox, how to axiomatize — come later. But the notation is where everything starts.

## Membership: The Primitive Relation

The most fundamental relation in set theory is **membership**, written with the symbol ∈ (a stylized epsilon, from the Greek word for "is"):

$$x \in A \quad \text{means} \quad x \text{ is an element of } A$$

Its negation is ∉:

$$x \notin A \quad \text{means} \quad x \text{ is not an element of } A$$

Notice that membership is not symmetric: $x \in A$ does not imply $A \in x$. The relation is also primitive — we do not define it in terms of anything more basic. Set theory takes ∈ as the sole undefined notion from which everything else is built.

**Examples:**
- $3 \in \{1, 2, 3, 4\}$
- $5 \notin \{1, 2, 3, 4\}$
- $\emptyset \in \{\emptyset, \{1\}, \{1, 2\}\}$ — the empty set is itself an element of this set
- $\{1\} \in \{\{1\}, \{2\}, \{3\}\}$ — sets can be elements of other sets

## Writing Sets: Roster and Set-Builder Notation

There are two primary ways to write a set explicitly.

**Roster (extensional) notation** lists the elements between curly braces:
$$A = \{2, 4, 6, 8, 10\}$$

This makes the set's content completely explicit. For small or patterned sets, this works well. The set $\{1, 2, 3, \ldots, 100\}$ uses an ellipsis for the obvious pattern.

**Set-builder (intensional) notation** specifies membership via a condition:
$$B = \{x \mid P(x)\}$$

Read: "the set of all $x$ such that $P(x)$." For example:
- $\{x \mid x \in \mathbb{Z} \wedge x > 0\} = \{1, 2, 3, 4, \ldots\} = \mathbb{Z}^+$
- $\{x \mid x \in \mathbb{R} \wedge x^2 = 2\} = \{-\sqrt{2}, \sqrt{2}\}$
- $\{n \in \mathbb{N} \mid n \text{ is prime}\} = \{2, 3, 5, 7, 11, 13, \ldots\}$

The vertical bar | is sometimes written as a colon : instead.

**Caution**: The expression $\{x \mid x \notin x\}$ — "the set of all sets that are not members of themselves" — looks like valid set-builder notation but leads to catastrophic contradiction. We will examine this in section 03 on Russell's Paradox. For now, file it away as a warning that set-builder notation requires constraints.

## The Empty Set

The set with no elements at all is called the **empty set**, written $\emptyset$ (or sometimes $\{\}$):

$$\emptyset = \{x \mid x \neq x\}$$

Since nothing satisfies $x \neq x$, this set has no elements. The empty set is unique — there is exactly one set with no elements, by the Extensionality axiom below.

The empty set is peculiar and worth reflecting on. It is a *something* — a mathematical object, a legitimate set — that contains *nothing*. It is like a container that is guaranteed to be empty. This distinguishes it from the absence of a container altogether: $\emptyset$ exists; it just has no members.

A fact that surprises many students: for any statement $P(x)$, the claim "for all $x \in \emptyset$, $P(x)$" is **vacuously true**. There are no elements to check, so the universal quantifier is trivially satisfied. We will see this pattern repeatedly in set theory proofs.

## The Extensionality Principle

What makes two sets *equal*? Not their syntax, not how they were defined, but their **elements** — and only their elements:

$$A = B \quad \iff \quad \forall x\,(x \in A \leftrightarrow x \in B)$$

This is the **Axiom of Extensionality** and it has profound consequences:

- $\{1, 2, 3\} = \{3, 1, 2\}$: sets are **unordered** — listing order does not matter
- $\{1, 1, 2, 3\} = \{1, 2, 3\}$: sets have **no duplicates** — repetition is ignored
- Two completely differently defined sets that happen to contain the same elements are literally the same set

This is why sets are sometimes called *extensional* objects — they are fully determined by their extension (the collection of things they contain), not their intension (how they are described). Compare: the set of even primes and the set $\{2\}$ are the same set, even though the descriptions differ.

## Standard Number Sets

Mathematics uses these standard infinite sets constantly:

| Symbol | Name | Elements |
|--------|------|----------|
| $\mathbb{N}$ | Natural numbers | $0, 1, 2, 3, \ldots$ (sometimes starting at 1) |
| $\mathbb{Z}$ | Integers | $\ldots, -2, -1, 0, 1, 2, \ldots$ |
| $\mathbb{Q}$ | Rationals | $p/q$ where $p, q \in \mathbb{Z}$, $q \neq 0$ |
| $\mathbb{R}$ | Real numbers | All decimals (including irrationals) |
| $\mathbb{C}$ | Complex numbers | $a + bi$ where $a, b \in \mathbb{R}$ |

Each is a subset of the next: $\mathbb{N} \subset \mathbb{Z} \subset \mathbb{Q} \subset \mathbb{R} \subset \mathbb{C}$.

## Sets as Formal Objects in Lean 4 and Coq

In dependent type theory — the foundation of Lean 4 and Coq — "sets" are represented as predicates. A set of elements of type `α` is simply a function `α → Prop`:

```lean
-- In Lean 4, Set α is defined as α → Prop
-- Membership x ∈ s means s x (the predicate holds for x)

def evens : Set ℕ := {n | n % 2 = 0}

example : 4 ∈ evens := by decide
example : 3 ∉ evens := by decide
```

This is elegant: a "set" is just a characteristic function — a predicate that returns true for members and false for non-members. The Extensionality axiom becomes: two sets (predicates) are equal if and only if they hold for exactly the same elements.

```coq
(* In Coq, using Ensembles or plain predicates *)
Definition evens : nat -> Prop := fun n => exists k, n = 2 * k.
Lemma four_in_evens : evens 4.
Proof. exists 2. reflexivity. Qed.
```

## Stop and Think

*Question*: Is $\{1, 2, 3\}$ the same as $\{\{1\}, \{2\}, \{3\}\}$? What about $\{1, \{1\}\}$?

Answer: No to both. The first contains numbers; the second contains singleton sets of numbers. And $\{1, \{1\}\}$ is a set containing *two* different things — the number 1 and the set $\{1\}$ — illustrating that sets and their elements can coexist as distinct objects.

This distinction matters in foundations: when we build the natural numbers from sets, $0 = \emptyset$ and $1 = \{\emptyset\}$ and $2 = \{\emptyset, \{\emptyset\}\}$, so $1 \neq \{1\}$ — they are fundamentally different set-theoretic objects representing the same number at different levels.

## Exercises
See [problems/ch06_set_theory/01_set_operations_exercises.md](../../../problems/ch06_set_theory/01_set_operations_exercises.md)
