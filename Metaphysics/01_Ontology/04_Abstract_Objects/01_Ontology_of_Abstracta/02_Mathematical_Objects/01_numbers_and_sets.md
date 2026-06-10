# Numbers and Sets

*The fundamental objects of mathematics and debates about their ontological status.*

---

Numbers are the most discussed abstract objects in philosophy of mathematics. Natural numbers (0, 1, 2, ...) are the counting numbers; integers extend to negative values; rationals are ratios of integers; reals add the limit points of rational sequences; complex numbers extend to the square root of -1. Each level of the hierarchy generates new mathematical objects, and the question is what kind of ontological status these objects have.

Set theory, developed by Cantor and Frege and axiomatized by Zermelo and Fraenkel (ZF set theory), provides a foundational framework in which all standard mathematical objects can be defined. Numbers can be *identified* with sets: the natural number 0 with the empty set ∅, the number 1 with {∅}, the number 2 with {∅, {∅}}, and so on (this is one possible identification; there are others that work equally well). This suggests that numbers are not a distinct ontological category but reduce to sets.

But Benacerraf's famous argument "What Numbers Could Not Be" (1965) shows that any identification of numbers with sets is arbitrary. The von Neumann ordinals and the Zermelo ordinals are both equally good candidates for the natural numbers; there is no fact about which one numbers "really are." If numbers are identical with sets, there must be a fact of the matter, but there is none. Benacerraf concludes that numbers are not objects at all in the traditional sense — the number 3 is not a particular abstract object but the *third position* in any progression satisfying the Peano axioms. This is the structuralist conclusion.

The ontology of sets is somewhat less puzzling than the ontology of numbers, since sets are built from their members and their existence can be explained (on iterative accounts) in terms of the cumulative hierarchy built from individuals by repeated application of the set-forming operation. But the existence of the empty set, and the consistency of large cardinal axioms extending ZF, still raises deep questions about what it means for abstract mathematical objects to exist.

## The Ontological Status of Numbers

Three broad positions on the ontological status of numbers:

**Platonism**: Numbers are abstract objects that exist independently of human thought, language, and practice. The number 7 exists in the same sense that Mars exists — it is an objective feature of reality. Mathematical truths about numbers are necessary truths: they could not have been otherwise. On Frege's logicist version, numbers are extensions of concepts — the number 2 is the extension of the concept *concept with exactly two instances*. On contemporary Platonism (Shapiro, Balaguer), numbers are either positions in the natural number structure or members of a plenitudinous realm of abstract objects.

**Structuralism**: Numbers are not free-standing objects but positions in the natural number structure ⟨ω, 0, S⟩. There is no fact about what the number 3 *intrinsically is* — its identity is exhausted by its structural role: being the successor of 2, being the predecessor of 4, being prime, and so on. Different instantiations of the natural number structure (von Neumann ordinals, Zermelo ordinals, Dedekind progressions) are equally valid representations of the numbers; none has metaphysical priority.

**Anti-realism**: Numbers are fictions, constructions, or useful instruments without genuine ontological status. Fictionalists (Field, Balaguer in his anti-Platonist mood) hold that "7 is prime" is literally false — there is no object 7 — but that the mathematical fiction is warranted by its practical utility. Constructivists (Brouwer, Dummett) hold that mathematical objects exist only insofar as they are constructed by mathematical activity; the natural numbers exist because we can construct them iteratively, but uncountable infinities whose existence cannot be constructively demonstrated are more questionable.

## Benacerraf's Identification Argument

Benacerraf's argument against numerical Platonism is one of the most influential arguments in philosophy of mathematics:

**P1**: If numbers are abstract objects, then statements of numerical identity ("3 is the von Neumann ordinal {∅, {∅}, {∅, {∅}}}") are either true or false.

**P2**: Both the von Neumann identification and the Zermelo identification satisfy all the mathematical desiderata for the natural numbers — both make every true arithmetic sentence come out true.

**P3**: The von Neumann identification makes "3 ∈ 4" true; the Zermelo identification makes "3 ∈ 4" false. These cannot both be correct if numbers are identical with particular sets.

**P4**: There is no principled reason to prefer one set-theoretic identification over the other — both are equally natural from a mathematical standpoint.

**C**: Numbers are not particular abstract objects. Either they do not exist, or they are positions in a structure rather than intrinsically constituted objects.

The argument generalizes: any property that distinguishes the von Neumann from the Zermelo ordinals (such as membership facts among ordinals) is a property that the natural numbers themselves lack. Numbers have only arithmetic properties, and any progression satisfying the Peano axioms has those.

## The Iterative Conception of Sets

While numbers may resist identification with particular objects, sets appear more tractable. The *iterative conception* explains set existence through a metaphor of stages:

- **Stage 0**: The empty set ∅ exists (or urelements — non-set individuals — exist).
- **Stage α+1**: For any collection of sets formed at stages ≤ α, a set containing exactly those sets exists.
- **Stage λ** (limit): For any limit ordinal λ, the sets formed at all earlier stages exist.

This cumulative hierarchy V = ⋃_α V_α generates all the sets of standard set theory. The iterative conception gives a natural explanation of the ZF axioms and explains why Russell's paradox does not arise: the "set of all sets" is not formed at any stage because no stage contains all sets.

The iterative conception raises its own ontological questions. It is a *temporal* metaphor: sets are "formed" in stages as if by a process. But if sets are abstract, there is no genuine temporal process. The metaphor must be cashed out in non-temporal terms — perhaps in terms of ontological priority: sets at stage α+1 depend for their existence on sets at earlier stages. This is a grounding claim: sets are grounded in their members.

## Large Cardinals and Mathematical Ontology

Standard ZF set theory does not settle all mathematical questions. Large cardinal axioms — asserting the existence of inaccessible cardinals, Mahlo cardinals, measurable cardinals, supercompact cardinals — extend ZF in progressively strong ways, settling questions that ZF leaves open (including some questions about projective sets of real numbers).

Large cardinal axioms raise a sharp ontological question: what makes these axioms true or false? If there are mind-independent mathematical facts, then either there are large cardinals or there are not. Gödel held that mathematical intuition — a faculty that gives us direct access to the mathematical universe — could in principle settle such questions, just as sensory perception settles empirical questions. On this view, the truth of large cardinal axioms is a matter for mathematical investigation, not decision.

The alternative is that large cardinal axioms express *choices* about which mathematical universe to work in. On the multiverse view (Hamkins), there is no single absolute mathematical universe — there are many set-theoretic universes, some containing large cardinals and some not. Mathematical truth is then relative to a universe. This view is pluralist rather than Platonist and connects to structuralism: what is mathematically true depends on which structure is under investigation.
