# Domain Survey: Modality

## Overview

Modality — the study of possibility, necessity, and contingency — stands at the intersection of logic, metaphysics, and semantics. Modal claims are ubiquitous: we say that it is necessary that 2 + 2 = 4, possible that there could have been talking donkeys, and contingent that this particular water is composed of hydrogen and oxygen (because we discovered this empirically, not a priori). The philosophical challenge is to give a metaphysical account of what these claims are about. If possibility is truth in some possible world, what are possible worlds? If necessity is truth in all possible worlds, what range of worlds counts as "all"? And if some truths are necessary because of what things essentially are, what is essence and how does it differ from mere modal profile?

The dominant framework for analyzing modal claims since the 1960s has been possible worlds semantics, developed formally by Saul Kripke (1963) and applied metaphysically by David Lewis (1968, 1973, 1986). The core idea is elegant: a proposition *p* is possible if there is a possible world in which *p* is true; *p* is necessary if *p* is true in every possible world. This framework provides the semantics for modal logic — the system S5 has become the standard logic of metaphysical possibility and necessity — and illuminates the logic of counterfactuals, propositional attitudes, and natural kind terms.

But the framework raises as many questions as it answers. What are these possible worlds? Lewis's answer — that they are concrete spatiotemporal realities, as real as the actual world, differing only in being spatiotemporally isolated from us — is called **modal realism**. It has the considerable advantages of providing a reductive analysis of modality without modal primitives and of generating a systematic semantics for counterfactuals. But it carries the ontological cost of an extraordinary proliferation of concrete reality. **Actualist** alternatives — Adams's worlds as maximal consistent sets of propositions, Plantinga's worlds as maximal possible states of affairs, Stalnaker's world as a concrete fact — attempt to preserve the utility of possible worlds talk without Lewis's ontological excess, but typically at the cost of reintroducing modal primitives somewhere in the analysis.

Parallel to the possible worlds debate, **essence** — what things are necessarily or essentially — has been transformed by Kripke's work in *Naming and Necessity* (1970/1972) and Kit Fine's subsequent refinements. Kripke argued that names are rigid designators (they pick out the same object in every possible world) and that many a posteriori truths, like "water is H₂O," are metaphysically necessary. Fine (1994) then drew a crucial distinction: essential truths are not simply necessary truths about a thing, because some necessary truths hold of a thing without being grounded in its nature. The fact that Socrates is necessarily a member of the singleton {Socrates} is necessary but not essential to Socrates — it is essential to the singleton, not to Socrates. This forces a reconception of essence as more fundamental than modality, rather than the other way around.

---

## Major Positions and Their Logical Relations

### Theories of Possible Worlds

**Lewis's modal realism** (concrete realism): Possible worlds are concrete spatiotemporal wholes — they are as real and as concrete as the actual world. The actual world is distinguished not by any special metaphysical status but only by being the world *we* inhabit (indexical actuality). Modal claims are analyzed without modal primitives: "Possibly *p*" means "There is a world in which *p*," where worlds are genuine concrete entities. Individuals exist in only one world but have counterparts in other worlds; cross-world identity is counterpart-theoretic.

- *Advantages*: systematic semantics for counterfactuals; reduction of modality to quantification over concrete entities; no modal primitives needed.
- *Problems*: extreme ontological excess; ad hoc counterpart relation; does not explain how we know about possible worlds; seems to make the actual world "no different" from merely possible worlds.

**Ersatzism** (abstractist alternatives): Possible worlds are not concrete but abstract — sets, propositions, or linguistic descriptions. Several varieties:

  - *Linguistic ersatzism* (Carnap, Quine): worlds are maximal consistent sets of sentences in a canonical language. *Problem*: expressive limits — it is possible that there are things no language actually names; a linguistic world cannot describe possibilities involving alien properties that actual language lacks.
  
  - *Propositional ersatzism* (Adams 1974): worlds are maximal consistent sets of propositions. *Problem*: propositions seem to be modally-defined entities (a proposition is something that is necessarily true in all worlds, etc.), so the account is circular unless propositions are given a prior analysis.
  
  - *Structural ersatzism* / combinatorialism (Skyrms, Cresswell): worlds are combinations of actual objects and their properties — each world is an arrangement of the actual elements. *Problem*: cannot accommodate possibilities involving alien individuals or alien properties not present in the actual world.

**Primitivism about modality**: Take possible worlds as primitive platonic entities (Stalnaker's concrete facts, Plantinga's abstract states of affairs) — or take modal operators as primitive, not reducible to quantification over worlds. The cost is that modality is unexplained; the benefit is avoiding the excesses of modal realism and the circularities of ersatzism.

**Modal fictionalism** (Rosen 1990): Talk about possible worlds is a useful fiction. We speak *as if* worlds exist, and our modal claims are true insofar as they would be true *if* Lewis's modal realism were correct. Modal claims are analyzed as claims about what is true *in the fiction* of modal realism.
- *Problem*: if Lewis's theory is a fiction, and we analyze "possibly p" as "in the fiction of modal realism, there is a world where p," then the analysis of modal claims involves quantification over fictional entities that don't exist — so it is unclear what grounds the truth of the fiction.

