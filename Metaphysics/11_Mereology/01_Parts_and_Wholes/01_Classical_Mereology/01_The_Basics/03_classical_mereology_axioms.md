# Classical Mereology: The Axioms

Classical mereology is standardly presented as a small set of axioms governing the parthood predicate *P(x, y)* ("x is a part of y"). We have a single undefined primitive, from which everything else is built:

**Primitive:** P(x, y): x is a part of y

**Definitions:**
- PP(x, y) =df P(x, y) ∧ ¬P(y, x)        [proper parthood]
- O(x, y) =df ∃z [P(z, x) ∧ P(z, y)]     [overlap]
- D(x, y) =df ¬O(x, y)                    [disjointness]
- AT(x) =df ¬∃y PP(y, x)                  [x is an atom]
- σx φ(x): the fusion of all φ-ers, defined below

What makes the system philosophically interesting — and philosophically expensive — is what happens when we try to articulate, one by one, the structural commitments that the ordinary concept of parthood carries without ever stating them explicitly.

## The Core Axioms

**Axiom M1 (Reflexivity):** ∀x P(x, x)

Every object is a part of itself. This makes parthood *improper* when x = y; proper parthood, PP, handles the asymmetric case.

**Axiom M2 (Antisymmetry):** ∀x ∀y [P(x, y) ∧ P(y, x) → x = y]

If each of two objects is a part of the other, they are identical. Antisymmetry is what connects parthood to identity, ruling out "mutual parthood" between distinct objects.

**Axiom M3 (Transitivity):** ∀x ∀y ∀z [P(x, y) ∧ P(y, z) → P(x, z)]

Parts of parts are parts of the whole. If my fingernail is part of my finger, and my finger is part of my hand, then my fingernail is part of my hand. Transitivity is sometimes contested — the door handle is part of the door, and the door is part of the house; is the handle part of the house in the *same* sense? The standard mereological answer is yes, and those who resist are typically working with a functional or contextual concept of parthood rather than the pure structural one.

Together M1–M3 make P(x, y) a *partial order*. This is the minimal mereological structure.

**Axiom M4 (Weak Supplementation):** ∀x ∀y [PP(x, y) → ∃z (P(z, y) ∧ D(z, x))]

If x is a proper part of y, then y has at least one part disjoint from x. Remove a proper part from a whole and something must remain. This rules out "minimal wholes" — objects with a single proper part and nothing else.

**Axiom M5 (Strong Supplementation):** ∀x ∀y [¬P(y, x) → ∃z (P(z, y) ∧ D(z, x))]

If y is not a part of x, then y has some part disjoint from x. Strong supplementation implies weak supplementation but not vice versa. It is also equivalent, given M1–M3, to the extensionality principle: objects with the same proper parts are identical. This axiom will carry most of the philosophical weight.

**Axiom M6 (Unrestricted Fusion / Universal Composition):**

∀φ [∃x φ(x) → ∃z ∀y (O(y, z) ↔ ∃x (φ(x) ∧ O(y, x)))]

For any non-empty condition φ, there is an object that is the fusion of all the φ-ers. The fusion z overlaps exactly what overlaps at least one φ-er. This axiom generates the most philosophical controversy: it guarantees that for any two objects a and b, their sum a + b exists; for any collection of objects, however gerrymandered, their fusion exists.

## Key Derived Theorems

**Theorem T1 (Uniqueness of Fusion):** If z₁ and z₂ are both fusions of the φ-ers, then z₁ = z₂.

*Proof sketch:* Suppose z₁ and z₂ are both fusions of the φ-ers. By the definition of fusion, ∀y (O(y, z₁) ↔ ∃x (φ(x) ∧ O(y, x))) and ∀y (O(y, z₂) ↔ ∃x (φ(x) ∧ O(y, x))). It follows that ∀y (O(y, z₁) ↔ O(y, z₂)), which by strong supplementation (M5) implies z₁ = z₂. The fusion of any non-empty collection is therefore unique; we can write it σx φ(x). □

**Theorem T2 (Extensionality):** ∀x ∀y [∀z (PP(z, x) ↔ PP(z, y)) → x = y]

Objects that have exactly the same proper parts are identical. This is the mereological counterpart of the set-theoretic axiom of extensionality — it follows from M2 and M5.

**Theorem T3 (Idempotence):** σx (x = a) = a. The fusion of a singleton collection is that object itself.

**Theorems T4–T5 (Commutativity and Associativity of Sum):** a + b = b + a, and (a + b) + c = a + (b + c). Fusion is order-independent and associative, confirming that the universe of individuals under mereological sum forms a join-semilattice.

## The Boolean Algebra Structure

Classical mereology — M1–M6 plus strong supplementation — generates the structure of a *complete Boolean algebra without a zero element*. Define:

- **Sum (join):** x + y = σz (P(z, x) ∨ P(z, y))
- **Product (meet):** x × y = σz (P(z, x) ∧ P(z, y)), defined when O(x, y)
- **Complement:** −x = σz D(z, x), defined when x is not the universe

The universe itself (the fusion of all objects) serves as the top element. There is no bottom element — no "null individual" — because classical mereology does not countenance an empty individual, and adding one is controversial. The elegance of this algebraic structure is one of the strongest arguments for the system; its philosophical commitments are the strongest arguments against.

## Where the System Gets Into Trouble

M5 — strong supplementation — is equivalent to mereological extensionality: if two objects share all their proper parts, they are identical. This generates the famous puzzle about the statue and the clay. Let s = the statue and c = the lump of bronze. At the time the statue exists, every proper part of s appears to be a proper part of c, and vice versa — they are made of the same atoms in the same configuration. By T2:

- P1. ∀z [PP(z, s) ↔ PP(z, c)]     (the statue and lump share all proper parts)
- P2. ∀x ∀y [∀z (PP(z, x) ↔ PP(z, y)) → x = y]     (T2, extensionality)
- C. s = c     (the statue is identical to the lump)

But Leibniz's Law seems to give the opposite conclusion:

- P1\*. The statue would be destroyed by melting
- P2\*. The lump would not be destroyed by melting
- P3\*. ∀x ∀y [x = y → (Fx ↔ Fy)]     (Leibniz's Law)
- C\*. s ≠ c

Two valid arguments, contradictory conclusions. The classical mereologist must reject P1\* or P2\*, typically by arguing that modal predications are description-relative rather than object-relative, or by adopting four-dimensionalism so that s and c differ in their temporal parts and thus violate the first argument's P1. Either way, the cost is real. Classical mereology is elegant and tractable — it admits complete axiomatization, is decidable (in the predicate-logic version), and has the rich algebraic structure of a Boolean algebra. Its philosophical price is high: vast numbers of gerrymandered objects from M6, and the identity of materially coincident objects from M5. Whether the formal tidiness is worth this price is the central question of contemporary mereology. Peter Simons's *Parts* (1987) surveys the space of weaker mereologies — systems that drop M5 or restrict M6 — and argues that a more nuanced theory can preserve the formal virtues while avoiding some of the philosophical costs.
