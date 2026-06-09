# Section 31.1: The Floor of Cognition

## The Bacterium as Cognitive Agent

In 2006, the philosopher and cognitive scientist Pamela Lyon published a paper that crystallized a growing sense of dissatisfaction in cognitive science with what she called "neurocentrism" — the assumption that cognitive science is fundamentally about neural systems and that non-neural systems are at most interesting analogs. Lyon's paper, "The Biogenic Approach to Cognition," argued for a simple but radical thesis: if standard cognitive science is willing to attribute cognition to non-human animals on the basis of behavioral and functional criteria, then by those same criteria, bacteria must also be cognitive — and if bacteria are not cognitive, then the criteria are inconsistent (Lyon, 2006).

The argument proceeds by noting that the behavioral criteria typically cited for animal cognition — sensitivity to environmental cues, integration of multiple signals, flexible response to novel situations, learning from experience, goal-directed navigation — are all exhibited by bacteria. *E. coli* is sensitive to dozens of distinct chemical signals; it integrates multiple signal types in its chemotaxis network; it exhibits adaptation (a form of habituation) to sustained stimuli; its behavior is goal-directed in the sense that it reliably moves toward nutrients and away from toxins; and its response to complex, novel chemical mixtures is flexible and not simply the sum of responses to each component alone. If these criteria are sufficient for cognition in rats and frogs, why are they not sufficient in bacteria?

Lyon's argument is not that bacteria are just like rats, cognitively speaking. Obviously, bacterial cognition — if we accept the term — is far simpler than rat cognition. The claim is that the difference is one of degree, not of kind. There is no sharp line between bacterial stimulus-response processing and animal cognition that is principled rather than substrate-based. If we draw the line at the possession of a nervous system, we are making a structural claim that the behavioral and functional evidence does not support.

This argument does not compel acceptance of the "bacterium is cognitive" thesis — one might instead respond by revising the criteria so that they no longer apply to bacteria — but it forces the issue: any adequate theory of minimal cognition must explain why it applies where it applies and not where it does not, using criteria that do not simply assume their conclusion.

## Criteria for Minimal Cognition

What are the candidates for necessary and sufficient conditions for minimal cognition? The literature has converged on several proposals:

### Information Processing

The most minimal criterion is information processing: a system is cognitive if it processes information about its environment in ways that influence its behavior. This criterion has the advantage of being clear and operationalizable — we can measure information processing using information-theoretic tools — but the disadvantage of being too permissive. A thermostat processes information (temperature) that influences its behavior (switching the heater). Most people do not want to call thermostats cognitive, but if information processing is the criterion, it is hard to explain why not.

The standard response is to require that the information processing be *about* something — that it represent aspects of the world — and to require that this representation be used flexibly in multiple contexts, not just in the fixed manner of a thermostat. But specifying what counts as "representation" and what counts as "flexible use" without circularity is itself a major theoretical challenge.

### Reactive vs. Anticipatory Systems

Rodney Brooks's distinction between reactive and anticipatory systems provides a different way of approaching the boundary. Reactive systems respond directly to current sensory input without any internal model of the world. Anticipatory systems maintain internal states that represent something about future states of the world and adjust behavior based on those representations (Rosen, 1985, as cited in Lyon, 2006).

Most very simple control systems are reactive: a negative feedback loop responds to the current value of a variable and adjusts a control output to bring the variable to its set point. It has no model of what will happen next; it just corrects what is happening now. More sophisticated systems are anticipatory: they generate predictions about future states and adjust behavior in advance of those states occurring.

On this criterion, bacteria might occupy an intermediate position. The methylation-based adaptation of chemoreceptors in *E. coli* can be interpreted as a form of anticipation: by averaging the concentration signal over time, the receptor adaptation system effectively computes the time derivative of the chemical gradient, which is predictive of where the gradient will be in the near future. This is a rudimentary form of anticipation, but it is anticipation nonetheless — the system is not simply responding to the current value but to the trend.

More sophisticated examples of bacterial anticipation have been documented: *E. coli* in the gut appears to use the timing of carbon source availability to predict downstream availability of other nutrients, suggesting something like a learned association between sequentially available resources (Mitchell et al., 2009). Whether this constitutes anticipation in the robust sense required for cognition, or a simpler form of chemotaxis toward multiple correlated cues, is debated.

### The Minimally Cognitive Organization Program

Barandiaran and Moreno (2008) developed a more systematic framework for specifying the minimal conditions for cognition, which they called the **minimally cognitive organization (MCO) program**. Their approach draws on dynamical systems theory and the autopoietic tradition to specify three conditions that any cognitive system must satisfy:

**1. Self-determination**: The system's behavior must be determined by its own internal dynamics, not solely by external inputs. This echoes the autopoietic concept of operational closure: the system must be an agent of its own behavior, not merely a passive responder to environmental stimuli.

**2. Adaptive self-regulation**: The system must be able to modulate its own behavior in response to environmental conditions in ways that serve its continued existence. This goes beyond simple reactive sensitivity: the modulation must be adaptive in the sense that it tends to maintain the system's viability.

**3. Sensorimotor coupling**: The system must have a specific, non-arbitrary relationship between its sensory and motor processes — it must sense the world in order to act in the world, not just sense independently and act independently. The sensorimotor loop must be integrated.

Barandiaran and Moreno argue that this combination of conditions is both necessary and sufficient for minimal cognition: any system satisfying all three is a cognitive agent; any system failing any one of them is not. They apply the framework to bacteria and conclude that bacteria satisfy all three: *E. coli* is self-determined (its chemotaxis behavior is controlled by its signaling network, not specified by the chemical gradient), it adaptively self-regulates (receptor adaptation maintains sensitivity across a wide range of background concentrations), and it has a specific sensorimotor coupling (the chemotaxis network specifically links chemical sensing to flagellar motor control in an adaptive way).

