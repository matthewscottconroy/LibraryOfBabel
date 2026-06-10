# Mereological Extensionality

Consider a gold ring. If you replace every atom with a qualitatively identical atom from another ring, in the same configuration — is it the same ring? The intuition that it is reflects a deep structural commitment: material objects are individuated by their content, not by their description or mode of presentation. Two things made of exactly the same stuff, organized in exactly the same way, are one thing. This is the mereological analog of the set-theoretic axiom of extensionality — just as sets with the same members are the same set, objects with the same parts are the same object.

**Extensionality (EXT):** ∀x ∀y [∀z (PP(z, x) ↔ PP(z, y)) → x = y]

In classical mereology, EXT is a theorem derivable from M1–M5. Suppose ∀z (PP(z, x) ↔ PP(z, y)). By M2 (Antisymmetry), it suffices to show P(x, y) and P(y, x). Suppose for contradiction that ¬P(x, y). By M5 (Strong Supplementation): ∃z [P(z, x) ∧ D(z, y)]. Let z₀ be such a part. The careful derivation uses the equivalence between EXT and M5 in the context of M1–M3, confirming that given M1–M3 and M5, any two objects with the same proper parts are identical — *co-partedness implies identity*. Objects are individuated by their mereological profiles.

EXT also has theoretical advantages beyond the intuitive. Without it, mereology becomes radically underdetermined: knowing the complete mereological profile of every object would not tell you how many objects there are, because multiple distinct objects could share all their parts. Parthood would then be a very weak guide to ontological structure.

## The Coincidence Puzzle

The main challenge to extensionality comes from cases of apparent *material coincidence* — objects that seem to share all their parts but are intuitively distinct. The canonical case: the statue *s* and the lump of bronze *c*.

**Argument for coincidence (against EXT):**

- P1. At time t, every proper part of s is a proper part of c, and vice versa: ∀z [PP(z, s) ↔ PP(z, c)]
- P2. s would be destroyed by melting (statues cannot survive being melted and recast)
- P3. c would not be destroyed by melting (lumps of bronze survive changes of shape)
- P4. ∀x ∀y [x = y → (Fx ↔ Fy)]     (Leibniz's Law)
- P5. *Would be destroyed by melting* is a genuine property
- C1. s ≠ c     (from P2, P3, P4, P5)
- C2. EXT is false     (from P1 and C1)

The argument is formally valid. To save EXT, one must reject at least one premise. Three strategies dominate the literature.

## Strategies for Preserving Extensionality

The natural first response is four-dimensionalism: reject P1 by arguing that the statue and the lump have different temporal parts. The lump has a temporal part extending before the casting — when the bronze existed in some earlier shape; the statue does not. As four-dimensional wholes, they differ mereologically, so ∀z [PP(z, s) ↔ PP(z, c)] is false when we count temporal parts. This is the response favored by Lewis and Sider. It preserves EXT at the cost of endorsing four-dimensionalism, which is itself contested.

A second strategy is modal property reductionism: reject P5 by arguing that modal properties like "would be destroyed by melting" are not genuine intrinsic properties of objects but are descriptions relative to a sortal concept or context. When we say "the statue would be destroyed," we mean: under the description *statue*, this object would cease to satisfy the relevant predicate. When we say "the lump would not be destroyed," we mean: under the description *lump*, the same object would continue. Modal truths are sortal-relative or description-relative, not properties of objects *simpliciter*. This response is associated with Nicholas Wolterstorff and, in different form, with David Wiggins's sortal essentialism.

A third and more revisionary approach comes from Kit Fine. In "Things and Their Parts" (1999), Fine argues that parthood is not a single relation but is always *relative to a principle of unity*. An atom is a proper part of the statue qua artwork, but not necessarily a proper part of the lump qua material stuff, because the principles of unity differ. EXT holds *within* each sortal domain — two statues with the same statue-relative proper parts are the same statue — but the statue and the lump inhabit different sortal domains and cannot be compared directly. The resulting system is more complex: the single relation P(x, y) must be replaced by a family of relations P_K(x, y) for each sortal K.

## The Reach of the Problem

Beyond the statue-clay case, EXT generates difficulties in several domains. The identity of biological organisms with their matter is contested: Aristotelians say form imposes additional structure that matter alone cannot carry; classical mereologists say same matter, same object. The theory of events is troubled: is the event of the water reaching boiling identical to the event of the pot's contents reaching boiling? If events are individuated mereologically by their spatiotemporal parts, EXT implies they are — but event theorists often want finer-grained individuation. And set theory draws a sharp line between singletons: {a} and {b} are distinct even if a = b, whereas a mereological sum is just a and b taken together. Lewis's *Parts of Classes* addresses this by arguing that the singleton function is a primitive operation that cannot be reduced to mereology alone.

## Varzi's Assessment

Achille Varzi's "Mereological Commitments" (2000) and the *Handbook of Mereology* (2019, with Cotnoir) offer the most systematic recent treatment. Varzi argues that commitment to EXT is a revisable component of classical mereology rather than a conceptual truth, and that there is a coherent space of "non-extensional mereologies" — systems dropping M5 or replacing it with something weaker — that can accommodate coincident objects while retaining unrestricted fusion. The cost is that the fusion operation is no longer guaranteed unique, and the elegant Boolean algebra structure is lost.

The debate over extensionality is thus a debate about what we want mereology to be: a theory of the minimal formal constraints on any part-whole relation, or a substantive metaphysical theory of how physical objects are individuated. Leśniewski and Goodman wanted the latter; Fine and the constitution theorists want something closer to the former. The tension is deep and unresolved, and which side you come down on has consequences that ramify through much of the rest of metaphysics.
