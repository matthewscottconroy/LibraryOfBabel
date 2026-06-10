# First-Order Logic and Existence

Before Frege, "exists" was commonly treated as an ordinary predicate — something you could predicate of an individual, the way you can say "is red" or "is tall." This was, it turns out, a mistake. Kant argued powerfully against it: to say a thing exists adds nothing to our concept of that thing; it merely affirms that the concept has an instance. Frege formalized Kant's insight in a way that transformed how ontological claims are stated and evaluated. Existence is not a first-order predicate applying to individuals but a second-order property of concepts — the property of being non-empty. "There are tigers" is not attributing a property to tigers but asserting that the concept *tiger* is instantiated.

Frege's key innovation in the *Begriffsschrift* (1879) and the *Grundlagen der Arithmetik* (1884) was to treat the existential quantifier as a second-level function. First-level functions take objects as arguments; second-level functions take first-level functions as arguments. "Exists" — more precisely, "there is at least one" — takes a concept and returns a truth value based on whether the concept has any instance. The logical form of "There are tigers" is therefore:

∃x Tiger(x)

This says that the concept Tiger is satisfied by at least one object. It does not predicate anything of any particular tiger; it says something *about the concept Tiger* — namely, that it is non-empty.

This analysis has immediate and significant consequences. It explains why the ontological argument seems to fail. Anselm's argument (*Proslogion*, chapter 2) runs: God is, by definition, the being than which nothing greater can be conceived; a being that exists is greater than a merely conceived being; therefore, if God did not exist, we could conceive of a greater being; so God must exist.

P1. God is, by definition, the being than which nothing greater can be conceived.
P2. A being that exists is greater than a merely conceived being.
P3. If God did not exist, we could conceive of a greater being (one that also exists).
C. God must exist.

The Fregean analysis targets P2: "exists" cannot be a greatness-making property that can figure in a concept's definition, because existence is not a first-order property at all. You cannot build existence into a concept and then read it off, because existence is a fact about concepts themselves — whether they are instantiated — not a feature that concepts confer on their objects. More formally: to say God exists is to say ∃x (x = God ∧ x is God-like). This cannot be derived from the definition of God-likeness alone, because the existential quantifier ranges over a domain that is independently given.

## Classical Logic and Its Presuppositions

A subtle but important feature of classical first-order logic is that it carries existential import differently from traditional Aristotelian logic. In classical logic, "All F's are G" is rendered as ∀x(Fx → Gx), which is vacuously true if nothing is F. This means classical logic allows "All unicorns are horned" to be true even if no unicorns exist — existence of the subject class is not presupposed.

Aristotle's syllogistic, by contrast, presupposed that subject classes are non-empty: "All A is B" was understood to imply that there are A's. His logic was designed for reasoning about kinds of things that were presumed to exist. Classical logic achieves greater generality, including hypothetical reasoning about possibly nonexistent kinds, at the cost of this existential presupposition.

This difference matters ontologically. If we reason in classical first-order logic, we can make true universal claims about unicorns without committing ourselves to their existence. But if we want to capture the Aristotelian intuition that all genuine universals must be instantiated — as Armstrong's "immanent realism" requires — we need to add extra constraints beyond what classical logic itself provides.

## Negative Existentials

A persistent puzzle for any logical analysis of existence is negative existential statements: "Pegasus does not exist," "Phlogiston does not exist," "The present king of France does not exist." These statements appear to be *about* something — Pegasus, phlogiston, the present king of France — yet they say that very something does not exist. How can we refer to what we are denying exists?

On the classical first-order analysis, following Russell, "Pegasus does not exist" is paraphrased as: it is not the case that there is exactly one thing satisfying the description associated with "Pegasus." The name is treated as a disguised definite description, and the negation ranges over the quantifier — no genuine reference to Pegasus is required. This is elegant, but it depends on descriptivism about names.

On a direct reference view (Kripke, Kaplan), "Pegasus" is a rigid designator that directly refers to an individual — not via any description. But if Pegasus does not exist, what does "Pegasus" refer to? Direct reference theorists face the problem of accounting for the truth and meaningfulness of negative existentials without assuming reference to nonexistent objects. Some appeal to possible objects; others to haecceities — individual essences — without their bearers; still others revise the semantics to handle empty names as a special case.

## Ontological Pluralism and the Limits of One Quantifier

One deeper challenge to the identification of existence with the existential quantifier concerns ontological pluralism — the view that there are multiple modes of being. If abstract objects exist differently from concrete ones, or if merely possible objects have a mode of being distinct from actual ones, then a single existential quantifier may not capture all the relevant distinctions. Aristotle's doctrine of *pros hen* predication — existence is said in many ways, all relating back to a central case — was precisely a response to this kind of concern.

Kris McDaniel argues that ontological pluralism requires a family of being-predicates, one for each mode of being, with the ordinary ∃ as a generic quantifier that abstracts away from the distinctions. The opposing Quinean view insists we keep ontology simple: there is one domain, one existential quantifier, and one sense of existence. Any apparent differences in mode of being are differences in the properties of existing things, not differences in their mode of existence. Ontological pluralism multiplies conceptual machinery without corresponding explanatory gain.

The debate here connects directly to free logic, which was developed precisely to allow for names that may fail to refer, separating the logical role of names from existential commitment. The choice between classical quantified logic and free logic is partly a choice about the correct analysis of "exists" — and it remains lively in both philosophy of logic and ontology.
