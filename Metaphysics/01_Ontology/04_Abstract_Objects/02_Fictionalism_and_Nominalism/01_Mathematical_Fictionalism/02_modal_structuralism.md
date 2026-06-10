# Modal Structuralism

*The view that mathematical truths are truths about what would hold in any structure of the relevant kind.*

---

Geoffrey Hellman's modal structuralism offers a way to retain the objectivity of mathematics without committing to abstract objects. The key move is to interpret mathematical claims as modal claims — claims about what would hold in any system of objects satisfying the relevant structural conditions.

The arithmetic statement "2 + 2 = 4," on modal structuralism, is not a claim about abstract natural numbers but a claim like: *if there were a system of objects satisfying the Peano axioms, then in any such system, the object playing the role of 2 combined with itself would yield the object playing the role of 4.* This is a modalized conditional: no actual abstract objects are required for its truth, only the coherence of the structural description.

This approach handles the indispensability argument indirectly: mathematics is indispensable because the modal structural claims are genuinely true, and they are true because the relevant structural possibilities hold. The truth of mathematics does not require actual abstract objects but only the possibility of concrete systems satisfying the structural axioms.

Hellman's view faces several challenges. First, it requires an irreducible notion of *logical possibility* (or mathematical possibility) in the modalized conditionals. If these modal notions are themselves understood in terms of abstract possible worlds or structures, the nominalist gain may be illusory. Second, the view must handle the higher reaches of set theory: the claim that large cardinal axioms are possible requires a rich notion of mathematical possibility that is difficult to cash out without appealing to abstract structures.

Third, the relationship between modal structuralism and practice: mathematicians typically do not formulate their claims as modal conditionals. The paraphrase strategy requires showing that the modal reading captures what mathematicians mean, which is contested. Mathematicians who prove theorems about the empty set seem to be committing to *something* more than a conditional claim.

Despite these challenges, modal structuralism represents one of the most sophisticated attempts to reconcile the objectivity of mathematics with nominalist metaphysical commitments, and it has been influential in subsequent debates.

## The Formal Framework

Hellman's modal structuralism translates mathematical claims into modal second-order logic. Let PA² be the conjunction of the Peano axioms in second-order logic, and let φ be an arithmetic sentence. The translation T(φ) is:

**T(φ) = □∀X∀f(PA²(X, f) → φ^(X,f))**

Where:
- □ is the necessity operator (logical necessity)
- X is a second-order variable ranging over possible domains of objects
- f is a function variable (the successor function)
- PA²(X, f) says that X and f satisfy the Peano axioms
- φ^(X,f) is φ relativized to X with f as successor

In addition to this categorical translation, Hellman also requires a *possibility axiom*:

**◇∃X∃f PA²(X, f)**

This asserts that there are possible systems satisfying the Peano axioms — arithmetic is not vacuously true simply because no possible omega-sequence exists. This axiom ensures that arithmetic is not vacuously true on all conditionals.

The result is a translation of arithmetic into modal logic without any quantification over abstract mathematical objects like numbers. The ontological commitments are limited to: (a) concrete objects that could form omega-sequences, and (b) a primitive notion of logical possibility that is not analyzed in terms of abstract possible worlds.

## The Primitiveness of Modality

The most fundamental challenge to modal structuralism is whether the modal primitive — logical possibility — is genuinely less metaphysically demanding than abstract mathematical objects.

Lewis, who was himself a modal realist, argued that possibility must be understood in terms of possible worlds, which are concrete entities on his account (alternative physical universes). If Lewis's analysis is correct, then modal structuralism presupposes a rich ontology of possible worlds rather than eliminating abstract ontology. Hellman rejects this: the modal operators can be taken as primitive, without requiring any analysis in terms of worlds.

**Primitive modality**: Logical possibility is a basic, unanalyzable feature of reality — some things are possible, others are not. This is not explicated in terms of possible worlds or abstract sets. Hellman argues that primitive modality is no more metaphysically suspicious than the abstract objects Platonists posit — and that unlike abstract objects, primitive modality does not give rise to the epistemological problem, since we have a priori access to facts about logical possibility.

**The challenge**: What is the ontological ground for modal facts? When we say "it is logically possible that there be an omega-sequence," what makes this true? For the Platonist, mathematical truth is grounded in abstract objects. For the modal structuralist, the modal truths seem to float free of any grounding — they are just primitively true. This connects to general debates about the truthmakers for modal claims.

## Extensions to Set Theory

The most challenging extension of modal structuralism is to set theory and higher mathematics. Arithmetic is relatively tractable: we can assert the logical possibility of omega-sequences without great difficulty. But what about set theory with large cardinal axioms?

Hellman's translation of set theory requires a possibility assertion:

**◇∃X PA²_set(X)**

where PA²_set is the second-order version of Zermelo-Fraenkel set theory (with appropriate axioms). The possibility of set-theoretic structures with large cardinals requires asserting the possible existence of structures far larger and more complex than any omega-sequence.

The stronger the set-theoretic axioms, the more demanding the corresponding possibility claim. Asserting the possibility of a set-theoretic universe satisfying the axiom of a supercompact cardinal is not obviously more credible than asserting the existence of such a cardinal in an abstract universe. At some point, the nominalist advantage of modal structuralism over Platonism may vanish.

Hellman's response is that the modal claims can be justified by reflection on the patterns of possible concrete systems — we reason upward from smaller possible structures to larger ones, without ever asserting the actual existence of a completed infinite domain. The justification is structural and schematic rather than direct.

## Comparing Modal Structuralism to Other Positions

| Position | Mathematical Objects | Modal Commitment | Epistemology |
|---|---|---|---|
| Platonism (Shapiro) | Abstract, ante rem | Standard | Intuition / pattern recognition |
| Modal structuralism (Hellman) | None (possible systems only) | Primitive logical possibility | A priori modal knowledge |
| Fictionalism (Field) | None | None beyond standard physics | Within-fiction reasoning |
| In re structuralism | Immanent (instantiated only) | Minimal | Empirical |

Modal structuralism occupies a distinctive middle position: it preserves the objectivity of mathematics (mathematical truths are not arbitrary fictions) while rejecting abstract mathematical objects (there are no numbers, just possible systems). The cost is a primitive notion of modality and the challenge of extending the framework to higher mathematics.
