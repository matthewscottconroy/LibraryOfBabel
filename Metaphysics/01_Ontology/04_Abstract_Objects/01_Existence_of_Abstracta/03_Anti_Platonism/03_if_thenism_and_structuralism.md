# If-Thenism and Structuralism

If-thenism (hypothetical necessity, or deductivism) is an anti-Platonist position that reads mathematical claims as conditional: "7 is prime" is not a claim about a particular abstract object but a claim that, if there were objects satisfying the Peano axioms, then the seventh such object would be prime. The "if" converts existential mathematical claims into claims about logical entailment from axioms, which do not themselves require abstract objects to exist.

The position has a long history, associated with Russell's early logicism and with various structuralist programs. It avoids the epistemological problem entirely: we do not need to explain how we access abstract objects, because we are not claiming they exist — we are claiming only that certain axioms entail certain theorems.

If-thenism faces several objections. First, the "if" may not be satisfiable for all mathematical systems: if there is no possible system of objects satisfying the axioms of some infinitary mathematics, then the conditionals are vacuously true (anything follows from a false antecedent), which makes them uninformative. Second, mathematicians do not seem to be talking about hypotheticals; they seem to be asserting straightforwardly that certain things are true about certain objects.

Mathematical structuralism (Benacerraf, Hellman, Shapiro, Resnik) is a more sophisticated descendant of if-thenism. Structuralists hold that mathematics is about structures, not particular objects. What arithmetic describes is the natural number structure — a certain abstract pattern — not any particular set of objects. Some structuralists are Platonists about structures (ante rem structuralism: structures exist independently); others are nominalists who interpret mathematical claims as modal claims about what structural systems could in principle be instantiated.

Structuralism has become one of the most actively developed positions in the philosophy of mathematics, bridging Platonic and nominalist concerns.

## Benacerraf's Identification Problem and Structuralism

Benacerraf's "What Numbers Could Not Be" (1965) created the context for structuralism by showing that numbers cannot be identified with any particular sets. The argument:

1. Multiple set-theoretic definitions of the natural numbers are equally adequate: the von Neumann ordinals (0 = ∅, 1 = {∅}, 2 = {∅, {∅}}, ...) and the Zermelo ordinals (0 = ∅, 1 = {∅}, 2 = {{∅}}, ...) both satisfy the Peano axioms.
2. Each identification makes some mathematical sentences true that the other makes false. On the von Neumann identification, 3 ∈ 4; on the Zermelo identification, 3 ∉ 4. But the question "Is 3 a member of 4?" should have a determinate answer if numbers are particular sets.
3. Since neither identification is privileged, and both are arbitrary, numbers cannot be particular sets.

**Structuralist conclusion**: Numbers are not particular objects at all — they are *positions in the natural number structure*. The natural number structure is the abstract pattern ⟨ω, 0, S⟩ (a set with a distinguished initial element and a successor function satisfying the Peano axioms). Any two systems satisfying these axioms are isomorphic instantiations of the same structure. "3 is prime" is true because the element in the third position in any such structure is prime — not because some particular set has this property.

## Hellman's Modal Structuralism

Geoffrey Hellman's *modal structuralism* is the most technically developed nominalist version of structuralism. Hellman translates mathematical claims into *possible system* claims:

**For arithmetic**: The axioms of arithmetic are satisfiable — there is a possible system of objects that satisfies them. An arithmetic claim "φ" is true iff φ is true in any possible system satisfying the Peano axioms.

Formally, let PA² be the Peano axioms in second-order logic, and let φ be an arithmetic sentence. Then:
- The arithmetic claim "φ" is translated as: □∀X(PA²(X) → φ^X)

Where □ is necessity and φ^X is φ relativized to the domain X.

This translation avoids quantification over abstract mathematical objects — we quantify over *possible systems of objects* (concrete or abstract) rather than over particular numbers. The modal operators (□, ◇) are primitive, not analyzed in terms of abstract possible worlds.

**Challenge**: The modal operators □ and ◇ require either (a) analysis in terms of possible worlds (which are abstract objects), or (b) primitive modality (which is itself metaphysically demanding). The nominalist gain may be illusory if primitive modality is as metaphysically demanding as abstract objects.

## Shapiro's Ante Rem Structuralism

Stewart Shapiro's *ante rem structuralism* (*Philosophy of Mathematics: Structure and Ontology*, 1997) accepts structures as abstract objects while holding that mathematical objects are positions in structures:

**Structures exist ante rem**: The natural number structure exists as an abstract object independently of whether any concrete systems instantiate it. The structure consists of positions — places in the abstract structure — with structural relations among them (0 is not the successor of anything, each position has exactly one successor, etc.).

**Mathematical objects are positions**: The number 3 is the third position in the natural number structure. It has no properties beyond its structural position — there is no fact about whether 3 is a member of 7 (that is a question about sets, not about the natural number structure).

**Epistemological response**: We know about structures through our capacity to recognize structural patterns in concrete instantiations. Arithmetic is learned by counting physical objects; the abstract structure is then grasped as the pattern common to all counting systems. This explains mathematical knowledge without positing mysterious non-causal access.

**Challenge**: The *Identity of Indiscernibles* problem: if two positions in a structure have exactly the same structural relations to all other positions (as happens in symmetric structures), they are structurally indiscernible. But structurally indiscernible entities are identical (by the structural principle). So symmetric structures cannot have genuinely distinct elements — which seems wrong.

## Resnik's Patterns

Michael Resnik's *Mathematics as a Science of Patterns* (1997) offers a related but somewhat different form of structuralism:

Resnik holds that mathematics is about *patterns* — abstract, multi-leveled relational structures that can be identified across different instantiating systems. Mathematical objects are positions in patterns, with no intrinsic nature beyond their pattern-position. Mathematical truths are truths about these patterns.

Resnik's view is explicitly Platonist: patterns exist as genuine abstract objects. But it connects to structuralist anti-Platonism in holding that what is fundamental in mathematics is not individual objects (numbers, sets) but the patterns (structures) they participate in.
