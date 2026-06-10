# Property Theory

Property theory is the formal and philosophical study of properties, their relations to each other, and their role in a general ontological framework. At its most ambitious, property theory aims to give a comprehensive account of properties as abstract objects with a rich formal structure — one that can serve as a foundation for semantics, ontology, and the philosophy of mathematics.

Formal property theories go back at least to Church's theory of types and Russell's ramified theory of types, which were developed to avoid paradoxes in the foundations of mathematics and logic. The key issue is that properties seem to give rise to paradoxes: the property of being a property that does not apply to itself generates contradictions if we are not careful about how properties can instantiate themselves.

Zalta's theory of abstract objects provides an axiomatic framework in which abstract objects — including properties and propositions — are admitted as primitive entities satisfying specific axioms. In Zalta's framework, ordinary (concrete) objects exemplify properties, while abstract objects encode properties: the abstract object representing Sherlock Holmes encodes the property of being a detective, even though the abstract object does not exemplify (concretely instantiate) that property.

Bealer's intensional logic treats properties and propositions as primitive logical entities, subject to principles that govern their identity and mutual relations. Properties are fine-grained: the property of being a creature with a heart and the property of being a creature with kidneys are distinct even if necessarily co-extensive, because there are possible contexts in which someone could believe one without believing the other.

The main challenge for property theory is to be rich enough to serve all the theoretical purposes that properties are called on to serve, while avoiding paradoxes and remaining metaphysically intelligible. Getting this balance right is one of the enduring challenges of formal metaphysics.

## Russell's Paradox and Type Theory

Russell's paradox arises from unrestricted comprehension: if for every condition φ(x) there is a property being-φ, then there should be a property of being a property that does not instantiate itself. Call this property R. Does R instantiate itself?

- If R instantiates itself, then R has the property of not instantiating itself — contradiction.
- If R does not instantiate itself, then R has the property of being a property that does not instantiate itself — so R instantiates itself — contradiction.

Russell's solution: *type theory*. Properties are stratified into types, and a property of type n can only be instantiated by entities of type less than n. So:
- Type 0: concrete individuals (Socrates, Mars, this table)
- Type 1: properties of individuals (wisdom, redness, mass)
- Type 2: properties of properties of individuals (being a color is a type-2 property of redness)
- etc.

No type-n property can instantiate itself (because instantiation requires the instantiated property to be of a higher type than the instantiating entity). Russell's R cannot be formed — there is no type at which a property can apply to properties of the same type.

Type theory prevents the paradox but complicates the logic and makes some natural generalizations inexpressible. "Every property has some instances" cannot be expressed as a single sentence in type theory because "every property" would have to range over properties of all types.

## Zalta's Theory of Abstract Objects

Edward Zalta's theory of abstract objects (*Abstract Objects*, 1983; *Principia Logica Metaphysica*, online) provides a formal framework for a broad range of abstract objects, including properties, propositions, relations, and the objects of fiction and mathematics.

**Two modes of predication**: Zalta distinguishes two ways an object can be related to a property:
- *Exemplifying* a property: concrete objects exemplify properties. Socrates exemplifies wisdom by being wise — this is ordinary predication.
- *Encoding* a property: abstract objects encode properties. The abstract object corresponding to Sherlock Holmes encodes the property of being a detective, even though the abstract object does not literally exemplify that property.

**Abstract objects as encoding objects**: Abstract objects (numbers, fictional characters, mathematical structures, Platonic forms) are completely characterized by the properties they encode. The abstract object that is the number 2 encodes the properties: being even, being the successor of 1, being less than 3, etc. The abstract object representing Holmes encodes: being a detective, living at 221B Baker Street, being brilliant at deduction.

**Comprehension for abstract objects**: For any condition on properties, there is a unique abstract object that encodes exactly those properties satisfying the condition. This is a *restricted* comprehension principle (it applies to abstract objects only, not to concrete ones) and avoids Russell's paradox.

**Applications**:
- Mathematical objects (numbers, sets, functions) are abstract objects encoding their mathematical properties.
- Fictional objects (Holmes, Hamlet, Santa Claus) are abstract objects encoding the properties ascribed to them in their respective fictions.
- Platonic Forms are abstract objects encoding the properties that concrete objects imperfectly instantiate.

Zalta's framework is technically powerful and accommodates a wide range of putative abstract objects within a single unified theory.

## Bealer's Intensional Logic

George Bealer's *Quality and Concept* (1982) and subsequent work develop a rich intensional logic in which properties, relations, and propositions are primitive logical entities — not sets, not functions, not possible-worlds constructions, but sui generis intensional objects with their own axioms.

**Fine-grained properties**: Properties are individuated hyperintensionally — more finely than by their extensions, and more finely than by their possible-worlds intensions. The property of being a creature with a heart and the property of being a creature with kidneys are distinct even if necessarily co-extensive. This fine-graining is required for intentional contexts (someone can believe one but not the other).

**Algebraic structure**: Properties form an algebraic structure under operations like conjunction (P ∧ Q: the property of being both P and Q), disjunction (P ∨ Q), negation (¬P), and predication (P applied to an individual a). These operations are subject to axioms that govern their behavior.

**Primitive intensional entities**: On Bealer's view, the attempt to reduce properties to sets or to possible-worlds constructions inevitably loses the fine-grained distinctions that properties require. Properties must be admitted as primitive intensional entities with their own distinctive logic.

The cost: admitting properties, propositions, and relations as primitive entities with primitive operations and axioms is a substantial ontological commitment. Bealer's system is mathematically complex, and the justification for specific axioms is not always clear.

## Property Theory and Formal Ontology

Formal property theory is part of the broader program of *formal ontology* — the application of formal logical methods to questions about what kinds of things exist and how they are related. Key questions for formal property theory:

**Identity of properties**: When are two properties identical? Fine-grained accounts (Bealer, Zalta) individuate properties hyperintensionally. Coarse-grained accounts (Lewis's abundant properties) identify co-extensive properties. The choice among identity criteria has downstream implications for semantics, logic, and metaphysics.

**Self-predication and reflexivity**: Can a property exemplify (or encode) itself? Wisdom might be wise (if wisdom is a virtue, and wisdom itself is the paradigm of virtue). Redness is not red (it is not a colored surface). Type theory blocks self-predication entirely; other systems allow it in restricted forms.

**Higher-order properties**: Properties of properties — like being instantiated, being natural, being simple — require a meta-theory. Formal property theory must accommodate higher-order properties without paradox, typically through stratification (types or orders) or through restriction of comprehension principles.

**Properties and individuals**: In some formal frameworks (Zalta), the distinction between properties and individuals is not sharp — an object can encode properties in a way that makes it a kind of formal analog of the property it encodes. This blurs the boundary between the theory of properties and the theory of objects.
