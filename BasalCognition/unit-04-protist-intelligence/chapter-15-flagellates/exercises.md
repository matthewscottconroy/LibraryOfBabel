# Chapter 15: Exercises

## Part I: Reflection and Discussion Questions

**1. The inheritance of the molecular toolkit**
The King et al. (2008) genomic data show that genes encoding cell adhesion molecules, receptor tyrosine kinases, and synaptic scaffolding proteins were present in the unicellular ancestor of animals. Does this finding make animal multicellularity seem more or less inevitable — in the sense that, given this molecular toolkit, multicellularity was likely to evolve? What additional ingredients (genetic, ecological, developmental) would have been needed to make the transition?

**2. Collective phototaxis without a brain**
Volvox performs directed phototaxis through the uncoordinated, local responses of thousands of individually photosensitive cells. In what sense is this "intelligent behavior"? In what sense is it not? Compare to the phototaxis of Chlamydomonas (single-celled). Is the collective phototaxis of Volvox more or less sophisticated than the individual phototaxis of Chlamydomonas? What criteria would you use to decide?

**3. The altruism of soma cells**
Volvox soma cells are sterile: they cannot reproduce. They sacrifice reproductive capacity for the collective. Does this sacrifice require any cognitive capacity on the part of the soma cells? Compare to the stalk cell altruism in Dictyostelium, and to soldier castes in ant colonies. What is similar about all three cases? What is different? Does the cognitive complexity of the organism seem to matter for whether we consider the sacrifice "true" altruism?

**4. Major transitions and cognitive jumps**
Maynard Smith and Szathmáry identify the multicellularity transition as one of a small number of "major transitions" in the history of life. Their framework identifies changes in how information is stored and transmitted as the key feature of these transitions. In what sense does the multicellularity transition change how information is stored and transmitted? Does it change where cognition "lives"? What would a "major transition in cognition" look like, by analogy?

**5. Pre-neural cognition**
We argued that the presence of synaptic proteins in choanoflagellates suggests that the molecular basis of neural cognition was present before neurons existed. What is the strongest objection to this argument? (Hint: consider what those proteins actually do in choanoflagellates versus what they do in neurons.) Does the objection defeat the argument, or only qualify it?

---

## Part II: Thought Experiments

**1. The Colony That Thinks**
Imagine a colonial choanoflagellate that, unlike known choanoflagellates, has evolved genuine division of cognitive labor among its cells: some cells are specialized for chemosensory detection, others for phototaxis, others for producing signal molecules that coordinate colony behavior, others for locomotion. Describe the architecture of this hypothetical system. Is it more or less "intelligent" than a Volvox colony? At what point, if ever, does this system become a nervous system? What is the minimal additional change that would make the transition?

**2. The Individual and the Colony**
Consider a human being. By the Maynard Smith-Szathmáry criterion, you are an individual because your cells have relinquished reproductive autonomy to the whole organism. But each of your cells still has a complete copy of your genome and retains, in principle, the molecular machinery for cell division. When a cell becomes cancerous, it reasserts its individuality — dividing on its own terms. Does this make cancer a "failure of individuality" at the organismal level? What does this thought experiment tell you about the relationship between evolutionary individuality and cognitive unity?

**3. The Reverse Engineer**
You are designing a robot that can perform collective phototaxis in the style of Volvox: a sphere of many independently sensory and motile units, each acting only on local information, but collectively performing directed movement toward a light source. What rules do you give each unit? What does the collective behavior depend on (number of units, spacing, local rule parameters)? At what point does the collective behavior break down — and does the answer tell you anything about the conditions for the evolution of Volvox's phototaxis?

---

## Part III: Laboratory Investigations

**1. Chlamydomonas Phototaxis**
*Goal*: Quantify phototaxis in Chlamydomonas and compare to collective phototaxis in a colonial alga.
*Materials*: Chlamydomonas reinhardtii culture (widely available from biological supply companies), directional light source, camera for time-lapse, ImageJ/FIJI for analysis.
*Procedure*: Place a concentrated culture of Chlamydomonas in a petri dish in otherwise dark conditions. Direct a narrow beam of light from one side. Record the culture at 5-minute intervals for 30 minutes. Measure the centroid of the cell distribution over time.
*Analysis*: How quickly do cells concentrate on the lit side? What is the angle of light that produces maximal drift? If you can obtain Volvox, repeat with a Volvox culture and compare the phototaxis speed and directionality.

**2. Simulating Colonial Phototaxis**
*Goal*: Demonstrate that local photosensory responses can produce collective directed movement without central coordination.
*Materials*: Computer with Python.
*Procedure*: Implement a simple agent-based simulation. Place N agents (N = 50–500) on the surface of a sphere. Give each agent an eyespot at a fixed position relative to the agent. The eyespot activates if the light source is within its field of view. When the eyespot is active, the agent reduces its flagellar force; when inactive, normal flagellar force. Light comes from one fixed direction. Compute the net torque on the sphere from all agents and update the sphere's orientation.
*Analysis*: Does the sphere orient toward the light? How does orientation speed depend on N? What happens if you add noise to each agent's response? How does your simulation compare to the behavior of real Volvox as described by Drescher et al. (2010)?

**3. Genomic Comparison of Choanoflagellates and Animals**
*Goal*: Explore the genomic evidence for shared molecular toolkit between choanoflagellates and animals.
*Materials*: Computer with internet access; NCBI, UniProt, and OrthoFinder or similar tools for comparative genomics.
*Procedure*: Using publicly available genome data for Monosiga brevicollis (or Salpingoeca rosetta), identify homologs of three animal proteins of your choice from the following categories: (a) a synaptic scaffolding protein (e.g., PSD-95/MAGUK family), (b) a cell adhesion molecule (e.g., cadherin), (c) a receptor tyrosine kinase. For each, compare the domain structure between the animal protein and the choanoflagellate homolog. How similar are they? What differences exist?
*Analysis*: Do the choanoflagellate versions of these proteins have the same domain structure as the animal versions? What does the domain structure tell you about whether the function might be similar or different? What additional experiments would be needed to determine whether the function is actually conserved?