### Metaphysical Modality: Its Source and Logic

**De re vs. de dicto necessity**: *De dicto*: "Necessarily, the number of planets is greater than seven" — the necessity is predicated of the proposition. *De re*: "The number nine is necessarily greater than seven" — necessity is predicated of the thing (the number nine) with respect to a property. Kripke argued that de re necessity is real, not merely verbal; things have essential properties they could not lack.

**Logical vs. metaphysical vs. natural necessity**: 
- Logical necessity: true in all logically possible worlds (consistent models).
- Metaphysical necessity: true in all metaphysically possible worlds (constrained by the essences of things).
- Natural (nomological) necessity: true in all worlds with the same laws of nature.
The distinctions are controversial: Lewis denied a principled distinction between metaphysical and logical possibility; Kripke and Fine insist the distinctions are real.

**S4 and S5 as logics of metaphysical modality**: The system S5 (where whatever is possible is necessarily possible, and whatever is necessary is necessarily necessary) is widely accepted as the correct logic of metaphysical modality. S4 (where what is necessary is necessarily necessary) is accepted by most; the characteristic S5 axiom (◇p → □◇p) is more contested.

### Essence and Essentialism

**Pre-Kripkean essentialism**: Aristotle distinguished essential from accidental properties. In traditional categories: Socrates is essentially human (he could not have been otherwise), but only accidentally snub-nosed.

**Kripkean essentialism** (*Naming and Necessity*): 
- Proper names are rigid designators — they pick out the same individual in every possible world.
- Origin essentialism: Kripke argued that Aristotle could not have had different parents (if this person had come from different gametes, they would not have been Aristotle).
- Substance essentialism: natural kinds have essential microscopic constitutions (water is necessarily H₂O, gold is necessarily element 79).

