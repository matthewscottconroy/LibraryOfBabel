# Euclid's Elements as a Formal System

Euclid's *Elements* (c. 300 BCE) is the prototype of every formal system in this book: thirteen books of geometry and number theory developed from explicitly stated first principles by chains of deduction, each proposition citing only postulates, common notions, and previously proved propositions. For over two thousand years it was the exemplar of rigor itself. Its hidden gaps — discovered only when nineteenth-century logic caught up with it — taught mathematics what an axiom system really is, and what it costs to have one.

## Definitions, Common Notions, Postulates

Book I opens with three kinds of first principles.

**Definitions** (23 of them): "A point is that which has no part"; "a line is breadthless length"; "a straight line is a line which lies evenly with the points on itself." To a modern eye these are not definitions at all: they do not reduce the defined terms to previously understood ones, and no proof in the *Elements* ever appeals to them. They are elucidations of intended meaning. In practice, *point*, *line*, and *circle* function as **primitives** — exactly the role Hilbert would later make official.

**Common notions** (5): topic-neutral principles of magnitude. CN1: "Things which are equal to the same thing are also equal to one another." CN5: "The whole is greater than the part." These are, in effect, the logical and algebraic axioms of the system.

**Postulates** (5): the specifically geometric assumptions.

## The Five Postulates

1. To draw a straight line from any point to any point.
2. To produce a finite straight line continuously in a straight line.
3. To describe a circle with any center and radius.
4. That all right angles are equal to one another.
5. That, if a straight line falling on two straight lines make the interior angles on the same side less than two right angles, the two straight lines, if produced indefinitely, meet on that side on which are the angles less than the two right angles.

