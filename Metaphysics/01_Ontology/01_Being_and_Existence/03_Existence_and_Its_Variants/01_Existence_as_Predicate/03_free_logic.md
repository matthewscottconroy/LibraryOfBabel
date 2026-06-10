# Free Logic and Existence

Classical predicate logic contains a hidden existential assumption that is easy to miss. Every singular term — every name or definite description — is assumed to refer to something, and the rules of inference are calibrated accordingly. From "Fa" (a has property F), classical logic permits us to infer "∃x(Fx)" (something has property F). This is the rule of existential generalization, and it looks innocuous. But it presupposes that "a" refers to something that exists. What about names that fail to refer — "Santa Claus," "the present king of France," "phlogiston"?

The problem is not merely academic. Scientists sometimes introduce theoretical terms and reason about entities whose existence they eventually discover to be fictional. We reason in and about fiction using proper names that do not refer to actually existing individuals. Mathematicians reason about ideal elements and entities whose ontological status is initially unclear. In all these cases, classical logic either commits us to entities we do not want or forces us to declare all such statements false — neither an attractive option.

Free logic is the family of logical systems that liberalize classical predicate logic by allowing for singular terms that may fail to refer, without thereby making inferences from atomic sentences involving those terms automatically valid. In free logic, "Fa" does not by itself entail "∃x(Fx)" — you need an additional existence assumption, "E!a" (a exists), to license that inference. The logical behavior of referring and non-referring terms is distinguished within the formal system.

## The Logical Problem

The rule of existential generalization (EG) is the problematic one: from Fa, infer ∃x(Fx). If "Sherlock Holmes is a detective" is true — if we accept D(s) as true, where s names Holmes — then by EG we can infer ∃x(Dx): something is a detective. But this seems to commit us to Holmes's existence.

Classical logic handles this by restricting interpretation: every name in the domain refers to some object in the domain. If we want "Sherlock Holmes is a detective" to be a true sentence, Holmes must be in the domain — i.e., must exist. Since we do not want to be committed to fictional characters, we have two options: declare "Sherlock Holmes is a detective" false (or without truth value), by excluding Holmes from the domain; or revise the logic so that EG is not automatically valid for all terms. Free logic takes the second option.

In free logic, the domain can include objects that do not "exist" in the robust ontological sense, so an explicit existence predicate E! marks which domain members are genuine existents. The result:

- ∃x(Fx) does not mean "some *existing* thing is F" — it means "some thing in the domain is F."
- E!(a) means "a is among the genuinely existing things."
- The inference from Fa to ∃x(Fx) is valid only if we add E!(a) as a premise.

## Positive, Negative, and Neutral Free Logic

Three main varieties of free logic differ in their treatment of atomic sentences containing non-referring terms. *Positive free logic* (Karel Lambert, Edward Zalta) holds that atomic sentences containing empty names can be true: "Sherlock Holmes is a fictional detective" can be true even if Holmes does not exist. The rationale: some predicates are "outer-domain" predicates that apply to non-existing things. "Is fictional" applies to fictional characters; "is mythological" applies to mythological figures.

*Negative free logic* holds that atomic sentences containing empty names are all false: "Holmes is a detective" is false, as is "Holmes is not a detective." Truth requires reference; if the name fails to refer, the predication fails. This is a more austere semantics that avoids the positive-free-logician's commitment to properties of nonexistent objects.

*Neutral free logic* holds that atomic sentences containing empty names lack truth value — they are neither true nor false. Without a referent, there is no subject, and so the predication goes undefined. This follows Frege's treatment of sentences with non-referring descriptions.

Each variety has different consequences for the logic of intensional contexts and for the semantics of fiction and mathematics. The choice among them is not merely formal; it reflects substantive commitments about how language relates to non-existing objects.

## Free Logic and Meinongianism

Free logic provides a natural formal framework for Meinongian ontology. On a Meinongian free logic, the quantifier ∃ ranges over all objects — existing and non-existing alike — while the existence predicate E! picks out the existing ones. Terence Parsons (*Nonexistent Objects*, 1980) develops a positive free logic in this spirit: the "outer domain" consists of nonexistent objects; the "inner domain" consists of existing ones; nuclear predicates (ordinary properties like being golden, being a detective) can apply to objects in both domains.

Edward Zalta's abstract object theory (*Abstract Objects*, 1983) uses a positive free logic in a different way. Zalta distinguishes between *encoding* and *exemplifying* a property. Concrete objects exemplify properties — a red apple exemplifies redness. Abstract objects encode properties — they are constituted by the properties attributed to them in the theories or fictions that posit them. Sherlock Holmes encodes the property of being a detective (this is part of what it is to be Holmes) but does not exemplify it (he does not literally instantiate it in the world).

## What Free Logic Makes Visible

Free logic is formally neutral between Meinongianism and the Frege-Russell view — it provides a framework that both positions can use. The substantive ontological question remains: are the objects in the outer domain genuinely part of reality, or are they merely notational devices for capturing useful inferences about discourse that does not commit us to any special ontology?

What free logic does contribute is to make explicit the existence assumption that is normally built silently into classical reasoning. By forcing us to include E!(a) as an explicit premise whenever we want to generalize existentially from a statement about a, free logic makes our ontological commitments visible. This is philosophically valuable in itself: it structures the options and reveals that the choice between different accounts of existence and reference is not merely a formal choice but a substantive one with far-reaching consequences for how we understand fiction, theoretical posits, and the relationship between language and the world.
