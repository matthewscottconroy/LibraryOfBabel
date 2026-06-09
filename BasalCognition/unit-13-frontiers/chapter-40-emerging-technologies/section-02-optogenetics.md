# Section 2: Optogenetics

## Light as a Scalpel

The ability to observe a biological process is valuable. The ability to control a specific molecular component of that process, with precise spatial and temporal resolution, while leaving everything else intact, is transformative. This is what optogenetics provides: the ability to activate or silence specific proteins — and, through them, specific cells or signaling pathways — using pulses of light.

The optogenetic revolution is one of the most consequential technical advances in neuroscience in the past half century. But its implications extend far beyond neuroscience, and its application to the study of basal cognition is one of the most exciting current frontiers.

## The Molecular Basis: Channelrhodopsins

The discovery of channelrhodopsin-2 (ChR2) — the protein that makes modern optogenetics possible — was reported by Nagel and colleagues in 2003. Channelrhodopsin-2 is a light-gated cation channel from the green alga *Chlamydomonas reinhardtii*: when illuminated with blue light, it opens and allows cations (primarily sodium and calcium) to flow into the cell, depolarizing the membrane. Crucially, it is a single protein — no other signaling components are required — and its gating kinetics are fast enough (opening in less than a millisecond, closing in tens of milliseconds) to follow light pulses with good temporal fidelity (Nagel et al., 2003).

The application of ChR2 to neuroscience — using gene delivery to express it in specific neuron types, then controlling those neurons with light — was reported by Boyden et al. (2005) and Zhang et al. (2007), launching the optogenetics field in earnest. The subsequent decade produced an enormous array of optogenetic tools: light-driven proton pumps (halorhodopsin and archaerhodopsin) for silencing neurons with yellow or green light; ChR2 variants with shifted spectral sensitivities, faster or slower kinetics, and higher or lower conductance; and tools for controlling not just membrane voltage but intracellular signaling cascades (optogenetic control of G-proteins, kinases, and transcription factors).

The *Chlamydomonas* origin of ChR2 is itself a telling detail for this book. Channelrhodopsin is a photosensory protein in a unicellular alga — it is what allows *Chlamydomonas* to orient its swimming relative to light direction, a behavior that has been studied in its own right as a form of phototaxis. The protein evolved not for neural control but for algal cognition — for a unicellular organism's need to move toward or away from light. Human neuroscience appropriated it for a different purpose, but its origin is in the basal cognitive toolkit of a non-neural organism.

## Application to Non-Neural Systems

The standard optogenetics workflow — deliver a light-sensitive protein to a specific cell type using viral or genetic methods, then illuminate with patterned light — was developed for neurons but applies in principle to any cell. The application to non-neural biological systems is expanding rapidly.

**Plant optogenetics.** Plants have their own photoreceptor systems — phytochromes, cryptochromes, and phototropins — that regulate a wide range of developmental and physiological processes. These native systems can be exploited to control plant signaling with light. More recently, the introduction of animal or algal light-sensitive proteins into plants has allowed the control of signaling pathways that plants do not regulate with light in their natural state. Papanatsiou and colleagues (2019) introduced a light-activated ion pump (based on the *Arabidopsis* proton pump) into plant guard cells and used it to control stomatal opening by illuminating individual stomata — demonstrating that the electrical state of plant cells can be directly and precisely controlled with light.

The ability to control the membrane potential of plant cells with light opens the possibility of manipulating the propagation of electrical signals in plants — asking which signals are necessary and sufficient for specific behaviors, and tracing the causal chain from electrical signal to behavioral outcome. This is, methodologically, exactly what optogenetics did for neuroscience: it transformed correlations (this neural activity is correlated with this behavior) into causal tests (this neural activity is necessary and sufficient for this behavior). The same transformation is now possible in plant biology.

**Bacterial optogenetics.** The introduction of optogenetic tools into bacteria faces different technical challenges — gene delivery in bacteria is less flexible than in mammalian cells — but several bacterial optogenetic systems have been developed, based on natural bacterial photosensory proteins (including bacteriophytochromes and LOV-domain proteins) that can control gene expression in response to red or blue light. These systems allow spatial patterning of gene expression across a bacterial colony by illuminating with spatially patterned light — creating defined regions of different gene expression states within a single colony.

