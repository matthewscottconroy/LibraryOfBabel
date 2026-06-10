# Vague Parthood

Distinct from vague composition is the question of vague *parthood*: can it be indeterminate whether one object is a part of another? At the periphery of your skin, cells are in various stages of desquamation. Cell C is attached to your skin at 60% of its surface and freely floating at 40%. Is C a part of your body? Intuitively, neither a clean yes nor a clean no seems appropriate. This is not a failure of information — even with complete physical knowledge of the cell's position, attachment, and physiology, the question might remain open.

Vague parthood is the thesis: ∃x ∃y [¬Det P(x, y) ∧ ¬Det ¬P(x, y)]

There exist objects x and y such that it is neither determinately true that x is a part of y, nor determinately true that x is not. In classical mereology, parthood is bivalent: for any x and y, either P(x, y) or ¬P(x, y) — there are no gaps.

Why this is particularly troubling: parthood is supposed to be the *primitive* concept of mereology — the undefined relation from which everything else is defined. If the primitive is vague, the entire formal edifice becomes unstable. Overlap (O(x, y) =df ∃z [P(z, x) ∧ P(z, y)]) inherits the vagueness of parthood; fusion (defined via overlap) inherits it too. The neat algebraic structure of classical mereology breaks down.

## Examples of Apparent Vague Parthood

The examples multiply across different domains. The rock at the edge of a river's bank: partly immersed, partly exposed. As water levels change, the rock transitions between clearly being part of the river's bed and clearly not; intermediate stages seem genuinely vague. The air at the peak of a mountain: the mountain's boundaries seem inherently vague — there is no sharp line between the rock of the mountain and the air above it. The molecule M at the periphery of a cloud: the cloud has no sharp boundary; the density of water vapor decreases continuously from the dense center to the surrounding atmosphere.

In each case, asking "is this a part of that?" generates no clear answer, and it seems implausible that we are merely ignorant of some hidden sharp fact.

## Four Responses

The *epistemic view* (Williamson) holds that parthood is bivalent — for any x and y, either P(x, y) or ¬P(x, y) — but we are ignorant of the fact in borderline cases. The boundary cell is either a part of your body or it isn't; we simply cannot determine which. The vagueness is epistemic, not ontic. Williamson's argument: vague predicates are governed by usage, and usage determines meaning. These dispositions, taken collectively, fix a sharp extension for "part" — a set of ordered pairs (x, y) such that P(x, y) is true. This extension exists even if no one can determine it.

The objection is that epistemicism seems phenomenologically implausible here. What additional information would resolve whether the boundary cell is a part of the body? Even with complete physical knowledge, the question seems to remain open — not because of physical ignorance but because there is no sharp fact to know. Williamson replies that we are distinguishing epistemic access from epistemic resolution: the sharp fact about parthood would remain unknowable not because of physical ignorance but because of semantic ignorance about the exact extension of the term "part" as determined by our usage. Semantic facts are themselves knowable in principle through sufficiently fine-grained analysis of usage, but in practice they are inaccessible.

The *semantic view* (supervaluationism) holds that "part" is a vague predicate — its extension is not sharply fixed by the norms governing its use. For any vague predicate, there are many admissible ways to sharpen it, and for borderline cases some sharpenings classify the pair (x, y) as satisfying P, others don't. Applied to the boundary cell: some admissible sharpenings classify it as a part of the body; others don't. Neither "is a part" nor "is not a part" is true under all sharpenings; hence the claim is indeterminate. The advantage is that classical logic is preserved within each sharpening. The disadvantage is a regress: what makes a sharpening "admissible"? If this is itself vague, the analysis is threatened.

The most radical approach accepts vague parthood and revises the logic to accommodate it: *fuzzy mereology*. Here P(x, y) takes values in [0, 1] rather than {0, 1}. The axioms of classical mereology are revised accordingly:

- *Reflexivity:* P(x, x) = 1 for all x
- *Antisymmetry:* P(x, y) + P(y, x) > 1 → x = y
- *Transitivity:* P(x, z) ≥ min(P(x, y), P(y, z))     (in the Łukasiewicz min-norm)
- *Overlap:* O(x, y) = sup_z min(P(z, x), P(z, y))

The boundary cell is a part of the body to degree 0.6; the fully embedded cell is a part to degree 1; the cell floating in the air is a part to degree 0. This directly models the graded character of many parthood intuitions. The disadvantages are significant: the unique-fusion theorem fails, many of the clean algebraic results collapse, and the assignment of specific numerical degrees of parthood seems arbitrary — why 0.6 rather than 0.5 or 0.7?

Four-dimensionalists typically analyze vague parthood as semantic indecision about which precise object we are referring to. "My body" picks out some precise four-dimensional object, but we have not fixed which precise object — there are many candidates, each a slightly different four-dimensional worm. Each candidate has the borderline cell either determinately as a part or determinately not as a part. The vagueness is in the reference of "my body," not in the parthood relation. This preserves the formal cleanness of classical mereology. The objection is that the response does not obviously extend to all cases: for the river-rock case, there are many candidate rivers and many candidate banks, and the vagueness may be multiply ineliminable rather than resolvable by fixing reference.

## Connections to Other Debates

Vague parthood connects to broader questions about vague objects — if parthood can be vague, perhaps objects themselves have vague boundaries, with no sharp line between where the cloud ends and the air begins. It also connects to Unger and Lewis's *Problem of the Many*: there are many slightly different collections of molecules, each with equal claim to being "the cloud." Classical mereology says all of them exist as distinct fusions; common sense says there is one cloud. Vague parthood suggests that the question "is molecule M part of the cloud?" has no sharp answer, which may dissolve rather than solve the problem. And if parthood is vague, constitution is vague (since constitution is defined partly in terms of shared parts), reinforcing the connections among all three topics in this section.
