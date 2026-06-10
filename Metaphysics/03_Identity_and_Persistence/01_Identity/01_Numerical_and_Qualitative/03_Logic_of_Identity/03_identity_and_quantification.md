# Identity and Quantification

*The interaction between identity statements and quantificational reasoning.*

---

How many dogs are in the room? The question seems simple. But answering it requires more than looking and counting — it requires knowing what makes two dog-sightings sightings of the *same* dog rather than different dogs. The criteria for numerical identity determine when we have one entity and when we have two. Ontological questions ("how many things are there?") are not purely matters of neutral observation but are shaped by our choice of identity criteria. Different criteria yield different counts: how many objects are in this room? If we count fundamental particles, the number is enormous; if we count ordinary medium-sized dry goods, it is small; if we count temporal stages, it depends on our persistence conditions. The number we report depends on which sortals and which identity criteria we apply. This connection between counting and identity is philosophically significant and underlies debates from Geach's relative identity to Quine's ontological relativity.

## Frege's Puzzle from the Quantificational Side

The quantificational perspective sheds new light on why "a = b" can be informative. When we learn "Hesperus = Phosphorus," we learn that the existential claim "there is something that is both Hesperus and Phosphorus" is satisfied by a *single* object. This is genuine information because we previously might have believed we were dealing with two objects:

> Before: ∃x∃y(Hx ∧ Py ∧ ¬(x = y)) — two distinct objects, one Hesperus and one Phosphorus.
> After: ∃x(Hx ∧ Px) ∧ ∀x∀y((Hx ∧ Py) → x = y) — one object that is both.

The quantificational content of the identity statement is genuinely new information about how many things there are, not merely about how names relate to things. The informativeness of identity is, at bottom, about revising our count of the world's furniture.

## Identity in Modal Logic

Modal logic complicates identity reasoning in ways that connect formal semantics to substantive metaphysics. We can prove, in S5 (or weaker modal systems), that if a = b and the names are rigid designators, then □(a = b):

- P1. a = b. [assumption]
- P2. a = a. [reflexivity]
- P3. □(a = a). [the necessity of self-identity: ∀x□(x = x)]
- P4. a = b → (□(a = a) → □(a = b)). [by Leibniz's Law applied to □(a = a)]
- C. □(a = b). [by P1, P4, P3]

This is Kripke's argument for the necessity of true identity statements between rigid designators. The logical machinery connects directly to the metaphysics of modality: if identity holds at all, it holds necessarily. Contingent identity, properly understood, is not really identity in the strict sense — as Kripke shows, what looks like contingency in identity statements is an epistemic phenomenon, a matter of how we access the identical object, not a feature of the identity relation itself.

In variable domain modal semantics, the quantifier at each possible world ranges only over the individuals existing in that world. Identity across worlds then requires additional resources — haecceities, counterpart relations, or primitive transworld identity. In constant domain semantics, all individuals exist in every world, and identity is simply the standard identity relation. The choice between these semantics tracks the debate between Kripkean transworld identity and Lewisian counterpart theory.

The Barcan Formula (∀xφ(x) → □∀xφ(x)) and its converse have implications for whether the same individuals exist across possible worlds. If the Barcan Formula holds, then whatever exists necessarily exists, in the sense that quantification over possible worlds and over actual objects commutes. If individuals can exist in some worlds but not others, the Barcan Formula fails. These are not merely technical questions: they determine whether our modal logic is adequate to the metaphysics of contingent and necessary existence.

## Actualism, Possibilism, and Identity

The scope of the quantifiers interacts with the theory of possible worlds in a further way. Actualists hold that only actual individuals exist — there are no merely possible individuals. Possibilists hold that possible but non-actual individuals exist (as abstract or concrete entities). For the actualist, ∀x ranges only over actual existents. To say "Sherlock Holmes could have existed" requires either that "Holmes" refers to an actual abstract individual (the character in Doyle's stories) or that the sentence is analyzed without existential commitment. For the Lewisian possibilist, ∀x ranges over all individuals in all possible worlds; identity is the standard identity relation holding within worlds, while counterpart relations substitute between worlds.

## Sortal Quantification and Ontological Relativity

Quine's thesis of ontological relativity argues that the question "what exists?" is always relative to a background language or theory that fixes the identity criteria for the entities posited. There is no view from nowhere — no language-independent fact about what the quantifiers range over. Different theories with different identity criteria posit different ontologies, and there is no neutral fact of the matter between them. This Quinean relativity is in tension with the view that there are mind-independent facts about numerical identity and diversity. If ontological relativity is true, then questions about how many things there are — and which things are identical — are theory-relative rather than absolute.

The logic of identity is thus not merely a formal matter but opens directly onto some of the deepest questions in metaphysics and philosophy of language. The formal apparatus — numerical quantification, modal logic, variable-domain semantics — is the systematic expression of genuinely philosophical commitments about what there is and how it is individuated.
