# Causation and Probability: Deeper Connections

The relationship between causation and probability runs deeper than the probability-raising account suggests. Statistical dependencies in data constrain causal structure; causal structure constrains which statistical dependencies are genuine; and the very interpretation of probability connects to questions about causal ontology. We are dealing not with two theories that intersect occasionally but with frameworks that are, at a deep level, perspectives on the same underlying reality.

## Screening Off and the Causal Markov Condition

One deep connection is through the **Causal Markov Condition (CMC)**. In a causal Bayesian network, each variable is conditionally independent of its non-effects given its direct causes:

**CMC**: For any variable X in a causal structure G, X is conditionally independent of every variable that is not a descendant of X, given X's parents (direct causes) in G.

This captures the idea that causes "screen off" their effects from earlier causal ancestors: once we know the direct cause, knowing earlier antecedents provides no additional probabilistic information. In the structure Low Pressure → Barometer Falling and Low Pressure → Storm: the CMC implies that once we conditionalize on Low Pressure (the common cause), Barometer Falling and Storm are independent. This is the probabilistic signature of common causation, and it allows us to distinguish causal structure from mere correlation.

The **Faithfulness assumption** is the complement: no additional independence relations hold "by coincidence" — every independence relation in the data is explained by the causal structure. Together, CMC and Faithfulness allow identification of causal structure from observational data, up to a class of statistically equivalent structures (Markov equivalence class). Within this class, additional assumptions — temporal order, invariance across contexts, instrumental variables — can pin down a unique causal model. This gives a mathematically precise sense in which probability theory is a window into causal structure.

## Lewis's Principal Principle

A second connection concerns objective chance and rational belief. Lewis's **Principal Principle** connects objective chance to rational credences:

**PP**: For a rational agent with credence function Cr, if the agent knows that the objective chance of event E at time t is p, then Cr(E | Cht(E) = p) = p.

If I know the fair coin has a 50% chance of landing heads, I should believe heads with credence 0.5. But this raises deep questions about what "objective chance" is and how we have epistemic access to it. For Lewis, chances are part of the Best System: the probability function that, together with the categorical facts about the world, forms the best systematization of all actual frequencies. The Principal Principle then connects these theoretical posits to rational belief via a constitutive norm on credences.

## Cartwright on Capacities and Probability

Nancy Cartwright has argued that the connection between causation and probability requires appeal to **capacities or powers**: the reason smoking raises the probability of cancer is that smoking has a *capacity* to cause cancer — a real, context-independent tendency — and this capacity is what grounds the probabilistic relationship (*Nature's Capacities and Their Measurement*, 1989).

The argument:

- **P1**: Statistical correlations between C and E can be present or absent depending on the context (selection effects, interference, confounders).
- **P2**: A purely statistical account cannot distinguish genuine causal relations from context-dependent spurious correlations.
- **P3**: Only appeal to an underlying capacity — a stable, context-independent causal tendency — can ground the distinction.
- **C**: Causal claims are ultimately claims about capacities, not merely about probability-raising.

This suggests that causation and probability are both dependent on a more fundamental layer: the dispositional properties of the relevant systems.

## Propensity and Causation

The propensity theory of probability (Popper, Giere) holds that probabilities are primitive dispositions of physical setups — tendencies to produce outcomes with certain frequencies, or single-case tendencies. On this view, the connection between causation and probability is explained by their common grounding in propensities: a cause C has a propensity to produce effect E with probability p, and that propensity is a genuine feature of C's causal power. This gives a unified picture: propensities are causal powers of a probabilistic kind. The probability distribution over outcomes reflects the magnitudes of the propensities of the causal system.

## Information Theory and Causation

Recent work has explored connections between information theory and causal structure. Causes are "information sources" for their effects: knowing the cause reduces uncertainty about the effect. This can be made precise using mutual information: I(C; E) = H(E) - H(E|C). But mutual information is symmetric — I(C; E) = I(E; C) — so information alone does not determine causal direction. The causal direction is given by the structural equations: E = f(C, noise), which specifies C as the independent variable and E as the dependent one.

Algorithmic causal discovery methods (Additive Noise Models, Post-Nonlinear Models) exploit the asymmetry of causal mechanisms: if E = f(C) + noise, then in general P(C) and P(E|C) are "independent" in a formal sense that is violated in the reverse direction P(E) and P(C|E). This independence is the probabilistic signature of the correct causal direction — a tool for inferring causal direction from observational data alone.

## Integration of Approaches

The deepest lesson here is that causation and probability are not rival frameworks but complementary perspectives on the same underlying causal reality. The statistician's correlation reveals the presence of causal connections; the philosopher's counterfactual analysis articulates what those connections involve; the scientist's mechanistic investigation discloses how the connections are implemented; and the powers theorist's ontology grounds all of these in the dispositional natures of things. Understanding the full relationship between causation and probability requires integrating the statistical, counterfactual, mechanistic, and powers approaches — and resisting the temptation to reduce any one of these to any other.
