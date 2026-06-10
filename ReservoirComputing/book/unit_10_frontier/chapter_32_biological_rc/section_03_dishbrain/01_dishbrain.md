# Section 32.3: The DishBrain Experiments

## 32.3.1 What the Experiment Was

The DishBrain study [Kagan2022] was published in *Neuron* in 2022. We describe the experimental setup carefully, drawing on the paper.

**Cell culture.** Primary cortical neurons were harvested from mouse embryos (E18 cortex) and human induced pluripotent stem cell (iPSC)-derived neurons. Cells were cultured on high-density multi-electrode arrays (MEAs) — chips containing 512 electrodes that can both record extracellular potentials from nearby neurons and deliver electrical stimulation. The cultures were maintained for 2–6 months.

**The game interface.** The neurons were connected bidirectionally to a simplified Pong-like game:
- **Input (stimulation)**: The game state (ball position) was encoded as electrical stimulation delivered to the MEA electrodes. The stimulation pattern depended on the ball's position (left region vs. right region of the screen) and the distance from the paddle.
- **Output (readout)**: The paddl's movement was determined by the neural firing rate in two regions of the MEA: high firing rate in region A moved the paddle up; high firing rate in region B moved the paddle down.

The specific encoding was: stimulations on the left 4 electrodes if the ball was in the left side, right 4 electrodes if on the right, and frequency of stimulation proportional to the ball-paddle distance.

**The closed loop.** The system was fully closed-loop: the ball position drove stimulation, the neural responses drove paddle movement, and the resulting ball-paddle interaction fed back to the neural stimulation. This is bidirectional coupling: the neurons were not just computing an output but were part of a feedback loop.

**Training.** The study used a form of operant conditioning via stimulation: when the paddle missed the ball (a "bad" outcome), more intense and irregular stimulation was delivered. When the paddle returned the ball (a "good" outcome), weaker stimulation followed. This was not gradient descent — it was a neuromodulation signal (approximately encoding surprise or "unexpectedness") designed to be consistent with free energy minimization principles.

## 32.3.2 What Was Demonstrated

The paper reports several specific findings. We list them precisely.

**Result 1: Above-chance performance.** The neuronal cultures playing Pong returned the ball more often than chance (defined as a random paddle movement baseline). Across sessions, the cultures achieved 35-50 rallies per session, significantly above the random baseline of ~20.

**Result 2: Learning over time.** Performance improved over the course of a session and over repeated sessions. The improvement was not dramatic (not orders of magnitude above chance) but was statistically significant across multiple cell cultures.

**Result 3: Context sensitivity.** The cultures responded differently to "good" (ball returned) vs. "bad" (ball missed) feedback stimulations. Firing patterns after bad outcomes were distinct from patterns after good outcomes, consistent with the idea that the feedback changed network state.

**Result 4: Human neurons performed comparably.** iPSC-derived human cortical neurons showed similar results to mouse neurons, demonstrating the effect in a human-relevant cell type.

## 32.3.3 The Authors' Interpretation

The authors of [Kagan2022] frame their results using the *free energy principle* [Friston2010] — the theoretical framework proposed by Karl Friston in which biological systems minimize "free energy" (a quantity related to surprise or prediction error) as a unified explanation of perception, action, and learning. The DishBrain paper explicitly argues:

1. The neuronal culture's behavior is consistent with free energy minimization: the "bad feedback" (irregular stimulation) is treated by the neurons as a high-surprise state that they act to minimize by changing their behavior (paddle movements).
2. The study demonstrates "sentient behavior" in the sense that the cells exhibit goal-directed behavior consistent with an internal model of the game.

The paper's title — "In vitro neurons learn and exhibit sentience when embodied in a simulated game-world" — uses the word "sentience," which the authors define narrowly as "the capacity to sense and respond to the environment in ways that support goal-directed behavior."

## 32.3.4 What Is Contested

Several aspects of the paper's claims have been challenged by the scientific community. We present the main criticisms fairly.

