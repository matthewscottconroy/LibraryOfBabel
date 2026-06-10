# Quantified Modal Logic

Modal propositional logic lets us say that it is necessary that P, or possible that Q — where P and Q are complete propositions. But we often want to say something about specific individuals: that Socrates is necessarily human, that some particular object could have been otherwise. For that we need quantifiers, and combining quantifiers with modal operators generates a range of puzzles that do not arise in propositional modal logic. This is the domain of quantified modal logic (QML).

## The Syntax

QML combines first-order quantifiers — ∀ (for all) and ∃ (there exists) — with the modal operators □ and ◇. Formation rules yield formulas such as:

- Atomic: Fx, Rxy, x = y
- Classical compounds: ¬φ, φ ∧ ψ, ∀xφ, ∃xφ
- Modal compounds: □φ, ◇φ
- Complex combinations: □(∀x)Fx, (∀x)□Fx, ◇(∃x)Gx, (∃x)◇Gx

The last pair illustrates the philosophical richness immediately: □(∀x)Fx says it is necessary that everything is F; (∀x)□Fx says every individual is necessarily F. These have different truth conditions, and understanding the difference is essential for making sense of de re necessity.

## De Dicto and De Re

Consider "Necessarily, all humans are mortal": □(∀x)(Hx → Mx). The modal operator takes wide scope over the quantifier — this is de dicto modality, the necessity of the proposition as a whole. The claim might be true as a matter of biological law without each individual human being essentially mortal.

Contrast "For all humans, necessarily mortal": (∀x)(Hx → □Mx). Here the modal operator has narrow scope within the predication — this is de re modality, necessity attaching to the thing with respect to the property. Each particular human is such that they could not be immortal.

These can come apart. It might be necessarily true that all humans are mortal (de dicto) without each particular human being necessarily mortal (de re) — perhaps the biological generalization holds necessarily, while individual humans might metaphysically have been immortal. The de dicto/de re distinction, formalized in QML, underlies the Aristotelian distinction between the essential and the accidental, and it is the formal vehicle for Kripke's claims about individual essences.

## Domain Semantics

The semantics of QML requires specifying, for each world w, a domain D(w) — the set of individuals that exist at w. The accessibility relation R carries over from propositional modal logic. The question of how domains at different worlds are related is philosophically central. Three standard options:

**Constant domains**: D(w) = D for all w. Every individual exists at every world. This corresponds to the validity of the Barcan Formula (discussed in the next section).

**Expanding domains**: D(w) ⊆ D(v) whenever wRv. Domains can only grow as we move to accessible worlds. New objects can come into existence, but actual objects cannot fail to exist.

**Variable domains**: D(w) can be arbitrary for each w. Objects can exist at some worlds and not others, independently of the accessibility relation.

Variable domains are most natural for the actualist intuition that individuals exist contingently — Socrates exists in some worlds but not others. But they generate a complication: what happens to de re modal claims about individuals that do not exist at some worlds? If Socrates does not exist at world w, what is the truth value of "Socrates is essentially human" at w?

One approach uses free logic — a logic tolerating non-referring singular terms, on which "Socrates is human" is either false or gappy at worlds where Socrates doesn't exist. Another approach uses inner/outer domain semantics (Kripke, 1963): each world has an inner domain (existing individuals) and an outer domain (individuals existing at some world). Quantifiers range over the inner domain; singular terms can refer to outer-domain elements. This allows us to make claims about non-existents while distinguishing them from actual existents.

## The Necessity of Identity

QML is the natural framework for formally proving the necessity of identity — one of Kripke's central results. The argument:

- P1: ∀x∀y(x = y → (Fx ↔ Fy)) — Leibniz's Law (indiscernibility of identicals).
- P2: ∀x□(x = x) — every individual is necessarily self-identical.
- From P1 with F = □(· = a): a = b → (□(a = a) ↔ □(b = a))
- Since □(a = a) is true (from P2), and a = b (assume), we get □(b = a), i.e., □(a = b).

Therefore: a = b → □(a = b). If two things are identical, they are necessarily identical.

This is the formal backbone of Kripke's argument for the necessity of identity statements using rigid designators. It is valid in QML with constant-domain semantics and the assumption that names are rigid designators. The philosophical implication: co-referential rigid names stand in a necessary identity, not a contingent correlation. If pain = C-fiber firing, this identity — if true — is necessarily true.

## QML and Historical Development

QML was first seriously developed by Ruth Barcan Marcus in the 1940s, whose system included the formula that bears her name. Kripke's variable-domain semantics (1963) gave the standard model theory. Philosophical applications exploded after *Naming and Necessity* (1980) provided the semantic and metaphysical tools for discussing essence, natural kinds, and identity through the QML framework.

Timothy Williamson's *Modal Logic as Metaphysics* (2013) defends "necessitism" — the view that necessarily everything necessarily exists, in a sufficiently thin sense — arguing for constant-domain QML as the correct logic for metaphysical modality. On Williamson's view, the debates about domains and existence are settled in favor of constant domains, with the concession that "existing" can be thin (as an abstract object) even for individuals that are not concrete at a given world. This is a bold and contested position, but it illustrates the extent to which formal choices in QML carry genuine metaphysical weight.
