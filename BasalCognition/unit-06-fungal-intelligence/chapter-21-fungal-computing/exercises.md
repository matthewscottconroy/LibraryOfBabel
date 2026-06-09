# Chapter 21 Exercises: Fungal Computing and Unconventional Intelligence

---

## Part I: Reflection and Discussion

**1. The Language Claim**
Adamatzky's 2022 paper proposed that fungi communicate using a "language" of electrical spike patterns. The paper received significant skepticism from the scientific community. What specifically do you think would need to be demonstrated — what evidence would need to exist — for the claim of fungal "language" to be scientifically defensible? Draft a research agenda of three to five experiments that could, if their results went a particular way, provide meaningful support for the language hypothesis. What results would you expect to see if the hypothesis were correct, and what results would you expect if the spike patterns were not encoding semantic information?

**2. Three Meanings of Computation**
The chapter distinguishes formal computation (Turing-machine sense), physical computation (reliable input-output relationships), and analogical computation (structural resemblance to computation). Evaluate whether fungal network optimization — the mycelium's ability to build near-optimal transport networks — constitutes computation in each of these three senses. Which level of description is most scientifically useful for understanding mycelial intelligence? What would it take to elevate the description from the physical to the formal level?

**3. Wetware Ethics**
If fungal mycelium turns out to have richer cognitive properties than we currently believe — if it has something like preferences, or something like suffering — what ethical obligations would that create for researchers using it as a computing substrate? How should research ethics frameworks handle organisms whose cognitive status is genuinely uncertain? Compare the uncertainty about fungal cognition to other cases of uncertain moral status in biology (invertebrates in research, early embryos, non-human primates) and assess whether existing frameworks apply.

**4. Embodied vs. Disembodied Computation**
The mycelium integrates sensing, computation, and action in a single physical substrate — there is no separation between sensor, processor, and effector. Most artificial intelligence systems, by contrast, are implemented in architectures that explicitly separate these functions. What might be lost, and what might be gained, by moving to more embodied computing architectures? Can you think of any existing AI systems that partially collapse these distinctions? What would it mean for an AI system to be "embodied" in the fungal sense?

**5. Convergence and Mechanism**
Physarum solves network optimization problems using oscillatory cytoplasmic contractions. Fungal mycelium solves similar problems using tip growth and reinforcement. Digital ant-colony optimization algorithms solve similar problems using pheromone-inspired reinforcement in software. What does the convergence of these very different mechanisms on similar solutions tell us about the structure of the network optimization problem? Does the convergence say anything about whether these mechanisms are "computing" in a common sense, or is the convergence purely at the level of the output?

---

## Part II: Thought Experiments

**1. The Minimum Language**
Imagine a system that generates sequences of symbols with the following properties: (a) the symbols are drawn from a finite alphabet; (b) the distribution of symbol frequencies follows Zipf's law; (c) the sequences are non-random (adjacent symbols are statistically dependent). Most linguists would not call this system a "language" — it lacks semantics, pragmatics, and the ability to form unbounded novel meanings from finite elements. Now add, one at a time, the minimal additional properties that would make the sequences qualify as a language. At what point does the system cross the threshold? What was the essential addition? How might you test for each additional property in a biological system like mycelium?

**2. The Fungal Internet**
Imagine a continent-scale mycelial network — a single genetically unified individual spanning a landmass, with hyphal cords extending from equator to pole. Information (as electrical signals) propagates at millimeters per minute. What is the effective "bandwidth" of this network, compared to the internet? Under what circumstances would such a network be useful for coordinating biological activity across the continent? What kinds of information would need to be transmitted, and what response times would be adequate? Does the biological substrate impose fundamental limits on the cognitive complexity of such a system, or are the limits purely quantitative (speed, bandwidth)?