The fifth is quoted verbatim (in Heath's translation) because its form is the point. Postulates 1–3 license constructions; Postulate 4 asserts a simple uniformity of space; Postulate 5 is a conditional with a consequent about arbitrarily distant intersections — it reads like a theorem that failed to find its proof. Its standard modern replacement is:

**Playfair's axiom.** Through a point not on a given line there is at most one line parallel to the given line.

Over the remaining axioms (more precisely, over *neutral geometry*: the incidence, order, and congruence axioms of the coming sections), Postulate 5 and Playfair's axiom are equivalent, and texts swap them freely. Euclid himself seems to have distrusted his postulate: he proves the first twenty-eight propositions without it, deferring its first use to I.29. The suspicion that it was a disguised theorem drove two millennia of attempted proofs — the subject of the next section.

## Worked Example: Proposition I.1 as a Derivation

**Proposition I.1.** *On a given finite straight line, to construct an equilateral triangle.*

Here is Euclid's proof, arranged as a derivation with explicit justifications. Given: a segment $AB$.

| Step | Assertion | Justification |
|------|-----------|---------------|
| 1 | Let $C_1$ be the circle with center $A$ and radius $AB$ | Postulate 3 |
| 2 | Let $C_2$ be the circle with center $B$ and radius $BA$ | Postulate 3 |
| 3 | Let $C$ be a point in which $C_1$ and $C_2$ intersect | **??** |
| 4 | Draw segments $CA$ and $CB$ | Postulate 1 |
| 5 | $AC = AB$ | Definition 15 (radii of $C_1$) |
| 6 | $BC = BA$ | Definition 15 (radii of $C_2$) |
| 7 | $AC = BC$ | CN1, from 5 and 6 |
| 8 | Triangle $ABC$ is equilateral | Definition 20, from 5–7 |

$\square$ — except for step 3.

## The Gap: Continuity Is Missing

Nothing in the postulates guarantees that the two circles meet. Euclid reads the intersection point off the diagram. That this is a genuine *logical* gap — not pedantry — is shown by a countermodel. Interpret "point" as a point of the rational plane $\mathbb{Q}^2$, "line" as the set of rational solutions of a linear equation, "circle" as the set of rational points at a given (realizable) distance from a center. The postulates survive this reinterpretation, but with $A = (0,0)$ and $B = (1,0)$ the circles of Proposition I.1 would have to meet at $(1/2, \pm\sqrt{3}/2)$ — and those are not rational points. In $\mathbb{Q}^2$ the circles pass through each other without intersecting, and Proposition I.1 fails.

So step 3 needs a **continuity principle** that Euclid never states, e.g. circle–circle intersection: *if a circle has one point inside and one point outside another circle, the two circles meet.* (In I.1 this applies: $B$ lies on $C_1$ and inside $C_2$, while the point of $C_1$ diametrically opposite $B$ lies outside $C_2$.) Such principles follow from a Dedekind-style completeness axiom — exactly what Hilbert's Group V later supplies. The gap is invisible in any drawing, because drawn circles visibly cross; it lives only in *unintended models*, and the concept of an unintended model did not exist before the nineteenth century. That is why the gap took twenty-two hundred years to find.

## More Gaps: Order and Superposition

**Pasch's axiom.** Euclid has no axioms of *order*: no primitive relation of one point lying between two others, and nothing governing the two sides of a line. Moritz Pasch (1882), in the first rigorous treatment of geometric order, isolated a missing principle: *a line that enters a triangle through one side must leave it through another side (or a vertex)*. Nothing of the sort is derivable from Euclid's postulates, yet Book I constantly uses such facts ("the point falls within the angle…"). The classic fake proof that every triangle is isosceles works precisely by drawing a diagram with a betweenness error that no Euclidean axiom rules out.

**Superposition.** Proposition I.4 (side–angle–side congruence, SAS) is "proved" by picking triangle $ABC$ up and *applying* it to triangle $DEF$, observing that the parts coincide. No postulate licenses moving a figure, and nothing defines what "applying" means. Euclid seems uneasy about the maneuver himself — he uses it only when unavoidable (I.4, I.8, III.24). Hilbert's later diagnosis: SAS is not provable from the rest at all; his system takes it, in essence, as congruence axiom III.6.

## Why the Gaps Went Unnoticed: Diagrams as Implicit Axioms

For two millennia these were not experienced as gaps, because Euclidean practice was never text alone: the diagram was a *regulated* component of proof. Kenneth Manders' analysis of the Euclidean diagram distinguishes **co-exact** attributes — those stable under perturbation of the drawing, such as intersection, containment, and betweenness — from **exact** attributes, such as equality of segments, which any perturbation destroys. Euclid systematically reads only co-exact facts from the diagram (step 3 above is one) and argues for all exact claims in the text. Avigad, Dean, and Mumma (2009) turned this discipline into a formal system, **E**, whose inference rules license precisely the co-exact diagrammatic moves; derivations in E track Euclid's actual proofs proposition by proposition, and E is sound and complete for the ruler-and-compass fragment of elementary geometry. On this reconstruction the *Elements* is not a defective formal system but a different, diagram-inclusive one — its "missing" axioms encoded in drawing practice rather than sentences.

## Material versus Formal Axiomatics

The deepest lesson concerns what an axiom *is*. For Euclid, axioms were **material**: true statements about a unique subject matter — space — admitted because self-evident. Under material axiomatics, drawing on the meaning of "point" and "line" (and on the diagram) is legitimate, since the axioms merely record salient truths about things already understood. For Hilbert, axioms are **formal**: uninterpreted sentence-forms that implicitly define their primitives, and a proof is correct only if it goes through under *every* interpretation of the primitives. Only from the formal standpoint is "the circles must intersect" a gap at all — a gap *is* a step that fails in some model of the stated axioms. The rest of this chapter is the story of that standpoint's emergence: the parallel postulate forced it (Section 2), Hilbert codified it (Section 3), Tarski pushed it to completeness and decidability (Section 4), and proof assistants mechanized it (Section 5).

## Exercises
See [problems/ch20_geometry_and_logic/](../../../problems/ch20_geometry_and_logic/)
