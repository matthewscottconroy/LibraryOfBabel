# Logic in Machine Learning

## Neuro-Symbolic AI

Traditional machine learning (neural networks, deep learning) excels at pattern recognition from data but struggles with:
- **Systematic generalization**: applying rules consistently to novel cases
- **Interpretability**: understanding why a model made a decision
- **Guarantees**: formal correctness properties for safety-critical applications
- **Data efficiency**: learning from few examples

Formal logic offers these properties but lacks the flexibility and data-driven learning of ML.

**Neuro-symbolic AI** combines both: neural networks for perception and pattern recognition, formal logic for reasoning and guarantees.

## Logic in Neural Network Verification

Deep neural networks used in safety-critical systems (autonomous vehicles, medical diagnosis) must be verified to satisfy safety properties:

**Adversarial robustness**: "For any input within $\epsilon$ of input $x$, the classification does not change." This is a formal property:
$$\forall x', \|x - x'\| < \epsilon \to f(x) = f(x')$$

**Verifying this requires reasoning about all inputs in a neighborhood** — essentially a quantified statement that SMT solvers can attack (with neural network encoded as a formula).

**Tools**: $\alpha,\beta$-CROWN, Marabou, ERAN — neural network verifiers using SMT, LP, and abstract interpretation.

## Probabilistic Logic and Statistical Relational Learning

**Markov Logic Networks (MLN)**: A combination of Markov random fields and first-order logic. Each FOL formula has a weight — the higher the weight, the more the formula constrains the probability distribution. Inference is performed via weighted model counting.

**DeepProbLog**: Extends probabilistic logic with neural predicates — predicates whose truth probabilities are given by neural networks. Combines deep learning with logical reasoning.

## Constraint Learning and ILP

**Inductive Logic Programming (ILP)**: Learn first-order rules from examples. Given positive and negative examples of a concept, ILP systems produce FOL rules that classify examples correctly. Applications: drug discovery (learning structure-activity relationships), natural language processing, game playing.

**Constraint Learning**: Learn explicit logical constraints from data. The learned constraints can be verified and interpreted — unlike neural network weights.

## Formal Specification of ML Systems

**Type systems for ML**: TensorFlow's type system tracks tensor shapes; dependent types could guarantee no shape mismatches.

**Fairness and bias**: Formal specifications of algorithmic fairness — "the model must not discriminate based on protected attributes" — can be verified formally or used to constrain training.

**Privacy**: Differential privacy is a mathematical guarantee ($\epsilon$-DP) about how much information a model reveals. Formal verification of differential privacy is an active research area.

## Exercises
See [problems/ch13_applications/](../../../problems/ch13_applications/)
