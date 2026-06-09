# Section 5: Parameter Estimation and Model Validation

A mathematical model without numbers is not a model — it is a cartoon. The equations may be correct, the qualitative behavior may match observations, but until parameters are quantified and predictions are tested against data, the model is hypothesis rather than knowledge.

This section bridges the gap between model construction and data-driven validation. It is one of the most practically important sections in the chapter, because even correct models can be rendered useless by poor parameter estimation practice. And even well-fitted models can be scientifically misleading if the limitations of the fitting procedure are not understood.

The central themes of this section are three: the inverse problem, identifiability, and sensitivity.

**The inverse problem** (subsection 5.1) is the challenge of going from data to parameters. It is harder than the forward problem (parameters to predictions) for mathematical reasons: the mapping is nonlinear, multiple parameter sets may fit equally well, and experimental noise must be explicitly modeled. This subsection covers least-squares and Bayesian approaches to parameter estimation, the critical importance of noise models, and multi-start optimization to detect multiple local optima.

**Identifiability** (subsection 5.2) asks whether parameters can even in principle be determined from the available data. This is not a question of experimental precision — it is a mathematical property of the model and what is observable. Structural non-identifiability means that no amount of data can disambiguate certain parameter combinations; practical non-identifiability means that the data in hand are insufficient. Profile likelihood is the most reliable practical tool for assessing identifiability and computing uncertainty bounds.

**Sensitivity analysis** (subsection 5.3) asks which parameters most influence the model outputs. Local sensitivity (derivatives) and global sensitivity (Sobol indices) answer this question at different levels of generality. Sensitivity analysis connects parameter estimation to experimental design: high-sensitivity parameters are the ones most worth measuring precisely.

Together, these three tools constitute the minimum required practice for trustworthy biological modeling. A model presented without identifiability analysis and sensitivity analysis is a model whose claims cannot be verified. The framework in this section gives you the tools to verify — or refute — your own models and those of others.