The MCO framework is more restrictive than Lyon's biogenic approach — it specifies conditions that rule out simple control systems — but it is also more principled: the three conditions can be operationalized and used to evaluate candidate cognitive systems across the biological kingdom.

## Representation and Its Alternatives

The concept of **representation** has been central to cognitive science since its inception as a discipline in the 1950s and 1960s. In its classical form, a representation is a structure in a cognitive system that "stands for" some aspect of the world — that has semantic content, that can be true or false, that can participate in computational processes that transform representations according to their content. The classical picture holds that cognition essentially involves the manipulation of representations.

The question for minimal cognition is whether representation in this robust sense is necessary for cognition, or whether more primitive concepts can do the work that representations are invoked to do.

### The Case for Representation

Defenders of representation-based accounts argue that without representations, there is no way to explain how organisms can respond appropriately to absent features of their environment, or how they can guide behavior by the goal-state rather than the current state. When a bacterium swims toward a distant glucose source, it seems to be responding to the location of the glucose, not to the currently experienced concentration at its membrane. This requires some form of representation of the glucose source's existence and location, even if that representation is rudimentary.

More sophisticated examples make the case stronger. When a honeybee performs a waggle dance to communicate the direction and distance of a food source to nestmates, it is producing a representation of the food source that influences the behavior of other bees who have not visited the source. Something must be "standing for" the food source in the dancing bee's behavior. The concept of representation seems indispensable for explaining this.

### The Case Against Representation

Critics of representation-based accounts — including many enactivists and some dynamicists — argue that representation is not needed and that the concept introduces unnecessary theoretical baggage. Chemotaxis can be explained without any appeal to internal representations: the bacterium's behavior is fully determined by its current signaling state and its immediately prior history (captured in receptor methylation). There is no point in the causal chain at which anything "stands for" the glucose source; there are only molecular interactions that bias the flagellar rotation.

More generally, critics argue that the concept of representation is doing no explanatory work in accounts of simple behavior — that it is a theoretical imposition from folk psychology and classical AI rather than a necessary feature of the biological systems we are studying. Hutto and Myin (2013) make this argument most forcefully, arguing that basic biological cognition involves no representation and that representation is a late evolutionary achievement associated with language and social cognition in humans.

The debate is genuinely difficult and has not been resolved. For graduate students, the most productive stance is to hold the concept of representation loosely: to note when a system's behavior requires appeal to internal states that "stand for" absent features of the world in order to explain the behavior, without assuming in advance that such states must exist.

### Affect and Valence as Alternatives

Some theorists propose that instead of representation, the fundamental cognitive concept for minimal systems is **affect** or **valence** — the positive/negative character of organism-world interactions. On this view, what is cognitively fundamental is not that the organism represents the world but that the world is, for the organism, positively or negatively valenced: some states of the world are better for it, others are worse.

Valence does not require representation. A bacterium swimming toward glucose is not representing glucose; it is being drawn toward a positively valenced region of its chemical environment. The valence is not in the bacterium's head (it has no head); it is in the structure of the organism-environment coupling.

This approach connects to the enactivist concept of sense-making and to the free energy principle's concept of surprise avoidance. It suggests that the foundation of cognition is not information processing or representation but the evaluative structure of organism-environment interaction — the fact that the world can be better or worse for an organism. Representation, on this view, is a later, more sophisticated cognitive achievement built on this evaluative foundation.

## Reactive vs. Anticipatory Systems Revisited

The reactive-anticipatory distinction provides a useful framework for organizing the space of possible minimal cognitive systems. But the distinction is not binary: there is a spectrum from purely reactive systems (responding only to current input) through mildly anticipatory systems (computing time derivatives or maintaining short-term running averages) to genuinely predictive systems (maintaining explicit models of future states and planning actions accordingly).

The simplest living organisms occupy the lower end of this spectrum. Bacterial chemotaxis is primarily reactive (responding to the current concentration) but with a mild anticipatory component (the adaptation system effectively computes the gradient rather than the concentration, which is predictive of near-future concentrations). As we move up the biological hierarchy, anticipatory capacity increases: insects can model the trajectories of moving prey; mammals can plan multi-step sequences of action based on modeled future states; humans can reason about counterfactual scenarios that will never occur.

The minimal cognition question is where on this spectrum the boundary of cognition lies. The biogenic approach says: wherever you draw the line, you will find that bacteria are already there. The MCO program says: at the first point where all three conditions (self-determination, adaptive self-regulation, sensorimotor coupling) are jointly satisfied. Both approaches agree that the boundary is probably somewhere in or near the bacterial world — that unicellular organisms are among the simplest cognitive systems in existence.

---

## References

Barandiaran, X.E., & Moreno, A. (2008). On what makes certain dynamical systems cognitive: A minimally cognitive organization (MCO) program. *Adaptive Behavior*, 16(5), 293–309.

Hutto, D.D., & Myin, E. (2013). *Radicalizing Enactivism: Basic Minds without Content*. MIT Press.

Lyon, P. (2006). The biogenic approach to cognition. *Cognitive Processing*, 7(1), 11–29.

Mitchell, A., Romano, G.H., Groisman, B., Yona, A., Dekel, E., Kupiec, M., ... & Pilpel, Y. (2009). Adaptive prediction of environmental changes by microorganisms. *Nature*, 460(7252), 220–224.