**Fine's essence-first approach** (1994): Essence is prior to modality, not derivative from it. Rather than defining essence in terms of necessity (a property *P* is essential to *x* iff *x* has *P* in every world where *x* exists), Fine argues for a primitive notion of essence: the essential properties of *x* are those that are constitutive of what *x* is. Necessary truths about *x* that are not grounded in *x*'s nature are not essential to *x* (e.g., Socrates is necessarily a member of {Socrates} but this is not part of Socrates's nature).

**Anti-essentialism** (Quine): There are no genuine de re modalities; talk of what an individual "could not have lacked" is always relative to a description. Objects do not have essential properties in themselves, only under certain descriptions.
- *Kripke's response*: Quine's argument assumes that identity is always description-relative. But rigid designation establishes identity across worlds without relying on descriptions.

### Counterfactuals

**Lewis's similarity semantics** (1973): "If *A* were the case, *C* would be the case" is true iff the closest *A*-worlds to the actual world are *C*-worlds. Closeness (similarity) is measured by a weighting of different aspects of similarity: overall match in particular facts, and then match in laws.
- *Problem*: the relevant notion of similarity is not antecedently clear; Lewis's own criteria (large regions of perfect match, followed by small violations of laws) seem stipulated rather than principled.

**Strict conditional analysis**: Counterfactuals are strict conditionals quantifying over some restricted class of possible worlds. (Stalnaker's single-selection function selects one world — the closest *A*-world — making counterfactuals false when there is no determinate closest world, or true by vacuity when the antecedent is necessarily false.)

---

## Key Arguments and Counterarguments

### The Argument from Recombination (for Modal Realism)

Lewis: We can form a possible world by taking any individuals and any properties from any existing worlds and recombining them freely (subject to exclusion of logical contradiction). This "principle of recombination" generates all possibilities. For this to work as a metaphysical thesis (not merely an epistemic rule of thumb), the possible worlds must be genuine entities that can be combined.

**Objection**: The principle of recombination is stated at the level of modal talk ("we can form a possible world..."); it is a modal principle that presupposes possibility rather than grounding it. Lewis cannot use it to justify modal realism without circularity.

### The Epistemology of Modality

If possible worlds are concrete realities causally isolated from us (Lewis), how do we know what is possible and what is not? We cannot observe other possible worlds. Lewis's response: we know about possible worlds the same way we know about abstract objects — by reason, inference, and understanding what our theories commit us to.

**Chalmers's conceivability-possibility principle**: What is ideally conceivable (that is, conceivable to a perfectly rational mind with full a priori knowledge) is possible. This links a priori reasoning to modal knowledge.

**Objections**: Ideal conceivability is not clearly achievable; the history of mathematics contains claims that seemed conceivable but turned out to be impossible (squaring the circle seemed conceivable before Lindemann 1882).

### Kripke's Argument for A Posteriori Necessities

**P1.** "Water" is a rigid designator — it refers to the same substance in every possible world (namely, the actual watery stuff, whatever it is).
**P2.** The actual watery stuff is H₂O (empirical discovery).
**P3.** Since "water" rigidly designates H₂O, "water is H₂O" is true in every world where water exists.
**C.** "Water is H₂O" is metaphysically necessary, even though it was discovered empirically (a posteriori).

**Putnam's Twin Earth argument**: The meaning of "water" is fixed by the actual substance, not by our descriptions of it. Even if our twin Earth counterparts have identical mental states but use "water" of XYZ (which plays the watery role on Twin Earth), their "water" refers to XYZ, not H₂O.

### Fine's Argument Against Modal Reductions of Essence

**P1.** Socrates is necessarily a member of {Socrates} — he exists in every world only if he is a member of his own singleton.
**P2.** Being a member of {Socrates} is not essential to Socrates — it is not part of *what Socrates is* but rather a consequence of properties of the set {Socrates}.
**C.** The essential properties of *x* cannot be analyzed as the properties *x* has in every world of existence; some necessary properties are non-essential.

**Conclusion**: Essence is a primitive notion that cannot be reduced to modality; the direction of explanation runs from essence to necessity, not vice versa.

---

## Key Papers and Books

1. **Lewis, David, *On the Plurality of Worlds* (1986)** — The definitive defense of modal realism; systematic, clear, and an indispensable starting point for any study of possible worlds.

2. **Kripke, Saul, *Naming and Necessity* (1980; originally 1972)** — Develops the theory of rigid designation, a posteriori necessity, and origin essentialism; one of the most influential works in analytic philosophy.

3. **Fine, Kit, "Essence and Modality" (1994)** — The paper that argues essence is prior to modality; forces a reconception of the relation between necessary truth and nature.

4. **Adams, Robert M., "Theories of Actuality" (1974)** — Develops propositional ersatzism as a realist alternative to Lewis's modal realism; the clearest statement of an actualist theory of worlds.

5. **Lewis, David, *Counterfactuals* (1973)** — Introduces the similarity semantics for counterfactuals; closely connected to his modal realism.

6. **Plantinga, Alvin, *The Nature of Necessity* (1974)** — Develops an actualist possible worlds framework using states of affairs; defends de re essentialism within an actualist framework.

7. **Rosen, Gideon, "Modal Fictionalism" (1990)** — Introduces the fictionalist interpretation of possible worlds talk; the definitive statement of that position and its difficulties.

8. **Chalmers, David, "Does Conceivability Entail Possibility?" (2002)** — Systematic treatment of the conceivability-possibility link; introduces the two-dimensional framework for modal reasoning.

---

## Live Debates and Open Questions

1. **The extent of modal space**: How far does metaphysical possibility extend? Can there be impossible objects (things violating logic)? Can there be alien properties? Can there be necessary existents? Each answer has implications for the structure of modal space and for the adequacy of any possible worlds account.

2. **Two-dimensional semantics and the necessary a posteriori**: Chalmers and Jackson develop two-dimensional frameworks to explain how "water is H₂O" can be both necessary and informative. Critics (Soames, Block and Stalnaker) argue that two-dimensionalism misrepresents the semantics of names and natural kind terms.

3. **Grounding modal facts**: If modal facts are not primitive and not reducible to concrete possible worlds (ersatzism), what grounds them? Fine's answer — essence — faces the challenge that essence itself seems to require modal notions.

4. **Modal logic for metaphysical necessity**: Is S5 the correct logic? Are there failures of S4 (some things are possibly necessary without being necessary)? Actualists and modal realists give different answers.

5. **Impossibility and impossible worlds**: Some phenomena (contradictory fictions, counterpossible conditionals) seem to require impossible worlds — worlds where logical contradictions hold. How should such worlds be understood?

---

## Connections to Other Domains

**Modality and Ontology (Domain 01)**: Possible worlds are entities whose ontological status is itself a major debate — concrete (Lewis) or abstract (actualism). The possible worlds framework is the most influential application of existence-claims and ontological commitment criteria.

**Modality and Causation/Laws (Domain 04)**: Laws of nature support counterfactuals ("if I were to drop this, it would fall"), and dispositional essentialism holds that natural properties are essentially modal — they are intrinsically directed toward their manifestations. Both connections tie the metaphysics of causation to the metaphysics of modality.

**Modality and Identity/Persistence (Domain 03)**: Transworld identity — whether Aristotle in another possible world is the same person as actual Aristotle — is the central issue of modal metaphysics of individuals. Haecceities (primitive individual essences) and origin essentialism are directly connected.

**Modality and Mereology (Domain 11)**: Modal mereology — when is it possible for certain parts to compose a whole? — connects composition questions to possibility. The statue and clay case is essentially a question about the modal properties of composite objects.

**Modality and Philosophy of Religion (Domain 13)**: Ontological arguments for the existence of God appeal directly to modal premises — that God is a necessary being, that existence is a perfection, that conceivability implies possibility. Plantinga's modal ontological argument uses Kripkean possible worlds semantics explicitly.
