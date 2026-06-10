# Leśniewski's Mereology

Suppose you want to do mathematics without abstract objects. No sets, no classes, no membership relation — just concrete individuals and whatever structure you can build from them. The project sounds quixotic, but it was exactly the ambition of Stanisław Leśniewski (1886–1939), the Polish logician who gave mereology its first rigorous axiomatization. Leśniewski was troubled by Russell's paradox and suspicious of the way set theory posits abstract collections as entities in their own right. His alternative was to treat "classes" as *concrete wholes* — spatiotemporal aggregates of their members — rather than as abstract sets defined by a membership predicate.

## The Nominalist Reinterpretation

Leśniewski worked within a distinctive tradition of Polish mathematical logic that included Jan Łukasiewicz and Alfred Tarski. His system, which he simply called *Mereology* (one of three foundational theories alongside Ontology and Protothetic), took parthood as primitive and derived compositional notions from it. The motivation was explicitly nominalist: Leśniewski refused to countenance any abstract objects, and he believed Russell's paradox showed that naïve set comprehension was incoherent. Rather than repair set theory, he proposed to replace it.

The key move is to reinterpret class membership. Where the set theorist says "a is a member of the class {a, b, c}," Leśniewski says "a is a part of the collective whole of a, b, and c." The class is not an abstract object over and above its members; it is a concrete whole, the same kind of thing as its parts, differing only in extent. This eliminates the membership relation — which threatens paradox — in favor of the parthood relation, which is extensional and structural.

## The Unrestricted Fusion Principle

The key formal commitment follows directly from this picture. Leśniewski's system is governed by the *Principle of Unrestricted Fusion* — also called Universal Composition: for any non-empty condition φ, there exists an object that is the fusion of all things satisfying φ.

Formally: ∃x φ(x) → ∃z ∀y [O(y, z) ↔ ∃x (φ(x) ∧ O(y, x))]

This makes the system ontologically generous: any bunch of objects, however scattered or heterogeneous, has a sum. The scattered object composed of your left shoe and the Eiffel Tower exists just as surely as a brick wall. The "class" of all red things is a concrete whole — the fusion of every red object in the universe — even though its parts share no spatial proximity. This might seem alarming. But Leśniewski's motivations were formally clean: the unrestricted fusion principle is what gives the system its algebraic structure. The universe of individuals under the parthood ordering forms a complete Boolean algebra minus the null element — every pair of overlapping individuals has a unique *product* (greatest lower bound), and every non-empty collection has a unique *sum* (least upper bound).

The argument for this principle is compelling once you accept the nominalist framework:

- P1. If parthood is a purely structural relation, there can be no structural ground for some collections to have fusions while others do not.
- P2. Any restriction on which collections have fusions would have to appeal to non-structural — perhaps intentional or causal — features of the objects.
- P3. A mereological theory that appeals to non-structural features in this way has abandoned the project of characterizing a purely formal part-whole relation.
- C. Therefore, the fusion principle must be unrestricted.

The standard objection is that unrestricted fusion populates the world with vast numbers of gerrymandered objects — the fusion of the left ear of Socrates and the rings of Saturn — that we do not ordinarily recognize or need. The reply, developed most sharply by Lewis, is that the fusions are not *posited* as new entities; they are acknowledged as already being there. Given the parts, the sum is ontologically costless. Our failure to name or care about gerrymandered fusions reflects our cognitive interests, not any ontological absence.

## The Three-System Architectonic

Leśniewski's three foundational systems form a nested hierarchy. *Protothetic* is a higher-order logic generalizing propositional logic. *Ontology* — not to be confused with metaphysics generally — is a logic of names and the singular/plural distinction, a formalization of the copula "is." *Mereology* is then built on top of both, using the resources of Ontology to define and reason about the parthood relation. This architectonic is unusual by contemporary standards. Modern mereology is typically treated as a first-order theory — a set of axioms in first-order predicate logic — rather than a component of a higher-order foundational system.

The shift from Leśniewski's original setting to the modern first-order formulation was accomplished partly by Henry Leonard and Nelson Goodman. Their "The Calculus of Individuals and Its Uses" (1940) introduced the ideas to English-speaking audiences in a more accessible format. Goodman's nominalist program — attempting to reconstruct meaningful discourse using only individuals and the calculus of individuals, without appealing to sets or universals — gave the formalism philosophical urgency beyond its purely logical interest. Goodman and Quine's joint paper "Steps Toward a Constructive Nominalism" (1947) attempted to deploy the calculus to reconstruct syntax without abstract objects, using inscription tokens rather than abstract sentence types. The project was ultimately abandoned as insufficiently general, but it demonstrated the philosophical ambitions attached to mereological foundations.

## Legacy and Open Questions

Today, Leśniewski's framework is recognized as the historical origin of the field, but contemporary mereology is largely conducted in the more flexible first-order idiom of Peter Simons's *Parts: A Study in Ontology* (1987) and Achille Varzi's systematic presentations. The central topics — the status of unrestricted composition, the connection between parthood and identity, the relationship to set theory — are directly continuous with Leśniewski's founding concerns.

The tension he bequeathed remains unresolved and productive. He wanted to use mereology to eliminate abstract sets, yet unrestricted composition generates a world teeming with fusions that seem no less abstract or mysterious than sets if they include scattered objects no one encounters or refers to. Whether one can be a nominalist about sets while being a realist about mereological sums is a live question: sums may be just as abstract as sets in the ways that matter, in which case the nominalist's bargain comes apart. These are exactly the questions that animate the contemporary literature.
