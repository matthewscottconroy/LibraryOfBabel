# Section 6: Model-Building Philosophy

By the time you reach this section, you will have learned how to write ODEs from biochemical reactions, analyze stability and bifurcations, simulate stochastic dynamics, model spatial patterns, estimate parameters, and assess identifiability. These are powerful technical tools. But tools do not guarantee good science. That requires judgment.

This section is about judgment — the principles and practices that distinguish modeling that generates genuine biological knowledge from modeling that generates well-fitting equations. It draws on decades of accumulated experience in the systems biology community about what works, what fails, and why.

**The modeling cycle** (subsection 6.1) frames mathematical modeling as iterative inquiry rather than one-time translation. The six stages — define question, hypothesize mechanism, translate to equations, analyze, compare to data, revise — describe the actual process of productive modeling. Each stage has characteristic failure modes that prevent the cycle from advancing.

**Principles for good models** (subsection 6.2) articulates six principles that should govern all biological model construction: start simple, require every parameter to be estimable, test for robustness, distinguish fitting from explaining, report uncertainty, and share models in standard formats. These are not stylistic preferences; they are the lessons of what makes models scientifically useful versus scientifically misleading.

**Common mistakes** (subsection 6.3) catalogs the eight most common failure modes in biological modeling, with mechanisms, consequences, and remedies. Over-parameterization, identifiability neglect, unstated assumptions, unit errors, mechanism-correlation confusion, inappropriate deterministic models, training-validation conflation, and the single-trajectory trap are all here — and all are far more common in the published literature than they should be.

Reading this section with the earlier technical material fresh will crystallize why the principles matter: you will recognize exactly which technical tools each principle invokes, and why violating each principle leads to a specific, diagnosable failure. The goal is not to make you cautious and conservative in your modeling — it is to make you rigorous and productive. Good principles do not constrain scientific imagination; they channel it.
