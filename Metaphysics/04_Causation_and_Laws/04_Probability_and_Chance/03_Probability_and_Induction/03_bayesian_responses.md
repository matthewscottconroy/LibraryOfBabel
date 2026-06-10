# Bayesian Responses to the Problem of Induction

Bayesianism offers what many regard as the most productive framework for addressing the problem of induction — not by solving it in the classical sense of providing a deductive justification for inductive inference, but by providing a formal account of how rational degrees of belief should be structured and updated in the light of evidence. The framework is built on two commitments: that rational degrees of belief satisfy the axioms of probability theory, and that beliefs should be updated by conditionalization when evidence arrives.

## Bayes' Theorem

The theorem is mathematically straightforward — a consequence of the definition of conditional probability:

**P(H|E) = P(E|H) × P(H) / P(E)**

P(H) is the **prior probability** of hypothesis H — the probability assigned before considering evidence E. P(E|H) is the **likelihood** — the probability that evidence E would be observed if H were true. P(E) is the marginal probability of E, summed over all hypotheses. P(H|E) is the **posterior probability** — the updated probability after considering E.

The philosophical work is in the interpretation and application. The theorem itself is trivial; the question is what role it should play in rational inference and what constraints govern the choice of priors.

## The Bayesian Response to Hume

The response to Hume's problem proceeds as follows. We begin with prior probabilities over hypotheses about the world — including "all emeralds are green," "the sun will rise tomorrow," "copper conducts electricity." As we observe green emeralds, rising suns, and conducting copper, we update via Bayes' theorem. If H is "all copper conducts" and E is "this copper sample conducts," then P(E|H) is very high (if the law holds, of course each sample conducts), while P(E|¬H) is lower (if some copper does not conduct, each observed sample is less certain to conduct). The posterior P(H|E) therefore exceeds the prior P(H): each piece of positive evidence raises the probability of the generalization.

This does not prove that past regularities will continue — Bayesianism does not dissolve the logical gap Hume identified. But it shows how rational agents can and should increase their confidence in generalizations as evidence accumulates, and it makes precise what "being supported by evidence" means: hypothesis H is confirmed by evidence E iff P(H|E) > P(H). The Bayesian framework converts the problem of justifying induction into a problem of justifying prior probability assignments — a further problem, but a more tractable one.

## The Prior Probability Problem

The central challenge is the choice of priors. If prior probabilities are arbitrary, posteriors will be heavily influenced by the prior, especially with limited data. Different agents with radically different priors can be internally rational (coherent) while reaching very different conclusions from the same evidence.

**Subjective Bayesianism** (de Finetti, Savage) accepts that priors are subjective. The constraints on rationality are coherence — priors must satisfy the probability axioms — and conditionalization — updating on evidence via Bayes' theorem. No further constraints are imposed. The difficulty is that if priors are arbitrary, Bayesianism cannot explain why all rational agents should converge on similar scientific conclusions after observing the same evidence.

The convergence theorems partially address this. Under mild conditions (agents assign positive prior probability to the same hypotheses), Bayesian agents with different priors will converge to the same posteriors as evidence accumulates. In the limit of infinite evidence, all rational Bayesians agree. This provides a long-run justification of induction: rational inquiry converges on the truth probabilistically. But the long run may be very long, and convergence requires that agents agree about which hypotheses are even possible — which is itself a constraint that excludes certain prior distributions.

**Objective Bayesianism** (Jaynes, Williamson) argues that there is a uniquely rational prior, often derived from symmetry (the Principle of Indifference — assign equal probability to outcomes that are symmetric in all relevant respects) or maximum entropy principles (assign the prior that is as uninformative as possible given what you know). Priors are not arbitrary; they are determined by logical or epistemic principles. The difficulty is that the Principle of Indifference generates paradoxes: different ways of partitioning the outcome space yield different "uninformative" priors, and no principled way of choosing between the partitions is available without further assumptions.

## Dutch Book Arguments

The requirement that rational credences satisfy the probability axioms is grounded in Dutch Book arguments. A Dutch Book is a collection of bets that an agent is committed to accepting — by their stated degrees of belief — that guarantees a net loss regardless of outcomes. If an agent's beliefs violate the probability axioms (for example, P(H) + P(¬H) ≠ 1), a clever betmaker can construct such a collection. Rationality requires that beliefs cannot be exploited this way. Therefore, rational credences must be coherent.

The dynamic Dutch Book argument (Teller, Lewis) extends this to updating: an agent who updates beliefs by any procedure other than conditionalization is susceptible to a diachronic Dutch Book — a set of bets placed at different times that guarantee a net loss. These arguments provide a coherentist, pragmatic justification for the probability calculus as the logic of degrees of belief without addressing which particular credences (priors) are rational.

## Connection to Laws and Naturalness

The Bayesian framework connects to the metaphysics of laws in a way that illuminates both. If laws of nature have genuine modal force, they license more confident inductive inferences: we can assign high prior probability to "all copper conducts" because we know (or believe with good reason) that this is backed by a law of nature, and laws hold universally. On the Humean view, there are no laws in this strong sense — only regularities. The Humean Bayesian must therefore assign more cautious prior probabilities to universal generalizations, expecting slower convergence and lower posteriors from the same evidence.

This means that the choice between Humean and non-Humean metaphysics of laws has direct epistemological consequences: the two positions license different prior assignments and hence different inductive conclusions from the same evidence. Bayesianism is formally compatible with both metaphysics, but non-Humean accounts provide more resources for grounding confident inductive inference. Goodman's riddle is addressed within Bayesianism by prior assignment: we assign much lower prior to "all emeralds are grue" than to "all emeralds are green" because grue is not a natural predicate and does not figure in any law of nature. This connects the Bayesian approach to Lewis's natural properties and to the BSA: only natural predicates appear in laws, and only law-backed generalizations earn high prior probabilities.
