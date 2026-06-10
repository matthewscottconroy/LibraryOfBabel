# Mathematical Structuralism

*The view that mathematics is the science of abstract structures rather than particular objects.*

---

Mathematical structuralism holds that mathematical objects are *positions* in structures rather than independently existing individuals. Numbers, points, functions, and groups are not free-floating Platonic objects; they are defined by their role in an abstract structure — by their relations to other objects in the structure. The number 3 is not any particular thing; it is whatever occupies the third position in any progression satisfying the Peano axioms.

This view elegantly handles Benacerraf's identification problem. Since multiple set-theoretic constructions can serve as the natural numbers, there is no fact about which sets are "really" the numbers. The structuralist response: there need be no fact, because numbers just *are* positions in the natural number structure, not particular objects. Any two progressions (with zero and a successor function satisfying the Peano axioms) are equally valid instantiations of the natural number structure.

Structuralism comes in several varieties. *In re* structuralism (Hellman): structures are not abstract objects but *possibilities* — possible patterns of relations. Mathematical truths are conditionals about what would hold in any system satisfying the structural axioms. *Ante rem* structuralism (Shapiro, Resnik): structures are genuinely abstract objects that exist independently of any particular instantiation. The natural number structure exists even if no concrete progression exists. The positions within the structure are genuine objects (thin, defined only by their structural roles) rather than nothing at all.

A persistent challenge for structuralism is the *incompleteness of structure-objects*. If the number 3 is only defined by its position in the natural number structure, then it has no properties beyond structural ones — there is no fact about whether 3 is a subset of 7, for instance (since that is a set-theoretic question, not a structural one). Some philosophers find this acceptable (mathematical objects are genuinely *thin*); others worry it conflicts with mathematical practice.

Structuralism connects to broader debates about whether objects in any domain — physical objects, persons, mathematical objects — have irreducible individual essences or are better understood as nodes in structural networks. It thus links philosophy of mathematics to structural realism in philosophy of science and to debates about identity and individuation more generally.

## The Varieties of Mathematical Structuralism

**Ante rem structuralism** (Shapiro, *Philosophy of Mathematics: Structure and Ontology*, 1997; Resnik, *Mathematics as a Science of Patterns*, 1997): Structures exist as abstract objects independently of any concrete or mathematical instantiation. The natural number structure ⟨ω, 0, S⟩ — a set with a distinguished initial element and a successor function — exists necessarily, even if no concrete progression exists to instantiate it. Mathematical objects are positions within such structures, with no intrinsic nature beyond their structural role.

Shapiro's ante rem structuralism combines Platonism about structures with a structuralist account of mathematical objects. The structures are abstract Platonic objects; the objects within them (numbers, points, functions) are positions in those structures. This avoids the Benacerraf identification problem while maintaining that mathematics is genuinely about something.

**In re structuralism** (Parsons, Pettigrew): Structures are instantiated by concrete or mathematical systems; there is no structure without an instance. The natural number structure exists only if some progression exists — whether the von Neumann ordinals in ZF set theory, or some concrete sequence of physical objects. Mathematical truths are truths that hold in any adequate instantiation of the relevant structure.

*Challenge*: In re structuralism faces an *ontological insufficiency* problem. If mathematical structures exist only if instantiated, and if the natural number structure can only be instantiated by infinitely many objects, then the truth of arithmetic depends on the existence of infinitely many concrete objects — a contingent fact, if it is a fact at all.

**Modal structuralism** (Hellman, *Mathematics Without Numbers*, 1989): Mathematical truths are translated into modal claims — claims about what would hold in any *possible* system satisfying the relevant axioms. An arithmetic sentence φ is true iff □∀X(PA²(X) → φ^X), where PA² are the Peano axioms in second-order logic and φ^X is φ relativized to X. This eliminates quantification over abstract structures: mathematical truths are modal conditionals, not truths about particular abstract objects.

## The Identity of Indiscernibles Problem

Shapiro's ante rem structuralism faces a serious challenge from the *Identity of Indiscernibles* (IndId): things that are structurally indiscernible — that have exactly the same structural properties and relations — should be identical. But some mathematical structures contain genuinely distinct positions that are structurally indiscernible.

**The example**: Consider the complex number field ℂ. The numbers i and −i are distinct (i ≠ −i), yet they satisfy exactly the same structural relations within ℂ — both are square roots of −1, both have the same absolute value, and any structural statement true of i is also true of −i (by the automorphism that maps each complex number to its conjugate).

If mathematical objects are nothing but positions in structures, and if i and −i occupy symmetric positions with identical structural properties, then by IndId, i = −i. But this is mathematically false.

Shapiro's response: mathematical objects are *places* in structures, and places are individuated partly by their being *different places*, not purely by their structural properties. The structure of ℂ contains two square roots of −1 as a matter of the structure's own nature; the individuation is structural (they are two square roots, not one) even if not discriminated by structural predicates. This response has been contested: it seems to introduce primitive individuation into an account that was supposed to eliminate it.

Keränen's version of the objection: if structures are genuinely abstract objects with multiple elements, the elements must be individuated by something — but structural properties alone cannot do the job for symmetric structures. Structuralism either requires primitive individuation (undermining the structuralist program) or cannot account for symmetric structures.

## Structuralism and Mathematical Practice

Mathematical structuralism aligns well with several features of mathematical practice:

**Abstraction and generality**: Mathematicians routinely work with structures defined abstractly — groups, rings, fields, topological spaces, categories — rather than with particular instantiating objects. Number theory is about the natural number structure; topology is about topological structures. The "objects" studied are the structures themselves.

**Isomorphism invariance**: Mathematical properties are always isomorphism-invariant. If two structures are isomorphic, every structural property that holds of one holds of the other. This is exactly what structuralism predicts: structural properties are the only properties mathematical objects have, and structural properties are isomorphism-invariant.

**Multiple realizations**: The same mathematical structure can be realized in many different ways — the natural numbers can be realized by sets, by strokes on a page, by computer memory states, by physical objects arranged in a sequence. This multiple realizability is natural on a structuralist account: what matters is the pattern, not the particular realization.

**Abstraction from content**: When mathematicians move from arithmetic to algebra, they abstract away from the particular features of numbers to the structural features (associativity, distributivity, etc.) that are preserved under algebraic operations. Structuralism makes this a metaphysical point, not just a methodological one: algebraic structures are the subject matter, and numbers are merely one instantiation.

## Structuralism and Categorical Foundations

Category theory provides a formalism that is structuralist in spirit. Categories are collections of objects and morphisms (structure-preserving maps between them) satisfying composition laws. A category is characterized entirely by its morphisms; the "objects" have no intrinsic properties beyond their roles in morphisms.

On a category-theoretic foundation for mathematics, mathematical structures are characterized up to isomorphism by their universal properties — by the morphisms into and out of them. The natural numbers ℕ are characterized as the initial commutative monoid: the unique (up to isomorphism) commutative monoid from which there is a unique monoid homomorphism into any other commutative monoid. This is a structural characterization: the natural numbers are individuated by their structural role among all commutative monoids.

The category-theoretic perspective supports structuralism by showing that objects within categories have no intrinsic properties that survive categorical equivalence. Two categories that are equivalent — connected by an adjoint equivalence of functors — have all the same categorical properties; no category-theoretic property distinguishes them. This suggests that categorical objects have only structural properties, as structuralists claim.
