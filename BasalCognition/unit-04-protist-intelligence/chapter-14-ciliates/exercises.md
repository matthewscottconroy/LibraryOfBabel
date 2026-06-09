# Chapter 14: Exercises

## Part I: Reflection and Discussion Questions

**1. Jennings vs. Loeb**
Jennings opposed Loeb's mechanist account of lower organism behavior, arguing that trial-and-error logic was real in Paramecium. Loeb would have replied that trial-and-error is simply a description of what the avoiding reaction produces — not evidence for any process beyond tropism. Who had the better of this argument? Does the mechanistic account of the avoiding reaction (ion channels, action potentials, ciliary reversal) vindicate Loeb's mechanist program, or does it support Jennings' view that something more than simple tropism is occurring?

**2. Hierarchical behavior without a hierarchy**
We proposed that the Stentor behavioral hierarchy might emerge from the differential calcium sensitivities of different behavioral subsystems, rather than from any explicitly encoded hierarchical control structure. If this mechanistic account is correct, should we still describe Stentor's behavior as "hierarchical" — or would a better description be "graded" or "threshold-dependent"? Does the terminology matter, and if so, why?

**3. The reliability problem**
The Stentor behavioral hierarchy was difficult to replicate for decades, and the Wood et al. reinvestigation found more variability than Jennings' account suggested. How should we respond to this kind of variability in behavioral experiments with single-celled organisms? Does variability between individual cells undermine the claim that "Stentor exhibits hierarchical behavior," or is behavioral variability itself an important finding that should be explained? What would a satisfying mechanistic explanation of the variability look like?

**4. Epigenetic inheritance and acquired characteristics**
The Beisson and Sonneborn cortical inheritance experiments show that a physically induced change in cellular organization can be inherited across many cell generations without any change in DNA sequence. This is sometimes described as Lamarckian inheritance. Is that description accurate? What did Lamarck actually claim, and how well does the ciliate example fit it? What does this example tell us about the relationship between genotype and phenotype?

**5. The macronucleus as a cognitive structure**
The ciliate macronucleus is a radically reorganized version of the germline genome, produced by processes involving RNA-guided DNA elimination and amplification. In what sense, if any, is the macronucleus a cognitive structure — an organized information store that encodes the organism's developmental and functional knowledge? Is the distinction between the macronucleus (functional genome) and the micronucleus (germline genome) analogous to any cognitive distinction (long-term vs. working memory? genome vs. epigenome?) in organisms with nervous systems?

---

## Part II: Thought Experiments

**1. The Headless Decision Maker**
Stentor is sometimes described as "making decisions" about how to respond to an irritant. Suppose we discover that the full behavioral hierarchy can be produced in a cell fragment — a piece of Stentor containing the cortex and myonemes but not the macronucleus. Would this finding change your assessment of whether Stentor makes decisions? What does it tell you about the relationship between genetic information and behavioral repertoire?

**2. The Cortical Immigrant**
Imagine you have a Paramecium cell with a fully inverted cortex (all rows rotated 180 degrees, so cilia everywhere point backward). The cell survives. It mates with a normal cell. You raise a population from the conjugation products. Describe the inheritance pattern you would expect if: (a) cortical structure is inherited exclusively through cortical templating, (b) cortical structure is entirely genetically specified, (c) both mechanisms contribute. Then consider: what experiment could distinguish between these possibilities? (Note: versions of this experiment have been done — but reason through it before looking up the results.)

**3. The Minimal Decision Cell**
You want to engineer a synthetic cell that exhibits genuine decision-making — the ability to choose among multiple behavioral options based on context, in a way that increases fitness. What molecular components would it need at a minimum? Using what you know about Paramecium and Stentor signaling, design the simplest possible such cell. What behavioral repertoire would it exhibit? Would you describe its behavior as decision-making, and why or why not?

---

## Part III: Laboratory Investigations

**1. Paramecium Behavioral Observation and Quantification**
*Goal*: Observe and quantify the avoiding reaction and taxis behaviors in Paramecium.
*Materials*: Paramecium culture (available from biological supply companies or obtainable from pond water), glass slides, coverslips, dissecting microscope or compound microscope, video recording capability, FIJI/ImageJ software for tracking.
*Procedure*: (a) Observe free-swimming Paramecium under low magnification. Record 5-minute videos. Use FIJI with the TrackMate plugin to track individual cells. (b) Add a small crystal of salt to one edge of the culture drop. Record cell trajectories before and after. (c) Touch a fine wire to one end of a swimming Paramecium (the anterior or the posterior) and record the response.
*Analysis*: Does the avoiding reaction occur more frequently after anterior vs. posterior contact? Measure the duration of backward swimming in each condition. Is the response graded with the force of contact?

**2. Stentor Observation and Irritant Response**
*Goal*: Observe the behavioral repertoire of Stentor coeruleus in response to localized irritation.
*Materials*: Stentor coeruleus culture, cotton thread or hair for local irritation, dissecting microscope with video capability.
*Procedure*: Allow Stentor to settle and attach to the bottom of a petri dish. Record baseline behavior for 5 minutes. Use a hair or fine thread to deliver gentle, repeated mechanical stimulation to the oral end of the animal. Record the sequence of behaviors observed. Test 10–15 individual cells.
*Analysis*: Do you observe all five behavioral responses described by Jennings? In what order? How variable is the sequence between individual cells? After detachment and re-settlement, does the cell respond differently on second exposure?

**3. Modeling Graded Behavioral Hierarchies**
*Goal*: Simulate how differential threshold sensitivities produce apparently hierarchical behavior.
*Materials*: Computer with Python or MATLAB.
*Procedure*: Implement a simple model with three behavioral outputs (B1 = bend, B2 = contract, B3 = detach), each activated when an internal variable C (representing calcium concentration) exceeds a threshold (θ1 < θ2 < θ3). Model C as increasing during stimulus exposure and decaying exponentially when the stimulus is removed. Simulate repeated, periodic stimulus presentations and record which behaviors are activated.
*Analysis*: Under what stimulus timing and intensity does the model produce the full hierarchical sequence? Under what conditions does it jump directly to high-level responses? How sensitive is the behavior to the values of the thresholds? Compare the model's predictions to the variability reported in the Dexter et al. (2019) study.
