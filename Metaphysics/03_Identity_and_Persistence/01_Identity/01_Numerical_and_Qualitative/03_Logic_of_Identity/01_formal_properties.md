# Formal Properties of Identity

*Reflexivity, symmetry, transitivity, and their role in the logic of identity.*

---

Whatever else identity is, it is an equivalence relation. That is, it satisfies three formal properties — reflexivity, symmetry, and transitivity — and these properties are not independently stipulated but follow from simpler logical commitments. Understanding their derivation is more illuminating than merely listing them.

**Reflexivity**: a = a. Every object is identical with itself. This seems trivial, and it is: no matter how an object changes or how it is described, it remains itself. The philosophical interest of reflexivity emerges when we ask whether self-identity is genuinely informative, and it does bear on Frege's puzzle about why "a = a" is trivial while "a = b" can be a discovery.

**Symmetry**: If a = b, then b = a. Identity is symmetric. If Venus is Hesperus, Hesperus is Venus. This ensures that identity is not a directional relation like "is taller than." Metaphysically, symmetry marks the contrast with the constitution relation: if the clay constitutes the statue, the statue does not constitute the clay. That asymmetry is precisely why constitution is not identity — a point that will matter when we examine coincident objects.

**Transitivity**: If a = b and b = c, then a = c. Identity is transitive. If Cicero is Tully and Tully is the greatest Roman orator, then Cicero is the greatest Roman orator. This property is crucial for chains of identity inference. When transitivity appears to fail in a putative identity-chain — as in Reid's brave officer case against Locke's memory theory — that is a diagnostic signal that the relation being tracked is not strict numerical identity but something weaker.

Together, these three properties make identity an *equivalence relation*. But identity is a very special equivalence relation: each equivalence class contains exactly one member. For any object x, the equivalence class [x] = {y : y = x} = {x}, since nothing distinct from x is identical with x. Every other equivalence relation on a domain coarsens identity — it takes distinct individuals and groups them together by some criterion. Identity never does this. It is the *finest* possible equivalence relation.

## Formal Derivations

The axiomatic basis for identity in first-order logic with identity is minimal:

- **(Ref)** ∀x(x = x)
- **(LL)** ∀x∀y(x = y → (φ(x) ↔ φ(y))) [Leibniz's Law schema, for any formula φ]

From these two axioms, symmetry and transitivity are derivable rather than independently assumed. For symmetry: given a = b, by (Ref) we have b = b; by (LL) with φ(x) being "x = a," from b = b and a = b we derive b = a. For transitivity: given a = b and b = c, by (LL) with φ(x) being "x = c," from a = b we get a = c iff b = c; since b = c holds, a = c. This economy is elegant — the logic of identity is underpinned by just one substantive axiom (Leibniz's Law) and one trivial one (reflexivity).

## Philosophical Significance of Each Property

We should not move past these formal properties too quickly. Each carries philosophical weight.

Reflexivity underlies the most basic form of self-knowledge: whatever I am, I am that thing. Descartes' *cogito* can be understood as establishing the reflexivity of the thinking subject. Reflexivity also grounds the logical principle that every proposition implies itself — it is the logical analogue of the metaphysical fact that being is not a predicate that a thing might fail to satisfy with respect to itself.

Symmetry has metaphysical consequences. The asymmetry of constitution — the clay constitutes the statue but not vice versa — shows that constitution is not identity. Any relation that fails symmetry is ipso facto not identity. This is not a merely logical observation: it settles a real metaphysical question about the relationship between a thing and its material.

Transitivity is philosophically vital because it underlies chains of identity inference across time. If the child is identical with the teenager who is identical with the adult, the adult is identical with the child. Violations of transitivity in apparent identity-chains reveal that the relation in question is not strict numerical identity but something weaker — psychological continuity, resemblance, or causal connection. Reid's brave officer case exploits exactly this: the transitivity of identity delivers a contradiction from Locke's memory theory, showing that memory-continuity is not the same as identity.

## Identity as a Relation

Some philosophers (following Russell) have questioned whether identity is a genuine *relation* at all. A genuine relation R(x,y) holds between two distinct relata — but the identity relation holds between an object and itself. This has led to the view that identity is a *degenerate* relation, a limiting case where both argument positions are filled by the same individual. In first-order logic, identity is treated as a primitive with axioms, sidestepping the ontological question. Whether identity is a genuine relation or a logical constant without relational structure is a matter of ongoing debate, but the practical and formal properties are clear and uncontroversial — which is why formal logic can proceed without settling it.
