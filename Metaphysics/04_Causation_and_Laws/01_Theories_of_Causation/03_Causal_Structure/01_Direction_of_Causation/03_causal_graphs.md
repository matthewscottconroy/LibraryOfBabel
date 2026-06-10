# Causal Graphs and the Structure of Causation

How do we infer, from a matrix of correlations in observational data, what the causal structure underlying those correlations is? The answer, developed by Judea Pearl, Peter Spirtes, Clark Glymour, Richard Scheines, and others over the past four decades, makes essential use of directed acyclic graphs (DAGs) — causal graphs. These have become one of the most important tools in both philosophy of causation and statistical causal inference, providing a formal language for expressing, testing, and discovering causal structure.

## Basic Definitions

In a causal graph, **nodes** represent variables (events, properties, states). **Directed edges** (arrows) represent direct causal relationships: an arrow from X to Y means X is a direct cause of Y, relative to the other variables in the graph. **Acyclicity** encodes the assumption that causation is non-circular — you cannot follow directed edges from a node back to itself.

The direct causes of a variable X are its "parents" in the graph, written PA_X. A **path** between two nodes is any sequence of connected edges, regardless of direction. Paths can be blocked by "colliders" (nodes where two arrowheads meet) or "conditioners" (variables we condition on), which determines conditional independence relationships through the d-separation criterion.

## D-Separation

The fundamental tool for reasoning about causal graphs is the **d-separation criterion**, which identifies which pairs of variables are conditionally independent given a set of conditioning variables.

A path between X and Y is **blocked** by a set of variables Z iff either the path contains a non-collider that is in Z, or the path contains a collider that is not in Z and has no descendants in Z. X and Y are **d-separated** by Z iff every path between them is blocked by Z. If X and Y are d-separated by Z in the causal graph G, then X and Y are conditionally independent given Z in any distribution faithful to G: X ⊥ Y | Z.

Notice the striking implication: conditioning on a collider can open a previously closed path — the phenomenon known as "explaining away" or Berkson's paradox. In the graph Low Pressure → Barometer and Low Pressure → Storm, Barometer and Storm are d-separated given Low Pressure. But if we condition on Storm (a downstream collider's ancestor), Barometer and Weather Forecast can become d-connected. This counterintuitive behavior is a precise prediction that can be tested against data.

## The Do-Calculus

Pearl's **do-calculus** provides rules for computing the effects of interventions from observational data. The key operation is **do(X = x)**: setting X to x by intervention, which removes all arrows pointing into X (bypassing X's natural causes) and fixes X's value. The resulting distribution P(Y | do(X = x)) is what we would observe in an experiment where X is externally forced to x.

Three rules of the do-calculus:

**Rule 1** (Insertion/deletion of observations): P(y | do(x), z, w) = P(y | do(x), w) if Y ⊥ Z | X, W in the modified graph where all arrows into X are deleted.

**Rule 2** (Action/observation exchange): P(y | do(x), do(z), w) = P(y | do(x), z, w) if Y ⊥ Z | X, W in the graph where arrows into Z are also deleted.

**Rule 3** (Insertion/deletion of actions): P(y | do(x), do(z), w) = P(y | do(x), w) if Y ⊥ Z | X, W in the graph where arrows into X are deleted and arrows into Z(W) are deleted, where Z(W) is the set of Z-nodes not ancestral to W.

These rules allow us to identify when and how interventional distributions P(Y | do(X = x)) can be computed from observational data P(Y | X = x) — the key step in causal inference from observational studies.

## Causal Discovery from Observational Data

Consider observing Smoking (S), Tar Deposits (T), Lung Cancer (C), and Yellowed Fingers (Y). We observe: S correlates with C, T, and Y; T correlates with C; Y does not correlate with C once we condition on S.

The d-separation analysis suggests: S → T → C (tar deposits mediate the smoking-cancer link); S → Y (smoking causes yellowed fingers); Y ⊥ C | S (yellowed fingers and cancer are independent given smoking — they share only the common cause S).

Crucially: intervening on Y (preventing yellowed fingers by wearing gloves) does not change C, because Y is not a cause of C. The do-calculus confirms: P(C | do(Y = 0)) = P(C) ≠ P(C | Y = 0) in general. The graph predicts this; algorithms like the PC algorithm can recover this graph from the pattern of conditional independences in observational data.

## Philosophical Questions About Causal Graphs

Are directed edges objective features of the world, or representational choices reflecting our interests? Realist philosophers (Pearl, Woodward) hold that causal graphs represent real causal structure. Epistemic approaches hold that they represent the best model given our knowledge. The acyclicity assumption rules out feedback loops; many real systems involve feedback, which requires either time-indexed variables or accepting cyclic causal graphs. And causal graphs represent causation at a chosen level of description — different choices yield different graphs, and there is no single "correct" causal graph for a complex system.

Process theorists raise a different objection: causal graphs represent causal relationships among variables, abstracting from the physical processes that implement them. This abstraction loses something metaphysically important — the physical transmission of quantities along causal pathways. Graphs identify *that* X causes Y; processes explain *how*. Whether this is a limitation of the formalism or an acceptable idealization is one of the central ongoing debates in the philosophy of causation.
