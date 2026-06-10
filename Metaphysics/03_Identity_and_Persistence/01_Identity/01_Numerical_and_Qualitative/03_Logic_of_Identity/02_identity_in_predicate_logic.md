# Identity in Predicate Logic

*How identity is treated in first-order logic and the philosophical issues this raises.*

---

Why does "Hesperus is Phosphorus" feel like a discovery — something Babylonian astronomers learned by painstaking observation — while "Hesperus is Hesperus" feels like an uninformative tautology, the kind of thing one says when making no claim at all? Both sentences express the same relation of identity between the same objects. That is Frege's puzzle, and it sits at the intersection of the logic of identity and the philosophy of language.

In standard first-order logic with identity, "=" is a primitive binary predicate specified by two axiom schemata:

1. **Reflexivity**: ∀x (x = x)
2. **Leibniz's Law schema**: ∀x∀y (x = y → (φ(x) ↔ φ(y))) for any formula φ with x free.

From these, symmetry and transitivity of identity are provable. The Leibniz's Law schema licenses substitution of co-referring terms in any (extensional) context: if "a = b" is true, then "Fa" and "Fb" have the same truth value.

## Numerical Quantification

One of the most important contributions of the identity predicate to logic is that it enables numerical claims. Counting presupposes an identity predicate to specify when two variables range over the same individual:

- "There is exactly one F": ∃x(Fx ∧ ∀y(Fy → y = x))
- "There are at least two Fs": ∃x∃y(Fx ∧ Fy ∧ ¬(x = y))
- "There are exactly two Fs": ∃x∃y(Fx ∧ Fy ∧ ¬(x = y) ∧ ∀z(Fz → (z = x ∨ z = y)))

These formulations are the formal basis of cardinality claims in classical mathematics. The addition of identity to first-order logic enables the expression of claims about how many things of a given kind exist — claims that are expressible in neither identity-free first-order logic nor in propositional logic. This is not merely a technical point: it shows that the logic of counting is deeply bound up with the metaphysics of identity.

## Frege's Puzzle About Informativeness

Frege's answer to the informativeness puzzle is that the *sense* (*Sinn*) of "Hesperus" differs from the sense of "Phosphorus" even though their *reference* (*Bedeutung*) is the same. The informative content of "Hesperus = Phosphorus" lies in the fact that two different senses — two different modes of presentation — pick out the same object. "a = a" is uninformative because both occurrences of "a" present the object in the same way; "a = b" is informative because "a" and "b" present it differently, and learning they co-refer is genuine information about the world.

This solution separates the logic of identity (which cares only about reference and extension) from the epistemology of identity statements (which involves sense and meaning). The identity relation itself is not enriched; what differs is our epistemic access to its terms. Notice that this implies a principled separation between the question "are a and b identical?" (a metaphysical question about the objects) and the question "is it informative to learn that a = b?" (an epistemological question about our access to those objects). Many confusions in the literature on identity arise from running these together.

## The Definability of Identity

A further question: is "=" a genuine logical constant, definable purely in terms of quantification, or an extra-logical primitive?

Russell's second-order definition proposes: x = y ↔ ∀F(Fx ↔ Fy). This defines identity as sharing all properties. But it is a second-order definition, not available in first-order logic. Worse, it faces the philosophical objection that it conflates numerical identity with qualitative indistinguishability — it defines "=" in terms of PII, which is a substantive metaphysical principle rather than a logical truth. Frege treats identity as a primitive logical relation whose axioms are stipulated rather than derived. Quine, skeptical of second-order quantification, accepts identity as a primitive in most contexts. The consensus in contemporary logic is that identity is a primitive constant of first-order logic with identity, characterized by its axioms.

## Intensional Contexts and the Failure of Substitution

A technically important feature: the Leibniz's Law schema in first-order logic licenses substitution of identicals in *all* formulas φ. But this generates incorrect results in intensional contexts. From "Hesperus = Phosphorus" and "George believes that Hesperus is a star," the schema appears to yield "George believes that Phosphorus is a star" — which may be false.

The standard response is to restrict the Leibniz's Law schema to *extensional* formulas — those not containing operators creating intensional contexts (belief, necessity, and so on). Intensional logics (epistemic logic, modal logic) require separate treatment of identity under intensional operators. In such logics, identity still holds in extensional contexts, but substitution may fail under intensional operators. This reveals that the simple two-axiom formulation of identity in first-order logic assumes a purely extensional language — a restriction that must be relaxed for a full treatment of identity in natural language reasoning, and that connects the technical logic of identity back to the metaphysical puzzles about reference and description.
