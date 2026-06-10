# Frege and Russell on Existence

Kant's insight that existence is not a real predicate was philosophically important but imprecise. What, exactly, is the difference between existence and ordinary predicates? Frege and Russell gave the precise answer, and it transformed how analytic philosophy thinks about both existence and ontological commitment.

For Frege, the analysis falls naturally out of his fundamental logical distinction between objects and concepts. Objects are self-subsistent; concepts are unsaturated — they require completion by an object. "Exists" is a concept that applies to other concepts: the concept *tiger* falls under the concept *has at least one instance*, just as the individual Frege falls under the concept *is a logician*. This neatly explains why "exists" behaves so differently from ordinary predicates: it is predication at a different logical level, the second level.

The contrast is precise. "Socrates is a philosopher" predicates the first-order concept *philosopher* of the object Socrates: Philosopher(Socrates). "Philosophers exist" predicates the second-order concept *is instantiated* of the first-order concept *philosopher*: ∃x Philosopher(x). In the first sentence, a property is attributed to an individual. In the second, a property is attributed to a concept — namely, the property of having at least one instance. This means "exists" cannot syntactically occupy the same position as "is wise" or "is mortal" in a predication about an individual. "Socrates exists" is grammatically well-formed but logically peculiar: existence is not a property that individuals can have or lack, it is a property of concepts.

Frege's clearest statement of this analysis appears in the *Foundations of Arithmetic* (1884) and "Function and Concept" (1891). The logical analysis also grounds his logicism: number statements, Frege argued, are second-order predications. "The number of F's is n" means that the concept F is equinumerous with the concept "natural number less than n." Arithmetic is thus a part of logic — the thesis Frege spent his career trying to establish.

## Russell and Definite Descriptions

Russell's theory of definite descriptions, developed in "On Denoting" (1905), extended Frege's second-order analysis to handle a broader class of expressions. Definite descriptions — "the present king of France," "the morning star," "the golden mountain" — appear to refer to individuals but may fail to do so. "The golden mountain does not exist" sounds like a statement *about* the golden mountain, implying that there is something — the golden mountain — that fails to exist. This would seem to commit us to nonexistent entities having some form of being.

Russell's paraphrase dissolves this: "The golden mountain exists" becomes ∃x [Golden(x) ∧ Mountain(x) ∧ ∀y((Golden(y) ∧ Mountain(y)) → y = x)]. This is false if there is no golden mountain, and no appeal to a nonexistent entity is needed. More generally, "The F is G" is analyzed as: there exists exactly one F, and that F is G:

∃x [Fx ∧ ∀y(Fy → y = x) ∧ Gx]

The apparent singular reference to "the F" is distributed across an existential quantifier, a uniqueness clause, and a predication. No singular term remains — and so no referential commitment to a specific individual survives. The apparent subject vanishes into the quantifier structure.

Russell and Frege diverge in one important respect. For Frege, definite descriptions have a sense and (if their referent exists) a reference. For Russell, descriptions never refer at all — they are not genuine singular terms but quantifier expressions. This means that for Russell, but not Frege, there is no special kind of entity (the "sense") associated with an empty description.

## The Singular Term Problem

One difficulty for the second-order analysis concerns sentences like "Socrates exists" or "Napoleon existed." These appear to predicate existence of a named individual. If existence is second-order, these sentences should be rewritten as: "The concept *is Socrates* has at least one instance." But this seems trivially true — there is obviously something satisfying the concept *is identical to Socrates*. So "Socrates exists" should be necessarily true if Socrates exists at all, which seems wrong: in a world without Socrates, the sentence should be false.

Frege's response is that singular positive existence statements are only trivially informative. They become substantive only in the context of denials: "Vulcan does not exist" is informative because the name "Vulcan" fails to refer — the concept *is Vulcan*, understood via its associated descriptions, is uninstantiated.

Kripke's *Naming and Necessity* (1980) challenged this. Names are not disguised descriptions; they are rigid designators — terms that pick out the same individual in every possible world in which that individual exists. "Aristotle" refers directly to the individual Aristotle, however that individual is described. If this is right, then singular existence statements are about the individual directly, not about a concept. The Frege-Russell second-order analysis, which treats names as descriptions, does not apply in the same way. Nathan Salmon argues for treating existence as an ordinary property that individuals instantiate — trivially co-extensive with identity in the actual world — rather than a second-order feature of concepts.

## The Meinongian Alternative

The Frege-Russell analysis is motivated partly by the need to avoid Meinongian ontology. Meinong's position: some objects do not exist; we can refer to them and truly predicate properties of them. The golden mountain is golden and is a mountain, even though it does not exist. Frege and Russell found this intolerable: it violates the principle that there are no objects that are not.

Meinong's defenders argue that the Frege-Russell analysis is forced. It works tolerably for definite descriptions but does not naturally handle all cases of apparently empty reference, especially in intensional contexts ("Ponce de León was searching for the Fountain of Youth"), in fiction ("Emma Bovary is more interesting than any real person I know"), and in discourse about failed theoretical posits. In these contexts, we seem to refer to nonexistent objects, make true statements about them, and reason from them to conclusions — and the Russellian paraphrase strategy, while technically available, often distorts the intended meaning.

The contemporary debate between neo-Meinongians (Parsons, Zalta, Priest) and adherents of the Frege-Russell analysis remains active. The choice has significant consequences for the philosophy of fiction, the semantics of intentional contexts, and the metaphysics of possible objects. Together, Frege and Russell's analyses had several landmark consequences: they explained why the ontological argument fails, gave us a logical analysis of negative existence statements without commitment to nonexistent entities, and provided the template for Quine's criterion of ontological commitment. The analysis is not universally accepted, but it represents the default view in analytic philosophy from which any competitor must depart.
