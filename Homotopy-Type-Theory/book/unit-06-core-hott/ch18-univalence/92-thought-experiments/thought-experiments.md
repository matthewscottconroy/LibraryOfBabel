# Thought Experiments: Univalence

## Thought Experiment 1: The Identical Twins

Maria and Marco are identical twins. They have the same DNA (up to minor variation). In classical logic, they are two different people — equal in many ways, but not the same person. In a naive application of Univalence, one might want to say: "they are equivalent as biological organisms, so they are equal."

This is wrong, of course. The Univalence Axiom says that *types* that are equivalent are equal. People are not types (in the mathematical sense). But the thought experiment reveals the important question: *which* structure is being equated when we say two types are equal?

For Z/2Z and {0,1} with mod-2 addition: we are saying they are equal *as groups*. Not as sets (they have different elements). Not as bare types (they may have different computational content). But as groups, with all the group-theoretic structure, they are the same.

*Question:* What exactly is the type in which Z/2Z and {0,1} are equal? Is it the type of all groups? The type of all abelian groups? The type of all finite groups? And does the answer matter? (Hint: the answer does matter — the path between them depends on which structure we are equating.)

## Thought Experiment 2: The Space of Equivalences

Consider the type `Bool ≃ Bool`. By Univalence, this is equivalent to `Bool = Bool`. We showed there are exactly two elements.

Now consider the type `N ≃ N` — all bijections from the natural numbers to themselves. This is an enormous type. Countably many elements. Each bijection is a different path `N = N` in the universe.

*Question:* Can you describe a specific "interesting" bijection from N to N? (For example: the bijection that swaps 0 and 1 and leaves everything else fixed. Or the bijection that maps n to n+1 for all n — but wait, this is not a bijection since 0 has no preimage.) The bijection that maps n to n+1 mod some N... pick a specific bijection and trace what path `N = N` it corresponds to. What would transport along this path do?

## Thought Experiment 3: The Uniqueness of the Canonical Isomorphism

In algebra, two isomorphic objects often have a "canonical" isomorphism between them — the "obvious" bijection. For Z/2Z and {0,1}: the canonical isomorphism sends [0] to 0 and [1] to 1. For any vector space V and its double dual V**, the canonical isomorphism is v ↦ (φ ↦ φ(v)).

*Question:* In HoTT, two equivalent types may have *multiple* paths between them (multiple elements of `A = B`). When mathematicians say "the canonical isomorphism," which path are they pointing to? Is there a type-theoretic way to single out the canonical path? What extra data would you need to specify a canonical path among all equivalences?

## Thought Experiment 4: The Logician's Objection

A logician objects to Univalence: "If Z/2Z = {0,1} (as groups), then I can't tell them apart. But Z/2Z has the element [0] (an equivalence class), while {0,1} has the element 0 (an integer). These are different objects! So the types are not equal!"

Answer this objection in HoTT terms. The key question is: what does "the element [0]" mean in a type-theoretic context where we know only that we have *some* group of order 2, without knowing which presentation?

## Thought Experiment 5: Univalence and the Axiom of Choice

The Axiom of Choice (AC) in classical set theory says: for any family of non-empty sets, there is a function selecting one element from each set. In ZFC, this is needed to construct many mathematical objects.

In HoTT, the situation is different. Consider the statement:

"For any family of types A : I → Type where each A(i) is contractible, the product Π(i:I).A(i) is contractible."

This is *provable* in HoTT without any choice axiom (since we can always take the center of each contractible fiber). But the stronger statement — "for any family where each A(i) is merely inhabited (propositionally truncated), the product is merely inhabited" — requires a form of AC.

*Question:* Why does contractibility make choice trivial, while mere inhabitation does not? What exactly is the "choice" that contractibility provides automatically?

## Thought Experiment 6: What Makes a Path "the Same"?

Given two proofs p, q : A = B in the universe (two paths between types A and B, corresponding to two equivalences by Univalence), are p and q "the same"?

In general: no. But when are they the same? When there is a 2-path H : p = q. By Univalence applied one level up, H corresponds to... a natural isomorphism between the two equivalences viewed as functors.

*Question:* What is the 2-path type `(A = B) = (A = B)` in the universe? (Hint: apply Univalence twice — once to get `(A=B) ≃ (A≃B)`, and once to the type of equivalences.) What does a 2-path between two equivalences correspond to mathematically?