**3. The Hive and the Neuron**
A mycelial network and a neural network (in the biological sense) both consist of many connected nodes exchanging signals. But neurons are discrete cells with clear boundaries, while hyphal nodes are regions of a continuous cytoplasmic space. Consider the difference this makes for information processing. In a neural network, a single neuron integrates signals from many synapses and produces a single output signal. In a mycelial network, a hypha junction point receives cytoplasmic flows from multiple input branches and routes flow to multiple output branches. What does "integration" mean in each case? Is the difference between the two systems fundamental to their different cognitive capacities, or is it merely a matter of quantitative parameters (speed, number of connections, signal range)?

---

## Part III: Laboratory Investigations

**1. Testing Stimulus-Response Propagation in Mycelium**

*Hypothesis*: Electrical signals generated by a localized chemical stimulus will propagate through a mycelial network and be detectable at unstimulated locations.

*Materials*: Actively growing mycelium on a large agar plate; surface microelectrodes or fine wire electrodes; a millivolt recorder with high impedance input; a concentrated solution of a chemical stimulus (glucose, ethanol, or dilute acid as a chemical stressor); a fine dropper or micropipette.

*Protocol*: Establish electrode recordings at multiple locations on the mycelial network simultaneously, or conduct trials with electrodes at systematically varied distances from the stimulus site. Record spontaneous electrical activity for 30 minutes to establish a baseline. Apply a discrete amount (5–10 µL) of the stimulus solution to one point on the mycelium and continue recording for at least 60 minutes. Repeat with the stimulus applied at different distances from each recording electrode.

*Analysis*: Does the stimulus produce a detectable change in electrical activity at the recording site? Is there a latency between stimulus application and the response that increases with electrode-stimulus distance in a way consistent with a propagating signal rather than a simultaneously generated response? Does the response amplitude decrease with distance?

**2. Comparative Growth Rate Analysis under Different Signal Regimes**

*Hypothesis*: Fungal hyphae grow faster toward regions that have previously been electrically stimulated, consistent with the hypothesis that electrical signals attract hyphal growth.

*Materials*: Mycelium of a relatively fast-growing species (*Rhizopus stolonifer* or *Trichoderma* species); split agar plates with a gap; stainless steel electrodes; a 9-volt battery with appropriate resistors to deliver safe, low-current electrical stimulation.

*Protocol*: Set up plates in which a growing mycelial front faces a gap of sterile agar. In experimental plates, position an electrode in the sterile agar on the far side of the gap and pass a small, constant current (microamperes) through the electrode for several hours before introducing the mycelium. In control plates, position the electrode but pass no current. Measure the growth of mycelium toward the electrode region in both conditions over 48–72 hours.

*Analysis*: Does the electrical stimulation alter the direction or rate of mycelial growth toward the stimulus site? If so, is the effect consistent across replicates? What control conditions would rule out explanations based on heating, desiccation, or electrolysis products from the electrode?

**3. Zipf Analysis of Mycelial Electrical Activity**

*Hypothesis*: Electrical spike trains recorded from actively growing mycelium will show a non-random temporal structure that deviates from a Poisson process, as Adamatzky's data suggest.

*Materials*: Any of the above electrical recording setups; spike-detection software (several open-source options are available for electrophysiology data); a statistics package capable of computing inter-event interval distributions and testing against Poisson and other null distributions.

*Protocol*: Record electrical activity from mycelium for extended periods (at least 4–6 hours per session) to accumulate sufficient spike counts for statistical analysis. Define "spikes" using an amplitude threshold above baseline noise. Compute the inter-spike interval distribution and compare it to the exponential distribution expected for a Poisson process. Compute the coefficient of variation of the inter-spike intervals (CV > 1 indicates clustering, CV = 1 indicates Poisson, CV < 1 indicates regular). If clustering is found, define "bursts" as groups of closely spaced spikes separated by long inter-spike intervals, and analyze the burst size distribution.

*Analysis*: Does the spike train show significant deviation from Poisson statistics? Does the burst size distribution follow a power law or other heavy-tailed distribution consistent with Zipf's law? How do these statistics compare between different environmental conditions (control vs. chemically stimulated vs. mechanically stimulated)?
