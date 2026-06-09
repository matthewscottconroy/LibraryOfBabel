# The Problem of Priors

Prior probabilities — the credences an agent holds before observing evidence — are the starting point for Bayesian inference. Conditionalization determines how to update; but it does not determine where to start. The problem of the priors is the question of what constraints, if any, govern the rational choice of prior probabilities.

## Why Priors Matter

In Bayesian inference, the posterior is a function of both the prior and the likelihood: P(H|E) = P(E|H) × P(H) / P(E). If two agents have very different priors, they can reach very different posteriors even when they observe the same evidence. With unlimited evidence, posteriors will converge (the "Bayesian swamping" of priors). But in practice, agents must make decisions with limited evidence, and the priors play a significant role.

In forensic, medical, and policy contexts, the choice of prior can be decisive. A Bayesian analysis of the probability of guilt, given DNA evidence, depends critically on the prior probability of guilt (the base rate). Different choices of prior lead to different verdicts.

## Objective Bayesianism: Constraints on Priors

Objective Bayesians hold that there are rational constraints on priors beyond coherence. The principle of indifference is the most intuitive: if you have no reason to prefer one outcome over another, assign them equal probability.

**The Principle of Indifference** (Laplace): In the absence of any relevant difference between outcomes, assign equal probability to each. If there are n mutually exclusive, exhaustive, and equally likely outcomes, each has probability 1/n.

The principle faces the *reference class problem*: the partition of outcomes is not unique. Suppose I have an urn with unknown proportions of black and white balls. Should I assign equal probability to "black" and "white" (1/2 each)? Or should I assign equal probability to each proportion from 0% to 100% black (a uniform distribution over proportions)? These give different results for the probability that the first ball drawn is black.

**Maximum Entropy** (Jaynes): The rational prior is the one that maximizes *entropy* — the measure of uncertainty — subject to any known constraints. This is the most "uninformative" prior that incorporates everything you know. Jaynes argued this is the objective Bayesian prior.

**Jeffreys priors**: Harold Jeffreys developed a family of "uninformative" priors that are invariant under reparametrization — they don't depend on how you choose to represent the problem.

## Subjective Bayesianism

Subjective (or "personalist") Bayesians hold that any coherent prior is rationally permissible. There is no further constraint beyond coherence on what you should believe before seeing evidence. The only rational requirement is internal consistency.

This view has the advantage of permissiveness: it doesn't require agreement on priors, and it allows the representation of genuinely different initial perspectives. But it has been criticized for being too permissive: it seems to permit almost any prior, including priors that are wildly inconsistent with background knowledge.

## Calibrated Priors

A more pragmatic approach focuses on *calibration*: priors are good if, when used in Bayesian updating, they lead to well-calibrated posteriors — posteriors that match the actual frequencies of outcomes. A prior is calibrated if the credences it generates track truth rates: events you assign probability 0.7 happen about 70% of the time.

This connects to the frequentist notion of probability and to empirical questions about the reliability of various prior-setting methods. It's a normative-empirical hybrid: the criterion for good priors is their track record in producing accurate beliefs.

## The Reference Class Problem

The reference class problem plagues both frequentist and Bayesian approaches to probability. When assigning a probability to a particular event (this patient's recovery, this stock's performance), what reference class should we use? The same event can be described as a member of many different reference classes, which may have different base rates.

There is no purely formal answer to the reference class problem — it requires judgment about which reference class is most *appropriate* given the practical context. This introduces an ineliminable element of practical reason into probability assessments.

## Sleeping Beauty and Imprecise Probabilities

Some decision theorists argue that the problem of priors motivates moving to *imprecise* probabilities: instead of a single probability distribution, rational agents can hold *sets* of probability distributions, representing genuine uncertainty about the prior. Decisions are made by maximizing expected utility according to any distribution in the set.

This view, sometimes called *Knightian uncertainty* (after Frank Knight's distinction between risk and uncertainty), represents genuine epistemic humility about probabilities — acknowledging that we don't always know how to assign precise credences.