**Criticism 1: Alternative explanations for performance.** Critics note that the "learning" observed could reflect (a) non-specific adaptation of the culture's firing patterns to any repeated stimulation (habituation/sensitization, not task-specific learning), (b) selection effects (cells that happened to fire appropriately at session start were maintained in a more active state by "good" feedback), or (c) homeostatic regulation (the network adjusting firing rates to maintain homeostasis, which incidentally improves game performance).

The paper acknowledges some of these alternatives but argues the results are not fully explained by them. The counter-counter-argument is that the distinction between "genuine learning" and "homeostatic adaptation that produces goal-directed behavior" may not be well-defined.

**Criticism 2: Use of "sentience."** Several neuroscientists and philosophers have criticized the use of "sentient" to describe the cells' behavior. The narrow definition used by the authors — "capacity to sense and respond to the environment" — is much weaker than the colloquial meaning (subjective experience). A thermostat "senses and responds to the environment"; this does not typically attract the label "sentient." Critics argue the word choice was misleading and generated unjustified media coverage.

The authors defend their usage as consistent with a technical definition from the free energy literature, but the controversy highlights the importance of precise language in this area.

**Criticism 3: Effect size.** The improvement in performance (from ~20 to ~35-50 rallies) is modest. Skeptics point out that the absolute performance remains far below what a simple rule-based controller achieves (100+ rallies easily). The "learning" observed in a cell culture does not approach the sophistication of even simple computational models.

**Criticism 4: Reproducibility.** The results have been reproduced independently at several labs (the Kagan group's technology is now commercialized by Cortical Labs), but the magnitude and consistency of effects vary across labs and culture preparations. This is not unusual in in vitro neuroscience, but it adds uncertainty.

## 32.3.5 The Reservoir Computing Interpretation

From a reservoir computing perspective, the DishBrain experiment can be interpreted as follows:

**The neural culture as reservoir.** The cortical neurons on the MEA implement a biological reservoir: a recurrent network with complex, nonlinear dynamics. The electrode readout extracts linear combinations of the firing rates of nearby neurons — exactly the linear readout step of RC.

**The feedback as adaptive readout.** The "learning" observed is, in the RC framework, adaptation of the readout rather than of the reservoir. The neuromodulatory feedback signal (bad outcomes → irregular stimulation) modulates the network's global excitability, which effectively changes the linear readout. This is consistent with the observed effect: not a dramatic reconfiguration of the network, but a shift in firing rates that changes the mapping from firing patterns to paddle movements.

**Why this perspective matters.** The RC interpretation suggests that the "learning" does not require anything as sophisticated as free energy minimization or an internal model of the game. A simple adaptation mechanism (global excitability modulation) could produce the observed improvement via a random search over readout weights in a high-dimensional state space.

This does not mean the experiment is unimportant. It remains a remarkable demonstration that:
1. Biological neurons can be interfaced with artificial systems in closed-loop real time.
2. The resulting system exhibits task-relevant behavioral change.
3. The reservoir computing framework is applicable to living biological systems.

## 32.3.6 What DishBrain Does and Does Not Tell Us

**What it tells us:**
- That cortical neurons on MEAs can be driven by and drive artificial systems in real time.
- That the combination of complex neural dynamics (reservoir) and feedback-driven adaptation produces task-relevant behavior at above-chance levels.
- That the biological neural substrate implements something like a reservoir: rich, nonlinear, high-dimensional dynamics from which task-relevant signals can be extracted.
- That living neurons can respond to "reward-like" feedback signals in ways that improve performance on simple tasks.

**What it does not tell us:**
- Whether the neurons have any subjective experience, awareness, or sentience in the philosophically relevant sense.
- Whether the mechanism is "learning" in a meaningful sense vs. homeostatic adaptation.
- Whether biological reservoirs offer computational advantages over artificial ones at comparable scale.
- Whether this approach will scale to more complex tasks or whether it will always remain a scientific curiosity rather than a practical technology.
