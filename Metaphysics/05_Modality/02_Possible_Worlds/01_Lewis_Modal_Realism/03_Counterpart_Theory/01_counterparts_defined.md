# Counterpart Theory: Definition and Motivation

Humphrey might have won the election. This seems like a straightforward modal claim about a specific person. The question is: what makes it true? On Lewis's modal realism, possible worlds are concrete realities. But Humphrey exists in only one of them — ours. He cannot exist in another world because no concrete individual can be part of two spatiotemporally disjoint systems. So what is the modal claim about?

Lewis's answer is counterpart theory: there is a world in which some individual — one who resembles Humphrey closely enough in the relevant respects, who shares Humphrey's causal history and character in the right ways — wins the election. That individual is Humphrey's *counterpart* in that world, and Humphrey's winning-possibility is grounded in his counterpart's victory.

## The Motivation: Against Transworld Identity

The target is transworld identity: the view that one and the same individual can exist in multiple possible worlds. Lewis rejected this on mereological grounds. An individual is a concrete spatiotemporal entity — a part of some concrete world. Possible worlds are maximal spatiotemporal systems, and these systems are disjoint: they share no spatial or temporal regions. No concrete individual is a part of two disjoint spatiotemporal systems. A person that existed in multiple worlds would be in multiple spatiotemporally isolated locations at once, which is incoherent for concrete objects (though it is unproblematic for abstract objects like numbers, which have no spatiotemporal location).

There is also a puzzle about properties. If Humphrey exists in both w₁ (where he is brave) and w₂ (where he is a coward), and if he is numerically identical in both worlds, then Humphrey is both brave and not brave — a contradiction. The standard response is to world-index the properties: Humphrey is brave-at-w₁ and coward-at-w₂. But Lewis finds this unsatisfying: the properties that matter — the intrinsic, non-world-indexed ones — cannot be had differently by the same individual. The world-indexing is an ad hoc maneuver that avoids the contradiction without explaining it.

Counterpart theory dissolves the puzzle: the world-w₁ individual is brave, and the world-w₂ individual is a coward. These are different individuals. No single individual has both properties, so there is no contradiction.

## The Counterpart Relation Defined

Lewis's analysis of de re modal claims:

"Humphrey might have won the election" is true if and only if there is a world w and an individual x in w such that x is a counterpart of Humphrey and x wins the election at w.

The **counterpart relation** is a relation of overall qualitative similarity, contextually weighted. x is a counterpart of y at world w in context c iff x is more similar to y (in the relevant respects for c) than any other individual in w. More precisely, the counterpart relation is:

- Many-many: an individual may have multiple counterparts in a world (if multiple individuals there are equally similar), and a world-individual may be the counterpart of multiple actual individuals.
- Not necessarily transitive: x may be a counterpart of y, and z may be a counterpart of x, without z being a counterpart of y.
- Context-sensitive: the relevant similarity relation — which features matter for counterparthood — varies with context.

## Formal Semantics

Lewis provided formal semantics for counterpart theory in "Counterpart Theory and Quantified Modal Logic" (1968). The formal language includes predicates for world-membership (Ixy: x is in world y), actuality (Ax: x is actual), and counterparthood (Cxy: x is a counterpart of y), plus property predicates. The semantic clauses for de re modal operators:

- □φ(x) is true relative to individual a in world w iff for every counterpart b of a in every world v accessible from w, φ(b) is true at v.
- ◇φ(x) is true relative to individual a in world w iff there is some counterpart b of a in some world v accessible from w such that φ(b) is true at v.

This departs from standard S5: modal operators involve the counterpart relation in a way that can generate non-standard behaviors. Modal properties are no longer simply properties of an individual across worlds but properties of the individual's counterparts.

## The Temporal Analogy

Lewis's most powerful argument for counterpart theory is the temporal analogy. How do we avoid contradicting ourselves when we say Socrates is standing at t₁ and sitting at t₂? The endurantist solution is to world-index — or time-index — the properties: Socrates is standing-at-t₁ and sitting-at-t₂. The perdurantist solution (Lewis's preferred view) is that there are distinct temporal parts: the t₁-Socrates-part is standing, the t₂-Socrates-part is sitting. No single temporal part has both properties.

Lewis proposes an exactly analogous treatment for modal variation. As temporal variation is handled by distinct temporal parts, modal variation is handled by distinct modal "parts" — counterparts in different possible worlds. The w₁-Humphrey-counterpart wins; the w₂-Humphrey-counterpart loses. The theoretical unity of treating temporal and modal variation symmetrically is a genuine virtue of the account.

## Context-Sensitivity

The context-sensitivity of the counterpart relation is not a bug but a feature. De re modal claims genuinely vary in what they are about. "Humphrey might have won" (given his character and political position) involves counterparthood based on character and political career. "Humphrey might have been born a woman" involves counterparthood based on origin. "This table might have been made of different wood" involves counterparthood based on general form and function. The flexibility tracks real differences in what we are asking when we ask different modal questions about the same individual.
