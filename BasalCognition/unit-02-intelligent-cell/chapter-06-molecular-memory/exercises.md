# Chapter 6 Exercises: Molecular Memory and Cellular Learning

---

## Part I: Reflection and Discussion

**1. The heritability of memory**
Neural memories are not heritable: what you learn in your lifetime does not change your children's DNA. But epigenetic memories can be transmitted across cell divisions, and sometimes across generations. Does this distinction matter for our understanding of what memory "is"? Is there something fundamentally different about heritable versus non-heritable memory, or is heritability just a continuum?

**2. The CRISPR immune system as a library**
CRISPR-Cas is often described as an "immune memory," but it differs from adaptive immune memory in important ways: it stores sequence information rather than antigen-binding configurations, it is encoded in DNA rather than in cell populations, and it is potentially germline-transmissible. Is CRISPR memory more analogous to genetic information (innate knowledge) or to learned memory? Does the distinction matter? What would a strict philosopher of mind say about whether CRISPR spacers constitute "knowledge"?

**3. Habituation criteria and *Stentor***
The criteria for distinguishing habituation from peripheral adaptation are: (a) specificity to the habituated stimulus, (b) reversibility with rest, and (c) dishabituation by a novel strong stimulus. The Dexter et al. *Stentor* study appears to satisfy all three. But critics might argue that these criteria, developed for nervous systems, are being applied in a context (single-cell biology) where they may mean something different. Is this a valid objection? How would you design an experiment to settle it?

**4. Evolutionary learning vs. individual learning**
The Mitchell et al. result shows that *E. coli* "anticipates" the lactose-maltose correlation, but this anticipation is evolutionarily encoded — it took many generations of selection to build the regulatory connection that produces it. Does evolutionary encoding of a correlation count as "learning"? Defend your position with reference to both the biology and to the philosophical literature on learning. Is there a coherent distinction between "the genome learned through evolution" and "the organism learned through experience"?

**5. Memory without a substrate?**
Every form of memory we have discussed in this chapter has a molecular substrate — DNA methylation, histone marks, CRISPR spacers, ion channel inactivation state, memory cell populations. Is this universally necessary? Could there be a form of memory that does not have a molecular substrate, or is the requirement for a physical substrate a necessary feature of any possible memory system? What does your answer imply about the possibilities for memory in non-biological systems?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Amnesiac Bacterium**
You have developed a tool that can instantly and reversibly remove all CRISPR spacers from a bacterium's genome, while leaving the rest of the genome intact. You use this on a bacterium that has acquired 50 spacers from 50 different phage encounters. You then expose this bacterium to one of the phage it was previously immune to. Describe what happens: (a) at the molecular level (what does the phage do, what does the cell do?), (b) at the population level (what happens to a colony of such bacteria?), and (c) at the evolutionary level (what selective pressure does this create?). Now imagine that the spacer-removal tool is naturally occurring — it is an enzyme evolved by phage to disrupt bacterial CRISPR immunity. What is the evolutionary arms race that would result?

**Thought Experiment 2: The Epigenetically Programmed Descendant**
Consider a scenario in which a mother organism experiences a severe famine during her reproductive period. The famine triggers epigenetic changes in her germline cells. Some of these changes are transmitted to her offspring. The offspring are born into an environment of plenty, but their metabolism is epigenetically "programmed" for scarcity — they are more prone to storing fat, are more metabolically conservative, and develop insulin resistance. From the perspective of the mother's evolutionary fitness, was this epigenetic transmission adaptive? From the perspective of the offspring's fitness? From the perspective of the offspring's descendants (assuming the famine did not recur)? What does this tell us about the "from whose perspective" question in evolutionary biology?

