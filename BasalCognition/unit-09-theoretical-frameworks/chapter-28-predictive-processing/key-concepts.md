# Key Concepts: Chapter 28 — Predictive Processing and the Free Energy Principle

---

## Predictive Processing (PP)

A theoretical framework in cognitive science and neuroscience proposing that the brain (and by extension, other biological systems) operates as a hierarchical prediction machine. Rather than passively processing incoming sensory signals, the brain continuously generates top-down predictions about what sensory input should look like, and only the discrepancy between prediction and actual input — the *prediction error* — is propagated upward through the neural hierarchy. Perceptual inference is the process of minimizing prediction error by updating the top-down predictions; learning is the process of adjusting the model to reduce prediction errors over longer timescales. Proposed in modern form by Rao and Ballard (1999) for visual cortex, and developed philosophically by Clark, Hohwy, and others.

---

## Prediction Error

The discrepancy between a top-down prediction and the actual bottom-up sensory signal at any level of the neural hierarchy. Prediction errors are the driving force of the predictive processing framework: they signal that the current model is failing to account for some feature of the input, and they propagate upward through the hierarchy to trigger model updates. The magnitude of a prediction error does not alone determine its influence; it is weighted by *precision* — the estimated reliability of the signal. High-precision prediction errors have large effects on model updating; low-precision errors have small effects. The concept of prediction error has empirical correlates in neural activity: burst firing in certain neural populations has been proposed to implement prediction error signals.

---

## Active Inference

The extension of predictive processing from perceptual inference to action. In active inference, the organism minimizes prediction error not only by updating its internal model (perception) but also by acting on the world to make the world conform to its predictions (action). Goals and desires are understood as predictions about desirable future states; action is the process of fulfilling those predictions by changing the organism's relationship to the environment. This reconceptualization dissolves the traditional distinction between perception and action, treating both as manifestations of a single free energy minimization imperative. Active inference also accounts for exploratory behavior (epistemic foraging) as the minimization of expected future surprise by seeking information about uncertain states.

---

## Free Energy (Variational Free Energy)

In Friston's framework, free energy is an information-theoretic quantity that provides an upper bound on the *surprise* of an organism's sensory signals given its generative model. Minimizing free energy ensures that sensory signals remain unsurprising — consistent with what the organism's model predicts — which is equivalent to maintaining the organism in its expected (viable) states. Technically, variational free energy is the sum of the "energy" term (how well the model fits the data) and a "complexity" penalty (how much the posterior model has deviated from the prior). Minimizing free energy with respect to internal states performs variational Bayesian inference, providing the connection between FEP and Bayesian brain theories. The term is borrowed from statistical physics but used in a specifically information-theoretic sense.

---

## Markov Blanket

In statistical graphical models, a node's Markov blanket is the minimal set of nodes that, when conditioned on, renders the node statistically independent of all other nodes. In Friston's FEP framework, the Markov blanket is used to define the statistical boundary of a self-organizing biological system: the set of states that mediates all interaction between the system's internal states and its external environment. For a cell, the Markov blanket is approximately the membrane and its embedded proteins. The concept is used to argue that any system with a Markov blanket can be formally described as implementing variational Bayesian inference on external states — that the existence of a statistical boundary is sufficient to ground the FEP's application to the system. The use of this concept has attracted criticism for conflating a statistical notion (a feature of our models) with an ontological one (a feature of the systems themselves).

---

## Surprise (Information-Theoretic)

In Friston's framework and in information theory generally, the "surprise" of an observation is the negative logarithm of its probability under a model: −log p(observation | model). High-probability observations generate little surprise; low-probability observations generate much surprise. The free energy principle proposes that living systems must minimize the long-run average surprise of their sensory signals — that is, they must exist in states that are consistent with their survival, because states that are highly surprising relative to a viable organism's model are states in which the organism is probably dying. Surprise in this technical sense is related to, but not identical with, the psychological sense of being startled by unexpected events. Also called "self-information."

---

## Generative Model

In the predictive processing framework, a generative model is a probabilistic model of how sensory signals are generated by external causes. The brain (or any system implementing the FEP) maintains a generative model that specifies the prior probability of external states and the likelihood of sensory signals given those states. The model "generates" predictions about what sensory input should look like given any hypothesized state of the world, and discrepancies between these predictions and actual input are the prediction errors that drive model updating. For unicellular organisms, the "generative model" is not an explicit computational object but is implicitly encoded in the organism's signaling architecture: the structure of the chemotaxis network encodes what chemical environments a healthy bacterium should inhabit.

---

## Precision-Weighting

In the predictive processing framework, precision is the reciprocal of variance — a measure of the reliability or signal-to-noise ratio of a prediction error signal. Prediction errors are not all equally influential: high-precision errors (from reliable, consistent sources) strongly influence model updating, while low-precision errors (from noisy, variable sources) have little influence. The brain regulates precision dynamically, estimating it top-down and using it to weight incoming error signals. This precision-weighting mechanism is proposed as the neural substrate of selective attention: attending to a stimulus is equivalent to estimating high precision for the prediction errors coming from that stimulus. Dysregulation of precision-weighting is proposed as a mechanism for various psychiatric symptoms, including hallucinations (excessive precision of internal predictions) and anxiety (excessive precision of threatening predictions).

---

## Epistemic Foraging

The behavior of seeking out information — reducing uncertainty about the state of the world — as a form of active inference. Under the FEP, minimizing expected future free energy requires not only minimizing current prediction error but also reducing the uncertainty that would generate large future errors. This motivates actively seeking informative states, even when no immediate reward is available. Epistemic foraging corresponds roughly to curiosity and exploratory behavior: the organism is drawn toward states that resolve uncertainty about its environment. The concept has been applied to explain why organisms explore novel environments, why humans seek knowledge for its own sake, and why infants preferentially attend to surprising stimuli. Whether unicellular organisms exhibit epistemic foraging in the relevant sense — whether their apparent exploration is driven by uncertainty reduction or by simpler gradient-following — is an open empirical question.

---

## Unconscious Inference

Helmholtz's term for the rapid, automatic, non-conscious inferential processes by which the brain constructs perceptions from sensory signals. Helmholtz argued that perception cannot be the direct apprehension of external objects but must involve an inference from sensory data to the probable external cause of that data — an inference that proceeds too rapidly and automatically to enter consciousness. The concept anticipates modern Bayesian and predictive processing accounts of perception and explains why perceptual illusions arise: they are cases where the brain's inferential process — based on typically accurate priors — leads to a wrong conclusion about the current stimulus. The term is historically important but has largely been superseded by more precise formulations in terms of Bayesian inference and prediction error minimization.
