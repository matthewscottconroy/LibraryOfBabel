# Properties of Grounding

Most accounts of grounding agree on a core set of formal properties, though the details are contested. Understanding these properties is important both for using the concept precisely and for evaluating whether the hierarchical picture of reality that grounding supports is coherent.

*Irreflexivity*: Nothing grounds itself — ¬(A < A) for any fact A. This seems intuitively clear: nothing is more fundamental than itself; no fact explains itself. It is also required for the notion of fundamentality to function: an entity is fundamental iff it is not grounded in any other entity. If grounding were reflexive, every entity would be grounded in itself, and nothing would be fundamental in the required sense.

*Asymmetry*: If A < B, then ¬(B < A). Grounding is a one-directional priority relation. The physical grounds the mental; the mental does not ground the physical. Asymmetry prevents circular explanations — explanatory loops in which A explains B and B explains A mutually.

*Transitivity*: If A < B and B < C, then A < C. This is what allows for a genuine hierarchy: physical facts ground chemical facts, chemical facts ground biological facts, biological facts ground psychological facts, psychological facts ground social facts — and by transitivity, physical facts ground social facts directly. Without transitivity, the grounding hierarchy would consist of disconnected links rather than an ordered structure.

*Necessitation* (contested): Many philosophers hold that if A grounds B, then □(A → B) — if A obtains, B must obtain. Arguments for: grounding is explanatory, and a full explanation of B by A requires that given A, B cannot fail. If B could fail to obtain given A, A does not fully explain B. Arguments against: Correia and Skiles (2017) argue that grounding can hold contingently. If so, grounding more closely resembles causation and less resembles conceptual entailment — which may or may not be desirable depending on the role we want grounding to play.

The debate about necessitation connects directly to the relationship between grounding and essence: if grounding is explained by essence (Fine's view), and essence generates necessary truths, then grounding should necessitate. If grounding is independent of essence, it might be contingent.

## Challenges to the Standard Properties

Each property faces at least one serious challenge. We should take these challenges seriously not because the standard picture is wrong but because they reveal where the real work lies.

Asymmetry faces the challenge of *mutual constitution*: social facts seem constituted by individual facts (individual beliefs and dispositions ground social institutions), but social facts can also ground individual facts (norms and institutions ground what certain individual actions are obligatory). Fine's response is to distinguish different grounding relations — different kinds of constitution that operate at different levels — and to note that asymmetry holds within each type of grounding relation even if the overall picture is complex.

Transitivity faces the *Trump objection* (Schaffer, 2012). A royal flush trumps a full house not because of its individual cards but because of its type. If we ground the truth "the hand is a royal flush" through "this hand has cards A, K, Q, J, 10 of spades," and then ground the intermediate fact through individual card facts, transitivity gives us: the individual card facts ground "royal flush." But what grounds "royal flush" is the fact that this arrangement constitutes a royal flush in the game of poker — not merely the card facts. Transitivity seems to give the wrong result. Fine's response: the Trump objection may show that grounding must be relativized to a context or level, and transitivity holds within a level but may fail across levels when intermediate grounds involve level-specific features.

Non-monotonicity — the possibility that adding facts to the grounding base can undermine existing grounding relations — deserves mention. "Someone promised to do X" grounds "they ought to do X." But "doing X would cause grave harm" may undercut this grounding. This is analogous to defeasible reasoning, and it suggests that a full grounding claim may need to specify that no defeating conditions are present: A, given that C₁, C₂, ..., Cₙ do not obtain, grounds B. Non-monotonicity is more natural for partial than for full grounding.

## Formal Systems

Recent work has developed formal systems that codify these properties precisely. Fine (2012) develops an "operational" theory of grounding, distinguishing strict ground (A strictly grounds B iff A ≠ B and A grounds B), weak ground (A ≤ B allows A = B), and immediate ground. Correia (2010) develops a related framework. A key formal result: in any system where grounding is irreflexive, asymmetric, and transitive, the grounding relation generates a strict partial order on facts — a hierarchy with no loops and no reflexive edges. The question of whether this partial order is well-founded (whether grounding chains always terminate) is the question of foundationalism versus infinitism, examined in the next section.
