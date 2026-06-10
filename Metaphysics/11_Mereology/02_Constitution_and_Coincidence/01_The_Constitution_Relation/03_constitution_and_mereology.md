# Constitution and Mereology

The constitution relation and mereology are intimately related but conceptually distinct — and in classical mereology, constitution theory is not merely an alternative view but a direct contradiction of one of the central axioms. In classical mereology, identity is determined by parts: two things with the same parts are identical (extensionality). The constitution view denies this. Two things can share all their parts and yet be numerically distinct. This makes constitution irreducible to mereological composition, and forces us to choose: revise classical mereology, or give up constitution.

## The Clash with Extensionality

Mereological extensionality (EXT) states: ∀x ∀y [∀z (PP(z, x) ↔ PP(z, y)) → x = y]

Constitution theory asserts that there are objects that violate EXT: the statue s and the lump c share all their proper parts at time t, yet s ≠ c. The argument against EXT from constitution runs:

- P1. Con(c, s, t) — the lump constitutes the statue at t
- P2. ∀z [PP(z, s) ↔ PP(z, c)] — the statue and lump share all proper parts at t
- P3. s ≠ c — they are numerically distinct (by Leibniz's Law arguments)
- C. EXT is false — there exist objects with the same proper parts that are not identical

This puts constitution theorists in a difficult position: they must either revise classical mereology to drop EXT, or show that EXT is not really violated (by finding some difference between the parts of s and c), or accept EXT and give up the distinctness of s and c.

## Three Strategies

The four-dimensionalist strategy attempts to preserve EXT while denying that the statue and lump share all their parts. The key is temporal parts. Let O_t denote the temporal part of object O at time t:

- The lump c has temporal parts extending from some time t₀ (when the bronze was first worked) to tₙ (when it is melted or destroyed)
- The statue s has temporal parts extending only from t₁ (when the sculptor finished) to t₂ (when the statue is destroyed)
- Since t₀ < t₁, the lump has temporal parts — c_{[t₀, t₁)} — that the statue lacks

Therefore: ∃z [PP(z, c) ∧ ¬PP(z, s)], and hence c ≠ s. EXT is preserved because they are genuinely distinct four-dimensional objects. The four-dimensionalist conclusion: the statue and lump share all their *instantaneous* parts (their time-slices during co-existence), but differ in their temporal parts outside this window.

The objection is that this response works only if temporal parts are genuine parts — which is precisely what endurantists deny. On the endurantist view, objects are wholly present at each moment; the "temporal parts" to which the perdurantist appeals simply do not exist. The four-dimensionalist response to the constitution puzzle thus buys its success at the cost of adopting four-dimensionalism, which has its own costs.

Kit Fine proposes the most technically developed alternative: *relativized mereology*. In Fine's framework, parthood is not a single two-place relation P(x, y) but a family of relations P_K(x, y) — parthood relative to a principle of unity K. An atom is a part of the statue *under the unity principle for artworks* (K_A), and part of the lump *under the unity principle for material stuff* (K_M). These are different parthood relations.

**Relativized EXT:** ∀x ∀y ∀K [∀z (PP_K(z, x) ↔ PP_K(z, y)) → x = y]

Within each principle of unity, extensionality holds. But across different principles of unity, there is no cross-domain extensionality. Applied to the statue-lump case: though they may have the same K_M-parts (both are composed of the same material atoms), they have different K_A-parts — the statue has parts-qua-artwork (the torso-as-compositional-element, the arm-as-gesture) while the lump does not. So cross-domain EXT does not apply.

The objection is that this move may be *ad hoc*: the multiplicity of parthood relations seems introduced precisely to handle the coincidence puzzle, without independent motivation. What determines whether z is an artwork-part of s? If this is determined by the artist's intentions or aesthetic conventions, then mereology has become dependent on intentional and normative facts in a way that seems foreign to the formal discipline. Fine's reply is that the principles of unity are metaphysically real — the artwork principle is determined by the kind *artwork*, a genuine natural kind with its own identity conditions, not a human convention.

Baker resists both moves. Against relativized mereology: the notion of "parts-qua-artwork" introduces a kind-relative notion of parthood that obscures the real issue. Parthood is a material relation; the atoms that compose the statue and the atoms that compose the lump are the same atoms — there are no "artwork-atoms" distinct from "material-atoms." Against four-dimensionalism: temporal parts are ontological posits that are themselves controversial. The common sense datum — that past stages of an object are not its current parts — is violated, not honored, by the four-dimensionalist reconstruction. Baker's conclusion is that constitution is a basic relation in the same way that causation is basic: we cannot reduce it to something more fundamental, and attempts to do so invariably distort the phenomena.

## Constitution, Composition, and Grounding

A more recent approach connects constitution theory to the framework of *grounding*. On this approach, the statue's existence is *grounded in* the lump's existence (given the right circumstances — the right artist, the right context of use). The grounding is asymmetric: the lump does not depend on constituting any particular artwork. And the grounding is not mereological: the statue is not grounded in the lump by being composed of it in the mereological sense but by being constituted by it.

This framing — associated with Fine's ontology and with grounding theorists like Audi, Rosen, and Correia — separates the question of existence from the question of mereological composition. The statue exists; its existence is grounded in the lump's existence; this grounding relation is neither identity nor mereological composition. Whether this resolves or merely relocates the tension with mereological extensionality depends on whether the grounding relation is compatible with two grounded and grounding objects sharing all their parts. That remains a live question in the literature — one whose resolution will depend on how the grounding relation itself is understood.
