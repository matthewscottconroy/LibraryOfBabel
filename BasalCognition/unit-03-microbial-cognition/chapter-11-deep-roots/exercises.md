# Chapter 11 Exercises: Deep Roots — Archaeal Cognition and Viral Decision-Making

---

## Part I: Reflection and Discussion

**1. The three-domain tree and the definition of cognition**
The discovery of Archaea required a fundamental reorganization of the tree of life. If we accept that cognition in the minimal sense (sensing + integrating + responding) is a feature of all three domains of life, what does this imply for the evolutionary origin of cognition? Does it make more sense to say that cognition evolved once (in LUCA) or multiple times (independently in each domain)? How would comparative molecular evidence distinguish these possibilities?

**2. Deep homology and reductionism**
The concept of deep homology — that functionally analogous structures in distantly related organisms share an ancient molecular ancestry — is a powerful but philosophically complex idea. Does the fact that the ion channels of bacteria and the ion channels of neurons share a common ancestor mean that bacterial and neural cognition are "the same kind of thing," just at different scales of elaboration? Or does elaboration produce qualitative differences that make the comparison misleading?

**3. The lambda decision and agency**
The lambda phage lysis-lysogeny decision is sensitive to host cell state, MOI, and DNA damage signals. It produces different outcomes depending on environmental information. But the phage is not an autonomous agent — it uses the host's molecular machinery. Does the lack of molecular autonomy disqualify the phage from being a "decision-maker" in any meaningful sense? Compare the phage's situation to that of a brain-in-a-vat: would a mind that depends entirely on external computational infrastructure for its operations still count as a mind?

**4. Arbitrium and the definition of communication**
The arbitrium system allows phage to communicate across time — a peptide produced by one infection influences the decision of a later infection. Biologists typically define communication as the transmission of information between sender and receiver that influences the receiver's behavior. By this definition, does arbitrium constitute communication? If so, does this change how we should think about the cognitive status of viruses?

**5. Are viruses alive?**
This is one of the most contested questions in biology. Lay out the strongest case for the position that viruses are alive and the strongest case against it. Then evaluate how your answer to the "alive" question affects your answer to the "cognitive" question. Is cognitive status contingent on being alive in the biological sense? Could there be cognitive systems that are not alive (e.g., very sophisticated AI)? What does this thought experiment reveal about the relationship between life and cognition?

---

## Part II: Thought Experiments

**Thought Experiment 1: Engineering a Smarter Phage**
You have been tasked with engineering a phage that makes "better" lysis-lysogeny decisions — decisions that maximize the long-term number of phage progeny across a fluctuating host population. You can modify the molecular machinery governing the CI/Cro switch. Consider the following modifications:
(a) Increase the sensitivity of the CI/Cro switch to MOI by making CI synthesis more responsive to phage density.
(b) Add a new sensor: a receptor that detects host DNA damage (SOS induction level) and feeds this information directly into the CI/Cro balance.
(c) Add a long-term memory component: a modification that "remembers" past lytic efficiency (how many progeny the phage produced in the last lytic cycle) and uses this to adjust the threshold for lysogeny.

For each modification, predict whether it would improve, harm, or have no effect on long-term phage fitness in: (i) a stable, healthy host population; (ii) a host population under periodic antibiotic stress; (iii) a host population with rapidly fluctuating density. Which modification produces the most robust improvement across conditions, and why?

**Thought Experiment 2: The Viral Social Contract**
The arbitrium system allows phage to coordinate the lysis-lysogeny decision based on accumulated population information. Imagine this system evolving further: phage develop not just one but multiple communication signals, allowing them to share information about host cell health, antibiotic exposure, competitor phage presence, and nutritional conditions. As this communication system elaborates, at what point (if any) would we be justified in saying the phage population has become a social collective capable of genuine collective decision-making? What criteria would you use to make this determination? How does this thought experiment relate to the transition from quorum sensing bacteria to multicellular organisms?

**Thought Experiment 3: The Archaeal Brain**
Imagine an extremophile archaea inhabiting a deep-sea hydrothermal vent that, over billions of years of evolution, develops increasingly sophisticated sensory and integrative capabilities: first more sensitive chemoreceptors, then receptor clustering for signal amplification, then rudimentary temporal integration, then a form of simple associative conditioning. At what point in this evolutionary trajectory would you attribute genuine cognition to this organism? Is there a threshold, or is cognition a continuum? Compare your answer to the evolutionary trajectory that actually led to vertebrate brains — does comparing them reveal anything about the necessary steps between minimal cellular cognition and neural cognition?

