# Frege and Concepts

---

When Gottlob Frege reconstructed logic from scratch in the *Begriffsschrift* (1879) and then gave it a philosophical foundation in the essays and *Grundgesetze* that followed, he did not set out to reopen the medieval dispute about universals. But his analysis of predication and generality produced a framework that transformed the debate, introducing conceptual tools that neither the Platonist nor the Aristotelian tradition had possessed.

Frege's fundamental distinction is between *objects* (the referents of singular terms) and *concepts* (the referents of predicates). "Socrates is wise" has a structure in which "Socrates" refers to an object and "is wise" expresses a concept. But concepts, for Frege, are not objects — they are *unsaturated* or *incomplete* entities. They have a gap, a slot, that needs to be filled by an object to form a complete thought. The concept WISE is essentially predicable; the object Socrates is essentially what can be inserted into a predicable slot. These are two fundamentally different logical categories, and the difference is not merely grammatical.

This generates a distinctive treatment of what predicates express. A concept, for Frege, is a function from objects to truth-values: WISE maps each object to either the True or the False, depending on whether that object is wise. "Socrates is wise" is true if and only if the function WISE maps Socrates to the True. This functional analysis has several immediate advantages. It captures the universality of predicates — a single concept applies to every object in its domain — and it gives precise content to logical operations. The complement of concept F is the function that maps each object to the opposite truth-value from F; the intersection of F and G maps each object to the True only if both F and G map it to the True:

- F: Objects → {T, F}
- ¬F: x ↦ (T if F(x) = F; F if F(x) = T)
- F ∧ G: x ↦ (T if F(x) = T and G(x) = T; F otherwise)

Quantification falls out naturally: "everything is wise" is true iff WISE maps every object to the True; "something is wise" is true iff there is at least one object that WISE maps to the True.

## The Concept Horse Paradox

Frege's sharp object/concept distinction generates a famous paradox that he acknowledged but could not fully resolve. The concept *horse* — the referent of "is a horse" — is a concept, not an object. But the phrase "the concept horse" purports to use "horse" as a singular term, as if referring to an object. The sentence "the concept horse is a concept" is, on Frege's own principles, confused: "the concept horse" refers to an object (the referent of a singular term), and objects cannot be concepts.

Frege's response — that we cannot speak *about* concepts without converting them into objects, and that this is an inevitable awkwardness of natural language — is more candid than satisfying. Kerry raised the objection directly in 1892, noting that the theory seems to be self-undermining: to state the theory, Frege must use singular terms for concepts, thereby misrepresenting their nature. Either Frege's sharp object/concept distinction is an artifact of his notation rather than a genuine ontological fact, or natural language is systematically misleading about the nature of predicative entities.

The paradox matters for the universals debate because it shows how difficult it is to articulate the predicative nature of properties. If properties are always "incomplete" in Frege's sense, they cannot be treated as objects of ontology in the straightforward way that the universals debate often assumes.

## Frege's Realism About Concepts

Despite the paradox, Frege is clearly a realist about what predicates express. Concepts are *objective* — they exist and have their extensions independently of any thinker. Two people can think about the same concept without having the same psychological representation; what they share is access to the same objective concept. Mathematics is the clearest illustration: all mathematicians who think about the concept PRIME NUMBER are thinking about the same objective concept, one that maps each natural number to the True or the False quite independently of anyone's mental state.

This is precisely the kind of claim that connects Frege's logic to the universals debate. His concepts play the same role as universals in the Platonist tradition: they are objective, shared, graspable entities that ground the truth of predications and secure the objectivity of thought. But Frege's framework also introduces a new dimension: concepts are extensional entities (determined by their extensions), and the questions of predication are logically prior to questions about the nature of what predicates express.

Russell and early Wittgenstein were shaped by Frege's analysis, and the subsequent analytic tradition's approach to properties and universals has been formed by this framework more than by the classical Platonic/Aristotelian debate it superficially displaced. When Armstrong argues that predicates do not automatically generate universals — that only sparse, natural properties deserve that status — he is already working within a conceptual space that Frege helped create.
