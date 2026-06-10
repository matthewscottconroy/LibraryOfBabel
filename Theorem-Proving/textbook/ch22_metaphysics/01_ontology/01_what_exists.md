# Ontology and Formal Ontology

Ontology — the study of what exists — is the oldest branch of philosophy. Formal ontology applies logical tools to ontological questions: what categories of entity exist, how they are individuated, how they relate.

## Quine's Criterion

W.V.O. Quine (1948): the ontological commitments of a theory are the values of its bound variables.

"To be is to be the value of a variable."

If a theory's quantifiers range over Xs, the theory is committed to Xs' existence. To determine what a theory says exists, ask: what must exist for the theory's sentences to be true?

**Application to mathematics**: Standard mathematical theories quantify over numbers, sets, functions. On Quine's criterion, these theories are committed to the existence of abstract mathematical objects. The nominalist must either deny the theory or reparse it to avoid quantification over abstracta.

## Categories of Being

Traditional ontological categories:
- **Particulars vs. universals**: This dog vs. the property of being a dog
- **Concrete vs. abstract**: Physical objects vs. numbers, propositions, types
- **Substances vs. tropes**: The cup vs. the redness of this cup specifically
- **Events**: Distinct category from objects? (Davidson vs. Kim)
- **States of affairs**: What makes propositions true (Armstrong)

Formal ontology in the tradition of Husserl and Lesniewski attempts to give axiomatized theories of these categories, amenable to logical analysis.

## Identity Conditions

Leibniz's Law: A = B iff A and B have exactly the same properties.

But what properties? The *indiscernibility of identicals* (if A=B, they share all properties) is uncontroversial. The *identity of indiscernibles* (if they share all properties, A=B) is controversial — can two distinct but qualitatively identical objects exist?

**Black's spheres**: Two qualitatively identical iron spheres in an otherwise empty symmetric universe. Do they have distinct identities?

**Formal treatment**: In FOL with identity, identity is governed by:
- Reflexivity: ∀x. x = x
- Substitutivity: x = y → (φ(x) → φ(y))

These axioms do not settle Black's question — it depends on the metaphysics of identity, not logic alone.

## Persistence Through Time

What makes an object the same object over time?

- **Endurantism**: Objects persist by being wholly present at each time.
- **Perdurantism**: Objects persist by having temporal parts at each time (a 4D worm).
- **Stage theory**: What we call "persisting objects" are instantaneous stages linked by a continuant relation.

Formal treatments use *temporal logic* or *4-dimensionalist* foundations. The metaphysics affects which formal framework is appropriate — not a purely logical question, but one where logic provides the tools.
