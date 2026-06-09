# Bayesian Updating and Conditionalization

The heart of Bayesian epistemology is a rule for updating beliefs in response to evidence: *conditionalization*. When a rational agent learns that E is true, they should update all their credences by conditionalizing on E. This is the Bayesian account of rational belief revision — the normative core of the Bayesian framework.

## Conditionalization

Bayes' theorem is a mathematical truth of probability theory:

P(H|E) = P(E|H) × P(H) / P(E)

In epistemological terms: your posterior credence in hypothesis H given evidence E equals the likelihood of E given H, times your prior credence in H, divided by your prior credence in E.

The *rule of conditionalization* says: when you learn E (and only E — all your new information is E), your new credence in any proposition H should be your old conditional credence cr(H|E):

cr_new(H) = cr_old(H|E) = cr_old(H ∧ E) / cr_old(E)

This rule is the Bayesian account of learning from evidence. The posterior is determined by the prior and the likelihood; there is no other rational response to evidence.

## Bayes' Theorem in Action

Consider updating on medical test results. A disease affects 1% of the population (prior probability 0.01). A test is 95% accurate: it returns positive 95% of the time when the disease is present, and 5% of the time when the disease is absent.

Prior to the test: cr(disease) = 0.01.
You test positive. What is your posterior credence?

P(positive | disease) = 0.95
P(positive | no disease) = 0.05
P(positive) = P(positive|disease)×P(disease) + P(positive|no disease)×P(no disease)
            = 0.95×0.01 + 0.05×0.99 = 0.0095 + 0.0495 = 0.059

P(disease | positive) = 0.95×0.01 / 0.059 ≈ 0.161

Despite a positive test result, your posterior credence is only about 16%. This is *base-rate neglect* made precise: a positive test is strong evidence, but the disease's rarity strongly constrains the posterior.

## Jeffrey Conditionalization

Standard conditionalization handles the case where you learn E with certainty. But what about evidence that is itself uncertain? Richard Jeffrey extended conditionalization to handle soft evidence.

Suppose you observe something in poor lighting that might be a red tomato or a red ball. You don't become certain of either; you become more confident of "red object" and change your credences over several hypotheses. Jeffrey conditionalization handles this by conditioning on a partition of hypotheses according to your new credences over those hypotheses.

## Dutch Book Arguments for Conditionalization

Just as Dutch Book arguments support coherence, *diachronic* Dutch Book arguments support conditionalization. If your updating rule differs from conditionalization — if you plan to update your credences in some other way when evidence arrives — a clever bookie can construct a series of bets that guarantee you a net loss, made at different times.

This argument is controversial: it assumes that you plan your updating strategy in advance and that the bookie knows your plan. But it provides a pragmatic justification for conditionalization as the rational updating rule.

## The Problem of the Priors

Conditionalization tells you how to update from a prior, but it doesn't tell you what prior to start with. Different priors can lead to very different posteriors, even after extensive evidence. The problem of the priors is one of the central challenges for Bayesianism.

**Objective Bayesianism** (Jaynes, Williamson): There are objective rational constraints on priors. Indifference or maximum entropy principles specify what prior to use in the absence of information.

**Subjective Bayesianism** (Ramsey, de Finetti, Jeffrey): Any coherent prior is rationally permissible. There is no objective fact about what prior to hold. The only rational constraint is coherence. Disagreement between rational agents is always permissible if they have different priors.

**Convergence to the truth**: Even if agents start with different priors, if they conditionalize on the same evidence, their posteriors will converge over time — given sufficient evidence, prior differences are "washed out." This provides some comfort: the subjectivity of priors is temporary; the objectivity of evidence eventually dominates.
