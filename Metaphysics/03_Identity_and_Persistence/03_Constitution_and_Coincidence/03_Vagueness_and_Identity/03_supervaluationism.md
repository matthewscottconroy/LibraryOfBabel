# Supervaluationism

*A semantic approach to vagueness that preserves classical logic.*

---

How can classical logic — including the law of excluded middle — be preserved while accommodating genuine cases where statements have no determinate truth value? This is the central problem that supervaluationism addresses. Developed by Kit Fine (1975) and applied to identity and vagueness by many subsequent philosophers, supervaluationism is one of the most systematic frameworks for handling vagueness while respecting the structural demands of classical logic.

The core idea: vague terms like "heap," "tall," or "the same object as" admit of multiple *precisifications* — ways of making them more precise without distorting their meaning. "Tall" might be precisified as "over 1.75m" or "over 1.80m." A statement is *super-true* (determinately true) if it comes out true under *all* admissible precisifications; *super-false* (determinately false) if false under all; and *indeterminate* if true under some and false under others. Classical tautologies come out super-true: "Either Tibbles is tall or Tibbles is not tall" is true under every precisification, even if there are borderline cases where neither "Tibbles is tall" nor "Tibbles is not tall" is super-true.

## Formal Structure

Let P = {P₁, P₂, ...} be the set of admissible precisifications of the relevant vague language. For each sentence φ:

- φ is *super-true* iff ∀Pᵢ ∈ P, Pᵢ ⊨ φ.
- φ is *super-false* iff ∀Pᵢ ∈ P, Pᵢ ⊭ φ.
- φ is *indeterminate* iff ∃Pᵢ, Pⱼ ∈ P such that Pᵢ ⊨ φ and Pⱼ ⊭ φ.

The supervaluationist retains classical logic at the level of supertruth: every classical tautology is supertrue, every classical contradiction is superfalse. The cost: some instances of tautologies may be indeterminate. "Tibbles is tall or not tall" is supertrue, but neither "Tibbles is tall" nor "Tibbles is not tall" may be supertrue. The disjunction is true without either disjunct being true — which violates the principle that the truth of a disjunction requires the truth of at least one disjunct.

## Application to Vague Objects and Identity

Applied to the Problem of Many: there are many candidate-clouds (collections of droplets), but under each precisification of "cloud," exactly one of them is the cloud. It is supertrue that there is exactly one cloud (since it is true under every precisification), even though it is indeterminate *which* candidate-collection is the cloud. Formally: for every precisification Pᵢ, ∃!x(x is a cloud in Pᵢ) — there is exactly one cloud under each precisification. So supertruth of "∃!x(x is a cloud)" follows. But "this particular collection of droplets is the cloud" is indeterminate — true under some precisifications, false under others.

Applied to vague objects and identity: "Cloud A is identical with Cloud A'" (where A' differs from A by one peripheral droplet) is neither super-true nor super-false. This captures the indeterminacy without claiming that it is *determinately true* or *determinately false* that they are distinct.

## Supervaluationism and Evans's Argument

Supervaluationism handles Evans's argument by noting that in this framework, "it is indeterminate whether a = b" does not imply that a has a property that b lacks. The property λx.∇(x = a) — the property of being such that it is indeterminate whether one is identical to a — is not well-defined in the supervaluationist framework in the way Evans requires. Under each precisification, "a = b" is either true or false; identity is sharp within each precisification. So under each precisification, ¬∇(a = b). Evans's argument applies within each precisification, showing that under each precisification, identity is sharp. It does not show that across-precisification indeterminacy is impossible.

## The Main Objection: Failures of Bivalence at the Instance Level

Critics of supervaluationism argue that it is deeply unintuitive to say that "Either Tibbles is tall or not" is true without its disjuncts being true. This violates the principle that the truth of a disjunction requires the truth of at least one disjunct — a principle that seems logically fundamental. Fine's response: this objection confuses the logic of a sentence with the logic of its semantic evaluation. The classical logic of sentences is preserved at the supertruth level. The failure of the disjunct to be supertrue is a semantic fact about the *terms* (the vague predicate "tall"), not a failure of classical logic.

Williamson presses further: the supervaluationist has no good account of *which* precisification is correct. Every precisification is admissible, but one of them must be correct — "tall" must have *some* precise extension, even if we do not know which. If there is a fact of the matter about which precisification is correct (epistemicism), supervaluationism is unnecessary. If there is no fact of the matter, then calling all precisifications "admissible" seems arbitrary.

## Supervaluationism and Persons

Applied to personal identity: if "the same person" is a vague predicate, then claims like "Person P at t₂ is the same person as Person Q at t₁" may be indeterminate — neither supertrue nor superfalse. Under each admissible precisification of "same person," the verdict is determinate; the indeterminacy arises only from the choice of precisification. This fits Parfit's observation that personal identity can be indeterminate. On the supervaluationist reading, the indeterminacy is semantic rather than ontic: it reflects the vagueness of our personal identity concept, not a genuine metaphysical gap in the world's facts. Defenders reply that this is exactly the right result — personal identity is not a "deep further fact" (as Parfit says) but a conceptual/semantic matter, and supervaluationism vindicates Parfit's reductionism through a semantic rather than metaphysical route.
