# Modal Systems: S4, S5, and Their Differences

Choosing a modal system is not a purely formal decision. Each system embodies a theory of what necessity and possibility are — how stable modal truths are, whether modal truths are themselves necessarily true, and whether there are inaccessible pockets of possibility. The differences between K, T, S4, S5, and B are philosophically significant.

## K: The Minimal Base

System K is the weakest modal logic, requiring only the distribution axiom: □(P → Q) → (□P → □Q). This captures the minimal inferential behavior of any necessity operator without presupposing anything about the structure of the accessibility relation. Crucially, K does not validate □P → P. A K-necessity operator might be satisfied even when the proposition is false at the world of evaluation — which means it does not represent genuine necessity in any metaphysically serious sense. What K captures is the bare logical behavior shared by all operators that distribute over modus ponens. K is sound and complete with respect to all Kripke frames (frames with unrestricted accessibility), making it the logic valid over the most general class of models.

## T: Reflexivity and Real Necessity

T adds the reflexivity axiom: □P → P. Necessary truth implies truth. Whatever is necessarily so, is so. This is a minimal requirement for any metaphysical notion of necessity: if P must be the case, then P is the case. T corresponds semantically to reflexive frames — every world can see itself: wRw for all w.

Epistemically, T captures the factivity of knowledge: □ₑP → P (what is known is true). Deontic logic typically does not validate T, since the actual world may fail to satisfy all obligations. The system appropriate for a given modality depends on whether the relevant "necessity" must be actual — and for genuine metaphysical necessity, it must.

## S4: Transitivity and Iterated Modality

S4 (T + 4) adds the transitivity axiom: □P → □□P. If P is necessary, it is necessarily necessary. The necessities are stable under iteration: if something must be true, then it must be true that it must be true. Semantically, 4 corresponds to a transitive accessibility relation — worlds accessible from accessible worlds are themselves accessible.

A key theorem of S4: □P ↔ □□P. Once we have necessity, iterating the box adds nothing. This collapse of iterated necessity reflects the transitivity of accessibility. S4 is appropriate for epistemic logic if one accepts the positive introspection principle (KK): if one knows P, one knows that one knows P. Not all epistemologists accept this, but those who do are committed to S4 or stronger.

## S5: Full Equivalence and Metaphysical Modality

S5 (T + 5) adds the Euclidean property, which combined with reflexivity and transitivity yields a full equivalence relation. In S5, all possible worlds are mutually accessible: ◇P → □◇P (if P is possible, it is necessarily possible). What is possible is possible necessarily; what is necessary is necessary necessarily. Modal truth in S5 is absolute — invariant across all worlds.

The characteristic S5 equivalences deserve emphasis:

- □P ↔ □□P (necessity of necessity)
- ◇P ↔ ◇◇P (possibility of possibility)
- ◇□P → □P (if possibly necessarily P, then necessarily P)
- □◇P ↔ ◇P (necessarily possibly P just iff possibly P)

The third formula is the key step in the modal ontological argument: if possibly necessarily a God exists (◇□∃xGx), then necessarily a God exists (□∃xGx). This inference is valid in S5 but not in weaker systems like S4, where possible necessity does not collapse into necessity. S5 is the system assumed in most contemporary metaphysical discussions, including Lewis's modal realism and Plantinga's modal ontology.

## B: Symmetry Without Transitivity

B (T + B axiom) corresponds to reflexive, symmetric frames. The B axiom — P → □◇P: if P is true, it is necessarily possible — captures the intuition that the actual world is always among the accessible worlds. Whatever is actually the case, it could not have been impossible. B is stronger than T but is incomparable with S4: it has symmetry without full transitivity. Some philosophers have argued that B better captures certain modalities, particularly when one doubts that possibility is "absolute" in the S5 sense but accepts that actuality implies possibility.

## The Choice of System as Metaphysical Commitment

The choice among modal systems reflects substantive commitments. S5 fits a picture where modal truths are fixed features of reality — the same from any possible perspective. What is possible is possible necessarily; there are no "local" possibilities accessible from one world but not from another. This fits both modal realism (Lewis: concrete worlds all mutually accessible) and modal actualism (Plantinga: abstract states of affairs in an S5 structure).

S4 allows iterated necessity to collapse but does not require full symmetry. Some modalities — epistemic, perhaps temporal — may validate 4 without 5. Weaker systems are appropriate for modalities where accessibility is genuinely asymmetric or non-Euclidean.

Timothy Williamson (*Modal Logic as Metaphysics*) defends S5 for absolute metaphysical modality while noting that different kinds of necessity — logical, metaphysical, natural — may require different systems. The formal diversity of modal systems is not a defect of modal logic; it is a feature, allowing us to represent the genuine diversity of modal concepts we employ.
