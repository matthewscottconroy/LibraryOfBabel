# Section 28.1: Predictive Processing

## Helmholtz's Unconscious Inference

Hermann von Helmholtz was among the most productive scientists of the nineteenth century — his contributions spanned thermodynamics, optics, acoustics, and electrodynamics. But one of his most lasting contributions was a philosophical observation about the nature of perception that he embedded in his *Treatise on Physiological Optics* (1867): the claim that perception is not the direct apprehension of the world but is mediated by what he called *unconscious inference*.

The observation began with a puzzle that anyone who has thought carefully about vision encounters: the retina receives a two-dimensional pattern of light intensity values, yet we perceive a rich three-dimensional world of objects with determinate shapes, distances, and colors. How? The information in the retinal image radically underdetermines the three-dimensional scene that caused it — infinitely many different scenes could produce any given retinal image. The brain must somehow choose among these possibilities, and Helmholtz argued that it does so by inference: by using accumulated experience to infer the most likely external cause of the current sensory signal.

This inference is unconscious: we are not aware of performing it, and it proceeds automatically and rapidly. We experience only its product — the perceived world. But Helmholtz's point was that the perceived world is a *hypothesis* — the brain's best guess about what is causing current sensory signals, constructed on the basis of prior experience with the world and the likelihood of various configurations. Illusions, on this account, are not failures of perception but cases where the brain's prior experience leads it to the wrong hypothesis.

Helmholtz's insight was too far ahead of its time to be fully developed in the nineteenth century. The computational tools needed to formalize it did not exist. But the idea persisted in various forms — in the Gestalt psychologists' principle of "good figure," in James Gibson's contrasting (and ultimately less successful) attempt to show that perception is direct rather than inferential, and in the information-theoretic discussions of perception in the 1950s and 1960s.

## Bayesian Brain and Hierarchical Prediction

The contemporary formalization of Helmholtz's idea draws on Bayesian probability theory and on computational approaches to neural coding. In its clearest statement, the **Bayesian brain hypothesis** proposes that the brain represents the world probabilistically — not as a fixed representation of what is out there, but as a probability distribution over possible states of the world — and updates that distribution in response to sensory input using Bayes' theorem (Knill & Pouget, 2004).

Bayes' theorem tells us how to update a prior probability distribution (what we believed before seeing the data) in light of new evidence (sensory input) to produce a posterior distribution (what we should believe after seeing the data). The brain, on the Bayesian account, maintains prior distributions based on past experience, and sensory input serves as evidence that updates these priors. The percept — what we experience — is something like the peak of the posterior distribution: the most likely cause of the current sensory signal, given prior experience.

This framework was given its most influential neural implementation by Rao and Ballard (1999) in a foundational paper on predictive coding in the visual cortex. Their proposal was that the hierarchical organization of the visual cortex implements a kind of predictive feedback: higher-level areas send predictions down to lower-level areas, lower-level areas compute prediction errors (discrepancies between predicted and actual input), and those error signals are sent upward. Learning consists of adjusting the predictions to reduce the error signals over time.

### Hierarchical Predictive Processing

The full hierarchical predictive processing (HPP) framework, elaborated most thoroughly by Andy Clark and Jakob Hohwy among philosophers and by Karl Friston in theoretical neuroscience, proposes that this prediction-error architecture operates across all levels of the neural hierarchy simultaneously (Clark, 2016).

At each level of the hierarchy, there are two kinds of units:

- **Prediction units** (sometimes called "representation units"): these encode the system's current best hypothesis about the causes of signals at the level below. They send top-down predictions downward.

- **Error units**: these compare the top-down predictions with the actual bottom-up signal and compute the discrepancy. They send prediction errors upward.

Learning and perceptual inference both consist of minimizing the prediction errors propagating up the hierarchy. The brain's goal, on this account, is to become a system whose top-down predictions account for the bottom-up sensory input so well that the error signals are minimized. A perfectly adapted brain (an idealized limit never achieved in practice) would be one whose model of the world is so accurate that no discrepancy appears at any level of the hierarchy.

This sounds like it might make the brain completely unresponsive to the world — a solipsistic predictor generating a fantasy world of its own. The crucial corrective is that prediction errors, when they do occur, propagate up through the hierarchy and trigger updates in the predictions. The brain is not generating a fixed fantasy; it is generating a continuously updated best hypothesis that is revised whenever the evidence demands it.

The degree to which prediction errors are trusted and acted upon is regulated by what the framework calls *precision*: a measure of the reliability or expected accuracy of a signal. High-precision errors are highly influential — they trigger strong updates in the predictions. Low-precision errors are given less weight. Precision is itself predicted by the system (estimated top-down) and represents the brain's estimate of how much to trust different sources of information. This precision-weighting mechanism is proposed as the neural substrate of attention.

### What the Framework Explains

The hierarchical predictive processing framework has been applied with some success to a striking range of phenomena:

**Perceptual illusions** are explained as cases where the brain's prior predictions override weak or ambiguous sensory evidence. The hollow-face illusion — in which a hollow mask appears as a convex face because the brain's strong prior expectation of convex faces dominates the (actually correct) sensory evidence for concavity — is a particularly clear example (Gregory, 1997).