**Thought Experiment 3: The Learning Molecule**
Design the simplest possible molecular system that satisfies all the criteria for habituation: it has a sensor, a motor output, stimulus-specific memory of recent activation, reversibility with time, and dishabituation by strong stimulation. Your system must consist of at most three distinct protein species and must be capable of existing in a lipid membrane. Specify the kinetic properties each protein must have (binding affinities, reaction rates, recovery time constants) for the system to exhibit robust habituation over 5-10 repeated stimuli with 1-minute inter-stimulus intervals, recovery over 30 minutes, and dishabituation by a 10-fold stronger stimulus. Is this physically realizable? If so, what does this tell us about the computational requirements for the simplest possible learning system?

---

## Part III: Laboratory Investigations

**Lab 1: CRISPR Spacer Diversity in Environmental Samples (computational/bioinformatics)**
CRISPR spacer sequences encode the immunological history of microbial populations — their past encounters with phage. By analyzing spacer sequences from environmental metagenomes, we can reconstruct aspects of this history.

*Procedure*: Download a publicly available metagenome dataset from a microbial community with known phage pressure (ocean surface water, cheese rind, or human gut microbiome datasets are good options; NCBI SRA has many). Use the CRISPRFinder or CRT (CRISPR Recognition Tool) to identify CRISPR loci in assembled sequences. Extract spacer sequences and BLAST them against known phage genome databases.

*Analysis*: What fraction of spacers match known phage? What is the diversity of phage represented in the CRISPR arrays? Do different taxonomic groups in the same environment have CRISPR arrays matching different phage? Does the number of spacers per array correlate with any ecological features of the environment?

*Discussion*: What does the pattern of spacer diversity tell you about the history of phage-bacteria interactions in this environment? How would you interpret an environment with very diverse spacer arrays versus one with many identical spacers?

**Lab 2: Stress Memory in *E. coli* (wet lab)**
The phenomenon of heat shock pre-conditioning — in which a mild heat shock prepares bacteria for a subsequent severe heat shock — can be demonstrated in a teaching laboratory.

*Procedure*: Grow *E. coli* K-12 cultures to mid-log phase (OD600 ≈ 0.4-0.5). Divide into three groups: (1) control (no pre-treatment), (2) mild heat shock (42°C for 30 min), (3) severe heat shock only (50°C for 10 min). After the mild heat shock and a 1-hour recovery at 37°C, expose groups 1 and 2 to severe heat shock (50°C for 10 min). Plate all three groups for colony counts immediately and after treatments.

*Analysis*: Calculate survival rates (colony counts after treatment / initial colony count) for all three conditions. Is the pre-conditioned group more resistant to the severe heat shock? How large is the effect? How does this compare to the control severe heat shock group?

*Discussion*: What molecular mechanisms likely underlie the pre-conditioning effect? How long does the thermotolerance persist? Design a follow-up experiment to determine the timescale of the stress memory.

**Lab 3: The Lac Operon and Predictive Logic (computational modeling)**
Using a mathematical modeling environment, model the lac operon regulation as a logical circuit and analyze its predictive properties.

*Procedure*: Implement the lac operon as a Boolean circuit with two inputs: [Lactose] (present/absent) and [Glucose] (present/absent). The output is [Lac Operon Active]. Add a second circuit representing maltose metabolism genes, which is activated by [Lactose] alone (representing the predictive connection from the Mitchell et al. result). Model both circuits as differential equations with a simple Hill function for each regulatory interaction.

*Analysis*: Simulate the response of both circuits to the following time sequences: (1) constant glucose, no lactose; (2) lactose added, glucose present; (3) lactose added, glucose absent; (4) lactose added first, then maltose added 30 minutes later. For case (4), compare the output of the maltose metabolism circuit in bacteria with vs. without the predictive connection to lactose.

*Discussion*: How much faster does the bacterium with predictive wiring respond to maltose compared to the bacterium without? Under what environmental conditions is this speed advantage most important? What is the cost of the predictive wiring (false positives — when does the predictive wiring cause unnecessary gene expression)?

---

*For further study, see the Further Reading list for Chapter 6.*
