# Temporal Parts

A road has spatial parts — the northern section and the southern section are different parts of the same road. That seems uncontroversial. Now consider a person. They exist on Monday and they exist on Tuesday. Are the person-on-Monday and the person-on-Tuesday different parts of the same person? The theory of temporal parts says: yes, in exactly the same sense. Just as spatial parts are portions of an object in space, temporal parts are portions of an object in time. People, mountains, tables, and atoms are "worms" extended through both space and time, and temporal parts are the cross-sections of these worms at particular instants or intervals.

## Formal Characterization

Let spacetime regions be denoted by R and temporal intervals by [t₁, t₂].

**Instantaneous temporal part:** s is a temporal part of O at instant t iff:
1. P(s, O)     (s is a part of O)
2. ∃r [Loc(s, r, t)]     (s exists at t)
3. ∀t' [∃r' Loc(s, r', t') → t' = t]     (s exists only at t)
4. ∀y [P(y, O) ∧ ∃r Loc(y, r, t) → P(y, s)]     (s includes all parts of O at t)

Conditions (3) and (4) together make s the *maximal* part of O confined to t — the complete time-slice of O at t.

**Interval temporal part:** s is a temporal part of O across [t₁, t₂] iff P(s, O) and s exists throughout [t₁, t₂] and only during [t₁, t₂], and s includes all parts of O during that interval. Equivalently, it is the fusion of all instantaneous temporal parts of O at times t ∈ [t₁, t₂]. Every temporal part shorter than the whole worm is a *proper* temporal part.

| Spatial | Temporal |
|---|---|
| x is a spatial part of O | x is a temporal part of O |
| O extends through space | O extends through time |
| Different regions of O have different spatial properties | Different temporal parts of O have different properties |
| O's surface is a spatial part of O | O's present-stage is a temporal part of O |

The analogy is not perfect: spatial parts co-exist; temporal parts are at different times. But the perdurantist argues this difference does not undermine the analogy — both are genuine parts, both are located in their respective dimensions, and both contribute to the mereological profile of the whole.

## Motivations for Positing Temporal Parts

The standard four-dimensionalist position (Lewis, Sider) accepts temporal parts for several converging reasons.

*Theoretical unification*: temporal parts let us treat time like space, making objects four-dimensional entities in a unified spacetime. This has theoretical elegance: it unifies our account of how objects extend in space and how they persist through time. The physics of special relativity, which dissolves the space-time distinction into a unified spacetime manifold, is naturally read as supporting this picture.

*Change without contradiction — Lewis's temporary intrinsics argument*: A poker is straight on Monday and bent on Tuesday. These are incompatible intrinsic properties. If the poker is a single object wholly present at both times, it has both *straight* and *bent* — which are incompatible. Contradiction. The temporal-parts solution is that the Monday-stage of the poker has *straight* simpliciter; the Tuesday-stage has *bent* simpliciter. These are distinct objects, so there is no contradiction:

- O_{Monday}: TP(O_{Monday}, O, Monday) ∧ Straight(O_{Monday})
- O_{Tuesday}: TP(O_{Tuesday}, O, Tuesday) ∧ Bent(O_{Tuesday})
- O_{Monday} ≠ O_{Tuesday}     (distinct temporal parts)
- ¬∃t [Straight(O_t) ∧ Bent(O_t)]     (no part of O is both) ✓

*Dissolving coincidence*: the statue and the lump are distinct four-dimensional worms with different temporal extents, preserving mereological extensionality.

*Handling personal identity puzzles*: questions about whether a person who survives a drastic psychological change is the "same person" become questions about whether certain temporal stages are parts of the same worm — a vocabulary that is at least precise, whatever one thinks of its adequacy.

## Objections

The most common objection is that temporal parts are simply strange. When I examine my hand, I do not encounter a temporal-part-of-a-hand; I encounter my hand, wholly present. The perdurantist reply is that strangeness reflects cognitive habits, not non-existence. We are temporally parochial creatures who interact with objects at particular times and do not directly perceive the full four-dimensional worm. We also don't directly perceive electrons, but they exist.

The phenomenology of persistence generates a related worry: when I think of myself persisting through time, I think of *myself* — the very same individual — being present yesterday and today, not of two distinct stages that are suitably related. The four-dimensionalist account seems to replace identity with a complex mereological relation — the two stages are parts of the same worm — which phenomenologically misrepresents what persistence is. The perdurantist reply is that what we care about when planning for the future is not numerical identity between stages but psychological and physical continuity — and continuity is exactly what the worm-relation captures.

There is also the causal agency problem: if my current temporal part is what acts, my past temporal parts are metaphysically inert. But responsibility seems to belong to me — the whole person — not to my current temporal part. The perdurantist's response is that the whole four-dimensional worm is the agent. When we say "I did X yesterday," we mean the worm of which my current stage is a part includes a past stage that did X. Agency and responsibility are worm-level facts.

## Temporal Parts and the Parthood Axioms

Temporal parts fit naturally into the classical mereological framework:

- **Reflexivity:** O is a temporal part of itself (an improper temporal part), satisfying M1.
- **Transitivity:** If s₁ is a temporal part of s₂ and s₂ is a temporal part of O, then s₁ is a temporal part of O, by M3.
- **Extensionality:** Two four-dimensional worms with exactly the same temporal parts are identical — this is what allows the perdurantist to distinguish the statue-worm from the lump-worm.
- **Fusion:** The worm is the fusion of all its instantaneous stages, by M6.

This natural fit between temporal parts and classical mereology is part of what makes perdurantism attractive to those already committed to the classical framework. The theory is not an add-on to mereology; it is the full deployment of mereological principles in the temporal dimension.