**Hallucinatory states** in schizophrenia and other conditions may reflect dysregulation of prediction error signals, such that internal states generate predictions that are experienced as perceptions (Corlett et al., 2019). The framework suggests that hallucinations arise when the balance between top-down predictions and bottom-up sensory evidence is shifted too far toward the former.

**Motor control** is reinterpreted as top-down prediction of the sensory consequences of movement. Rather than the motor system sending commands that cause movement, the predictive processing framework proposes that the motor system generates predictions about the sensory states that would result from successful movement — proprioceptive predictions — and that the body moves in ways that minimize the discrepancy between those predictions and current proprioceptive input. The movement is the prediction error minimization process.

## Active Inference

The extension of predictive processing to action generates one of the framework's most distinctive and counterintuitive claims: that the organism does not simply update its internal model to match the world, but also acts on the world to make it match the model. This is called **active inference** (Friston et al., 2010).

In passive perceptual inference, prediction errors are minimized by updating the internal model. In active inference, prediction errors are minimized by changing the sensory input — which means changing the organism's relationship to the world, which means taking action. The key insight is that both perception and action are forms of the same underlying process: minimizing the discrepancy between prediction and sensory input. They differ only in *where* the minimization happens: in the internal model (perception) or in the external world (action).

This reconceptualization has interesting implications. It means that goals and desires can be understood as predictions: an organism that "wants" to be in a particular state generates predictions of the sensory consequences of being in that state, and then acts to minimize the error between those predictions and current sensory input. Wanting becomes predicting-how-things-should-be, and acting becomes predicting-error-minimization. This framework dissolves the traditional distinction between perception and action, treating both as manifestations of a single prediction-minimization imperative.

### Active Inference and Behavior

Active inference also provides a natural account of exploratory behavior. An organism minimizing long-run prediction error must not only minimize current error but must also reduce uncertainty about the state of the world — because highly uncertain states will generate large future errors. This motivates what Friston and colleagues call "epistemic foraging": actively seeking out information that will reduce uncertainty, not just food or mates or shelter. Curiosity, attention, and information-seeking behavior are all, on this account, forms of active inference aimed at reducing epistemic free energy.

This is an elegant prediction: organisms under this framework should explore their environments in proportion to their uncertainty about those environments, preferring informative states over uninformative ones even when no immediate reward is available. There is evidence for this kind of epistemic behavior in mammals and birds, and some researchers have attempted to apply similar concepts to simpler organisms. The extent to which bacterial exploratory behavior during the chemotaxis random walk constitutes "epistemic foraging" in the relevant sense is an interesting question we will return to in the next section.

## Empirical Status and Challenges

The predictive processing framework has attracted both enormous enthusiasm and significant skepticism. On the positive side, it provides a unified account of a striking range of cognitive phenomena — perception, attention, learning, action, hallucination, emotion — within a single theoretical vocabulary. The mathematical framework is rigorous and generates quantitative predictions. And it aligns with a broad consensus in theoretical neuroscience about the importance of top-down modulation and internal model generation.

On the negative side, critics have raised several challenges:

**The specificity problem**: The framework makes predictions at a high level of abstraction (minimize prediction error) but many specific implementations of this principle are consistent with a wide range of observable behaviors. It is not always clear what the framework uniquely predicts versus what it accommodates post hoc.

**The prior problem**: The framework is highly sensitive to assumptions about the prior distributions encoded by the brain. These priors are not independently measured but are often inferred from behavior — making the framework somewhat circular in practice.

**The representation problem**: Despite claiming to be an "antidote to cognitivisim" (Clark, 2016), the predictive processing framework is deeply committed to internal representations — the predictions that the system generates. Critics from the enactivist tradition argue that this represents a return to cognitivism under a different name (Bruineberg et al., 2018).

These are genuine issues, and any serious engagement with the framework must take them seriously. The response from framework defenders is typically that these are problems of application and formalization, not of the core theoretical claim — but this response itself invites scrutiny, which we continue in the next section.

---

## References

Bruineberg, J., Kiverstein, J., & Rietveld, E. (2018). The anticipating brain is not a scientist: The free-energy principle from an ecological-enactive perspective. *Synthese*, 195(6), 2417–2444.

Clark, A. (2016). *Surfing Uncertainty: Prediction, Action, and the Embodied Mind*. Oxford University Press.

Corlett, P.R., Horga, G., Fletcher, P.C., Alderson-Day, B., Friston, K., & Powers, A.R. (2019). Hallucinations and strong priors. *Trends in Cognitive Sciences*, 23(2), 114–127.

Friston, K., Daunizeau, J., Kilner, J., & Kiebel, S.J. (2010). Action and behavior: A free-energy formulation. *Biological Cybernetics*, 102(3), 227–260.

Gregory, R.L. (1997). Knowledge in perception and illusion. *Philosophical Transactions of the Royal Society B*, 352(1358), 1121–1127.

Knill, D.C., & Pouget, A. (2004). The Bayesian brain: The role of uncertainty in neural coding and computation. *Trends in Neurosciences*, 27(12), 712–719.

Rao, R.P.N., & Ballard, D.H. (1999). Predictive coding in the visual cortex: A functional interpretation of some extra-classical receptive-field effects. *Nature Neuroscience*, 2(1), 79–87.
