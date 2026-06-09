# Chapter 13: Exercises

## Part I: Reflection and Discussion Questions

**1. The meaning of "solving"**
We described the Physarum maze experiment as "maze-solving" — but in the discussion we were careful to point out that the organism does not search, evaluate, or select a path in any representational sense. It spreads everywhere and lets flow dynamics select the efficient path. Is this "solving" the maze? Construct a precise definition of "solving a problem" that either includes or excludes Physarum's behavior, and defend your definition with reference to what you think matters most about problem-solving.

**2. Memory without representation**
The Saigusa et al. (2008) experiment showed that Physarum's behavior at time T2 is systematically influenced by experience at time T1. We proposed that this might be explained by the entrainment of the organism's internal oscillator, rather than by any discrete stored memory. What does this distinction tell us about the relationship between "being influenced by the past" and "having a memory"? Is entrainment a form of memory? If not, what additional property does genuine memory require, and why does that property matter?

**3. The computation question**
Adamatzky's work shows that Physarum can implement logical gate operations in appropriate spatial configurations. But we argued that this is not "true" symbolic computation in any deep sense, because the logical operation is a consequence of the geometry imposed by the experimenter, not of any internal program. Evaluate this distinction. Does it matter — for our assessment of Physarum's cognitive status — whether the computation is implemented symbolically or physically? What criteria should we use to distinguish "genuine" computation from "mere" physics?

**4. The substrate question**
Throughout this chapter, we have noted that Physarum performs its problem-solving feats without neurons, without synapses, without any dedicated information-processing machinery. Does the absence of these structures tell us something interesting about cognition in general? Or does it simply tell us that the specific cognitive operations Physarum performs (path optimization, periodic anticipation) can be implemented in simpler substrates than thought? How do you decide which interpretation is more warranted?

**5. Ecological context**
All of Physarum's putative cognitive abilities — maze-solving, network optimization, temporal anticipation — can be understood as solutions to real biological problems: finding food efficiently, building transport networks economically, anticipating predictable environmental changes. Does this ecological context make the cognitive interpretation more or less compelling? Would you be more or less impressed by a slime mold that could solve problems with no ecological relevance?

---

## Part II: Thought Experiments

**1. The Physarum Computer**
Imagine a future in which engineers have found a way to maintain Physarum cultures indefinitely, supply them with nutrients through microfluidic channels, and interface their tube network dynamics with electronic sensors and actuators. In this scenario, Physarum could be used as the computational substrate for a real device. What kinds of problems would this device be genuinely good at? What kinds of problems would it be poor at? Most importantly: would this device be "thinking," and would the answer to that question depend on the problem it was solving?

**2. The Ship of Theseus for Slime Molds**
Physarum plasmodia can fuse: if two genetically compatible plasmodia meet, they merge into a single larger plasmodium. Now consider: two Physarum plasmodia each separately navigate a maze. Plasmodium A finds path P1; Plasmodium B, in an identical maze, finds path P2 (a different path, also shortest). They are then merged. The combined plasmodium reorganizes its network. Whose "experience" dominates? Does the merged plasmodium "know" what either individual knew? This thought experiment probes the relationship between physical continuity, information retention, and identity. What does it tell you about what memory and individuality require?

**3. What Would Falsify It?**
The claim that Physarum has "intelligence" or "cognition" is sometimes resisted on the grounds that it cannot be falsified — if the organism does something impressive, we call it smart; if it fails, we say it's just physics. Design an experiment that would genuinely falsify the claim that Physarum's maze-solving represents a form of intelligence (as opposed to mere physics). If you cannot design such an experiment, what does that tell you about the nature of the intelligence claim?

---

## Part III: Laboratory Investigations

**1. Physarum Maze Navigation**
*Goal*: Observe Physarum path optimization in a simple two-path maze.
*Materials*: Physarum polycephalum plasmodium (available from biological supply companies), non-nutrient agar, petri dishes, razor blade or paper template, oat flakes (food), dark conditions (Physarum avoids light), camera for time-lapse.
*Procedure*: Pour non-nutrient agar into a petri dish. Once set, use a razor blade to cut away agar along a template creating two connected pathways of different lengths between two chambers. One path should be approximately 2x the length of the other. Place oat flakes at both chamber positions. Transfer a small piece of Physarum plasmodium to one chamber. Photograph the setup every 30 minutes for 24 hours.
*Analysis*: Does the organism preferentially thicken tubes along the shorter path? Measure tube diameters (from photographs) at 2-hour intervals and plot as a function of time. Does the ratio of short-path to long-path tube diameter increase over time?

**2. Network Comparison: Physarum vs. Human Design**
*Goal*: Quantitatively compare a Physarum-formed network to a human-designed network for the same node configuration.
*Materials*: Physarum plasmodium, agar, petri dishes, oat flakes placed at positions corresponding to nodes of your choice (e.g., major buildings on your campus, or cities in your state).
*Procedure*: Design a node layout of 6–10 points. Place oat flakes at those positions on an agar plate. Allow Physarum to colonize and form a network over 24–48 hours. Photograph and measure the network. Separately, design what you think is a good road/path network connecting those same nodes. Compare the two on: (a) total path length, (b) average shortest path between all pairs of nodes, (c) robustness (what fraction of pairs remain connected if you remove the single most-connected node).

**3. Simulating Flow-Reinforcement Dynamics**
*Goal*: Understand the mathematical basis of Physarum network optimization.
*Materials*: Computer with Python (NumPy, NetworkX, Matplotlib).
*Procedure*: Implement the Tero et al. (2007) mathematical model of Physarum network adaptation. Start with a random planar graph connecting several nodes. Initialize all edge conductivities to a small value with slight random variation. At each time step: (1) compute flow through each edge using Kirchhoff's laws (solve a linear system); (2) update conductivity of each edge proportional to flow through that edge, minus a decay term. Iterate for 1000 steps. Visualize the network at each 100th step, with edge width proportional to conductivity.
*Analysis*: Does the network converge to a tree or does it maintain loops? Does the final network minimize total edge length, path length between terminal nodes, or some combination? How does changing the decay rate (the constant subtracted from conductivity) affect the final network topology?
