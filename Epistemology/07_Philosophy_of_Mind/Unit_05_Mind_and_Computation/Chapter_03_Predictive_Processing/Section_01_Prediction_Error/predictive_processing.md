# Predictive Processing and the Bayesian Brain

Predictive processing is a theoretical framework that has rapidly become one of the dominant approaches in cognitive science and philosophy of mind. The core idea: the brain is a prediction machine that constantly generates predictions about its sensory inputs and updates these predictions when they are violated. Perception, cognition, and action are all manifestations of a single, unified process of hierarchical prediction and error minimization.

## The Generative Model

The brain maintains a *generative model* of the world — a model that generates predictions about what sensory inputs should be if the world is a certain way. The model is hierarchical: higher levels generate predictions about lower levels, and these cascade down through the processing hierarchy.

When sensory input arrives, it is compared to the prediction. *Prediction error* — the mismatch between prediction and input — is propagated back up the hierarchy, updating the model. The goal is to minimize prediction error: to have predictions that accurately anticipate sensory inputs.

On this framework:
- **Perception** is the brain's best guess about the causes of its sensory inputs, given its generative model.
- **Action** is a way of reducing prediction error by changing the sensory input to match predictions, rather than updating the model.
- **Learning** is updating the generative model to improve predictions over time.
- **Attention** modulates the precision assigned to different prediction errors — attending to a stimulus increases the weight given to prediction errors in that domain.

## The Bayesian Framework

Predictive processing is typically cast in Bayesian terms. The brain maintains probability distributions over hypotheses about the causes of sensory input (*priors*) and updates these distributions when sensory evidence (*likelihoods*) arrives, yielding *posteriors*. The generative model implements approximate Bayesian inference.

Karl Friston's *free energy principle* is the mathematical backbone: the brain minimizes *free energy* — a measure of the divergence between the brain's model of the world and the actual probability distribution of sensory inputs. Minimizing free energy is equivalent to minimizing prediction error (under certain approximations).

This connects the predictive processing account to information theory and statistical mechanics, giving it a principled mathematical foundation.

## Hallucination, Perception, and the Bayesian Brain

A striking implication: perception is "controlled hallucination" (Anil Seth's term). We do not passively receive information from the world; we actively construct our experience by projecting a generative model onto sensory inputs. The sensory inputs don't determine our experience directly; they refine a prediction.

This explains various perceptual phenomena:
- **Multistable perception** (Necker cube, duck-rabbit): The model "flips" between two equally good hypotheses.
- **Perceptual filling-in**: We "see" things in the blind spot or in peripheral vision because the model fills in predictions.
- **Priming and context effects**: Prior expectations (priors) strongly influence perception.
- **Hallucinations in psychosis**: Overly strong priors that dominate sensory input, so the brain "sees" what it expects even without corresponding sensory evidence.

## Action and Active Inference

Friston's active inference framework treats action and perception as two ways of resolving prediction error. Perception updates the model; action changes the world to match the model. When you reach for a cup, your motor system generates predictions about the proprioceptive sensations that will result from reaching, and the motor system then acts to bring those sensations about.

This unifies perception and action under a single principle: minimize the divergence between model and world.

## Philosophical Implications

Predictive processing has significant philosophical implications:

**Representationalism**: The framework is thoroughly representationalist — the brain represents the world via its generative model. This places predictive processing in tension with enactivism and direct realism.

**Consciousness**: Anil Seth argues that consciousness is the predictive model run in the context of interoception (sensing the body's internal states). The self is a "controlled hallucination" — a predictive model of the embodied agent.

**Top-down and bottom-up integration**: The framework dissolves the traditional distinction between perception (bottom-up) and cognition (top-down) — all processing involves both prediction (top-down) and error correction (bottom-up).

**Mental disorders**: Predictive processing has been applied to depression (overly strong priors dominating evidence), autism (under-weighting of priors, leading to sensory overwhelm), and schizophrenia (attribution of self-generated signals as external causes).

Predictive processing is perhaps the most ambitious unified theory of mind in contemporary cognitive science, though debates about its scope, formalization, and relationship to consciousness remain active.
