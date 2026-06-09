# Credences and Degrees of Belief

Classical epistemology works with a binary notion of belief: either you believe a proposition or you don't. But ordinary thought and inquiry seem to involve more fine-grained attitudes — you might be fairly confident it will rain today, very confident that 2+2=4, and quite uncertain about who will win the next election. Formal epistemology, particularly Bayesian epistemology, models these graded attitudes using *credences* — degrees of belief — formalized as probabilities.

## The Credence Concept

A credence is a degree of belief — a numerical measure of how confident you are that a proposition is true. Credences range from 0 (complete certainty that the proposition is false) to 1 (complete certainty that it is true), with intermediate values representing intermediate confidence.

This is not merely a technical convenience. Many epistemologists argue that credences are the psychologically and normatively fundamental doxastic states — that "full belief" in the binary sense is either derivative from credences (perhaps belief is credence above some threshold) or is itself a limiting case of credences.

The credence framework connects to decision theory: rational action is action that maximizes expected utility, where expected utility is calculated using credences as probability weights over outcomes.

## Coherence and the Probability Axioms

The most fundamental requirement on credences is *coherence*: credences should satisfy the axioms of probability theory.

The Kolmogorov axioms (informally):
1. Credences are non-negative: cr(P) ≥ 0 for all P
2. Credences in tautologies are 1: cr(⊤) = 1
3. Credences are finitely additive: cr(P ∨ Q) = cr(P) + cr(Q) when P and Q are mutually exclusive

Why should credences obey these axioms? The standard argument is the *Dutch Book argument* (Ramsey, de Finetti). If your credences violate the probability axioms, a clever bookie can construct a series of bets that you will each regard as fair (or even favorable) but that together guarantee you a net loss — a "Dutch Book." Since rational agents shouldn't accept guaranteed losses, rational credences must be coherent.

An agent whose credences are coherent is safe from Dutch Books, but coherence alone is a weak constraint. It rules out glaring inconsistencies but permits many different prior probability distributions.

## Conditional Credences and Independence

*Conditional credence* cr(P|E) represents how confident you are in P, given that E is true. The standard definition: cr(P|E) = cr(P ∧ E) / cr(E), provided cr(E) > 0.

Conditional credences play a crucial role in learning from evidence: when you observe E, your new unconditional credence in P should be your old conditional credence in P given E. This is *conditionalization* (or Bayesian updating), discussed in the next section.

Two propositions P and Q are *probabilistically independent* if cr(P|Q) = cr(P) — learning Q doesn't change your confidence in P. Independence is a model of genuine evidential irrelevance.

## The Principal Principle

Lewis's *Principal Principle* connects credences to objective chances (physical probabilities). If you know that the objective chance of P is x, and you have no "inadmissible" information (information about P's truth that doesn't come through its chance), then your credence in P should be x.

This principle connects Bayesian epistemology (about subjective degrees of belief) to the philosophy of probability (about objective chances). It is a rationality constraint: rational credences should "track" objective chances when we know them.

## Beyond Binary Belief

The move from binary belief to credences has implications for classical epistemological debates:

- **Knowledge**: If knowledge requires certainty (credence 1), very little is known. If knowledge requires credence above some threshold, the lottery problem arises (why doesn't high credence in lottery-losing amount to knowledge?).
- **Justification**: Degrees of justification correspond naturally to degrees of credence.
- **The regress argument**: In the credence framework, beliefs don't require support from other beliefs in a linear chain; instead, the entire probability distribution is evaluated for coherence.

The credence framework provides tools for rigorously modeling epistemic states that binary accounts struggle with, but it raises its own questions about the nature, rationality, and epistemological significance of credences.
