# Important Terms and Ideas: Causal Structure

**Structural equation models (SEMs)**: A formalism for representing causal relationships using equations that specify how the value of each variable is determined by others. Structural equations capture the asymmetric, productive aspect of causation.

**Directed acyclic graph (DAG)**: A graphical representation of causal structure: nodes are variables, arrows indicate direct causal influence, and the graph has no cycles. DAGs underlie the d-separation criterion for probabilistic independence.

**Intervention (do-operator)**: Pearl's formalization: an intervention on variable X sets its value exogenously, "cutting" the usual causal arrows into X. The effect of intervention can differ from the effect of mere conditioning (observation).

**D-separation**: A graphical criterion for reading off conditional independence from a DAG: X and Y are d-separated by Z iff all paths between X and Y are blocked by Z. D-separation entails (given causal Markov condition) that X and Y are conditionally independent given Z.

**Causal Markov condition**: The assumption that each variable in a causal model is independent of its non-effects, conditional on its direct causes. This bridges the causal and probabilistic structure of a model.

**Causal faithfulness**: The assumption that all conditional independencies in the probability distribution are explained by d-separation in the causal graph — no accidental cancellations of causal paths. Faithfulness enables causal discovery from observational data.

**Two concepts of causation (Hall)**: Hall distinguishes production (a cause brings its effect into existence via a continuous causal process) from dependence (the effect counterfactually depends on the cause). These can come apart in cases of redundant causation.
