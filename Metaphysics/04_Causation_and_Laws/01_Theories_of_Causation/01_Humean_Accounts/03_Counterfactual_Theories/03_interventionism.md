# Interventionism as a Counterfactual Theory

Imagine two pieces of evidence that smoking causes lung cancer. In the first, we observe that smokers and cancer patients overlap extensively. In the second, we conduct a randomized trial — we actually assign some people to smoke and others not, then wait. Why does the second carry so much more evidential weight? The difference, Woodward argues, is that the second involves an intervention: we actually reach into the system and set a variable to a value, rather than merely observing what values it happens to take. James Woodward's interventionism, developed most fully in *Making Things Happen* (2003), takes this observation as the basis for a theory of causation.

Rather than asking "what would have happened if C had not occurred?", interventionism asks "what would happen if we intervened to set C to a different value?" Causal claims are about relationships that are invariant under interventions — relationships that would be preserved under experimental manipulation.

## What Is an Intervention?

An **intervention** on X with respect to Y is a causal process that changes the value of X, acts on X directly (not through a path that also directly affects Y), breaks any other causal connections into X (the intervention is "exogenous"), and has no effect on Y through any path that does not go through X.

Formally: an ideal intervention I on X is such that I is the sole cause of X's value in the intervened scenario, and I has no direct effect on Y except through X. This corresponds to Pearl's **do-operator**: do(X = x) sets X to value x by intervening, severing all arrows pointing into X in the causal graph. The resulting distribution P(Y | do(X = x)) represents what we would observe under the intervention.

The core interventionist argument:

- **P1**: X causes Y iff there is a possible intervention on X that would change Y (holding other things constant through the intervention).
- **P2**: A relationship between X and Y that is invariant under interventions on X describes a genuine causal connection.
- **P3**: Mere statistical correlations may not be invariant under interventions (confounded by common causes, selection effects, etc.).
- **C**: Only relationships invariant under ideal interventions capture genuine causal structure.

## Invariance as the Criterion

A key virtue of interventionism is its criterion of **invariance**: a genuine causal relationship holds stably — invariantly — under a range of interventions and background conditions. The relationship between aspirin and headache relief is causally real because it holds across different doses, different subjects, different biochemical implementations of the headache mechanism.

Consider the contrast: yellowed fingers correlate with lung cancer, but intervening to remove the yellowing (by wearing gloves) does not reduce cancer risk. The relationship evaporates under intervention because the yellowing is a common-effect indicator, not a cause. Smoking causes both; the relationship between yellowing and cancer is entirely explained by their common cause. Interventionism cleanly separates the genuine causes from the correlated non-causes by asking which relationships survive when we actively manipulate a variable.

Interventionism connects naturally to the framework of structural equations and directed acyclic graphs (DAGs). A structural equation system represents each variable as a function of its direct causes plus a noise term. An intervention on X replaces the structural equation for X with a constant, severing the causal connections into X and fixing its value. The do-calculus then provides rules for computing interventional distributions from observational distributions when given the causal graph.

## The Circularity Objection

The theory faces a central philosophical objection: the definition of an intervention is itself causal. An intervention on X must act on X directly, must not affect Y except through X, and must be "exogenous" to the system. All of these conditions are stated in causal terms.

Woodward accepts this. Interventionism is not a reductive analysis of causation. It is an **explication** of causal structure that illuminates the inter-relationships among causal concepts — direct cause, total cause, contributing cause, intervention — without reducing them all to non-causal terms. The goal is clarity about causal structure, not elimination of causal concepts. We might wish for more, but there is nothing wrong with a theory that illuminates its subject without eliminating it.

## Comparison with Lewis

Lewis's counterfactual analysis and Woodward's interventionism share the broad commitment that causation is counterfactual in character. But they differ in important ways. Lewis analyzes causation in terms of closeness of possible worlds — a metaphysical concept. Woodward analyzes it in terms of invariance under interventions — a scientific-practice concept. Lewis aims for reduction: an account that avoids primitive causal concepts. Woodward accepts that some causal concepts are primitive and aims for illumination. Woodward connects directly to scientific methodology (randomized trials, natural experiments, instrumental variables) in ways that Lewis's pure metaphysics does not. And interventionism handles levels of causation naturally — interventions can be defined at any level, from molecular to social — whereas Lewis's framework faces difficulties with higher-level causation.

Interventionism also has natural connections to agency and free will. The notion of an intervention is partly defined in terms of what an agent could do, suggesting that causation is conceptually connected to agency in a way that purely physical or Humean accounts do not acknowledge. Von Wright, Menzies, and Price have developed explicit "agency theories" of causation along these lines. Woodward's account is more modest but still reflects the practical, agent-centered importance of causal knowledge: we care about causes because knowing them tells us what to do.