This capacity to impose spatial patterns of gene expression on bacterial communities is directly relevant to studying how quorum sensing and collective behavior are regulated. By using light to control the expression of quorum sensing signal synthases or receptors in spatially defined regions, researchers can ask how information propagates from a locally stimulated subpopulation to the rest of the community, and how the community integrates spatially distributed signals.

**Bioelectric state engineering.** The most ambitious application of optogenetics to non-neural systems is the direct control of bioelectric state in developing tissues. Levin's group has used channelrhodopsins expressed in *Xenopus* embryos to modify the bioelectric pattern of specific regions of the embryo, and shown that these modifications alter downstream developmental outcomes — changing the spatial pattern of tissue differentiation, directing cells toward different fates, or creating ectopic structures (Levin et al., 2011, and subsequent work). These experiments provide causal evidence that the bioelectric pattern, not just the gene expression pattern, is a determinant of developmental outcome — a key claim of the bioelectric medicine hypothesis.

## Engineering Bioelectric States

The combination of optogenetic actuation (light-activated ion channels and pumps) with bioelectric imaging (GEVIs and voltage-sensitive dyes) creates a closed-loop system for reading and writing the bioelectric state of tissues. In such a system, the current bioelectric state is measured optically, compared to a desired target state, and the deviation is corrected by optical activation of ion channels in the cells that need to be depolarized or hyperpolarized.

This closed-loop bioelectric control is still in early stages of development, but it is the technological realization of the target morphology concept: a system that can detect deviations from a target bioelectric state and apply corrective signals to reduce those deviations. In principle, such a system could be used to guide developing tissues toward non-natural configurations — to impose a target morphology that evolution did not produce — or to restore damaged tissues to normal bioelectric patterns after injury or disease.

## What Optogenetics Can and Cannot Tell Us

Optogenetics is a tool for establishing causal relationships: it can tell us whether a specific molecular or electrical event is necessary and sufficient for a specific behavioral outcome. In the context of basal cognition, this is invaluable for moving from correlation (this bioelectric state is correlated with this behavior) to mechanism (this bioelectric state causes this behavior).

What optogenetics cannot tell us is anything about subjective experience. The ability to control membrane voltage with light tells us about the physics and biochemistry of the system; it does not tell us whether that physics and biochemistry is accompanied by experience. This limitation is not specific to optogenetics — it is a limitation of all third-person methods, as discussed in Chapter 39.

What optogenetics can do is sharpen the experimental tests. By controlling bioelectric states with unprecedented precision, it allows us to determine which patterns of bioelectric activity are necessary and sufficient for specific adaptive behaviors, and therefore to characterize the information content and functional role of bioelectric patterns in basal cognitive systems. This mechanistic understanding is the necessary foundation for any future attempt to address the experiential question with the rigor it deserves.

---

## References

Boyden, E. S., Zhang, F., Bamberg, E., Nagel, G., & Deisseroth, K. (2005). Millisecond-timescale, genetically targeted optical control of neural activity. *Nature Neuroscience*, 8(9), 1263–1268.

Nagel, G., Szellas, T., Huhn, W., Kateriya, S., Adeishvili, N., Berthold, P., ... & Bamberg, E. (2003). Channelrhodopsin-2, a directly light-gated cation-selective membrane channel. *Proceedings of the National Academy of Sciences*, 100(24), 13940–13945.

Papanatsiou, M., Petersen, J., Henderson, L., Wang, Y., Christie, J. M., & Blatt, M. R. (2019). Optogenetic manipulation of stomatal kinetics improves carbon assimilation, water use, and growth. *Science*, 363(6427), 1456–1459.

Zhang, F., Wang, L.-P., Brauner, M., Liewald, J. F., Kay, K., Watzke, N., ... & Deisseroth, K. (2007). Multimodal fast optical interrogation of neural circuitry. *Nature*, 446(7136), 633–639.