---

## Part III: Laboratory Investigations

**Lab 1: Phage Lambda Life Cycle Observation (wet lab)**
Observe the lytic and lysogenic outcomes of lambda phage infection under different multiplicity of infection conditions.

*Procedure*: Prepare serial dilutions of lambda phage stock (10^4 to 10^9 PFU/mL). Mix with *E. coli* K-12 host culture at different MOI (phage:bacteria = 0.001, 0.1, 1, 10, 100). Plate with top agar on LB plates and incubate overnight. Count plaques (clear zones = lytic; turbid/cloudy = lysogenic with some lytic production) to estimate lytic fraction. Alternatively, pick individual plaques and test colonies from within turbid plaques for lysogeny by superinfection immunity.

*Analysis*: Does the fraction of turbid (lysogenic) plaques increase with MOI? At what MOI is lysogeny most frequent? What does this tell you about the molecular mechanism governing the lysis-lysogeny decision?

*Discussion*: The CI protein produced by multiple co-infecting phage genomes accumulates to levels that favor lysogeny. At high MOI, more phage infect the same cell simultaneously. Design an experiment using CI-overexpressing phage mutants to test whether artificially elevating CI levels at low MOI mimics high-MOI lysogeny preference.

**Lab 2: Phylogenetic Analysis of Ion Channel Evolution (computational/bioinformatics)**
Trace the evolutionary history of ion channel proteins using publicly available sequence and structure databases.

*Procedure*: Download protein sequences for potassium channel family members from organisms across all three domains of life: KcsA from *Streptomyces lividans* (bacteria), a homolog from *Methanocaldococcus jannaschii* (archaea), and Kir2.1 from *Homo sapiens* (eukaryote). Using MUSCLE or CLUSTALW for alignment and IQ-TREE or RAxML for phylogenetic reconstruction, build a maximum-likelihood phylogenetic tree of the alignment.

*Analysis*: What is the branching order of bacterial, archaeal, and eukaryotic potassium channels? Is the selectivity filter sequence (TVGYG or similar) conserved across all three domains? Identify which regions of the protein are most conserved (the selectivity filter, the gating helix, the cytoplasmic domain) and which are most variable.

*Discussion*: Your phylogenetic tree may or may not faithfully reflect the three-domain tree of life. Horizontal gene transfer between bacteria and archaea is common; how might you detect it in your ion channel phylogeny? What would it mean if the phylogeny of ion channels is incongruent with the organismal phylogeny — and would it change the conclusion that ion channels are ancient?

**Lab 3: Arbitrium Signaling — A Computational Model**
Model the arbitrium system mathematically and explore how it regulates the phage population's lysis-lysogeny ratio.

*Model*: Define: N_total = total host cell population; N_lytic = cells currently undergoing lytic infection; N_lysogenic = cells harboring prophage; P_arbitrium = extracellular arbitrium peptide concentration (proportional to past lytic activity). Each new infection leads to lysis with probability p_lysis = 1 / (1 + K * P_arbitrium), where K is the arbitrium sensitivity parameter.

*Procedure*: Simulate the system over 100 infection cycles. Start with all naive cells, introduce a small number of phage. Track N_lytic, N_lysogenic, and P_arbitrium over time. Vary K from 0.01 (low sensitivity, predominantly lytic) to 100 (high sensitivity, predominantly lysogenic). Compare the equilibrium lysis/lysogeny ratio across K values.

*Analysis*: At what K value does the population settle into a stable steady-state mix of lytic and lysogenic infections? Does the system exhibit oscillations? What happens when the host population is perturbed (sudden reduction in N_total, simulating antibiotic killing)?

*Discussion*: How does the arbitrium system's behavior compare to bacterial quorum sensing? What analogous parameters in bacterial QS correspond to K (sensitivity), P_arbitrium (signal concentration), and p_lysis (behavioral output)? Does this analogy suggest that arbitrium and bacterial QS share common evolutionary origins or common design principles?

---

*For further study, see the Further Reading list for Chapter 11.*
