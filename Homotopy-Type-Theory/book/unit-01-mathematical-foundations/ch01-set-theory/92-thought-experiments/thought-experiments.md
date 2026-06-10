# Thought Experiments: Set Theory

## 1. The Ship of Theseus Is a Set

The Ship of Theseus: the ship's planks are gradually replaced, one by one, until every original plank has been replaced by a new one. Is it still the same ship?

Now make this precise using set theory. In ZFC, a ship is a set of planks. After all planks are replaced, the ship is a different set — the original planks are not members of the new ship-set, even if the new ship-set has the same number of members arranged in the same way. The ship's identity in ZFC is determined by extensionality: same elements = same set. Different elements = different set. Period.

But this seems wrong. The gradual replacement preserves the ship's identity through *structural continuity*, not extensional identity. The Univalence Axiom would say: if the two ship-configurations are equivalent (same structure, same relations among parts), they are the same ship.

Question: what is the right notion of identity for physical objects? For mathematical objects? Are these the same question? Does ZFC's extensional identity correspond to anything real, or is it just one choice among many? The answer matters because it determines what "equality" means in our foundation.

## 2. Cantor's Hotel Is Full

Hilbert's Hotel: a hotel with infinitely many rooms, all occupied. A new guest arrives. Can you accommodate them? Yes: move the guest in room n to room n+1, freeing room 1 for the newcomer. The hotel accommodates one more guest while remaining "full."

Now push this further. Infinitely many new guests arrive — a countably infinite bus. Move the guest in room n to room 2n, freeing all odd-numbered rooms for the newcomers. The "full" hotel accommodates infinitely many new guests.

Now an uncountably infinite number of guests arrives — one for each real number. Can you accommodate them? No. The hotel, despite its infinite capacity, cannot accommodate uncountably many new guests while remaining in the same cardinality of rooms. The Cantor diagonal argument shows there is no bijection between ℝ and ℕ.

Question: what does this tell us about the nature of infinity? Is there something genuinely "larger" about uncountable sets, or is this just a formal property of bijections? Aristotle distinguished *potential* infinity (you can always add one more) from *actual* infinity (an infinite collection existing all at once). Is Cantor's hierarchy of actual infinities a coherent mathematical reality, or a formal construction with no metaphysical import?

## 3. The Set of All Sets That Do Not Contain Themselves

We know R = {x | x ∉ x} is not a set in ZFC (it would lead to contradiction). In ZFC, R is a *proper class* — a collection definable by a formula but too large to be a set.

But consider: what does it mean to say R "exists" but is not a set? The membership relation x ∈ R is well-defined for any set x: either x ∈ x or x ∉ x (by the law of excluded middle). And for any set x, either x ∈ R or x ∉ R. So R seems to be a perfectly well-defined object. It just cannot be a set without contradiction.

In NBG set theory, R is a proper class — an object that exists but cannot be a member of anything. In ZFC, R is not an object at all; it is a virtual collection defined by a formula but without any corresponding set.

In type theory: the "collection of all types" is a universe U, not itself a type in U (that would be Type:Type, which is inconsistent). Instead, U lives in a higher universe U'. The question "does R exist?" has a different answer depending on what "exist" means — which universe level you are working in.

Question: is the distinction between "set" and "proper class" a mathematical fact or a foundational choice? Could there be a coherent mathematics in which R is a legitimate object? (The answer is yes — in non-well-founded set theories. What do those theories look like, and what do they enable?)

## 4. Two Real Numbers

Cauchy sequences: define the real number √2 as the equivalence class of all Cauchy sequences of rationals converging to √2. Specifically, consider (1, 1.4, 1.41, 1.414, ...).

Dedekind cuts: define √2 as the set {q ∈ ℚ | q < 0 or q² < 2}.

In ZFC, these are different sets. The Cauchy sequence equivalence class is a set of sequences of rationals. The Dedekind cut is a set of rationals. Different sets, definitionally.

Yet they represent "the same real number." Mathematicians treat them as identical. The transition between representations is transparent in practice.

Question: in what sense are these "the same"? In ZFC, strictly, they are not. In HoTT, the Univalence Axiom would say: if the two constructions are equivalent (there is a canonical isomorphism of complete ordered fields between them), then they are equal in the type universe. But "equal in the type universe" means they are connected by a path — not that they are definitionally identical.

Does this resolve the problem, or just push it back a level? What would it mean to have a foundation where the two constructions of ℝ are *definitionally* identical, not just equivalent? Is that achievable, and at what cost?

## 5. The Banach-Tarski Ball

From the Axiom of Choice, the Banach-Tarski paradox follows: a ball in ℝ³ can be decomposed into finitely many pieces and reassembled into two balls of the same size. This is a theorem, not a paradox (there is no contradiction — only contradiction of geometric intuition).

The pieces involved are non-measurable sets — they have no volume, in the sense that the Lebesgue measure cannot be consistently defined for them.

Question: does this mean the Axiom of Choice is false? Brouwer and his followers said yes: a principle that implies geometrically absurd consequences must be rejected. Most mathematicians say no: the pieces are not physical objects, they have no volume, and the "decomposition" is purely mathematical. The theorem reveals a difference between abstract mathematics and physical geometry, not an inconsistency.

But consider: if we reject AC, we lose Zorn's lemma, Tychonoff's theorem, Hamel bases, algebraic closures. We lose much of mathematics as currently practiced. If we keep AC, we accept non-measurable sets and the Banach-Tarski theorem.

Is this a forced choice, or a choice? Are there "intermediate" set theories — stronger than ZF, weaker than ZFC — that preserve the applications without the paradoxes? (Yes, in fact: the Boolean Prime Ideal Theorem, weaker than AC, suffices for Tychonoff and Hahn-Banach but does not imply Banach-Tarski. What other intermediate principles exist?)

## 6. Gödel's Oracle

Gödel conjectured that CH is false. He believed the universe of sets has much richer structure than the constructible universe L in which CH holds — that there are "large cardinal" axioms that force 2^ℵ₀ > ℵ₁.

Suppose you had a mathematical oracle that could tell you whether any given sentence of set theory is true or false. You ask: "Is the Continuum Hypothesis true?" The oracle answers. Has mathematics been extended?

The disturbing answer: not necessarily. Gödel's completeness theorem tells you that whatever the oracle says is consistent with ZFC (if it says "true," then ZFC + CH is consistent; if "false," then ZFC + ¬CH is consistent). Neither answer tells you anything new about ZFC itself. The oracle is giving you information about a different object — the "true" set theory, if there is one — that lies beyond ZFC.

Question: is there a "true" set theory? Is CH determinately true or false, even if ZFC cannot decide it? This is the question of *set-theoretic realism* versus *set-theoretic pluralism*. Realists (Gödel, Woodin) believe there is a unique intended model of set theory in which every statement is determined. Pluralists (Hamkins, Cohen) believe there is a "multiverse" of equally legitimate set-theoretic universes. What's at stake in this debate? Does it matter for practicing mathematics?
